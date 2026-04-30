---
id: 23
status: backlog
tags:
- needs-triage
created_at: 2026-04-30T02:30:12Z
updated_at: 2026-04-30T03:32:54Z
---
## Goal
Move status transition behaviour behind a deeper core interface so command modules only resolve, read, write, and report.

## Architectural friction
`src/commands/transition.rs`, `done.rs`, `abandon.rs`, `reopen.rs`, `block.rs`, and `unblock.rs` currently braid transition rules, timestamping, archive awareness, and metadata mutation. `src/core/increment.rs` exposes public fields, so status invariants have weak locality.

## Desired benefit
Improve locality for Increment status rules and get more leverage from tests at the transition interface without tempdir-heavy command setup.
