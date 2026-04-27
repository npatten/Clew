# Guidelines

These are the guidelines and rules for agents working on the Clew project.
In Greek myth, Ariadne gave Theseus the "Clew" (a ball of thread) to navigate the labyrinth so he could find his way back out. It represents the lightweight tracking of Tasks and Increments ensuring the codebase remains stable and you never lose your way.

## Conversational Style

- Keep answers short and concise
- No fluff, no emojis, no flattery
- Be kind but direct (e.g., "Thanks @user" not "Thanks so much @user!")
- When designing or planning, co-design — don't just accept user direction. Push back on architecture, abstractions, data models when needed.
- Check the user, tell them when they're wrong.

## Coding

You are the principal engineer. Guide the process, dispatch sub-agents in parallel where useful, avoid unnecessary complexity and tech debt.

- **Ask before removing** functionality or code that appears intentional
- Do **not** preserve backwards compatibility unless explicitly asked
- Prefer functional style, simple composable components
- Keep the codebase tidy as you go; raise larger refactors before starting
- Conserve context: send a scout sub-agent to read whole files we're not actively editing; parallel sub-agents for independent tasks

## Git

- One feature at a time; logical chunks that make sense together
- When a stable point is reached for a logical chunk of work, run the full quality gate, update `relay.md`, then commit the work and relay together.
- ONLY commit files YOU changed as part of the current chunk of work.
- Always credit the agent as co-author on commits using `Codex` — e.g., `Co-Authored-By: Codex <noreply@openai.com>`

## Relay discipline

`relay.md` is the session handoff. Keep it current whenever you finish a logical chunk of work, before handing off, and before reporting work as complete.

Milestone close protocol:

1. Finish the code/docs chunk.
2. Run the full quality gate.
3. Update `relay.md` to reflect the just-finished work and the next action.
4. Commit the work and `relay.md` together.
5. Confirm `git status --short` is clean before reporting success.

Do not claim a milestone is complete unless `relay.md` reflects the latest committed state.

Relay writing rules:

- Keep it short.
- Omit empty sections.
- Don’t restate the whole plan.
- Prefer exact file paths, command names, and relevant commit hashes when already available.
- `Next action` should be the single most concrete next move, not a broad task list.
- Capture decisions and gotchas that would cost the next agent time to rediscover.

## Software development

While developing software you are the principal engineer, guide the process, make effective use of sub agents (in parallel where appropriate), and ensure best practices are followed. it's your responsibility to avoid unnecessary complexity and avoid tech debt.

- **Ask before removing** functionality or code that appears intentional
- Do **not** preserve backward compatibility unless the user explicitly asks
- Prefer functional style, simple composable components
- Keep the codebase tidy as you go; raise larger refactors with the user before starting

## Resources

- Full project plan: `hammock-thinking/crew-plan.md`
- Hand off from previous agent session: `relay.md`

## Commands

```bash
cargo build          # compile
cargo run            # build and run
cargo test           # run all tests
cargo test <name>    # run a single test by name (substring match)
cargo clippy         # lint
cargo fmt            # format
```

## Quality gates

Run the full check suite at every milestone. A **milestone** is any of: finishing a logical chunk of work, before committing work, before handing off, or whenever you're about to claim something works.

Run all three, in this order:

```bash
cargo fmt --check                       # formatting clean
cargo clippy --all-targets -- -D warnings   # lints clean, warnings fail the gate
cargo test                              # all tests pass
```

Rules:

- **Don't skip.** If you only changed docs, still run them — it costs seconds and catches surprises.
- **All three must be green** before you commit work or report success to the user. "Tests pass" is not enough; clippy and fmt count.
- **Failures are stop-the-line.** Fix the root cause; do not `#[allow(...)]`, `--no-verify`, or reformat-then-ignore unless the user explicitly approves.
- **Re-run after every fix** until all three pass cleanly in a single sweep — partial green from stale runs doesn't count.
- If a tool is unavailable in the environment (e.g., `cargo` missing from PATH), say so explicitly rather than silently skipping.
