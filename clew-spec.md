---
last_major_update: 2026-04-28
---

# Clew — Design Plan

> **Status: living design doc.** Active iteration; nothing here is set in stone. If something looks wrong, push back.

## Revisions

- 2026-04-28 — archive/reopen moves stay plain filesystem moves, not `git mv`; Clew must not mutate the git index, and reviewers can stage with `git add -A` to see renames.
- 2026-04-28 — relay (`relay.md` + `clew relay`) pulled out of the design; in practice increments themselves are carrying cross-session context well enough that the rolling relay file added noise without earning its keep. Design parked under #0015 in case we revive it.
- 2026-04-28 — `clew list` default includes the active working set (`backlog`, `todo`, `in_progress`); `-a`/`--all` adds archived terminal work for history scans.
- 2026-04-28 — `clew new` accepts non-TTY stdin as increment body so agents can create titled, fully-described backlog items in one shell call.
- 2026-04-27 — direct frontmatter edit is first-class; `clew promote` deferred (pure-metadata transitions have no side effects to manage). Self-loop tolerance added for `done`/`abandon`/`reopen` so hand-edit-then-CLI flows complete cleanly.

_Older entries pruned; use `git log hammock-thinking/crew-plan.md` for full history._

## Open questions

- Full CLI surface: flags, output formats — refine as commands are implemented.
  - list design still has open questions around output shape and additional filters.
- Epics / nesting increments. `[[backlinks?]]` formatting? see: [[notes-on-epics]]
- Agent's expected workflow loop, codified into `.clew/README.md`.
- The `.clew/README.md` template content — including the "copy this into your AGENTS.md" harness-integration section. Stub for scaffolding; iterate post-MVP.
- Distribution: `cargo install`? curl-to-bash? homebrew? (Default to `cargo install` for v1; revisit later.)

## What is Clew

Clew is a fast, lightweight, local, git-native project management system for hobby projects and tiny teams (and the agents working on them). The name _clew_ refers to the ball of thread Ariadne gave Theseus to navigate the labyrinth.

Guiding philosophy: [Simple Made Easy (Hickey)](https://www.youtube.com/watch?v=LKtk3HCgTa8) — pursue the goldilocks zone of capabilities; resist complexity that doesn't pay rent.

## Goals

- Works locally, no server, no subscription.
- Pragmatic, simple, effective
- CLI-first (Rust). TUI/GUI possibly later, without changing the data model.
- Optimized for agents and humans equally — but realistically agents will be the dominant consumer; design tradeoffs favor agents when they conflict.
- Agent harness agnostic — easy to swap harnesses (Claude Code, Codex, Cursor, etc.) within the same project. No coupling to harness-specific conventions.
- Minimize context window / token usage.
  - Push as much work as possible to deterministic software (Clew CLI).
  - Aid agent session:session handoffs with minimal token cost _(addresses the severe anterograde amnesia of LLMs)_.
- Backlog of work, taggable by humans or agents.
- Integrate beautifully with git locally; rely on the project's git remote for cloud sync / backup (no separate sync layer).

## Non-goals

- pushing complex consultant project management frameworks.
- Maximizing parallel agent sessions or token-maxing. Default assumption: 1–2 primary agents at a time, starting fresh sessions per stable increment. (keep token spend down, and probability of increment success up)
- Replacing enterprise tooling (Jira) or broader knowledge systems (Monday, ClickUp).

---

## Vocabulary

- **Task** — The most atomic unit of work. A single action or reminder. Lives as a checkbox inside an increment, doesn't merit it's own file.
- **Increment** — A standalone unit of work containing zero or more tasks. When completed, the goal is for the codebase to be stable, tested, linted, and safely committable. The unit of work an agent typically completes in a session. _[1 session : 1 Increment] is an encouraged pattern, not a requirement._ An increment may have a parent increment; a parent with multiple children forms an epic (a larger body of work that must ship together).
- **Path** — The hand-curated priority order across in-flight increments, expressed in `.clew/path.md`. Line order = priority.
- **Archive** — `.clew/archive/`, the resting place for `done` or `abandoned` increments. Files are moved there on archive and back on reopen. Git recognizes those moves as renames when staged or diffed with rename detection.

---

## Storage model

### Format: Markdown files with YAML frontmatter

All state lives in plain markdown files with YAML frontmatter. Reasoning:

- Git integration is free (human-readable diffs, working merges, blame, history).
- Agents read markdown natively and cheaply — no schema or CLI required to inspect.
- Token-efficient: frontmatter handles structured fields; body holds prose.
- Deterministic CLI tools (`grep`, `rg`, `awk`, plus our own `clew`) work on top.
- Graceful failure mode: data is always readable and editable.

### Directory layout

```
.clew/
├── increments/
│   ├── 0042-add-oauth-routes.md
│   ├── 0043-token-refresh.md
│   └── 0007-oauth-overhaul.md      
├── archive/
│   └── 0001-old-work.md             # completed or abandoned increments
├── path.md                          # ordered priority list
└── README.md                        # conventions for humans + agents
```

- **Hidden directory** (`.clew/`) — matches `.git/`, `.github/`, etc. Tooling/metadata convention. **Commit it.** `.clew/` is the project's shared state; treat it like source. Don't `.gitignore` it.
- **Single `increments/` directory** — all items are increments. 
- WIP: Parent-child relationships are still being designed. An increment with children is semantically an epic (a larger body of work that must ship together), but it's stored and treated like any other increment.
- **Archive on done** — completed or abandoned increments move to `.clew/archive/`. Keeps working set small; preserves git history via normal rename detection once staged. Reopening (`clew reopen`) moves them back.
- **User-level config lives elsewhere.** Editor preferences and other per-user settings live at `~/.config/clew/config.toml` (platform-correct path via the `directories` crate), NOT in `.clew/`. No project level config currently supported.

### Tasks live inside increments

Tasks are GitHub-flavored markdown checkboxes inside the increment file:

```markdown
- [x] Scaffold auth route handlers
- [ ] Implement token refresh
- [ ] [Human] Manually verify OAuth flow in browser
- [ ] Add integration tests
```

- **Two states only**: `[ ]` and `[x]`. No in-progress marker — the increment's own status field already says "someone is working this." Within an increment, the first unchecked box is implicitly "current."
- **`[Human]` annotation** — denotes a manual task the agent cannot complete (e.g., browser-verifying a UI). Agent surfaces it to the human and proceeds; only checks it at explicit instruction by human user.
- **No per-task IDs, timestamps, or status enum.** Granularity trap. The increment is the unit of state; tasks are a checklist.
- **Cross-increment dependencies** are expressed in prose ("blocked on #0039"), not via task-level IDs. Increment-level blocking seems to be the point of utility since agents typically complete an increment in one go without issue.

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
parent: 7 # optional; this increment is a child of increment 0007 - WIP design
tags: [auth, p0] # optional, free-form, CLI-aware
created_at: 2026-04-20T10:00:00Z
updated_at: 2026-04-25T14:30:00Z
---
```

CLI-managed fields: `id`, `status`, `created_at`, `updated_at`. CLI-aware: `parent`, `blocked_reason`, `abandoned_reason`, `tags`. Everything else is preserved-but-ignored — see Extensibility below.
(note: tags are searchable via CLI)

**`id` and `parent` in frontmatter are plain integers**, not zero-padded strings. Zero-padding is a presentation rule for **filenames** (`0042-add-oauth-routes.md`) and **prose references** (`#0042`); the YAML scalar is just an integer. (YAML 1.2 parses `0042` as a string anyway, which would break `u32` deserialization.) The CLI renders `#NNNN` form on output regardless.

**Why `abandoned_reason` is persisted in frontmatter:** when an agent later searches `archive/` to see if a feature was ever attempted, the "why we stopped" context must be permanently attached to the file. Otherwise agents might hallucinate that they should retry the dead end. Parallel to `blocked_reason`, but written once by `clew abandon` and not cleared (the file is archived; the reason is part of the historical record).

---

## ID scheme

### Hybrid: sequential numeric ID + slug filename

- Filename: `0042-add-oauth-routes.md`
- ID in frontmatter: `id: 42`
- Canonical reference in prose: `#0042`
- CLI accepts `clew show 42`, `clew show 0042`, `clew show add-oauth-routes`

### Rules

- **Zero-padded 4-digit IDs** (`0042`). Token-cheap, memorable, sortable.
- **Slug is for humans**; ID is for references. Slug can change freely (`git mv` the file, edit frontmatter); references stay valid because they use the ID.
- **Single counter for all increments.** No distinction by type (parent vs. child). Numbers stay small and meaningful.
- **`#` prefix for references** (`#0042`). Matches GitHub convention; disambiguates references from numbers in prose.
- **Merge conflicts** (when two agents create the same ID): live with them at this scale.
  - `clew renumber` is the affordance — atomically renames file, updates frontmatter ID, rewrites references.

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
            ↑                   ↓
            └── reopen ─── abandoned
```

- **`backlog`** — captured but not yet committed. Raw, possibly underspecified.
- **`todo`** — sharpened, ready for an agent to pick up without asking questions.
- **`in_progress`** — actively being worked.
- **`done`** — completed and shipped. Archived.
- **`abandoned`** — explicitly dropped, with reason. Archived but distinguishable from `done`.

### Blocked is a flag, not a status

```yaml
status: in_progress
blocked_reason: "waiting on #0039"
```

Status reflects intent ("I want to be working this"); flag reflects reality. Clearing the block is a field deletion, not a status transition.

### Allowed transitions

- `backlog → todo` (direct file edit)
- `backlog → in_progress` (via `clew start`)
- `todo → in_progress` (via `clew start`)
- `in_progress → todo` (direct file edit)
- `in_progress → done` (via `clew done`; archives the file)
- Any state `→ abandoned` (via `clew abandon "reason"`; archives)
- `done | abandoned → todo` (via `clew reopen`; unarchives)
- **No CLI support**: `backlog → done`. (could always manually edit file)

**Self-loops on terminal-side-effect transitions are tolerated, not rejected.**

- `clew done` on an item already in `status: done` (but unarchived) completes the archive move and emits `warning: #NNNN already marked done; completing archive`.
- Same shape for `abandon` and `reopen`.

_This handles hand-edit-then-CLI workflows without inviting `--force`. Note: `clew start` does not get this tolerance — it has no side effects to complete, so an already-`in_progress` start is a genuine no-op and stays `InvalidTransition` to surface stale assumptions._

### Direct edit is first-class

Operators (humans or agents) can hand-edit `status:` and frontmatter directly. The CLI is convenience over the markdown, not a gate.

- **Pure-metadata changes** (`backlog → todo`, tags, `blocked_reason`, body) need no reconciliation. `updated_at` won't bump per the timestamp rules above; that's the documented tradeoff.
- **Terminal-side-effect transitions** (`done`, `abandoned`, `reopen`) involve file moves and `path.md` updates — prefer the CLI command. If hand-edited first, the corresponding CLI command tolerates the already-flipped state and completes the side effects (see self-loop tolerance above).
- **`clew lint` is advisory.** It surfaces drift (e.g., `status: done` in `increments/`) and names the right command. It does not silently fix; reconciliation goes through the original transition command, used after the fact.

Why this asymmetry: pure metadata flips have no derived state for the CLI to manage, so the CLI gesture isn't faster than the file edit you were already making. Side-effect transitions do real work the operator can't replicate by editing one field. Build CLI commands where they earn their keep through side work or frequency, not for symmetry.

### Timestamps

Just `created_at` and `updated_at` in frontmatter, both CLI-managed. No per-transition timestamps — git history covers that for free if you ever need it.

**Format rules:**

- **RFC 3339 / ISO 8601 with `Z` (UTC) suffix.** Example: `2026-04-26T15:30:00Z`. Sortable as strings, unambiguous, zero timezone confusion.
- **Second precision**, no subseconds. Token waste; second-resolution is hopefully plenty.
- **UTC always.** Never local-tz — local introduces "what time is it for the file" ambiguity across collaborators.
- **`chrono` crate** for parsing/formatting (stripped: `default-features = false, features = ["clock", "serde"]`).

**When `updated_at` bumps:**

- **Any CLI write.** `clew start`, `clew done`, `clew block`, tag edits — all bump it.
- **Manual file edits don't bump it.** If a user opens the increment in an editor and tweaks the body, the CLI doesn't see it; `updated_at` stays put. This is a deliberate tradeoff for simplicity — the CLI is the source of truth for timestamps. If users want to bump it manually, they can re-save via the CLI or accept the mismatch.

---

## Extensibility (or: why there's no config file)

YAML frontmatter is _already_ extensible. Users who want `priority: high` or `some-other-tracker: id-1234` or `assignee: alice` can just add those fields. Files still parse, still work with `cat`/`rg`/`git`.

### Rules

1. **Permissive parser.** The CLI reads frontmatter, acts only on fields it knows, and **preserves unknown fields on write.** This is the single most important behavior.
2. **Documented in `.clew/README.md`**: "You can add any fields you want. Clew preserves them but won't act on them. Use `rg` or `grep` to query."
3. **`tags` is the universal escape hatch.** CLI-aware (`clew list --tag p0`), free-form, covers ~95% of real extensibility needs.

### What we deliberately don't have

- No `priority` field. Order in `path.md` is priority (effective 'rank' sort).
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
- **Increments only** — path lists individual increments
- Permissive parser: extracts `#NNNN` references, ignores everything else (so users can add prose annotations freely).
- Bullet list (no numbering — order is positional, renumbering on edit is annoying).

### Rules

- **Opt-in.** Empty `path.md` is fine for projects with 1–3 todos.
- **Resolution order**: `clew next` returns the top of `path.md` if non-empty; otherwise the oldest `todo` by `created_at`. Always returns a single increment (parent increments still WIP).
- **CLI auto-maintains.**
  - `clew done 0042` removes `#0042` from `path.md`.
  - `clew abandon 0042` removes `#0042` from `path.md` (it's no longer in flight).
  - `clew reopen 0042` appends `#0042` to the **end** of `path.md` (back in flight, lowest priority — the operator can hand-edit to reprioritize).
  - CLI normalizes entries to current ID+slug form on write (self-healing against scope/slug drift).
- **`clew lint`** flags drift: items in path that don't exist; `todo` items not in path that maybe should be.
- **Branch hygiene.** `path.md` is intended as `main`-line state, not branch-local scratch. Reordering it on a feature branch invites needless merge churn — keep priority edits on the trunk where possible.

---

## Session loop

A typical agent session:

1. `clew next` → get raw markdown of next priority task.
2. Work.
3. If increment complete: `clew done {id}` _(auto-removes from path)._
4. If priority shifts: edit `path.md` for next session.
5. Commit; _recommendation is to check in the updated clew files with the completed work._

Cross-session context lives inside the active increment file (decisions, gotchas, discoveries). A separate session-handoff artifact (`relay.md` + `clew relay`) was prototyped and pulled — see #0015 for the parked design if we ever revive it.

---

## CLI sketch

- `clew init` — scaffold `.clew/` in the current directory: creates `increments/`, `archive/`, empty `path.md`, and a templated `README.md`. Also creates `#0000-bootstrap-clew` as a real setup task (instructs the user to copy the harness-integration section from `.clew/README.md` into their `AGENTS.md` / `CLAUDE.md`, then run `clew done 0000`). The bootstrap takes `#0000` so the user's first real increment is `#0001`.
- `clew new "<title>"` — creates in `backlog` (or `todo` with `--ready`). Optional `--parent <id>` flag to link to a parent increment. If stdin is non-TTY, reads it verbatim as the increment body; stdin is body-only, and leading frontmatter delimiters are rejected.
- `clew show <id>` — accepts numeric ID or slug. Default output: raw markdown (frontmatter + body) to stdout. In an interactive TTY, opens the file in the configured editor instead. `--json` optional for structured output.
- `clew list [--tag X] [--status Y] [-a|--all]` — filtered listing.
  - **Default:** `backlog + todo + in_progress` (non-archived, non-terminal working set).
  - `-a` / `--all` — include all statuses, including archived `done` / `abandoned` increments.
  - `--status X` — explicit single-status filter, overrides defaults.
- `clew promote <id>` — _deferred._ Direct frontmatter edit (`status: backlog` → `status: todo`) suffices; the transition has no side effects. Revisit if MVP self-hosting reveals friction.
- `clew start <id>` — → in_progress.
- `clew block <id> "reason"` / `clew unblock <id>` — toggle blocked flag.
- `clew done <id>` — → done, archive, remove from path.
- `clew abandon <id> "reason"` — → abandoned, archive.
- `clew reopen <id>` — → todo, unarchive.
- `clew next [--start]` — show (or start) the top of path / oldest todo.
- `clew path` — open `path.md` in the user's configured editor.
- `clew lint` — flag drift (path/file mismatches, dangling references).
- `clew renumber <old> <new>` — atomic ID renumber. Renames the file, rewrites the `id:` (and any `parent:`) field in frontmatter, scans **other increment files** (`increments/` and `archive/`) for `#NNNN` references in body or frontmatter and updates them, and updates `path.md`. Does **not** rewrite git history, commit messages, or external code/docs — those are immutable or out of scope.
- `--json` optional flag: unclear if this will help agent comprehension or just waste tokens, will have to test later

---

## Implementation

> Stack (`Cargo.toml`), project layout (`src/`), and the `Increment` struct shape are settled and live in code. Module organization conventions live in `AGENTS.md`. This section keeps only behavioral specs that shape future commands.

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

## Git integration

### No auto-commits, ever

Clew never invokes `git commit` on the user's behalf. Not on `clew done`, not on `clew abandon`, not on any other state transition. Reasons:

- Agents need to chunk their work logically (one feature, one commit). Auto-commits create messy histories and strip the agent's ability to write a thoughtful summary.
- A failing test or lint between Clew state changes shouldn't be hidden inside an auto-commit; the agent should see it and decide.
- Clew owns _project state_, not _git workflow_. Mixing the two creates surprise and reduces the user's trust.

The agent or human is always the one running `git commit`. Clew's CLI output makes this easy: `clew done` mutates files and leaves the changes unstaged for the user's normal commit flow.

### No index mutation

Clew does not run `git add`, `git mv`, `git reset`, or any command that mutates the git index. Archive transitions use filesystem moves (`rename`):

- `clew done` / `clew abandon`: `.clew/increments/NNNN-slug.md` → `.clew/archive/NNNN-slug.md`
- `clew reopen`: `.clew/archive/NNNN-slug.md` → `.clew/increments/NNNN-slug.md`

The cost: immediately after the command, `git status --short` may show a deleted file plus an untracked directory/file:

```text
 D .clew/increments/0001-example.md
?? .clew/archive/
```

The benefit: Clew never surprises users or agents by staging unrelated work or changing what is already staged. For review, run `git add -A` before inspecting the staged diff; git then reports the archive move as a rename when similarity is high enough:

```text
R  .clew/increments/0001-example.md -> .clew/archive/0001-example.md
```

If the user wants to review without staging, use `git diff --find-renames` / `git diff -M` plus normal `git status` awareness. This keeps Clew's responsibility to project state separate from the operator's responsibility for staging and commits.

### Commit message convention (recommended, not enforced)

Increments are referenced by `#NNNN` per the ID scheme. Commit messages should prefix the increment ID where applicable:

```
[#0042] add OAuth route handlers
[#0042] fix token refresh edge case
```

This is documented in `.clew/README.md`'s "copy this into your AGENTS.md" section, not enforced by Clew itself. Convention earns adoption; enforcement creates friction.

### Hooks: deferred

A `prepare-commit-msg` hook that auto-prefixes `[#NNNN]` based on the active increment is **explicitly deferred** for v1. Two real concerns:

- **Staleness risk:** the hook would need to know the active increment, but the agent's actual focus can lag what Clew has recorded (e.g., agent ran `clew start 0050` but the active state hasn't caught up — hook prepends the prior `#0042` and creates a wrong-tag bug that's hard to debug).
- **Platform fragility:** `.git/hooks/` shell scripts are OS-dependent; `clew init` shipping a hook becomes a per-platform support burden.

If, after real-world use, agents consistently fail to prefix commits in practice, revisit. A `clew commit "msg"` wrapper (cross-platform, explicit, no hook) would be cleaner than `.git/hooks/` if we ever need automation.

---
