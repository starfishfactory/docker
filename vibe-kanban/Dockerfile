# Enhanced vibe-kanban image with additional tools
FROM --platform=linux/amd64 node:18

# Install additional utilities first
RUN apt-get update && \
    apt-get install -y screen curl gnupg lsb-release ca-certificates git

# Install GitHub CLI
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
    -o /usr/share/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] \
    https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && \
    apt-get install -y gh

# Clean up APT cache
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# Clone and setup vibe-kanban
WORKDIR /app
RUN git clone https://github.com/BloopAI/vibe-kanban.git .

# Install dependencies
RUN npm ci

# Create necessary directories
RUN mkdir -p /app/data /repos /app/config

# Set environment variables
ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=3000
ENV DATABASE_URL=sqlite:///app/data/vibe-kanban.db

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --retries=3 --start-period=40s \
    CMD wget -q --spider http://localhost:3000/api/health || exit 1

# Start the application
CMD ["npm", "start"]