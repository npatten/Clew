# dev-tools

Development utilities for the Clew project. Not part of Clew itself.

---

## extract_clew_usage.py

Scans agent session logs for `./clew` invocations and emits a markdown report. The goal is to surface real usage friction — failed commands, workarounds, unexpected flows — without reading entire session logs.

Supports Pi (`~/.pi/agent/sessions/`) and Claude Code (`~/.claude/projects/`) JSONL formats; auto-detected per file.

**Usage:**

```bash
# Pi sessions for this project
python3 dev-tools/extract_clew_usage.py \
  ~/.pi/agent/sessions/--Users-npatten-Projects-clew--/*.jsonl > report.md

# Claude Code sessions
python3 dev-tools/extract_clew_usage.py \
  ~/.claude/projects/-Users-npatten-Projects-clew/*.jsonl > report.md

# Mixed
python3 dev-tools/extract_clew_usage.py \
  ~/.pi/agent/sessions/--Users-npatten-Projects-clew--/*.jsonl \
  ~/.claude/projects/-Users-npatten-Projects-clew/*.jsonl > report.md
```

**Output:** one section per session, one cluster per assistant turn that touched `./clew`. Each cluster shows:
- **Before** — agent intent just before the call (text or thinking block)
- The commands run and their output (truncated)
- **After** — first agent text following the cluster

**Note on "Before" context:** Pi agents emit thinking blocks rather than prose text in most turns, so "Before" reflects internal reasoning. Claude Code agents emit explicit text before tool calls. Different in kind, both useful.
