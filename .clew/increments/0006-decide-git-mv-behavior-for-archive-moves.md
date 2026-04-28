---
id: 6
status: backlog
created_at: 2026-04-28T02:11:47Z
updated_at: 2026-04-28T02:11:47Z
---

# Decide git mv behavior for archive moves

Commands that move increment files between `.clew/increments/` and `.clew/archive/` currently use filesystem moves. In `git status`, that can appear as a deleted file plus an untracked file instead of a rename, which is noisy for humans reviewing Clew state changes.

## Notes

Consider whether archive/unarchive transitions (`done`, `abandon`, `reopen`) should use `git mv` when inside a git worktree.

Tradeoff: `git mv` stages the rename in the index. That may be more readable (`R old -> new`) but it also mutates git staging state, which may surprise users and agents. A design may need one of:

- keep filesystem moves and document `git status --renames` / staging behavior;
- use `git mv` only when the old file is already tracked and document the staging side effect;
- add an explicit flag/config for staged moves;
- implement another approach that improves status readability without surprising index mutation.

## Tasks

- [ ] Verify current `git status` behavior for `done`, `abandon`, and `reopen`.
- [ ] Decide whether Clew should ever mutate the git index during transition commands.
- [ ] If yes, implement and test git-aware moves with fallback outside git repos.
- [ ] If no, document the expected delete/add display and recommended review workflow.
