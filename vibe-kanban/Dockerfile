FROM node:18-slim

# Install required tools
RUN apt-get update && apt-get install -y \
    git \
    openssh-client \
    wget \
    unzip \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory to /repos (matching official Dockerfile)
WORKDIR /repos

# Set npm cache to temporary directory to avoid permission issues
ENV NPM_CONFIG_CACHE=/tmp/npm-cache
ENV NPM_CONFIG_YES=true

# Create necessary directories
RUN mkdir -p /repos /root/.local/share/vibe-kanban

# Run as root user (default)
# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]