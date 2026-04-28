# Clew — Project State

This directory holds Clew project state (increments, archive, path, relay).

## Creating increments

Create an empty backlog item:

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

Clew writes frontmatter itself; stdin is body content only.
