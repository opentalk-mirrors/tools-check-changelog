FROM orhunp/git-cliff:2.4.0 AS cliff

FROM debian:buster-slim

WORKDIR /app
COPY cliff.toml /app/cliff.toml

COPY check_changelog.sh /usr/local/bin/check_changelog.sh
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
COPY --from=cliff /usr/local/bin/git-cliff /usr/local/bin/git-cliff

ENTRYPOINT ["sh", "/usr/local/bin/entrypoint.sh"]
