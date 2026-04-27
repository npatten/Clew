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

- Full project plan: `hammock-thinking/crew-plan.md`
- Hand off from previous agent session: `relay.md`

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

### Git

- One feature at a time; logical chunks that make sense together
- ONLY commit files YOU changed as part of the current chunk
- Do not prefix commit messages with Clew increment refs until this repo has been bootstrapped with real `.clew/` increments. Use plain messages for now.
- Credit your clanker as co-author. Always prefix the co-author line(s) with a `Clankers:` header, e.g.:
  - `Clankers:`
  - `Co-Authored-By: Claude <noreply@anthropic.com>`
  - `Co-Authored-By: Codex <noreply@openai.com>`

### Quality gates

Run at every milestone (finishing a logical chunk, before commit, before claiming success):

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

- All three green before commit or reporting success — tests passing alone isn't enough.
- Failures are stop-the-line. Fix the root cause; no `#[allow(...)]`, `--no-verify`, or reformat-then-ignore without explicit user approval.
- Re-run after every fix until all three pass in a single sweep.
- If `cargo` is unavailable, say so explicitly rather than silently skipping.

### Plan & Relay discipline

`crew-plan.md` is the load-bearing living spec. It carries a `## Revisions` log and a `last_major_update` frontmatter field for human reviewers picking it up async.

- **When you make a meaningful design edit to `crew-plan.md`** (a pivot, a new decision, a deferral, a structural change), add one line to `## Revisions` and bump `last_major_update` to today's date.
- **Skip both for typo fixes, wording polish, formatting tweaks** — the threshold is "would a returning reviewer want to know this changed?"
- **Revisions entries describe the _why_, not the diff.** "Switched relay to single rolling file (per-increment didn't justify the scale)" — not "edited section 4."
- **Keep ~5 entries.** Prune the oldest when adding new ones; git history is the long memory.

`relay.md` is the session handoff (format guidance in: `crew-plan.md` → Relay format). Keep it current with every commit.

**Milestone close protocol:**

1. Finish the chunk / milestone.
2. Run the full quality gate.
3. Update `relay.md` to with essential context for next time and the next milestone.
4. Ask for user approval.
5. Only after user approval: Commit work + `relay.md` together
6. Confirm `git status --short` clean before reporting success.

Do not claim a milestone is complete unless quality gate passes and `relay.md` reflects the latest committed state.

**Writing the relay:**

- Goal: capture what's expensive to re-derive next session.
- `Next milestone` is the next product milestone/increment of work, not process mechanics like asking for review, running the gate, or committing.
- If process state matters, capture it under Status or Context worth carrying.
- Capture decisions and gotchas the next agent would otherwise lose time on.
- Prefer exact paths, command names, and commit hashes already at hand.
- Don't restate what's in crew-plan.md; reference where valuable.
- Summarize — git logs detail.
- Omit empty sections.
