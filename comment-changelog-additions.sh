#!/bin/bash

# This script adds a comment with the added changelog entries

set -e

for cmd in git-cliff git-cliff-enhancer ot-gitlab-cli; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is not installed or not found in PATH" >&2
        exit 1
    fi
done

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

export GIT_CLIFF_CONFIG=${GIT_CLIFF_CONFIG:-$SCRIPT_DIR/cliff.toml}

# Use GITLAB_API_URL if present or CI_API_V4_URL from the gitlab CI
export GITLAB_API_URL=${GITLAB_API_URL:-$CI_API_V4_URL}

# Use GITLAB_REPO if present or build CI_PROJECT_PATH from the gitlab CI
export GITLAB_REPO=${GITLAB_REPO:-$CI_PROJECT_PATH}
export GITLAB_MR=${GITLAB_MR:-$CI_MERGE_REQUEST_IID}
export GITLAB_MR_REF="$GITLAB_REPO!$GITLAB_MR"

# Don't include any header in the output
export GIT_CLIFF__CHANGELOG__HEADER=""

temp_file=$(mktemp)

# we need to unset the GITLAB variales otherwise the context will be overwritten
MAIN=$( git rev-parse --abbrev-ref 'main@{u}' )
git-cliff --config "$GIT_CLIFF_CONFIG" --context "$MAIN"..HEAD \
    | git-cliff-enhancer -vvvv \
    | git-cliff --config "$GIT_CLIFF_CONFIG" --from-context - -o "$temp_file"

echo -e "This MR will add the following changelog entries:

\`\`\`md
$(<"$temp_file")
\`\`\`
" | ot-gitlab-cli put-latest -vv -i -

rm -f "$temp_file"
