---
id: 9
status: done
created_at: 2026-04-28T02:26:51Z
updated_at: 2026-04-29T02:28:41Z
---
# Add Filepath in Responses

## Goal

Successful Clew commands should return enough machine-usable data for an agent or shell pipeline to continue without a follow-up lookup. In particular, commands that create, move, or identify an increment should return both the canonical increment reference and the current filepath.

## Design

Adopt the CLI output contract captured in `clew-spec.md`:

> stdout emits machine-usable result data for successful commands; stderr is reserved for warnings, errors, progress, and human-oriented status chatter.

For commands that create, move, or identify an increment, stdout should use a simple two-token shape:

```text
#NNNN .clew/<location>/NNNN-slug.md
```

Examples:

```text
$ clew new "Add OAuth routes"
#0042 .clew/increments/0042-add-oauth-routes.md

$ clew start '#0042'
#0042 .clew/increments/0042-add-oauth-routes.md

$ clew done '#0042'
#0042 .clew/archive/0042-add-oauth-routes.md
```

Human-oriented confirmations may remain on stderr if useful, but consumers should not need stderr to discover the ID or path.

## Scope

- Update `clew new` to print `#NNNN <repo-relative-path>` on stdout instead of the bare zero-padded ID.
- Update mutating increment commands that identify or move a file to print the same stdout result shape:
  - `start`
  - `done`
  - `abandon`
  - `reopen`
  - likely `block` / `unblock`, because they mutate and identify the same increment in place
- Teach increment query resolution to accept the canonical prose reference form: `#0042`.
- Keep argument validation strict. Do not make commands silently ignore extra positional arguments just to support `xargs` pass-through.
- Use repo-relative paths (`.clew/increments/...`, `.clew/archive/...`), not absolute paths.
- Keep warnings and errors on stderr.

## Rationale

- Agents get both pieces of follow-up context in one command response.
- Shell behavior stays principled: consumable data is on stdout, diagnostics/status is on stderr.
- The first stdout token remains easy to pipe explicitly:

```bash
clew new "Add OAuth routes" | awk '{print $1}' | xargs clew start
```

- Accepting `#NNNN` closes a mismatch: Clew already renders canonical references in prose, so the CLI should accept them as input.

## Acceptance criteria

- `clew new "Title"` prints exactly one stdout data line in this shape:

```text
#NNNN .clew/increments/NNNN-slug.md
```

- `clew start`, `block`, and `unblock` print the same shape with the in-place increment path.
- `clew done` and `abandon` print the same shape with the archive path after the move.
- `clew reopen` prints the same shape with the increments path after the move.
- Commands that accept an increment query resolve `#NNNN` successfully.
- Successful command data needed by agents or pipelines is available on stdout; warnings/errors remain on stderr.
- Commands still reject unexpected extra positional arguments.
- Integration tests or snapshots cover stdout/stderr behavior for the changed commands.

## Implementation notes

- Add a small shared formatter/helper rather than duplicating `#NNNN path` formatting across commands, e.g. `result_line(id, repo_relative_path)`.
- Prefer a shared repo-relative path helper so output stays consistent and testable.
- Archive/reopen commands should print the destination path, not the pre-move source path.
- Be careful with self-loop/archive-tolerance cases: output should still report the current file location.
- Keep human status lines only if they add value; if retained, they belong on stderr and tests should assert they do not pollute stdout.

## Deferred / open questions

- Should `clew next` also print `#NNNN <path>` by default, or preserve its current bare-ID output until a broader `next` design pass?
- Should `show` gain a flag or header for filepath, or remain raw markdown only?
