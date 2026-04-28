---
topic: Clew CLI — #0001 help cleanup ready for review
updated_at: 2026-04-28T01:24:37Z
---

# Relay: #0001 help cleanup ready for review

## Status

Increment #0001 is complete in the working tree, pending user review/approval before commit.

Changes made:

- Started and completed `#0001` via `./clew start 1` / `./clew done 1`; the increment is archived at `.clew/archive/0001-clew-new-missing-from-clew-help.md`.
- Verified `./clew --help` and `./clew new --help`.
- Found `new` was listed, but clap usage strings were blank because the `usage` feature was disabled.
- Enabled clap's `usage` feature in `Cargo.toml`.
- Added `new` argument/flag help text in `src/cli.rs` for `<TITLE>`, `--ready`, and `--parent`.
- Added integration tests covering top-level help and `clew new --help` output in `tests/integration_test.rs`.

Verification:

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

Passed after the final `./clew done 1` archive move.

## Context worth carrying

- The previous relay still described M2 cutover review, but `git status` showed that only `.clew/relay.md` was modified before starting #0001; the M2 cutover files appear already tracked/clean.
- Current working tree changes are #0001, this relay update, and newly added backlog items #0006-#0009.
- Added backlog items:
  - `#0006` — decide whether archive/reopen moves should use `git mv`.
  - `#0007` — add stdin/heredoc support for `clew new`.
  - `#0008` — add distinction for bugs? (currently title-only, needs sharpening later).
  - `#0009` — add filepath in responses (rough note, needs design).
- `./clew list --all` now shows `#0001 done`, `#0002/#0003/#0005-#0009 backlog`, and `#0004 abandoned`.

## Next milestone

Likely pick the next backlog item from `.clew/`: `#0002` (`path.md` in-progress section) or `#0005` (reconcile plan drift around deferred bootstrap behavior).
