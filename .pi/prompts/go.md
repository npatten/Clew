---
description: Plan, implement, review, and close a Clew increment
argument-hint: "<increment-id>"
---

Run the full Clew increment workflow for increment `$1`.

If `$1` is empty, ask me for the increment ID before doing anything else.

## Authority and constraints

- Parent session owns orchestration, Clew commands, repo edits outside delegated worker runs, quality gates, user approval, `clew done`, and git commits.
- Invoke Clew as `clew` from the repo root for normal project workflow.
- Use `./clew` only when intentionally testing this repository's promoted local development build after `scripts/promote-clew`.
- Use one worker by default.
- Child subagents must receive concrete role-specific tasks. Do not ask child agents to run their own subagent workflows.
- Do not mark the increment done or commit until `scripts/promote-clew` passes in one sweep and I approve.
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

### 2. Start the increment

After scope is clear, run:

```bash
clew start $1
```

### 3. Build planning context

Launch a fresh-context chain with a parallel `context-builder` step. Use distinct output paths under the temporary chain directory, not repo files:

- `clew-context/request-and-scope.md`
- `clew-context/codebase-and-patterns.md`
- `clew-context/validation-and-risks.md`ce

Each builder should inspect the increment text and relevant repo files directly.

Use `researcher` only if external docs, APIs, platform behavior, or ecosystem facts materially affect the increment.

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

Run:

```bash
clew done $1
```

Commit only files changed for this increment. Prefix the commit message with `[#$1]` and include the required `Clankers:` co-author block. Finally run:

```bash
git status --short
```

Confirm the workspace is clean or only contains expected unrelated changes.
