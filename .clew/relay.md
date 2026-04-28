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

### Clew Usage Issues

Sonnet hallucinated 
Bash Create increment 0010 for distribution
`./clew new --id 0010 --title "distribution: ship clew so others can install it" 2>&1`
OUT
`Exit code 2
`error: unexpected argument found`

self reported full timeline from sonnet:
---
**Context gathering**

- Read `.clew/relay.md` — standard session start per AGENTS.md; told me #0007 was last completed and gave candidate next picks
- `grep` on `crew-plan.md` for deploy/bundle/distribute keywords — wanted to know what was already decided before writing anything; found the one relevant line ("Default to `cargo install` for v1; revisit later")

**Scoping the backlog**

- `ls .clew/*.md` — tried to get a rough picture of existing increments; only returned `path.md` because the increments are actually in `.clew/increments/`, not `.clew/` root — slight mismatch between my mental model and the actual layout
- `./clew list` — correct tool for the job; did exactly what I expected: showed all backlog items with IDs, let me confirm the next ID would be `0010`

**Creating the increment**

- `./clew new --id 0010 --title "..."` — failed with "unexpected argument"; I assumed flags I'd seen in other CLIs, didn't check `--help` first
- `./clew new --help` — should have done this first; revealed the correct signature (positional title only)
- `./clew new "distribution: ..."` — worked; returned bare `0010` on stdout, which is clean

**Verifying**

- `./clew show 10` — output was missing the `title` field in the frontmatter, which was unexpected; the title should have been persisted from `new` but wasn't visible in `show`'s output
- `find .clew -name "0010*"` — had to resort to this because `show` didn't confirm enough; found the file directly
- Read + Edit the file directly — added body content by hand since there's no `clew edit` command yet

**Notable friction points:**

- The `ls .clew/*.md` pattern misses `.clew/increments/` — a new agent would likely make the same mistake
- `./clew show` doesn't display the title even though `./clew list` does — inconsistent
- No way to set body content at create time (stdin heredoc support exists per #0007, but I didn't use it — I forgot and fell back to direct file edit)

---
and Opus observations from logs:
1. **`clew new` flag discovery failure.** Claude tried `--id` and `--title` as flags — reasonable guesses from other CLIs. Neither exists. The `--help` recovery is fine, but the initial misfire suggests the `new` usage string could be more distinctive. Right now `clew new --help` probably says `<TITLE>` as a positional — that's easy to miss if you're pattern-matching on "new + title flag."
    
2. **`clew list` output is the right first move.** Before creating the increment, Claude ran `list` to establish current state and infer the next ID. That's sensible. But the fact that the agent then _hardcoded_ `0010` (rather than letting `clew new` pick it) shows it's doing ID arithmetic mentally — which is fragile. The ID allocation is implicit from the output; nothing in the `list` output says "next ID will be X."
    
3. **`clew show` after `new` returns bare.** The show output for a freshly-created increment shows only frontmatter, no body — because there is no body yet. That's correct, but the agent had to then go add content separately. The `new` → `show` → edit flow is slightly clunky. Possibly worth noting whether the `new` command's stdout (just `0010`) is enough signal to proceed, or whether agents consistently reach for `show` immediately after.