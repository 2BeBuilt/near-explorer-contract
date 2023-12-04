#!/bin/bash

DEFAULT_IMAGE="nearprotocol/contract-builder:latest-amd64"

IMAGE_NAME=${1:-$DEFAULT_IMAGE}

docker run \
    --mount type=bind,source=${HOST_DIR:-$(pwd)/..},target=/host \
    --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
    -i -t $IMAGE_NAME \
    /bin/bash
