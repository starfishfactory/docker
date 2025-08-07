FROM node:18-slim

# Install required tools for both Claude Code and vibe-kanban
RUN apt-get update && apt-get install -y \
    git \
    openssh-client \
    wget \
    unzip \
    curl \
    libnotify-bin \
    dbus \
    screen \
    gnupg \
    lsb-release \
    ca-certificates \
    procps \
    && rm -rf /var/lib/apt/lists/*

# GitHub CLI GPG key registration
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
    -o /usr/share/keyrings/githubcli-archive-keyring.gpg

# GitHub CLI APT repository registration
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] \
https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null

# Install GitHub CLI
RUN apt-get update && apt-get install -y gh && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install global NPM packages
RUN npm install -g @anthropic-ai/claude-code
RUN npm install -g @google/gemini-cli

# Set npm config for vibe-kanban
ENV NPM_CONFIG_CACHE=/tmp/npm-cache
ENV NPM_CONFIG_YES=true

# Create user 'vibe' for vibe-kanban (matching expected behavior)
RUN useradd -m -s /bin/bash vibe

# Create necessary directories
RUN mkdir -p /repos /home/vibe/.local/share/vibe-kanban /workspace

# Set proper permissions
RUN chown -R vibe:vibe /repos /home/vibe/.local/share/vibe-kanban

# Copy startup script
COPY --chown=vibe:vibe start.sh /usr/local/bin/start.sh
RUN chmod +x /usr/local/bin/start.sh

# Set working directory
WORKDIR /workspace

# Set environment for vibe user
ENV HOME=/home/vibe

# Set default environment variables for vibe-kanban
ENV PORT=3001
ENV HOST=0.0.0.0
ENV BACKEND_PORT=3001
ENV FRONTEND_PORT=3001

# Default command
CMD ["/usr/local/bin/start.sh"]