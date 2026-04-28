---
id: 13
status: backlog
created_at: 2026-04-28T03:50:14Z
updated_at: 2026-04-28T03:50:14Z
---
## Goal

Implement `-a` as an alias for `--all` on `clew list`. Also correct `crew-plan.md` which mis-describes the default behavior.

## Current reality

`clew list` currently shows backlog + todo + in_progress (everything non-archived). The spec incorrectly says default is `todo + in_progress` only and that `-a` adds backlog. The current behavior is actually good and should be preserved.

## The gap

`./clew list -a` errors with "unexpected argument found". Following ls convention, `-a` should mean "show all" — which here means adding archived (done + abandoned) to the default output.

## Design

- Default: backlog + todo + in_progress (current behavior, keep as-is)
- `-a` / `--all`: all statuses — adds done and abandoned
- `--status <X>`: explicit single-status filter (unchanged)

## Tasks

- [ ] Implement `-a` as alias for `--all` in the list subcommand
- [ ] Update `crew-plan.md` CLI sketch to reflect: default includes backlog; `-a`/`--all` adds archived
- [ ] Integration test: `-a` and `--all` produce same output and include archived items
