FROM orhunp/git-cliff:sha-c34aaa0 AS cliff

FROM git.opentalk.dev:5050/opentalk/backend/containers/rust:1.79.0-bookworm AS builder

WORKDIR /git-cliff-enhancer

COPY git-cliff-enhancer/Cargo.toml ./Cargo.toml
COPY git-cliff-enhancer/Cargo.lock ./Cargo.lock
COPY git-cliff-enhancer/src src

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo auditable build --release

FROM debian:buster-slim

WORKDIR /app
COPY cliff.toml /app/cliff.toml
COPY --from=cliff /usr/local/bin/git-cliff /usr/local/bin/git-cliff

COPY --from=builder /git-cliff-enhancer/target/release/git-cliff-enhancer /usr/local/bin/git-cliff-enhancer

COPY check_changelog.sh /usr/local/bin/check_changelog.sh
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

ENTRYPOINT ["sh", "/usr/local/bin/entrypoint.sh"]
