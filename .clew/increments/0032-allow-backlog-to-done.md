---
id: 32
status: backlog
created_at: 2026-05-04T18:55:33Z
updated_at: 2026-05-04T18:55:33Z
---
## Goal

Allow `clew done <id>` to archive an increment directly from `status: backlog`, without requiring a prior move to `todo` or `in_progress`.

## Why

Sometimes backlog items are resolved by an external decision, duplicate discovery, documentation already existing, or tiny cleanup, and the current workflow forces a meaningless intermediate lifecycle edit before archival.

## Current behavior

`clew-spec.md` currently says `backlog → done` has no CLI support, and `clew done` is specified for `in_progress → done` only.

## Desired behavior

`clew done <id>` should accept backlog increments and perform the same terminal side effects as other done transitions:

- set `status: done`
- bump `updated_at`
- move the file to `.clew/archive/`
- remove the increment from `.clew/path.md` if present
- keep stdout/stderr contracts consistent with existing `done`

## Acceptance criteria

- A backlog increment can be completed with `clew done <id>` in one command.
- No manual `status: todo` edit or `clew start` step is required.
- Path cleanup and archive behavior match existing `done` behavior.
- Tests cover backlog-to-done and still reject inappropriate transitions.
- `clew-spec.md` is updated to document the supported transition.
