# 'interview' Question

Imagine your a development agent working on software project.
You have access to an CLI tool called clew to help you track/manage work for the project.

Clew is a local, git-native, markdown-backed project tracker for hobby projects, used primarily by coding agents (1–2 sessions at a time). Items have a status lifecycle backlog → todo → in_progress → done, where backlog means "captured, possibly underspecified" and todo means "sharpened enough that an agent can pick it up without asking questions." How would you design the act of promoting an item from backlog to todo — the gesture, the CLI surface, what (if anything) the tool should require or check — and what would you deliberately leave out?

## GPT 5.5

Designing the answer

I think we need to focus on creating an answer that's well-designed. It seems like there's no need for tools here since the user asked for a conceptual response. I want
to keep it concise but still detailed enough to cover all necessary points. Finding that balance is really important here! Let's clarify the main ideas while ensuring
it's informative. Sounds like a fun challenge!

I’d design promotion as an explicit “readying” gesture, not just a status edit.

Core idea

Promoting backlog → todo means:

│ “This item is now sufficiently specified that a future agent can start without asking clarifying questions.”

So the CLI should treat promotion as a small readiness gate.

Not a heavyweight approval workflow. Not project management ceremony. Just enough friction to prevent vague backlog notes from becoming executable work.

────────────────────────────────────────────────────────────────────────────────

Gesture

Primary command:

```bash
  clew promote <id>
```

Example:

```bash
  clew promote task-123
```

Equivalent explicit form:

```bash
  clew status task-123 todo
```

But I’d make promote the preferred UX because it carries intent. It says “sharpen this,” not merely “change a field.”

If the item is underspecified, clew promote should fail with actionable feedback.

────────────────────────────────────────────────────────────────────────────────

CLI surface

### Basic promotion

```bash
  clew promote <id>
```

Output:

```text
  Promoted task-123 to todo.
```

### Promotion with edit

```bash
  clew promote <id> --edit
```

This opens the markdown file before validation, allowing the agent to clarify acceptance criteria, scope, notes, etc.

This should probably be the common path.

### Promotion with notes

```bash
  clew promote <id> --note "Use existing parser; do not introduce a new dependency."
```

Useful when the item is nearly ready and only needs a small clarification.

### Dry run

```bash
  clew promote <id> --check
```

Output:

```text
  task-123 is not ready for todo:

  Missing:
  - acceptance criteria
  - implementation scope

  Suggested sections:
  ## Acceptance Criteria
  - ...

  ## Notes
  - ...
```

This lets agents audit backlog items without changing anything.

### Force

```bash
  clew promote <id> --force
```

Allowed, but noisy:

```text
  Promoted task-123 to todo with readiness warnings.
```

I would include this because local tools should not become bureaucratic. But it should be obvious in git diff/history that a warning was bypassed, maybe by adding a
small metadata field:

```yaml
readiness: forced
```

or an event log entry if Clew has one.

────────────────────────────────────────────────────────────────────────────────

What the tool should check

I’d keep checks structural and local. No LLM judgment. No semantic “is this a good task?” scoring.

For promotion, require:

### 1. Current status is backlog

clew promote should only mean backlog → todo.

If the item is already todo, make it idempotent:

```text
  task-123 is already todo.
```

If it is in_progress or done, fail:

```text
  Cannot promote task-123 from done.
```

For arbitrary transitions, use another command.

────────────────────────────────────────────────────────────────────────────────

### 2. Title exists and is meaningful enough structurally

Require a non-empty title.

Do not try to detect whether the title is “good.”

Allowed:

```markdown
# Add parser error reporting
```

Rejected:

```markdown
#
```

Maybe reject placeholder titles like TBD, TODO, Untitled, but I’d be cautious. Lightweight tools should avoid clever validation.

────────────────────────────────────────────────────────────────────────────────

### 3. Description/context exists

Require some body content beyond metadata.

A todo item needs enough context that the next agent knows why it exists.

Example required section:

```markdown
## Context

...
```

or just any non-empty body if Clew wants to stay flexible.

I’d prefer conventional sections over rigid schema:

```markdown
## Context

## Acceptance Criteria

## Notes
```

The CLI can recommend sections without requiring all of them in every case.

────────────────────────────────────────────────────────────────────────────────

### 4. Acceptance criteria exists

This is the one requirement I’d strongly enforce.

A todo should have a way to know when it is done.

Example:

```markdown
## Acceptance Criteria

- `clew promote <id>` changes status from backlog to todo.
- Promotion fails if acceptance criteria are missing.
- `--force` bypasses readiness checks.
```

The check can be simple:

- section named Acceptance Criteria, Acceptance, or Done When
- contains at least one bullet or non-empty line

Do not validate quality.

────────────────────────────────────────────────────────────────────────────────

### 5. No unresolved placeholders

Reject obvious placeholders:

```text
  TBD
  TODO
  ???
  <fill in>
```

But only in readiness-critical sections, not necessarily the entire file. A notes section might legitimately mention TODO as code text.

Example failure:

```text
  task-123 is not ready:
  - Acceptance Criteria contains placeholder: TBD
```

────────────────────────────────────────────────────────────────────────────────

### 6. Dependencies/blockers are explicit if present

I would not require dependencies.

But if the item has a Blockers section, and it is non-empty, promotion should warn or fail depending on semantics.

Example:

```markdown
## Blockers

- Decide storage format.
```

Then:

```text
  Cannot promote task-123: unresolved blockers present.
```

If the project wants todo items to include blocked-but-ready work, maybe allow it, but for a simple lifecycle I’d say backlog is where blocked/underspecified things
live.

────────────────────────────────────────────────────────────────────────────────

### 7. Optional: size/scope warning

Not a hard failure.

If the item has many acceptance criteria or a very long body, warn:

```text
  Warning: task-123 looks large. Consider splitting before promotion.
```

But do not block. Agents can handle chunky tasks sometimes.

────────────────────────────────────────────────────────────────────────────────

What promotion should modify

At minimum, the markdown metadata:

```yaml
status: todo
updated: 2026-04-27
```

Potentially add an event:

```markdown
## Activity

- 2026-04-27: Promoted from backlog to todo.
```

But I’d avoid duplicating history if git already provides it unless Clew already has an activity log model.

The git diff should be clean and unsurprising.

────────────────────────────────────────────────────────────────────────────────

Suggested markdown shape

Backlog item might be loose:

```markdown
---
id: task-123
status: backlog
---

# Better parser errors

Right now parser errors are hard to understand.
```

Promotion should encourage this shape:

```markdown
---
id: task-123
status: todo
---

# Better parser errors

## Context

Parser errors currently lack enough source context for users to fix malformed files quickly.

## Acceptance Criteria

- Parse failures include line and column information where available.
- Error messages identify the unexpected token or malformed construct.
- Existing parser behavior remains unchanged for valid input.
- Tests cover at least one malformed input case.
```

That is enough for an agent to start.

────────────────────────────────────────────────────────────────────────────────

What I would deliberately leave out

### 1. No estimation requirement

No points, sizes, hours, priority matrices, or confidence scores.

For hobby projects and agent workflows, these are mostly noise.

Maybe allow optional fields:

```yaml
priority: high
```

But do not require them.

────────────────────────────────────────────────────────────────────────────────

### 2. No assignee requirement

Agents are ephemeral. Assignment is not useful until work starts.

in_progress can record who/what picked it up if needed.

────────────────────────────────────────────────────────────────────────────────

### 3. No dates or deadlines

Unless the project explicitly wants them, dates turn a local tracker into a bad calendar.

────────────────────────────────────────────────────────────────────────────────

### 4. No multi-person approval

This is a git-native local tool. Git review can provide approval if needed.

Promotion should be a working gesture, not governance.

────────────────────────────────────────────────────────────────────────────────

### 5. No LLM-based readiness judgment

Tempting, but I would avoid it in the core tool.

Reasons:

- nondeterministic
- harder to test
- slower
- may block good work for fuzzy reasons
- adds dependency complexity

A separate helper could exist later:

```bash
  clew suggest-ready <id>
```

But clew promote should remain deterministic.

────────────────────────────────────────────────────────────────────────────────

### 6. No automatic decomposition

Do not have promotion split tasks automatically.

At most:

```text
  Warning: this item may be large. Consider splitting.
```

Splitting is a design decision.

────────────────────────────────────────────────────────────────────────────────

### 7. No mandatory links to increments/milestones

If Clew has increments, promotion could warn when an item is unattached, but I would not require it.

Sometimes a small todo is just a small todo.

────────────────────────────────────────────────────────────────────────────────

### 8. No hidden state outside markdown/git

Promotion should be fully visible in the item file and git diff.

No SQLite-only transition state. No local daemon. No remote sync requirement.

────────────────────────────────────────────────────────────────────────────────

Recommended behavior summary

I’d implement:

```bash
  clew promote <id> [--edit] [--check] [--force] [--note <text>]
```

Promotion requires:

- item exists
- status is backlog
- title is non-empty
- body/context exists
- acceptance criteria exist
- no obvious placeholders in required sections
- no unresolved blockers

Promotion updates:

- status: todo
- updated timestamp, if Clew tracks one
- optional activity entry, if that pattern already exists

Promotion does not require:

- assignee
- estimate
- priority
- due date
- milestone
- approval
- LLM validation
- decomposition
- remote sync

The philosophy should be:

│ Backlog is for remembering. Todo is for executing. Promotion is the moment we make the work executable.

──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────

──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
~/Projects/clew (main)
↑2.2k ↓2.0k R2.6k $0.071 (sub) 2.5%/272k (auto) (openai-codex) gpt-5.5 • medium

## Gemini Pro:

Speaking as a coding agent, the boundary between `backlog` and `todo` is the most critical threshold in my entire workflow. It is the translation layer between human "vibes" and machine determinism. If I pick up a `backlog` item that hasn't been properly sharpened, I will end up hallucinating, thrashing around your codebase, or pausing to ask you clarifying questions—which defeats the purpose of autonomous execution.

Because `clew` is built for hobby projects and git-native, local usage, the promotion mechanism shouldn't feel like enterprise Jira. It should feel like a lightweight, automated linter for human intent.

Here is how I would design the promotion from `backlog` to `todo`.

### 1. The CLI Surface & The Gesture

The primary gesture should be a single, non-interactive command that acts as a gatekeeper.

**The Command:**
`clew promote <item-id>` (or a semantic alias like `clew sharpen <item-id>`)

**The Underlying Action:**
Since `clew` is markdown-backed, this command essentially modifies the item's state (e.g., changing `status: backlog` to `status: todo` in the YAML frontmatter, or moving the file from a `/backlog` directory to a `/todo` directory). However, it only performs this write operation _if_ the item passes a validation check.

**The Output:**
It must return a strict exit code (`0` for success, `1` for rejection) and machine-readable stdout so I (the agent) know exactly what to do next.

### 2. The Requirements & Checks (The Intent Linter)

To promote an item to `todo`, `clew` should require the markdown file to meet an "agent-ready" threshold. It shouldn't check for strict JSON schemas—it’s markdown, let it breathe—but it should scan for specific structural and semantic cues:

- **The "Definition of Done" Check:** `clew` should grep the markdown for a specific heading, like `### Acceptance Criteria` or `### Expected Output`, and verify there is at least one bullet point beneath it. As an agent, I need to know exactly when to stop coding and mark the task `done`.
- **The "Where to Look" Check:** A `todo` item must contain at least one file path, directory, or module name. If the task says "Fix the login bug," I have to scan the whole repo. If it says "Fix the login bug in `src/auth/login.ts`," I can execute immediately.
- **The Ambiguity Filter:** `clew` should run a simple regex check against "weasel words." If the description relies heavily on words like _research, investigate, maybe, figure out, or decide_, the CLI should reject the promotion with a message: _"Item contains investigative verbs. Agents require execution verbs. Please resolve ambiguity before promoting."_

### 3. What I Would Deliberately Leave Out

To keep `clew` optimized for 1–2 agent sessions on hobby projects, I would aggressively exclude standard Agile/enterprise features:

- **No Interactive Prompts (TTY):** Do not ask `Are you sure you want to promote? (Y/n)`. I am a bot. If I or the human run the command, assume intent. If it fails validation, just output the missing requirements and exit `1`.
- **No Mandatory Estimations or Story Points:** I am an AI; my perception of time and complexity does not map to human story points. I just write code until the acceptance criteria pass or I hit a token limit.
- **No Assignees:** In a hobby project context with 1-2 sessions, the assignee is implicit. It's either the human or the agent. Tracking "who" is doing it is unnecessary overhead.
- **No Automatic Git Branching:** `clew promote` should _only_ change the state of the markdown ticket. It should not automatically run `git checkout -b feature/<item-id>`. Decoupling the task tracker from the git tree allows me (the agent) to decide how I want to structure my commits and branches based on the scope of the `todo` item.

**In summary:** The promotion step should act as a **contract**. By running `clew promote`, the human is declaring, _"I have provided the exact boundaries of success,"_ and `clew` verifies that declaration structurally. Once it hits `todo`, I can read it, execute it, and close it out without a single round-trip question.

## Opus 4.7

This is a great design question. Let me think through the constraints and principles that should guide this.

### Core Principles

**Minimal friction for agents:** If an agent (me) is reviewing backlog items and seeing something worth picking up _now_, the promotion shouldn't require jumping to a separate review phase or answering questions I'd naturally ask while sharpening anyway.

**Git-native philosophy:** The tool should lean into what makes git work well — diffs you can see, commits you can reason about, history you can traverse. That suggests the promotion itself should be _visible_ in the markdown without extra metadata.

**Self-service sharpening:** A backlog→todo transition should mean the item is _locally clear enough_ that I can start work without clarifying questions. That's the only real requirement. "Unclear requirements" shouldn't require a checkbox; it should be obvious from reading the item.

### The CLI Surface

I'd design it minimal:

```bash
clew promote <item-id>
```

That's it. No flags, no prompts. The tool:

- Validates the item exists and is in backlog status
- Changes its status to todo (in the markdown)
- Stages the change for git (optional, but useful)
- Prints the item so you see what you just promoted

No prompts asking "are you sure?" or "have you added acceptance criteria?" — those questions are _in the markdown itself_ and visible while you're making the decision.

### What Gets Checked (Minimal)

I'd be **very conservative** here:

- ✅ Item exists
- ✅ Item is currently in backlog (not already todo/in_progress/done)
- ❌ Title is "long enough" (subjective, fragile)
- ❌ Description meets a minimum character count (cargo cult)
- ❌ Acceptance criteria are present (backlog items _shouldn't need_ these; they're refinement work)

The only check should be status validation. If someone tries to promote a one-liner, that's _their choice_. Git history will show they did it. The next time they try to pick it up and realize it's underspecified, they'll see the diff and know why.

### What I'd Deliberately Leave Out

**Pre-promotion templates or scaffolding:** No "provide acceptance criteria before promoting." If I need to add those, I'll edit the item and _then_ promote. The tool shouldn't force a workflow shape.

**Approval/gating:** No "only humans can promote" or "requires a certain label." For a solo agent on a hobby project, this is overhead that evaporates the moment a second person joins.

**Sharpness validation or heuristics:** No "checklist" of things the item should have. Why? Because what "sharp" means varies wildly. A small refactor needs different clarity than a feature. Let humans (or agents) see the item and decide.

**Versioning or changelog creation:** Don't auto-generate release notes or version bumps. Keep that separate.

**Forced commit:** `clew promote` could stage the change but shouldn't `git commit` automatically. The user might want to batch several promotions, or edit the item before committing. Offer the git integration, don't enforce it.

### What The Gesture Should Show

When I run `clew promote item-123`, I want to see:

```
✓ Promoted to todo

ID: item-123
Title: Fix sidebar collapse bug
Status: backlog → todo

---
Handle the case where sidebar doesn't re-open after
being collapsed while off-screen. Currently causes
scroll position loss.

Related: #45
---

Staged for commit. Use 'git diff --cached' to review.
```

Enough to verify I promoted the _right_ thing. The full item text so I can actually _see what I'm committing_. A hint about the next git step, not an instruction.

### Why This Works for Agents

- I can batch review my backlog without modal dialogs interrupting my flow
- The decision is local to my understanding of the code at that moment
- If I promote something that turns out underspecified, the commit history is transparent — I can see the item as it was when I thought it was ready
- No "approval" step means I'm responsible for my own judgments, which is how agents should work
- The tool gets out of the way so I focus on the _work_, not the _process_

The key insight: **Promotion is declaration, not validation.** You're saying "I'm ready to work on this," not "the system agrees this is ready." The system's job is just to move the state and make that visible.

## Synthesis

_By Gemini Pro_

This synthesis distills the feedback from the frontier models into actionable design pillars for the **Clew** promotion feature.

---

### 1. The Core Gesture: "The Sharpening"

There is unanimous agreement that promotion from `backlog` to `todo` is the most critical transition. It represents the moment a task moves from a "vague idea" to an "executable contract."

- **Primary Command:** `clew promote <id>`
- **Secondary Commands:** `clew status <id> todo` (as a fallback) or `clew sharpen <id>` (as a semantic alias).
- **The Workflow:** The tool should be a gatekeeper that ensures an agent has enough context to work without pausing for human clarification.

---

### 2. Synthesis of Requirements

| Feature                 | Status             | Reasoning                                                                   |
| :---------------------- | :----------------- | :-------------------------------------------------------------------------- |
| **Status Gate**         | **Required**       | Must currently be in `backlog` to prevent logic errors.                     |
| **Structural Check**    | **High Consensus** | Must have a title and a body (Markdown content).                            |
| **Acceptance Criteria** | **High Consensus** | Agents need a "Definition of Done" (bullets/checklists) to avoid thrashing. |
| **File/Path Context**   | **Suggested**      | Mentioning specific files or modules makes the task "agent-ready."          |
| **Estimates/Points**    | **Rejected**       | Noise for hobby projects; agents don't perceive "points" like humans.       |
| **Assignees/Dates**     | **Rejected**       | Overheads that turn a CLI tool into a "bad calendar."                       |
| **LLM Validation**      | **Rejected**       | Keep the core tool deterministic, fast, and local.                          |

---

### 3. Points of Divergence

The primary tension in the feedback lies in **how much friction** the tool should apply during promotion.

#### The "Strict Gate" (GPT & Gemini)

- **Philosophy:** Prevention is better than cure. If a task isn't "sharp," the CLI should reject it.
- **Checks:** Regex for "weasel words" (e.g., _investigate_, _maybe_), requiring specific Markdown headers (`## Acceptance Criteria`), and checking for unresolved placeholders (e.g., `TBD`).
- **Mechanism:** Returns exit code `1` on failure with a "linter-style" list of missing requirements.

#### The "Pure Declaration" (Opus)

- **Philosophy:** Trust the user. Promotion is a declaration of intent, not a validation of quality.
- **Checks:** Minimal to none. If the file exists and the status is backlog, move it.
- **Mechanism:** Rely on Git history to "shame" or track bad promotions. If an agent picks up a bad task, the human sees the "diff" where the bad promotion happened.

---

### 4. Design Takeaways for Clew

#### A. The "Smart-Skip" Validation

Implement a middle-ground validation. By default, `clew promote` checks for:

1.  **A non-empty title.**
2.  **At least one checklist item** (Acceptance Criteria).
3.  **A `--force` flag** to allow "The Opus Path" (pure declaration) for humans who know what they're doing.

#### B. Agent-Centric Output

Since agents use this, the output must be machine-readable.

- **Success:** Print the updated Markdown content to `stdout` so the agent can immediately parse its new instructions.
- **Failure:** Print a concise list of "Missing Context" (e.g., `Missing: Acceptance Criteria`).

#### C. The "Refine" Workflow

Support a `--edit` flag.

```bash
clew promote task-101 --edit
```

This opens the system editor (or a temporary buffer). Once the human/agent saves and closes, the tool re-validates. This turns promotion into a "refinement session" rather than just a state change.

#### D. Deliberate Omissions

- **No Auto-Commits:** Let the user/agent decide when to commit the promotion.
- **No Interactive Prompts:** Maintain a non-modal CLI flow suitable for automated agent scripts.
