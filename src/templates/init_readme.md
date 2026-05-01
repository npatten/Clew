# Clew — Project State

This directory holds Clew project state (increments, archive, path).

## Agent contract

Copy this contract into the persistent instruction artifact your coding harness reads (`AGENTS.md`, `CLAUDE.md`, Cursor rules, Codex instructions, a skill, or equivalent):

```markdown
## Clew workflow

### Core concepts

- **Increment** — a standalone unit of work that should leave the codebase stable, tested, and committable when complete.
- **Task** — a Markdown checkbox inside an Increment.
- **Path** — the hand-curated priority order across active Increments, stored in `.clew/path.md`.
- **Archive** — completed or abandoned Increments moved to `.clew/archive/`.
- **Tag** — lightweight frontmatter classification for filtering, such as `bug`, `docs`, or `p0`.

### Use Clew to track work in this project

- Run `clew list` to see all active increments. It excludes archived and abandoned work.
- If given an ID, run `clew show <id>` before starting implementation to get the full increment body via stdout.
- Run `clew start <id>` before starting work.
- Keep discoveries, decisions, and remaining tasks in the Increment markdown.
- Do not run `clew done <id>` until the code is stable and project checks pass.
- When complete, run `clew done <id>` and commit the code changes with the `.clew/` changes.
- Create new work with `clew new "Short title"`; pass a Markdown body on stdin for detailed Increments.
```

## Common commands

| Need                | Command                            |
| ------------------- | ---------------------------------- |
| See active work     | `clew list`                        |
| Filter by status    | `clew list --status todo`          |
| Filter by tag       | `clew list --tag bug`              |
| Read an Increment   | `clew show 0024`                   |
| Create an Increment | `clew new "Short title"`           |
| Create with body    | `clew new "Short title" < body.md` |
| Add tags            | `clew tag 0024 bug p0`             |
| Remove tags         | `clew untag 0024 p0`               |
| Start work          | `clew start 0024`                  |
| Finish work         | `clew done 0024`                   |

## Creating increments

Create a backlog item with a title heading:

```bash
clew new "Add OAuth routes"
```

Create an increment with a Markdown body by passing non-TTY stdin:

```bash
clew new "Add OAuth routes" <<'EOF'
## Context

Why this increment matters.

## Tasks

- [ ] First task
EOF
```

Attach tags at capture time with repeated singular `--tag` flags:

```bash
clew new "Verify Clew on WSL" --tag windows --tag distribution <<'EOF'
## Goal
Verify Clew works on WSL.
EOF
```

Clew writes frontmatter itself; stdin is body content only. Passing stdin replaces the default title-heading body. Tags must match `[a-z0-9][a-z0-9-]*`.

For existing increments, use:

```bash
clew tag 0019 windows p0
clew untag 0019 windows
```

## Reviewing archive moves

`clew done`, `clew abandon`, and `clew reopen` move increment files with normal filesystem renames. Clew does not run `git mv` or mutate the git index.

Before staging, `git status --short` may show a deleted file plus an untracked file or directory. After `git add -A`, git normally reports the move as a rename:

```text
R  .clew/increments/0001-example.md -> .clew/archive/0001-example.md
```
