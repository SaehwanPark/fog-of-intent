# Request Summary

## Requested Outcome

Advance the next dependency-complete slice of the governed repository baseline
so contributors can understand the project's source-licensing posture, conduct
expectations, unofficial fan-project boundary, original-setting fallback, and
core design vocabulary before implementation begins.

## Roadmap Milestone

M0 — Governed repository baseline (active).

## Current Evidence

- `origin/main` contains the merged repository foundation and harness.
- The executable remains the Rust 2024 placeholder binary.
- M0 policy, distribution, design-principle, and terminology checklist items are
  still incomplete in `ROADMAP.md` and `SPEC.md`.

## In Scope

- Add an explicit source license.
- Add contribution and conduct policies.
- Add an unofficial/noncommercial fan-project and original-setting fallback
  notice with a conservative distribution boundary.
- Add `DESIGN_PRINCIPLES.md` as a concise stable index.
- Add an authoritative terminology reference for the M0 vocabulary.
- Add the first ADR for host-owned deterministic authority, resolved inputs, and
  adapter boundaries.
- Reconcile affected roadmap, specification, architecture, and changelog state.

## Non-Goals

- No simulation mechanics, CLI, MCP, persistence, GUI, or new dependency.
- No claim of legal clearance, permission from third parties, public-release
  readiness, or human-experience evidence.
- No selection of the final M1 crate/workspace layout or CI implementation.

## Project Boundaries Touched

- Contributor and distribution policy.
- Terminology and authority boundaries used by future domain code.
- Documentation-only project-state records.

## Source Files

- `README.md`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `docs/project-proposal.md`
- `docs/harness/fog-of-intent/team-spec.md`

## Expected Outputs

- `LICENSE`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `NOTICE.md`
- `DESIGN_PRINCIPLES.md`
- `docs/TERMINOLOGY.md`
- `docs/adr/0001-authoritative-transition-boundary.md`
- Updated canonical project-state documents.

## Verification

- All new and changed local Markdown links resolve by inspection.
- Policy documents explicitly state their scope and evidence limits.
- The ADR names one authoritative transition owner and keeps adapters from
  owning simulation truth.
- `git diff --check` and the existing Rust formatting, lint, and test commands
  pass without claiming new runtime capability.

## Evidence Limits and Open Questions

- The selected license and notices are repository policy, not legal advice or a
  determination of third-party rights.
- The fan-project boundary remains conservative until a later release review.
- M1 still needs an implementation-backed decision on package layout,
  dependencies, schema compatibility, and toolchain pinning.
