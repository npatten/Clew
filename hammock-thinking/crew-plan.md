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
- **Increment** — A standalone unit of work containing zero or more tasks. When completed, the goal is for the codebase to be stable, tested, linted, and safely committable. The unit of work an agent typically completes in a session. _[1 session : 1 Increment] is an encouraged pattern, not a requirement._ An increment may have a parent increment; a parent with multiple children forms an epic (a larger body of work that must ship together).
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
├── increments/
│   ├── 0042-add-oauth-routes.md
│   ├── 0043-token-refresh.md
│   └── 0007-oauth-overhaul.md       # parent increment (has children via their `parent:` field)
├── archive/
│   └── 0001-old-work.md             # completed or abandoned increments
├── relay.md                         # single rolling session-handoff file
├── path.md                          # ordered priority list
└── README.md                        # conventions for humans + agents
```

- **Hidden directory** (`.clew/`) — matches `.git/`, `.github/`, etc. Tooling/metadata convention.
- **Single `increments/` directory** — all items are increments. Parent-child relationships are expressed via the `parent:` field in frontmatter. An increment with children is semantically an epic (a larger body of work that must ship together), but it's stored and treated like any other increment.
- **Archive on done** — completed or abandoned increments move to `.clew/archive/`. Keeps working set small; preserves git history via `git mv`. Reopening (`clew reopen`) moves them back.
- **Single rolling `relay.md`** — one session-handoff file at the top level, overwritten each session. Git provides history. Not archived with increments.
- **User-level config lives elsewhere.** Editor preferences and other per-user settings live at `~/.config/clew/config.toml` (platform-correct path via the `directories` crate), NOT in `.clew/`. The "no project-level config file" rule stays pure: `.clew/` holds project state only.

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

### Task parser tolerance

Postel's Law: liberal in what we accept, strict in what we produce. LLMs hallucinate minor whitespace variations; the parser shouldn't break when an agent gets it slightly wrong.

**Accept on read** (all valid GFM, plus reasonable whitespace tolerance):

- `- [ ]`, `- [x]`, `- [X]` (uppercase X)
- `* [ ]`, `* [x]` (asterisk bullet)
- Extra spaces between bullet and `[`: `-   [ ]`
- Tabs in place of spaces

**Reject on read** (invents non-GFM dialect; would break markdown renderers):

- `-[ ]` (no space between bullet and bracket — not a valid GFM checkbox in any spec; tools like GitHub, VSCode preview, etc. won't render it as a checkbox)

**Write canonically:** the CLI always emits `- [ ]` / `- [x]` (hyphen bullet, single space, lowercase `x`). On any CLI write that touches a task list, normalize to canonical form. This keeps files compatible with every other markdown tool and prevents drift from agent-introduced variations accumulating over time.

### Frontmatter shape

```yaml
---
id: 42
status: in_progress # backlog | todo | in_progress | done | abandoned
blocked_reason: "waiting on #0039" # optional; presence = blocked. Quote any value containing `#` (YAML treats bare `#` as comment)
abandoned_reason: "..." # optional; written by `clew abandon`; preserved through archive/reopen
parent: 7 # optional; this increment is a child of increment 0007
tags: [auth, p0] # optional, free-form, CLI-aware
created_at: 2026-04-20T10:00:00Z
updated_at: 2026-04-25T14:30:00Z
---
```

CLI-managed fields: `id`, `status`, `created_at`, `updated_at`. CLI-aware: `parent`, `blocked_reason`, `abandoned_reason`, `tags`. Everything else is preserved-but-ignored — see Extensibility below.

**`id` and `parent` in frontmatter are plain integers**, not zero-padded strings. Zero-padding is a presentation rule for **filenames** (`0042-add-oauth-routes.md`) and **prose references** (`#0042`); the YAML scalar is just an integer. (YAML 1.2 parses `0042` as a string anyway, which would break `u32` deserialization.) The CLI renders `#NNNN` form on output regardless.

**Why `abandoned_reason` is persisted in frontmatter:** when an agent later searches `archive/` to see if a feature was ever attempted, the "why we stopped" context must be permanently attached to the file. Otherwise agents hallucinate that they should retry the dead end. Parallel to `blocked_reason`, but written once by `clew abandon` and not cleared (the file is archived; the reason is part of the historical record).

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
- **Single counter for all increments.** No distinction by type (parent vs. child). Numbers stay small and meaningful.
- **`#` prefix for references** (`#0042`). Matches GitHub convention; disambiguates references from numbers in prose.
- **Merge conflicts** (when two agents create the same ID): live with them at this scale. Build `clew renumber 0042 0044` early — atomically renames file, updates frontmatter ID, rewrites references. Cheap in Rust. Don't use UUIDs/hashes; cure worse than disease.

### Allocation: scan-and-increment

`clew new` allocates IDs by scanning the filesystem rather than maintaining a counter file:

1. List filenames in `increments/` and `archive/`.
2. Parse the leading 4-digit ID from each.
3. Pick `max(all_ids) + 1`.

Both directories must be scanned — archived IDs can be higher than active ones (e.g., create `#0001` and `#0002`, archive `#0002`, max-active is now `1` but max-archive is `2`; next ID must be `3`). The "single counter, no reuse" rule depends on this.

No counter file: it adds maintenance without adding safety (it would conflict on merge just like file IDs do), and scanning is microseconds even at thousands of increments.

### Slug rules

Slugs are **part of the identifier contract** — `clew show <slug>` is a supported lookup form, so slugs must be unique. Generation rules:

- Use the `slug` crate (which wraps `deunicode` for ASCII-folding).
- Apply a 50-character truncation on top of the crate's output.
- If transliteration yields an empty string (e.g., all-CJK input), fall back to `untitled`. The user can `git mv` to rename if they care.

**Collision handling: error at creation, no auto-suffixing.**

```
$ clew new "Add OAuth"
error: slug 'add-oauth' is already used by #0042-add-oauth.md
       try a more specific title (e.g., "Add OAuth for Google")
```

- The collision check spans `increments/` AND `archive/` — once a slug is used, it's reserved forever (otherwise `clew reopen` could collide).
- No `-2`, `-3` magic suffixes — files don't sprout numbers the user didn't choose.
- Forces better titles. Better titles produce better slugs, which is good for both humans and agents.

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

**Format rules:**

- **RFC 3339 / ISO 8601 with `Z` (UTC) suffix.** Example: `2026-04-26T15:30:00Z`. Sortable as strings, unambiguous, zero timezone confusion.
- **Second precision**, no subseconds. Token waste; second-resolution is plenty for human-scale work.
- **UTC always.** Never local-tz — local introduces "what time is it for the file" ambiguity across collaborators.
- **`chrono` crate** for parsing/formatting (stripped: `default-features = false, features = ["clock", "serde"]`).

**When `updated_at` bumps:**

- **Any CLI write.** `clew start`, `clew done`, `clew block`, tag edits — all bump it.
- **Manual file edits don't bump it.** If a user opens the increment in an editor and tweaks the body, the CLI doesn't see it; `updated_at` stays put. This is a deliberate tradeoff for simplicity — the CLI is the source of truth for timestamps. If users want to bump it manually, they can re-save via the CLI or accept the mismatch.

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

A single hand-curated markdown file expressing priority order across all in-flight increments.

### Format

```markdown
# Path

- #0042-add-oauth-routes
- #0044-fix-session-timeout
- #0043-token-refresh-logic
```

- Line order = priority.
- **Full ID+slug form** for human scannability — `path.md` is read every session and is small; readability wins over token-shaving here.
- **Increments only** — path lists individual increments, not parent increments. Parent-child relationships are expressed in frontmatter (`parent:`), not in path.
- Permissive parser: extracts `#NNNN` references, ignores everything else (so users can add prose annotations freely).
- Bullet list (no numbering — order is positional, renumbering on edit is annoying).

### Rules

- **Opt-in.** Empty `path.md` is fine for projects with 1–3 todos.
- **Resolution order**: `clew next` returns the top of `path.md` if non-empty; otherwise the oldest `todo` by `created_at`. Always returns a single increment (never a parent increment).
- **CLI auto-maintains.** `clew done 0042` removes `#0042` from `path.md`. CLI normalizes entries to current ID+slug form on write (self-healing against scope/slug drift).
- **`clew lint`** flags drift: items in path that don't exist; `todo` items not in path that maybe should be.

---

## Relay format

A relay is the artifact created when one agent session ends and another begins. It captures the **ephemeral context** that doesn't live in the increment file, git history, or the codebase: discoveries, next-actions, open questions, drift from plan.

### Single rolling relay

`.clew/relay.md` — one file at the top level, overwritten each session. Git provides history if needed. **Not archived** with increments — the relay is "current focus" state, not increment-bound.

Rationale: the design assumes 1–2 primary agents at a time, working a single in-progress increment per session. A per-increment relay model solves a problem that doesn't exist at this scale. If concurrent in-flight work ever becomes real, per-increment relays can be added later (small migration, additive change).

### When written

Manually at session end via `clew relay`. The CLI:

1. Opens the existing relay (or a template if none).
2. Pre-fills with `git log` since the last relay (raw material for "what's done").
3. Pre-fills task checkbox state for the current focus increment.
4. Lets the agent fill the rest.
5. Saves and timestamps.

Manual > auto-generated: the relay's value is in _judgment_ about what's worth carrying forward.

`clew done` does **not** touch `relay.md`. The next session overwrites it naturally. (Auto-clearing or auto-archiving on `done` is purely additive and can be added later if it earns its keep.)

### Format

```markdown
---
increment: 0042 # which increment this session focused on (omit for non-increment work)
updated_at: 2026-04-26T15:30:00Z
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

- `path.md` is **cross-project priority**: "across all the work, what's next?"
- `relay.md` is **session-handoff context**: "where am I right now, and what's the next move?" (effectively tailored )

A typical session:

1. Read `path.md` → find next increment.
2. Read the increment file → plan, tasks, criteria.
3. Read `relay.md` → current focus, next action, context.
4. Work.
5. `clew relay` at session end.
6. If increment complete: `clew done {id}` (also auto-removes from path).
7. If priority shifts: edit `path.md` for next session.

---

## CLI sketch

- `clew init` — scaffold `.clew/` in the current directory: creates `increments/`, `archive/`, empty `path.md`, `relay.md`, and a templated `README.md`. Also creates `#0001-bootstrap-clew` as a real setup task (instructs the user to copy the harness-integration section from `.clew/README.md` into their `AGENTS.md` / `CLAUDE.md`, then run `clew done 0001`).
- `clew new "<title>"` — creates in `backlog` (or `todo` with `--ready`). Optional `--parent <id>` flag to link to a parent increment.
- `clew show <id>` — accepts numeric ID or slug.
- `clew list [--tag X] [--status Y] [--all]` — filtered listing. Default: in-flight items only. `--all` includes archived.
- `clew promote <id>` — backlog → todo.
- `clew start <id>` — → in_progress.
- `clew block <id> "reason"` / `clew unblock <id>` — toggle blocked flag.
- `clew done <id>` — → done, archive, remove from path. Does NOT touch `relay.md`.
- `clew abandon <id> "reason"` — → abandoned, archive.
- `clew reopen <id>` — → todo, unarchive.
- `clew next [--start]` — show (or start) the top of path / oldest todo.
- `clew path` — open `path.md` in the user's configured editor.
- `clew relay` — open/edit `.clew/relay.md` (no ID arg — single rolling file).
- `clew lint` — flag drift (path/file mismatches, dangling references).
- `clew renumber <old> <new>` — atomic ID renumber with reference rewrites.
- `--json` optional flag: unclear if this will help agent comprehension or just waste tokens, will have to test later

---

## Implementation

### Stack

| Concern | Crate | Notes |
|---|---|---|
| CLI parsing | `clap` | `default-features = false, features = ["derive", "std", "help"]` — POSIX-correct, agent-friendly, stripped of color/suggest bloat |
| YAML | `yaml_serde` (official YAML org fork of archived `serde_yaml`) + `serde` | Manual `---` splitter for frontmatter; pipe the YAML chunk to `yaml_serde` |
| Errors (lib) | `thiserror` | Typed errors in `lib.rs`, `core/`, `storage/`, `commands/` |
| Errors (bin) | `anyhow` | Loose chaining at the `main.rs` boundary |
| Datetime | `chrono` | `default-features = false, features = ["clock", "serde"]` — RFC 3339 UTC with second precision |
| Slug | `slug` (wraps `deunicode`) | Plus 50-char truncation on top |
| User config paths | `directories` | Cross-platform (`~/.config/clew/` on Linux, `~/Library/Application Support/clew/` on macOS, `%APPDATA%\clew\` on Windows) |
| TTY detection | `std::io::IsTerminal` | Stdlib; no extra dep |
| Test: CLI invocation | `assert_cmd` | Runs the compiled binary; stdout/stderr separately assertable |
| Test: filesystem isolation | `assert_fs` | Self-destructing tempdirs per test |
| Test: snapshot | `insta` | Snapshots of generated markdown+frontmatter (catches format drift; this output IS our agent-facing API) |
| Test: parameterized | `rstest` | Many similar inputs to one parser/validator |
| Test: predicates | `predicates` | Composable assertions for `assert_cmd` / `assert_fs` |

### Project layout

```
clew/
├── Cargo.toml
├── src/
│   ├── main.rs              # Thin: calls clew::run()
│   ├── lib.rs               # Public API, re-exports
│   ├── cli.rs               # clap derive structs
│   ├── error.rs             # thiserror types
│   ├── core.rs              # module declaration
│   ├── core/
│   │   ├── increment.rs     # Increment, Status enum
│   │   ├── frontmatter.rs   # split + serialize, preserves unknown fields
│   │   └── path.rs          # path.md parser/writer
│   ├── storage.rs           # module declaration
│   ├── storage/
│   │   └── fs.rs            # filesystem ops (read/write/move increments)
│   ├── commands.rs          # module declaration
│   ├── commands/
│   │   ├── new.rs
│   │   ├── show.rs
│   │   ├── list.rs
│   │   ├── start.rs
│   │   ├── done.rs
│   │   └── next.rs
│   └── templates/
│       └── init_readme.md   # `include_str!`'d template for `clew init`
└── tests/
    └── integration_test.rs  # assert_cmd + assert_fs end-to-end
```

- **Modern module style** — `core.rs` + `core/` directory, not `core/mod.rs`. Cleaner editor tabs.
- **`lib.rs` + `main.rs` split** — integration tests can import the library cheaply; `main.rs` stays thin.
- **`core/` is pure logic** (no I/O); **`storage/` is the I/O seam**; **`commands/` orchestrates**. Easy to unit-test the pure parts.
- **Templates as `include_str!` markdown files** — easier to iterate on than escaped Rust string literals.

### Frontmatter struct shape

Strongly typed for known fields, with a flatten catch-all that **preserves unknown fields on round-trip** (the single most important parser behavior, per the Extensibility rules):

```rust
struct Increment {
    id: u32,
    status: Status,
    parent: Option<u32>,
    blocked_reason: Option<String>,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(flatten)]
    extra: HashMap<String, yaml_serde::Value>,
    // body kept separately, not in the struct
}
```

### Error model

Layered:
- **Library** (`thiserror`): typed variants like `NotFound(u32)`, `InvalidTransition { from, to }`, `SlugCollision(String)`, `Frontmatter(...)`, `Io(...)`. Integration tests can match on variants.
- **Binary** (`anyhow`): `?` propagation in `main.rs` with `.context(...)` for user-facing display.

**Exit codes:**
- `0` — success
- `1` — user error (not found, invalid transition, slug collision, dangling reference)
- `2` — system error (I/O failure, frontmatter parse failure, internal bug)

### Testing strategy

- **Unit tests in-module** (`#[cfg(test)] mod tests`) for `core/` — parser edge cases, status transitions, slug rules, ID parsing. Fast, no I/O. Use `rstest` for parameterized cases.
- **Integration tests in `tests/`** for command wiring. One happy path per command + a few cross-command flows (create → start → done). Use `assert_cmd` + `assert_fs` for isolated tempdirs.
- **Snapshot tests with `insta`** for the generated markdown+frontmatter. The output IS our agent-facing API; format drift is a real bug.
- **stdout = data; stderr = status/errors.** Codified and tested. `clew next | xargs ...` should give clean data; `clew done 0042` may print "Archived #0042" to stderr.
- Coverage target: not chasing a number. The goal is "every status transition, every frontmatter edge case, every command's happy path" — naturally lands ~80%+.

### Editor resolution

Clew is a system-wide install operating on project-scoped state. Editor preference is per-user (different developers, different editors), so it lives in user config — not in `.clew/`.

**Config file:** `~/.config/clew/config.toml` (resolved via the `directories` crate; platform-correct paths on Linux/macOS/Windows).

```toml
# ~/.config/clew/config.toml
editor = "cursor --wait"
```

The `--wait` flag is essential for Electron-based editors (VSCode, Cursor) — without it, the CLI thinks the editor closed instantly and proceeds before the user finished editing. We capture the full command string, not just the binary name.

**Resolution order** (first match wins):

1. `--editor=<cmd>` flag (highest priority)
2. `CLEW_EDITOR` env var (tool-specific override)
3. Global config file (`~/.config/clew/config.toml`)
4. `$VISUAL` env var
5. `$EDITOR` env var
6. **Interactive PATH scan & prompt** (TTY only): scan for known editors (`code`, `cursor`, `nvim`, `vim`, `nano`, `helix`); prompt user to pick; save choice to global config
7. **Typed error** if non-TTY and no source matched

**TTY detection** via `std::io::IsTerminal`. Agents (no TTY) hit step 7 directly, get a typed `NoEditorConfigured` error, and proceed without hanging.

**Editor launch failure** (e.g., the configured editor was uninstalled): typed `EditorLaunchFailed` error with a hint pointing to the config file. No auto-reprompt — masking real problems is worse than asking the user to fix the config once.

**Lazy setup:** `clew init` does NOT prompt for editor. The first interactive command that needs an editor (e.g., `clew path`) triggers the prompt. Keeps `clew init` fast and noninteractive-friendly.

---

## Scaffolding milestone (first build)

The first scaffolding pass delivers **"skeleton + frontmatter parser"**:

- `cargo new` with all deps in `Cargo.toml`
- Module structure exists with stubs (`unimplemented!()` or trivial placeholders)
- One end-to-end smoke test: `clew --version` returns a version string
- **`core/frontmatter.rs` fully implemented with unit tests** (the lynchpin — round-trip preservation of unknown fields, malformed input handling, edge cases)
- `clew init` template files exist as placeholders (`src/templates/init_readme.md` with stub content)
- All commands either don't exist yet or return `unimplemented!()` errors

Why this scope:
- "Skeleton + smoke test" alone leaves the next agent staring at empty modules.
- "Skeleton + parser + first command" bundles too much for a single scaffolding task.
- Frontmatter parser is the riskiest piece; doing it standalone gets it scrutinized as its own artifact.

The next session picks up with `clew show` as a clean vertical slice on the foundation.

---

## Git integration

### No auto-commits, ever

Clew never invokes `git commit` on the user's behalf. Not on `clew done`, not on `clew abandon`, not on any other state transition. Reasons:

- Agents need to chunk their work logically (one feature, one commit). Auto-commits create messy histories and strip the agent's ability to write a thoughtful summary.
- A failing test or lint between Clew state changes shouldn't be hidden inside an auto-commit; the agent should see it and decide.
- Clew owns *project state*, not *git workflow*. Mixing the two creates surprise and reduces the user's trust.

The agent or human is always the one running `git commit`. Clew's CLI output makes this easy: `clew done` mutates the file (`git mv` to archive, etc.) and leaves the changes staged-or-unstaged for the user's normal commit flow.

### Commit message convention (recommended, not enforced)

Increments are referenced by `#NNNN` per the ID scheme. Commit messages should prefix the increment ID where applicable:

```
[#0042] add OAuth route handlers
[#0042] fix token refresh edge case
```

This is documented in `.clew/README.md`'s "copy this into your AGENTS.md" section, not enforced by Clew itself. Convention earns adoption; enforcement creates friction.

### Hooks: deferred

A `prepare-commit-msg` hook that auto-prefixes `[#NNNN]` based on the active increment is **explicitly deferred** for v1. Two real concerns:

- **Staleness risk:** the hook would need to know the active increment, but `relay.md` can lag the agent's actual focus (e.g., agent ran `clew start 0050` but hasn't relay'd yet — hook prepends the prior `#0042` and creates a wrong-tag bug that's hard to debug).
- **Platform fragility:** `.git/hooks/` shell scripts are OS-dependent; `clew init` shipping a hook becomes a per-platform support burden.

If, after real-world use, agents consistently fail to prefix commits in practice, revisit. A `clew commit "msg"` wrapper (cross-platform, explicit, no hook) would be cleaner than `.git/hooks/` if we ever need automation.

---

## Open questions / next decisions

- Full CLI surface: flags, output formats — refine as commands are implemented.
- Agent's expected workflow loop, codified into `.clew/README.md`.
- The `.clew/README.md` template content — including the "copy this into your AGENTS.md" harness-integration section. Stub for scaffolding; iterate post-MVP.
- Distribution: `cargo install`? curl-to-bash? homebrew? (Default to `cargo install` for v1; revisit later.)
- Auto-clearing or auto-archiving `relay.md` on `clew done` — purely additive feature, defer until earned.
- Whether/how to support a TUI later without changing the data model.
