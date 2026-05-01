---
id: 5
status: backlog
tags:
- launch-polish
- p0
created_at: 2026-04-28T00:44:43Z
updated_at: 2026-04-30T12:00:00Z
---
# Add init bootstrap increment

## Goal

Make `clew init` create a real `#0000-bootstrap-clew` Increment that guides first-time users through connecting Clew to their agent instructions.

## Why

Clew only becomes useful to agents after the user copies the recommended workflow into whatever persistent instruction artifact their harness reads (`AGENTS.md`, `CLAUDE.md`, Cursor rules, Codex instructions, skills, or equivalent). That activation step should be visible in the generated project state, not buried in external docs.

This resolves the earlier `#0000` bootstrap deferral by implementing it rather than keeping a historical cleanup increment around.

## Scope

- Update `clew init` to create `.clew/increments/0000-bootstrap-clew.md`.
- Keep init idempotent: do not overwrite an existing `#0000` file or existing `.clew/README.md`.
- Ensure the first normal user-created Increment remains `#0001`.
- Update `src/templates/init_readme.md` with the canonical copy-paste agent contract.
- Generated user docs should assume Clew is installed and invoked as `clew`, not `./clew`.
- Update integration/snapshot tests for the new init output and generated files.

## Acceptance criteria

- A fresh `clew init` creates `.clew/README.md` and `#0000-bootstrap-clew`.
- The bootstrap Increment tells the user to copy the agent contract into their preferred persistent agent instruction artifact.
- Re-running `clew init` reports existing files and does not overwrite user edits.
- `clew new "First real work"` after init allocates `#0001`.
