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

# Get current user ID and group ID
USER_ID=$(id -u)
GROUP_ID=$(id -g)

echo "Current user ID: $USER_ID, Group ID: $GROUP_ID"
echo ""
echo "NOTE: Update docker-compose.yml with your user ID:"
echo "  user: \"$USER_ID:$GROUP_ID\""
echo ""
echo "Data directory initialized successfully!"
echo "You can now run: docker compose up -d"