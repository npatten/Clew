# Setup — Clew Scaffolding Task

> **One-time instructions for the agent picking up scaffolding.**
> This document is self-contained; you should not need this conversation's history. Design context lives in [`hammock-thinking/crew-plan.md`](hammock-thinking/crew-plan.md). Session handoff context lives in [`relay.md`](relay.md).

## Goal

Deliver the **"skeleton + frontmatter parser"** milestone:

- `cargo new` Rust project with all dependencies wired in `Cargo.toml`
- Module structure exists per the locked layout (with `unimplemented!()` stubs where appropriate)
- One smoke test: `clew --version` returns a version string
- **`core/frontmatter.rs` fully implemented with thorough unit tests** — this is the lynchpin
- `clew init` template files exist as placeholders
- All other commands present as stubs that compile and return `unimplemented!()`

This is one session's worth of work. Do not scope-creep into implementing other commands — `clew show`, `clew list`, etc., come in the next session as their own vertical slices.

---

## Why this scope

- **Skeleton + smoke test alone** leaves the next agent staring at empty modules and re-deriving foundations.
- **Skeleton + parser + first command** bundles too much: harder to review, more failure surface in one PR.
- The frontmatter parser is the **riskiest piece** (preservation of unknown fields, edge cases). Doing it standalone gets it scrutinized as its own artifact, separate from CLI plumbing.

---

## Step-by-step

### 1. Initialize the Rust project

```bash
cargo init --bin --name clew .
```

You'll need both a binary target (`main.rs`) and a library target (`lib.rs`). After `cargo init`, add the lib by creating `src/lib.rs`. The binary should be a thin wrapper that calls `clew::run()`.

### 2. Configure `Cargo.toml`

Pin to current stable versions (check crates.io at scaffold time):

```toml
[package]
name = "clew"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", default-features = false, features = ["derive", "std", "help"] }
serde = { version = "1", features = ["derive"] }
yaml_serde = "*"        # Official YAML org's actively maintained fork — https://crates.io/crates/yaml_serde
chrono = { version = "0.4", default-features = false, features = ["clock", "serde"] }
slug = "0.1"
directories = "5"
thiserror = "1"
anyhow = "1"

[dev-dependencies]
assert_cmd = "2"
assert_fs = "1"
predicates = "3"
insta = "1"
rstest = "0.18"

[lib]
name = "clew"
path = "src/lib.rs"

[[bin]]
name = "clew"
path = "src/main.rs"
```

**Note on `yaml_serde`:** confirmed as the right pick — the official YAML organization's actively maintained fork (https://crates.io/crates/yaml_serde), replacing the archived (and security-flagged) `serde_yaml`. Do NOT substitute `serde_yml` (community fork — adds confusion, no equivalent provenance) or the original `serde_yaml`.

### 3. Create the module structure

Use **modern module style** — `core.rs` + `core/` directory, not `core/mod.rs`.

```
src/
├── main.rs              # Thin: calls clew::run()
├── lib.rs               # Public API, re-exports run() and types
├── cli.rs               # clap derive structs (Cli, Command enum)
├── error.rs             # thiserror types (ClewError)
├── core.rs              # `pub mod increment; pub mod frontmatter; pub mod path;`
├── core/
│   ├── increment.rs     # Increment struct, Status enum
│   ├── frontmatter.rs   # FULLY IMPLEMENTED — see Step 5
│   └── path.rs          # path.md parser (stub OK for this milestone)
├── storage.rs           # `pub mod fs;`
├── storage/
│   └── fs.rs            # filesystem ops (stubs OK)
├── commands.rs          # `pub mod new; pub mod show; ...`
├── commands/
│   ├── new.rs           # stub: `unimplemented!()`
│   ├── show.rs
│   ├── list.rs
│   ├── start.rs
│   ├── done.rs
│   └── next.rs
└── templates/
    └── init_readme.md   # placeholder content; iterated later
```

Each stub command should be a function with the right signature, returning `Err(ClewError::Unimplemented)` or panicking with `unimplemented!()`. The point is that the project compiles end-to-end.

### 4. Define types and errors

**`src/error.rs`** — typed errors via `thiserror`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClewError {
    #[error("increment not found: #{0:04}")]
    NotFound(u32),

    #[error("invalid status transition: {from} → {to}")]
    InvalidTransition { from: String, to: String },

    #[error("slug collision: '{0}' is already used")]
    SlugCollision(String),

    #[error("frontmatter parse error: {0}")]
    Frontmatter(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("not yet implemented")]
    Unimplemented,
}
```

Add variants as needed during implementation; this is a starting set. Map to exit codes in `main.rs`: `0` success, `1` user error (NotFound, InvalidTransition, SlugCollision), `2` system error (Io, Frontmatter, internal).

**`src/core/increment.rs`** — the core type:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Backlog,
    Todo,
    InProgress,
    Done,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Increment {
    pub id: u32,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Preserves any frontmatter fields the CLI doesn't know about.
    /// THIS IS LOAD-BEARING for the extensibility model — do not remove.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,  // adjust to yaml_serde::Value
}
```

Body content (the markdown after the frontmatter) is held alongside the struct, not inside it — see Step 5.

### 5. Implement the frontmatter parser (the lynchpin)

**File: `src/core/frontmatter.rs`**

This is the most important module in this milestone. It handles the round-trip read/write of markdown files with YAML frontmatter, **preserving unknown fields**.

**API surface (rough):**

```rust
pub struct ParsedFile {
    pub increment: Increment,
    pub body: String,
}

/// Parse a file's text into Increment + body.
pub fn parse(text: &str) -> Result<ParsedFile, ClewError>;

/// Serialize Increment + body back to markdown text.
pub fn serialize(file: &ParsedFile) -> Result<String, ClewError>;
```

**Algorithm for `parse`:**

1. Verify the file starts with `---\n`. If not, error (no frontmatter).
2. Split on `---\n` with `splitn(3, ...)`. Should yield three parts: leading empty string, YAML chunk, body.
3. Deserialize the YAML chunk into `Increment`. The `#[serde(flatten)] extra` field captures unknowns.
4. Return `ParsedFile { increment, body }` where `body` is the trailing markdown (preserved verbatim, including its own leading newline).

**Algorithm for `serialize`:**

1. Serialize `Increment` to YAML.
2. Wrap with `---\n` delimiters.
3. Append body verbatim.

**Required tests** (in-module `#[cfg(test)] mod tests`, using `rstest` for parameterized cases):

- **Round-trip preserves unknown fields**: input has `priority: high` and `jira: PROJ-1234` → parse → serialize → output is byte-identical (or at least semantically identical) to input.
- **Round-trip preserves body verbatim**: body with mixed checklist + prose + code blocks survives unchanged.
- **Missing leading `---`**: returns frontmatter error.
- **Missing closing `---`**: returns frontmatter error.
- **Empty body**: parses fine; serializes back with empty body.
- **Required fields missing** (`id`, `status`, `created_at`, `updated_at`): error.
- **Unknown status value**: error.
- **`parent` is optional**: parses fine when absent.
- **`tags` is optional**: parses fine when absent or empty list.
- **Snapshot test** with `insta`: parse a fixture, serialize back, compare to a saved snapshot. Catches accidental format drift.

Make sure the unknown-fields preservation is **bulletproof**. This is the single most-relied-upon behavior in the design.

### 6. Wire up `main.rs` and `lib.rs`

**`src/lib.rs`:**

```rust
pub mod cli;
pub mod commands;
pub mod core;
pub mod error;
pub mod storage;

pub use error::ClewError;

pub fn run() -> Result<(), ClewError> {
    use clap::Parser;
    let cli = cli::Cli::parse();
    cli.dispatch()
}
```

**`src/main.rs`:**

```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    match clew::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            // Map error variant to exit code:
            //   user errors → 1
            //   system errors → 2
            match &e {
                clew::ClewError::NotFound(_)
                | clew::ClewError::InvalidTransition { .. }
                | clew::ClewError::SlugCollision(_)
                | clew::ClewError::Unimplemented => ExitCode::from(1),
                clew::ClewError::Io(_) | clew::ClewError::Frontmatter(_) => ExitCode::from(2),
            }
        }
    }
}
```

**`src/cli.rs`** — clap derive scaffolding. Define a `Cli` struct with a `Command` subcommand enum covering all commands from the CLI sketch in `crew-plan.md`. Each variant dispatches to a stub in `commands/`. Most return `Err(ClewError::Unimplemented)` for now.

### 7. Smoke test

**`tests/integration_test.rs`:**

```rust
use assert_cmd::Command;

#[test]
fn version_flag_works() {
    let mut cmd = Command::cargo_bin("clew").unwrap();
    cmd.arg("--version")
        .assert()
        .success();
}
```

This proves: project compiles, binary runs, clap is wired correctly.

### 8. Init template stubs

**`src/templates/init_readme.md`** — a placeholder. Real content is iterated later. Suggested stub:

```markdown
# Clew — Project State

This directory holds Clew project state (increments, archive, path, relay).

> **TODO:** Real conventions and a "copy this into your AGENTS.md" section will be filled in once `clew init` is implemented for real.
```

Also create the rest of the template directory as needed (just one file for this milestone).

### 9. Verify everything passes

```bash
cargo build
cargo test
cargo clippy -- -D warnings    # optional but recommended
```

All should succeed. The frontmatter tests should be substantive (10+ test cases covering the scenarios in Step 5).

### 10. Commit

One commit for the scaffold, following the project's commit conventions (see `AGENTS.md`):

```
scaffold Rust CLI skeleton + frontmatter parser

- cargo project structure with locked deps (clap, yaml_serde, chrono, slug, directories, thiserror, anyhow)
- modern module layout: core/, storage/, commands/
- frontmatter parser: round-trip-preserving with full unit test suite
- stub commands return Unimplemented; smoke test verifies --version
- next session: implement clew show as the first vertical slice

Co-Authored-By: Clankers <noreply@anthropic.com>
```

### 11. Update relay

After committing, run through `relay.md` (root of the repo) and update it for the next session. Cover:
- Status: scaffolding done, frontmatter parser working
- Just finished: brief bullets, reference commit hash
- Next action: implement `clew show` as a vertical slice — read by ID or slug, parse the file, write to stdout
- Context worth carrying: anything surprising about the parser, library choices that didn't pan out, deviations from this setup doc
- Open questions: anything that came up

---

## Things to push back on

If anything in this plan looks wrong while you're building, **flag it in the relay and the design doc** rather than silently working around it. The design is meant to be challenged.

Specific things to validate:
- **`#[serde(flatten)]` with YAML** — confirm it round-trips cleanly. If it has surprising behavior with nested types, push back; we may need a different approach.
- **Modern module style with stubs** — if compile errors get nasty with empty submodules, fall back to `mod.rs` style. Pragmatism wins.

---

## Reference

- Design: [`hammock-thinking/crew-plan.md`](hammock-thinking/crew-plan.md) — full design doc, especially the "Implementation" and "Scaffolding milestone" sections
- Conventions: [`AGENTS.md`](AGENTS.md) — coding style, commit conventions
- Handoff: [`relay.md`](relay.md) — where the previous session left off
