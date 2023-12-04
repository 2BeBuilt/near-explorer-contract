#!/bin/bash

# Default image and container names
DEFAULT_IMAGE="nearprotocol/contract-builder:latest-amd64"
CONTAINER_NAME="sourcescan-builder-rust"

# Use the first argument as the image name, or default if no argument is provided
IMAGE_NAME=${1:-$DEFAULT_IMAGE}

# Function to check if a container exists
container_exists() {
  docker ps -a --format '{{.Names}}' | grep -w $CONTAINER_NAME > /dev/null 2>&1
}

# Function to get the image of an existing container
get_container_image() {
  docker inspect --format='{{.Config.Image}}' $CONTAINER_NAME
}

# Check if the container exists
if container_exists; then
  # Check if the existing container was created with a different image
  if [ "$(get_container_image)" != "$IMAGE_NAME" ]; then
    echo "Container exists with a different image, recreating..."

    # Stop and remove the existing container
    docker stop $CONTAINER_NAME > /dev/null 2>&1
    docker rm $CONTAINER_NAME > /dev/null 2>&1
  else
    echo "Container already exists with the same image, remounting..."
  fi
fi

# Run the Docker container
docker run \
    --name $CONTAINER_NAME \
    --mount type=bind,source=${HOST_DIR:-$(pwd)/..},target=/host \
    --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
    -i -t $IMAGE_NAME \
    /bin/bash
