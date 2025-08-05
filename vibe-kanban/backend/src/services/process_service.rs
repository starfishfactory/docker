use std::str::FromStr;

use sqlx::SqlitePool;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    command_runner,
    executor::Executor,
    models::{
        execution_process::{CreateExecutionProcess, ExecutionProcess, ExecutionProcessType},
        executor_session::{CreateExecutorSession, ExecutorSession},
        project::Project,
        task::Task,
        task_attempt::{TaskAttempt, TaskAttemptError},
    },
    utils::shell::get_shell_command,
};

/// Service responsible for managing process execution lifecycle
pub struct ProcessService;

impl ProcessService {
    /// Run cleanup script if project has one configured
    pub async fn run_cleanup_script_if_configured(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), TaskAttemptError> {
        // Get project to check if cleanup script exists
        let project = Project::find_by_id(pool, project_id)
            .await?
            .ok_or(TaskAttemptError::ProjectNotFound)?;

        if Self::should_run_cleanup_script(&project) {
            // Get worktree path
            let task_attempt = TaskAttempt::find_by_id(pool, attempt_id).await?.ok_or(
                TaskAttemptError::ValidationError("Task attempt not found".to_string()),
            )?;

            tracing::info!(
                "Running cleanup script for project {} in attempt {}",
                project_id,
                attempt_id
            );

            Self::start_cleanup_script(
                pool,
                app_state,
                attempt_id,
                task_id,
                &project,
                &task_attempt.worktree_path,
            )
            .await?;
        } else {
            tracing::debug!("No cleanup script configured for project {}", project_id);
        }

        Ok(())
    }

    /// Automatically run setup if needed, then continue with the specified operation
    pub async fn auto_setup_and_execute(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
        operation: &str, // "dev_server", "coding_agent", or "followup"
        operation_params: Option<serde_json::Value>,
    ) -> Result<(), TaskAttemptError> {
        // Check if setup is completed for this worktree
        let setup_completed = TaskAttempt::is_setup_completed(pool, attempt_id).await?;

        // Get project to check if setup script exists
        let project = Project::find_by_id(pool, project_id)
            .await?
            .ok_or(TaskAttemptError::ProjectNotFound)?;

        let needs_setup = Self::should_run_setup_script(&project) && !setup_completed;

        if needs_setup {
            // Run setup with delegation to the original operation
            Self::execute_setup_with_delegation(
                pool,
                app_state,
                attempt_id,
                task_id,
                project_id,
                operation,
                operation_params,
            )
            .await
        } else {
            // Setup not needed or already completed, continue with original operation
            match operation {
                "dev_server" => {
                    Self::start_dev_server_direct(pool, app_state, attempt_id, task_id, project_id)
                        .await
                }
                "coding_agent" => {
                    Self::start_coding_agent(pool, app_state, attempt_id, task_id, project_id).await
                }
                "followup" => {
                    let prompt = operation_params
                        .as_ref()
                        .and_then(|p| p.get("prompt"))
                        .and_then(|p| p.as_str())
                        .unwrap_or("");
                    Self::start_followup_execution_direct(
                        pool, app_state, attempt_id, task_id, project_id, prompt,
                    )
                    .await
                    .map(|_| ())
                }
                _ => Err(TaskAttemptError::ValidationError(format!(
                    "Unknown operation: {}",
                    operation
                ))),
            }
        }
    }

    /// Execute setup script with delegation context for continuing after completion
    async fn execute_setup_with_delegation(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
        delegate_to: &str,
        operation_params: Option<serde_json::Value>,
    ) -> Result<(), TaskAttemptError> {
        let (task_attempt, project) =
            Self::load_execution_context(pool, attempt_id, project_id).await?;

        // Create delegation context for execution monitor
        let delegation_context = serde_json::json!({
            "delegate_to": delegate_to,
            "operation_params": {
                "task_id": task_id,
                "project_id": project_id,
                "attempt_id": attempt_id,
                "additional": operation_params
            }
        });

        // Create modified setup script execution with delegation context in args
        let setup_script = project.setup_script.as_ref().unwrap();
        let process_id = Uuid::new_v4();

        // Create execution process record with delegation context
        let _execution_process = Self::create_execution_process_record_with_delegation(
            pool,
            attempt_id,
            process_id,
            setup_script,
            &task_attempt.worktree_path,
            delegation_context,
        )
        .await?;

        // Setup script starting with delegation

        tracing::info!(
            "Starting setup script with delegation to {} for task attempt {}",
            delegate_to,
            attempt_id
        );

        // Execute the setup script
        let child = Self::execute_setup_script_process(
            setup_script,
            pool,
            task_id,
            attempt_id,
            process_id,
            &task_attempt.worktree_path,
        )
        .await?;

        // Register for monitoring
        Self::register_for_monitoring(
            app_state,
            process_id,
            attempt_id,
            &ExecutionProcessType::SetupScript,
            child,
        )
        .await;

        tracing::info!(
            "Started setup execution with delegation {} for task attempt {}",
            process_id,
            attempt_id
        );
        Ok(())
    }

    /// Start the execution flow for a task attempt (setup script + executor)
    pub async fn start_execution(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), TaskAttemptError> {
        use crate::models::task::{Task, TaskStatus};

        // Load required entities
        let (task_attempt, project) =
            Self::load_execution_context(pool, attempt_id, project_id).await?;

        // Update task status to indicate execution has started
        Task::update_status(pool, task_id, project_id, TaskStatus::InProgress).await?;

        // Determine execution sequence based on project configuration
        if Self::should_run_setup_script(&project) {
            Self::start_setup_script(
                pool,
                app_state,
                attempt_id,
                task_id,
                &project,
                &task_attempt.worktree_path,
            )
            .await
        } else {
            Self::start_coding_agent(pool, app_state, attempt_id, task_id, project_id).await
        }
    }

    /// Start the coding agent after setup is complete or if no setup is needed
    pub async fn start_coding_agent(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        _project_id: Uuid,
    ) -> Result<(), TaskAttemptError> {
        let task_attempt = TaskAttempt::find_by_id(pool, attempt_id)
            .await?
            .ok_or(TaskAttemptError::TaskNotFound)?;

        let executor_config = Self::resolve_executor_config(&task_attempt.executor);

        Self::start_process_execution(
            pool,
            app_state,
            attempt_id,
            task_id,
            crate::executor::ExecutorType::CodingAgent {
                config: executor_config,
                follow_up: None,
            },
            "Starting executor".to_string(),
            ExecutionProcessType::CodingAgent,
            &task_attempt.worktree_path,
        )
        .await
    }

    /// Start a dev server for this task attempt (with automatic setup)
    pub async fn start_dev_server(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), TaskAttemptError> {
        // Ensure worktree exists (recreate if needed for cold task support)
        let _worktree_path =
            TaskAttempt::ensure_worktree_exists(pool, attempt_id, project_id, "dev server").await?;

        // Use automatic setup logic
        Self::auto_setup_and_execute(
            pool,
            app_state,
            attempt_id,
            task_id,
            project_id,
            "dev_server",
            None,
        )
        .await
    }

    /// Start a dev server directly without setup check (internal method)
    pub async fn start_dev_server_direct(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), TaskAttemptError> {
        // Ensure worktree exists (recreate if needed for cold task support)
        let worktree_path =
            TaskAttempt::ensure_worktree_exists(pool, attempt_id, project_id, "dev server").await?;

        // Get the project to access the dev_script
        let project = Project::find_by_id(pool, project_id)
            .await?
            .ok_or(TaskAttemptError::TaskNotFound)?;

        let dev_script = project.dev_script.ok_or_else(|| {
            TaskAttemptError::ValidationError(
                "No dev script configured for this project".to_string(),
            )
        })?;

        if dev_script.trim().is_empty() {
            return Err(TaskAttemptError::ValidationError(
                "Dev script is empty".to_string(),
            ));
        }

        let result = Self::start_process_execution(
            pool,
            app_state,
            attempt_id,
            task_id,
            crate::executor::ExecutorType::DevServer(dev_script),
            "Starting dev server".to_string(),
            ExecutionProcessType::DevServer,
            &worktree_path,
        )
        .await;

        if result.is_ok() {
            app_state
                .track_analytics_event(
                    "dev_server_started",
                    Some(serde_json::json!({
                        "task_id": task_id.to_string(),
                        "project_id": project_id.to_string(),
                        "attempt_id": attempt_id.to_string()
                    })),
                )
                .await;
        }

        result
    }

    /// Start a follow-up execution using the same executor type as the first process (with automatic setup)
    /// Returns the attempt_id that was actually used (always the original attempt_id for session continuity)
    pub async fn start_followup_execution(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
        prompt: &str,
    ) -> Result<Uuid, TaskAttemptError> {
        use crate::models::task::{Task, TaskStatus};

        // Get the current task attempt to check if worktree is deleted
        let current_attempt = TaskAttempt::find_by_id(pool, attempt_id)
            .await?
            .ok_or(TaskAttemptError::TaskNotFound)?;

        let actual_attempt_id = attempt_id;

        if current_attempt.worktree_deleted {
            info!(
                "Resurrecting deleted attempt {} (branch: {}) for followup execution - maintaining session continuity",
                attempt_id, current_attempt.branch
            );
        } else {
            info!(
                "Continuing followup execution on active attempt {} (branch: {})",
                attempt_id, current_attempt.branch
            );
        }

        // Update task status to indicate follow-up execution has started
        Task::update_status(pool, task_id, project_id, TaskStatus::InProgress).await?;

        // Ensure worktree exists (recreate if needed for cold task support)
        // This will resurrect the worktree at the exact same path for session continuity
        let _worktree_path =
            TaskAttempt::ensure_worktree_exists(pool, actual_attempt_id, project_id, "followup")
                .await?;

        // Use automatic setup logic with followup parameters
        let operation_params = serde_json::json!({
            "prompt": prompt
        });

        Self::auto_setup_and_execute(
            pool,
            app_state,
            attempt_id,
            task_id,
            project_id,
            "followup",
            Some(operation_params),
        )
        .await?;

        Ok(actual_attempt_id)
    }

    /// Start a follow-up execution directly without setup check (internal method)
    pub async fn start_followup_execution_direct(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project_id: Uuid,
        prompt: &str,
    ) -> Result<Uuid, TaskAttemptError> {
        // Ensure worktree exists (recreate if needed for cold task support)
        // This will resurrect the worktree at the exact same path for session continuity
        let worktree_path =
            TaskAttempt::ensure_worktree_exists(pool, attempt_id, project_id, "followup").await?;

        // Find the most recent coding agent execution process to get the executor type
        // Look up processes from the ORIGINAL attempt to find the session
        let execution_processes =
            ExecutionProcess::find_by_task_attempt_id(pool, attempt_id).await?;
        let most_recent_coding_agent = execution_processes
            .iter()
            .rev() // Reverse to get most recent first (since they're ordered by created_at ASC)
            .find(|p| matches!(p.process_type, ExecutionProcessType::CodingAgent))
            .ok_or_else(|| {
                tracing::error!(
                    "No previous coding agent execution found for task attempt {}. Found {} processes: {:?}",
                    attempt_id,
                    execution_processes.len(),
                    execution_processes.iter().map(|p| format!("{:?}", p.process_type)).collect::<Vec<_>>()
                );
                TaskAttemptError::ValidationError("No previous coding agent execution found for follow-up".to_string())
            })?;

        // Get the executor session to find the session ID
        // This looks up the session from the original attempt's processes
        let executor_session =
            ExecutorSession::find_by_execution_process_id(pool, most_recent_coding_agent.id)
                .await?
                .ok_or_else(|| {
                    tracing::error!(
                        "No executor session found for execution process {} (task attempt {})",
                        most_recent_coding_agent.id,
                        attempt_id
                    );
                    TaskAttemptError::ValidationError(
                        "No executor session found for follow-up".to_string(),
                    )
                })?;

        let executor_config: crate::executor::ExecutorConfig = match most_recent_coding_agent
            .executor_type
            .as_deref()
        {
            Some(executor_str) => executor_str.parse().unwrap(),
            _ => {
                tracing::error!(
                                    "Invalid or missing executor type '{}' for execution process {} (task attempt {})",
                                    most_recent_coding_agent.executor_type.as_deref().unwrap_or("None"),
                                    most_recent_coding_agent.id,
                                    attempt_id
                                );
                return Err(TaskAttemptError::ValidationError(format!(
                    "Invalid executor type for follow-up: {}",
                    most_recent_coding_agent
                        .executor_type
                        .as_deref()
                        .unwrap_or("None")
                )));
            }
        };

        // Try to use follow-up with session ID, but fall back to new session if it fails
        let followup_executor = if let Some(session_id) = &executor_session.session_id {
            // First try with session ID for continuation
            debug!(
                "SESSION_FOLLOWUP: Attempting follow-up execution with session ID: {} (attempt: {}, worktree: {})",
                session_id, attempt_id, worktree_path
            );
            crate::executor::ExecutorType::CodingAgent {
                config: executor_config.clone(),
                follow_up: Some(crate::executor::FollowUpInfo {
                    session_id: session_id.clone(),
                    prompt: prompt.to_string(),
                }),
            }
        } else {
            // No session ID available, start new session
            tracing::warn!(
                "SESSION_FOLLOWUP: No session ID available for follow-up execution on attempt {}, starting new session (worktree: {})",
                attempt_id, worktree_path
            );
            crate::executor::ExecutorType::CodingAgent {
                config: executor_config.clone(),
                follow_up: None,
            }
        };

        // Try to start the follow-up execution
        let execution_result = Self::start_process_execution(
            pool,
            app_state,
            attempt_id,
            task_id,
            followup_executor,
            "Starting follow-up executor".to_string(),
            ExecutionProcessType::CodingAgent,
            &worktree_path,
        )
        .await;

        // If follow-up execution failed and we tried to use a session ID,
        // fall back to a new session
        if execution_result.is_err() && executor_session.session_id.is_some() {
            tracing::warn!(
                "SESSION_FOLLOWUP: Follow-up execution with session ID '{}' failed for attempt {}, falling back to new session. Error: {:?}",
                executor_session.session_id.as_ref().unwrap(),
                attempt_id,
                execution_result.as_ref().err()
            );

            // Create a new session instead of trying to resume
            let new_session_executor = crate::executor::ExecutorType::CodingAgent {
                config: executor_config,
                follow_up: None,
            };

            Self::start_process_execution(
                pool,
                app_state,
                attempt_id,
                task_id,
                new_session_executor,
                "Starting new executor session (follow-up session failed)".to_string(),
                ExecutionProcessType::CodingAgent,
                &worktree_path,
            )
            .await?;
        } else {
            // Either it succeeded or we already tried without session ID
            execution_result?;
        }

        Ok(attempt_id)
    }

    /// Unified function to start any type of process execution
    #[allow(clippy::too_many_arguments)]
    pub async fn start_process_execution(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        executor_type: crate::executor::ExecutorType,
        activity_note: String,
        process_type: ExecutionProcessType,
        worktree_path: &str,
    ) -> Result<(), TaskAttemptError> {
        let process_id = Uuid::new_v4();

        // Create execution process record
        let _execution_process = Self::create_execution_process_record(
            pool,
            attempt_id,
            process_id,
            &executor_type,
            process_type.clone(),
            worktree_path,
        )
        .await?;

        // Create executor session for coding agents
        if matches!(process_type, ExecutionProcessType::CodingAgent) {
            // Extract follow-up prompt if this is a follow-up execution
            let followup_prompt = match &executor_type {
                crate::executor::ExecutorType::CodingAgent {
                    follow_up: Some(ref info),
                    ..
                } => Some(info.prompt.clone()),
                _ => None,
            };
            Self::create_executor_session_record(
                pool,
                attempt_id,
                task_id,
                process_id,
                followup_prompt,
            )
            .await?;
        }

        // Process started successfully

        tracing::info!("Starting {} for task attempt {}", activity_note, attempt_id);

        // Execute the process
        let child = Self::execute_process(
            &executor_type,
            pool,
            task_id,
            attempt_id,
            process_id,
            worktree_path,
        )
        .await?;

        // Register for monitoring
        Self::register_for_monitoring(app_state, process_id, attempt_id, &process_type, child)
            .await;

        tracing::info!(
            "Started execution {} for task attempt {}",
            process_id,
            attempt_id
        );
        Ok(())
    }

    /// Load the execution context (task attempt and project) with validation
    async fn load_execution_context(
        pool: &SqlitePool,
        attempt_id: Uuid,
        project_id: Uuid,
    ) -> Result<(TaskAttempt, Project), TaskAttemptError> {
        let task_attempt = TaskAttempt::find_by_id(pool, attempt_id)
            .await?
            .ok_or(TaskAttemptError::TaskNotFound)?;

        let project = Project::find_by_id(pool, project_id)
            .await?
            .ok_or(TaskAttemptError::ProjectNotFound)?;

        Ok((task_attempt, project))
    }

    /// Check if setup script should be executed
    fn should_run_setup_script(project: &Project) -> bool {
        project
            .setup_script
            .as_ref()
            .map(|script| !script.trim().is_empty())
            .unwrap_or(false)
    }

    fn should_run_cleanup_script(project: &Project) -> bool {
        project
            .cleanup_script
            .as_ref()
            .map(|script| !script.trim().is_empty())
            .unwrap_or(false)
    }

    /// Start the setup script execution
    async fn start_setup_script(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project: &Project,
        worktree_path: &str,
    ) -> Result<(), TaskAttemptError> {
        let setup_script = project.setup_script.as_ref().unwrap();

        Self::start_process_execution(
            pool,
            app_state,
            attempt_id,
            task_id,
            crate::executor::ExecutorType::SetupScript(setup_script.clone()),
            "Starting setup script".to_string(),
            ExecutionProcessType::SetupScript,
            worktree_path,
        )
        .await
    }

    /// Start the cleanup script execution
    async fn start_cleanup_script(
        pool: &SqlitePool,
        app_state: &crate::app_state::AppState,
        attempt_id: Uuid,
        task_id: Uuid,
        project: &Project,
        worktree_path: &str,
    ) -> Result<(), TaskAttemptError> {
        let cleanup_script = project.cleanup_script.as_ref().unwrap();

        Self::start_process_execution(
            pool,
            app_state,
            attempt_id,
            task_id,
            crate::executor::ExecutorType::CleanupScript(cleanup_script.clone()),
            "Starting cleanup script".to_string(),
            ExecutionProcessType::CleanupScript,
            worktree_path,
        )
        .await
    }

    /// Resolve executor configuration from string name
    fn resolve_executor_config(executor_name: &Option<String>) -> crate::executor::ExecutorConfig {
        if let Some(name) = executor_name {
            crate::executor::ExecutorConfig::from_str(name).unwrap_or_else(|_| {
                tracing::warn!(
                    "Unknown executor type '{}', defaulting to EchoExecutor",
                    name
                );
                crate::executor::ExecutorConfig::Echo
            })
        } else {
            tracing::warn!("No executor type specified, defaulting to EchoExecutor");
            crate::executor::ExecutorConfig::Echo
        }
    }

    /// Create execution process database record
    async fn create_execution_process_record(
        pool: &SqlitePool,
        attempt_id: Uuid,
        process_id: Uuid,
        executor_type: &crate::executor::ExecutorType,
        process_type: ExecutionProcessType,
        worktree_path: &str,
    ) -> Result<ExecutionProcess, TaskAttemptError> {
        let (shell_cmd, shell_arg) = get_shell_command();
        let (command, args, executor_type_string) = match executor_type {
            crate::executor::ExecutorType::SetupScript(_) => (
                shell_cmd.to_string(),
                Some(serde_json::to_string(&[shell_arg, "setup-script"]).unwrap()),
                Some("setup-script".to_string()),
            ),
            crate::executor::ExecutorType::CleanupScript(_) => (
                shell_cmd.to_string(),
                Some(serde_json::to_string(&[shell_arg, "cleanup-script"]).unwrap()),
                Some("cleanup-script".to_string()),
            ),
            crate::executor::ExecutorType::DevServer(_) => (
                shell_cmd.to_string(),
                Some(serde_json::to_string(&[shell_arg, "dev_server"]).unwrap()),
                None, // Dev servers don't have an executor type
            ),
            crate::executor::ExecutorType::CodingAgent { config, follow_up } => {
                let command = if follow_up.is_some() {
                    "followup_executor".to_string()
                } else {
                    "executor".to_string()
                };
                (command, None, Some(format!("{}", config)))
            }
        };

        let create_process = CreateExecutionProcess {
            task_attempt_id: attempt_id,
            process_type,
            executor_type: executor_type_string,
            command,
            args,
            working_directory: worktree_path.to_string(),
        };

        ExecutionProcess::create(pool, &create_process, process_id)
            .await
            .map_err(TaskAttemptError::from)
    }

    /// Create executor session record for coding agents
    async fn create_executor_session_record(
        pool: &SqlitePool,
        attempt_id: Uuid,
        task_id: Uuid,
        process_id: Uuid,
        followup_prompt: Option<String>,
    ) -> Result<(), TaskAttemptError> {
        // Use follow-up prompt if provided, otherwise get the task to create prompt
        let prompt = if let Some(followup_prompt) = followup_prompt {
            followup_prompt
        } else {
            let task = Task::find_by_id(pool, task_id)
                .await?
                .ok_or(TaskAttemptError::TaskNotFound)?;
            format!("{}\n\n{}", task.title, task.description.unwrap_or_default())
        };

        let session_id = Uuid::new_v4();
        let create_session = CreateExecutorSession {
            task_attempt_id: attempt_id,
            execution_process_id: process_id,
            prompt: Some(prompt),
        };

        ExecutorSession::create(pool, &create_session, session_id)
            .await
            .map(|_| ())
            .map_err(TaskAttemptError::from)
    }

    /// Execute the process based on type
    async fn execute_process(
        executor_type: &crate::executor::ExecutorType,
        pool: &SqlitePool,
        task_id: Uuid,
        attempt_id: Uuid,
        process_id: Uuid,
        worktree_path: &str,
    ) -> Result<command_runner::CommandProcess, TaskAttemptError> {
        use crate::executors::{CleanupScriptExecutor, DevServerExecutor, SetupScriptExecutor};

        let result = match executor_type {
            crate::executor::ExecutorType::SetupScript(script) => {
                let executor = SetupScriptExecutor {
                    script: script.clone(),
                };
                executor
                    .execute_streaming(pool, task_id, attempt_id, process_id, worktree_path)
                    .await
            }
            crate::executor::ExecutorType::CleanupScript(script) => {
                let executor = CleanupScriptExecutor {
                    script: script.clone(),
                };
                executor
                    .execute_streaming(pool, task_id, attempt_id, process_id, worktree_path)
                    .await
            }
            crate::executor::ExecutorType::DevServer(script) => {
                let executor = DevServerExecutor {
                    script: script.clone(),
                };
                executor
                    .execute_streaming(pool, task_id, attempt_id, process_id, worktree_path)
                    .await
            }
            crate::executor::ExecutorType::CodingAgent { config, follow_up } => {
                let executor = config.create_executor();

                if let Some(ref follow_up_info) = follow_up {
                    executor
                        .execute_followup_streaming(
                            pool,
                            task_id,
                            attempt_id,
                            process_id,
                            &follow_up_info.session_id,
                            &follow_up_info.prompt,
                            worktree_path,
                        )
                        .await
                } else {
                    executor
                        .execute_streaming(pool, task_id, attempt_id, process_id, worktree_path)
                        .await
                }
            }
        };

        result.map_err(|e| TaskAttemptError::Git(git2::Error::from_str(&e.to_string())))
    }

    /// Register process for monitoring
    async fn register_for_monitoring(
        app_state: &crate::app_state::AppState,
        process_id: Uuid,
        attempt_id: Uuid,
        process_type: &ExecutionProcessType,
        child: command_runner::CommandProcess,
    ) {
        let execution_type = match process_type {
            ExecutionProcessType::SetupScript => crate::app_state::ExecutionType::SetupScript,
            ExecutionProcessType::CleanupScript => crate::app_state::ExecutionType::CleanupScript,
            ExecutionProcessType::CodingAgent => crate::app_state::ExecutionType::CodingAgent,
            ExecutionProcessType::DevServer => crate::app_state::ExecutionType::DevServer,
        };

        app_state
            .add_running_execution(
                process_id,
                crate::app_state::RunningExecution {
                    task_attempt_id: attempt_id,
                    _execution_type: execution_type,
                    child,
                },
            )
            .await;
    }

    /// Create execution process database record with delegation context
    async fn create_execution_process_record_with_delegation(
        pool: &SqlitePool,
        attempt_id: Uuid,
        process_id: Uuid,
        _setup_script: &str,
        worktree_path: &str,
        delegation_context: serde_json::Value,
    ) -> Result<ExecutionProcess, TaskAttemptError> {
        let (shell_cmd, shell_arg) = get_shell_command();

        // Store delegation context in args for execution monitor to read
        let args_with_delegation = serde_json::json!([
            shell_arg,
            "setup-script",
            "--delegation-context",
            delegation_context.to_string()
        ]);

        let create_process = CreateExecutionProcess {
            task_attempt_id: attempt_id,
            process_type: ExecutionProcessType::SetupScript,
            executor_type: Some("setup-script".to_string()),
            command: shell_cmd.to_string(),
            args: Some(args_with_delegation.to_string()),
            working_directory: worktree_path.to_string(),
        };

        ExecutionProcess::create(pool, &create_process, process_id)
            .await
            .map_err(TaskAttemptError::from)
    }

    /// Execute setup script process specifically
    async fn execute_setup_script_process(
        setup_script: &str,
        pool: &SqlitePool,
        task_id: Uuid,
        attempt_id: Uuid,
        process_id: Uuid,
        worktree_path: &str,
    ) -> Result<command_runner::CommandProcess, TaskAttemptError> {
        use crate::executors::SetupScriptExecutor;

        let executor = SetupScriptExecutor {
            script: setup_script.to_string(),
        };

        executor
            .execute_streaming(pool, task_id, attempt_id, process_id, worktree_path)
            .await
            .map_err(|e| TaskAttemptError::Git(git2::Error::from_str(&e.to_string())))
    }
}
