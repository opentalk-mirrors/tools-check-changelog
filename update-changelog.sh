#!/bin/bash
set -e -o pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if ! command -v git-cliff-enhancer &> /dev/null; then
    echo "Error: git-cliff-enhancer is not installed or not found in PATH" >&2
    exit 1
fi

# Ensure the gitlab token is available
if [ -z "$GITLAB_TOKEN" ]; then
    echo "Error: GITLAB_TOKEN must be set and contain a valid GitLab access token with 'read_api' scope."
    exit 1
fi

# Check that the next version number is given as a cli argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <version>"
    echo "Description: This script requires the version for the new changelog section."
    exit 1
fi
NEXT_VERSION="$1"

# Use GITLAB_API_URL if present or CI_API_V4_URL from the gitlab CI
if [ -z "$GITLAB_API_URL" ] && [ -z "$CI_API_V4_URL" ]; then
    echo "Error: Either GITLAB_API_URL or CI_API_V4_URL must be set."
    exit 1
fi
export GITLAB_API_URL=${GITLAB_API_URL:-$CI_API_V4_URL}

if ! { [ -n "$GIT_CLIFF_CONFIG" ] && [ -f "$GIT_CLIFF_CONFIG" ]; } && [ ! -f "$SCRIPT_DIR/cliff.toml" ]; then
    echo "Error: Neither GIT_CLIFF_CONFIG is set to a valid file nor does $SCRIPT_DIR/cliff.toml exist."
    exit 1
fi
export GIT_CLIFF_CONFIG=${GIT_CLIFF_CONFIG:-$SCRIPT_DIR/cliff.toml}

# Use GITLAB_REPO if present or use CI_PROJECT_PATH from the gitlab CI
if [ -z "$GITLAB_REPO" ] && [ -z "$CI_PROJECT_PATH" ]; then
    echo "Error: Either GITLAB_REPO or CI_PROJECT_PATH must be set."
    exit 1
fi
export GITLAB_REPO=${GITLAB_REPO:-$CI_PROJECT_PATH}

# we need to unset the GITLAB variales otherwise the context will be overwritten
git-cliff --config "$GIT_CLIFF_CONFIG" --unreleased --tag "$NEXT_VERSION" --context \
    | git-cliff-enhancer -vvvv \
    | git-cliff --config "$GIT_CLIFF_CONFIG" --from-context - --unreleased --tag "$NEXT_VERSION" --prepend CHANGELOG.md
