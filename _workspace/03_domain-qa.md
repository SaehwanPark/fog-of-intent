# Domain QA

## Status

pass

This QA covers the one-window M2 scenario-goal and terminal-objective
projection over ordinary and allied-coordinated records. It does not promote
the complete M2 scenario and does not validate a playable host, human
experience, accessibility, trust, legal clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-coordination-request-summary.md`,
  `_workspace/01-simulation-design-m2-coordination.md`,
  `_workspace/03-domain-qa-m2-coordination.md`, and
  `_workspace/final/m2-coordination-handoff.md` as immutable prior-slice
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `docs/harness/fog-of-intent/team-spec.md`, `docs/TERMINOLOGY.md`, and
  `docs/adr/0001-authoritative-transition-boundary.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

The implementation adds exactly one scenario goal,
`HoldLaneSpaceThroughWindow`, and evaluates it after an existing one-window
record is committed. It does not add a second window, new lane mechanics, a
state field, transition event/effect, objective framework, or playable-host
surface.

The host derives objective inputs from committed lane/coordination facts.
`transition_lane` remains the authority for state, outcome, events/effects, and
hash. Objective evaluation is pure and cannot feed back into the transition.
Ordinary records retain `NotApplicable` coordination; coordinated records
retain the exact committed disposition.

## Information and Attribution Findings

The evaluator receives result facts, intent, coordination disposition, and
explicit execution trace. It does not receive opponent truth, jungle truth,
proposal scores, policy internals, source receipts, or a snapshot. The
privileged input identity carries replay/hash provenance, while the visible
`ObjectiveReport` omits source hashes and private receipts.

`SpaceHeld` and `SurvivedBeat` are classified as committed criteria. The
achieved/partial/missed result is a diagnostic disposition, not an optimality,
balance, win-rate, trust, or human-value judgment. Coordination and execution
remain separately attributable.

## Determinism and Replay Findings

The evaluator is deterministic for identical typed inputs and accepts only the
two versioned source replay identities. `ObjectiveInputIdentity` hashes the
canonical committed facts. `ObjectiveReviewRecord` stores source replay and
record identity, inputs, and review; verification reconstructs expected facts
and review and rejects altered hashes, results, coordination, traces, goal, or
review data. Existing ordinary history, bounded branch, and coordinated
history replay remain valid.

## Required Fixes

None for the declared one-window objective slice.

## Residual Risks

- Objective reviews are in-memory only; portable serialization and migration
  remain deferred.
- Partial achievement is a typed evaluation case and is not necessarily
  reachable from every current mechanical input combination.
- Multiple windows, pacing, recall, gank response, communication, richer
  resources, and full debrief/presentation remain unimplemented.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 40 tests passed: 19 M1 and 21 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
