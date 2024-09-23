FROM orhunp/git-cliff:sha-c34aaa0 AS cliff

FROM git.opentalk.dev:5050/opentalk/backend/containers/rust:1.81.0-bookworm AS builder

WORKDIR /git-cliff-enhancer

COPY git-cliff-enhancer/Cargo.toml ./Cargo.toml
COPY git-cliff-enhancer/Cargo.lock ./Cargo.lock
COPY git-cliff-enhancer/src src

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo auditable build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=cliff /usr/local/bin/git-cliff /usr/local/bin/git-cliff
COPY --from=builder /git-cliff-enhancer/target/release/git-cliff-enhancer /usr/local/bin/git-cliff-enhancer

COPY cliff.toml /usr/local/etc/cliff.toml
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

ENV GIT_CLIFF_CONFIG=/usr/local/etc/cliff.toml

ENTRYPOINT ["bash", "/usr/local/bin/entrypoint.sh"]
