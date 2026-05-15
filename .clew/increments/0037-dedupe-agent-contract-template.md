---
id: 37
status: backlog
tags:
- docs
- tech-debt
created_at: 2026-05-15T04:16:28Z
updated_at: 2026-05-15T04:16:28Z
---
## Goal

Eliminate duplication between the agent contract block embedded in `src/templates/init_readme.md` and `src/templates/bootstrap_increment.md`. Today both files carry a verbatim copy of the same "## Clew workflow" section, fenced by `====` lines. They drift easily; #0035 had to sync them by hand.

## Context

The contract block is the canonical "copy this into AGENTS.md" snippet. It appears:

- In `.clew/README.md` (rendered from `init_readme.md`) — the long-lived reference.
- In `.clew/increments/0000-bootstrap-clew.md` (rendered from `bootstrap_increment.md`) — what the first agent reads on `clew init`.

Both are produced via `include_str!` at compile time in `src/storage/fs.rs`. The two templates are independent files with no shared source.

## Design options

1. **Single source + include_str! into both consumers.** Extract the contract block into `src/templates/agent_contract.md`. `init_readme.md` and `bootstrap_increment.md` keep placeholder markers; build-time (`build.rs`) or a small runtime concat splices the contract in. Pro: one source of truth. Con: adds a tiny build step or runtime concat, and the standalone `.md` files are no longer directly readable as final output.

2. **Runtime composition in `src/storage/fs.rs`.** Each template file contains a marker like `{{AGENT_CONTRACT}}`. At init time, the seeder reads the contract from a third template and substitutes. Pro: simple, no build.rs. Con: templates are no longer pure files; minor templating engine surface.

3. **Bootstrap defers to README.** Slim `bootstrap_increment.md` down to "Open `.clew/README.md` and copy the section between the `====` lines into your AGENTS.md equivalent." No embedded contract. Pro: zero duplication, no plumbing. Con: one more hop for the first agent; relies on the README being present (it always is — `clew init` creates both).

4. **Test-only guard.** Keep both copies; add an integration test that extracts the `====`-fenced block from each rendered file and asserts equality. Pro: lowest churn. Con: doesn't fix the duplication, just catches drift after the fact.

Recommendation: **(3)** — it removes the duplication entirely with no plumbing, and the bootstrap increment is the right place to teach the first agent that `.clew/README.md` exists and is the source. Fall back to **(2)** if we decide the first agent shouldn't have to make that hop.

## Tasks

- [ ] Decide between options (3) and (2) (or surface a better one).
- [ ] Implement the chosen approach.
- [ ] Refresh affected snapshots (`init_readme_matches_snapshot`, `init_bootstrap_increment_body_matches_snapshot`).
- [ ] Run the full quality gate.

## Out of scope

- Restructuring the README beyond what the chosen approach requires.
- Wider templating engine (e.g., Tera/Handlebars) — keep it minimal.
