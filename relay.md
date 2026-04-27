---
topic: Clew CLI — clew done shipped; abandon is next
updated_at: 2026-04-27T14:30:00Z
---

# Relay: Clew CLI — done shipped, abandon is next

## Status

`clew done` is complete end-to-end and committed as `1431a23`. Quality gate green after implementation: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Just finished

- **`clew done <id-or-slug>`** in `src/commands/done.rs`: resolves ID or slug, transitions `in_progress → done`, bumps `updated_at`, writes via the shared frontmatter round-trip path, archives from `.clew/increments/` to `.clew/archive/`, updates `path.md`, and prints `Done #NNNN` to stderr with empty stdout.
- **Self-loop tolerance for `done`**: if an increment is already `status: done` but still in `.clew/increments/`, `clew done` completes the archive move, emits `warning: #NNNN already marked done; completing archive`, and intentionally does **not** bump `updated_at`.
- **Shared transition helper** added in `src/commands/transition.rs`: handles read/parse/status validation/mutate/timestamp/write for read-mutate-write commands. `clew start` now uses it too.
- **Filesystem seams** added in `src/storage/fs.rs`: `archive_increment`, `read_path_md`, `write_path_md`.
- **`path.md` helpers** added in `src/core/path.rs`: removes done entries and normalizes remaining known references to current `#NNNN-slug` while preserving annotations.
- **`Done { id }` CLI arg widened** from `u32` to `String` in `src/cli.rs`, matching `start` and enabling slug lookup.
- **Tests added** for done happy path, invalid transition, slug lookup, path removal + normalization, self-loop tolerance without timestamp bump, and unknown-field/body preservation.

## Next action

Implement `clew abandon <id-or-slug> "reason"` as the next vertical slice. Reuse `commands::transition::apply`, but it needs to set `abandoned_reason` before serialization, so either extend the helper with a mutation closure or split the helper into a lower-level parsed-file transition seam. Side effects mirror `done`: archive the file and remove it from `path.md`; self-loop tolerance should warn `warning: #NNNN already marked abandoned; completing archive` and should not bump `updated_at`.

## Context worth carrying

- Commit `1431a23` is the stable point for `clew done`.
- `transition::apply` currently only changes `status` and `updated_at`. It returns `AppliedTransition { id, path, blocked_reason, self_loop }`. This was enough for `start` and `done`, but `abandon` likely needs the helper to support command-specific frontmatter mutation (`abandoned_reason`). Don’t shoehorn that logic into `done` or duplicate the old `start` body.
- `done` updates `path.md` **before** archiving because it scans entries to normalize remaining references; this currently still includes the soon-to-be-archived done file, which is harmless because its line has already been removed. Keep an eye on this ordering if extracting a shared archive-side-effect helper.
- `core::path::remove` is intentionally permissive and removes any line containing `#NNNN`. `core::path::normalize` only rewrites the first parseable `#NNNN[-slug]` reference per line. That matches current path format but is not a general markdown ref rewriter.
- `fs::archive_increment` uses `std::fs::rename`, not `git mv`, by design. Clew mutates project state; git workflow remains user/agent-owned.
- Output discipline remains: stdout = data; stderr = status/errors/warnings. Transition commands should keep stdout empty.
- Self-loop tolerance remains limited to terminal-side-effect transitions (`done`/`abandon`/`reopen`). `start` still rejects already-`in_progress`.
- Quality gate before the next commit/relay: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Drift from plan

- `path.md` normalization is implemented pragmatically, not as a full parser/writer. It handles current bullet-style `#NNNN-slug` entries and preserves trailing annotations. That is enough for the MVP; revisit only if real path formats require broader reference rewriting.
