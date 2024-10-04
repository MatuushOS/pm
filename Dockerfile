FROM alpine:latest as builder
WORKDIR .
COPY "build_scripts/01-build_alpine.sh" "01-build_alpine.sh"
RUN ["sh", "01-build_alpine.sh"]

# FROM busybox:musl as host
# WORKDIR .
# COPY "build_scripts/02-copy_to_bb.sh" "02-copy_to_bb.sh" 
# COPY "build_scripts/03-set_path.sh" "03-set_path.sh"
# RUN ["sh", "02-copy_to_bb.sh"]
# RUN ["sh", "03-set_path.sh"]