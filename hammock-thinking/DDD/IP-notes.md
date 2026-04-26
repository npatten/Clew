# Interaction Paths — How We Write Them

This is the meta-doc. It defines the shape of entries in `interaction-paths.md`. Keep it short.

## Agent role

While working on these interaction paths, if you don't already have it in context, read `hammock-thinking/crew-plan.md` — it's the living design doc for Clew (vocabulary, storage model, statuses, relay format, open questions). That's the source of truth for in-progress design decisions.

`README.md` is the public-facing intro for future humans landing on the GitHub repo, not the working spec — don't treat it as authoritative for design state.

## Purpose

Problem-space exploration (DDD inspired) before locking CLI design. We explore example interactions humans and agents have with Clew, then read across them to inform our design.

Not user stories. Not specs. **Traces of interaction** — actor, situation, sequence, forks.

## Actors

- **Agent** — a coding agent session. Sub-states: _fresh_ (no context), _mid-session_ (loaded), _recovering_ (returning after error/interruption).
- **Human** — the developer. Sub-states: _curator_ (shaping backlog/path), _reviewer_ (checking work), _operator_ (running commands themselves).
- **System** — git hooks, CI, scheduled jobs. Future; flag if a path implies one.

Sub-state matters more than role. A "fresh agent" and a "fresh human on a new project" share more than a "fresh agent" and a "mid-session agent."

## Path entry template

**Single-actor path** (most common):

```markdown
### IP-NN: <short imperative title>

**Actor:** <agent | human | any | system> — <sub-state>
**Situation:** <one or two sentences setting the scene>
**Goal:** <one line — what the actor is trying to achieve>

**Path:**

1. <action — what the actor does or reads>
2. <action>
3. ...

**Variants:** _(optional — same actor + goal, different starting state, meaningfully different path shape)_

- **IP-NNa: <variant title>** — <one-line distinguishing condition>
  1. <action>
  2. ...
- **IP-NNb: <variant title>** — <one-line distinguishing condition>
  1. <action>
  2. ...

**Notes:** _(optional — free-form, brainstormy, half-formed ideas surfaced while writing)_

- <observation, proto-idea, design hunch>

**Open questions:** _(optional — design questions exposed by this path)_

- <things this path exposes that we haven't decided>
```

Use `any` when the path applies equally to human or agent (e.g., inspection). Use a specific actor when the path's shape depends on who's driving.

**Multi-actor path** (handoffs — both actors take action in sequence):

```markdown
### IP-NN: <short imperative title>

**Actors:**

- <actor> (<sub-state>) — <role in this path>
- <actor> (<sub-state>) — <role in this path>

**Situation:** ...
**Goal:** ...

**Path:**

1. [<actor>] <action>
2. [<actor>] <action>
3. ...
```

Prefix each step with the acting actor in brackets. Only use this form when both actors genuinely act — not when one passively benefits from the other's work.

### Mermaid diagrams (optional)

Include a mermaid diagram only when the path's _shape_ matters more than its _sequence_ — non-trivial branching, loops, or multi-actor handoffs where a picture clarifies what prose can't. For linear paths, the numbered list is enough. Prose is canonical; mermaid supplements.

### Field rules

- **Title** — imperative, concrete. "Fresh agent picks up next priority work" — not "Agent workflow."
- **Path steps** — atomic. One read, one command, one decision per step. If a step is "agent thinks for a while," that's a step.
- **Variants** — same actor and goal, different starting state. If the difference is just a mid-path branch, fold it into the main path's steps. If it's a different actor or different goal, it's a separate IP, not a variant.
- **Notes** — free-form. The brainstormy ideas that surface while drafting ("maybe this reveals a proto nature of X?"). Cheap to capture, often valuable later. Synthesis across notes happens in a separate doc once enough paths exist.
- **Open questions** — design questions only. Not "what if the file is missing" — that's an implementation detail.

### Numbering

`IP-01`, `IP-02`, ... Sequential, no semantic grouping in the number. Group via section headers in `interaction-paths.md`.

## Groupings (section headers in `interaction-paths.md`)

1. **Onboarding** — first-time interactions. Brand-new project, fresh agent on existing project, human starting Clew for the first time.
2. **Steady-state work** — the common loop. Agent picks up, works, hands off.
3. **Triage & curation** — human (or agent) shapes the backlog and path.
4. **Exception handling** — blocked, abandoned, scope drift, ID conflicts, dead increments.
5. **Inspection** — read-only. Looking without acting.
6. **Lifecycle edges** — init, archive, renumber, reopen, cleanup.

A path lives in exactly one group. If it spans groups, it's two paths — split it.

## Cross-cutting observations → `CLI-design-notes.md`

`interaction-paths.md` stays descriptive. CLI-design implications live in a separate file: `CLI-design-notes.md`.

Things that belong there:

- Commands that recur across many paths (load-bearing).
- Common command sequences that suggest aliases or bundled commands.
- Information needs that aren't served by any current command (gap).
- Friction points (paths that feel verbose or awkward).

Capture observations as they surface — even before path bodies are drafted — so nothing gets lost. Synthesis (turning observations into actual CLI design decisions) waits until enough paths are written to confirm patterns.

## Discipline

- **Actor-grounded.** Every path starts from a concrete actor in a concrete situation. No abstract "the system handles X."
- **Trace, don't justify.** Describe what happens, not why it's good.
- **One screen per path.** If a path doesn't fit on one screen, it's two paths.
- **Don't design the CLI here.** If you find yourself writing "and then `clew foo --bar` should..." — stop. That belongs in the CLI design doc that comes after this.
