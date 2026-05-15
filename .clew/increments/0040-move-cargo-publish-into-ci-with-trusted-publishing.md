---
id: 40
status: backlog
tags:
- release
- ci
created_at: 2026-05-15T05:16:33Z
updated_at: 2026-05-15T05:16:33Z
---
## Goal

Remove the local `cargo publish` step from the release flow by publishing from GitHub Actions on tag push using crates.io trusted publishing (OIDC). Eliminates the single most error-prone release step and the dependency on the releaser's laptop state.

## Why this matters

Today's order is: `cargo publish` locally → `git push origin main` → `git push origin vX.Y.Z`. If the publish succeeds but either push fails, we have an immutable crate version on crates.io with no matching tag in the repo. crates.io versions cannot be re-published, only yanked, so the only recovery is a version bump.

Moving the publish into CI makes tag-push the single trigger; everything (crate, GH Release, Homebrew) keys off the same event.

## End-state release flow (target)

After this lands, the human's release job becomes:

1. Bump `Cargo.toml` version and `CHANGELOG.md`.
2. Run `scripts/release-preflight` (#0039).
3. Commit, push `main`, push `vX.Y.Z` tag.

CI does crates.io, GH Release, Homebrew, and announce. Document this end-state in `docs/release.md` as part of this increment.

## Tasks

- [ ] Configure trusted publishing for `clew` on crates.io, scoped to the release workflow on `npatten/Clew`
- [ ] Add a `publish-crates-io` job to `.github/workflows/release.yml`, gated on the tag and on successful build/host jobs
- [ ] Verify a dry run end-to-end on a prerelease tag (e.g. `v0.1.4-rc.1`) before relying on it for a real release
- [ ] Remove the manual `cargo publish` step from `docs/release.md` and replace with the end-state flow above
- [ ] Revoke the existing local crates.io API token after trusted publishing is verified. The token has full publish rights for the account, not just `clew`, so this is a real attack-surface reduction, not just hygiene.

## Notes

Confirm the GitHub OIDC subject claim used by crates.io matches our workflow path. Without this, the publish job will fail authentication. Document any quirks in `docs/release.md`.

Dependency on #0039: the preflight script's `cargo publish --dry-run` step may shrink or change shape once CI owns the publish. Sweep for consistency once both are merged.
