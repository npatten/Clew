# Review: current Clew Rust codebase

Reviewed against `setup.md` and `hammock-thinking/crew-plan.md`, with focus on early correctness, tests, plan drift, Rust/clippy, dependencies/API choices, and frontmatter round-trip behavior.

Validation run:

- `cargo clippy -- -D warnings` passes.
- `cargo test` fails on the `insta` snapshot.
- `cargo fmt --check` fails.
- Note: running `cargo test` generated `src/core/snapshots/clew__core__frontmatter__tests__snapshot_round_trip.snap.new` because the snapshot failed. I did not edit source files.

## P0 — `cargo test` is currently failing; snapshot output is nondeterministic

- `src/core/frontmatter.rs:188-195` defines the snapshot test.
- `src/core/increment.rs:44-45` stores flattened unknown fields in `HashMap<String, yaml_serde::Value>`.
- `src/core/snapshots/clew__core__frontmatter__tests__snapshot_round_trip.snap:15-16` records `priority` before `jira`, but the latest test run emitted `jira` before `priority`.

Why this matters: `HashMap` iteration order is randomized, so serialized frontmatter with unknown fields can drift between runs/processes. That breaks snapshot stability and makes the frontmatter output unreliable as an agent-facing API.

Suggested fix:

- Replace `HashMap` for `extra` with a deterministic/order-preserving map.
  - If semantic deterministic output is enough: `BTreeMap<String, yaml_serde::Value>`.
  - If preserving input order matters: add a direct `indexmap` dependency with serde support and use `IndexMap<String, yaml_serde::Value>`.
- Update/accept the snapshot only after ordering is deterministic.

## P0 — zero-padded IDs in the plan are not represented or tested correctly

- `src/core/increment.rs:29` models `id` as `u32`.
- `src/core/increment.rs:31-32` models `parent` as `Option<u32>`.
- Tests use unpadded values: `src/core/frontmatter.rs:61`, `src/core/frontmatter.rs:141`, `src/core/frontmatter.rs:192`.
- Current snapshot serializes unpadded values: `src/core/snapshots/clew__core__frontmatter__tests__snapshot_round_trip.snap:6` (`id: 42`) and `:8` (`parent: 7`).

Why this matters: the design/frontmatter examples use `id: 0042` and `parent: 0007`, and the ID scheme says IDs are zero-padded 4-digit IDs. With `yaml_serde`/YAML 1.2, leading-zero scalars like `0042` are treated as strings, not integers, so the intended frontmatter shape is likely to fail deserialization into `u32`. Serialization also writes `42`, not `0042`.

Suggested fix: make an explicit choice now:

1. Preferable for simplicity: change the data-model contract to numeric unpadded frontmatter (`id: 42`, `parent: 7`) while keeping filenames/references zero-padded (`0042-...`, `#0042`). Update `crew-plan.md`, templates, and snapshots.
2. If frontmatter must be zero-padded: introduce custom serde for ID fields that accepts both numeric and digit-string values and serializes canonically. Add tests for `id: 0042`, `id: "0042"`, `parent: 0007`, and serialization output.

## P1 — unknown-field preservation tests are too weak for the load-bearing behavior

- `src/core/frontmatter.rs:66-75` checks only that unknown keys exist after parse/serialize/reparse.
- `src/core/increment.rs:44-45` is the load-bearing flatten field.

Why this matters: the plan calls unknown-field preservation the most important parser behavior. The current test would still pass if unknown values changed type/content, if nested unknown objects were altered, or if list values were damaged, as long as the keys survived.

Suggested fix:

- Assert values, not just keys, e.g. `priority == "high"`, `jira == "PROJ-1234"`.
- Compare `parsed.increment.extra == reparsed.increment.extra` after serialize/reparse.
- Add cases for nested maps, arrays, booleans, numbers, and strings containing `#`.
- After fixing map determinism, add a deterministic snapshot covering these unknown-field shapes.

## P1 — timestamp format rules from the plan are not enforced

- `src/core/increment.rs:39-40` uses plain `DateTime<Utc>` serde.
- `src/core/frontmatter.rs:180-185` only verifies one valid timestamp parses.

Why this matters: the plan requires UTC `Z` timestamps with second precision and no subseconds. `chrono` serde is permissive: it accepts RFC3339 offsets and subseconds, and values with subseconds can serialize back with subseconds. That drifts from the stated frontmatter contract.

Suggested fix:

- Add a small custom serde module for Clew timestamps that serializes as `%Y-%m-%dT%H:%M:%SZ` and rejects non-`Z` offsets or subsecond precision if the plan should be strict.
- Add tests for rejecting/normalizing `+02:00` offsets and `.123Z` subseconds.

## P1 — repository is not rustfmt-clean

- `src/cli.rs:1-5` is reordered/reflowed by rustfmt.
- `src/core/frontmatter.rs:24-33` and `src/core/frontmatter.rs:119-126` are reflowed by rustfmt.

Evidence: `cargo fmt --check` exits nonzero and prints diffs.

Suggested fix: run `cargo fmt` and include it in the normal validation gate.

## P2 — required-field test coverage misses `created_at`

- `src/core/frontmatter.rs:119-126` tests missing `updated_at`, `id`, and `status`, but not missing `created_at`.

Why this matters: `setup.md` explicitly lists `id`, `status`, `created_at`, and `updated_at` as required-field error cases.

Suggested fix: add a `rstest` case with `created_at` omitted.

## P2 — command stubs panic instead of returning the typed `Unimplemented` error

- `src/commands/new.rs:3-4`
- `src/commands/show.rs:3-4`
- `src/commands/list.rs:3-4`
- `src/commands/start.rs:3-4`
- `src/commands/done.rs:3-4`
- `src/commands/next.rs:3-4`
- `src/main.rs:8-13` has an explicit exit-code mapping for `ClewError::Unimplemented`.

Why this matters: `unimplemented!()` panics, bypasses the typed error path, and exits as a panic rather than the planned `error: not yet implemented` with exit code 1. `setup.md` permits panicking stubs, so this is not a milestone blocker, but the existing `main.rs` mapping suggests the intended behavior is already available.

Suggested fix: change stubs to `Err(ClewError::Unimplemented)`.

## P2 — smoke test does not assert that `--version` returns a version string

- `tests/integration_test.rs:3-7` only asserts success.

Why this matters: `setup.md` asks for a smoke test proving `clew --version` returns a version string. The current test would pass even if stdout were empty and the command exited successfully.

Suggested fix: assert stdout contains the package name and version, e.g. `clew 0.1.0` or `env!("CARGO_PKG_VERSION")`-derived text.

## Things that look aligned

- `Cargo.toml:6-21` includes the planned core/dev dependencies and uses `yaml_serde`, not `serde_yaml`.
- The modern module layout is present (`src/core.rs` + `src/core/`, `src/commands.rs` + `src/commands/`, `src/storage.rs` + `src/storage/`).
- `src/main.rs:3-17` has the planned thin binary wrapper and exit-code mapping.
- `src/templates/init_readme.md:1-5` provides the requested placeholder template.
- `src/core/frontmatter.rs:15-44` keeps body content separate from the `Increment` struct and appends body verbatim on serialize.
