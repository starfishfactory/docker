#!/bin/bash

# Start script for AI development environment
echo "Starting AI Development Environment..."
echo "======================================="

# Function to check if vibe-kanban is running
check_vibe_kanban() {
    if pgrep -f "vibe-kanban" > /dev/null; then
        return 0
    else
        return 1
    fi
}

# Start vibe-kanban in background using screen
echo "Starting vibe-kanban..."
screen -dmS vibe-kanban bash -c "cd /repos && su vibe -c 'npx --yes vibe-kanban@latest'"

# Wait for vibe-kanban to start
sleep 5

# Check if vibe-kanban started successfully
if check_vibe_kanban; then
    echo "✓ vibe-kanban is running"
    echo "  Access at: http://localhost:3000"
else
    echo "⚠ vibe-kanban may not have started properly"
fi

echo ""
echo "Available tools:"
echo "  - Claude Code: 'claude-code' command"
echo "  - Gemini CLI: 'gemini' command"
echo "  - GitHub CLI: 'gh' command"
echo "  - vibe-kanban: http://localhost:3000"
echo ""
echo "Screen sessions:"
echo "  - vibe-kanban: screen -r vibe-kanban"
echo ""
echo "Working directories:"
echo "  - /workspace: Main workspace"
echo "  - /repos: Repository storage for vibe-kanban"
echo ""

# Keep container running and provide interactive shell
exec bash