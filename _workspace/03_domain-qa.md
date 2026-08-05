# Domain QA

## Status

pass

This QA covers the bounded two-window M2 scenario wrapper over existing lane
transition records. It does not promote the complete M2 scenario and does not
validate variable pacing, recall, gank response, communication, a playable
host, human experience, accessibility, trust, legal clearance, or research
validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-strategy-fixtures-request-summary.md`,
  `_workspace/01-simulation-design-m2-strategy-fixtures.md`,
  `_workspace/03-domain-qa-m2-strategy-fixtures.md`, and
  `_workspace/final/m2-strategy-fixtures-handoff.md` as immutable prior-slice
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

`LaneScenarioHistory` composes exactly two ordinary one-window records. It
does not change `transition_lane`, `LaneSnapshot::hash()`, old replay IDs,
branch behavior, or existing coordination/objective/fixture contracts. The
wrapper owns sequence order, start-state evidence, and one deterministic
reopen boundary; a third append is rejected.

`reopen_lane_window` accepts only an opaque committed transition result and
checks its state-hash/outcome consistency before preserving player, opponent,
wave, hidden threat, ruleset, and advanced turn while clearing only
phase/terminal-window status. The first resolved result remains in the stored
record; the second resolved result is the scenario terminal state.

## Information and Replay Findings

The second window uses the existing actor-valid player observation and command
validation. No hidden truth or prior private record is projected to the actor.
Replay reconstructs both observations and transitions, compares exact start
states and complete records, reconstructs the reopened state, and compares the
terminal state. Invalid reopen, third-window, and tampered reopen-state cases
fail. Existing M1/M2 window, branch, coordination, objective, and strategy
fixtures remain passing.

## Required Fixes

None for the declared bounded two-window slice.

## Residual Risks

- Scenario histories are in-memory only; portable serialization remains
  deferred.
- The wrapper is fixed at two one-beat windows and does not yet compose allied
  coordination across windows.
- Variable pacing, recall, gank response, richer resources, communication, and
  full debrief/presentation remain unimplemented.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 43 tests passed: 19 M1 and 24 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
