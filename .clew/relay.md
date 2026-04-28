---
topic: Clew CLI — M2 dogfood cutover ready for review
updated_at: 2026-04-28T00:46:10Z
---

# Relay: M2 dogfood cutover ready for review

## Status

M2 dogfood cutover is complete in the working tree, pending user review/approval before commit.

Cutover changes:

- Added executable `./clew` wrapper; project workflow should use this instead of `cargo run` or `target/debug/clew`.
- Ran `./clew init` in this repo, preserving ignored `.clew/.obsidian/*` state.
- Moved root `relay.md` to `.clew/relay.md`.
- Migrated root `backlog.md` into real Clew increments:
  - `#0001` backlog — `clew new` missing from help.
  - `#0002` backlog — `path.md` in-progress section.
  - `#0003` backlog — `clew promote` command.
  - `#0004` abandoned — `clew touch` / `clew lint --fix` reconciler, with rejection reasoning preserved.
  - `#0005` backlog — decide/update plan around deferred `#0000-bootstrap-clew` behavior.
- Removed old root `backlog.md` and one-shot `bigbang-cutover.md`.
- Updated `AGENTS.md` to point at `.clew/relay.md` and document the self-hosted `./clew` workflow.

Verification performed during the cutover:

```bash
./clew lint
./clew list --all
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

`./clew lint` was clean and the full quality gate passed.

## Context worth carrying

- `./clew list` defaults to in-flight work only, so with the current backlog-only state it prints nothing. Use `./clew list --all` to see the migrated backlog until items are promoted to `todo`.
- The generated `.clew/README.md` is still the M1 stub. Real project conventions remain in `AGENTS.md` and `hammock-thinking/crew-plan.md` for now.
- The plan still says `clew init` creates `#0000-bootstrap-clew`; implementation does not. That drift is now tracked as `#0005` instead of being resolved in M2.

## Next milestone — first self-hosted Clew increment

Use `.clew/` as the source of truth. Pick or promote the next increment from the migrated backlog, likely `#0001` if the goal is a small CLI/help cleanup or `#0005` if the goal is to reconcile the plan before more feature work.
