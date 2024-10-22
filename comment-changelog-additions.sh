#!/bin/bash

# This script takes a specific project and merge request as input, clones the
# project, retrieves the changelog entries introduced by that merge request, and
# posts them as a comment on the same merge request.

set -e

for cmd in git-cliff git-cliff-enhancer ot-gitlab-cli git awk; do
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
        exit 1  # Exit with error if a variable is not set
    fi
done

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

export GIT_CLIFF_CONFIG=${GIT_CLIFF_CONFIG:-$SCRIPT_DIR/cliff.toml}
export GITLAB_MR_REF="$TARGET_REPO!$GITLAB_MR"
export GITLAB_API_URL=${GITLAB_API_URL:-$CI_API_V4_URL}
TARGET_BRANCH="main"
echo " Creating changelog notification for
Merge Request: $GITLAB_MR_REF
Target Repository: $TARGET_REPO
Target Branch: $TARGET_BRANCH
Source Repository: $SOURCE_REPO
Source Branch: $SOURCE_BRANCH
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
git-cliff-enhancer -vv \
    --config "$GIT_CLIFF_CONFIG" \
    -o "$temp_file" \
    "$TARGET_BRANCH..mr-remote/$SOURCE_BRANCH"

# We prepand every line with `> ` using awk
echo -e "This MR will add the following changelog entries:

$(awk '{print "> "$0}' < "$temp_file")

Visit the [changelog bot repository](https://git.opentalk.dev/opentalk/tools/check-changelog/-/blob/main/README.md)
for more information or [open an issue](https://git.opentalk.dev/opentalk/tools/check-changelog/-/issues/new)
if you encounter any problems.
" | ot-gitlab-cli discussion put-latest -vv -i -

popd

rm -f "$temp_file"
rm -rf "$temp_dir"
