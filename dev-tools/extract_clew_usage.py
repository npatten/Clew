#!/usr/bin/env python3
"""
Extract and summarize clew CLI invocations from Pi and Claude Code session logs.

Usage:
    python3 dev-tools/extract_clew_usage.py ~/.pi/agent/sessions/--Users-npatten-Projects-clew--/*.jsonl
    python3 dev-tools/extract_clew_usage.py session1.jsonl session2.jsonl > report.md
"""

import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

# Match ./clew or clew only as an actual shell command — anchored to command
# boundaries (start-of-string, or after ; & | && || \n), not inside quoted strings.
CLEW_RE = re.compile(
    r'(?:^|[;&|]\s*|\n\s*)\.?/clew\b'
)

MAX_CONTEXT = 160
MAX_RESULT  = 120


def is_clew(cmd: str) -> bool:
    return bool(CLEW_RE.search(cmd))


def trunc(s: str, n: int) -> str:
    s = " ".join(s.split())  # collapse whitespace / newlines
    return s[:n] + "…" if len(s) > n else s


# ---------------------------------------------------------------------------
# Format detection
# ---------------------------------------------------------------------------

def detect_format(lines: list[str]) -> str:
    for line in lines[:6]:
        try:
            rec = json.loads(line)
        except Exception:
            continue
        t = rec.get("type", "")
        if t == "session":
            return "pi"
        if t == "queue-operation":
            return "claude"
    return "unknown"


# ---------------------------------------------------------------------------
# Normalised event stream
#
# Both parsers emit the same event shapes:
#   {"kind": "text",        "role": "assistant", "text": "...", "ts": "..."}
#   {"kind": "tool_call",   "role": "assistant", "id": "...", "cmd": "...", "ts": "..."}
#   {"kind": "tool_result", "id": "...", "output": "...", "ts": "..."}
#   {"kind": "turn_end"}   — emitted after each assistant turn's results arrive
#
# turn_end events break up the cluster detection so that parallel tool calls
# in one assistant turn don't bleed into the next turn's calls.
# ---------------------------------------------------------------------------

def _result_text(content) -> str:
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        parts = []
        for item in content:
            if isinstance(item, dict):
                parts.append(item.get("text", ""))
            elif isinstance(item, str):
                parts.append(item)
        return " ".join(parts)
    return str(content)


def parse_pi(lines: list[str]) -> tuple[dict, str, list[dict]]:
    """
    Pi JSONL structure:
      - type:"session"            — session header
      - type:"model_change"       — model switch
      - type:"message" role:"user"       — human turn
      - type:"message" role:"assistant"  — content: thinking + toolCall blocks
      - type:"message" role:"toolResult" — one per tool call, toolCallId links back
    """
    session_info: dict = {}
    model = "unknown"
    events: list[dict] = []

    # We need to track how many results are expected per assistant turn so we
    # can emit a turn_end after the last result arrives.
    pending_calls: int = 0   # tool calls emitted in the current turn
    received_results: int = 0

    for line in lines:
        try:
            rec = json.loads(line)
        except Exception:
            continue

        t = rec.get("type", "")

        if t == "session":
            session_info = rec

        elif t == "model_change":
            model = rec.get("modelId", model)

        elif t == "message":
            msg = rec.get("message", {})
            role = msg.get("role", "")
            content = msg.get("content", [])
            ts = rec.get("timestamp", "")

            if role == "user":
                # Real user message — reset turn tracking
                pending_calls = 0
                received_results = 0

            elif role == "assistant":
                # Reset result counter for this new assistant turn
                pending_calls = 0
                received_results = 0

                # Extract thinking as context (often the only text in Pi turns)
                thinking_parts = []
                for block in content:
                    if not isinstance(block, dict):
                        continue
                    bt = block.get("type", "")
                    if bt == "thinking":
                        t_text = block.get("thinking", block.get("text", "")).strip()
                        if t_text:
                            thinking_parts.append(t_text)
                    elif bt == "text":
                        text = block.get("text", "").strip()
                        if text:
                            events.append({"kind": "text", "role": "assistant", "text": text, "ts": ts})
                    elif bt == "toolCall":
                        name = block.get("name", "").lower()
                        if name in ("bash", "shell", "execute", "run_bash", "computer"):
                            args = block.get("arguments", {})
                            cmd = args.get("command", args.get("cmd", ""))
                            events.append({"kind": "tool_call", "role": "assistant",
                                           "id": block.get("id", ""), "cmd": cmd, "ts": ts})
                            pending_calls += 1

                # Emit condensed thinking as context only if no explicit text was emitted
                # and there are tool calls (thinking is pre-call intent)
                if thinking_parts and pending_calls > 0:
                    first_thought = thinking_parts[0]
                    events.append({"kind": "text", "role": "assistant",
                                   "text": first_thought, "ts": ts, "source": "thinking"})

            elif role == "toolResult":
                result_content = _result_text(content)
                call_id = msg.get("toolCallId", "")
                events.append({"kind": "tool_result",
                               "id": call_id,
                               "output": result_content,
                               "ts": ts})
                received_results += 1
                # Once all results for this assistant turn are in, signal turn end
                if pending_calls > 0 and received_results >= pending_calls:
                    events.append({"kind": "turn_end"})
                    pending_calls = 0
                    received_results = 0

    return session_info, model, events


def parse_claude(lines: list[str]) -> tuple[dict, str, list[dict]]:
    """
    Claude Code JSONL structure:
      - type:"queue-operation"  — session bookkeeping
      - type:"assistant"        — content: text + tool_use blocks
      - type:"user"             — content: tool_result blocks (after assistant turn)
    """
    session_info: dict = {}
    model = "claude"
    events: list[dict] = []
    pending_calls: int = 0

    for line in lines:
        try:
            rec = json.loads(line)
        except Exception:
            continue

        t = rec.get("type", "")
        ts = rec.get("timestamp", "")

        if t == "assistant":
            msg = rec.get("message", {})
            m = msg.get("model", "")
            if m:
                model = m
            if not session_info:
                session_info = {"id": rec.get("sessionId", ""), "timestamp": ts,
                                "cwd": rec.get("cwd", "")}

            pending_calls = 0
            for block in msg.get("content", []):
                if not isinstance(block, dict):
                    continue
                bt = block.get("type", "")
                if bt == "text":
                    text = block.get("text", "").strip()
                    if text:
                        events.append({"kind": "text", "role": "assistant", "text": text, "ts": ts})
                elif bt == "tool_use" and block.get("name") == "Bash":
                    cmd = block.get("input", {}).get("command", "")
                    events.append({"kind": "tool_call", "role": "assistant",
                                   "id": block.get("id", ""), "cmd": cmd, "ts": ts})
                    pending_calls += 1

        elif t == "user":
            msg = rec.get("message", {})
            result_count = 0
            for block in msg.get("content", []):
                if not isinstance(block, dict):
                    continue
                if block.get("type") == "tool_result":
                    result_content = block.get("content", "")
                    events.append({"kind": "tool_result",
                                   "id": block.get("tool_use_id", ""),
                                   "output": _result_text(result_content),
                                   "ts": ts})
                    result_count += 1
            if result_count > 0:
                events.append({"kind": "turn_end"})
                pending_calls = 0

    return session_info, model, events


# ---------------------------------------------------------------------------
# Cluster extraction
# ---------------------------------------------------------------------------

def extract_clusters(events: list[dict]) -> list[dict]:
    """
    A cluster = an assistant turn (or consecutive turns with no text between)
    that contains at least one clew call.

    turn_end events break the run — they mark the boundary between turns.
    """
    results: dict[str, str] = {}
    for ev in events:
        if ev["kind"] == "tool_result" and ev.get("id"):
            results[ev["id"]] = ev["output"]

    clusters = []
    last_text = ""
    i = 0

    while i < len(events):
        ev = events[i]

        if ev["kind"] == "text" and ev["role"] == "assistant":
            last_text = ev["text"]
            i += 1
            continue

        if ev["kind"] in ("tool_result", "turn_end"):
            i += 1
            continue

        if ev["kind"] == "tool_call":
            # Collect ALL calls up to the next turn_end (= one assistant turn)
            run: list[dict] = []
            while i < len(events) and events[i]["kind"] == "tool_call":
                e = events[i]
                run.append({
                    "cmd": e["cmd"],
                    "output": results.get(e.get("id", ""), ""),
                    "is_clew": is_clew(e["cmd"]),
                    "ts": e["ts"],
                })
                i += 1

            if any(c["is_clew"] for c in run):
                context_after = ""
                for j in range(i, min(i + 12, len(events))):
                    if events[j]["kind"] == "text" and events[j]["role"] == "assistant":
                        src = events[j].get("source", "")
                        if src != "thinking":  # prefer real text for "after"
                            context_after = events[j]["text"]
                            break
                    if events[j]["kind"] == "tool_call":
                        break  # next agent action started, no text interlude

                clusters.append({
                    "before": last_text,
                    "calls": run,
                    "after": context_after,
                    "ts": run[0]["ts"],
                })
            continue

        i += 1

    return clusters


# ---------------------------------------------------------------------------
# Markdown formatting
# ---------------------------------------------------------------------------

def _fmt_ts(ts: str) -> str:
    if not ts:
        return "unknown time"
    try:
        dt = datetime.fromisoformat(ts.replace("Z", "+00:00"))
        return dt.strftime("%Y-%m-%d %H:%M UTC")
    except Exception:
        return ts


def format_session(path: str, session_info: dict, model: str, clusters: list[dict]) -> str:
    sid = (session_info.get("id") or "")[:8] or "unknown"
    ts_str = _fmt_ts(session_info.get("timestamp", ""))
    cwd = session_info.get("cwd", "")
    filename = Path(path).name

    lines = [f"## Session `{sid}` — {ts_str}  ({model})"]
    lines.append(f"_file: `{filename}`_")
    if cwd:
        lines.append(f"_cwd: `{cwd}`_")
    lines.append("")

    if not clusters:
        lines.append("_No clew invocations._\n")
        return "\n".join(lines)

    for idx, cluster in enumerate(clusters, 1):
        first_clew_cmd = next(
            (c["cmd"] for c in cluster["calls"] if c["is_clew"]),
            cluster["calls"][0]["cmd"]
        )
        lines.append(f"### Cluster {idx} — `{trunc(first_clew_cmd, 70)}`")
        lines.append("")

        if cluster["before"]:
            lines.append(f"**Before:** {trunc(cluster['before'], MAX_CONTEXT)}")
            lines.append("")

        for call in cluster["calls"]:
            marker = "" if call["is_clew"] else "_(side)_ "
            output = trunc(call["output"], MAX_RESULT) if call["output"].strip() else "_(no output)_"
            lines.append(f"- {marker}`{call['cmd']}`")
            lines.append(f"  → {output}")

        if cluster["after"]:
            lines.append("")
            lines.append(f"**After:** {trunc(cluster['after'], MAX_CONTEXT)}")

        lines.append("")

    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def process_file(path: str) -> str:
    with open(path, encoding="utf-8", errors="replace") as f:
        lines = f.readlines()

    fmt = detect_format(lines)

    if fmt == "pi":
        session_info, model, events = parse_pi(lines)
    elif fmt == "claude":
        session_info, model, events = parse_claude(lines)
    else:
        return f"## `{Path(path).name}`\n_Unknown format — skipping._\n"

    clusters = extract_clusters(events)
    return format_session(path, session_info, model, clusters)


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: extract_clew_usage.py <session.jsonl> ...", file=sys.stderr)
        sys.exit(1)

    paths = sys.argv[1:]
    print("# Clew Usage Report\n")
    print(f"_Generated {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')} from {len(paths)} session(s)_\n")
    print("---\n")

    for path in paths:
        print(process_file(path))
        print("---\n")


if __name__ == "__main__":
    main()
