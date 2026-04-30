---
id: 24
status: backlog
created_at: 2026-04-30T02:30:12Z
updated_at: 2026-04-30T02:30:12Z
---
## Goal
Extract pure new-increment planning from command I/O.

## Architectural friction
`src/commands/new.rs` combines ID allocation, parent validation, slug collision checks, default body selection, initial status, timestamping, serialization, and filesystem writes in one command module.

## Desired benefit
Improve locality for creation rules and test creation behaviour through a small pure interface instead of CLI/tempdir setup.
