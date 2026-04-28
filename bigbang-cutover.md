# BigBang Cut-over — MVP Launch of Clew!

_One-time runbook for M2: cut over from ad-hoc markdown files (root `relay.md`, root `backlog.md`) to dogfooding Clew on itself. Delete this file at the end of M2._

## Prerequisites

- M1 shipped: `clew init` is implemented, tested, and `Command::Init` is wired in `src/cli.rs` (no longer returns `Unimplemented`).
- Quality gate green on `main` before starting.

## Steps

### 1. Add the wrapper script

Create `./clew` at the repo root (executable bit set):

```sh
#!/usr/bin/env bash
set -euo pipefail
cargo build --quiet
exec ./target/debug/clew "$@"
```

`chmod +x clew`. From here on, every `clew` invocation in this repo goes through `./clew` — never `cargo run` and never bare `target/debug/clew`.

### 2. Update AGENTS.md

- Add a `## Clew workflow` section (somewhere near `## Plan & Relay discipline`) covering:
  - **Always invoke as `./clew`** (rebuilds-then-runs; never use `cargo run` or bare binary).
  - **Documented commands:** `init`, `new`, `start`, `done`, `show`, `list`. Other commands are wired and discoverable via `./clew --help` but aren't part of the documented loop yet.
  - **Session-start protocol:** read `.clew/relay.md`; wait for the user to direct you to a specific increment ID (`./clew show <id>`); if no direction, `./clew list` and ask.
- Update `## Resources` so the relay pointer reads `.clew/relay.md` (not `relay.md`).

### 3. Run init + migrate state

```sh
./clew init
rm .clew/relay.md           # init creates an empty one; we'll move the real one in
git mv relay.md .clew/relay.md
```

### 4. Create increments from backlog.md

Three "Ideas" → backlog increments, one "Rejected" → abandoned increment:

```sh
./clew new "Clew new missing from clew --help"
./clew new "path.md in-progress section"
./clew new "clew promote command"

./clew new "clew touch / lint --fix post-hand-edit reconciler"
# Note the ID printed above, then:
./clew abandon <id> "Collapses 'direct edit is first-class' into a two-step CLI workflow; --force in disguise; reconciliation requires intent inference the CLI can't do safely; overlaps with clew lint's advisory role. Instead: lint stays advisory, and done/abandon/reopen tolerate already-flipped state with a warning. Revisit only if hand-edit-to-terminal-status becomes a real pattern."
```

Sharpen each new increment's body (paste the deferral reasoning from the old `backlog.md` into the body) before declaring done.

### 5. Delete the old surfaces

```sh
git rm backlog.md
```

### 6. Add a follow-up backlog item

Per session decision: the `crew-plan.md` Revisions entry for "deferred #0000 bootstrap" is now backlog, not part of M2:

```sh
./clew new "Plan revision: note that #0000 bootstrap was deferred for self-host"
```

### 7. Verify

```sh
./clew lint                                  # should be clean
./clew list                                  # should show backlog items
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

### 8. Milestone close

- Update `.clew/relay.md` (the new home) per the milestone-close protocol.
- `git rm bigbang-cutover.md` — runbook is one-shot.
- Get user approval, then commit.
