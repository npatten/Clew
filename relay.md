---
topic: Clew CLI — clew list shipped; next is state transitions
updated_at: 2026-04-27T13:00:00Z
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

Implement `clew start` as the first state-transition vertical slice. Steps:

1. Widen the CLI arg from `u32` to `String` (in `src/cli.rs`) so it accepts ID-or-slug, matching `show`. Same widening applies to all other transition commands — do them as we ship each, not pre-emptively.
2. Resolve via `fs::resolve(root, query)` (already supports padded ID, unpadded ID, slug across both subdirs).
3. Parse with `frontmatter::parse`, validate transition to `in_progress` (use existing `ClewError::InvalidTransition`), bump `updated_at` via the `Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true).parse()` round-trip pattern from `commands::new`.
4. Add `fs::write_increment(path, contents)` as the existing-file overwrite seam — symmetric with `write_new_increment`. Define it now so every transition command shares one write path.
5. Serialize via `frontmatter::serialize` (preserves body + unknown fields by construction).
6. Integration tests: happy path, invalid transition, ID and slug lookup, unknown ID.

Do `start` end-to-end before extracting any shared `transition()` helper — second use of the read-mutate-write pattern is the right time to extract, not the first.

## Context worth carrying

- `fs::scan(root) -> Vec<FileEntry>` is filename-only and intentionally cheap. `fs::scan_with_frontmatter(root) -> Vec<LoadedEntry>` is the richer read path used by `list`; it now fails on parse errors.
- `fs::resolve(root, query)` already supports padded ID, unpadded ID, and slug lookup across `increments/` then `archive/`; use it for transition commands that take an ID/slug query.
- There is not yet a generic overwrite helper for existing increment files. Plan: add `fs::write_increment(path, contents)` in `src/storage/fs.rs`, symmetric with `write_new_increment`. Every transition command should go through it — keep `commands/` from touching the filesystem directly.
- Timestamp format invariant: RFC3339 UTC with `Z`, second precision, no subseconds. `clew new` currently creates this by round-tripping `Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true).parse()`.
- Preserve unknown frontmatter fields and body verbatim. `frontmatter::parse` + `frontmatter::serialize` are the intended route.
- `clew new` stdout is just padded ID + newline. `clew list` stdout is data only. Keep status/progress/errors on stderr.
- Quality gate before each milestone: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Decisions locked in this session

- **Transition commands accept ID-or-slug**, matching `show`. The current `<id>: u32` typing in `src/cli.rs` for `Promote`/`Start`/`Block`/`Unblock`/`Done`/`Abandon`/`Reopen`/`Renumber` should be widened to `String` and resolved via `fs::resolve`. Widen as each command ships.
- **One vertical slice at a time, no upfront shared helper.** Do `clew start` end-to-end first; extract a shared `transition()` only on the second use, where duplication is concrete.
