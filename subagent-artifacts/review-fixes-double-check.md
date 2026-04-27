# Review: Clew scaffold fixes double-check

## Findings

- Correct: unknown frontmatter fields are deterministic and value-preserving. `Increment.extra` is `BTreeMap<String, yaml_serde::Value>` with `#[serde(flatten)]` (`src/core/increment.rs:42-47`), the parser deserializes through `yaml_serde` (`src/core/frontmatter.rs:32-33`), and tests now assert full `extra` equality plus deterministic serialization (`src/core/frontmatter.rs:96-113`).
- Correct: padded ID policy is implemented in code. `id` and `parent` are `u32` (`src/core/increment.rs:29-32`), tests/fixtures use plain integer YAML (`src/core/frontmatter.rs:240`), matching the explicit policy in `hammock-thinking/crew-plan.md:121-123`.
- Note: `hammock-thinking/crew-plan.md` still has one stale contradictory example: `ID in frontmatter: id: 0042` (`hammock-thinking/crew-plan.md:131-135`) conflicts with the plain-integer policy immediately above (`hammock-thinking/crew-plan.md:121-123`). Relay correctly calls out plain integers (`relay.md:16-18`, `relay.md:41-42`).
- Not fixed/deferred: strict timestamp validation is still absent. The plan requires `Z` suffix, UTC, and second precision (`hammock-thinking/crew-plan.md:222-227`), but the implementation relies on `chrono::DateTime<Utc>` serde with no custom validation (`src/core/increment.rs:39-40`, `src/core/frontmatter.rs:32-33`). Tests only cover valid timestamp parsing (`src/core/frontmatter.rs:227-233`), not rejection of non-`Z` offsets or subseconds. Relay accurately documents this as deferred (`relay.md:24`, `relay.md:45`).
- Correct: stronger parser tests are present. Required-field cases include missing `created_at` (`src/core/frontmatter.rs:157-174`), unknown status is covered (`src/core/frontmatter.rs:176-181`), optional parent/tags are covered (`src/core/frontmatter.rs:183-205`), and the snapshot exists (`src/core/frontmatter.rs:236-243`; `src/core/snapshots/clew__core__frontmatter__tests__snapshot_round_trip.snap`).
- Correct: command stubs return typed unimplemented errors instead of panicking for the scaffolded command modules (`src/commands/new.rs:3-5`, `src/commands/show.rs:3-5`, `src/commands/list.rs:3-5`, `src/commands/start.rs:3-5`, `src/commands/done.rs:3-5`, `src/commands/next.rs:3-5`). Other CLI variants fall through to `Err(ClewError::Unimplemented)` (`src/cli.rs:67-80`), and `main` maps that to exit code 1 (`src/main.rs:8-13`).
- Correct: version smoke test asserts both command name and package version (`tests/integration_test.rs:4-11`).
- Correct: relay is broadly accurate on completed fixes and validation counts (`relay.md:8-24`). The only caveat is the timestamp strictness is accurately marked deferred, not fixed.

## Validation commands/results

- `cargo build` — passed.
- `cargo fmt --check && cargo clippy -- -D warnings && cargo test` — passed. Results: 17 unit tests passed, 1 integration test passed, 0 failures.
- `cargo run -- --version` — passed; output: `clew 0.1.0`.
- `cargo run -- show 1` — returned exit code 1 with `error: not yet implemented`, confirming typed stub behavior.

No source edits made by this review.
