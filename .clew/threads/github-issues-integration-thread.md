---
title: GitHub Issues Integration Thread
status: draft
tags: [integrations, github, issues, prd, exploration]
created_at: 2026-04-28
updated_at: 2026-04-29
---

# GitHub Issues Integration Thread

> Pre-increment exploration. Goal: map the value space of Clew ↔ GitHub Issues integration broadly enough that we can later narrow into one or more concrete increments. Not picking a winner yet.

## 1. Problem & positioning

Clew is local-first and git-native. GitHub Issues is the dominant collaboration surface for OSS and small-team projects. Many plausible Clew users might migrate to GH as their project grows, or want to live in both at once, or even move to Clew from GH.

Working positioning to test:

> Clew is local-first, but not a trap. You can come from GitHub Issues, you can graduate to GitHub Issues, it's your data and your work. Clew is a tool.

This thread explores what that means in practice without committing to a specific implementation.

## 2. User scenarios

Ranked by how confident we are the scenario is real today (for the project author's own usage):

**Strong / likely real:**

- **S2. Clew solo dev gaining contributors.** Project picked up traction, needs GH Issues for collaboration and visibility. _Author can imagine ending up here._
- **S3. Solo dev who wants public visibility without losing local loop.** Stays in Clew day-to-day, mirrors selected increments to GH so external watchers see roadmap. _Author can imagine this; suspects it pulls toward sync._
- **S5. Drive-by external contributor.** Files a GH issue; maintainer wants to pull it into Clew local loop. _Author can imagine this soon._

**Appealing but speculative:**

- **S7. Agent-driven status broadcast.** Agent finishes increment, drops a comment on the linked GH issue so human reviewers know without checking `.clew/`. Lightweight publish-shaped. _Worth keeping on the radar._

**Plausible but unverified:**

- **S1. GH-native solo dev going local.** Existing GH Issues backlog, project went solo or quiet, wants Clew's cheap local loop. _Real demand unknown._
- **S4. Mixed team, agents in Clew, humans in GH.** True bidirectional sync case. _Aspirational; hardest to deliver._
- **S6. Project archaeology.** Someone evaluating an old repo wants closed GH issues alongside archived Clew increments for context. _Real demand unknown._

## 3. Mode space

The integration space splits along one axis: **does the action create or maintain a relationship between two active systems, or is it a one-shot move?**

### Moves

One-shot transfers. Lineage recorded. No ongoing relationship between the two sides after the action.

- **Import (GH → Clew).** Pull GH issues into `.clew/` as increments. Source GH issues are read-only inputs.
- **Export (Clew → GH).** Send Clew increments to GH; the local increment exits Clew's working set into a `migrated/` resting place. (See §4.)

### Relations

Both systems remain active. The design problem is how they relate.

- **Publish.** Ongoing one-way mirror; Clew is source of truth, GH is a published view. Drift is likely, but potentially solved by another publish?
- **Sync.** Ongoing bidirectional reconciliation. Authority is shared. Drift detection and conflict resolution become first-class problems. Merge conflict dragons await?

### Overlap and tension

- Moves are well-bounded; relations have unbounded operational surface.
- Publish has gravity toward sync the moment GH-side comments or edits accumulate — once a user has published, "what about the comments?" becomes a real question.
- Import + export combined cover most of the "no trap" promise. Publish/sync cover the "live in both" promise.

This thread treats moves as the explorable territory now, and relations as named-but-deferred.

## 4. Move: Export (Clew → GH)

### Mental model

Export _is_ migration. The Clew increment is leaving the system. The local file moves to `.clew/migrated/`, gets a banner and frontmatter pointer, and the rest of the Clew CLI ignores it from then on.

### Why transfer, not copy

Copy-only would leave the increment live in both systems, immediately producing the drift problem we're trying to avoid in this thread. Transfer is clear: "this work item now lives over there."

### Disk shape

```
.clew/
├── increments/      # active work; CLI reads/writes
├── archive/         # done | abandoned (terminal in Clew)
├── migrated/        # exited the system; CLI writes once, never reads except for ID/slug uniqueness
└── path.md
```

- `archive/` is preserved for `done` and `abandoned`. Migrated work isn't terminal in the same sense — it's continuing elsewhere — so it gets its own resting place.
- `migrated/` is invisible to most of the CLI (`list`, `show`, `start`, `done`, `reopen`, etc. do not read it).
- The ID allocator and the slug-uniqueness check **do** scan `migrated/`. Microsecond cost; preserves the "`#0042` means one thing forever" guarantee for prose references in commits, PRs, agent transcripts.
- No new status enum value. Original `status` is preserved as a snapshot of where the work was when it left.

### Per-file shape on migration

Frontmatter gains a `migrated_to` block:

```yaml
---
id: 42
status: in_progress # snapshot at migration time
migrated_to:
  system: github
  repo: OWNER/REPO
  issue: 123
  url: https://github.com/OWNER/REPO/issues/123
  migrated_at: 2026-04-28T12:00:00Z
---
```

Body gets a banner above the original content:

```markdown
> Migrated to GitHub issue OWNER/REPO#123 on 2026-04-28.
> See: https://github.com/OWNER/REPO/issues/123

...original body...
```

One-time action; affordable to do nicely.

### Recovery

No `reopen` for migrated increments initially.

- If the GH issue still exists: `clew import github` brings it back as a fresh increment with new ID; lineage points to both the GH issue and the original migrated file.
- If the GH issue is gone: manual `cp` from `migrated/` back to `increments/`, edit frontmatter. Documented, not a CLI affordance.

Migration is effectively one-way at the CLI level, recoverable manually via git history. Easy path covers the common case; the hard path exists for the rare one.

### User values to optimize for

- **Selectivity** — pick what migrates; not everything is ready or appropriate.
- **Confidence before commit** — see the planned changes (which increments, which repo, what the GH issues will look like) before any remote mutation happens.
- **Predictable bilateral effect** — clear understanding of what happens locally (file moves to `migrated/`) _and_ remotely (GH issue created, labels applied) for each item.
- **Honest irreversibility** — migration is intentionally one-way; the CLI shouldn't pretend otherwise.
- **Fits the Clew CLI ergonomic** — positional, no-flag-by-default, stdout = machine-usable, stderr = humans.

### Sparse notes for later

- **Pre-flight as editable markdown checklist.** Generate a plan file with checkboxes; user unchecks what they don't want; then apply. Symmetric idea also applies to import. Candidate UX, not a decision.
- **Granularity of invocation** (single, batch, whole-backlog) and **filtering** (by tag, status) deferred. The values above can be served by several command shapes.
- **Bulk "we're leaving Clew"** behavior is plausible but rare; design for selective use first.

## 5. Move: Import (GH → Clew)

### Mental model

Import is read-only on the GH side by default. GH issues are pulled into Clew as new increments with full lineage frontmatter. Whether the GH issue is then closed, labeled, or commented is a separate, explicit step (see sparse notes).

### Per-file shape on import

Imported increments are normal active increments. They live in `.clew/increments/` like any other. Frontmatter records provenance:

```yaml
---
id: 51
status: backlog
github:
  source:
    repo: OWNER/REPO
    issue: 123
    url: https://github.com/OWNER/REPO/issues/123
    imported_at: 2026-04-28T12:00:00Z
---
```

Body gets a footer or top-of-body banner noting the source. (Exact placement TBD; the value is traceability.)

### User values to optimize for

- **Selectivity** — pull in specific issues, not the whole backlog by default.
- **Lineage preservation** — frontmatter records the source GH issue; future agents can trace where an increment came from.
- **Faithful translation** — title, body, task lists, labels-as-tags arrive intact; nothing silently dropped.
- **Honest about what doesn't translate** — comments, assignees, milestones, projects, reactions don't have Clew equivalents. The CLI surfaces what was skipped, not hides it.
- **Decoupled from GH state mutation by default** — import reads GH; doesn't close, comment, or label the source. Mutations are a separate explicit step.
- **Curatable, not dumped** — bulk import shouldn't pollute the working set. The S1 user wants to _curate_ their old GH backlog into a sharper Clew backlog, not faithfully replicate noise.

### Sparse notes for later

- **Pre-flight as editable markdown checklist** (mirrors export's note). Especially valuable for bulk imports; the user unchecks issues they don't want to bring in.
- **Codex's "import as threads first" idea.** Drop GH issues into `.clew/threads/` for review before promoting to increments. Fits the curate-not-dump value well; possibly the best shape for bulk import. Worth prototyping.
- **Optional GH-side post-action** — close as duplicate, label "migrated-to-clew", comment-link back. Feature enrichment for later.
- **Granularity** (single issue, list of issues, label/state filter) deferred.

## 6. Relations: Publish & Sync (deferred)

Both kept open; not designed in this thread. Sketched here so the venn isn't empty.

### Publish (one-way ongoing mirror)

- Clew is source of truth; GH is a published view.
- Appeals to S3 (visibility without losing local loop) and S7 (agent-driven status broadcast).
- Lightest version of S7: agent posts a single GH comment when an increment is marked `done`. Doesn't require full publish machinery; might be a useful spike independent of the larger publish question.
- **Pivot condition for revisiting:** a Clew user reports actually living in S3 — they're publishing manually-curated subsets to GH and finding the staleness painful enough to want CLI help.

### Sync (bidirectional reconciliation)

- Both sides write; both sides try to converge.
- Drift detection and conflict resolution become first-class problems.
- Appeals to S4 (mixed team, agents in Clew, humans in GH).
- **Pivot condition for revisiting:** S4 emerges as a real user need _and_ someone has thought hard about conflict resolution semantics. Don't open this door without a strong forcing function.

### Known design pull

Publish has gravity toward sync. Once GH-side comments and edits accumulate, "what about the GH side?" becomes a real question and the pure one-way model degrades. Worth naming as a design cost of publish, not a separable choice.

## 7. Cross-cutting concerns

### Information preservation

Three principles to apply across all modes:

- **Preserve what information can be preserved.**
- **Give users enough additional information to trace back if needed** (banners, footers, frontmatter lineage).
- **Don't duplicate every convenience feature if core information is re-derivable.** Permission to under-build at the edges; the lineage carries enough that readers can resolve ambiguity themselves.

### Cross-system references in body text

Body content like `#0042` (Clew) or `#123` (GH) is ambiguous once moved across systems. GH will auto-link `#NNNN` in published bodies to its own issues, which can be actively misleading.

No specific approach decided. Direction: lean on the three principles above — keep body prose untouched, let banners/footers carry namespace context, surface risks (e.g., GH auto-link footgun on export) honestly when they bite.

### IDs and identity

- Clew IDs (`#NNNN`) and GH issue numbers will diverge. That's fine.
- Treat GH issue numbers as external metadata in frontmatter; never as Clew IDs.
- Migrated Clew IDs are not reused (ID allocator scans `migrated/`).
- Imported GH issues get fresh Clew IDs; the GH number lives in `github.source.issue`.

### Labels and tags

- Direction unresolved. Spec questions: Clew tag normalization rules on import, Clew-owned label prefix (`clew:*`?) on export, whether to auto-create labels.
- Risk to name: exporting can clutter repos with new labels users didn't intend.
- Defer specifics to per-mode design when we narrow.

### Status mapping

- Lossy in both directions. GH is mostly open/closed plus labels; Clew has `backlog | todo | in_progress | done | abandoned`.
- Specifics deferred. The principle: be conservative, document the mapping, let users override.

### Comments

- Major source of impedance mismatch. Clew has no comment concept.
- Defer; possible future homes include thread files (`.clew/threads/`) or body appendices.

## 8. Dependency: `gh` CLI

**Hard dependency.** Use `gh` for all GitHub interaction; fail fast if missing or unauthenticated.

Rationale:

- Auth is `gh`'s problem, not Clew's.
- Most users already have it installed and authed.
- Keeps Clew's surface small; no token storage, no API versioning, no rate-limit handling in Clew.

Costs to name honestly:

- Users without `gh` can't use the integration, period. No silent fallback.
- `gh`'s own version drift is a coupling point.
- Use `gh ... --json` for stable parsing; human-readable output is unstable.

Pivot conditions for revisiting:

- A real user need to run Clew in environments where `gh` can't be installed (e.g., minimal CI containers).
- A real user need for a non-GitHub system (Linear, Gitea), at which point an internal adapter trait might earn its keep. Not before.

## 9. Open decisions

To resolve when narrowing into design/implementation increments:

- Command shape and naming for import and export (positional vs subcommand; `clew import github` vs `clew github import`, etc.).
- Granularity of invocation (single, list, filter).
- Pre-flight UX (confirm prompt vs editable plan file vs both).
- Comment handling on import.
- Label normalization and creation policy.
- Status mapping table.
- Body-text reference handling specifics (rewriting vs warning vs leaving alone).
- Where repo target lives (per-increment frontmatter, project config, inferred from `gh`).
- Whether import-as-threads is the default for bulk import.
- Whether and how S7 (agent status broadcast) gets a small spike independent of larger publish work.

## 10. What we don't know yet (signals to watch)

- Are S1 and S6 real? Do users actually arrive _from_ GH Issues, or care about archaeology across both systems?
- Does S3 (publish for visibility) materialize, and how quickly does it pull users toward sync?
- Does S7 (agent status broadcast) earn its weight, or is "linked GH issue + the user checks `./clew show`" enough?
- How much do users care about preserving GH issue comments vs. accepting that comments live only on GH?
- For exporters: do users want partial migration (some increments graduate, others stay) or whole-project migration ("we're leaving Clew") more often?

## 11. Candidate next steps

Not implementation increments yet — these are the shapes that would come out of further design work, in rough order of value-to-cost.

1. **Design GitHub issue integration(s)** — the existing backlog stub. Narrow this thread into one or more concrete increments.
2. **Pure mapping core spike** — increment ↔ GH issue title/body/labels/state, no `gh` calls. Reusable across import, export, and any future publish/sync work.
3. **Single-issue import spike** — `clew import github <issue>` for one issue, no GH-side mutation. Lowest risk, unblocks S5.
4. **Single-increment export spike** — `clew export github <id>` for one increment, with `migrated/` mechanics. Unblocks S2.
5. **Pre-flight plan file** — editable markdown plan for bulk operations. Unblocks bulk import (S1) and bulk export.
6. **S7 status-broadcast spike** — minimal "agent comments on linked GH issue when increment is done." Independent of larger publish question.
7. **Documentation pass** — caveats, recovery paths, limits.

---

## Appendix A: `gh` CLI surface

The `gh` CLI supports the issue primitives we'd need:

```
gh issue create
gh issue list
gh issue view
gh issue edit
gh issue close       # --reason completed | "not planned"
gh issue reopen
gh issue comment
gh issue status
gh issue develop     # links branches to issues; not used here yet
```

Repo targeting:

```bash
gh issue <command> -R OWNER/REPO
```

Stable output via:

```bash
gh issue list --json number,title,state,labels,body,assignees,updatedAt
gh issue view 123 --json number,title,body,state,labels,comments
```

Auth check:

```bash
gh auth status
```

## Appendix B: Data mapping reference

Working draft; specifics are open in §9.

| Clew                   | GitHub                                    |
| ---------------------- | ----------------------------------------- |
| increment file         | issue                                     |
| H1 / title             | issue title                               |
| body markdown          | issue body                                |
| tags                   | labels                                    |
| status                 | open/closed plus optional `clew:*` labels |
| `done` (archived)      | closed as completed                       |
| `abandoned` (archived) | closed as not planned                     |
| `blocked_reason`       | body note and/or `clew:blocked` label     |
| `migrated_to.issue`    | external metadata (export side)           |
| `github.source.issue`  | external metadata (import side)           |
| parent                 | issue link / prose only; not designed     |
| path priority          | not mapped; GH Projects out of scope      |

## Appendix C: Frontmatter shapes

Migrated increment (in `.clew/migrated/`):

```yaml
---
id: 42
status: in_progress
migrated_to:
  system: github
  repo: OWNER/REPO
  issue: 123
  url: https://github.com/OWNER/REPO/issues/123
  migrated_at: 2026-04-28T12:00:00Z
---
```

Imported increment (in `.clew/increments/`):

```yaml
---
id: 51
status: backlog
github:
  source:
    repo: OWNER/REPO
    issue: 123
    url: https://github.com/OWNER/REPO/issues/123
    imported_at: 2026-04-28T12:00:00Z
---
```
