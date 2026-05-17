---
id: 43
status: backlog
created_at: 2026-05-16T21:23:15Z
updated_at: 2026-05-16T21:23:15Z
---
## Goal
Add a lightweight update-availability notice so users can learn when a newer Clew release exists.

## Rationale
Users should not need to manually check for new releases, but Clew should avoid surprising network calls or noisy prompts during normal workflow.

## Initial recommendation
- Prefer a best-effort check against the published release source, likely crates.io if that remains the distribution channel.
- Cache the last check result locally and rate-limit checks, for example once per day or once per week.
- Never block the command, fail the command, or print update errors by default.
- Show a short notice only when a newer stable version is available.
- Include the installed version, latest version, and the recommended update command.
- Provide a quiet/disable path if the check becomes annoying, either config-based or environment-based.

## Questions
- Where should update-check state live: project `.clew/`, user config/cache, or platform cache directory?
- Should checks run only for selected commands, or at most once after any user-facing command?
- What should the recommended update command be for each supported install path?
- Do we need a manual `clew update-check` or `clew version --check` command?

## Acceptance criteria
- Clew has a documented, non-obnoxious update notification policy.
- Update checks are cached/rate-limited and best-effort.
- Normal commands remain fast and reliable if the network or registry is unavailable.
- Tests cover newer-version, current-version, stale-cache, and failed-check behavior.
