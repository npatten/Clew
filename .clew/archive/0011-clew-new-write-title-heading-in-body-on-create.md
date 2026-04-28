---
id: 11
status: done
created_at: 2026-04-28T03:43:06Z
updated_at: 2026-04-28T05:12:18Z
---
## Goal

When `clew new "<Title>"` creates an increment file, auto-populate the body with `# <Title>\n\n` as the first line.

## Why

`clew list` derives and displays titles from the filename slug. `clew show` dumps raw markdown — if the body is empty (no `# Title` line), there's no visible title. A freshly-created increment via `clew new` looks inconsistent: listed with a title, shown without one.

Adding the heading on create also gives agents a natural fill-in template rather than a blank file.

## Acceptance criteria

- `clew new "My Feature"` → file body starts with `# My Feature\n\n`
- `clew show <id>` on a brand-new increment displays the title
- If stdin body is provided (non-TTY), the heading is NOT prepended (stdin IS the body)
- Snapshot tests updated to reflect new template
