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

# Create npm cache directory and change ownership
RUN mkdir -p /tmp/.npm && \
    chown -R vibe:vibe /repos /tmp/.npm

# Switch to non-root user
USER vibe

# Configure npm to use user's home directory for global packages
RUN mkdir -p ~/.npm-global && \
    npm config set prefix '~/.npm-global' && \
    echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/home/vibe/.npm-cache
ENV NPM_CONFIG_YES=true
ENV PATH="/home/vibe/.npm-global/bin:${PATH}"

# Install Claude Code CLI in user space
RUN npm install -g @anthropic-ai/claude-code

# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]