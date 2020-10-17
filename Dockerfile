FROM rust:alpine as builder
RUN apk add --no-cache build-base

WORKDIR /usr/src/stegbrute
RUN cargo install stegbrute

FROM alpine:latest
RUN apk add --no-cache --update && \
    apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing steghide
COPY --from=builder /usr/local/cargo/bin/stegbrute /usr/local/bin/stegbrute
ENTRYPOINT [ "/usr/local/bin/stegbrute" ]
