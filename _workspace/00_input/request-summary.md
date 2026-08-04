# Request Summary

## Requested Outcome

Complete the remaining implementation portion of M0 by adding clean-checkout
CI and dependency-free repository currentness checks, then use the hosted run as
the evidence gate before promoting M0 and selecting the first bounded M1 slice.

## Roadmap Milestone

M0 — Governed repository baseline (active until hosted evidence passes).

## Current Evidence

- Policy, authority, terminology, package, toolchain, lockfile, compatibility,
  and dependency-policy slices are merged to `main`.
- The Rust placeholder is pinned to exact `1.96.0`; the package is now version
  `0.1.1` because this slice adds executable repository tooling, and it has no
  third-party dependencies.
- M0's remaining checklist item is CI formatting, lint, test, documentation-link,
  and currentness checks.

## In Scope

- Add a pinned GitHub Actions workflow for clean-checkout Rust verification.
- Add a dependency-free Python checker for local Markdown links, canonical
  milestone/spec currentness, pinned package metadata, lockfile identity,
  images/reference links/outside-root paths, and the current no-dependency
  policy guard.
- Add focused standard-library tests for link parsing and stale milestone state.
- Expose the new local check in README and reconcile affected architecture,
  specification, dependency-policy, and changelog state.
- After hosted checks pass, promote M0 to `Complete` and move M0 to `SPEC.md`
  `Past`, identifying one bounded M1 transition/replay fixture slice.

## Non-Goals

- No simulation mechanics, schema serializer, replay engine, CLI, MCP, database,
  or model-provider dependency.
- No full advisory/license scanner is claimed while the dependency graph is
  empty; the CI guard blocks future dependencies until that scanner is adopted
  or a machine-readable defer record is recorded.
- No human, legal, accessibility, enjoyment, or research-validity claim.

## Project Boundaries Touched

- Clean-checkout reproducibility and repository currentness.
- CI/adaptor edge only; the deterministic core remains unchanged.
- M0 promotion bookkeeping and the selected M1 entry contract.

## Source Files

- `.github/workflows/ci.yml`
- `scripts/check_repository.py`
- `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `docs/DEPENDENCY_POLICY.md`

## Expected Outputs

- Pinned CI workflow with contents-read permissions.
- Local currentness/link/package-policy checker.
- Focused checker parser/currentness tests.
- Updated M0 verification and, after hosted evidence, M0 completion state plus
  one bounded M1 slice in `SPEC.md`.

## Verification

- `python3 scripts/check_repository.py` passes.
- `python3 -m unittest discover -s scripts -p 'test_*.py'` passes.
- Exact pinned `cargo +1.96.0` metadata, format, lint, and test commands pass.
- Local Markdown links and `git diff --check` pass.
- GitHub Actions runs the same checks from a clean Ubuntu checkout before merge.

## Evidence Limits and Open Questions

- The checker validates repository contracts and current package state; it does
  not validate future simulation behavior or human experience.
- The no-dependency guard is not an advisory/license scanner. The first future
  dependency requires an approved scanner or a complete machine-readable defer
  record with owner, rationale, security/license status, and future expiry.
- M1 implementation begins only after M0 hosted evidence and promotion are
  recorded.
