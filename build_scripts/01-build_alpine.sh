#!/bin/sh
apk add alpine-sdk gcc cmake rustup libarchive libarchive git pkgconf
rustup-init -y
. ~/.cargo/env
rustup default stable & rustup toolchain add $(arch)-unknown-linux-musl
git clone https://github.com/MatuushOS/pm mtos
cd mtos
cargo build --workspace --release