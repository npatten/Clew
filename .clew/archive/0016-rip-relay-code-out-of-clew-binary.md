---
id: 16
status: done
created_at: 2026-04-28T04:28:47Z
updated_at: 2026-04-28T05:00:03Z
---
## Goal

Remove the residual relay-related code/scaffolding now that the relay concept has been pulled from the design (see #0015 for the parked design, and the 2026-04-28 revisions entry in `hammock-thinking/crew-plan.md`).

## Context

`#0015` parked the relay *concept* and the docs were stripped, but the binary still scaffolds `.clew/relay.md` on `clew init` and exposes a `Relay` CLI variant. This increment cleans that up.

## Known touch points (from `rg -n "[Rr]elay" src/ tests/` on 2026-04-28)

- `src/cli.rs:63-64` — `Relay` subcommand declaration (`Open relay.md in your editor`). Note: no handler is wired in `main.rs` / `commands/`, so this is a stub.
- `src/storage/fs.rs:37` — `InitSpec::File(".clew/relay.md", "")` in the init scaffold list.
- `src/templates/init_readme.md:3` — README template mentions relay.
- `tests/integration_test.rs` — multiple lines pinning `.clew/relay.md` in `INIT_CREATED_STDERR` / `INIT_EXISTS_STDERR` and the init body assertions (lines ~44, 45, 67, 76, 93, 94, 114, 115, 144).
- `tests/snapshots/integration_test__init_readme_matches_snapshot.snap` — snapshot needs regen after the README template edit (`cargo insta review`).
- `.clew/relay.md` — the existing file in this repo. Delete via `git rm`.

## Tasks

- [x] Remove the `Relay` variant from `src/cli.rs`.
- [x] Remove the `.clew/relay.md` entry from `InitSpec` in `src/storage/fs.rs`.
- [x] Update `src/templates/init_readme.md` to drop the relay reference (already done in `.clew/README.md` — mirror the change in the template).
- [x] Update integration tests to reflect the new init output (no `relay.md` line in stderr, no relay file body assertions).
- [x] Regenerate the affected `insta` snapshot for the README template.
- [x] `git rm .clew/relay.md` so this repo stops carrying it.
- [x] Run the full quality gate (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`).

## Note on #0015

If anything useful surfaces while ripping this out — implementation fragments, lessons about how `clew init` scaffolds files, parser/template ideas, anything that would help a future revival — append it to #0015 (the parked relay design increment) rather than discarding it. The revival notes section there is the right home.
