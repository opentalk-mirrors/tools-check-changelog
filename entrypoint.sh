#!/bin/bash
set -ex

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

GIT_CLIFF_CONFIG=${GIT_CLIFF_CONFIG:-$SCRIPT_DIR/cliff.toml}

# we need to unset the GITLAB variales otherwise the context will be overwritten
git-cliff -c $GIT_CLIFF_CONFIG --unreleased --context \
    | git-cliff-enhancer -vvvv \
    | git-cliff -c $GIT_CLIFF_CONFIG --from-context - -o CHANGELOG.should.md

$SCRIPT_DIR/check_changelog.sh CHANGELOG.md CHANGELOG.should.md
