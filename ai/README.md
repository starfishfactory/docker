# AI Development Environment

Simple Docker environment for AI CLI tools (Claude Code & Gemini CLI).

## Features

- **Claude Code**: Anthropic's AI coding assistant
- **Gemini CLI**: Google's AI CLI tool
- **GitHub CLI**: GitHub operations from terminal
- Ready for future vibe-kanban integration

## Quick Start

### 1. Setup Environment Variables

```bash
cp .env.example .env
# Edit .env and add your API keys
```

### 2. Build and Run

```bash
# Build the image
docker compose build

# Run the container
docker compose up -d

# Access the container
docker compose exec ai-app bash
```

### 3. Use AI Tools

Inside the container:
```bash
# Claude Code
claude-code

# Gemini CLI
gemini [command]

# GitHub CLI
gh [command]
```

## Environment Variables

### Required
- `ANTHROPIC_API_KEY`: Claude API key from [console.anthropic.com](https://console.anthropic.com/)
- `GEMINI_API_KEY`: Gemini API key from [makersuite.google.com](https://makersuite.google.com/app/apikey)
- `GITHUB_TOKEN`: GitHub token from [github.com/settings/tokens](https://github.com/settings/tokens)

### Optional (for vibe-kanban)
These variables are pre-configured with defaults for future vibe-kanban integration:
- `GITHUB_CLIENT_ID`: OAuth app ID (default: Bloop AI's app)
- `HOST`: Server bind address (default: 0.0.0.0)
- `BACKEND_PORT`: Backend port (default: 0 for auto-assign)
- `FRONTEND_PORT`: Frontend port (default: 3000)
- `POSTHOG_API_KEY`: Analytics key (optional)

## Volume Mount

The workspace is mounted at:
```
/var/services/homes/yoojinhyung/workspace:/workspace
```

## Stop and Clean

```bash
# Stop the container
docker compose down

# Remove with volumes
docker compose down -v
```