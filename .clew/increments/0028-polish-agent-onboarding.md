---
id: 28
status: in_progress
tags:
- launch-polish
- docs
created_at: 2026-05-01T01:40:33Z
updated_at: 2026-05-01T01:40:41Z
---
## Goal

Improve the public and generated documentation so a first-time user can make a Clew-enabled repo reliably usable by coding agents before the demo.

## Scope

- Clean up the public `README.md` onboarding story.
- Remove or demote stale concepts that are not part of the current shipped loop, especially Relay and speculative Epic language.
- Use Clew vocabulary consistently: Increment, Task, Path, Archive, Tag.
- Replace user-facing `./clew` examples with installed `clew` examples, except in the explicit self-hosting/development section for this repository.
- Clarify the core agent loop versus broader/advanced commands.
- Keep this docs/onboarding focused; spin CLI behavior changes into separate Increments.

## Acceptance criteria

- [x] Public README tells users to install `clew`, run `clew init`, copy the agent instructions, and commit `.clew/`.
- [ ] Public README and generated `.clew/README.md` tell the same activation story.
- [x] No shipped docs call Clew work items issues/tickets where Increment is meant.

## Progress

- Public `README.md` was cleaned up to use installed `clew` commands, remove stale Relay/Epic core concepts, and foreground the agent-instruction copy step.
- `clew-spec.md` now records first-agent onboarding as a decided init behavior.
