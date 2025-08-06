FROM node:18-slim

# Install git and other required tools
RUN apt-get update && apt-get install -y \
    git \
    openssh-client \
    wget \
    unzip \
    curl \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -s /bin/bash vibe && \
    echo "vibe ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set working directory to /repos (matching official Dockerfile)
WORKDIR /repos

# Change ownership of working directory
RUN chown -R vibe:vibe /repos

# Switch to non-root user
USER vibe

# Install Claude Code CLI globally
RUN npm install -g @anthropic-ai/claude-code --dangerously-skip-permissions

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/tmp/.npm
ENV NPM_CONFIG_YES=true

# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]