FROM git.opentalk.dev:5050/opentalk/backend/containers/rust:1.82.0-bookworm AS builder

WORKDIR /ck-changelog

COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY git-cliff-enhancer/ git-cliff-enhancer/
COPY ot-gitlab-cli/ ot-gitlab-cli/

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo auditable build --release

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo install --root /usr/local git-cliff@2.6.1

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
RUN git config --global --add safe.directory /repository

WORKDIR /repository

COPY --from=builder /usr/local/bin/git-cliff /usr/local/bin/git-cliff
COPY --from=builder /ck-changelog/target/release/git-cliff-enhancer /usr/local/bin/git-cliff-enhancer
COPY --from=builder /ck-changelog/target/release/ot-gitlab-cli /usr/local/bin/ot-gitlab-cli

COPY cliff.toml /usr/local/etc/cliff.toml
COPY comment-changelog-additions.sh /usr/local/bin/comment-changelog-additions.sh
COPY update-changelog.sh /usr/local/bin/update-changelog.sh

ENV GIT_CLIFF_CONFIG=/usr/local/etc/cliff.toml

ENTRYPOINT ["bash", "/usr/local/bin/comment-changelog-additions.sh"]
