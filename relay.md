---
topic: Clew CLI — reopen shipped; block/unblock is next
updated_at: 2026-04-27T16:12:35Z
---

# Relay: Clew CLI — reopen shipped; block/unblock is next

## Status

`clew reopen <id-or-slug>` is shipped and review follow-ups are complete. Quality gate green after the latest commit: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Just finished

- `27200ae` implemented `clew reopen <id-or-slug>`: resolves ID or slug, allows `done|abandoned → todo`, moves archived files back to `.clew/increments/`, bumps `updated_at` on real transitions, preserves body/unknown frontmatter/`abandoned_reason`, and keeps stdout empty with status/warnings on stderr.
- Added `fs::unarchive_increment` and refactored archive/unarchive movement through a shared helper. The helper now checks destination existence before `rename` so archive/unarchive cannot overwrite a colliding file.
- Review found no blockers. `7ae5ed9` added explicit regression coverage for unarchived terminal drift (`done`/`abandoned` files already in `.clew/increments/`) being reopened to `todo`.

## Next action

Implement `clew block <id-or-slug> "reason"` and `clew unblock <id-or-slug>` as the next vertical slice. Start by changing the CLI args from `u32` to `String`, then add command modules that resolve via `fs::resolve`, mutate only `blocked_reason`, bump `updated_at`, preserve body/unknown frontmatter, keep stdout empty, and print status to stderr. Blocking should work for active increments; decide deliberately whether terminal archived increments should be rejected or allowed before coding.

## Context worth carrying

- Stable commits now: `7ae5ed9` (reopen drift tests), `27200ae` (`clew reopen`), `4e20538` (allow abandon without reason), `7cc9f2a` (abandon review fixes), `149178f` (`clew abandon`), `2f7c58b` (done cleanup), `1431a23` (`clew done`).
- `transition::apply_with` intentionally does not mutate on tolerated self-loops. This preserves terminal side-effect reconciliation without bumping timestamps or rewriting frontmatter.
- `reopen` currently does not touch `path.md`. That matches the plan: reopen moves to `todo`, but priority/path placement remains a human/agent decision.
- Existing resolver searches `.clew/increments/` before `.clew/archive/`. A future hardening pass should detect ambiguous duplicate active/archive matches instead of silently shadowing archive entries.
- Transition side-effect ordering still favors completing file moves after status writes. If stronger consistency is needed later, introduce a transition+move helper with preflight/rollback rather than fixing one command ad hoc.
- Full quality gate before the next commit/relay: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
