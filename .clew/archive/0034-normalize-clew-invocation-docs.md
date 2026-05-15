---
id: 34
status: done
created_at: 2026-05-15T03:37:36Z
updated_at: 2026-05-15T03:41:44Z
---
## Goal
Update project instructions and docs to use the globally installed `clew` command for normal workflow.

## Context
Clew is now installed globally via Homebrew. The repo-local `./clew` launcher should remain documented only as a way to test this repository's promoted local development build after `scripts/promote-clew`.

## Scope
- Replace agent workflow guidance that says to always use `./clew` with `clew`.
- Keep `./clew` references only where the repo-local promoted dev build is the point.
- Avoid introducing `cargo run` as a documented workflow unless a separate decision is made.
