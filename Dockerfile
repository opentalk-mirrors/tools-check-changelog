FROM git.opentalk.dev:5050/opentalk/tools/git-cliff:v2.7.0-ot.1 AS git-cliff
FROM git.opentalk.dev:5050/opentalk/backend/containers/rust:1.82.0-bookworm AS builder

WORKDIR /app

COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY ot-gitlab-cli/ ot-gitlab-cli/

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo auditable build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
RUN git config --global --add safe.directory /repository

WORKDIR /repository

COPY --from=builder /app/target/release/ot-gitlab-cli /usr/local/bin/ot-gitlab-cli
COPY --from=git-cliff /usr/local/bin/git-cliff /usr/local/bin/git-cliff

COPY comment-changelog-additions.sh /usr/local/bin/comment-changelog-additions.sh
COPY update-changelog.sh /usr/local/bin/update-changelog.sh

ENTRYPOINT ["bash", "/usr/local/bin/comment-changelog-additions.sh"]
