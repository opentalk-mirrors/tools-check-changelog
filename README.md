# Check Changelog

This tool ensures that the changelog is maintained as expected by git-cliff.

## How it works

This tool first creates the latest changelog section and stores the output in a
temporary file. Afterwards a script verifies that the actual changelog starts
with the computed parts.

## Why would we use this?

This tool helps to write commit messages that produce a readable changelog.
Without this tool, the changelog is updated only when the next release is created.
This usually leads to many adjustments that are required to ensure the changelog
is readable.

This tool makes it possible to check in the CI that each Merge Request updates the
changelog and that the changelog contains all changes that are part of the Merge Request.

## Usage

Include this script in your CI.
