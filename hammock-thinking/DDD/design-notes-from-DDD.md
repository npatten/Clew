# CLI Design Notes

Cross-cutting observations surfaced while drafting interaction paths in `interaction-paths.md`. Captured here as we go so nothing gets lost in context. Synthesis into actual CLI design decisions happens after enough paths are drafted.

Each entry: short title, the observation, source paths (filled in as we draft them), and a tentative direction (if any).

---

## Editor-spawning commands and the agent flow

**Observation:** Several commands in the current CLI sketch (`clew path`, `clew relay <id>`, implicitly `clew new`) imply opening `$EDITOR`. That works for humans. The agent equivalent is unclear, and the right shape probably hinges on minimizing tool-call round-trips and token cost.

**TTY detection is a reliable dispatch signal.** Tested across three agent modes (Pi, Claude Code extension, Claude CLI) — all return `No TTY / Non-interactive`. Humans at a terminal return `TTY Detected / Interactive`. So `[ -t 0 ]` is a sound basis for dynamic dispatch: open `$EDITOR` for humans, fall back to deterministic CLI behavior (stub-and-print-path or stdin) for agents. No flag needed.

**Shapes worth thinking through (none decided yet):**

- _Stub-and-print-path_ — CLI creates the file with seed frontmatter, prints its relative path; caller edits via whatever tool they have (human's editor, agent's file-edit tool). Minimum round-trips for an agent: 1 CLI call + 1 file read + N edits.
- _Argument-driven_ — `clew new increment --title "..." --tags "..."`; everything passed at the command line. Cheap for agents on simple cases; awkward for prose-heavy fields.
- _Stdin-driven_ — caller pipes a prepared body into the CLI. Same round-trip cost as stub-and-print-path but lets the CLI validate before writing.
- _TTY-detected_ — open `$EDITOR` if interactive, fall back to one of the above otherwise. Convenient for humans without breaking agents.

**What we don't know yet:** which shape (or combination) actually minimizes agent tokens across the full IP coverage. Worth re-examining once IP-05, IP-08, IP-09, IP-11 are drafted with concrete CLI-call sketches in their steps.

**Surfaces in:** IP-05 (write relay), IP-08 (capture new work), IP-09 (decompose plan), IP-11 (reorder path), and any other path where the human would naturally edit a file in-place.

---

## Prefer new subcommands over conditional flags for agent ergonomics

**Observation (from external research):** Agents navigate dedicated subcommands better than flags that alter a command's behavior. A flag like `--fragment` or `--bulk` changes what a command _does_, which the agent has to reason about; a distinct command (`clew capture`, `clew import`) is unambiguous.

**Tentative principle:** When a behavior fork is significant enough to have a different destination, output shape, or validation path, give it its own command rather than a flag on an existing one. Flags are fine for modifiers (e.g., `--status`, `--tag`, `--stdin`) that tune behavior without changing its nature.

**Surfaces in:** IP-08 (`clew new` vs. `clew capture` for fragments), IP-09 (bulk import as `clew import` vs. `clew new --bulk`).

---

## Editor resolution: $EDITOR/$VISUAL unreliable with modern Electron-based editors

**Observation:** Most developers today use Electron-based editors (VS Code, Cursor, Zed). These install CLI shims into `$PATH` (`code`, `cursor`, `zed`) but typically do not set `$EDITOR` or `$VISUAL` at install time. The shims also require a `--wait` flag to block until the file is closed — without it, they open the file and exit immediately, breaking the typical `$EDITOR` contract.

**Consequence:** Relying purely on `$EDITOR` will fail silently for a large portion of users. `$EDITOR=code` without `--wait` is wrong; `$EDITOR` unset is common.

**Tentative directions (not decided):**

- _`clew init` scans `$PATH` for known editors_ (`code`, `cursor`, `zed`, `nvim`, `vim`, `nano`) and prompts the user to confirm a preferred one, storing it in `.clew/config.toml` as `editor = "code --wait"`. One-time setup; handles the `--wait` requirement explicitly.
- _`.clew/config.toml` `editor` key_ overrides `$EDITOR`/`$VISUAL` when set. Clew-local config sidesteps the global env var problem.
- _Safe fallback_: if no editor is configured and TTY is present, print the file path and a short instruction rather than crashing. Never block on an unconfigured editor.

**What to decide later:** whether init auto-detects or just prompts; whether `$EDITOR` is respected at all or only `.clew/config.toml` is used.

**Surfaces in:** IP-01 (`clew init`), any editor-spawning command (relay, path).

---

## Relays may need to exist outside the increment frame _(maybe)_

**Observation:** The current relay format assumes one relay per numbered increment (`.clew/relays/{id}.md`, `clew relay <id>`). Dogfooding the format on this DDD work — which isn't tracked as a Clew increment, since Clew doesn't exist yet — required improvising a `topic:` field instead of `increment:`. The format itself worked well; the binding to an increment ID was the only thing that didn't fit.

**Two possible reads, neither confirmed:**

- This is just a side effect of experimenting before Clew exists. Once Clew is real, all such work would naturally be tracked as increments under an epic, and the constraint stops biting.
- There may be a generalizable pattern — session-local context that hasn't crystallized into an increment yet (early exploration, design rounds, spike work) — that wants relay-shaped artifacts but no increment to attach to.

**Hold off on resolving.** Revisit after IP-03, IP-05, and IP-09 are drafted; if the pattern shows up in path bodies organically, it's real.

**Surfaces in:** IP-05 (write relay), possibly IP-09 (decompose plan — early sessions on a fresh PRD might want session context before any increment exists).
