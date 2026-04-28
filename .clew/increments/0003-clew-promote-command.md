---
id: 3
status: backlog
created_at: 2026-04-28T00:44:23Z
updated_at: 2026-04-28T00:44:23Z
---

# `clew promote` command

Reconsider whether Clew needs a `clew promote <id>` command for `backlog → todo`.

## Deferral reasoning

Deferred 2026-04-27. The transition has no side effects: it is a single frontmatter edit from `status: backlog` to `status: todo`. In practice, the operator is usually already editing the body to sharpen the work, so hand-editing the status is the natural gesture.

The cost of adding this command is another workflow path to document and test. Revisit only if self-hosting shows real friction with the hand-edit path.

## Tasks

- [ ] During self-hosting, note whether backlog promotion feels annoying or error-prone.
- [ ] If implementing, update `crew-plan.md` with the new rationale first.
- [ ] If still unnecessary, abandon this increment with the observed reasoning.
