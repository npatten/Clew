# Backlog

Pre-MVP scratchpad for ideas deferred during build. Once Clew can manage itself, these graduate into real increments via `clew new`.

## Ideas

- **`path.md` "in progress" section.** Surface in-flight increments at the top of `path.md` so the priority list is also a current-focus view. Deferred 2026-04-27: cross-file state duplicating `status: in_progress` in frontmatter; risks drift. Cheaper alternative: `clew next` (or a dedicated `clew status`) scans frontmatter and prints "in progress: #0042" without persisting the duplication. Revisit once core CLI is self-hosting.

- **`clew promote <id>`.** Deferred 2026-04-27. The `backlog → todo` transition has no side effects — it's a single frontmatter field flip. Hand-editing `status: backlog` → `status: todo` in the markdown is the natural gesture (the operator is usually in the file sharpening the body anyway). Revisit if self-hosting Clew reveals real friction with the hand-edit path.

## Rejected (with reasoning)

- **`clew touch` / `clew lint --fix` (post-hand-edit reconciler).** Rejected 2026-04-27. Considered as a way to let operators hand-edit `status:` to a terminal state and then have the CLI complete side effects (archive move, `path.md` update, `updated_at` bump). Rejected because (1) it collapses the "direct edit is first-class" principle into a two-step CLI workflow; (2) it's `--force` in disguise — a gesture that exists to clean up a workflow nobody should be using; (3) reconciliation requires intent inference the CLI can't do safely (e.g., `status: done` in `increments/` could be "please archive" or "I was experimenting"); (4) it overlaps with `clew lint`'s advisory role and would create two paths to the same state. Instead: `clew lint` stays advisory ("increment 0042 has terminal status but isn't archived; run `clew done 0042`"), and `clew done`/`abandon`/`reopen` tolerate the already-flipped state with a `warning:` line. Revisit only if hand-edit-to-terminal-status becomes a real pattern in practice.
