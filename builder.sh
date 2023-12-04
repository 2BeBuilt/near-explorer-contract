#!/bin/bash

CONTAINER_NAME="sourcescan-builder-rust"

# Check if the container already exists
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Container with name ${CONTAINER_NAME} already exists."

    # Check if the container is running
    if docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo "Container is already running."
    else
        echo "Starting existing container."
        docker start "${CONTAINER_NAME}"
        docker exec -it "${CONTAINER_NAME}" /bin/bash
    fi
else
    echo "Creating and starting a new container."
    docker run \
        --name "${CONTAINER_NAME}" \
        --mount type=bind,source=${HOST_DIR:-$(pwd)/..},target=/host \
        --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
        -i -t nearprotocol/contract-builder:latest-amd64 \
        /bin/bash
fi
