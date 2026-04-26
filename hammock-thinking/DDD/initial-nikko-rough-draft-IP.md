# Nikko Drafts:

_(for future readers: I wrote this out by hand as an example for the clanker, and to make myself start thinking through the interaction patterns)_

## IP 01 [Nikko note: no real opinion on syntax / structure here]

**Actor:** H or A _(Human or Agent)_
**Goal:** Create a new item (Epic or Increment)

[Nikko note: really like the idea of having little mermaid diagrams]

```mermaid
flowchart TD
  [work for later identified] --> [use Clew CLI to add work-item to backlog] --> [some confirmation from Clew] --> prior work resumes
```

[Nikko note: immediate found that there are variants for a given actor / goal]

### Variant 1)

Actor might start with only a rough notion of the work-item
e.g. 'add export to markdown capability'

I think desired outcome would be for this new item to end up in the backlog.
At time of creation actor might not know if the scope yet (could be an Epic, could be simpler Increment)

## #Variant 2)

Actor starts knowing exactly the scope (maybe was clearly identified during current work)

[Nikko note: also found side ideas / notes coming out while writing out the interation path and variants]

### notes:

- maybe this reveals a proto nature of backlog items?
- maybe this means fluidity between epic and increment is valuable?
  - is an epic just an increment that contains other increments (and thus is bigger than a single session of agent work?); not sure, need to consider / explore more.
