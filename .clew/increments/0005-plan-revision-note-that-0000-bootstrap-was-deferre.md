---
id: 5
status: backlog
created_at: 2026-04-28T00:44:43Z
updated_at: 2026-04-28T00:44:43Z
---

# Plan revision: note that `#0000` bootstrap was deferred for self-host

M1 `clew init` intentionally did not create `#0000-bootstrap-clew`, despite the plan's CLI sketch saying init should create it. Capture that design drift in `hammock-thinking/crew-plan.md` once the self-hosting cutover settles.

## Context

This was left out of M1 to keep init as a simple idempotent scaffold command. Now that Clew is dogfooding itself, decide whether the bootstrap increment still earns its complexity or should be removed from the plan.

## Tasks

- [ ] Re-read the `clew init` CLI sketch in `hammock-thinking/crew-plan.md`.
- [ ] Decide whether `#0000-bootstrap-clew` remains planned or is permanently deferred/removed.
- [ ] Update `crew-plan.md` and its revision log if the decision is meaningful.
