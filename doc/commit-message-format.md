# Commit Message Format

Commit messages follow the [Conventional Commit specification][CC]. This is enforced by a CI check (using [`commitlint`][commit-lint]). Since the Conventional Commit specification is quite flexible, most of the rules are fixed by the configuration of `commitlint`. We use the [default `commitlint` configuration][commit-lint-config].

## Structure

The structure is as follows:

```txt
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

The combination of `type` and `scope` is used to map a commit to a specific changelog section.

The `footer` can contain special information (e.g. issue number). Learn more in the [footer section](#Footer).

## Types & scopes

The valid types are limited to the following list:

- `chore` – repetitive task, housekeeping
- `ci` – changes to the ci pipeline
- `docs` – documentation
- `feat` – Code change which fulfills new requirements, adds new features
- `fix` – code change that adjust the behavior to match already existing requirements
- `refactor` – changes to code without changing behavior
- `test` – changed tests, no change in application code

The following types are also allowed but used less ofter.

- `perf` – no behavior change, but performance improvement
- `revert` – `git revert` or other undo action
- `style`
- `build`

## Changelog Section

- 📦 Dependencies
    - `chore(deps)`
    - `fix(deps)`
- 🚀 New features
    - type: `feat`
    - exception:
        - `feat(ux)` (see UX section)
- 🥰 User experience
    - `feat(ux)`
    - `fix(ux)`
    - `chore(ux)`
- 🛡️ Security
    - includes everything that contains the word `security`
- 🐛 Bug fixes
    - type: `fix`
    - exception:
        - `fix(deps)` (see Dependency section)
        - `fix(ux)` (see UX section)
- ⚡ Performance
    - type: `pref`
- 📚 Documentation
    - type: `docs`
- 🔨 Refactor
    - type: `refactor`
- ✨ Style
    - type: `style`
- ⚙ Miscellaneous
    - type: `chore`
    - exception:
        - `chore(deps)` (see Dependency section)
        - `chore(ux)` (see UX section)
        - `chore(release)` (excluded from changelog)

## Footer

The changelog entry can be changed using special footer entries:

- `changelog: ignore` – exclude this commit from the changelog
    - this is useful for commits that are not note worthy in the changelog
- `closes #<issue number>` – add a link to the issue number that is closed by this commit.

[CC]: https://www.conventionalcommits.org/en/v1.0.0/
[commit-lint]: https://commitlint.js.org/
[commit-lint-config]: https://github.com/conventional-changelog/commitlint/blob/master/@commitlint/config-conventional/src/index.ts
