---
id: 39
status: backlog
tags:
- dev-tools
- release
created_at: 2026-05-15T05:16:33Z
updated_at: 2026-05-15T05:16:33Z
---
## Goal

Reduce manual error in the release flow with two scripts and a CHANGELOG presence check, plus add a recovery section to `docs/release.md`.

## Tasks

- [ ] Add `scripts/release-preflight vX.Y.Z` that asserts clean git tree, version match in `Cargo.toml`, `## [X.Y.Z]` entry in `CHANGELOG.md`, then runs `dist generate --check`, `dist plan --tag vX.Y.Z`, `cargo publish --dry-run`, and `scripts/promote-clew` (no `--allow-dirty` once tree is clean)
- [ ] Add `scripts/release-smoketest vX.Y.Z` covering `cargo install --version`, shell installer in a scratch dir, and `clew --version` check
- [ ] Add a CHANGELOG-presence job to `.github/workflows/release.yml` on tag push that fails if no matching `## [X.Y.Z]` block exists
- [ ] Add "Recovery" section to `docs/release.md` covering `cargo yank` and the bump-and-re-release path
- [ ] Update `docs/release.md` to point at the new scripts
- [ ] Quality gate via `scripts/promote-clew`

## Notes

Shell preamble: `set -euo pipefail`. Quote everything. If preflight grows past ~100 lines, port to a small Rust binary in `dev-tools/` rather than letting bash sprawl.
