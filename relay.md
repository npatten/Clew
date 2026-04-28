---
topic: Clew CLI — pivoting to dogfood; M1 is implement clew init
updated_at: 2026-04-27T19:00:00Z
---

# Relay: Pivot to dogfooding — M1 is `clew init`

## Status

Lint chunk shipped previously. This session pivoted: instead of building `path`/`relay` (editor seam), we're cutting over to dogfood Clew on this repo. Plan split into two milestones.

- **M1 (next session):** implement `clew init`. Code only, no state migration.
- **M2 (after M1):** execute the cutover per `bigbang-cutover.md` (root-level runbook). Adds `./clew` wrapper, AGENTS.md workflow section, runs init, migrates `relay.md` and `backlog.md` into `.clew/`.

## Next milestone — M1: `clew init`

Implement and wire `Command::Init` (currently falls through to `ClewError::Unimplemented` in `src/cli.rs`).

**Behavioral spec** (deltas from `crew-plan.md` `## Storage model` and `## CLI sketch`):

- **Bare minimum scope.** Create `.clew/`, `.clew/increments/`, `.clew/archive/`, empty `path.md`, empty `relay.md`, and `README.md` from `src/templates/init_readme.md` (existing stub — leave it as a stub; we'll iterate later).
- **No `#0000` bootstrap increment.** The plan describes one; we're skipping it for self-host because we already have `AGENTS.md` set up. (A backlog increment will be filed in M2 to update the plan accordingly.)
- **Idempotent create-if-missing.** Each piece (dir or file) is created only when absent. Never overwrite. Print to stderr what was created vs already-present.
- **No flags.** No `--force`, no prompt.
- **Working dir = target dir.** Inits in `std::env::current_dir()`. No path arg for MVP.

**Implementation notes:**

- New file: `src/commands/init.rs`. Add `pub mod init;` to `src/commands.rs`. Wire in `src/cli.rs` dispatch (replace the wildcard `Some(_) => Err(ClewError::Unimplemented)` arm or add `Command::Init` ahead of it).
- README template lives at `src/templates/init_readme.md`; embed via `include_str!`.
- Storage seam: add a `pub fn init(root: &Path) -> Result<InitReport, ClewError>` (or similar) in `src/storage/fs.rs` that owns the create-if-missing logic. Keep `commands/init.rs` as orchestration + reporting.
- `InitReport` should let the command print "created: X" / "exists: Y" lines. Stable, parseable order is nice but not required at MVP.

**Testing:**

- Integration test in `tests/integration_test.rs` (`assert_cmd` + `assert_fs`):
  - Fresh tempdir → `./clew init` → expected layout exists; exit 0; stderr lists creations.
  - Re-run on populated dir → no overwrites; stderr lists existing items; exit 0.
  - Partial state (delete `archive/` then re-init) → archive recreated; other files untouched.
- Snapshot test via `insta` for the generated `README.md` content (matches the embedded template). The output IS our agent-facing API per the existing testing strategy.

**Out of scope for M1:** wrapper script, AGENTS.md edits, any state migration. Those are M2 — see `bigbang-cutover.md`.

## Context worth carrying

- **Why this pivot:** user wants to start dogfooding now; `init` is the only critical-path command missing for the basic loop (`init`, `new`, `start`, `done`, `show`, `list` are all that day-1 needs; `new`/`start`/`done`/`show`/`list` are already built and wired).
- **Plan vs spec philosophy** (decided this session): `crew-plan.md` is the durable spec; ephemeral task context lives in this relay; one-shot runbooks (like `bigbang-cutover.md`) live as standalone files and get deleted after execution.
- **Don't write the M2 cutover into this relay.** It's already in `bigbang-cutover.md` at the repo root — single source.
- **Existing `relay.md` (this file) and `backlog.md` stay at the root** until M2. M1 doesn't touch them.
- **No commit prefix yet.** Per `AGENTS.md` Git section, don't prefix commits with `[#NNNN]` until the repo is bootstrapped with real `.clew/` increments. M1 is still pre-bootstrap; use plain commit messages.
- Quality gate before commit: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Drift from plan

- `crew-plan.md` `## CLI sketch` says `clew init` creates `#0000-bootstrap-clew`. M1 deliberately skips that. The plan will be updated in a separate increment filed during M2 (per session decision: keep `crew-plan.md` as spec, capture deferral in backlog instead of editing the plan now).
