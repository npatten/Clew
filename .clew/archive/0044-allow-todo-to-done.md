---
id: 44
status: done
created_at: 2026-05-17T17:19:33Z
updated_at: 2026-05-17T17:28:44Z
---
## Goal
Allow `clew done <id>` to complete increments that are still in `todo` status.

## Context
This handles the uncommon case where starting the increment was missed before completion. `todo` remains a valid status.

## Acceptance
- `todo -> done` is a valid status transition.
- Existing transition tests cover the behavior.
