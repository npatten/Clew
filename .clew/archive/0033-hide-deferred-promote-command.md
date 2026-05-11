---
id: 33
status: done
created_at: 2026-05-11T04:07:43Z
updated_at: 2026-05-11T04:27:05Z
---
## Goal
Remove the unimplemented `clew promote` command from user-facing help output.

## Context
Increment 0003 (`clew-promote-command`) was abandoned, and `clew-spec.md` marks `clew promote <id>` as deferred. A CLI enum stub still exists in `src/cli.rs`, so `./clew --help` advertises `promote` even though running it returns `error: not yet implemented`.

Do not touch `scripts/promote-clew`; that is the separate local runner promotion script.

## Acceptance criteria
- `./clew --help` no longer advertises `promote`.
- `./clew promote --help` is not exposed as a normal documented command.
- Add regression coverage so the deferred `promote` command does not appear in top-level help.
- Keep the spec's deferred note unless a broader spec cleanup is explicitly approved.
