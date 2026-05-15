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

- [ ] Add `scripts/release-preflight vX.Y.Z` (local-only) that asserts clean git tree, version match in `Cargo.toml`, `## [X.Y.Z]` entry in `CHANGELOG.md`, then runs `dist generate --check`, `dist plan --tag vX.Y.Z`, `cargo publish --dry-run`, and `scripts/promote-clew`. Do not pass `--allow-dirty` to any check; the preceding clean-tree assertion makes it both unnecessary and harmful (it masks the exact class of bug those checks exist to catch).
- [ ] Add `scripts/release-smoketest vX.Y.Z` covering `cargo install clew --version X.Y.Z --force`, running the shell installer in a scratch dir, and asserting `clew --version` output contains `X.Y.Z` (not just exit code zero — that catches a stale install on PATH).
- [ ] Add the Homebrew leg of the smoke test as a CI job on a macOS runner rather than in the local script, since the releaser may not be on macOS. `brew install npatten/tap/clew && clew --version | grep X.Y.Z`.
- [ ] Add a CHANGELOG-presence job to `.github/workflows/release.yml` on tag push that fails if no matching `## [X.Y.Z]` block exists. Belt-and-suspenders with the preflight script.
- [ ] Add "Recovery" section to `docs/release.md` covering `cargo yank` and the bump-and-re-release path. crates.io versions are immutable; recovery is always forward, never overwrite.
- [ ] Update `docs/release.md` to point at the new scripts and drop `--allow-dirty` from the documented checklist.
- [ ] Quality gate via `scripts/promote-clew`

## Notes

Shell preamble: `set -euo pipefail`. Quote every expansion. One explicit check per command rather than clever pipelines.

Caveat on `-e`: it does NOT fire inside `if` conditions, the left side of `&&`/`||`, or commands whose output is being captured by `$(...)`. Do not trust it as a universal safety net. Where exit status really matters, check it explicitly.

If the preflight script grows past ~100 lines, port to a small Rust binary in `dev-tools/` rather than letting bash sprawl. Shell error handling is bad and a quietly-broken preflight is worse than no preflight.

Dependency on #0040: once trusted publishing lands, the `cargo publish --dry-run` step in preflight may shrink or disappear (CI owns the actual publish). Whoever does these second should sweep for consistency.
