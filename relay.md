---
topic: Clew CLI — done cleanup complete; abandon is next
updated_at: 2026-04-27T15:00:00Z
---

# Relay: Clew CLI — done cleanup complete, abandon is next

## Status

`clew done` is shipped and the reviewer cleanup patch is complete. Quality gate green after cleanup: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Just finished

- **`clew done <id-or-slug>`** shipped in `1431a23`: resolves ID or slug, transitions `in_progress → done`, bumps `updated_at`, preserves frontmatter/body, archives from `.clew/increments/` to `.clew/archive/`, updates `path.md`, and prints `Done #NNNN` to stderr with empty stdout.
- **Reviewer cleanup patch** prepared after `1431a23`:
  - `transition::apply` now takes `root: &Path` so callers do the `.clew` root walk once.
  - `AppliedTransition` now includes `already_archived`.
  - Already-archived same-status `done` is now success with `warning: #NNNN already archived`, not `InvalidTransition done → done`.
  - `done` now prepares path changes in memory, archives first, then writes `path.md`. If the path write fails, the terminal archive state is still correct and lint can catch stale path drift.
  - `core::path` now has code TODOs documenting the intentionally narrow MVP parser: `remove` drops any line containing the target `#NNNN`, and `normalize` rewrites only the first reference per line.
- **Tests added** for already-archived done success/warning and stale `path.md` cleanup.

## Next action

Commit the cleanup patch, then implement `clew abandon <id-or-slug> "reason"` as the next vertical slice. Reuse `commands::transition::apply`, but it needs to set `abandoned_reason` before serialization, so either extend the helper with a mutation closure or split the helper into a lower-level parsed-file transition seam. Side effects mirror `done`: archive the file and remove it from `path.md`; self-loop tolerance should warn `warning: #NNNN already marked abandoned; completing archive`, and already-archived abandoned should warn `warning: #NNNN already archived`.

## Context worth carrying

- Stable commits before this cleanup: `1431a23` (`clew done`) and `28444a8` (prior relay update).
- `transition::apply(root, query, allowed_from, to, tolerate_self_loop)` currently only changes `status` and `updated_at`. It returns `AppliedTransition { id, path, blocked_reason, self_loop, already_archived }`.
- `AppliedTransition.blocked_reason` is still start-specific. Don’t keep adding one-off fields. `abandon` should push the helper toward a command-specific mutation/result shape.
- `done` now scans/normalizes `path.md` before archive, archives if needed, then writes path. This order is intentional: archive correctness beats path advisory drift if a later write fails.
- `core::path::remove` and `normalize` are pragmatic MVP helpers, not a markdown reference rewriting system. The TODOs are now in code, so this context is durable.
- `fs::archive_increment` uses `std::fs::rename`, not `git mv`, by design. Clew mutates project state; git workflow remains user/agent-owned.
- Output discipline remains: stdout = data; stderr = status/errors/warnings. Transition commands should keep stdout empty.
- Self-loop tolerance remains limited to terminal-side-effect transitions (`done`/`abandon`/`reopen`). `start` still rejects already-`in_progress`.
- Quality gate before the next commit/relay: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Drift from plan

- Already-archived terminal self-loops are being treated as successful no-ops with a terse warning. This is more operator-friendly than surfacing `InvalidTransition done → done` when the requested terminal side effect is already complete.
