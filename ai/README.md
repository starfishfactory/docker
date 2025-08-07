# AI Development Environment

Simple Docker environment for AI CLI tools (Claude Code & Gemini CLI).

## Features

- **Claude Code**: Anthropic's AI coding assistant
- **Gemini CLI**: Google's AI CLI tool
- **GitHub CLI**: GitHub operations from terminal
- **vibe-kanban**: Pre-installed globally with configured environment
  - Port configuration: 3000
  - Helper script: `start-vibe.sh` for easy launching
  - Data persistence in `./data` folder

## Quick Start

### 1. Setup Environment Variables

```bash
cp .env.example .env
# Edit .env and add your API keys
```

### 2. Setup Permissions (Important for NAS)

```bash
# Create data directory with proper permissions
mkdir -p data
chmod 755 data

# If permission issues occur, run:
sudo chown -R $USER:$USER data
```

### 3. Build and Run

```bash
# Build the image
docker compose build

# Run the container
docker compose up -d

# Access the container
docker compose exec ai-app bash
```

### 4. Use AI Tools

Inside the container:
```bash
# Claude Code
claude-code

# Gemini CLI
gemini [command]

# GitHub CLI
gh [command]

# Run vibe-kanban (Option 1: Using helper script)
./start-vibe.sh
# or
start-vibe.sh

# Run vibe-kanban (Option 2: Direct command)
# vibe-kanban is pre-installed globally
vibe-kanban

# Run vibe-kanban (Option 3: With custom port)
PORT=3000 HOST=0.0.0.0 vibe-kanban
```

## Environment Variables

### Required
- `ANTHROPIC_API_KEY`: Claude API key from [console.anthropic.com](https://console.anthropic.com/)
- `GEMINI_API_KEY`: Gemini API key from [makersuite.google.com](https://makersuite.google.com/app/apikey)
- `GITHUB_TOKEN`: GitHub token from [github.com/settings/tokens](https://github.com/settings/tokens)

### Optional (for vibe-kanban)
These variables are pre-configured for vibe-kanban:
- `GITHUB_CLIENT_ID`: OAuth app ID (default: Bloop AI's app)
- `PORT`: Application port (default: 3000)
- `HOST`: Server bind address (default: 0.0.0.0)
- `POSTHOG_API_KEY`: Analytics key (optional)

## Volume Mounts

- **Workspace**: `/var/services/homes/yoojinhyung/workspace:/workspace` - Main working directory
- **Repos**: `/var/services/homes/yoojinhyung/workspace:/repos` - For vibe-kanban repositories
- **Data**: `./data:/root/.local/share/vibe-kanban` - vibe-kanban data persistence

## Troubleshooting

### Permission Issues
If you encounter permission errors with the data folder:
```bash
# On NAS/Linux
sudo chown -R 1000:1000 data

# Or set current user
sudo chown -R $USER:$USER data
```

## Stop and Clean

```bash
# Stop the container
docker compose down

# Remove with volumes (keeps data folder)
docker compose down -v

# Complete cleanup (including data)
docker compose down -v
rm -rf data/*
```