---
topic: Clew CLI — clew new vertical slice complete
updated_at: 2026-04-26T23:55:00Z
---

# Relay: Clew CLI — clew new shipped, write path proven

## Status

`clew new "<title>"` is complete end-to-end. 32 unit tests + 20 integration tests pass; clippy clean (`-D warnings`); fmt clean. The write path works: ID allocation, slug generation + collision check, optional `--parent` validation, frontmatter emission via the existing `serialize`. The storage seam now has `scan(root) -> Vec<FileEntry>` (single walk over both subdirs) and `write_new_increment(root, filename, contents)` — both will get reused by every later write command.

## Just finished

- **`clew new "<title>"`** with `--ready` and `--parent <id>` flags. Default status `backlog`, `--ready` flips to `todo`. Walks up to find `.clew/`, scans both subdirs once, validates parent if passed, generates slug, checks collision, allocates `max(ids) + 1`, writes the file, prints padded ID to stdout (`0001\n`). Empty body — agent fills it.
- **Slug module** in [src/core/slug.rs](src/core/slug.rs): `generate(title)` → `slug` crate → 50-char chars-truncate → `trim_matches('-')` (truncation can land on a dash) → `untitled` fallback when result is empty. 8 unit tests via `rstest`: ASCII, unicode (`Café résumé` → `cafe-resume`), all-empty cases (whitespace, `---`, `!!!`), length truncation, trailing-dash trim.
- **Storage seam grew** in [src/storage/fs.rs](src/storage/fs.rs): `FileEntry { id, slug, path }`, `pub fn scan(root)` walks both subdirs and returns parseable `NNNN-slug.md` entries (skips malformed silently); `pub fn write_new_increment(root, filename, contents)` creates the increments dir if missing, writes the file. `scan` is the single-walk helper for ID alloc + slug collision + parent existence.
- **Errors** in [src/error.rs](src/error.rs): `SlugCollision` enriched from `(String)` to `{ slug: String, existing: String }`. Display string is two lines per the plan: `slug 'X' is already used by #NNNN-slug.md\n       try a more specific title`.
- **CLI plumbing**: `Some(Command::New { title, ready, parent }) => ...` in [src/cli.rs:75-79](src/cli.rs#L75-L79).
- **Integration tests** in [tests/integration_test.rs](tests/integration_test.rs): backlog default, `--ready`, sequential allocation, archive-aware allocation, `--parent` valid + missing, slug collision in both subdirs, walk-up discovery, outside-project, stdout-pipes-to-show roundtrip.

## Next action

Pick the next vertical slice. With `clew new` shipped, every later command can use real fixtures instead of hand-crafted markdown. Two strong candidates:

1. **`clew list`** — read-only; reuses `fs::scan` directly. Adds `--tag`, `--status`, `--all` filters and one-line-per-increment output. Default: in-flight items only (no `done`/`abandoned`); `--all` includes archived. The output format is the open question — agent-facing API design (per `crew-plan.md` lines 487, 411). Probably `#NNNN status slug` per line, sorted by ID? Or by status group?
2. **`clew start <id>`** — first state-transition write: `todo → in_progress`, bumps `updated_at`. Forces us to design the round-trip-edit path (read → mutate → write) on top of the existing `frontmatter::parse` + `serialize`. Also forces the typed-transition validator (`InvalidTransition` already exists in error.rs).

Recommendation: **`clew list`** next. It's smaller, read-only, and gives agents the ability to discover what's in the project — which is the natural pairing with `new` in the agent workflow loop. State transitions can come right after; they all share the same read-mutate-write pattern, so we may as well do them in a small batch (`promote`, `start`, `done`, `block`, `unblock`) once we lock in the pattern.

## Context worth carrying

- **Storage seam shape** (load-bearing): `scan(root) -> Vec<FileEntry>` is the canonical "list everything in both subdirs" call. `FileEntry` only has `id`, `slug`, `path` — no frontmatter parsing yet. `clew list` will need richer entries (status, tags from frontmatter); decide whether to enrich `FileEntry` or layer a `scan_with_frontmatter` on top. Lean toward layering — keep `scan` cheap for the common case.
- **`write_new_increment` is intentionally dumb** — caller builds filename and contents. Keeps storage as the I/O seam; commands compose. Pattern to repeat for `write_increment` (overwrite path, used by state transitions).
- **`SlugCollision` is now a struct variant**, not tuple. Don't switch back — the two-line error message needs both fields. The `existing` field carries the full filename like `0042-add-oauth.md`.
- **Stdout from `clew new` is just the padded ID + newline.** No `#`, no path. This is for piping (`clew new "X" | xargs clew show` works, verified by `new_output_is_pipeable_to_show`). Stick to the "stdout = data" principle for every new command.
- **`--parent` validates against both `increments/` and `archive/`.** Existing-but-archived parent is allowed (e.g., bug fix on a shipped feature); only missing IDs fail. Error reads `not found: parent #0007`.
- **Body for new increments is empty.** Considered a `# {title}` heading; decided agent should pick its own structure. Easy to add later if it earns its keep.
- **Timestamp on create**: `Utc::now()` round-tripped through `to_rfc3339_opts(SecondsFormat::Secs, true).parse()` to enforce the second-precision invariant before the value ever hits the frontmatter serializer. Reuse this pattern in transition commands.
- **`clew` binary at runtime needs `~/.cargo/bin` on PATH.** Prepend `export PATH="$HOME/.cargo/bin:$PATH" &&` to cargo invocations in this shell.
- **Quality gate**: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`. Run all three, in one sweep, before any milestone. Codified in [AGENTS.md](AGENTS.md).

## Open questions

- [Decide] `clew list` default sort: by ID ascending, by `updated_at` descending, or grouped by status? My read: by ID ascending is simplest and matches "scan order"; status filtering via `--status` is the more useful axis.
- [Decide] `clew list` output line format: agent-facing, so terse and parseable. `#0042 in_progress add-oauth-routes` (space-separated)? Or `0042 in_progress add-oauth-routes` (no `#` so it's pipeable like `clew new`)? Lean toward no `#` for symmetry with `new`'s stdout.
- [Decide] `clew list --all`: include archived alongside active? Or only show archived when `--all` is the *only* filter? The plan says "Default: in-flight items only. `--all` includes archived." — I read that as "archived appears mixed in with active when `--all` is set," sorted together by ID.
- [Note] Slug-collision message dropped the plan's `(e.g., "Add OAuth for Google")` parenthetical — judged it as illustrative, not normative. If that turns out to be wrong, the suggestion would need to be parameterized on the user's title (the plan's literal example wouldn't make sense for non-OAuth titles).
