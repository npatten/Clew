---
topic: DDD interaction-paths drafting
updated_at: 2026-04-26
session_count: 2
---

# Relay: DDD interaction paths

## Status

DDD phase complete — design locked. Meta-doc, 24-path candidate list, and updated crew-plan.md all finalized. Ready to begin drafting path bodies starting with IP-03.

## Just finished

- Co-designed epic/increment distinction: collapsed into single increment type with optional parent-linkage. Parent increment with children = epic (semantic, not structural). All increments use same counter, stored in single `increments/` dir.
- Updated `crew-plan.md`: vocabulary, storage model, frontmatter (epic: → parent:), path rules, CLI sketch. Design is now internally consistent.
- Identified three new CLI-design observations: (a) TTY detection confirmed reliable across agent modes; (b) agents prefer dedicated subcommands over conditional flags; (c) $EDITOR/$VISUAL unreliable with Electron editors — init should scan PATH and store preference in config.
- Codified session-unit-of-work: `clew next` always returns a single increment (never a parent). Path lists increments only. Human review can batch per-epic or stay per-increment.

## Next action

Draft IP-03 body (fresh agent picks up work — canonical loop). Use template in `IP-notes.md`. After IP-03, suggested order: IP-08 → IP-09 → IP-05.

## Context worth carrying

- **Epic/increment distinction collapsed:** one type, parent-linkage in frontmatter. `parent: 0007` not `epic: 0007`. Update any code/templates referencing the old model.
- **`clew next` always returns an increment** (never a parent). Path lists increments only. Session unit of work is always a single increment.
- **Editor resolution (TTY-detected fallback)**: scope is still open on init auto-detection vs. manual config. CLI-design-notes.md has tentative directions. Flag when drafting IP-01.
- **Subcommand vs. flag principle:** when behavior changes destination/shape/validation, use a new subcommand (`clew import`, `clew capture`) not a flag. Apply when drafting IP-08/IP-09.

## Open questions

- [Decide] Editor init: auto-detect editors in PATH + prompt, or just manual config in `.clew/config.toml`? (see CLI-design-notes.md § Editor resolution)
- [Decide] When drafting bulk import (IP-09), use storage-format-mirror (agent emits markdown-with-frontmatter blocks) or YAML manifest? (leaning storage-mirror for consistency)

## Drift from plan

- None this session. Design moved faster and more coherently than expected; no task scope creep.
