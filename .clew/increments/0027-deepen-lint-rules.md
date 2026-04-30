---
id: 27
status: backlog
created_at: 2026-04-30T02:30:12Z
updated_at: 2026-04-30T02:30:12Z
---
## Goal
Extract pure lint evaluation over loaded Increment and Path facts.

## Architectural friction
`src/commands/lint.rs` encodes many cross-file invariants but ties them directly to filesystem loading and command reporting.

## Desired benefit
Improve locality for consistency rules and get high leverage from tests through one lint interface.
