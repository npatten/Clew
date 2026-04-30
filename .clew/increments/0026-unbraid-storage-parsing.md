---
id: 26
status: backlog
created_at: 2026-04-30T02:30:12Z
updated_at: 2026-04-30T02:30:12Z
---
## Goal
Keep storage focused on filesystem facts and move parse-all behaviour to a domain/read module.

## Architectural friction
`src/storage/fs.rs::scan_with_frontmatter` crosses the storage seam by importing frontmatter parsing and choosing parse error policy. `list` and `lint` depend on that mixed interface.

## Desired benefit
Clarify the storage seam, improve locality for frontmatter policy, and keep storage tests about I/O.
