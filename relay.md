---
topic: Clew CLI — abandon review fixes complete; reopen is next
updated_at: 2026-04-27T15:35:00Z
---

# Relay: Clew CLI — abandon review fixes complete, reopen is next

## Status

`clew abandon <id-or-slug> "reason"` is shipped and review follow-ups are complete. Quality gate green before the latest code commit: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Just finished

- Implemented `clew abandon` in `149178f`: accepts ID or slug, allows `backlog|todo|in_progress|done → abandoned`, records `abandoned_reason`, bumps `updated_at`, preserves unknown frontmatter/body, archives the increment, removes it from `path.md`, keeps stdout empty, and prints status/warnings to stderr.
- Updated agent co-author guidance in `AGENTS.md` and amended the relay commit to use `Co-Authored-By: Codex <noreply@openai.com>`.
- Addressed review follow-ups in `7cc9f2a`:
  - Empty/whitespace `clew abandon` reasons now fail with `error: abandon reason must not be empty` and exit 1.
  - Hand-edited `status: abandoned` self-loops without `abandoned_reason` are accepted for archive side-effect completion but warn that the reason is missing.
  - Added regression coverage for archived `done → abandoned`.

## Next action

Implement `clew reopen <id-or-slug>` as the next vertical slice. It should resolve archive/increments, transition `done|abandoned → todo`, move archived files back to `.clew/increments/`, and bump `updated_at` on real transition. Decide self-loop behavior deliberately: `todo` already unarchived may be a successful no-op with warning if used to reconcile a hand-edit, but `start` should remain strict. Preserve `abandoned_reason` on reopen unless we explicitly revise the plan.

## Context worth carrying

- Stable commits now: `7cc9f2a` (abandon review fixes), `149178f` (`clew abandon`), `2f7c58b` (done cleanup), `1431a23` (`clew done`).
- `transition::apply_with` intentionally does not mutate on tolerated self-loops. This preserves terminal self-loop behavior: complete file-move side effects without bumping timestamps or rewriting frontmatter.
- `abandon` now pre-reads the target to detect the specific self-loop/missing-reason warning. It still does not backfill the reason on self-loop.
- `abandon` mirrors `done` side-effect ordering: prepare path changes in memory, archive first, then write `path.md`. Archive correctness beats advisory path cleanup if a later write fails.
- `fs::archive_increment` exists, but there is not yet a symmetric unarchive helper for `reopen`.
- Transition commands still follow stdout=data, stderr=status/errors/warnings.
- Full quality gate before the next commit/relay: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
