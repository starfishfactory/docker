FROM node:18-slim

# Install git and other required tools
RUN apt-get update && apt-get install -y \
    git \
    openssh-client \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Set working directory to /repos (matching official Dockerfile)
WORKDIR /repos

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/tmp/.npm
ENV NPM_CONFIG_YES=true

# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]