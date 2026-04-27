---
topic: Clew CLI — lint implemented; next is review/commit
updated_at: 2026-04-27T18:20:00Z
---

# Relay: Clew CLI — lint implemented; next is path/editor milestone

## Status

`clew lint` is implemented as an advisory drift checker. It is read-only, exits 0 when clean, exits 1 with `LintFailed` when drift is found, and prints findings to stderr as warnings before the final error line. Clean lint intentionally prints `No lint issues found` to stderr for explicit confirmation.

## Just finished

- Added `src/commands/lint.rs` and wired `Command::Lint` in `src/cli.rs` / `src/commands.rs`.
- Added `ClewError::LintFailed(usize)` and exit-code mapping as a user error.
- Lint checks now flag:
  - terminal `done` / `abandoned` statuses left under `.clew/increments/` with transition-command hints;
  - non-terminal statuses under `.clew/archive/` without suggesting invalid `reopen` commands;
  - filename ID vs frontmatter `id` mismatches;
  - missing, archived, non-`todo`, and non-canonical `path.md` references;
  - active `todo` increments missing from a non-empty `path.md` priority list.
- Added integration coverage for clean lint, path drift, terminal status drift, todo-not-in-path drift, archived non-terminal drift, ID mismatch, and per-line stale path references.

## Next milestone

Path/editor milestone: implement the user-editor resolution seam, then wire the first editor-backed command (`clew path`) so it opens `.clew/path.md` via the resolved editor without hanging in non-TTY agent contexts. Use the plan’s resolution order in `hammock-thinking/crew-plan.md` and keep `clew relay` for the follow-on slice if path/editor gets too large.

## Context worth carrying

- Output choice: lint findings go to stderr as `warning:` lines; `main.rs` then prints `error: lint found N issue(s)` and exits 1. Clean lint prints `No lint issues found` to stderr.
- `path.md` missing-todo warnings only fire when `path.md` has at least one valid reference, preserving the plan’s “empty path is fine” rule.
- `path.md` canonical checks are per reference line, not global substring checks, so stale `#0001-old` is still flagged even if a comment elsewhere contains `#0001-current`.
- A reviewer caught two important edge cases during implementation: archived `backlog`/`in_progress` cannot be fixed with `clew reopen`, and filename/frontmatter ID drift can make transition-command hints unsafe. Both are covered by tests now.
- Quality gate passed before this relay update: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`. Re-run all three before committing because `relay.md` changed afterward.
- This lint chunk is awaiting user approval before commit per `AGENTS.md`; that approval step is process state, not the next product increment.

## Drift from plan

- Relay discipline was tightened in `AGENTS.md`: use `Next milestone` for the next product milestone/increment, not micro process steps like “ask for approval” or “commit after review.” Put those process details in Status/Context if they matter.
