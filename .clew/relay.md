---
topic: Clew CLI — #0007 stdin/heredoc support ready for review
updated_at: 2026-04-28T02:52:13Z
---

# Relay: #0007 stdin/heredoc support ready for review

## Context worth carrying

- Increment #0007 implemented stdin/heredoc support for `clew new` and was archived via `./clew done 7`.
- `clew new` now reads non-TTY stdin verbatim as body content, rejects leading `---\n` / `---\r\n` frontmatter-like payloads with `ClewError::InvalidStdin`, and keeps stdout as the bare ID.
- Tests that invoke `clew new` through `assert_cmd` must call `.write_stdin("")` unless they intentionally provide body content; otherwise inherited non-TTY stdin can hang while `clew new` reads stdin by design.
- Updated user-facing docs in `src/templates/init_readme.md`, `.clew/README.md`, and the `clew new` plan sketch in `hammock-thinking/crew-plan.md`.
- Quality gate passed after archiving #0007: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Next milestone

Pick the next backlog item from `.clew/`: likely #0002 (`path.md` in-progress section), #0005 (reconcile plan drift around deferred bootstrap behavior), or #0009 (design filepath output consistently across mutating commands).
