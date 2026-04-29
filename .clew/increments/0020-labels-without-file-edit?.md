---
id: 20
status: backlog
created_at: 2026-04-29T02:55:10Z
updated_at: 2026-04-29T02:55:10Z
---
## Question
Should Clew support adding labels when an increment is created?

## Context
Today labels can only be added by editing the increment file after creation. That keeps the creation flow small, but makes labeling feel like a manual follow-up step instead of part of capture.

## Idea
Consider a lightweight way to attach labels during `clew new`, without expanding the control surface too much.

Possible shapes:
- Keep creation positional, but prompt for optional labels interactively.
- Support labels in the stdin body/frontmatter if already present.
- Add a documented flag only if the ergonomics justify the extra API surface.

## Design concern
Adding label input to creation may make the MVP loop feel more complicated. The feature should only exist if it improves common capture flow without making `clew new` harder to explain.

## Acceptance notes
- Decide whether label-on-create belongs in the documented workflow.
- If yes, choose the smallest interface that preserves simple capture.
- Update `clew-spec.md` if the creation workflow changes.


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

Worth noting as a small papercut — might be worth a backlog item for `clew new --tag windows` as a flag, since "tag at creation" is the one frontmatter field where the timing matters (you'd otherwise have to do a two-step create-then-edit dance).

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