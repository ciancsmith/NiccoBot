#!/bin/bash

# Check if at least one argument is passed; if not, provide a usage message
if [ $# -lt 2 ]; then
    echo "Usage: $0 <tag> <platform1> [platform2] ..."
    echo "Example: $0 1.0.0 linux/amd64 linux/arm64"
    exit 1
fi

# Assign the first argument to TAG variable
TAG=$1

# Remove the first argument (tag) so only platforms remain
shift

# Join the remaining arguments (platforms) into a comma-separated string
PLATFORMS=$(printf ",%s" "$@")
PLATFORMS=${PLATFORMS:1}

# Use Docker Buildx to build and push the image
DOCKER_BUILDKIT=1 docker buildx build --platform "$PLATFORMS" -t "crowlc/niccobot:$TAG" . --push