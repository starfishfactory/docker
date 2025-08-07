#!/bin/bash

# Start vibe-kanban with configured environment variables
echo "Starting vibe-kanban..."
echo "================================"
echo "Port: ${PORT:-3000}"
echo "Host: ${HOST:-0.0.0.0}"
echo "URL:  http://localhost:${EXTERNAL_PORT:-8100}"
echo "================================"

# Run vibe-kanban with environment variables (already installed globally)
PORT=${PORT:-3000} \
HOST=${HOST:-0.0.0.0} \
GITHUB_CLIENT_ID=${GITHUB_CLIENT_ID:-Ov23li9bxz3kKfPOIsGm} \
POSTHOG_API_KEY=${POSTHOG_API_KEY} \
POSTHOG_API_ENDPOINT=${POSTHOG_API_ENDPOINT} \
vibe-kanban