---
id: 22
status: backlog
created_at: 2026-04-30T02:08:41Z
updated_at: 2026-04-30T02:08:41Z
---
## Context

Agent work is starting to hit Pi harness timeouts during routine validation. The visible `cargo test` runtime is still short once compiled, so the likely costs are compile/check churn, `cargo clippy --all-targets`, release builds, and process-heavy integration tests rather than test assertions alone.

Do not optimize blindly. First measure where the time goes, then split fast feedback from release-quality promotion without weakening `scripts/promote-clew`.

## Goal

Make the common agent feedback loop faster and less timeout-prone while keeping the full promotion gate strict.

## Scope

### In

- Measure cold and warm timings for:
  - `cargo fmt --check`
  - `cargo test`
  - targeted integration tests
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo build --release`
  - `scripts/promote-clew`
- Identify whether the bottleneck is compile time, process-spawn integration tests, clippy, release build, or Pi timeout behavior.
- Consider a fast local check script for agents, distinct from `scripts/promote-clew`.
- Consider restructuring integration tests so most command behavior is tested through library functions, with fewer `assert_cmd` CLI smoke tests.
- Consider grouping slow or end-to-end tests separately if measurement proves they matter.
- Document recommended validation commands for small edits vs promotion.

### Out

- Weakening `scripts/promote-clew`; it remains the release/promotion gate.
- Skipping tests in the closing workflow.
- Adding test flakiness workarounds without understanding the root cause.

## Design notes

Potential shape:

- `scripts/check-fast`: fmt check + `cargo test --lib` + selected integration smoke tests.
- `scripts/promote-clew`: unchanged strict gate for closing increments.
- Move pure command behavior toward direct library tests over time; keep CLI tests for argument parsing, stdout/stderr contracts, cwd/root walking, and end-to-end archive moves.

Cost: two validation paths can confuse contributors. Mitigate with clear names: fast feedback is not promotion.

## Tasks

- [ ] Record cold/warm timing table for each gate command
- [ ] Identify the actual timeout source under Pi
- [ ] Decide whether a `scripts/check-fast` helper earns its keep
- [ ] If yes, add it and document when to use it
- [ ] Audit integration tests for candidates that should become library-level tests
- [ ] File follow-up increments for any larger test-suite refactors
