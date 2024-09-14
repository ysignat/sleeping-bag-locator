set export := true
set dotenv-load := true

CONTAINER_NAME := 'api'
ALPINE_VERSION := '3.20'

default:
  @just --list

start:
  #!/usr/bin/env sh
  set -eu

  WORKDIR='/app'
  RUST_VERSION="$(grep 'rust-version' Cargo.toml | sed 's/rust-version = \"\(.*\)\"/\1/')"
  TAG="$(
    docker build \
      --quiet \
      --file dev.dockerfile \
      --build-arg "RUST_VERSION=${RUST_VERSION}" \
      --build-arg "ALPINE_VERSION=${ALPINE_VERSION}" \
      .
  )"
  docker run \
    --rm \
    --user "$(id -u):$(id -g)" \
    --volume "${PWD}:${WORKDIR}" \
    --publish '8080:8080' \
    --workdir "${WORKDIR}" \
    --name "${CONTAINER_NAME}" \
    "${TAG}"

stop:
  #!/usr/bin/env sh
  set -eu

  docker kill "${CONTAINER_NAME}" || true

restart:
  just stop
  just start
