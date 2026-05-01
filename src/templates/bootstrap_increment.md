# Bootstrap Clew

## Goal

Connect Clew to your agent instructions so future agents can find and use project work items.

## Instructions

Copy this agent contract into the persistent instruction artifact your coding harness reads, such as `AGENTS.md`, `CLAUDE.md`, Cursor rules, Codex instructions, a skill, or an equivalent project-level instruction file.

---

## Clew workflow

#### Core concepts

- **Increment** — a standalone unit of work that should leave the codebase stable, tested, and committable when complete.
- **Task** — a Markdown checkbox inside an Increment.
- **Path** — the hand-curated priority order across active Increments, stored in `.clew/path.md`.
- **Archive** — completed or abandoned Increments moved to `.clew/archive/`.
- **Tag** — lightweight frontmatter classification for filtering, such as `bug`, `docs`, or `p0`.

#### Use Clew to track work in this project

```md
- Run `clew list` to see all active increments. (Excludes archived and abandoned)
- If given an ID, run `clew show <id>` before starting implementation to get the full increment body via stdout
- Run `clew start <id>` before starting work.
- Keep discoveries, decisions, and remaining tasks in the Increment markdown.
- Do not run `clew done <id>` until the code is stable and project checks pass.
- When complete, run `clew done <id>` and commit the code changes with the `.clew/` changes.
- Create new work with `clew new "Short title"`; pass a Markdown body on stdin for detailed Increments.
```

#### Creating detailed Increments

```bash
clew new "Add OAuth routes" --tag auth <<'EOF'
## Goal

Add route handlers for OAuth login.

## Tasks

- [ ] Add route definitions
- [ ] Add request validation
- [ ] Add tests
EOF
```

Clew writes frontmatter itself; stdin is body content only.

## Done when

- [ ] The agent contract above is copied into your preferred persistent agent instruction artifact.
- [ ] `.clew/` and the instruction artifact are committed together.
- [ ] A future agent can run `clew list` and understand how to continue work.
- [ ] This bootstrap is marked done with `clew done 0000`.
