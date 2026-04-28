---
id: 4
status: abandoned
abandoned_reason: 'Collapses ''direct edit is first-class'' into a two-step CLI workflow; --force in disguise; reconciliation requires intent inference the CLI can''t do safely; overlaps with clew lint''s advisory role. Instead: lint stays advisory, and done/abandon/reopen tolerate already-flipped state with a warning. Revisit only if hand-edit-to-terminal-status becomes a real pattern.'
created_at: 2026-04-28T00:44:23Z
updated_at: 2026-04-28T00:44:28Z
---

# `clew touch` / `clew lint --fix` post-hand-edit reconciler

Rejected 2026-04-27.

Considered as a way to let operators hand-edit `status:` to a terminal state and then have the CLI complete side effects such as archive moves, `path.md` updates, and `updated_at` bumps.

## Rejection reasoning

- It collapses the "direct edit is first-class" principle into a two-step CLI workflow.
- It is `--force` in disguise: a gesture that exists to clean up a workflow nobody should be using.
- Reconciliation requires intent inference the CLI cannot do safely. For example, `status: done` in `increments/` could mean "please archive" or "I was experimenting".
- It overlaps with `clew lint`'s advisory role and would create two paths to the same state.

Instead, `clew lint` stays advisory: "increment #0042 has terminal status but is not archived; run `clew done 0042`." The terminal commands (`done`, `abandon`, `reopen`) tolerate already-flipped state with a `warning:` line and complete their own side effects.

Revisit only if hand-edit-to-terminal-status becomes a real pattern in practice.
