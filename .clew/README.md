# Clew — Project State

This directory holds Clew project state (increments, archive, path).

## Creating increments

Create an empty backlog item:

```bash
./clew new "Add OAuth routes"
```

Create an increment with a Markdown body by passing non-TTY stdin:

```bash
./clew new "Add OAuth routes" <<'EOF'
## Context

Why this increment matters.

## Tasks

- [ ] First task
EOF
```

Clew writes frontmatter itself; stdin is body content only.

## Reviewing archive moves

`./clew done`, `./clew abandon`, and `./clew reopen` move increment files with normal filesystem renames. Clew does not run `git mv` or mutate the git index.

Before staging, `git status --short` may show a deleted file plus an untracked file or directory. After `git add -A`, git normally reports the move as a rename:

```text
R  .clew/increments/0001-example.md -> .clew/archive/0001-example.md
```
