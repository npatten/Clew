---
id: 14
status: done
tags:
- ready-for-agent
created_at: 2026-04-28T04:13:51Z
updated_at: 2026-05-04T18:53:52Z
---
## Goal

Change `clew list` (and `clew list --all`) to order active increments by `path.md` rank first, then by status, while keeping the existing one-line-per-increment output stable for piping back into path-maintenance workflows.

## Behaviour

Rows stay one-per-line. Do not add section headers or other non-row output.

For active increments (`in_progress`, `todo`, `backlog`):

1. Ranked increments first. Higher entries in `.clew/path.md` print higher in `clew list` output.
2. Unranked active increments next, ordered by status priority:
   1. `in_progress`
   2. `todo`
   3. `backlog`
3. Within equal rank/status groups, sort by ID ascending.

For `clew list --all`, terminal increments append after active work and ignore any rank if present:

1. `done`
2. `abandoned`
3. ID ascending within each terminal status.

Applies to both `clew list` and `clew list --all` — no new flags.

## Out of scope

- Section headers
- `--sort` / `--sort-by` flags (hold until there is a real need)
- Any changes to filtering behaviour
