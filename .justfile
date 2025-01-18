set export := true
set dotenv-load := true

CONTAINER_NAME := 'api'
ALPINE_VERSION := '3.20'
YC_PROFILE_NAME := 'sleeping-bag-locator-terraform'
AWS_REGION := 'ru-central1'
AWS_ACCESS_KEY_ID := env('AWS_ACCESS_KEY')
AWS_SECRET_ACCESS_KEY := env('AWS_SECRET_KEY')

default:
  @just --list

start:
  #!/usr/bin/env sh
  set -eu

  WORKDIR='/app'
  RUST_VERSION="$(grep 'rust-version' Cargo.toml | sed 's/rust-version = \"\(.*\)\"/\1/')"
  PORT='8080'
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
    --publish "${PORT}:${PORT}" \
    --workdir "${WORKDIR}" \
    --name "${CONTAINER_NAME}" \
    --env "HOST=0.0.0.0" \
    --env "PORT=${PORT}" \
    "${TAG}"

stop:
  #!/usr/bin/env sh
  set -eu

  docker kill "${CONTAINER_NAME}" || true

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

submit *args:
  #!/usr/bin/env sh
  set -eu

  GITHUB_TOKEN=${GITHUB_TOKEN}

  current_branch="$(git branch --show-current)"

  if [ "${current_branch}" = 'main' ]
  then
    git checkout -b "feature/${USER}/$(uuidgen)"
  fi

  git add "{{ args }}"
  git commit
  git push

  gh pr create --base main --web

delete:
  git branch --merged | grep -v 'main' | xargs --no-run-if-empty git branch --delete