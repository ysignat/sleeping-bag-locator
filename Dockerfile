ARG ALPINE_VERSION='3.20'

FROM alpine:${ALPINE_VERSION}

ARG SRC_BINARY_PATH
ARG TGT_BINARY_PATH='/bin/app'

COPY --chmod=755 <<EOT /entrypoint.sh
#!/usr/bin/env sh
set -eu
${TGT_BINARY_PATH}
EOT

COPY --chmod=755 "${SRC_BINARY_PATH}" "${TGT_BINARY_PATH}"

ENTRYPOINT ["/entrypoint.sh"]