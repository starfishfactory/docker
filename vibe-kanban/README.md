# Vibe Kanban Docker Setup

This Docker setup runs the official Vibe Kanban NPM package in a containerized environment.

## 🚀 Quick Start

1. **Configure environment** (optional):
   ```bash
   cp .env.example .env
   # Edit .env to customize ports or GitHub OAuth settings
   ```

2. **Start the container**:
   ```bash
   docker-compose up -d
   ```

3. **Access Vibe Kanban**:
   - Open your browser at http://localhost:8100
   - The application will download and start automatically on first run

## 📋 Features

- ✅ Runs official `vibe-kanban` from NPM (always latest version)
- ✅ Includes git for project initialization
- ✅ Persistent data storage in `./data` directory
- ✅ Configuration files stored in `./config` directory
- ✅ Easy port configuration via environment variables
- ✅ Workspace mounted at `/repos` for your projects (matching official setup)

## ⚙️ Configuration

### Environment Variables

Edit `.env` file to customize:

- `APP_PORT`: Internal application port (default: 3000)
- `EXTERNAL_PORT`: External access port (default: 8100)
- `BACKEND_PORT`: Backend server port (default: 3001)
- `FRONTEND_PORT`: Frontend port (default: 3000)
- `GITHUB_CLIENT_ID`: GitHub OAuth client ID
- `GITHUB_CLIENT_SECRET`: GitHub OAuth client secret

### Volume Mounts

- `./data:/app/data`: Application data and SQLite database
- `./config:/app/config`: Configuration files
- `/var/services/homes/yoojinhyung/workspace:/repos`: Your project workspace (mounted at /repos)

## 🛠️ Common Commands

```bash
# Start container
docker-compose up -d

# View logs
docker-compose logs -f

# Stop container
docker-compose down

# Restart container
docker-compose restart

# Update to latest vibe-kanban version
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

## 📦 What's Included

- Node.js 18 Alpine Linux base image
- Git and OpenSSH client for repository management
- NPX for running the latest vibe-kanban package
- Persistent storage for data and configuration

## 🔧 Troubleshooting

### Port Already in Use
Change `EXTERNAL_PORT` in `.env` file to a different port.

### Permission Issues
Ensure the mounted workspace directory has proper permissions.

### Update Issues
Force rebuild with: `docker-compose build --no-cache`

## 📝 Notes

- The container runs `npx vibe-kanban@latest` which automatically downloads and runs the latest version
- First startup may take longer as it downloads the package
- All data is persisted in the `./data` directory
- GitHub OAuth uses Bloop AI's default app unless you configure your own

## 🔗 Links

- [Vibe Kanban GitHub](https://github.com/BloopAI/vibe-kanban)
- [Official Documentation](https://vibekanban.com/)
- [NPM Package](https://www.npmjs.com/package/vibe-kanban)