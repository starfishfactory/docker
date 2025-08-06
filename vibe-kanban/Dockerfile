FROM node:18-alpine

# Install git and other required tools
RUN apk add --no-cache git openssh-client wget

# Set working directory to /repos (matching official Dockerfile)
WORKDIR /repos

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/tmp/.npm

# Run npx vibe-kanban by default
CMD ["npx", "vibe-kanban@latest"]