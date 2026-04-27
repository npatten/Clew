---
topic: Clew CLI — clew show vertical slice complete
updated_at: 2026-04-26T23:30:00Z
---

# Relay: Clew CLI — clew show shipped, ready for next command

## Status

`clew show <id|slug>` is complete end-to-end. 23 unit tests + 9 integration tests pass; clippy clean (`-D warnings`); fmt clean. Storage seam (`find_clew_root`, `resolve`, `read_file`) exists and will be reused by every subsequent command. AGENTS.md now codifies the three-command quality gate.

## Just finished

- **`clew show <id|slug>`** with `view` alias. Outputs the file verbatim to stdout (frontmatter + body). Walks up from CWD to find `.clew/` like git. Searches `increments/` then `archive/`.
- **Storage seam in [src/storage/fs.rs](src/storage/fs.rs)**: `find_clew_root(start)`, `resolve(root, query)` (matches by padded ID, unpadded ID, or slug), `read_file(path)`, `split_filename` helper with unit tests.
- **Errors** in [src/error.rs](src/error.rs): `NotFound(String)` (changed from `u32` since lookup is by string), new `ClewRootNotFound`. Both exit code 1 in [src/main.rs](src/main.rs).
- **CLI plumbing**: `Show { id }` arg now passed through `dispatch()` in [src/cli.rs](src/cli.rs); `view` added as `#[command(alias = "view")]`.
- **Integration tests** in [tests/integration_test.rs](tests/integration_test.rs): padded ID, unpadded ID, slug, `view` alias, archived lookup, walk-up discovery, not-found (exit 1), outside-project (exit 1).
- **AGENTS.md "Quality gates" section**: defines "milestone" concretely, codifies the three-command sweep (`fmt --check`, `clippy --all-targets -- -D warnings`, `test`), stop-the-line on red. Reference for every future session.

## Next action

Pick the next vertical slice. Two strong candidates:

1. **`clew new "<title>"`** — exercises the write path: slug generation, ID allocation (scan-and-increment across `increments/` + `archive/`), frontmatter emission, `--ready` and `--parent` flags. Good follow-up because it's the "create" half of CRUD and forces the slug-collision check from `crew-plan.md` lines 161-176.
2. **`clew list`** — lighter, read-only, also reuses the storage seam. Adds `--tag`, `--status`, `--all` filters and one-line-per-increment output. Good if we want to bank a quick win before tackling write semantics.

Recommendation: **`clew new`** next. It's the higher-risk piece (ID allocation, slug rules, collision handling) and once it's in, every other command has fixtures to work against without hand-crafting markdown in tests.

## Context worth carrying

- **Storage seam shape** (load-bearing for every command): `find_clew_root(&Path) -> Result<PathBuf>` walks up; `resolve(root, query) -> Result<PathBuf>` searches both subdirs; helper `split_filename(stem) -> Option<(u32, &str)>` parses the `NNNN-slug` shape and rejects malformed names (3-digit, 5-digit, non-numeric prefix).
- **`NotFound` is now `String`-typed**, not `u32`. The lookup query is the user's input verbatim ("0042", "42", or "add-oauth-routes"). Don't switch back to `u32` — slug lookups won't have an ID to report.
- **`clew` binary at runtime needs `~/.cargo/bin` on PATH.** This shell environment has rustup-installed cargo at `~/.cargo/bin/cargo` but it's not on `$PATH` by default. Prepend `export PATH="$HOME/.cargo/bin:$PATH" &&` to cargo invocations.
- **Quality gate is codified** in [AGENTS.md](AGENTS.md). Run `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` at every milestone — not optional.
- **`clew show` outputs verbatim** (frontmatter + body to stdout). Confirmed as the right default per `crew-plan.md`'s "stdout = data, the markdown IS the agent-facing API" principle.
- **`view` is a `#[command(alias = ...)]` on `Show`**, not a separate variant. Single source of truth; help shows `show` as canonical.
- **Three open decisions from the previous relay are now settled**: raw verbatim output ✓, walk-up `.clew/` discovery ✓, slug lookup strips `NNNN-` prefix ✓.

## Open questions

- [Decide] For `clew new`: should `--parent` validate that the parent ID exists? (Lean yes — fail fast beats dangling refs. Cheap to check during the same scan that allocates the new ID.)
- [Decide] Slug-collision check in `clew new`: error message format. `crew-plan.md` lines 168-172 sketches it; confirm wording before implementing.
- [Decide] `clew new` output: print the new ID to stdout (`#0043`), or the full path (`.clew/increments/0043-add-oauth.md`), or both? stdout = data principle suggests just the ID for piping (`clew new "X" | xargs clew start`).
