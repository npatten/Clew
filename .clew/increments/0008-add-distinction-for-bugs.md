---
id: 8
status: backlog
tags:
- needs-triage
created_at: 2026-04-28T02:18:39Z
updated_at: 2026-05-07T00:00:00Z
---

## Goal

Add a first-class `clew bug` capture path so bugs are filed with a structured template and automatically tagged, without adding a new frontmatter type field.

## Design decisions

**No `type` field.** `tags: [bug]` is sufficient for classification and filtering. Adding a dedicated frontmatter field would create a parallel taxonomy alongside tags with no behavioral payoff. `clew list --tag bug` already works today.

**`clew bug "<title>" [flags]`** is a thin subcommand alias over `clew new --tag bug` that substitutes a bug-specific body template. All `clew new` flags pass through (`--tag`, `--ready`, stdin body override).

**Bug body template:**

```markdown
## What happened

<!-- Brief description of the bug -->

## Steps to reproduce

1.

## Expected behavior

## Actual behavior

## Context

<!-- Environment, version, relevant config -->
```

**Stdin override still works.** If stdin is non-TTY, the template is replaced by stdin content (same rule as `clew new`). This lets agents pipe structured bug reports directly.

**`bug` is a well-known tag** — document it in `.clew/README.md` alongside the agent contract. No validation or allowlist; remains advisory.

**No `clew feature` alias yet.** The default `clew new` body is already suitable for features. Add feature/chore aliases only if self-hosting reveals friction.

## Tasks

- [ ] Add `clew bug` subcommand in `src/commands/`
- [ ] Wire bug body template (string constant in `core/`)
- [ ] Pass `--tag bug` automatically; merge with any user-supplied tags
- [ ] Add `clew bug` to CLI sketch in `clew-spec.md`
- [ ] Document `bug` as a well-known tag in `.clew/README.md`
- [ ] Integration test: `clew bug "title"` creates increment tagged `bug` with template body
- [ ] Integration test: stdin override replaces template
