FROM git.opentalk.dev:5050/opentalk/backend/containers/rust:1.94.0-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY ot-gitlab-cli ot-gitlab-cli
COPY opentalk-git-cliff opentalk-git-cliff
COPY git-cliff-gitlab git-cliff-gitlab
COPY git-cliff-config git-cliff-config

RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo auditable build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y git jq && rm -rf /var/lib/apt/lists/*
RUN git config --global --add safe.directory /repository

WORKDIR /repository

COPY --from=builder /app/target/release/ot-gitlab-cli /usr/local/bin/ot-gitlab-cli
COPY --from=builder /app/target/release/opentalk-git-cliff /usr/local/bin/opentalk-git-cliff

COPY git-cliff-config /usr/local/etc/git-cliff/

COPY comment-changelog-additions.sh /usr/local/bin/comment-changelog-additions.sh
COPY update-changelog.sh /usr/local/bin/update-changelog.sh

ENTRYPOINT ["bash", "/usr/local/bin/comment-changelog-additions.sh"]
