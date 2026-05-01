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
- normalizes path entries to the canonical `NNNN slug` 2-column form (drops any pasted status column from `clew list` output)
- gives `clew lint` a concrete repair command to suggest when it sees pasted 3-column list output
- prints what changed

### Why on-demand normalization matters

The parser is intentionally lenient — it accepts 3-column `clew list` output (`NNNN status slug`) so `clew list > .clew/path.md` is a viable seeding flow. But the bash redirect bypasses Clew entirely, so the file sits in 3-column form until the next mutating CLI command (`new`, `done`, `abandon`, ...) triggers normalize-on-write. That intermediate state can mislead a human reader into thinking status is persisted in `path.md`. `clew path sync` is the natural place to close that gap on demand without forcing an unrelated mutation.

## Questions

- Should sync be dry-run by default, or mutate directly with clear stdout/stderr?
- Should there be a separate `clew path init` that only works when `path.md` is empty?
- Should `clew lint` suggest this command when path drift is detected?
