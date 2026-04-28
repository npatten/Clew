---
id: 7
status: done
created_at: 2026-04-28T02:17:10Z
updated_at: 2026-04-28T02:51:39Z
---

# Add stdin/heredoc support for `clew new`

## Why

Observed agent flow today: `clew new "title"` → `read` the new file → `write` body content back. Three round-trips for one logical action. Collapsing to a single `clew new "title" <<'EOF' … EOF` keeps the agent's edit local to one tool call and avoids re-reading a file we just wrote.

Heredoc is the natural shell idiom for "command + multi-line body" (`git commit -F -`, `mail`, `kubectl apply -f -`). Agents already produce heredoc-shaped Bash invocations comfortably.

## Design

### Trigger: non-TTY stdin

When `std::io::stdin().is_terminal()` is **false**, read stdin verbatim and store it as the increment body. When **true** (interactive shell, no redirect), behave as today — body stays empty.

> When stdin is non-TTY, `clew new` reads it verbatim and stores it as the increment body. Clew owns frontmatter; stdin is body content only. This intentionally makes heredoc creation ergonomic, at the cost that scripts with redirected stdin must use `< /dev/null` when they want an empty-body increment.

This makes all of these Just Work:

```bash
./clew new "title" <<'EOF'
## Context
...
EOF

echo "body" | ./clew new "title"

./clew new "title" < body.md

# explicit empty-body in a scripted context
./clew new "title" < /dev/null
```

Interactive `./clew new "title"` at a prompt still produces an empty-body backlog item — no behavior change.

### Body handling

- Read stdin via `std::io::read_to_string(std::io::stdin())`.
- **Preserve verbatim.** No trimming of leading or trailing whitespace. A trailing newline from heredoc is normal for text files; tests assert the exact byte sequence rather than papering over it. If `frontmatter::serialize` doubles a trailing newline when joining, that's a serializer bug to fix there, not here.
- Empty stdin (0 bytes) → empty body, identical to today. Important for agent harnesses that wire stdin to `/dev/null`.

### Frontmatter passthrough

Stdin is the **body only**. We do not parse frontmatter from stdin and merge — that's a footgun (which `id` wins? which `created_at`?). The CLI owns frontmatter; the operator owns prose. If a future need emerges (importing increments wholesale), that's a separate `clew import` command.

If the stdin payload starts with `---\n` or `---\r\n`, error with a clear message:

```
error: stdin appears to contain frontmatter (starts with `---`).
       `clew new` writes frontmatter itself; pass body content only.
```

Cheap guardrail, prevents the most likely misuse. A `---` later in the body (e.g., a thematic-break inside prose) is allowed — only the leading sequence is rejected.

### Interaction with future `--body` flag

Not adding `--body` in this increment, but reserve the design: if both stdin (non-TTY) and `--body` are ever passed, error rather than pick a precedence. Document in the help text when `--body` lands.

### Error model

- Stdin read I/O failure → `ClewError::Io` (existing variant, exit code 2).
- Frontmatter-in-stdin guardrail → new typed variant (e.g., `ClewError::InvalidStdin(&'static str)` or similar). The failure mode is "user passed forbidden input," not "frontmatter parse failed" — reusing `Frontmatter` would muddy the variant's meaning. Exit code 1 (user error).

## Tasks

- [x] Add stdin read to [src/commands/new.rs](src/commands/new.rs): detect non-TTY via `std::io::IsTerminal`, read verbatim with `read_to_string`.
- [x] Plumb the body string through `ParsedFile.body` instead of `String::new()`.
- [x] Add the `---\n` / `---\r\n` leading-prefix guardrail with a clear error message.
- [x] Add the new typed `ClewError` variant for invalid stdin payload; map to exit code 1.
- [x] Verify `frontmatter::serialize` handles a non-empty body with trailing newline cleanly (no doubled newlines). Fix the serializer if it doesn't.
- [x] Update the `clew new` `--help` text (clap doc comment) to mention stdin body.
- [x] Integration test: piped stdin body preserved exactly, including trailing newline.
- [x] Integration test: heredoc-equivalent stdin (via `assert_cmd`'s `.write_stdin(...)`) round-trips correctly.
- [x] Integration test: empty stdin (`< /dev/null` shape) produces empty body — agent harness path.
- [x] Integration test: leading whitespace in body preserved.
- [x] Integration test: stdin starting with `---\n` returns a user error (exit 1) with the guardrail message.
- [x] Integration test: stdin starting with `---\r\n` rejected the same way.
- [x] Integration test: `---` appearing later in the body is allowed and preserved.
- [x] Integration test: no-redirect interactive case still writes empty body — unchanged behavior. Skip with a comment if `assert_cmd` cannot simulate a TTY; document the manual-test alternative.
- [x] Update `.clew/README.md` template (and any user-facing help) to document the heredoc form.
- [x] Update [hammock-thinking/crew-plan.md](hammock-thinking/crew-plan.md) CLI sketch entry for `clew new` to mention stdin body.

## Open questions

_None._

## Decisions

- **stdout stays as the bare ID.** No "captured N bytes" or path echo when a body is provided — the contract `clew new` already has (a single 4-digit ID line) holds for both empty-body and body-from-stdin cases. Filepath-in-output is a separate concern tracked in [#0009](.clew/increments/0009-add-filepath-in-responses.md) and should be designed once across all path-mutating commands, not bolted onto `new`.
