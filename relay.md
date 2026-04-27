---
topic: Clew CLI — abandon shipped; reopen is next
updated_at: 2026-04-27T15:20:00Z
---

# Relay: Clew CLI — abandon shipped, reopen is next

## Status

`clew abandon <id-or-slug> "reason"` is shipped in `149178f`. Quality gate was green before the commit: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Just finished

- Implemented `clew abandon` as the next terminal transition: accepts ID or slug, allows `backlog|todo|in_progress|done → abandoned`, records `abandoned_reason`, bumps `updated_at`, preserves unknown frontmatter/body, archives the increment, removes it from `path.md`, keeps stdout empty, and prints status/warnings to stderr.
- Added `transition::apply_with` so commands can supply command-specific frontmatter mutations without growing one-off result fields.
- Added integration tests for normal abandon, slug lookup, path cleanup, self-loop archive completion, already-archived abandoned no-op warning, and preservation behavior.

## Next action

Implement `clew reopen <id-or-slug>` as the next vertical slice. It should resolve archive/increments, transition `done|abandoned → todo`, move archived files back to `.clew/increments/`, bump `updated_at` on real transition, and tolerate already-unarchived `todo` only if that matches the direct-edit reconciliation story we want. Decide whether reopening an abandoned item should clear or preserve `abandoned_reason`; the design currently says `abandoned_reason` is preserved through archive/reopen, so default to preserving it unless we revise the plan.

## Context worth carrying

- Stable commits now: `149178f` (`clew abandon`), `2f7c58b` (done cleanup), `1431a23` (`clew done`).
- `transition::apply_with` intentionally does not mutate on tolerated self-loops. This preserves the existing `done` self-loop behavior: complete archive side effects without bumping timestamps or rewriting frontmatter.
- `abandon` mirrors `done` side-effect ordering: prepare path changes in memory, archive first, then write `path.md`. Archive correctness beats advisory path cleanup if a later write fails.
- `fs::archive_increment` exists, but there is not yet a symmetric unarchive helper for `reopen`.
- Transition commands still follow stdout=data, stderr=status/errors/warnings.
- Full quality gate before the next commit/relay: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
