---
topic: DDD interaction-paths drafting
updated_at: 2026-04-26
session_count: 2
---

# Relay: DDD interaction paths

## Status

Mid-stream on DDD work in `hammock-thinking/DDD/`. Meta-doc and 24-path candidate list are locked. `CLI-design-notes.md` is open and accumulating. **No path bodies drafted yet** — that's next session's main work.

## Just finished

- Reconciled `IP-notes.md` template against `initial-nikko-rough-draft-IP.md`: mermaid optional, added Variants and Notes, removed Forks. Pointed Agent role at `crew-plan.md` (the working spec) rather than `README.md` (public intro).
- Collapsed candidate IPs 37 → 24 in `interaction-paths.md`. Added new IP-09 (decompose plan/PRD/spec into epics+increments) — judgment work, Clew receives output.
- Started `CLI-design-notes.md` with two open observations: (a) editor-spawning commands vs. agent flow — four shapes sketched, no decision; (b) maybe-relays-outside-increment-frame — hold off, revisit.

## Next action

Draft IP-03 body (fresh agent picks up work — canonical loop). Use template in `IP-notes.md`. After IP-03, suggested order: IP-08 → IP-09 → IP-05.

## Context worth carrying

- **Working source of truth is `hammock-thinking/crew-plan.md`**, not `README.md`. `IP-notes.md` § Agent role makes this explicit.
- **Cross-cutting observations go to `CLI-design-notes.md` as they surface** — no waiting for synthesis. Backfill source-path references when bodies are drafted.
- **Editor-spawning question is unresolved** — don't lock in a shape while drafting; just note the call-and-edit pattern each path implies. The optimal shape is a token-cost question across IP coverage.

## Open questions

- [Decide] Path body draft order (proposal in "Next action").
- [Human?] Project-root `relay.md` location is a manual workaround until Clew exists. Fine to keep there?

## Drift from plan

- Relay format spec'd `increment:` in frontmatter; this relay uses `topic:` because there's no increment to attach to. Captured as an open observation in `CLI-design-notes.md` — don't generalize from it yet.
