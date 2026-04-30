---
id: 25
status: backlog
created_at: 2026-04-30T02:30:12Z
updated_at: 2026-04-30T02:30:12Z
---
## Goal
Give Path maintenance a deeper module that owns removal and canonical-reference normalization policy.

## Architectural friction
`src/core/path.rs` has line transforms, while `src/commands/done.rs`, `abandon.rs`, and `lint.rs` own the higher-level invariant that terminal increments leave Path and Path references stay canonical.

## Desired benefit
Improve locality for Path rules and give callers more leverage from one operation instead of sequencing transforms correctly.
