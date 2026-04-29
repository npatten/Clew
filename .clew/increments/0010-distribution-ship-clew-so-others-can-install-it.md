---
id: 10
title: "distribution: ship clew so others can install it"
status: backlog
created_at: 2026-04-28T03:20:05Z
updated_at: 2026-04-28T03:20:05Z
---

Stand up a real distribution story so developers outside this repo can install and use Clew on macOS, Linux, and (eventually) Windows.

## Strategy

Use [`dist`](https://github.com/axodotdev/cargo-dist) (formerly `cargo-dist`) as the primary release tool. Clew is a single Rust binary CLI — dist's sweet spot — and it collapses pre-built binaries, a Homebrew tap, and a curl-to-bash installer into one config triggered by a git tag.

`cargo install clew` remains supported as the Rust-user fallback path (and the simplest thing to ship first).

## Open questions

- **Binary name collision**: `clew` is a common word. Check crates.io and Homebrew for conflicts *before* any release infra lands — if we need to rename, every other step depends on the final name.
- **Release-build path for `./clew`**: the local wrapper rebuilds the debug binary today. Decide whether installed users get a fully separate `clew` on `$PATH` (preferred) or whether the wrapper grows a release mode. Default: leave the wrapper alone, installed users use the dist-built binary.
- **Init UX for new users**: after install, what does the first `clew init` experience look like for a fresh project? Worth a quick walkthrough before tagging v0.1.0.
- **crates.io publish automation**: let dist run `cargo publish`, or keep it manual for v1?

## Plan

1. **Resolve the name collision** (gate). Search crates.io and `brew search`; if taken, pick a final name and update the repo before going further.
2. **Publish to crates.io** so `cargo install clew` works. Smallest viable v1.
3. **Adopt dist**: run `dist init`, commit `release.yml` + `[workspace.metadata.dist]`. Targets: macOS (arm64 + x86_64) and Linux (x86_64). Installers: shell (curl-to-bash) and Homebrew tap.
4. **Cut v0.1.0** by pushing a tag; verify the GitHub Release, the shell installer, and the brew formula all work end-to-end on a clean machine.
5. **Document install** in the README: brew, curl, cargo — in that order.

## Deferred

- Windows targets (covered by separate native-Windows / WSL increments).
- MSI / signed installers, code signing, notarization.
- An [oranda](https://github.com/axodotdev/oranda) landing page.
- Letting dist drive `cargo publish` automatically.

## Costs to keep in mind

- `release.yml` is real surface area — readable YAML, but ours to maintain.
- dist self-hosts: published artifacts are built by the *previous* dist version, so schema-incompatible upgrades occasionally need care.
- axo.dev is a small shop; bus factor is non-zero. Mitigated by OSS + readable generated CI.
