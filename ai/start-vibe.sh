#!/bin/bash

# Start vibe-kanban with configured environment variables
echo "Starting vibe-kanban..."
echo "================================"
echo "Frontend: http://localhost:${FRONTEND_PORT:-3000}"
echo "Backend:  Port ${BACKEND_PORT:-3001}"
echo "Host:     ${HOST:-0.0.0.0}"
echo "================================"

# Run vibe-kanban with environment variables
HOST=${HOST:-0.0.0.0} \
BACKEND_PORT=${BACKEND_PORT:-3001} \
FRONTEND_PORT=${FRONTEND_PORT:-3000} \
GITHUB_CLIENT_ID=${GITHUB_CLIENT_ID:-Ov23li9bxz3kKfPOIsGm} \
POSTHOG_API_KEY=${POSTHOG_API_KEY} \
POSTHOG_API_ENDPOINT=${POSTHOG_API_ENDPOINT} \
npx --yes vibe-kanban@latest