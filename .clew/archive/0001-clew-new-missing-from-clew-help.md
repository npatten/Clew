---
id: 1
status: done
created_at: 2026-04-28T00:44:21Z
updated_at: 2026-04-28T01:24:26Z
---

# Clew new missing from `clew --help`

`Command::New` exists and works, but the public help text should be checked for whether `new` appears correctly and documents the supported flags.

## Notes

This came from the pre-MVP backlog. It may already be fixed; the increment is to verify the current help output and either adjust the clap metadata/tests or close as already done.

## Tasks

- [x] Run `./clew --help` and `./clew new --help`.
- [x] If missing or unclear, update CLI help text and tests.
- [x] Run the full quality gate.
