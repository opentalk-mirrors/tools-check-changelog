#!/bin/bash

# This script takes a specific project and merge request as input, clones the
# project, retrieves the changelog entries introduced by that merge request, and
# posts them as a comment on the same merge request.

set -e

for cmd in opentalk-git-cliff ot-gitlab-cli git awk; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is not installed or not found in PATH" >&2
        exit 1
    fi
done

# Expected variables
env_vars=("TARGET_REPO" "SOURCE_REPO" "GITLAB_MR" "SOURCE_BRANCH" "GITLAB_TOKEN")

# Loop through the list and check each variable
for var in "${env_vars[@]}"; do
    if [ -z "${!var}" ]; then
        echo "Error: Environment variable $var is not set." >&2
        exit 1 # Exit with error if a variable is not set
    fi
done

export GITLAB_MR_REF="$TARGET_REPO!$GITLAB_MR"
export GITLAB_API_URL=${GITLAB_API_URL:-$CI_API_V4_URL}
TARGET_BRANCH="main"

echo " Creating changelog notification for
Merge Request: $GITLAB_MR_REF
Target Repository: $TARGET_REPO
Target Branch: $TARGET_BRANCH
Source Repository: $SOURCE_REPO
Source Branch: $SOURCE_BRANCH

git-cliff config: ${GIT_CLIFF_CONFIG:-"<Default Config>"}
"

# Don't include any header in the output
export GIT_CLIFF__CHANGELOG__HEADER=""

REPO_REMOTE=$( ot-gitlab-cli project authorized-clone-url -p "$TARGET_REPO" )
REPO_MR_REMOTE=$( ot-gitlab-cli project authorized-clone-url -p "$SOURCE_REPO" )

# Temporary file for the changelog output
temp_file=$( mktemp )

# Temporary folder for the git repository
temp_dir=$( mktemp -d )

pushd "$temp_dir"

# Setup the repository. Don't download any files, we only need the commit history.
# Add the fork and fetch the branch that should get merged.
git clone --no-checkout "$REPO_REMOTE" .
git remote add mr-remote "$REPO_MR_REMOTE"
git fetch mr-remote "$SOURCE_BRANCH"

# We assume that the target branch is always main.
TARGET_BRANCH=$( git rev-parse --abbrev-ref "$TARGET_BRANCH@{u}" )

export GITLAB_REPO="$TARGET_REPO"
RUST_LOG=git_cliff=debug,opentalk=debug opentalk-git-cliff \
    -o "$temp_file" \
    --tag hidden \
    "$TARGET_BRANCH..mr-remote/$SOURCE_BRANCH"

GITLAB_CLI_OPTIONS=()
SHOULD_RESOLVE=$(ot-gitlab-cli merge-request get-labels -f json | jq '. | any(contains("Maintenance"))')
if [ "$SHOULD_RESOLVE" == "true" ]; then
    echo "Mark discussion as resolved. This is a maintenance MR."
    GITLAB_CLI_OPTIONS+=(--resolve)
fi

# We prepend every line with `> ` using awk
COMMENT="This MR will add the following changelog entries:

$(awk '{print "> "$0}' < "$temp_file")

If you are happy with the changelog entry, resolve this thread.

* [How to write commit messages?](https://git.opentalk.dev/opentalk/tools/check-changelog/-/blob/main/doc/commit-message-format.md)
* Visit the [changelog bot repository](https://git.opentalk.dev/opentalk/tools/check-changelog)
* [Report an issue or request a feature](https://git.opentalk.dev/opentalk/tools/check-changelog/-/issues/new)
"

if [ -n "$DRY_RUN" ]; then
    echo "[DRY RUN] Would post the following comment:"
    echo -e "$COMMENT"
else
    echo -e "$COMMENT" | ot-gitlab-cli discussion put-latest -vv --input - "${GITLAB_CLI_OPTIONS[@]}"
fi

popd

rm -f "$temp_file"
rm -rf "$temp_dir"
