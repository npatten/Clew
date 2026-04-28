---
topic: Clew CLI — M1 `clew init` implemented; next is dogfood cutover
updated_at: 2026-04-28T00:30:28Z
---

# Relay: M1 `clew init` complete; ready for M2 cutover

## Status

M1 implementation is complete in the working tree, not yet committed pending user approval.

Implemented `clew init`:

- `Command::Init` now dispatches to `src/commands/init.rs`.
- `src/storage/fs.rs` has `init(root: &Path) -> InitReport` as the storage seam.
- Creates missing `.clew/`, `.clew/increments/`, `.clew/archive/`, `.clew/path.md`, `.clew/relay.md`, and `.clew/README.md` from `src/templates/init_readme.md`.
- Idempotent create-if-missing behavior; existing paths are reported and not overwritten.
- Command writes stable `created: ...` / `exists: ...` lines to stderr and no stdout.

Tests added in `tests/integration_test.rs`:

- Fresh init layout and stderr.
- Re-run preserves existing file contents and reports existing paths.
- Partial state recreates missing `archive/` without touching existing files.
- Snapshot for generated `.clew/README.md` at `tests/snapshots/integration_test__init_readme_matches_snapshot.snap`.

Quality gate passed in one sweep:

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

Note: `AGENTS.md` had an unrelated pre-existing local change (`plan` → `spec`) before this work; do not include it in the M1 commit unless the user explicitly wants it.

## Context worth carrying

- M1 deliberately did **not** create `#0000-bootstrap-clew`; that remains a plan drift to file during M2.
- M1 did **not** touch wrapper scripts, AGENTS workflow, or state migration.
- `relay.md` and `backlog.md` are still root-level until M2.

## Next milestone — M2: execute dogfood cutover

Follow `bigbang-cutover.md` at repo root:

- Add `./clew` wrapper.
- Add AGENTS.md workflow section.
- Run `clew init` in this repo.
- Migrate root `relay.md` and `backlog.md` into `.clew/` state.
- File the deferred backlog item to update `crew-plan.md` around the skipped bootstrap increment.
