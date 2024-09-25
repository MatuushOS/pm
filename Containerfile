# Maintainer: Matus Mastena <Shadiness9530@pm.me>
FROM alpine:latest as builder
WORKDIR .
RUN ["apk", "update"]
RUN ["apk", "add", "clang", "rust", "cmake", "cargo"]
COPY "../mtos-docker" "/"
WORKDIR "/mtos-docker"
RUN ["cargo", "build", "--release"]
FROM busybox:musl as host_os
COPY "target/release/pm" "/usr/bin"
COPY "target/release/shapkg" "/usr/bin"