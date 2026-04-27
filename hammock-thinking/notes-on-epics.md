# Notes on Epics / Parent Increments

**Status:** deferred out of MVP. Capture of the initial rough design conversation so we can pick it up later. 

_note: this is still very rough WIP, nothing is written in stone here_

## The core tension

Current plan says:

- A parent increment with multiple children forms an "epic" — a larger body of work that must ship together.
- `path.md` lists individual increments only, never parents. Parent-child is in frontmatter (`parent:`).
- `clew next` always returns a single increment, never a parent.

If children of an epic must ship together, they share the parent's priority slot. Listing them individually in path is redundant: you'd never want child A of E1 above child B of E2 without also moving the rest of E1's children. Reordering becomes painful (move epic above another = move N child lines, each).

So the natural model: **parents go in path; `clew next` walks into the parent and returns the next incomplete child.**

This is consistent with the "must ship together" rule; the current "increments only in path" rule fights it.

## Proposed model

### Path

- Parents allowed as path entries.
- Render hint: `- #0007-oauth-overhaul [epic]` — `[epic]` is a CLI render hint, not data; source of truth is the presence of children. Permissive parser ignores it on read.
- Forbid parent + any of its children in path simultaneously. `clew lint` flags it; CLI normalization on write strips the redundant child.

### `clew next` resolution

When the top of path is a parent:

1. Walk the parent's `children:` list **in order** (the list order is the work order).
2. Return the first child that is `todo` (or `in_progress`, just in case — but the likely case is the next `todo`).
3. Skip `backlog` children — surfacing unsharpened work violates the "agent picks up without asking questions" contract.
4. If all remaining children are `backlog` or `blocked`, return an informative error naming the parent and the reason. Do NOT fall through to the next path entry — the operator's expressed priority is this epic.

### Bi-directional linking

- Child has `parent: 7` (already in plan).
- Parent gets `children: [42, 43, 44]` (new).
- CLI maintains both sides on `clew new --parent`, `clew done`, `clew abandon`, `clew renumber`.
- `clew lint` checks reciprocity: every `parent: N` has a matching entry in `#N`'s `children:`, and vice versa.

**Why bi-directional:** for agents, reading `#0007` and seeing its scope in one file read beats `rg "parent: 7"` across the tree. Worth the maintenance cost.

### Child order is in the parent

- The `children:` array IS the work order — operator reorders by hand-editing the parent's frontmatter, same gesture as reordering `path.md`.
- `clew new --parent 7` appends to `#0007`'s `children:` list. This means `clew new --parent` writes two files; flag this.

### Parent file shape

```yaml
---
id: 7
status: in_progress
children: [42, 43, 44]
created_at: ...
updated_at: ...
---

# OAuth overhaul

Why this epic exists, success criteria, cross-cutting concerns.
Body is for the epic-level "why"; individual tasks live in children.
```

**Open question — does a parent have its own task checkboxes?** Lean: no. Parent is a container; tasks belong to children. `clew lint` could warn if parent has checkboxes ("consider moving to a child"). Keeps the model clean: tasks are leaves.

### Status propagation

- `clew start <child>` → if parent is `todo`, auto-flip parent to `in_progress`. Work on the epic has begun.
- `clew done <child>` when it's the last open child → does NOT auto-`done` the parent. Operator runs `clew done <parent>` after verifying the epic actually ships as a unit. Auto-doning hides the "is this really shippable?" check that was the whole point of grouping.
- `clew done <parent>` while open children exist → error, name the open children.

### Nesting depth

Cap at one level. Parent → children, no grandchildren. Keeps `clew next` resolution to a single walk and avoids "epic of epics."

### `clew list` rendering

When a parent is in-flight, show children indented beneath it. Cheap readability win, no data-model change.

## Tradeoff being accepted

You lose the ability to interleave epic children with unrelated work in priority order ("child A of E1, then unrelated D, then children B+C of E1"). That's correct: if you wanted that, the children weren't actually an epic — they didn't have to ship together. The constraint is doing its job. Worth calling out explicitly in the plan when this lands.

## Demo `.clew/` for self-hosting

`/.clew/` already exists in repo root, empty. Strongly recommend seeding it as a real fixture once epics land:

- One epic + 2–3 children covering interesting states (one done child, one in_progress, one todo).
- One standalone increment for contrast.

Self-hosting forces the design through real friction — agents will hit parent/child path resolution, lint rules, bi-directional sync in actual use rather than hypothetically. Also gives `clew show`/`list`/etc. a fixture to test against during scaffolding.

## Open questions to resolve when picking this up

- Bi-directional `children:` worth the CLI maintenance burden? (Lean: yes.)
- Parent body has no tasks — too strict, or right? (Lean: lint warning, not error.)
- `clew new --parent 7` writes both files atomically — fine, or do we want a separate `clew link` step? (Lean: atomic, no separate command.)
- Demo `.clew/` scope — how rich? Minimal coverage is one epic + 3 children + 1 standalone.
- Plan sections needing edits when this lands: Vocabulary, Path (Format/Rules/Resolution order), Statuses & transitions (propagation rules), CLI sketch (`clew next`, `clew done`, `clew lint`), Frontmatter shape (`children:` field).
