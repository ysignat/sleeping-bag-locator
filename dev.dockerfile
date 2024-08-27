ARG RUST_VERSION
ARG ALPINE_VERSION="3.20"

FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS build

RUN \
    apk update && apk upgrade \
    && apk add musl-dev

ENTRYPOINT [ "cargo", "run", "--" ]