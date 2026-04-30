---
id: 2
status: backlog
tags:
- needs-info
created_at: 2026-04-28T00:44:22Z
updated_at: 2026-04-30T03:32:54Z
---

# `path.md` in-progress section

Consider whether `path.md` should surface in-flight increments at the top so the priority list also acts as a current-focus view.

## Deferral reasoning

Deferred 2026-04-27: this duplicates `status: in_progress` from increment frontmatter into `path.md`, creating cross-file state that can drift. A cheaper alternative is for `clew next`, `clew list`, or a later `clew status` to scan frontmatter and print the current in-progress increment without persisting duplication.

Revisit only after self-hosting shows whether humans/agents actually miss an at-a-glance in-progress view.

## Tasks

- [ ] Interview self-hosting use: is current focus hard to discover?
- [ ] Prefer computed CLI output over persistent duplication unless there is a clear need.
- [ ] If accepted, update `crew-plan.md` before implementation.
