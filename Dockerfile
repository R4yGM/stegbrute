FROM rust:alpine as builder

WORKDIR /usr/src/stegbrute
RUN cargo install stegbrute

FROM alpine:3.12
RUN apk add --no-cache --update && \
    apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing steghide
COPY --from=builder /usr/local/cargo/bin/stegbrute /usr/local/bin/stegbrute
ENTRYPOINT [ "/usr/local/bin/stegbrute" ]
