---
id: 18
status: backlog
tags:
- windows
- needs-info
created_at: 2026-04-29T02:52:09Z
updated_at: 2026-04-30T03:32:54Z
---
# Native Windows support (speculative)

> **Status: backlog, speculative.** Parked design thinking. Gated on real demand for native cmd/PowerShell users. Sibling: see the WSL + Git Bash verification increment for the concrete near-term work.

## Context

Clew currently targets macOS and Linux. The one known Windows user develops in Git Bash, so the immediate need is covered by the WSL/Git Bash verification increment. This increment stashes the fuller design thinking for a future "first-class native Windows" push so it isn't lost.

Distribution is **out of scope** — that lives in #0010 (likely via `dist` / cargo-dist, which already produces Windows binaries from CI). This increment is about Clew's own behavior on native Windows, not how the binary gets there.

Dev-loop scripts (`./clew`, `scripts/promote-clew`) are also out of scope: bash-only, intended for contributors, who can use WSL or Git Bash. End users get the released `.exe` and never touch these.

## Goal

Clew's Rust core runs cleanly under native cmd.exe / PowerShell, with CI proving it.

## Audit: Windows-portability concerns in the Rust core

Estimated total: ~50–80 lines of source, ~6 tests, plus a CI matrix entry. Most items are small and one-time.

### 1. Path separator in stdout output

`Path::display()` emits `\` on Windows, so the canonical result line becomes `#0042 .clew\increments\0042-foo.md`. Clew's stdout is a documented machine contract (see `clew-spec.md` "CLI output contract"); agents and shell pipelines should not have to branch on platform.

**Fix:** normalize `repo_relative_path` output to forward slashes in `result_line` (`src/commands.rs:25`). Forward slashes work in every Windows API since NT, in PowerShell, and in `cd` / `cat` / git. Backslashes work in fewer places.

Roughly a 3-line change plus a test. Document the always-`/` behavior in the stdout-contract section of `clew-spec.md`.

### 2. Reserved filenames

Windows reserves `CON`, `PRN`, `AUX`, `NUL`, `COM1`–`COM9`, `LPT1`–`LPT9`, plus trailing-dot and trailing-space names. The `slug` crate happily produces any of these from titles like "Con" or "Aux setup".

**Fix:** in slug generation, after the existing empty-string fallback, check the result against a denylist. On hit, treat it like the empty case — error and prompt for a more specific title. ~15 lines + a parameterized test covering each reserved name.

### 3. Case-insensitive filesystem

NTFS is case-insensitive by default. The `slug` crate already lowercases, so deterministic generation is fine. The risk is hand-renamed files: a user `git mv`-ing `0042-foo.md` to `0042-Foo.md` is a no-op on NTFS and Clew's directory scan still finds it.

**Fix:** likely none in code. Add one integration test that creates two files differing only in slug case to confirm slug-collision detection still fires. Document the behavior.

### 4. Line endings (CRLF round-trip)

Git's Windows default is `core.autocrlf=true`, which rewrites LF → CRLF on checkout. If Clew reads-then-writes a file, our string handling produces LF, and git re-converts on commit. Round-trip should be clean, but worth verifying.

**Fix:** one integration test that writes a file with `\r\n` line endings, runs a CLI write that touches it (e.g. `clew start`), and confirms parsing didn't fail and content is preserved. No source changes anticipated; `serde_yaml` and our line splitting already tolerate `\r\n`.

### 5. Console encoding

Modern Windows Terminal and PowerShell 7 default to UTF-8. Legacy `cmd.exe` on older Windows uses CP-437/1252, which mangles non-ASCII output. Clew's own emitted strings are ASCII; the only non-ASCII bytes flowing through stdout are user-typed body content (titles, frontmatter values), which we round-trip as bytes.

**Fix:** likely none. One CI smoke test under `cmd.exe` that runs `clew show` on an increment with non-ASCII body content and confirms the bytes round-trip cleanly.

### 6. Cross-volume `std::fs::rename`

Non-issue. Archive moves are within `.clew/`, same volume by definition. Documented here only so a future reader doesn't reopen the question.

### 7. Path-length 260 limit

Non-issue. Slugs cap at 50 chars; full repo-relative paths stay well under 260. Long-path support (`\\?\` prefix) not needed.

### 8. TTY detection

`std::io::IsTerminal` works on Windows. No change needed.

### 9. Editor resolution

The `directories` crate already returns the platform-correct config path (`%APPDATA%\clew\config.toml` on Windows). The PATH-scan editor list (`code`, `cursor`, `nvim`, `vim`, `nano`, `helix`) is already mostly Windows-friendly; consider adding `notepad` as a last-resort fallback. `$VISUAL` / `$EDITOR` env vars are less common on Windows but still respected when set.

**Fix:** add `notepad` to the PATH-scan list. Trivial.

## CI

Add `windows-latest` to the GitHub Actions matrix once the core fixes land. Expected to be ~10 lines of YAML. Real cost is ongoing: keeping a third platform's CI green when you don't develop on it. One flaky `assert_fs` tempdir cleanup on Windows can eat an afternoon, and `cargo test` on Windows runners is slower and historically flakier.

## Non-goals

- Porting `./clew` and `scripts/promote-clew` to PowerShell. Bash-only dev tooling stays bash-only; contributors on Windows use WSL or Git Bash.
- Distribution channels (winget, Scoop, Chocolatey, signed `.exe`). Belongs to #0010.
- Replacing or supplementing the bash launcher with a `.cmd` / `.ps1` shim in this repo.

## Decision triggers (revisit when)

- A second non-contributor user reports they want native Windows without WSL or Git Bash.
- `dist` ships and the released `.exe` shows up in real Windows installs that hit a portability bug we did not predict.
- A Windows-native contributor wants to hack on Clew and finds WSL too heavy.

Until one of those fires, this stays in backlog.

## Tasks (when picked up)

- [ ] Implement path-separator normalization in `result_line` + test
- [ ] Add reserved-filename denylist to slug generation + parameterized test
- [ ] Add case-insensitive slug-collision integration test
- [ ] Add CRLF round-trip integration test
- [ ] Add `notepad` to editor PATH-scan list
- [ ] Add `windows-latest` to CI matrix
- [ ] Update `clew-spec.md` stdout-contract section to specify forward slashes always
- [ ] Update `README.md` with native Windows support statement
