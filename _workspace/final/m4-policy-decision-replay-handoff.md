# M4 Policy-Decision Replay Handoff

## Delivered

- Added the versioned `m4-scripted-agent-replay-v1` record for actor-visible
  scripted-agent decisions.
- Added expected versus declared-anomalous disposition labels and deterministic
  replay with bounded `DecisionMismatch` rejection.
- Preserved default and seeded policy selection, optional seed provenance, and
  all host/lane authority boundaries.
- Synchronized canonical/workspace docs, CHANGELOG, ARCHITECTURE, and
  LESSONS.md.

## Verification

The focused agent suite contains 15 tests. The full suite contains 169 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test,
plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Domain QA disposition

Pass for the bounded library-only decision-replay slice. Replay uses only
actor-visible observations and explicit seed inputs; it does not validate,
transition, commit, persist, or mutate authoritative history.

## Open boundaries

Declared-anomalous is an inspection label, not a degenerate-policy or outcome
claim. Scenario-level replay, durable persistence, population sampling,
strategic-quality analysis, and human behavioral realism remain open.
