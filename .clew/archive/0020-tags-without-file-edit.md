---
id: 20
status: done
created_at: 2026-04-29T02:55:10Z
updated_at: 2026-04-30T03:26:01Z
---
## Question
Should Clew support adding tags when an increment is created?

## Context
Today tags can only be added by editing the increment file after creation. That keeps the creation flow small, but makes tagging feel like a manual follow-up step instead of part of capture.

## Idea
Consider a lightweight way to attach tags during `clew new`, without expanding the control surface too much.

Possible shapes considered:
- Keep creation positional, but prompt for optional tags interactively.
- Support tags in the stdin body/frontmatter if already present.
- Add a documented `--tag` flag for `clew new`.
- Add dedicated `clew tag` / `clew untag` commands for after-the-fact edits.

Current design direction:
- Preserve heredoc/stdin as body-only. This is the main agent capture path and should keep working:

  ```bash
  ./clew new "Verify Clew on WSL and Git Bash" --tag windows --tag distribution <<'EOF'
  ## Goal
  Verify Clew works on WSL and Git Bash.
  EOF
  ```

- Repeated singular `--tag` supports multiple tags. Do not add CSV parsing (`--tags windows,distribution`) unless real usage justifies the permanent parser contract.
- Keep leading stdin frontmatter rejected; frontmatter merge semantics are too ambiguous.
- Add dedicated commands for non-creation edits:

  ```bash
  ./clew tag 0019 windows p0
  ./clew untag 0019 windows
  ```

- Validate tags instead of silently normalizing them. Proposed grammar: `[a-z0-9][a-z0-9-]*`.
  - Reject uppercase, whitespace, commas, and `#` with a hint, e.g. `try: windows`.
  - Rationale: Git generally validates names and preserves valid input rather than silently mutating it. Clew tags should be deterministic and avoid case-sensitive duplicate drift (`Windows` vs `windows`).
- Deduplicate while preserving first-seen order.

## Design concern
Adding tag input to creation may make the MVP loop feel more complicated. The feature should only exist if it improves common capture flow without making `clew new` harder to explain.

## Acceptance notes
- Decide whether tag-on-create belongs in the documented workflow.
- If yes, choose the smallest interface that preserves simple capture.
- Update `clew-spec.md` if the creation workflow changes.

## Implementation handoff notes
An empty-context agent should preserve the current `clew new` heredoc behavior and add tag support around it, not through stdin frontmatter.

Likely implementation shape:
- Update `src/cli.rs` so `clew new` accepts repeated `--tag <tag>` values.
- Update `src/commands/new.rs` so created increments write those tags into frontmatter.
- Add `clew tag <id> <tag>...` and `clew untag <id> <tag>...` command variants and command modules.
- Reuse existing increment lookup/write seams rather than direct path guessing where possible.
- Touch `updated_at` when tag state changes on an existing increment.
- Keep command stdout consistent with the existing output contract: successful commands that identify an increment should print `#NNNN <repo-relative-path>`.
- Add integration tests for:
  - `clew new "Title" --tag windows --tag distribution <<'EOF' ... EOF`
  - heredoc body remains body-only and verbatim.
  - leading stdin frontmatter is still rejected even when `--tag` is present.
  - duplicate tags are deduped preserving first-seen order.
  - invalid tags are rejected with a useful hint.
  - `clew tag` appends tags and updates `updated_at`.
  - `clew untag` removes tags and updates `updated_at`.
  - `clew list --tag X` can find tags created by both surfaces.
- Update `clew-spec.md` CLI sketch and frontmatter/tag behavior.

Resolved design decision:
- `clew tag <id> existing-tag` is idempotent success.
- `clew untag <id> missing-tag` is a user error, because it likely means the operator mistyped the tag or selected the wrong increment.


# usage guess
from Claude Opus 4.7 
from session where it created two increments, then manually edited the file after to add tags.

Output guess:
```bash
./clew new "Verify Clew on WSL and Git Bash" <<'EOF'
---
tags: [windows]
---

# Verify Clew on WSL and Git Bash

> **Status: backlog.** ...
EOF
```

The reasoning: the increment file _as written to disk_ has frontmatter on top, so the natural agent guess is "stdin mirrors the on-disk shape, Clew merges my frontmatter with its CLI-managed fields (`id`, `status`, `created_at`, `updated_at`)."

But — per the spec at `clew-spec.md:346`, this is exactly the case Clew explicitly rejects: _"stdin is body-only, and leading frontmatter delimiters are rejected."_ So the guess fails fast with an error, which is the right behavior (avoids ambiguous merge semantics) but means the agent's only path today is: create with stdin body, then either direct-edit frontmatter or wait for a future `--tag` flag on `clew new`.

Worth noting as a small papercut — `clew new --tag windows` is likely justified because "tag at creation" is the one frontmatter field where the timing matters (you'd otherwise have to do a two-step create-then-edit dance).

---
*(Nikko: maybe additionally suggests clew command to add tags or frontmatter?)*
Opus 4.7 guess for that:
**1. A dedicated `clew tag` subcommand** — narrowest, matches the existing verb-per-job style (`block` / `unblock`, `start` / `done`):

```bash
./clew tag 0019 windows
./clew tag 0019 windows p0           # multiple in one call
./clew untag 0019 windows
```

This is what I'd reach for first because tags are the one frontmatter field the spec already calls out as CLI-aware (`clew list --tag X`), so a paired write command is symmetric with the read command. Cheap to implement, no flag parsing, scriptable.