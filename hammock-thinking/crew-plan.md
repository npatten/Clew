# Clew — Design Plan

> **Status: living design doc.** Active iteration; nothing here is set in stone. If something looks wrong, push back. Companion notes: `hammock-thinking/DDD/CLI-design-notes.md` (interaction-pattern sketches).

## What Clew is

A lightweight, local, git-native project management system for hobby projects and tiny teams (and the agents working on them). The name _clew_ refers to the ball of thread Ariadne gave Theseus to navigate the labyrinth.

Guiding philosophy: [Simple Made Easy (Hickey)](https://www.youtube.com/watch?v=LKtk3HCgTa8) — pursue the goldilocks zone of features; resist complexity that doesn't pay rent.

## Goals

- Open source, works locally, no server, no subscription.
- CLI-first (Rust). TUI/GUI possibly later, without changing the data model.
- Optimized for agents and humans equally — but realistically agents will be the dominant consumer; design tradeoffs favor agents when they conflict.
- Agent harness agnostic — easy to swap harnesses (Claude Code, Codex, Cursor, etc.) within the same project. No coupling to harness-specific conventions.
- Minimize context window / token usage.
  - Push as much work as possible to deterministic software (Clew CLI).
  - Aid agent session:session handoffs with minimal token cost _(addresses the severe anterograde amnesia of LLMs)_.
- Backlog of work, taggable by humans or agents.
- Integrate beautifully with git locally; rely on the project's git remote for cloud sync / backup (no separate sync layer).

## Non-goals

- Selling complex Agile/Scrum methodology. Pragmatic and effective only.
- Maximizing parallel agent sessions or token-maxing. Default assumption: 1–2 primary agents at a time, starting fresh sessions per stable increment.
  - Sub-agents are fine and encouraged — but for token efficiency (e.g., scout agents reading large files), not for parallelizing tracked work.
- Replacing enterprise tooling (Jira) or broader knowledge systems (Monday, ClickUp). Individuals and tiny teams only.

---

## Vocabulary

- **Task** — The most atomic unit of work. A single action or reminder. Lives as a checkbox inside an increment, never as its own file.
- **Increment** — A standalone unit of work containing zero or more tasks. When completed, the goal is for the codebase to be stable, tested, linted, and safely committable. The unit of work an agent typically completes in a session. _[1 session : 1 Increment] is an encouraged pattern, not a requirement._
- **Epic** — A larger body of work consisting of two or more increments that must ship together for the new functionality to work.
- **Relay** — An ephemeral transition of context between agent sessions. Captures what doesn't live anywhere else (discoveries, next-actions, open questions).

---

## Storage model

### Format: Markdown files with YAML frontmatter

All state lives in plain markdown files with YAML frontmatter. Reasoning:

- Git integration is free (human-readable diffs, working merges, blame, history).
- Agents read markdown natively and cheaply — no schema or CLI required to inspect.
- Token-efficient: frontmatter handles structured fields; body holds prose.
- Deterministic CLI tools (`grep`, `rg`, `awk`, plus our own `clew`) work on top.
- Graceful failure mode: if the CLI breaks, data is still readable and editable.

### Directory layout

```
.clew/
├── epics/
│   └── 0007-oauth-overhaul.md
├── increments/
│   ├── fragments.md                  # permanent catch-all, never archived
│   ├── 0042-add-oauth-routes.md
│   └── 0043-token-refresh.md
├── archive/
│   ├── epics/
│   └── increments/
├── relays/
│   └── 0042.md                       # one rolling relay per increment
├── path.md                           # ordered priority list
└── README.md                         # conventions for humans + agents
```

- **Hidden directory** (`.clew/`) — matches `.git/`, `.github/`, etc. Tooling/metadata convention.
- **Separate `epics/` and `increments/` directories** — type encoded in path. Cheapest possible "what's in flight" query (one `ls`, no parsing). Different lifecycles (increments archive frequently, epics rarely). Self-documenting taxonomy.
- **`fragments.md`** — special permanent increment for orphaned tasks (work discovered outside the current increment's scope). Not numbered, never archives. Agent's instruction: don't expand current increment's scope; append to `fragments.md` instead.
- **Archive on done** — completed increments and epics move to `.clew/archive/`. Keeps working set small (token-efficient `ls`); preserves git history via `git mv`. Reopening (`clew reopen`) moves them back.

### Tasks live inside increments

Tasks are GitHub-flavored markdown checkboxes inside the increment file:

```markdown
- [x] Scaffold auth route handlers
- [ ] Implement token refresh
- [ ] [Human] Manually verify OAuth flow in browser
- [ ] Add integration tests
```

- **Two states only**: `[ ]` and `[x]`. No in-progress marker — the increment's own status field already says "someone is working this." Within an increment, the first unchecked box is implicitly "current."
- **`[Human]` annotation** — denotes a manual task the agent cannot complete (e.g., browser-verifying a UI). Agent surfaces it to the human and proceeds; never checks it on the human's behalf.
- **No per-task IDs, timestamps, or status enum.** Granularity trap. The increment is the unit of state; tasks are a checklist.
- **Cross-increment dependencies** are expressed in prose ("blocked on #0039"), not via task-level IDs. Increment-level blocking is the right granularity.

### Frontmatter shape

```yaml
---
id: 0042
status: in_progress # backlog | todo | in_progress | done | abandoned
blocked_reason: "..." # optional; presence = blocked
epic: 0007 # optional
tags: [auth, p0] # optional, free-form, CLI-aware
created_at: 2026-04-20T10:00:00Z
updated_at: 2026-04-25T14:30:00Z
---
```

CLI-managed fields: `id`, `status`, `created_at`, `updated_at`. CLI-aware: `epic`, `blocked_reason`, `tags`. Everything else is preserved-but-ignored — see Extensibility below.

---

## ID scheme

### Hybrid: sequential numeric ID + slug filename

- Filename: `0042-add-oauth-routes.md`
- ID in frontmatter: `id: 0042`
- Canonical reference in prose: `#0042`
- CLI accepts `clew show 42`, `clew show 0042`, `clew show add-oauth-routes`

### Rules

- **Zero-padded 4-digit IDs** (`0042`). Token-cheap, memorable, sortable.
- **Slug is for humans**; ID is for references. Slug can change freely (`git mv` the file, edit frontmatter); references stay valid because they use the ID.
- **Separate counters for epics and increments.** Directory split disambiguates; numbers stay small and meaningful within their type. CLI can output `E7`/`I42` when ambiguity matters in display.
- **`#` prefix for references** (`#0042`). Matches GitHub convention; disambiguates references from numbers in prose.
- **Merge conflicts** (when two agents create the same ID): live with them at this scale. Build `clew renumber 0042 0044` early — atomically renames file, updates frontmatter ID, rewrites references. Cheap in Rust. Don't use UUIDs/hashes; cure worse than disease.

---

## Statuses & transitions

### Status set

```
backlog → todo → in_progress → done
                                ↓
                              abandoned
```

- **`backlog`** — captured but not yet committed. Raw, possibly underspecified.
- **`todo`** — sharpened, ready for an agent to pick up without asking questions.
- **`in_progress`** — actively being worked.
- **`done`** — completed and shipped. Archived.
- **`abandoned`** — explicitly dropped, with reason. Archived but distinguishable from `done`.

Triage is dropped as a status — it's an _activity_ (the act of moving something from `backlog` to `todo`), not a place where items linger.

### Blocked is a flag, not a status

```yaml
status: in_progress
blocked_reason: "waiting on #0039"
```

Status reflects intent ("I want to be working this"); flag reflects reality. Clearing the block is a field deletion, not a status transition.

### Allowed transitions

- `backlog → todo` (via `clew promote`)
- `backlog → in_progress` (skip todo for trivial work)
- `todo → in_progress` (via `clew start`)
- `in_progress → todo` (kicked back; should include a note explaining why)
- `in_progress → done` (via `clew done`; archives the file)
- Any state `→ abandoned` (via `clew abandon "reason"`; archives)
- `done | abandoned → todo` (via `clew reopen`; unarchives)
- **Not allowed**: `backlog → done`. If you didn't ship it, use `abandoned`.

### Timestamps

Just `created_at` and `updated_at` in frontmatter, both CLI-managed. No per-transition timestamps — git history covers that for free if you ever need it.

---

## Extensibility (or: why there's no config file)

YAML frontmatter is _already_ extensible. Users who want `priority: high` or `jira: PROJ-1234` or `assignee: alice` can just add those fields. Files still parse, still work with `cat`/`rg`/`git`.

### Rules

1. **Permissive parser.** The CLI reads frontmatter, acts only on fields it knows, and **preserves unknown fields on write.** This is the single most important behavior.
2. **Documented in `.clew/README.md`**: "You can add any fields you want. Clew preserves them but won't act on them. Use `rg` or `grep` to query."
3. **`tags` is the universal escape hatch.** CLI-aware (`clew list --tag p0`), free-form, covers ~95% of real extensibility needs.

### What we deliberately don't have

- No `priority` field. Order in `path.md` is priority.
- No rank floats, no per-item ordering field.
- No config schema, no custom workflows, no story points.
- Revisit in 12 months only if real demand emerges.

---

## Path: `path.md`

A single hand-curated markdown file expressing priority order across all increments and epics.

### Format

```markdown
# Path

- #0042-add-oauth-routes
- #0044-fix-session-timeout
- #0043-token-refresh-logic
- #0007-oauth-overhaul (epic)
```

- Line order = priority.
- **Full ID+slug form** for human scannability — `path.md` is read every session and is small; readability wins over token-shaving here.
- Permissive parser: extracts `#NNNN` references, ignores everything else (so users can add prose annotations freely).
- Bullet list (no numbering — order is positional, renumbering on edit is annoying).

### Rules

- **Opt-in.** Empty `path.md` is fine for projects with 1–3 todos.
- **Resolution order**: `clew next` returns the top of `path.md` if non-empty; otherwise the oldest `todo` by `created_at`.
- **Epics allowed in path.** When `clew next` hits an epic, it descends to the epic's first ready increment.
- **CLI auto-maintains.** `clew done 0042` removes `#0042` from `path.md`. CLI normalizes entries to current ID+slug form on write (self-healing against scope/slug drift).
- **`clew lint`** flags drift: items in path that don't exist; `todo` items not in path that maybe should be.

---

## Relay format

A relay is the artifact created when one agent session ends and another begins. It captures the **ephemeral context** that doesn't live in the increment file, git history, or the codebase: discoveries, next-actions, open questions, drift from plan.

### One rolling relay per increment

`.clew/relays/{id}.md`. Each session updates the same file (overwrites or edits). Git provides history if needed. Archives with the increment on `clew done`.

Rationale: a fresh agent on #0042 reads exactly one relay — always current. Old relay content is mostly dead weight (it's been integrated into the codebase or the increment file, or superseded).

### When written

Manually at session end via `clew relay {id}`. The CLI:

1. Opens the existing relay (or a template if none).
2. Pre-fills with `git log` since last relay (raw material for "what's done").
3. Pre-fills task checkbox state.
4. Lets the agent fill the rest.
5. Saves and timestamps.

Manual > auto-generated: the relay's value is in _judgment_ about what's worth carrying forward.

### Format

```markdown
---
increment: 0042
updated_at: 2026-04-26T15:30:00Z
session_count: 3
---

# Relay: #0042-add-oauth-routes

## Status

One- or two-sentence skim. Where things stand.

## Just finished

- Bullets summarizing what got done this session.
- Reference commits by short hash (a3f2..b1d4).

## Next action

The most concrete possible description of the immediate next move.
Includes file paths and approach. Not a task list — _the_ next thing.

## Context worth carrying

- Highest-value section. Things that took time to learn.
- Discoveries, gotchas, decisions and their reasoning.
- Code references with file:line where useful.

## Open questions

- [Human?] Things needing human input.
- [Decide] Things the next agent should pick a direction on.

## Drift from plan

- New tasks discovered outside the original increment scope.
- Original tasks revealed as unnecessary.
- Triggers explicit updates to the increment file.
```

### Discipline

- **Empty sections are omitted entirely.** Don't write "N/A." Token efficiency through silence.
- **Prose where prose serves better, bullets where they don't.** "Next action" reads as one short paragraph, not a list.
- **No restating the increment.** Tasks, acceptance criteria, scope live in the increment file; the relay points back.
- **The relay summarizes; git logs detail.** Don't re-litigate every action.

### Path/relay relationship

- `path.md` is **cross-increment priority**: "across all the work, what's next?"
- Relay is **intra-increment context**: "within this increment, where are we?"

A typical session:

1. Read `path.md` → find next increment.
2. Read the increment file → plan, tasks, criteria.
3. Read the relay → current context, next action.
4. Work.
5. `clew relay {id}` at session end.
6. If increment complete: `clew done {id}` (also auto-removes from path).
7. If priority shifts: edit `path.md` for next session.

---

## CLI sketch (pending Q7)

Tentative command shapes mentioned so far:

- `clew new <epic|increment>` — creates in `backlog` (or `todo` with `--ready`).
- `clew show <id>` — accepts numeric ID or slug.
- `clew list [--tag X] [--status Y]` — filtered listing.
- `clew promote <id>` — backlog → todo.
- `clew start <id>` — → in_progress.
- `clew block <id> "reason"` / `clew unblock <id>` — toggle blocked flag.
- `clew done <id>` — → done, archive, remove from path.
- `clew abandon <id> "reason"` — → abandoned, archive.
- `clew reopen <id>` — → todo, unarchive.
- `clew next [--start]` — show (or start) the top of path / oldest todo.
- `clew path` — open `path.md` in `$EDITOR`.
- `clew relay <id>` — open/edit the relay for an increment.
- `clew lint` — flag drift (path/file mismatches, dangling references).
- `clew renumber <old> <new>` — atomic ID renumber with reference rewrites.
- ~~`--json` flag on every read command for agent-friendly output.~~ (realized that json is unnecessary token bloat when providing information to the agent; decided output will just be the direct yaml frontmatter + markdown of the item (Increment or Epic)

---

## Open questions / next decisions

- Full CLI surface: command set, flags, output formats, exit codes.
- Agent's expected workflow loop, codified.
- The `.clew/README.md` template — how to teach the conventions in one short file.
- `clew init` behavior — what does a new project's `.clew/` look like out of the box?
- Git integration specifics: hooks? auto-commits? commit-message conventions linking commits to increments?
- Distribution: cargo install? curl-to-bash? homebrew?
- Whether to ship a relay-history archive or trust git fully.
- Whether/how to support a TUI later without changing the data model.
