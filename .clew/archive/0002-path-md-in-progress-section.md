---
id: 2
status: done
created_at: 2026-04-28T00:44:22Z
updated_at: 2026-05-01T15:42:33Z
---

# `path.md` ranked work view

Users want one visible place to see open work and rank priorities. The useful editor affordance is a flat `path.md` list where moving lines up/down changes rank.

## Current direction

`path.md` should convey **rank only**. Frontmatter remains the source of truth for lifecycle state, tags, timestamps, and all other metadata.

Avoid `## In Progress` or other status sections in `path.md`: sections would either duplicate frontmatter status or imply that moving a line between sections mutates status. That is drift-prone and complects ranking with lifecycle state.

Final on-disk format mirrors the leading two columns of `clew list` so users can copy `clew list` output directly into `path.md`:

```text
0019 verify-clew-on-wsl-and-git-bash
0028 polish-agent-onboarding
0022 speed-up-quality-gate
0003 clew-promote-command
```

No `#` sigil, no list markers, no status column. Status is joined from frontmatter at render time by `clew list`:

```text
0019 in_progress verify-clew-on-wsl-and-git-bash
0028 in_progress polish-agent-onboarding
0022 backlog     speed-up-quality-gate
0003 backlog     clew-promote-command
```

The parser tolerates a pasted 3-column `clew list` line (ID, status, slug) for rank extraction, but `clew lint` flags the status column because canonical `path.md` remains 2-column. CLI normalization rewrites unambiguous pasted-list lines to the 2-column form on mutating commands (`new`, `done`, `abandon`, `reopen`, and `next` when it repairs membership). Status-column detection is contextual: only treat column 2 as status when column 3 matches the known current slug, and prefer canonical interpretation when the slug itself is a status word (`0001 todo p0`).

`clew next` should pull from the top eligible item in `path.md`. Clew owns path membership while the user owns path order: if `next` encounters terminal/archived entries at the top, it may remove them with warnings and continue to the next valid ranked item. Missing IDs should still fail loudly because they may indicate typo, branch mismatch, or corruption. If `path.md` is empty, fall back to the existing default selection rule.

## Implementation notes

- Decide whether `path.md` ranks all non-terminal increments (`backlog`, `todo`, `in_progress`) or only actionable work. Current leaning: all non-terminal increments, because users are asking to rank their work backlog in one file.
- `path.md` parser should treat ID as authoritative; slug/title text is cosmetic and can be normalized by CLI writes.
- `clew list` should be able to render path order plus frontmatter status as the primary single-work-view.
- `clew lint` should flag duplicates, missing IDs, archived/done/abandoned IDs in path, and stale cosmetic slug text.
- Lifecycle commands should keep path consistent: `new` appends to an already-ranked path at lowest priority and normalizes; terminal transitions remove IDs and normalize; `reopen` appends at lowest priority unless the ID is already ranked, then normalizes; `next` may repair terminal/archived path membership before selecting; renumber should update references when it exists.

## Tasks

- [x] Interview self-hosting use: current focus and priority ranking are hard to see in one view.
- [x] Prefer computed CLI output over persistent status duplication.
- [x] Update `clew-spec.md` before implementation.
- [x] Implement ranked `path.md` list/view behavior.
- [x] Land bare 2-column `NNNN slug` line format with status-column-tolerant parser.
- [x] Make `clew start` idempotent so `clew next --start` round-trips on already-in-progress picks.
- [x] Tighten `clew lint`: per-line stale-slug message instead of a substring `contains` check.
- [x] Warn on pasted 3-column `clew list` rows in `path.md`; defer repair command to #0029.
- [x] Make `new`, `done`, `abandon`, and `reopen` normalize `path.md` on write; keep `reopen` idempotent for already-ranked self-loops.
- [x] Cover parser ambiguity where the slug itself is a status word.
- [x] Make `clew next` remove terminal/archived path entries with warnings before selecting the next valid ranked item.
