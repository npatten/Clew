---
topic: Clew CLI — block/unblock shipped; next is next
updated_at: 2026-04-27T17:07:08Z
---

# Relay: Clew CLI — block/unblock shipped; next is next

## Status

`clew block <id-or-slug> "reason"` and `clew unblock <id-or-slug>` are shipped. `AGENTS.md` now makes relay updates part of the milestone close protocol: run the quality gate once, update `relay.md`, commit work and relay together, then report only from a clean tree.

## Just finished

- `5f2d133` implemented block/unblock: resolves ID or slug, mutates only `blocked_reason`, bumps `updated_at` on real writes, preserves body/unknown frontmatter, keeps stdout empty, and prints status/warnings to stderr.
- Added explicit policy: block/unblock only apply to active, non-terminal increments. Archived increments are rejected with “reopen it first”; unarchived `done`/`abandoned` drift is rejected as an invalid transition.
- Added integration coverage for block reason quoting with `#`, slug lookup, empty reason rejection, terminal/archived rejection, no-op unblock warning without timestamp bump, and preservation behavior.
- Reviewer subagent found no blockers.
- Updated `AGENTS.md` relay discipline so future agents commit the relay with the completed work instead of as an afterthought.
- Trimmed `AGENTS.md` (~30% fewer tokens): collapsed software-dev intro, dropped duplicate `Commands` block, tightened quality-gate rules, consolidated relay discipline into one section. Co-author rule now lists Claude/Codex with a `Clankers:` header. Milestone close protocol gained an explicit user-approval gate before commit.
- Removed the duplicate `### Discipline` subsection from `hammock-thinking/crew-plan.md` (Relay format) — discipline now lives only in `AGENTS.md`; the plan owns format/shape.

## Next action

Implement `clew next` as the next vertical slice. `src/commands/next.rs` currently returns `Unimplemented`, and `src/cli.rs` currently ignores the `--start` flag by routing `Some(Command::Next { .. })` to `next::run()`. Start by changing the command signature to accept `start: bool`, then implement path-first resolution from `.clew/path.md`; if path is empty, choose the oldest active `todo` by `created_at`. For `--start`, reuse the same transition behavior as `clew start` after selecting the increment.

## Context worth carrying

- Stable commits now: `5f2d133` (block/unblock), `9f487b9` (relay after reopen), `7ae5ed9` (reopen drift tests), `27200ae` (`clew reopen`), `4e20538` (allow abandon without reason), `7cc9f2a` (abandon review fixes), `149178f` (`clew abandon`), `2f7c58b` (done cleanup), `1431a23` (`clew done`).
- `block` trims reasons and rejects empty/whitespace-only input via `ClewError::EmptyReason`.
- `unblock` is idempotent for active non-terminal increments: if already unblocked, it warns and does not rewrite or bump `updated_at`.
- The blockability helper lives in `src/commands/block.rs` and is reused by `unblock`; it rejects archived files before terminal statuses.
- `fs::resolve()` still searches `.clew/increments/` before `.clew/archive/`. A future hardening pass should detect ambiguous duplicate active/archive matches instead of silently shadowing archive entries.
- Full quality gate before the next commit: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`; then update `relay.md` and commit both together.
