---
topic: Clew CLI — next implemented; next is path/editor or lint
updated_at: 2026-04-27T17:44:00Z
---

# Relay: Clew CLI — next implemented; next is path/editor or lint

## Status

`clew next` is implemented as the next vertical slice. It selects the first valid `#NNNN` reference from `.clew/path.md` when present, otherwise falls back to the oldest active `todo` increment by `created_at`; `--start` transitions the selected increment to `in_progress` using the same start behavior.

## Just finished

- Implemented `src/commands/next.rs`: root discovery, path-first selection, oldest-todo fallback, pipeable stdout (`NNNN\n`), and `--start` support.
- Wired `src/cli.rs` so `Command::Next { start }` passes the flag instead of ignoring it.
- Extracted reusable `start::start(&Path, query) -> u32` so `next --start` shares transition logic and warnings with `clew start`.
- Added `core::path::references()` for permissive `#NNNN` extraction from `path.md`.
- Added `ClewError::NoNextIncrement` for the empty-queue case.
- Added integration coverage for path priority, oldest-todo fallback, `next --start`, no-todo error, and stale path reference error.

## Next action

Ask the user to review the `clew next` behavior/output choice before commit, especially the decision that stdout is only the selected zero-padded ID (`0001`) rather than `#0001-slug` or full file content. If approved, run the full quality gate again after this relay update, then commit the code changes and `relay.md` together.

## Context worth carrying

- `clew next --start` prints the selected ID to stdout and `Started #NNNN` to stderr; this keeps stdout pipeable while preserving existing `start` UX.
- Path selection currently requires the referenced increment to be active and `status: todo`; archived refs return `ArchivedIncrement { action: "select" }`, and non-`todo` refs return `InvalidTransition { to: "next" }`. This is stricter than silently skipping path drift.
- Fallback ignores `backlog`, `in_progress`, terminal statuses, and archived files; ties on `created_at` break by filename ID.
- `path::references()` mirrors the existing path parser style: permissive and simple, first valid `#NNNN` per line.
- Quality gate already passed before this relay update: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. Re-run all three before committing because `relay.md` changed afterward.

## Open questions

- [Decide] Is `clew next` stdout as `NNNN` the desired long-term agent-facing API, or should it output canonical `#NNNN-slug` / full show content?
