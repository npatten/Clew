---
topic: Clew CLI — first vertical slice (clew show)
updated_at: 2026-04-26T22:00:00Z
---

# Relay: Clew CLI — scaffold complete, ready for clew show

## Status

Scaffolding is done and committed (6c002a8). The Rust project compiles clean, all 15 frontmatter unit tests pass, 1 integration smoke test passes, clippy is clean. The frontmatter parser is the load-bearing piece and it's working correctly with full unknown-field round-trip preservation.

## Just finished

- Initialized Rust project with full dep set: `clap`, `yaml_serde`, `chrono`, `slug`, `directories`, `thiserror`, `anyhow`; dev deps: `assert_cmd`, `assert_fs`, `insta`, `rstest`, `predicates`.
- Implemented `src/core/frontmatter.rs` with `parse()` / `serialize()` and 15 tests covering: round-trip unknown fields, round-trip body verbatim, missing delimiters, empty body, required-field errors, unknown status, optional `parent`, optional `tags`, all statuses, timestamp parsing, and an `insta` snapshot.
- Modern module layout as designed: `core.rs` + `core/` dir style (no `mod.rs`), lib + bin split.
- Stub commands all return `unimplemented!()` and compile. `clew --version` works.
- Committed 6c002a8: "scaffold Rust CLI skeleton + frontmatter parser".

## Next action

Implement `clew show <id>` as the first vertical slice. This means:

1. Flesh out `src/storage/fs.rs` — find `.clew/increments/` and `.clew/archive/` relative to the working directory (walk up until `.clew/` is found), list filenames, parse leading 4-digit ID, match by ID or slug.
2. Implement `src/commands/show.rs` — call into storage to load the file, parse it with `frontmatter::parse()`, write the raw file content to **stdout** (the full markdown+frontmatter — this IS the agent-facing API).
3. Wire the `Show { id }` variant in `src/cli.rs` `dispatch()` to pass the id arg through.
4. Integration test: `assert_cmd` + `assert_fs` — create a temp `.clew/increments/` dir with a fixture file, run `clew show 0001`, assert stdout equals the fixture content.

## Context worth carrying

- **YAML bare `#` is a comment character.** `blocked_reason: waiting on #0039` silently drops the `#0039` part. Values containing increment references must be quoted: `blocked_reason: "waiting on #0039"`. Document this in `.clew/README.md` when writing that template.
- **`yaml_serde` serializes quoted strings with single quotes** when they contain `#`. Output: `blocked_reason: 'waiting on #0039'`. This is valid YAML and round-trips correctly — just looks different from what the user typed. Not a bug.
- **`#[serde(flatten)]` works perfectly** with `yaml_serde`. Unknown fields are preserved on round-trip without any special handling. The design assumption holds.
- **`yaml_serde::Value` is the correct type** for the `extra` HashMap values — not `serde_json::Value` or `serde_yaml::Value`. The `Increment` struct in `src/core/increment.rs` uses it directly.
- **`INSTA_UPDATE=always cargo test`** is how to accept new snapshots without installing `cargo-insta` CLI.
- **Snapshot location**: `src/core/snapshots/clew__core__frontmatter__tests__snapshot_round_trip.snap`. The `.snap.new` temp file can be deleted after accepting.
- **`serde_yaml` is NOT in the project.** The `extra` field in `Increment` was sketched in `setup.md` with `serde_yaml::Value` — that was corrected to `yaml_serde::Value` during implementation. Don't reintroduce `serde_yaml`.
- **Timestamps serialize without quotes** in `yaml_serde` output (e.g., `created_at: 2026-04-20T10:00:00Z`). The parser accepts both quoted and unquoted via `chrono`'s serde support. No issue.

## Open questions

- [Decide] Should `clew show` output the raw file verbatim (frontmatter + body), or just the body, or a formatted view? The design doc says "stdout = data" and the markdown+frontmatter output IS the agent-facing API — raw verbatim is the right default. Confirm before implementing.
- [Decide] `.clew/` discovery strategy: walk up from CWD until `.clew/` is found (like git), or require running from the project root? Walking up is friendlier. Pick this before implementing `storage/fs.rs`.
- [Decide] `clew show` by slug: should the slug lookup strip the leading 4-digit prefix from the filename? Filenames are `0042-add-oauth-routes.md`; the slug the user passes would be `add-oauth-routes`. Confirm lookup strips the `NNNN-` prefix.

## Drift from plan

- `setup.md` had `cargo new clew --bin` + `cd clew` — corrected to `cargo init --bin --name clew .` (running in-place). Minor. setup.md was pre-modified before this session started (not committed by this session).
- `serde_yaml::Value` in `setup.md` Step 4 → corrected to `yaml_serde::Value` in implementation. The setup.md sketch was wrong; implementation is right.
