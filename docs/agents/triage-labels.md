# Triage Labels

Matt Pocock's skills speak in terms of five canonical triage roles. Clew does not have external tracker labels; represent triage roles as Clew tags except where noted.

| Label in mattpocock/skills | Clew representation | Meaning |
| --- | --- | --- |
| `needs-triage` | tag `needs-triage` | Maintainer needs to evaluate this increment |
| `needs-info` | tag `needs-info`; add `blocked_reason` when useful | Waiting on reporter or human clarification |
| `ready-for-agent` | tag `ready-for-agent` | Fully specified, ready for an AFK agent |
| `ready-for-human` | tag `ready-for-human` | Requires human implementation or judgment |
| `wontfix` | abandon the increment with a durable reason | Will not be actioned |

## Rules

- Do not confuse triage tags with Clew statuses.
- Clew statuses are lifecycle states: `backlog`, `todo`, `in_progress`, `done`, `abandoned`.
- Triage tags are classification/state hints for humans and agents.
- If multiple triage tags conflict, ask the maintainer before editing.
- If the user says "won't fix", prefer abandoning the increment with the reason preserved rather than adding a tag.

Until Clew has dedicated tag commands, direct markdown edits to frontmatter `tags:` are acceptable.
