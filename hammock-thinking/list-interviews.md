# list functionality

Imagine you are a software development agent working on software project.

You have access to an CLI tool called `clew` to help you track/manage work for the project.

Clew is a local, git-native, markdown-backed project tracker for hobby projects, used primarily by coding agents (1–2 sessions at a time). Items have a status lifecycle backlog → todo → in_progress → done, where backlog means "captured, possibly underspecified" and todo means "ready enough that an agent can pick it up without asking questions."

What would you most likely reach for / how would you try to call clew to see a list of all in progress items (todo & in_progress)

# Responses

## Gemini 3.1 Pro

As an autonomous development agent dropping into a repository, I rely on standard CLI conventions and predictable patterns. If I know `clew` is managing the project but don't have the `--help` docs memorized yet, here is exactly what I would intuitively reach for, in order of preference:

### 1. The Standard Filter (Most Likely First Try)

I would default to a standard `list` command with a status filter flag. This is the most universal pattern across tools like `gh`, `jira-cli`, or `task`.

```bash
clew list --status todo,in_progress
# or the shorthand
clew ls -s todo,in_progress
```

### 2. The "Active Work" Alias (What I _Hope_ Exists)

Since you defined a strict boundary between `backlog` (needs refinement) and `todo` (ready for me to work on), the combination of `todo` + `in_progress` represents the **actionable queue**. I would try an alias that assumes this logical grouping:

```bash
clew active
# or
clew list active
```

_Why?_ Because as an agent, my primary loop is "find work -> do work." Having a single command that filters out the noise (done/backlog) is highly efficient for my context window.

### 3. The Query Language Approach

If the tool is built for agents, it might support a slightly more robust query syntax so I can be precise without guessing comma-separated flag rules.

```bash
clew query "status:todo OR status:in_progress"
```

### 4. Agent-Specific Output Formatting

Since I am an LLM processing the output, standard terminal tables can sometimes be token-heavy or tricky to parse if text wraps. If I were invoking this programmatically in a tool-use loop, I would immediately append a format flag:

```bash
clew list --status todo,in_progress --json
# or
clew list active --format markdown
```

_Why?_ A JSON array of objects `[{id: 12, status: "todo", title: "Setup auth"}]` is perfectly reliable for me to read, and raw markdown is equally native to my training data.

### Summary of my initial "probe" sequence:

If I just landed in your terminal right now, I would run:
`clew --help` (just to be safe), but if forced to guess blindly, I'm firing off **`clew ls --status todo,in_progress`**.

## Gemma4

The most likely approach would be to use a filtering command, specifying the two required statuses (todo and in_progress) separated by a comma or space, depending on the assumed syntax of the tool.

Since I need to list items and I have a set of desired statuses, I would attempt a command structured like this:

🔍 The Command
clew list --statuses todo,in_progress
⚙️ Rationale (The Agent's Thought Process)
Identify the Core Action: I need to view or list items (clew list or clew show).
Identify the Filtering Mechanism: The tool must support filtering by status. I anticipate a flag like --status or --statuses.
Determine the Syntax: Since the goal is to include both todo and in_progress items, I need a syntax that handles multiple values, which is almost universally done with a comma-separated list in modern CLI design.
Why not just clew status todo? That would only show items only in todo. I must explicitly combine the statuses.
Why not clew list --status=todo AND in_progress? CLI tools rarely support boolean logic like AND in simple filters; they use comma separation or arrays.
This command efficiently tells the tool: "Give me a list, but only include items where the status matches either todo or in_progress."

## Sonnet 4.6

As a development agent wanting to see actionable work, I'd most likely reach for something like:

```bash
clew list --status todo,in_progress
```

or the shorthand if it exists:

```bash
clew list --active
```

**My reasoning:**

As an agent, "in progress" from my perspective really means _"what can I or should I be working on right now"_ — which is both `todo` (ready to pick up) and `in_progress` (already started). A `--active` flag feels like the most agent-natural affordance because it matches that mental model without requiring me to know the exact status names.

If filtering flags didn't exist, I'd fall back to:

```bash
clew list
```

and hope the default output either filters to active items or at least groups/sorts by status so I can visually scan. I'd be less likely to run two separate commands (`clew list --status todo` + `clew list --status in_progress`) since that fragments a single conceptual query.

**What I'd want from the output:**

A compact table or list — item ID, title, status, maybe a priority hint — ordered so `in_progress` floats to the top. Enough to pick my next action without opening individual items.
