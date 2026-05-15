---
id: 42
status: backlog
tags:
- bug
- storage
created_at: 2026-05-15T05:20:24Z
updated_at: 2026-05-15T05:20:24Z
---
## Goal
Make `clew new` ID allocation safe under concurrent invocation so two near-simultaneous calls cannot allocate the same ID or corrupt `.clew/path.md`.

## Observed bug
Running two `clew new` calls back-to-back (as parallel tool calls from an agent) produced:

- Two increment files both allocated as ID 39:
  - `0039-release-tooling-preflight-and-smoketest.md` (frontmatter `id: 39`)
  - `0039-move-cargo-publish-into-ci-with-trusted-publishing.md` (frontmatter `id: 39`)
  - Both with identical `created_at` timestamps.
- `.clew/path.md` ended up with a truncated/garbled entry: the slug line for the second increment was clipped to just `publishing`, leaving:
  ```
  0039 release-tooling-preflight-and-smoketest
  publishing
  ```
  instead of a proper `0040 ...` line.

Recovery required manual `mv`, manual frontmatter edit, and manual `path.md` repair, which is exactly the workflow we tell agents not to do.

## Hypothesis
`clew new` reads the current max ID from disk, then writes the new increment file and appends to `path.md` without a lock. Two processes racing the read-then-write window:

1. Both observe max ID = 38, both allocate 39.
2. Both append to `path.md` concurrently; one append interleaves into the other's write, producing the truncated `publishing` line.

## Scope
- Identify the seam in `storage/` (or wherever allocation lives) that performs read-max-id then write.
- Serialize ID allocation and `path.md` mutation. Options to weigh:
  - Advisory file lock (`fs2::FileExt::lock_exclusive` on `.clew/` or a dedicated `.clew/.lock`) around the allocate+write critical section. Cost: another dep / platform behavior on Windows + WSL.
  - `O_EXCL` create on the candidate filename, retry on collision. Cost: needs a retry loop; `path.md` append still needs its own protection.
  - Write `path.md` atomically (write to temp, rename) instead of appending. Cost: more I/O per write, but eliminates interleaved-append corruption regardless of locking.
- Decide whether `path.md` append should be atomic-rewrite even after locking, as defense in depth.

## Acceptance
- A stress test (N parallel `clew new` invocations) produces N distinct IDs and a well-formed `path.md`.
- No manual recovery steps needed if an agent fires `clew new` in parallel.
- `clew lint` passes on the post-test state.

## Out of scope
- Locking for other commands (`start`, `done`, `tag`, etc.). Track separately if the same race exists there; this increment is specifically about `new` + `path.md`.

## Notes
Cross-ref with #0041 (agent guidance): even with hardened agent rules, the tool should not corrupt state when called in parallel. Fixing this is the durable fix; guidance is the cheap mitigation.
