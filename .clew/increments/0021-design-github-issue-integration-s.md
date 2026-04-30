---
id: 21
status: backlog
tags:
- ready-for-human
created_at: 2026-04-29T03:20:55Z
updated_at: 2026-04-30T03:32:54Z
---

# Design Github issue integration(s)

## Goal

Refine [[github-issues-integration-thread]] into a concrete design and then turn into likely multiple increments _(first Epic use case?)_.

## Context

Key decisions to settle:

- Is v1 `publish`, `mirror`, `export`, or true `sync`?
- source of truth?
- Merge nightmares?
  - How do we detect or avoid overwriting GitHub-side edits?
- Where does GitHub repo/link metadata live?

## Tasks

- [ ] Review the thread and tighten the problem statement
- [ ] Decide v1 source-of-truth and command naming
- [ ] Define GitHub metadata and issue body ownership model
- [ ] Split implementation into small follow-up increments
