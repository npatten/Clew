# Interaction Paths

See `IP-notes.md` for the template and rules.

This doc enumerates concrete interaction paths between actors (humans, agents) and Clew. We brainstorm titles first, agree on coverage, then fill each path out. Cross-cutting observations accumulate at the bottom and feed into `CLI-design-notes.md`.

---

## Candidate paths (brainstorm — titles + actor only)

> Status: brainstorming. Not yet written out. Goal: confirm coverage of the problem space before drafting full paths. Add, remove, merge, split freely.

### 1. Onboarding

- **IP-01: Human initializes Clew.** _(human, operator)_ — `clew init`; what gets created, what's the first thing they read?
  - _Variants:_ existing project (code already there) / brand-new project (empty repo).
- **IP-02: Fresh agent on a new-to-it Clew project.** _(agent, fresh)_ — agent has never seen this `.clew/` before; bootstraps via `.clew/README.md`, path, in-flight increments.

### 2. Steady-state work

- **IP-03: Fresh agent picks up work.** _(agent, fresh)_ — canonical loop. Reads path, increment, relay, starts.
  - _Variants:_ clean start (no in-progress increment) / resuming an in-progress increment with an existing relay.
- **IP-04: Mid-session agent finishes an increment and starts the next.** _(agent, mid-session)_ — `clew done`, archive, path update, relay disposition; then orient on the next increment without losing context.
- **IP-05: Agent ends session mid-increment and writes a relay.** _(agent, mid-session)_ — handoff to future self or another agent.
- **IP-06: Human directs agent to specific increment (overriding path).** _(human + agent)_ — "work on #0044 next, ignore path."
- **IP-07: Agent works fragments instead of an increment.** _(agent)_ — picks tasks off `fragments.md`; what's the loop look like?

### 3. Triage & curation

- **IP-08: Capture new work.** _(any)_ — adds an item to the backlog via `clew new`.
  - _Variants:_ actor (human curator / agent mid-work surfacing a discovery) × type (epic / increment) × scope clarity (rough notion / fully scoped). Also covers human bulk-capturing a first batch at onboarding.
- **IP-09: Decompose a plan, PRD, or spec into epics and increments.** _(human, curator — possibly agent-assisted)_ — reads the source doc, identifies the epic boundary, breaks into increments, seeds task checklists; optionally links items back to the source. Decomposition is human/agent judgment work; Clew's role is to receive the output without getting in the way.
  - _Variants:_ human-led (curator drives every decision) / agent-assisted (agent proposes decomposition from the doc, human reviews and edits) / collaborative (back-and-forth).
- **IP-10: Human triages backlog → todo.** _(human, curator)_ — sharpens raw items, promotes ready ones.
- **IP-11: Human reorders `path.md`.** _(human, curator)_ — priority shift. What does the CLI help with vs. just opening the file?
- **IP-12: Human reviews what's in flight.** _(human, reviewer)_ — wants a quick read of "where are we?" without opening 5 files.

### 4. Exception handling

- **IP-13: Increment becomes blocked mid-work.** _(agent or human)_ — sets blocked flag, writes reason, what happens to path?
- **IP-14: Increment is abandoned mid-work.** _(human, curator)_ — explicitly dropped with reason; archive vs. delete; relay disposition.
- **IP-15: Increment scope drifts during work.** _(agent, mid-session)_ — discovers it's bigger than thought; split? extend? note in relay?
- **IP-16: Two agents create conflicting IDs.** _(system, conflict)_ — merge produces two `0042` files; triggers IP-23.
- **IP-17: Reopen a closed increment.** _(agent or human)_ — `clew reopen`; restores file, frontmatter, relay; status reset.
  - _Variants:_ from `done` (fixing something post-completion) / from `abandoned` (resuming dropped work).
- **IP-18: Agent gets stuck and needs human input.** _(agent → human)_ — flags `[Human?]` somewhere; how does the human see it?
- **IP-19: Increment file becomes inconsistent (lint failures).** _(human, operator)_ — `clew lint` finds drift; what does fixing look like?

### 5. Inspection

- **IP-20: Actor queries Clew state.** _(any)_ — read-only.
  - _Variants:_ "what's next?" (`clew next`) / "what's in flight?" / filter by tag (`--tag p0`) / show specific increment by ID or slug.
- **IP-21: Human searches across all clew files for a keyword.** _(human)_ — likely just `rg`, but worth confirming the CLI doesn't need to help.
- **IP-22: Human reviews recent activity.** _(human, reviewer)_ — what changed in `.clew/` lately? Git-driven or CLI-driven?

### 6. Lifecycle edges

- **IP-23: Renumbering after ID conflict.** _(human, operator)_ — `clew renumber`; what gets rewritten? (Response to IP-16.)
- **IP-24: Archiving cleanup (manual).** _(human, operator)_ — does `.clew/archive/` ever need pruning? Or trust git?

---

## Path bodies

> To be filled in once the candidate list is agreed. Cross-cutting observations and CLI-design implications go in a separate `CLI-design-notes.md` (started once enough paths are written to surface real patterns).
