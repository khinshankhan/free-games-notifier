FROM rust:1-trixie AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM debian:trixie-slim
ARG SUPERCRONIC_VERSION=v0.2.51
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl libsqlite3-0 libssl3t64 tzdata \
 && arch="$(dpkg --print-architecture)" \
 && case "$arch" in \
      amd64) sha1=1692066e350a88ff4d7d6474fe3c7a6c41ec5e89 ;; \
      arm64) sha1=9d4c2000493215a1898502b2fc2dada3e8ea8c7a ;; \
      *) echo "unsupported arch: $arch" >&2; exit 1 ;; \
    esac \
 && curl -fsSLo /usr/local/bin/supercronic \
    "https://github.com/aptible/supercronic/releases/download/${SUPERCRONIC_VERSION}/supercronic-linux-${arch}" \
 && echo "${sha1}  /usr/local/bin/supercronic" | sha1sum -c - \
 && chmod +x /usr/local/bin/supercronic \
 && apt-get purge -y curl \
 && apt-get autoremove -y \
 && rm -rf /var/lib/apt/lists/*

COPY --from=build /src/target/release/free-games-notifier /usr/local/bin/

# offers.db is opened relative to the working directory, so /data must be a
# persistent volume. settings.toml and crontab are expected as mounted files
# in /etc/free-games-notifier/ (see the .example files). Set TZ to schedule in
# a timezone other than UTC.
WORKDIR /data
ENV NO_COLOR=1
CMD ["supercronic", "-passthrough-logs", "/etc/free-games-notifier/crontab"]
