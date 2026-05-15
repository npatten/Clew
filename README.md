<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/banner-dark.svg">
  <img alt="clew — find your way" src="assets/banner-light.svg" width="100%">
</picture>

# Clew

In Greek myth, Ariadne gave Theseus a clew (a ball of thread) to navigate the labyrinth and find the way back out. Clew does the same for small software projects: it keeps local, git-native work state that humans and coding agents can both follow.

There are endless ways to manage software projects, if the below resonates, maybe this one suits you as well.

## What Clew is for

Clew is a lightweight project tracker for hobby projects, tiny teams, and agent-assisted development.

It is designed to:

- run locally with no server, subscription, or external tracker
  - _(GH issues import/export coming soon)_
- store work as committed Markdown in the repo
- fit agent habits with stable IDs and familiar CLI workflows that frontier models are optimized for
- be agent harness agnostic (e.g. can switch between Claude Code and Pi in the same session without issues)
- integrate with normal git workflow without auto-commits or hidden sync
- help keep context light and save tokens by offloading repetitive work to clew
- reduce the cognitive load for humans to capture, sharpen, and implement ideas, so we can focus on the essential bits

## Installation

Install Clew with Homebrew, the GitHub Release shell installer, or Cargo:

```bash
# Homebrew
brew install npatten/tap/clew

# Shell installer from the latest GitHub Release
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/npatten/Clew/releases/latest/download/clew-installer.sh | sh

# Cargo
cargo install clew
```

Windows: use WSL2 for the release artifacts. Native Windows / Git Bash distribution is still experimental.

## First-time setup

From a project repo:

```bash
clew init
```

This creates `.clew/` project state. Commit it alongside your code changes.

Then open `.clew/README.md` and copy the recommended agent instructions into the persistent instruction file your agent reads for this repo: `AGENTS.md`, `CLAUDE.md`, Cursor rules, Codex instructions, a skill, or equivalent.

`clew init` creates an initial bootstrap Increment with instructions to do this; asking your clanker to do it for you works great.

In [Pi](https://pi.dev/):

```bash
!clew show 0
Please follow the above and update our Agents.md
```

## Essential workflow for agents

_Recommended agent context: copy this into your project agent instructions._

#### Core concepts

- **Increment** — a standalone unit of work that should leave the codebase stable, tested, and committable when complete.
- **Task** — a Markdown checkbox inside an Increment.
- **Path** — the hand-curated priority order across active Increments, stored in `.clew/path.md`.
- **Archive** — completed or abandoned Increments moved to `.clew/archive/`.
- **Tag** — lightweight frontmatter classification for filtering, such as `bug`, `docs`, or `p0`.

#### Use Clew to track work in this project

```md
- Run `clew list` to see all active increments. (Excludes archived and abandoned)
- If given an ID, run `clew show <id>` before starting implementation to get the full increment body via stdout
- Run `clew start <id>` before starting work.
- Keep discoveries, decisions, and remaining tasks in the Increment markdown.
- Do not run `clew done <id>` until the code is stable and project checks pass.
- When complete, run `clew done <id>` and commit the code changes with the `.clew/` changes.
- Create new work with `clew new "Short title"`; pass a Markdown body on stdin for detailed Increments.
```

#### Common commands:

| Need                | Command                            |
| ------------------- | ---------------------------------- |
| See active work     | `clew list`                        |
| Filter by status    | `clew list --status todo`          |
| Filter by tag       | `clew list --tag bug`              |
| Read an Increment   | `clew show 0024`                   |
| Create an Increment | `clew new "Short title"`           |
| Create with body    | `clew new "Short title" < body.md` |
| Add tags            | `clew tag 0024 bug p0`             |
| Remove tags         | `clew untag 0024 p0`               |
| Start work          | `clew start 0024`                  |
| Finish work         | `clew done 0024`                   |

#### Creating detailed Increments

```bash
clew new "Add OAuth routes" --tag auth <<'EOF'
## Goal

Add route handlers for OAuth login.

## Tasks

- [ ] Add route definitions
- [ ] Add request validation
- [ ] Add tests
EOF
```

Clew writes frontmatter itself; stdin is body content only.

## Tips for humans

- You can open any `.clew/` folder with Obsidian as a convenient UI.
- `!clew <command>` in Pi works great and gets your agent the context if you already know what you want it to see.
- Clew fits quite nicely into [Matt Pocock's](https://github.com/mattpocock/skills) workflows/skills as a local GH issues alternative.
- Clew is under active development; feel free to submit an issue or reach out if you run into functionality gaps for your workflow.

## Developing Clew itself

Use the installed `clew` command for normal workflow, including while developing Clew itself. Use this repository's `./clew` runner only when intentionally testing the promoted local development build from `scripts/promote-clew`.

After a successful Clew increment, promote the current source build with:

```bash
scripts/promote-clew
```

That script runs `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, builds the release binary, and copies it to `.clew/bin/clew`. The promoted binary is local-only and ignored by git.

## Usage highlights

```bash
# Show all the things!
for id in 0002 0003 0005 0008 0010 0012 0014 0018 0019 0021 0022 0023 0024 0025 0026 0027; do echo "--- $id ---"; clew show $id; done
```

```bash
# Reseeding path.md from clew list
clew list > .clew/path.md && clew lint && echo "---" && cat .clew/path.md
```
