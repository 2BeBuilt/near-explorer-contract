docker run \
     --mount type=bind,source=${HOST_DIR:-$(pwd)/..},target=/host \
     --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
     -i -t nearprotocol/contract-builder:latest-amd64 \
     /bin/bash