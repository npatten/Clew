---
description: Plan, implement, review, and close a Clew increment in it's own git worktree
argument-hint: "<increment-id>"
---

Run the full Clew increment workflow for increment `$1`.

If `$1` is empty, ask me for the increment ID before doing anything else.

## Authority and constraints

- Parent session owns orchestration, Clew commands, repo edits outside delegated worker runs, quality gates, user approval, `clew done`, and git operations.
- The entire increment effort runs in a dedicated git worktree at `../clew-worktrees/$1/` on branch `clew/$1`, branched from `main`. This isolates parallel `/go` runs on different increments.
- After the worktree is created, the parent session and all child subagents operate with `cwd` set to the worktree path. Pass `cwd: ../clew-worktrees/$1` (or absolute equivalent) to every subagent invocation.
- Invoke Clew as `clew` from the worktree root for normal project workflow.
- Use `./clew` only when intentionally testing this repository's promoted local development build after `scripts/promote-clew`.
- Use one worker by default.
- Child subagents must receive concrete role-specific tasks. Do not ask child agents to run their own subagent workflows.
- Do not commit, push, or open a PR until `scripts/promote-clew` passes in one sweep and I approve.
- Cap review/fix loops at 3 unless I approve more.
- Don't blindly trust reviewer feedback, think critically about it and act accordingly in the best interest of the project.
- If a finding requires an unapproved product, scope, architecture, or data-model decision, stop and ask me.

## Workflow

### 1. Load and sharpen the increment

Run:

```bash
clew show $1
```

Review the increment for:

- clear goal and scope;
- acceptance criteria;
- non-goals;
- validation expectations;
- docs/spec impact;
- open decisions.

If material requirements are unclear, ask me clarification questions before starting. If the increment needs light sharpening, propose the edit and ask before making it unless I already authorized backlog cleanup.

### 2. Create the worktree and start the increment

After scope is clear:

1. Confirm `../clew-worktrees/$1/` does not already exist and branch `clew/$1` is not already checked out. If either is true, stop and ask me how to proceed (resume? clean up first?).
2. Create the worktree off `main`:

   ```bash
   git worktree add ../clew-worktrees/$1 -b clew/$1 main
   ```

3. From here on, treat `../clew-worktrees/$1/` as the working root. Run every command and every subagent with `cwd` set to that path.
4. Inside the worktree, run:

   ```bash
   clew start $1
   ```

   The status edit becomes the first change on the `clew/$1` branch.

### 3. Build planning context

Launch a fresh-context chain with a parallel `context-builder` step. Use distinct output paths under the temporary chain directory, not repo files:

- `clew-context/request-and-scope.md`
- `clew-context/codebase-and-patterns.md`
- `clew-context/validation-and-risks.md`ce

Each builder should inspect the increment text and relevant repo files directly.

### 4. Plan implementation

Launch `planner` using the context-builder outputs. Ask it for:

- recommended implementation approach;
- likely files/modules touched;
- test and validation strategy;
- docs/spec updates;
- risks and escalation points;
- compact worker meta-prompt.

### 5. Oracle review

Launch `oracle` to challenge the plan against the increment intent and current context. Ask it to focus on:

- wrong problem framing;
- hidden scope creep;
- missing decisions;
- risky architecture/data-model choices;
- validation gaps;
- whether the worker prompt is safe and complete.

Oracle is advisory. Parent decides what to accept.

### 6. Capture implementation notes in the increment

Synthesize the accepted plan and oracle feedback into compact implementation notes. Update the Clew increment with only durable notes useful to a future reader:

- final approach;
- relevant files or seams;
- validation plan;
- known risks;
- explicit non-goals;
- decisions made during planning.

Do not dump raw subagent output into the increment.

### 7. Implement with one worker

Launch `worker` with a concrete meta-prompt containing:

- increment ID and title;
- clarified requirements;
- accepted implementation notes;
- non-goals;
- expected files/seams;
- validation expectations;
- escalation rules for unapproved decisions.

The worker may edit files. It should run focused validation where practical and summarize changed files and checks.

### 8. Review implementation

After worker returns, inspect the diff locally, then launch parallel fresh-context `reviewer` agents:

1. correctness and regressions;
2. tests and validation;
3. simplicity, maintainability, and unnecessary complexity.

Reviewers should inspect the current diff and relevant files directly. They should report evidence-backed findings with file/line references. They should not edit unless explicitly asked.

### 9. Fix material findings

Synthesize reviewer feedback into:

- blockers/material fixes worth doing now;
- optional improvements to defer;
- feedback to ignore, with reason;
- decisions requiring user input.

If material fixes exist and no user decision is required, launch `worker` to apply only those fixes. Then repeat the fresh-context review step.

Stop the review/fix loop when:

- no material findings remain;
- 2 loops have completed;
- a user decision is required.

### 10. Validate and prepare closure

Check docs and `clew-spec.md` impact. If a meaningful design edit to `clew-spec.md` is needed, update `last_major_update` and `## Revisions` per project instructions.

Run:

```bash
scripts/promote-clew
```

If it fails, fix the root cause and re-run until it passes in a single sweep.

When it passes, summarize:

- implementation completed;
- material reviewer findings fixed or deferred;
- validation result;
- docs/spec updates;
- any remaining risks.

Then ask me for approval before running `clew done $1` or committing.

### 11. After approval only

From inside the worktree (`../clew-worktrees/$1/`):

1. Mark the increment done:

   ```bash
   clew done $1
   ```

2. Commit only files changed for this increment. Prefix the commit message with `[#$1]` and include the required `Clankers:` co-author block.
3. Confirm the worktree is clean:

   ```bash
   git status --short
   ```

4. Push the branch and open a PR for me to review:

   ```bash
   git push -u origin clew/$1
   gh pr create --fill --base main --head clew/$1
   ```

5. Print the worktree cleanup hint (do not run it):

   ```
   After the PR is merged, clean up with:
     git worktree remove ../clew-worktrees/$1
     git branch -d clew/$1
   ```

Leave merging the PR and removing the worktree to me.
