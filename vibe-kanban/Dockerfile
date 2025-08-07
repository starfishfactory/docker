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

# Install Claude Code CLI globally as root
RUN npm install -g @anthropic-ai/claude-code

# Create directories with proper ownership for vibe user
RUN mkdir -p /home/vibe/.npm-global/lib \
             /home/vibe/.npm-global/bin \
             /home/vibe/.local/share/vibe-kanban \
             /home/vibe/.cache/npm && \
    chown -R vibe:vibe /home/vibe

# Now switch to non-root user for runtime
USER vibe

# Configure npm to use user's home directory for global packages
RUN npm config set prefix '/home/vibe/.npm-global' && \
    npm config set cache '/home/vibe/.cache/npm' && \
    echo 'export PATH=/home/vibe/.npm-global/bin:$PATH' >> ~/.bashrc && \
    mkdir -p /home/vibe/.npm-global/lib/node_modules

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/home/vibe/.cache/npm
ENV NPM_CONFIG_YES=true
ENV PATH="/home/vibe/.npm-global/bin:${PATH}"
ENV HOME=/home/vibe

# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]