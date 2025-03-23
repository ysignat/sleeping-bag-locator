set export := true
set dotenv-load := true

CONTAINER_NAME := 'api'
REDIS_CONTAINER_NAME := 'redis'
NETWORK_NAME := 'app-network'
YC_PROFILE_NAME := 'sleeping-bag-locator-terraform'
AWS_REGION := 'ru-central1'
AWS_ACCESS_KEY_ID := env('AWS_ACCESS_KEY')
AWS_SECRET_ACCESS_KEY := env('AWS_SECRET_KEY')
OAUTH_CLIENT_ID := env('OAUTH_CLIENT_ID')
OAUTH_CLIENT_SECRET := env('OAUTH_CLIENT_SECRET')

default:
  @just --list

start:
  #!/usr/bin/env sh
  set -eu

  docker network create \
    --driver bridge \
    "${NETWORK_NAME}"

  docker run \
    --rm \
    --detach \
    --name "${REDIS_CONTAINER_NAME}" \
    --network "${NETWORK_NAME}" \
    redis:7

  WORKDIR='/app'
  RUST_VERSION="$(grep 'rust-version' Cargo.toml | sed 's/rust-version = \"\(.*\)\"/\1/')"
  PORT='8080'
  TAG="$(
    docker build \
      --quiet \
      --file dev.dockerfile \
      --build-arg "RUST_VERSION=${RUST_VERSION}" \
      --build-arg "ALPINE_VERSION=$(gh variable get ALPINE_VERSION)" \
      .
  )"
  docker run \
    --rm \
    --detach \
    --user "$(id -u):$(id -g)" \
    --volume "${PWD}:${WORKDIR}" \
    --publish "${PORT}:${PORT}" \
    --workdir "${WORKDIR}" \
    --name "${CONTAINER_NAME}" \
    --network "${NETWORK_NAME}" \
    --env "HOST=0.0.0.0" \
    --env "PORT=${PORT}" \
    --env "OAUTH_CLIENT_ID=${OAUTH_CLIENT_ID}" \
    --env "OAUTH_CLIENT_SECRET=${OAUTH_CLIENT_SECRET}" \
    --env "SESSION_STORE_TYPE=redis" \
    --env "SESSION_STORE_DSN=redis://redis:6379" \
    --env "LOG_LEVEL=TRACE" \
    "${TAG}"

stop:
  #!/usr/bin/env sh
  set -eu

  docker container rm \
    --force \
    --volumes \
    "${CONTAINER_NAME}"

  docker container rm \
    --force \
    --volumes \
    "${REDIS_CONTAINER_NAME}"

  docker network rm \
    --force \
    "${NETWORK_NAME}" 

restart:
  just stop
  just start

_yc-iam:
  yc config profile activate "${YC_PROFILE_NAME}" > /dev/null
  yc iam create-token

tf target *args:
  YC_TOKEN=$(just _yc-iam) terraform -chdir={{ target }} {{ args }}

aws *args:
  poetry run aws {{ args }}
