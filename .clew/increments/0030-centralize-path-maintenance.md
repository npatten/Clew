---
id: 30
status: backlog
created_at: 2026-05-01T14:24:45Z
updated_at: 2026-05-01T14:24:45Z
---
## Goal
Reduce repeated `path.md` read/normalize/write plumbing across lifecycle commands without introducing a broad path service abstraction.

## Context
Increment #0002 added path maintenance to `new`, `done`, `abandon`, and `reopen`. The current duplication is acceptable for now, but if path behavior keeps growing it could become a drift point.

## Direction
Consider one or two small helpers only:

- collect current slugs by ID
- normalize and write `.clew/path.md`
- optionally append/remove before normalization

Avoid a heavyweight abstraction. Keep `core/path.rs` pure and keep I/O in command/storage code.

## Acceptance
- Lifecycle commands share the boring repeated path plumbing where it improves clarity.
- No change to user-visible behavior.
- Tests still cover command-level path effects.
