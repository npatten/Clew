---
id: 41
status: backlog
tags:
- agents
- docs
created_at: 2026-05-15T05:18:22Z
updated_at: 2026-05-15T05:18:22Z
---
## Goal
Tighten agent-facing guidance so coding agents stop reaching for raw `mv`/`edit` on `.clew/increments/*.md` files when Clew already exposes the right operation.

## Motivation
Observed mis-use: an agent renamed an increment file with `mv` to change its ID, then tried to `edit` the old path and failed. Manual renames bypass Clew's ID allocation and invariants, and the recovery path is awkward.

## Scope
- Audit `AGENTS.md` (root + `docs/agents/*.md`) for any place where renaming, renumbering, or moving increment files could plausibly seem allowed.
- Add explicit "never `mv`/rename increment files; never hand-allocate IDs" rule alongside the existing "Let `clew new` allocate IDs" line.
- Call out the supported operations for common temptations:
  - Want a different ID? Don't. IDs are immutable once allocated.
  - Want to edit body? `edit` the existing file in place is fine.
  - Want to change status? Use `clew start` / `clew done` (and future status commands), not file moves.
  - Want to retitle? Direct markdown edit of the title line is fine; do not rename the file.
- Consider whether `clew-spec.md` should state the file-name immutability invariant.

## Out of scope
- Enforcement in code (e.g., a checker that detects orphaned/renamed increment files). Track separately if we want it.

## Acceptance
- Guidance edits land in `AGENTS.md` and any relevant `docs/agents/*.md`.
- A returning agent reading the rules would not attempt the `mv` + `edit` pattern.
