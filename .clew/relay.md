---
topic: Clew CLI — list output follow-up
updated_at: 2026-04-28T04:30:00Z
---

# Relay: list output follow-up

## Context worth carrying

- `clew list` semantics are now settled in `hammock-thinking/crew-plan.md`: default shows `backlog`, `todo`, and `in_progress`; `-a` / `--all` adds archived terminal statuses.
- `src/commands/list.rs` still sorts globally by ID; grouping by status is a separate follow-up and should not be mixed with filter semantics unless the increment asks for it.
- There is an untracked backlog file at `.clew/increments/0014-clew-list-group-output-by-status-with-section-head.md`; do not accidentally drop it when cleaning the tree.

## Next milestone

Pick up #0014 if grouping `clew list` output by status is still the desired next CLI polish item.
