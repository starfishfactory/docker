#!/bin/bash
# Initialize data directory with proper permissions

echo "Initializing vibe-kanban data directory..."

# Create data directory if it doesn't exist
if [ ! -d "./data" ]; then
    echo "Creating data directory..."
    mkdir -p ./data
fi

# Set permissions for data directory
echo "Setting permissions for data directory..."
chmod 777 ./data

echo "Data directory initialized successfully!"
echo "You can now run: docker compose up -d"