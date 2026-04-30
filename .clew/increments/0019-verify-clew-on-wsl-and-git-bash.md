---
id: 19
status: in_progress
tags:
- windows
- ready-for-human
created_at: 2026-04-29T02:53:17Z
updated_at: 2026-04-30T03:32:54Z
---
# Verify Clew on WSL and Git Bash

> **Status: in progress.** Concrete, near-term Windows work for the one known Windows user (who develops in Git Bash). For the larger speculative "native cmd/PowerShell" design thinking, see #0018.

## Context

One serious Clew user develops on Windows and uses Git Bash. WSL2 is the other reasonable Windows option for developers and is effectively Linux from Clew's perspective. Both should "just work" today since the launcher and promote scripts are bash and the Rust core has no obvious unix-only constructs — but no one has actually verified it end-to-end on a Windows host.

This increment is a sanity-check pass: smoke-test the documented core loop, fix anything that breaks, document the supported posture, and add one defensive test motivated by Git Bash's `core.autocrlf=true` default. It is intentionally narrow. Native cmd / PowerShell support is out of scope and lives in #0018.

## Goal

Document and verify that `./clew` and `scripts/promote-clew` work correctly under WSL2 and Git Bash on a real Windows host, with a regression test guarding the most likely Windows-shaped bug (CRLF round-trip).

## Relationship to #0010

This increment does not block macOS/Linux distribution in #0010. It blocks only native Windows / Git Bash release claims and whether #0010 includes `x86_64-pc-windows-msvc` artifacts. If this passes before the first public release tag, Windows artifacts may ship as experimental. If not, #0010 should document WSL and Cargo-based installation as the Windows path for now.

## Scope

### In

- Smoke-test on a Windows machine: `./clew init`, `clew new`, `clew start`, `clew done`, `clew list`, `clew show` under both WSL2 and Git Bash.
- Run `scripts/promote-clew` under Git Bash to confirm `cargo build --release` and the binary copy succeed. `chmod +x` is expected to be a no-op on NTFS — harmless.
- Verify archive transitions (`clew done` / `clew reopen`) move files correctly on NTFS via Git Bash.
- Add one integration test that round-trips a CRLF-on-disk increment through a CLI write (e.g. `clew start`) and confirms parsing succeeds and body content is preserved. Runs on any platform; motivated by Git Bash's `autocrlf=true` default.
- README section: "Development on Windows — use WSL2 or Git Bash. Native cmd / PowerShell is unsupported; see #0018 for the parked design."
- `clew-spec.md` revisions entry noting WSL/Git Bash as the documented Windows path.

### Out

- Native cmd.exe / PowerShell support. Parked in #0018.
- PowerShell ports of `./clew` or `scripts/promote-clew`.
- Distribution / installer story. Belongs to #0010.
- A `windows-latest` CI matrix entry. Useful eventually, but earning its keep depends on doing the #0018 audit first.

## Risks and unknowns

- Git Bash + `assert_fs` tempdirs: unverified. If the integration suite has tempdir teardown issues on Windows, that surfaces here.
- Git's `autocrlf=true` may already be silently working — the CRLF test might pass on first write. Still worth having as a regression guard.
- `chmod +x` on NTFS: expected no-op. If the promoted binary fails to execute, that's a real bug to chase, not a doc fix.

## Tasks

- [ ] [Human] Smoke-test `clew init` / `new` / `start` / `done` / `list` / `show` under WSL2 on the Windows host
- [ ] [Human] Smoke-test the same commands under Git Bash on the Windows host
- [ ] [Human] Run `scripts/promote-clew` under Git Bash; confirm the promoted binary executes
- [ ] [Human] Verify archive `done` and `reopen` move files correctly on NTFS via Git Bash
- [ ] Verify `cargo install --path .` or `cargo install --git <repo>` works from Git Bash
- [x] Audit for Windows-sensitive path, process, executable-bit, and newline assumptions
- [x] Write a CRLF round-trip integration test (write `\r\n` increment to disk, run `clew start`, assert no parse failure and body preserved)
- [ ] Add a "Development on Windows" section to `README.md` documenting WSL2 + Git Bash as supported, native shells as not, with a pointer to #0018
- [ ] Update `clew-spec.md` (revisions entry + a sentence in the implementation section noting the Windows posture)
- [ ] File follow-up increments for any concrete bugs the smoke tests surface
