# Guidelines

These are the guidelines and rules for agents working on the Clew project.
In Greek myth, Ariadne gave Theseus the "Clew" (a ball of thread) to navigate the labyrinth so he could find his way back out. It represents the lightweight tracking of Tasks and Increments ensuring the codebase remains stable and you never lose your way.

## Conversational Style

- Keep answers short and concise
- No fluff, no emojis, no flattery
- Be kind but direct (e.g., "Thanks @user" not "Thanks so much @user!")
- When designing or planning, co-design — don't just accept user direction. Push back on architecture, abstractions, data models when needed.
- Check the user, tell them when they're wrong.

## Resources

- Full project spec: `clew-spec.md`
- All open work items run: `./clew list`

## Software development

You are the principal engineer — guide the process, use sub-agents in parallel where appropriate, avoid unnecessary complexity and tech debt.

- **Ask before removing** functionality or code that appears intentional
- Do **not** preserve backward compatibility unless the user explicitly asks
- Prefer functional style, simple composable components
- Raise larger refactors with the user before starting
- Conserve context: scout sub-agents for whole-file reads; parallel sub-agents for independent tasks

### Design discipline

- **Simple ≠ easy.** Familiar is not simple. Before choosing a construct, ask: does it _complect_ (braid together) concerns that could be independent? Can the next reader reason about one piece without loading the others? "I already know this pattern" is not an argument.
- **Modularity is not simplicity.** Splitting tangled code across files leaves it tangled. Watch for hidden coupling: shared mutable state, implicit ordering, call-site assumptions about internals.
- **Name the downside.** Any tool, pattern, or abstraction you propose — state its cost alongside its benefit. "Has benefit X" without "and costs Y" is incomplete analysis.

### Code organization

- **Modern module style** — `core.rs` + `core/` directory, not `core/mod.rs`.
- **`core/` is pure logic** (no I/O); **`storage/` is the I/O seam**; **`commands/` orchestrates**. Keep the seam clean — pure logic stays unit-testable without tempdirs.
- **`lib.rs` + `main.rs` split** — `main.rs` stays thin; integration tests import the library.

### Git

- One feature / increment at a time; logical chunks that make sense together
- ONLY commit files YOU changed as part of the current increment / chunk
- Prefix commit messages with `[#NNNN]` when the work belongs to a real Clew increment. Use a plain message for repository/process cutovers that are not tied to an increment.
- Credit your clanker as co-author. Always prefix the co-author line(s) with a `Clankers:` header, e.g.:

```
  Clankers:
  Co-Authored-By: Claude <noreply@anthropic.com>
  Co-Authored-By: Codex <noreply@openai.com>
```

### Clew workflow

#### Core rules

- Always invoke Clew as `./clew` from the repo root.
- Documented loop commands: `init`, `new`, `start`, `done`, `show`, `list`. Other commands may be wired and visible in `./clew --help`, but are not part of the stable MVP loop yet.
- Prefer Clew's documented no-flag workflow; the core loop is mostly positional.
- Do not guess flags. Use the no-flag form unless `./clew <cmd> --help` documents a flag needed for the task.
- Use `./clew list` as the canonical view of project work. Prefer it over inspecting `.clew/increments/` or `.clew/archive/` directly.
- Let `clew new` allocate IDs. Do not pre-compute the next ID from list output.

#### Starting work

- If the user has not provided an increment ID, ask before proceeding.
- Run `./clew show <id>` which returns the full increment text; review carefully, ask questions, suggest improvements.
- Run `./clew start <id>` before beginning implementation.

#### Creating work

Prefer creating increments with the body supplied on stdin:

```bash
./clew new "Title here" <<'EOF'
## Goal
...
EOF
```

- Keep titles under 7 words.
- Use direct markdown edits for backlog sharpening and simple metadata changes when clearer than adding CLI ceremony.
- If material work emerges without an associated increment, pause and propose creating one before continuing.

#### Closing an increment

1. Finish the implementation.
2. Sanity check docs (especially `clew-spec.md`) and make any required updates.
3. Run the full quality gate and promote the stable local runner:

   ```bash
   scripts/promote-clew
   ```

   This runs:

   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo build --release
   ```

- Failures are stop-the-line. Fix the root cause; no `#[allow(...)]`, `--no-verify`, or reformat-then-ignore without explicit user approval.
- Re-run after every fix until the promotion script passes in a single sweep.
- If `cargo` is unavailable, say so explicitly rather than silently skipping.

4. Ask the user for approval.

Only after user approval:

5. Mark the increment done with `./clew done <id>`.
6. Commit code, docs, and updated Clew files.
7. Run `git status --short` and confirm the workspace is clean or only contains expected unrelated changes.

Do not claim an increment is complete unless the quality gate passed and Clew work items are updated.

### Plan discipline

`clew-spec.md` is the load-bearing living spec. It carries a `## Revisions` log and a `last_major_update` frontmatter field for human reviewers picking it up async.

- **When you make a meaningful design edit to `clew-spec.md`** (a pivot, a new decision, a deferral, a structural change), add one line to `## Revisions` and bump `last_major_update` to today's date.
- **Skip both for typo fixes, wording polish, formatting tweaks** — the threshold is "would a returning reviewer want to know this changed?"
- **Revisions entries describe the _why_, not the diff.** "Collapsed status set; dropped `blocked` as a status" — not "edited section 4."
- **Keep ~5 entries.** Prune the oldest when adding new ones; git history is the long memory.

## Agent skills

Matt Pocock engineering skills must read `docs/agents/*.md` before making issue-tracker, triage, or domain-doc assumptions.

### Issue tracker

Work items are tracked in Clew increments, not GitHub Issues. See `docs/agents/issue-tracker.md`.

### Triage labels

Matt's triage roles map to Clew tags, with `wontfix` handled by abandoning the increment. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context repo: read `CONTEXT.md` for vocabulary and `clew-spec.md` for the full living design. See `docs/agents/domain.md`.
