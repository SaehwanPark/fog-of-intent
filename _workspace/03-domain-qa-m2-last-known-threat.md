# Domain QA

## Status

pass

This QA covers the bounded player-visible last-known jungle-threat report over
the existing one-window lane authority. It does not promote complete vision,
belief updates, gank response, variable pacing, a playable host, human
experience, accessibility, trust, legal clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-recall-request-summary.md`,
  `_workspace/01-simulation-design-m2-recall.md`,
  `_workspace/03-domain-qa-m2-recall.md`, and
  `_workspace/final/m2-recall-handoff.md` as immutable prior-slice evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/lane.rs`, `src/kernel.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

`ThreatReport::LastKnown` is a projection of the bounded RiverSide reportable
case. `Absent` and current hidden `InLane` threat truth remain `Unknown`, so
the observation does not become an omniscient view of the jungle threat. No
new state field, command, intent, transition rule, or policy authority was
introduced.

The report exposes only a region label and observation turn. It omits source
hashes, exact entities, current movement, opponent truth, and execution
values. The allied proposal artifact remains unchanged and continues to use
its prior unknown-threat boundary.

## Replay and Information Findings

Player observation regeneration during lane-history replay reproduces the
RiverSide report exactly. Existing transition outputs, state hashes, command
validation, intent availability, branch identities, objective records, and
debrief contracts remain unchanged. The report is explicitly last-known and
does not claim current truth after the observation.

## Required Fixes

None for the declared bounded last-known threat-report slice.

## Residual Risks

- Complete vision, belief updates, threat timing, and gank-response semantics
  remain unimplemented.
- Portable serialization, variable pacing, communication, and broader
  presentation remain deferred.
- The repository remains an internal non-playable fixture; no human-experience,
  accessibility, trust, balance, or strategy-quality claim is supported.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 50 tests passed: 19 M1 and 31 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
