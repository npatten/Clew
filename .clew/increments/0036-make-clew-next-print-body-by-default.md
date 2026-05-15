---
id: 36
status: backlog
tags:
- cli
created_at: 2026-05-15T04:05:14Z
updated_at: 2026-05-15T04:05:14Z
---
## Goal

Align `clew next` output with `clew-spec.md` §"Resolution order" (and the example at line 338, "`clew next` → get raw markdown of next priority task"). Today `clew next` prints only the ID, forcing agents to make a second `clew show <id>` call to read the body. Make the full markdown the default; keep ID-only available as an opt-in.

## Design

- `clew next` (no flags) prints the full increment markdown to stdout, equivalent to `clew show <selected-id>`. Exit code semantics unchanged. Warnings about path repair continue to go to stderr.
- `clew next --id` prints only the zero-padded ID followed by a newline (current default behavior). This is the escape hatch for shell scripts that did `clew start "$(clew next)"`.
- `clew next --start` continues to transition the selected increment to `doing`. Its stdout should match the new default (full body), so an agent gets pick + start + read context in one call. Confirm and adjust if needed.
- No other commands change.

## Breaking change

This changes the default stdout of `clew next`. Pre-1.0 and the command is recent, so the blast radius is small. Document the change in `clew-spec.md` Revisions and bump `last_major_update`.

## Tasks

- [ ] Update `clew next` handler so the default prints the resolved increment's full markdown (reuse the `clew show` rendering path).
- [ ] Add `--id` flag for ID-only output; keep the trailing newline behavior.
- [ ] Confirm/adjust `--start` so it also prints the body after transitioning.
- [ ] Update integration tests covering `clew next`, including `--id` and `--start` variants.
- [ ] Add a `Revisions` entry to `clew-spec.md` and bump `last_major_update`.
- [ ] Update `src/templates/init_readme.md` (and snapshot) to reflect the new default — the "Pick next work" row no longer needs a follow-up `clew show`.
- [ ] Run the full quality gate via `scripts/promote-clew`.

## Out of scope

- Changing path resolution semantics.
- Changing `clew next --start` lifecycle behavior beyond stdout.
- Any parent/epic rollup behavior.
