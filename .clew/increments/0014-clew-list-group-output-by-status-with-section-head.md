---
id: 14
status: backlog
created_at: 2026-04-28T04:13:51Z
updated_at: 2026-04-28T04:13:51Z
---
## Goal

Change `clew list` (and `clew list --all`) to group increments by status rather than ordering purely by ID. Print a dim section header between groups.

## Behaviour

Priority order (top to bottom):
1. `in_progress`
2. `todo`
3. `backlog`
4. `done`
5. `abandoned`

Within each group, sort by ID ascending (existing behaviour).

Only print a section header for groups that have at least one item. Header format TBD but something like `--- in progress ---` (dimmed/grey if the terminal supports it).

Applies to both `clew list` and `clew list --all` — no new flags.

## Out of scope

- `--sort` / `--sort-by` flags (hold until there is a real need)
- Any changes to filtering behaviour
