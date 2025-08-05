# Build stage
FROM node:18-alpine AS builder

# Install Rust and dependencies
RUN apk add --no-cache curl build-base perl tini git openssh-client

# Install Rust with retry logic
RUN for i in 1 2 3; do \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && break || \
    echo "Retry $i failed, waiting..." && sleep 5; \
    done
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy package files first for dependency caching
COPY package*.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY frontend/package*.json ./frontend/
COPY npx-cli/package*.json ./npx-cli/

# Install pnpm and dependencies (cached if package files unchanged)
RUN npm install -g pnpm
RUN pnpm install

COPY frontend/ ./frontend/
COPY shared/ ./shared/
RUN cd frontend && npm run build

# Build dependencies first (for caching)
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
COPY backend/build.rs ./backend/
# Create dummy source files to build dependencies
RUN mkdir -p backend/src/bin && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "pub fn lib() {}" > backend/src/lib.rs && \
    echo "fn main() {}" > backend/src/bin/cloud_runner.rs && \
    echo "fn main() {}" > backend/src/bin/generate_types.rs && \
    echo "fn main() {}" > backend/src/bin/mcp_task_server.rs && \
    echo "fn main() {}" > backend/src/bin/test_remote.rs
# Create dummy frontend/dist for build.rs
RUN mkdir -p frontend/dist && echo "<html></html>" > frontend/dist/index.html
RUN cargo build --release --manifest-path backend/Cargo.toml

# Copy actual source code and rebuild (only recompiles our code)
COPY backend/ ./backend/
# Touch the main.rs to ensure rebuild
RUN touch backend/src/main.rs && cargo build --release --manifest-path backend/Cargo.toml

# Runtime stage
FROM alpine:latest

# Install runtime dependencies only
RUN apk add --no-cache tini git openssh-client ca-certificates

# Copy built artifacts from builder
COPY --from=builder /app/target/release/vibe-kanban /usr/local/bin/vibe-kanban
COPY --from=builder /app/frontend/dist /app/frontend/dist

# Expose port
ENV HOST=0.0.0.0
ENV PORT=3001
EXPOSE 3001

# Run the application
WORKDIR /repos
ENTRYPOINT ["/sbin/tini", "--"]
CMD ["vibe-kanban"]
