Light weight software project management system designed to:

- be open source
- work locally
- CLI first (TUI and GUI might come later, but not sure)
- Coding Agents as the primary user ( humans as a close 2nd)
- minimize context window usage / minimize token usage
- ability to have a backlog that work can be tagged in (by human or agent)
- ideally push as much of the work to deterministic software as possible (custom CLI tools / scripts)
- integrates beautifully with git locally
- aids in handoffs between agent sessions with minimal token usage

**Not about:**

- selling complicated Agile scrum consulting - this tool/system should be highly pragmatic and effective
- maximizing parallel agent sessions, default is likely 1 or two agents at a time

Terms I'm leaning towards, but open to change

- Task: The most atomic unit of work. A single, trackable action or reminder (e.g., "run tests" or "update config").
- Increment: A standalone unit of work containing one or more tasks. When completed, the codebase must be stable, tested, linted, and safely committable.
  - typically agents will be deployed to work on increments
  - after finishing an increment some
- Epic: A larger, complex feature consisting of two or more increments that must be shipped together for the new functionality to work.
- relay: a more ephemeral transition of context between agent sessions

- Statuses: Backlog > Triage > To Do > In progress > Done (open to suggestions here)

Interview me relentlessly about every aspect of this plan until we reach a shared understanding. Walk down each branch of the design tree, resolving dependencies between decisions one-by-one. For each question, provide your recommended answer.
Ask the questions one at a time.
