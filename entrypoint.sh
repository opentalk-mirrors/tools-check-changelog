#!/bin/bash
set -e

git-cliff -c ./cliff.toml -o CHANGELOG.should.md --unreleased

./check_changelog.sh CHANGELOG.should.md CHANGELOG.md
