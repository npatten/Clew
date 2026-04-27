---
topic: Clew CLI — clew list shipped; next is state transitions
updated_at: 2026-04-27T12:44:48Z
---

# Relay: Clew CLI — list shipped, transition write path next

## Status

`clew new` and `clew list` are complete end-to-end. Latest commit is `0d2f937 fix clew list discovery semantics`, on top of `669f200 implement clew list with status/tag/all filters` and `797cb1b implement clew new with slug, ID allocation, and parent validation`. Quality gate was green after the latest changes: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Just finished

- **`clew list`** in `src/commands/list.rs`: supports `--tag`, `--status`, and `--all`; sorts by ID ascending; emits pipeable lines as `NNNN status slug`.
- **Default list semantics fixed**: `clew list` now hides terminal statuses (`done`, `abandoned`) even if those files are still in `.clew/increments/`. `--all` includes archived files and terminal statuses.
- **Malformed frontmatter handling fixed** in `src/storage/fs.rs`: `scan_with_frontmatter()` now returns `ClewError::Frontmatter(...)` with the path instead of warning and silently skipping the broken increment. This exits as code `2` via `src/main.rs`.
- **Tests added** in `tests/integration_test.rs`: unarchived terminal statuses hidden by default, `--all` includes terminal statuses, malformed frontmatter fails with exit code `2`.

## Next action

Implement the first state-transition write path, preferably `clew start <id>` or a small shared helper that `promote`, `start`, `block`, `unblock`, `done`, `abandon`, and `reopen` can reuse. Start with `start`: resolve the increment, parse frontmatter, validate transition to `in_progress`, bump `updated_at` with whole-second UTC, serialize back preserving body and unknown fields, then add integration tests.

## Context worth carrying

- `fs::scan(root) -> Vec<FileEntry>` is filename-only and intentionally cheap. `fs::scan_with_frontmatter(root) -> Vec<LoadedEntry>` is the richer read path used by `list`; it now fails on parse errors.
- `fs::resolve(root, query)` already supports padded ID, unpadded ID, and slug lookup across `increments/` then `archive/`; use it for transition commands that take an ID/slug query.
- There is not yet a generic overwrite helper for existing increment files. Add one in `src/storage/fs.rs` rather than writing directly from each command.
- Timestamp format invariant: RFC3339 UTC with `Z`, second precision, no subseconds. `clew new` currently creates this by round-tripping `Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true).parse()`.
- Preserve unknown frontmatter fields and body verbatim. `frontmatter::parse` + `frontmatter::serialize` are the intended route.
- `clew new` stdout is just padded ID + newline. `clew list` stdout is data only. Keep status/progress/errors on stderr.
- Quality gate before each milestone: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Open questions

- [Decide] Whether transition commands should accept only numeric IDs as currently sketched in `src/cli.rs`, or match `show` and accept slug queries too. The design plan says `clew show` accepts slug; transition commands are sketched as `<id>`, but agent ergonomics may favor ID-or-slug consistently.
- [Decide] Whether to implement transitions one command at a time (`start` first) or create the shared read-mutate-write helper up front. Lean: do `start` as the vertical slice, extract helper only where it avoids duplication for the next command.
