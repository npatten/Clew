---
id: 15
status: abandoned
abandoned_reason: Pulled relay concept from design; increments are carrying cross-session context well enough on their own. Parked here in case we revive it.
created_at: 2026-04-28T04:24:33Z
updated_at: 2026-04-28T04:26:31Z
---
## Context

Original concept (from earlier `crew-plan.md` and `AGENTS.md`):

- **Relay** — an ephemeral transition of context between agent sessions. Captures what doesn't live anywhere else (discoveries, next-actions, open questions, drift from plan). Anything essential to an increment belongs in the increment; the relay is strictly meta-context.
- **Single rolling `.clew/relay.md`** at the top of `.clew/`, overwritten each session. Git provides history. **Not archived** with increments.
- **Boundary with the increment file:** plan/criteria/tasks/file paths the next session must touch live in the increment. The relay is for in-flight discoveries, judgment calls, gotchas. Anything recoverable from `git log` or the archived increment does not belong here.
- **Rationale for single rolling file:** design assumes 1–2 primary agents at a time, working a single in-progress increment per session. Per-increment relay solves a problem that doesn't exist at this scale; can be added later as an additive change.

### Lifecycle

- Written manually at session end via `clew relay`.
- CLI opens existing relay (or template), pre-fills with `git log` since last relay, pre-fills task checkbox state for current focus increment, agent fills the rest, save + timestamp.
- Manual > auto-generated — value is in *judgment* about what's worth carrying forward.
- `clew done` does **not** touch `relay.md`. Next session overwrites it naturally.

### Format

```markdown
---
work-completed: [ ]   # array of completed increments; usually one
updated_at: 2026-04-26T15:30:00Z
---

# Relay:

## Context worth carrying
- Highest-value section. Things that took time to learn.
- Discoveries, gotchas, decisions that didn't have a home and their reasoning.
- Code references with file:line where useful.

## Open questions
- [Human?] Things needing human input.
- [Decide] Things the next agent should pick a direction on.

## Drift from plan
- New tasks discovered outside the original increment scope.
- Original tasks revealed as unnecessary.
```

### Path/relay relationship

- `path.md` = project-wide priority (organized backlog).
- `relay.md` = session-handoff context (efficient handoff between agent sessions).

Typical session loop included: read `relay.md` → `clew next` → work → `clew done` → maybe edit `path.md` → `clew relay` at session end → commit.

### CLI surface

- `clew init` scaffolded an empty `relay.md` alongside `path.md` and `README.md`.
- `clew relay` — open/edit `.clew/relay.md` (no ID arg — single rolling file).
- `clew done` deliberately did not touch `relay.md`.

### Writing discipline (from AGENTS.md)

- Goal: capture what's expensive to re-derive next session.
- `Next milestone` points at the next product chunk, not process mechanics (review, gate, commit).
- Skip lines that go stale on approval ("pending review", "ready to commit") — those belong in chat.
- Capture decisions and gotchas the next agent would otherwise lose time on.
- Prefer exact paths, command names, commit hashes already at hand.
- Don't restate `crew-plan.md`; reference it.
- Post-commit: drop the play-by-play. Once `[#NNNN]` is committed, commit + archived increment are the record.

### Deferred hook concern

The `prepare-commit-msg` hook deferral cited relay staleness as a concrete risk: the hook would need to know the active increment, but `relay.md` can lag the agent's actual focus (e.g., `clew start 0050` ran but no relay yet — hook prepends prior `#0042`).

## Why abandoned (2026-04-28)

In practice we found we just weren't using the relay — increments themselves are proving a great place to store cross-agent context, and the rolling relay file added noise without earning its keep. Pulling all relay-related material out of `crew-plan.md` and `AGENTS.md` and parking the design here in case we want to revive it later.

## Revival notes (if we ever come back to this)

- If concurrent in-flight work becomes real, reconsider a per-increment relay rather than reviving the single rolling file.
- Consider whether the value can be captured by a richer "session notes" section inside the active increment instead of a separate file.
- Any revival should justify the cost of a second context surface (where does it live, when does it get pruned, how do agents avoid duplicating increment content into it).
