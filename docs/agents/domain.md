# Domain Docs

How engineering skills should consume this repo's domain documentation.

## Layout

This is a single-context repo.

Before architecture, diagnosis, TDD, issue-writing, or PRD work, read:

1. `CONTEXT.md` — compact domain vocabulary for Clew.
2. `clew-spec.md` — full living design source and authoritative product semantics.
3. `docs/adr/` — architectural decision records, if this directory exists.

If ADRs do not exist, proceed silently. Do not suggest creating ADRs upfront unless a durable architectural decision has actually been made.

## Source of truth

- `CONTEXT.md` defines preferred vocabulary and terms to avoid.
- `clew-spec.md` remains authoritative for detailed behavior, data model, workflow, and open questions.
- If `CONTEXT.md` and `clew-spec.md` disagree, treat it as drift: flag it and ask before making a semantic change.

## Use the glossary vocabulary

When output names a domain concept, use the terms from `CONTEXT.md`.

Examples:

- Say **Increment**, not issue or ticket.
- Say **Task**, not sub-issue.
- Say **Path**, not roadmap.
- Say **Archive**, not closed issues.
- Say **Tag**, not label, when referring to Clew frontmatter tags.

## Multi-context repos

This repo is not multi-context. Do not look for `CONTEXT-MAP.md` unless the repository layout changes later.
