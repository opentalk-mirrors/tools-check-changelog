#!/bin/bash
set -ex

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

export GIT_CLIFF_CONFIG=${GIT_CLIFF_CONFIG:-$SCRIPT_DIR/cliff.toml}

# Use GITLAB_API_URL if present or CI_API_V4_URL from the gitlab CI
export GITLAB_API_URL=${GITLAB_API_URL:-$CI_API_V4_URL}

# Use GITLAB_REPO if present or build CI_PROJECT_PATH from the gitlab CI
export GITLAB_REPO=${GITLAB_REPO:-$CI_PROJECT_PATH}

# we need to unset the GITLAB variales otherwise the context will be overwritten
git-cliff --config "$GIT_CLIFF_CONFIG" --unreleased --context \
    | git-cliff-enhancer -vvvv \
    | git-cliff --config "$GIT_CLIFF_CONFIG" --from-context - --unreleased --prepend CHANGELOG.md
