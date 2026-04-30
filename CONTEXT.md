# Clew

Clew is a lightweight, local, git-native work tracker for small projects and the agents working on them. This context defines Clew's domain language; `clew-spec.md` remains the full living design source.

## Language

**Clew**:
A CLI-first project management system where work is stored as committed markdown in the repository.
_Avoid_: GitHub Issues, Jira, external issue tracker

**Task**:
The smallest unit of work, represented as a markdown checkbox inside an **Increment**.
_Avoid_: sub-issue, ticket, story

**Increment**:
A standalone unit of work that should leave the codebase stable, tested, and committable when complete.
_Avoid_: issue, ticket, PR, sprint item

**Path**:
The hand-curated priority order across active **Increments**.
_Avoid_: roadmap, kanban board, sprint backlog

**Archive**:
The resting place for completed or abandoned **Increments**.
_Avoid_: closed issues, trash, history folder

**Frontmatter**:
The YAML metadata block at the top of an **Increment** file.
_Avoid_: database row, issue metadata

**Status**:
The lifecycle state of an **Increment**.
_Avoid_: label, tag, workflow column

**Blocked reason**:
A frontmatter field explaining why an **Increment** cannot currently proceed.
_Avoid_: blocked status, dependency ticket

**Tag**:
A free-form frontmatter label used for filtering and lightweight classification.
_Avoid_: status, state

**Slug**:
The human-readable filename segment after the numeric **Increment** ID.
_Avoid_: title, identifier

**Reopen**:
The act of moving an archived **Increment** back into active work.
_Avoid_: unclose, restore issue

**Abandon**:
The act of explicitly dropping an **Increment** with a durable reason.
_Avoid_: delete, close as invalid

## Relationships

- A **Clew** project contains many **Increments**.
- An **Increment** contains zero or more **Tasks**.
- An **Increment** has exactly one **Status**.
- An **Increment** may have zero or more **Tags**.
- An **Increment** may have a **Blocked reason** regardless of its **Status**.
- The **Path** orders active **Increments** by priority.
- The **Archive** contains **Increments** whose **Status** is `done` or `abandoned`.
- A **Slug** belongs to an **Increment** filename, while the numeric ID is the stable reference.

## Example dialogue

> **Dev:** "Should we create a GitHub issue for this feature?"
> **Domain expert:** "No — create a Clew **Increment**. If the work needs smaller steps, put them in the **Increment** as **Tasks**."
>
> **Dev:** "The work is waiting on a design answer. Should I change the **Status**?"
> **Domain expert:** "No — keep the lifecycle **Status** as-is and add a **Blocked reason**."

## Flagged ambiguities

- "Issue" usually means a Clew **Increment** in this repo, not GitHub Issues.
- "Label" from external tracker workflows usually maps to a Clew **Tag**, not a **Status**.
- "Closed" should be made precise as either **done** or **abandoned**.
