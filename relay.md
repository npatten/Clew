---
topic: Clew CLI scaffolding (pre-bootstrap)
updated_at: 2026-04-26T20:00:00Z
---

# Relay: Clew CLI scaffolding

## Status

Design phase complete. All architectural decisions locked in `hammock-thinking/crew-plan.md`. Ready to scaffold the Rust project per `setup.md`. The next session is the first real code — bootstrap the project, implement the frontmatter parser, hand off the next vertical slice (`clew show`) to a follow-up session.

## Just finished

- Walked through ~10 design decisions in interview format: CLI framework (`clap` stripped), YAML library (`yaml_serde` + manual `---` splitter), project layout (modern module style, lib + bin), error handling (`thiserror` + `anyhow` layered), testing stack (`assert_cmd` + `assert_fs` + `insta` + `rstest`), `clew init` behavior (creates `#0001-bootstrap-clew`), slug rules (`slug` crate + 50-char trunc + error-on-collision), ID allocation (scan-and-increment over both `increments/` and `archive/`), timestamps (RFC 3339 UTC, second precision, `chrono`), editor resolution (user-level config via `directories` crate).
- **Revised the relay model:** dropped per-increment relays in favor of a single rolling `.clew/relay.md` at top level. Per-increment was over-engineered for the actual workflow.
- Decided scaffolding scope: "skeleton + frontmatter parser" (not just smoke test, not full first command).
- Updated `hammock-thinking/crew-plan.md` extensively: new sections for Implementation (stack, layout, error model, testing, editor resolution), Scaffolding milestone, Slug rules, ID allocation; rewrote Relay section; updated CLI sketch; trimmed Open questions.
- Wrote `setup.md` at root: self-contained scaffolding instructions for the next agent.
- Retired `TODO.md` — its content was absorbed into `crew-plan.md` (design + scaffolding milestone) and `setup.md` (the actual instructions).
- Added refinements to `crew-plan.md` post-initial-capture: `abandoned_reason` frontmatter field (parallel to `blocked_reason`, persisted through archive so future agents don't retry dead ends); task parser tolerance subsection (Postel's Law — accept GFM checkbox variants on read, write canonically); new "Git integration" section codifying no-auto-commits-ever, commit-prefix convention, and explicit deferral of `prepare-commit-msg` hooks.

## Next action

Pick up `setup.md` from the root of the repo and execute it. The deliverable is a Rust project that compiles, has the locked module layout, has a fully-implemented and well-tested `src/core/frontmatter.rs`, and stubs everything else with `unimplemented!()`. Smoke test: `clew --version` returns a version string. Commit at the end.

After scaffolding lands, the session after that picks up `clew show` as the first vertical slice on the foundation.

## Context worth carrying

- **The frontmatter parser is load-bearing for extensibility.** The `#[serde(flatten)] extra: HashMap<String, Value>` pattern preserves unknown YAML fields on round-trip. Users add `priority: high` or `jira: PROJ-1234`; we preserve them. Test this thoroughly — it's the most-relied-upon parser behavior in the system.
- **YAML library is `yaml_serde`** (https://crates.io/crates/yaml_serde) — the actively maintained fork by the official YAML organization, replacing the archived `serde_yaml`. Do NOT use `serde_yml` (a community fork that adds confusion and doesn't carry the same provenance). The original `serde_yaml` is archived and had security issues — do not use it either.
- **Modern module style** (`core.rs` + `core/` directory) is preferred over `mod.rs`. If empty-submodule compile errors get nasty during stubbing, fall back to `mod.rs` style — pragmatism wins over style.
- **System-wide install, project-scoped state.** Editor preference goes in `~/.config/clew/config.toml` (via the `directories` crate), NOT in `.clew/`. This preserves the "no project-level config file" design rule.
- **stdout = data, stderr = status/errors.** Codified and tested. The markdown+frontmatter output IS the agent-facing API; snapshot tests via `insta` catch format drift.
- **Single rolling relay.** No `relays/` directory. `clew done` does NOT touch `relay.md`. If concurrent in-progress work ever becomes real, per-increment relays can be added later (small additive migration).
- **Error-on-collision for slugs**, checked across active + archive. Slugs are part of the lookup contract (`clew show <slug>`), so they must be unique forever.

## Open questions

- [Decide] Distribution method (`cargo install`? homebrew? curl-to-bash?) — default to `cargo install` for v1, defer the rest.
- [Decide] `.clew/README.md` template content — stub for scaffolding milestone, real content iterated post-MVP.
- [Decide] Git integration specifics (hooks? auto-commits? commit-message conventions). Defer.

## Drift from plan

- **Per-increment relay → single rolling relay.** Original design had `.clew/relays/{id}.md`; revised to `.clew/relay.md`. Scope reduction; matches actual single-stream workflow.
- **`clew init` creates #0001-bootstrap-clew.** Original sketch had a bare init; we're using the first increment as a real (non-toy) setup task that demonstrates the increment + task pattern by accomplishing the actual harness-integration work.
- **User-level config introduced.** Original rule was "no config file"; revised to "no project-level config" — user-level config (`~/.config/clew/config.toml`) is fine and necessary for editor preference.
