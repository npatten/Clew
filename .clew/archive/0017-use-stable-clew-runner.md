---
id: 17
status: done
created_at: 2026-04-28T05:50:23Z
updated_at: 2026-04-28T05:56:21Z
---
## Goal

Decouple `./clew` from `cargo build` so Clew remains usable while source changes temporarily break the Rust build.

## Plan

- Store the promoted known-good binary at `.clew/bin/clew`.
- Change root `./clew` into a thin launcher that execs `.clew/bin/clew`.
- Add `.clew/bin/` to gitignore so the binary is not tracked.
- Add a promotion script that runs the quality gate and only copies the newly built binary after success.
- Document the workflow for agents/contributors.

## Notes

This intentionally avoids rebuilding on every Clew command. The downside is that `./clew` may lag behind source until explicitly promoted, but that is preferable to losing task-management access during broken builds.
