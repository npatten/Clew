---
id: 35
status: done
created_at: 2026-05-15T03:40:10Z
updated_at: 2026-05-15T04:18:49Z
---
## Goal

Update the generated `.clew/README.md` source template so new Clew projects document the current core workflow.

Source of truth to edit: `src/templates/init_readme.md`. Do not hand-edit this repo's rendered `.clew/README.md` except as generated/test fixture output.

## Context

Current `.clew/README.md`/template documents the basics, but is missing several important commands and concepts now present in the CLI:

- `clew show` requires an ID or slug. If an agent does not know the ID, it should run `clew list` or `clew next` first.
- Blocking exists via `clew block <id> "reason"` and `clew unblock <id>`.
- Blocking is a `blocked_reason` flag, not a lifecycle status.
- Parent linkage exists via `clew new "Child title" --parent <id>`, but current support is just a frontmatter relationship. Do not overstate full epic/path behavior.
- Archive lifecycle commands exist: `clew abandon <id> [reason]`, `clew reopen <id>`, and `clew list --all`.
- Path/maintenance commands exist: `clew next`, `clew next --start`, `clew path`, `clew lint`, and `clew renumber <old> <new>`.
- Optional triage-style tags are useful examples: `needs-info`, `ready-for-agent`, `ready-for-human`, `needs-triage`.

We explicitly decided not to add `clew new --ready` to the generated README's core workflow. It is useful, but not core. `clew new --help` already documents it with text and an example, so no backlog item is needed for that.

## Suggested README changes

- In the agent contract, add a no-ID instruction: run `clew list` or `clew next` before `clew show <id>` when no ID is known.
- In common commands, add rows for:
  - next work: `clew next`
  - start next work: `clew next --start`
  - list archive/all work: `clew list --all`
  - block/unblock
  - abandon/reopen
  - edit path: `clew path`
  - lint state: `clew lint`
  - renumber: `clew renumber 24 34`
- In creating increments, add a short parent example using `--parent`, with a caveat that parents are links, not full epic automation yet.
- Add a short tags section explaining recommended optional tags:
  - `needs-info`: waiting on clarification; pair with `clew block` when progress is actually blocked.
  - `ready-for-agent`: fully specified for agent work; optional and not a replacement for `status: todo`.
  - `ready-for-human`: requires human judgment or manual work.
  - `needs-triage`: needs maintainer review.
- Add or adjust wording so these are called Clew tags, not tracker labels.

## Tasks

- [ ] Update `src/templates/init_readme.md`.
- [ ] Refresh affected snapshots/tests for generated README output.
- [ ] Run targeted tests for init README/template coverage.
- [ ] Run the normal quality gate before closing.
