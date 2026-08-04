# Request Summary

## Requested Outcome

Use the passing hosted CI run to complete M0 bookkeeping, then select the first
bounded M1 deterministic-kernel fixture so implementation can continue without
silently broadening scope.

## Roadmap Milestone

M0 — Governed repository baseline promoted to `Complete`; M1 — Deterministic
Simulation Kernel is now the active milestone.

## Current Evidence

- PR #4 hosted GitHub Actions `verify` passed from a clean Ubuntu checkout.
- The repository checker, seven Python tests, locked Rust metadata, formatting,
  clippy, and Rust tests passed locally and in the hosted workflow.
- M0 policy, package, compatibility, dependency-policy, and CI slices are
  merged or ready to merge as one reviewed PR.

## In Scope

- Mark M0 complete in `ROADMAP.md` and move its verified work to `SPEC.md` Past.
- Set the current milestone to M1 and make exactly one bounded M1 slice active.
- Define the smallest typed transition/history/hash/replay fixture contract:
  stable ID, bounded resource, immutable world state, command validation,
  explicit resolved inputs, event/effect output, and per-transition replay hash.
- Reconcile README, architecture, changelog, and current handoff state.

## Non-Goals

- No M1 implementation code in this bookkeeping handoff.
- No lane mechanics, CLI, MCP, serializer, database, agent ecology, or GUI.
- No human, legal, accessibility, enjoyment, or research-validity claim.

## Project Boundaries Touched

- Evidence-gated milestone promotion.
- Active spec contract for the first deterministic kernel slice.
- Currentness checker invariants: exactly one active roadmap phase/spec entry and
  matching README current-milestone state.

## Source Files

- `ROADMAP.md`, `SPEC.md`, `README.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/03_domain-qa.md`
- Existing CI and checker files.

## Expected Outputs

- M0 `Complete` state with hosted verification recorded.
- M1 active bounded-slice specification and roadmap framing.
- Consistent currentness-checker state across canonical documents.

## Verification

- `python3 scripts/check_repository.py` passes against the promoted M1 state.
- Local Markdown links, seven checker tests, locked metadata, format, clippy,
  Rust tests, and `git diff --check` pass.
- The existing hosted PR workflow reruns successfully after the promotion
  commit.

## Evidence Limits and Open Questions

- M0 completion establishes repository governance and CI evidence only; it does
  not establish simulation behavior or release readiness.
- M1 is selected, not implemented. Its transition and replay claims remain
  acceptance criteria until code and tests exist.
- The first M1 implementation must preserve the host-owned authority ADR and
  package/compatibility policies already merged to `main`.
