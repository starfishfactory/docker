FROM node:18-alpine

# Install git and other required tools
RUN apk add --no-cache git openssh-client wget

# Set working directory to /repos (matching official Dockerfile)
WORKDIR /repos

# Set environment for npm packages
ENV NPM_CONFIG_CACHE=/tmp/.npm
ENV NPM_CONFIG_YES=true

# Run npx with --yes flag to auto-install without prompts
CMD ["npx", "--yes", "vibe-kanban@latest"]