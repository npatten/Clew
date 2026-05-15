# Changelog

All notable user-facing changes to Clew are documented here.

## [0.1.3] - 2026-05-15

### Added

- `clew list` now orders active increments by path rank when path metadata is available.
- `clew new --help` documents stdin/heredoc body creation.
- New Pi prompt helpers for standard and worktree-based agent workflows.

### Changed

- `clew init` now generates a clearer bootstrap increment and agent README contract.
- Project docs now standardize on the installed `clew` command, keeping `./clew` for explicit local promoted-build testing.
- Bugs are represented as tags for now, avoiding a separate bug status/model until the workflow needs it.

### Fixed

- `clew done` can now complete backlog increments.

[0.1.3]: https://github.com/npatten/Clew/compare/v0.1.2...v0.1.3
