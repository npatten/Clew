---
id: 29
status: backlog
created_at: 2026-05-01T03:38:20Z
updated_at: 2026-05-01T03:38:20Z
---
## Goal

Add a command to seed or repair `.clew/path.md` from current non-terminal increments so users do not have to manually copy every open item when adopting ranked path.

## Current thinking

Empty `path.md` is currently an opt-out from explicit ranking. Once users want the ranked single-work-view, they need a cheap way to populate the file.

Likely command shape:

```sh
./clew path sync
```

or, if `path` is not a command namespace yet:

```sh
./clew sync-path
```

Prefer a command that:

- preserves existing path order for valid non-terminal IDs
- removes missing, archived, done, and abandoned IDs
- appends missing non-terminal increments at the end by current default order
- normalizes path entries to canonical `#NNNN-slug` form
- prints what changed

## Questions

- Should sync be dry-run by default, or mutate directly with clear stdout/stderr?
- Should there be a separate `clew path init` that only works when `path.md` is empty?
- Should `clew lint` suggest this command when path drift is detected?
