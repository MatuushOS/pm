# Maintainer: Matus Mastena <Shadiness9530@pm.me>
FROM alpine:latest as builder
WORKDIR .
RUN ["apk", "update"]
RUN ["apk", "add", "clang", "rust", "cmake", "cargo"]
RUN ["cargo", "build", "--release"]
FROM busybox:musl as host_os
COPY "target/debug/pm" "/sbin"