#!/bin/sh
# Entrypoint script for vibe-kanban

echo "Starting vibe-kanban..."

# Check if /repos directory is accessible
if [ ! -d "/repos" ]; then
    echo "Warning: /repos directory not found, creating it..."
    mkdir -p /repos
fi

# Check if data directory exists
if [ ! -d "/app/data" ]; then
    echo "Creating data directory..."
    mkdir -p /app/data
fi

# Check if config directory exists
if [ ! -d "/app/config" ]; then
    echo "Creating config directory..."
    mkdir -p /app/config
fi

# Install and run vibe-kanban
echo "Installing vibe-kanban..."
npm install -g vibe-kanban@latest

echo "Starting vibe-kanban server..."
exec vibe-kanban