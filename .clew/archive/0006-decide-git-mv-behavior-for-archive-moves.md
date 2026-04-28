---
id: 6
status: done
created_at: 2026-04-28T02:11:47Z
updated_at: 2026-04-28T04:48:25Z
---

# Decide git mv behavior for archive moves

Commands that move increment files between `.clew/increments/` and `.clew/archive/` currently use filesystem moves. In `git status`, that can appear as a deleted file plus an untracked file instead of a rename, which is noisy for humans reviewing Clew state changes.

## Notes

Decision: keep filesystem moves. Clew should not run `git mv` or otherwise mutate the git index during transition commands.

Verified behavior in a temporary git repo:

- `done`, `abandon`, and `reopen` currently show as a deleted tracked file plus an untracked destination before staging.
- After `git add -A`, git reports the move as a rename (`R old -> new`) when similarity is high enough.

Tradeoff accepted: pre-staging status is noisier, but Clew does not surprise users or agents by changing staged state. The review workflow is: inspect normal status awareness, then `git add -A` when ready to review/commit the Clew state move as a rename.

Docs updated in `clew-spec.md`, `.clew/README.md`, and the init README template.

## Tasks

- [x] Verify current `git status` behavior for `done`, `abandon`, and `reopen`.
- [x] Decide whether Clew should ever mutate the git index during transition commands.
- [x] If no, document the expected delete/add display and recommended review workflow.
