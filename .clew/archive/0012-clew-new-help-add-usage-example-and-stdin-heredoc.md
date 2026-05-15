---
id: 12
status: done
tags:
- ready-for-agent
created_at: 2026-04-28T03:43:50Z
updated_at: 2026-05-15T02:59:34Z
---
## Goal

Improve `clew new --help` output to surface the correct calling convention and the stdin/heredoc feature.

## Why

Agents (and humans) pattern-match on familiar CLIs and try `--title`/`--id` flags that don't exist, then fall back to `--help` to recover. Once they're in `--help`, there's no example and no mention of stdin — so stdin falls back to direct file editing even though #0007 shipped support for it.

Two issues, one fix surface.

## What to add

1. **A concrete usage example** in the help string, e.g.:
   ```
   USAGE:
     clew new "Add OAuth route handlers"
     clew new --ready "Add OAuth route handlers"
     clew new "Add OAuth route handlers" < body.md
     printf 'Body text here' | clew new "Add OAuth route handlers"
   ```

2. **Stdin description** — one line in the help text: "If stdin is non-interactive, it is read verbatim as the increment body."

## Acceptance criteria

- `clew new --help` shows at least one example with the positional title form
- Stdin usage is documented in `--help`
- No behavioral change; help text only

## Implementation notes

- Add command-specific clap help text on `Command::New` in `src/cli.rs`; preserve the existing short summary so top-level help stays stable.
- Prefer appended examples/stdin copy over a custom help template. The examples should cover positional title, `--ready`, file redirection, and pipe stdin.
- Extend `tests/integration_test.rs::new_help_documents_arguments_and_flags` with substring assertions for the examples and stdin sentence.
- Do not change `src/commands/new.rs`, parser semantics, stdin behavior, or docs/spec unless a real inconsistency is found.
- Validate focused help tests and inspect generated help before the final `scripts/promote-clew` gate.
