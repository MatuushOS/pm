CONTAINER_CMD := podman
COPY := {pm,mkrepo,shapkg}
build:
    cargo build --workspace --release
    ${CONTAINER_CMD} build .
    