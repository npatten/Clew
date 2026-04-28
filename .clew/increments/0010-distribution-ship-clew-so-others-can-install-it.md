---
id: 10
title: "distribution: ship clew so others can install it"
status: backlog
created_at: 2026-04-28T03:20:05Z
updated_at: 2026-04-28T03:20:05Z
---

Decide on and implement a distribution strategy so developers outside this repo can install and use Clew.

## Open questions

- **Install method for v1**: plan notes `cargo install` as the default — is that good enough, or do we need a pre-built binary for non-Rust users?
- **Binary name collision**: `clew` is a common enough word; check crates.io / Homebrew for conflicts.
- **The `./clew` wrapper**: currently rebuilds the debug binary — the install story needs a release build path.
- **Init UX for new users**: after install, what does the first `clew init` experience look like for a fresh project?

## Candidate approaches

1. `cargo install clew` — simplest; requires Rust toolchain on the user's machine.
2. GitHub Releases pre-built binaries — broader reach; adds a CI release pipeline.
3. Homebrew tap — Mac-friendly; depends on having binaries first.
4. curl-to-bash installer — widest reach; higher support burden and security scrutiny.

Plan default is (1) for v1, revisit later.
