---
id: 32
status: done
created_at: 2026-05-04T18:55:33Z
updated_at: 2026-05-15T02:30:04Z
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

## Implementation notes

- Final approach: keep the implementation narrow by adding `Status::Backlog` to the existing `clew done` transition allowlist; do not add `Status::Todo`.
- Relevant seams: `src/commands/done.rs` owns the command policy; existing storage/archive/path cleanup behavior should remain unchanged.
- Validation plan: update integration coverage for backlog-to-done success, backlog path cleanup, and `todo -> done` rejection; run focused `done` tests plus the full project quality gate.
- Docs/spec: update `clew-spec.md` to document `backlog -> done` as supported and keep `todo -> done` unsupported.
- Known risks: assert timestamp changes on the `updated_at:` line only because `created_at` remains unchanged; create any secondary fixture needed before expecting path slug normalization.
- Non-goals: no archive collision/atomicity refactor, no stdout/stderr contract changes, and no new handling for archived backlog drift.
