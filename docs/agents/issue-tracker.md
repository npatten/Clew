# Issue tracker: Clew

Work items for this repo live in Clew, not GitHub Issues.

GitHub is the code remote only. Do not suggest or use GitHub Issues for project work unless the user explicitly asks for GitHub integration.

## Canonical commands

Always invoke Clew from the repository root as `./clew`.

- List work: `./clew list`
- Read a work item: `./clew show <id>`
- Create a work item: `./clew new "Title"`
- Start work: `./clew start <id>`
- Finish work: `./clew done <id>`

Prefer creating increments with a markdown body supplied on stdin:

```bash
./clew new "Title here" <<'EOF'
## Goal
...
EOF
```

## When a skill says "issue"

Interpret "issue" as a Clew increment.

Use Clew vocabulary in user-facing output where possible:

- Say **Increment** instead of issue/ticket.
- Say **Task** for checkboxes inside an increment.
- Say **Tag** for lightweight labels.
- Say **Status** only for Clew lifecycle state.

## When a skill says "publish to the issue tracker"

Create one or more Clew increments with `./clew new`.

Publish dependency order first: blockers before blocked increments, so later increment bodies can reference earlier IDs.

## When a skill says "fetch the relevant ticket"

Use `./clew show <id>` and read the full increment text before acting.

## Direct markdown edits

Direct edits to `.clew/` files are acceptable for backlog sharpening, metadata tweaks, tags, and body edits when clearer than CLI ceremony.

Prefer CLI commands for lifecycle transitions with side effects, especially `start`, `done`, and `abandon` when available/documented.
