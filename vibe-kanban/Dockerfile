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

# Create npm cache directory and repos directory with proper ownership
RUN mkdir -p /tmp/.npm /repos && \
    chown -R vibe:vibe /tmp/.npm && \
    chmod 755 /repos

# Don't switch user yet - stay as root for now

# Configure npm to use user's home directory for global packages
# Also create vibe-kanban data directory
RUN mkdir -p ~/.npm-global ~/.local/share/vibe-kanban && \
    npm config set prefix '~/.npm-global' && \
    echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/home/vibe/.npm-cache
ENV NPM_CONFIG_YES=true
ENV PATH="/home/vibe/.npm-global/bin:${PATH}"
ENV HOME=/home/vibe

# Install Claude Code CLI globally as root
RUN npm install -g @anthropic-ai/claude-code

# Now switch to non-root user for runtime
USER vibe

# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]