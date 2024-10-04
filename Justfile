CONTAINER_CMD := "podman"
build:
    cargo build --workspace --release
    ${CONTAINER_CMD} build .
    