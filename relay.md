---
topic: Clew CLI — clew start shipped; promote deferred; clew done is next
updated_at: 2026-04-27T14:00:00Z
---

# Relay: Clew CLI — start shipped, done with self-loop tolerance is next

## Status

`clew start` is complete end-to-end. Quality gate green: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (40 passing, 10 new for `start`). Plan updated with a "Direct edit is first-class" principle that shifted the roadmap: `clew promote` is deferred; `clew done` is the next vertical slice and gets new tolerance behavior.

## Just finished

- **`clew start`** in `src/commands/start.rs`: resolves ID-or-slug via `fs::resolve`, validates `backlog|todo → in_progress` (anything else is `InvalidTransition`), bumps `updated_at` to second-precision UTC, preserves unknown fields + body via `frontmatter::serialize`. stdout empty; stderr `Started #NNNN`, plus `warning: #NNNN is blocked: <reason>` when `blocked_reason` is set.
- **`fs::write_increment(path, contents)`** added in `src/storage/fs.rs` as the shared overwrite seam for transition commands. Symmetric with `write_new_increment`. All transition commands should go through it.
- **`Start` CLI arg widened** from `u32` to `String` in `src/cli.rs`. Other transition commands (`Promote`/`Block`/`Unblock`/`Done`/`Abandon`/`Reopen`/`Renumber`) still take `u32`; widen as each ships.
- **Plan updates** in `hammock-thinking/crew-plan.md`:
  - New §"Direct edit is first-class" subsection under §Statuses & transitions: hand-editing `status:` and frontmatter is a co-equal path; CLI is convenience, not gate; `clew lint` is advisory, not corrective.
  - §"Allowed transitions" gained a paragraph on **self-loop tolerance for terminal-side-effect transitions** (`done`/`abandon`/`reopen`): hand-edited-then-CLI workflow completes side effects with a terse `warning: #NNNN already marked done; completing archive`. `clew start` explicitly excluded from this tolerance.
  - §"CLI sketch" `clew promote` line marked deferred — direct frontmatter edit suffices for `backlog → todo`.
- **`backlog.md`** new "Rejected (with reasoning)" section records the `clew touch` / `clew lint --fix` rejection so future-us doesn't re-litigate. `clew promote` deferral also captured under "Ideas".
- **Interview artifact**: `hammock-thinking/promote-interviews.md` captures GPT 5.5, Gemini Pro, Opus 4.7 takes on `promote` plus a synthesis. Drove the deferral decision.

## Next action

Implement `clew done` as the next vertical slice. This is the **second** read-mutate-write transition, so it's the right time to extract a shared `transition()` helper rather than duplicating `start`'s body. Steps:

1. Widen `Done { id }` from `u32` to `String` in `src/cli.rs`; route through `commands::done::run(&id)`.
2. Extract a shared helper. Tentative shape: `commands::transition::run(query, allowed_from: &[Status], to: Status, on_success: Fn)` or similar — but design for the actual call sites, don't pre-design. `start` becomes a one-liner that calls it; `done` adds the side-effect closure.
3. **Side effects on success**: `git mv`-equivalent file move from `.clew/increments/` to `.clew/archive/` (use `std::fs::rename`; we don't shell out to git — that's the user's job per plan §"Git integration"). Add `fs::archive_increment(path) -> PathBuf` as the move seam.
4. **Self-loop tolerance** (per plan): if status is already `done` but the file is in `increments/`, complete the archive move and emit `warning: #NNNN already marked done; completing archive`. Don't bump `updated_at` in this case — the operator's hand-edit timestamp wins; the CLI is just finishing side effects.
5. **`path.md`**: `clew done` removes `#NNNN` from `path.md`. We don't have a path.md writer yet — add `core::path::remove(path_md_text, id) -> String` (pure function) and `fs::read_path_md` / `fs::write_path_md`. Self-healing: on write, normalize remaining entries to current ID+slug form (per plan line 282).
6. **Output**: stderr `Done #NNNN` (terse, matches `Started #NNNN`). stdout empty.
7. **Integration tests**: happy path (`in_progress → done`), invalid-from rejections (e.g., `backlog → done` not allowed), self-loop tolerance with warning, ID and slug lookup, `path.md` removal, archive move verification, unknown-field preservation.

`clew abandon` and `clew reopen` follow the same pattern (file move + tolerance + warning); they should reuse the same helpers but ship as separate slices.

## Context worth carrying

- **Terse warning pattern is locked in** as a token-frugal principle. `warning: #NNNN <reason>` — short enough that the operator already knows context. Don't write explanatory warnings.
- **`start` does NOT get self-loop tolerance** — only terminal-side-effect transitions (`done`/`abandon`/`reopen`) do. Already-`in_progress` start stays `InvalidTransition` to surface stale assumptions. The asymmetry is intentional: tolerance exists to *complete side effects*, not to make every command idempotent.
- **`fs::write_increment(path, contents)`** is the existing-file overwrite seam (added in this session). All transition commands should go through it; don't `std::fs::write` from `commands/` directly.
- **`fs::resolve(root, query)`** handles padded ID, unpadded ID, and slug across `increments/` then `archive/`. Use it for any ID-or-slug query.
- **Timestamp invariant**: RFC 3339 UTC with `Z`, second precision, no subseconds. The round-trip pattern is `Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true).parse().expect("RFC 3339 round-trip")`. Used in `commands/new.rs` and `commands/start.rs`.
- **Frontmatter round-trip preserves unknown fields and body verbatim** via `frontmatter::parse` + `frontmatter::serialize`. Don't bypass.
- **Output discipline**: stdout = data, stderr = status/errors/warnings. `clew new` → padded ID on stdout. `clew list` → data lines on stdout. `clew start` → empty stdout, status on stderr. Maintain this for every transition.
- **YAML quoting note**: `yaml_serde` emits single quotes for `#`-bearing strings (e.g., `blocked_reason: 'waiting on #0039'`). Both single and double quotes are valid YAML; tests should match the canonical (single-quote) output, not whatever was hand-written in the input fixture.
- **Quality gate before each milestone**: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`. All three must pass before commit/relay.

## Decisions locked in this session

- **`clew promote` deferred.** Direct frontmatter edit (`status: backlog` → `status: todo`) is first-class. The transition has no side effects, and the operator is usually in the file sharpening the body anyway. Revisit if MVP self-hosting reveals real friction.
- **`clew touch` / `clew lint --fix` rejected outright.** Reconcile-after-hand-edit is `--force` in disguise; collapses the direct-edit principle; requires intent inference the CLI can't do safely. Recorded in `backlog.md` under "Rejected (with reasoning)" so this doesn't get re-proposed.
- **`clew lint` stays advisory.** Surfaces drift, names the right command, never silently fixes. The original transition command is the reconciliation path.
- **Self-loop tolerance is the answer for hand-edit-then-CLI workflows.** `done`/`abandon`/`reopen` tolerate already-flipped status, complete side effects, emit terse warning. `start` does NOT — pure-metadata transition with no side effects to complete.
- **Extract `transition()` helper on the second use, not the first.** That moment is now (`clew done` is #2). Design for actual call sites, not hypothetical future ones.
