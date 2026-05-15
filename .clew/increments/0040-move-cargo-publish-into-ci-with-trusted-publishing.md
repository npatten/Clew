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

## Tasks

- [ ] Configure trusted publishing for `clew` on crates.io, scoped to the release workflow on `npatten/Clew`
- [ ] Add a `publish-crates-io` job to `.github/workflows/release.yml`, gated on the tag and on successful build/host jobs
- [ ] Verify a dry run end-to-end on a prerelease tag before relying on it
- [ ] Remove the manual `cargo publish` step from `docs/release.md`
- [ ] Rotate or revoke the local crates.io API token once trusted publishing is verified

## Notes

Confirm the GitHub OIDC subject claim used by crates.io matches our workflow path. Without this, the publish job will fail authentication. Document any quirks in `docs/release.md`.
