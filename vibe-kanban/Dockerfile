# Multi-stage build for vibe-kanban
FROM --platform=$BUILDPLATFORM rust:1.75 as rust-builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Node.js and pnpm for frontend
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g pnpm@8

# Clone vibe-kanban
WORKDIR /app
RUN git clone https://github.com/BloopAI/vibe-kanban.git .

# Install Rust dependencies
RUN cargo install cargo-watch sqlx-cli

# Install root dependencies
RUN pnpm install

# Install frontend dependencies
WORKDIR /app/frontend
RUN npm install

# Go back to root
WORKDIR /app

# Make build script executable and build the application
RUN chmod +x ./build-npm-package.sh && \
    ./build-npm-package.sh

# Final stage
FROM --platform=$TARGETPLATFORM node:18-slim

# Install required packages
RUN apt-get update && \
    apt-get install -y \
    screen \
    curl \
    gnupg \
    lsb-release \
    ca-certificates \
    git \
    wget && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install GitHub CLI
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
    -o /usr/share/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] \
    https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && \
    apt-get install -y gh && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy built package from builder
COPY --from=rust-builder /app/npx-cli/*.tgz ./

# Install the built package globally
RUN npm install -g *.tgz

# Create necessary directories
RUN mkdir -p /app/data /repos /app/config

# Set environment variables
ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV FRONTEND_PORT=3000
ENV BACKEND_PORT=3001

# Expose ports
EXPOSE 3000 3001

# Health check
HEALTHCHECK --interval=30s --timeout=10s --retries=3 --start-period=40s \
    CMD wget -q --spider http://localhost:3000 || exit 1

# Start the application using npx
CMD ["npx", "vibe-kanban"]