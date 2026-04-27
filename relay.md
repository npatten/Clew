---
topic: Clew CLI — first vertical slice (clew show)
updated_at: 2026-04-26T23:00:00Z
---

# Relay: Clew CLI — scaffold + review fixes complete, ready for clew show

## Status

Scaffolding committed (6c002a8) and review feedback addressed (1e3694d). 17 unit tests + 1 integration test pass, clippy clean, fmt clean. The frontmatter parser is the load-bearing piece and is working correctly with deterministic output and full unknown-field round-trip preservation (now value-asserted, not just key-asserted).

## Just finished

- Initial scaffold (6c002a8): Rust project with full dep set, modern module layout, frontmatter parser with 15 tests, smoke test for `clew --version`.
- Review-feedback pass (1e3694d):
  - **`Increment.extra` is now `BTreeMap`**, not `HashMap`. The markdown+frontmatter output is our agent-facing API and must be deterministic across processes; HashMap iteration order is randomized.
  - **`id`/`parent` are plain integers in YAML**, not zero-padded. YAML 1.2 parses `0042` as a string, which would break `u32` deserialization. Zero-padding is now explicitly documented in `crew-plan.md` as a filename + prose-reference rule only. The CLI still renders `#NNNN` form on output.
  - **`round_trip_preserves_unknown_fields` now does a full equality assertion** on the entire `extra` map, across nested maps, arrays, scalars, bools, and `#`-containing strings. Previous test only checked key presence.
  - New test: `unknown_field_serialization_is_deterministic` guards against ordering regressions if `extra` ever gets swapped back to a non-deterministic map type.
  - Added missing `created_at` rstest case.
  - **Stub commands return `Err(ClewError::Unimplemented)`** instead of panicking, so the typed exit-code path (panic → 1) is exercised.
  - Smoke test now asserts stdout contains `clew` + `CARGO_PKG_VERSION`.
  - `cargo fmt` clean.
- **Deferred from review:** strict timestamp serde (reject non-`Z` offsets, reject subseconds). The CLI is the only writer; `chrono`'s default serde produces canonical form. If a human hand-edits a timestamp wrong, that's a `clew lint` concern later, not a parser concern now. Document the format rule in `.clew/README.md` when that template is fleshed out.

## Next action

Implement `clew show <id>` as the first vertical slice. This means:

1. Flesh out `src/storage/fs.rs` — find `.clew/increments/` and `.clew/archive/` relative to the working directory (walk up until `.clew/` is found), list filenames, parse leading 4-digit ID, match by ID or slug.
2. Implement `src/commands/show.rs` — call into storage to load the file, parse it with `frontmatter::parse()`, write the raw file content to **stdout** (the full markdown+frontmatter — this IS the agent-facing API).
3. Wire the `Show { id }` variant in `src/cli.rs` `dispatch()` to pass the id arg through.
4. Integration test: `assert_cmd` + `assert_fs` — create a temp `.clew/increments/` dir with a fixture file, run `clew show 0001`, assert stdout equals the fixture content.

## Context worth carrying

- **YAML bare `#` is a comment character.** `blocked_reason: waiting on #0039` silently drops the `#0039` part. Values containing increment references must be quoted: `blocked_reason: "waiting on #0039"`. Document this in `.clew/README.md` when writing that template.
- **`yaml_serde` serializes quoted strings with single quotes** when they contain `#`. Output: `blocked_reason: 'waiting on #0039'`. Valid YAML, round-trips correctly. Not a bug.
- **`#[serde(flatten)]` works perfectly** with `yaml_serde` + `BTreeMap`. Unknown fields are preserved on round-trip with deterministic ordering.
- **`yaml_serde::Value` is the correct type** for the `extra` map values — not `serde_json::Value` or `serde_yaml::Value`. (`serde_yaml` is archived; do not reintroduce.)
- **`extra` is a `BTreeMap`, not `HashMap`.** Required for deterministic serialization. If you ever need input-order preservation instead of alphabetical, swap to `IndexMap` with a direct `indexmap` dep — but only if real demand emerges.
- **`id` and `parent` in frontmatter are plain integers** (`id: 42`, not `id: 0042`). Zero-padding is presentation-only: filenames (`0042-add-oauth-routes.md`) and prose references (`#0042`). YAML 1.2 parses `0042` as a string anyway.
- **`INSTA_UPDATE=always cargo test`** accepts new snapshots without needing the `cargo-insta` CLI.
- **Snapshot location**: `src/core/snapshots/clew__core__frontmatter__tests__snapshot_round_trip.snap`. Delete `.snap.new` after accepting.
- **Timestamps round-trip via `chrono`'s default serde.** Output is unquoted `2026-04-20T10:00:00Z`. Strict format enforcement (reject `+02:00` offsets, reject subseconds) is deferred — `clew lint` concern, not a parser concern.

## Open questions

- [Decide] Should `clew show` output the raw file verbatim (frontmatter + body), or just the body, or a formatted view? The design doc says "stdout = data" and the markdown+frontmatter output IS the agent-facing API — raw verbatim is the right default. Confirm before implementing.
- [Decide] `.clew/` discovery strategy: walk up from CWD until `.clew/` is found (like git), or require running from the project root? Walking up is friendlier. Pick this before implementing `storage/fs.rs`.
- [Decide] `clew show` by slug: should the slug lookup strip the leading 4-digit prefix from the filename? Filenames are `0042-add-oauth-routes.md`; the slug the user passes would be `add-oauth-routes`. Confirm lookup strips the `NNNN-` prefix.

## Drift from plan

- `setup.md` had `cargo new clew --bin` + `cd clew` — corrected to `cargo init --bin --name clew .` (running in-place). Minor. setup.md was pre-modified before this session started (not committed by this session).
- `serde_yaml::Value` in `setup.md` Step 4 → corrected to `yaml_serde::Value` in implementation. The setup.md sketch was wrong; implementation is right.
