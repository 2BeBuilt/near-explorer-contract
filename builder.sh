#!/bin/bash

# Default image and container names
DEFAULT_IMAGE="nearprotocol/contract-builder:latest-amd64"
CONTAINER_NAME="sourcescan-builder-rust"

# Use the first argument as the image name, or default if no argument is provided
IMAGE_NAME=${1:-$DEFAULT_IMAGE}

# Function to check if a container exists
container_exists() {
  docker ps -a --format '{{.Names}}' | grep -qw $CONTAINER_NAME
}

# Function to get the image of an existing container
get_container_image() {
  docker inspect --format='{{.Config.Image}}' $CONTAINER_NAME
}

# Check if the container exists
if container_exists; then
  echo "Container $CONTAINER_NAME already exists."
  
  # Check if the existing container was created with a different image
  if [ "$(get_container_image)" != "$IMAGE_NAME" ]; then
    echo "Existing container was created with a different image. Updating image..."
    docker stop $CONTAINER_NAME
    docker rm $CONTAINER_NAME
    docker run \
        --name $CONTAINER_NAME \
        --mount type=bind,source="$(pwd)",target=/host \
        --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
        -it $IMAGE_NAME \
        /bin/bash
  else
    echo "Reusing existing container with the same image."
    docker start -ai $CONTAINER_NAME
  fi
else
  echo "Creating a new container..."
  docker run \
      --name $CONTAINER_NAME \
      --mount type=bind,source="$(pwd)",target=/host \
      --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
      -it $IMAGE_NAME \
      /bin/bash
fi
