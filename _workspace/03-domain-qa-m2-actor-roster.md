# Domain QA — M2 Explicit Actor Roster

## Status

`pass` for the bounded actor-roster slice. This is not an M2 promotion and does
not establish playability, balance, or human-experience evidence.

## Reviewed Inputs

- `_workspace/00_input/m2-actor-roster-request-summary.md`
- `_workspace/01_simulation-design-m2-actor-roster.md`
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/lane/values.rs`, `src/lane/observation.rs`, and focused lane tests

## Scope and Roadmap Findings

The change is limited to the unchecked M2 actor-definition item: four stable
role identities and actor-visible roster metadata. No CLI, persistence, new
transition, or future milestone surface was added. The corresponding roadmap
item and current-state documents now describe verified behavior.

## Authority and Information-Boundary Findings

`LaneActorRoster` is fixed metadata and is not part of `LaneSnapshot`; the host
still owns true state and observations remain projections. Player and allied
observations expose role/identity only. Existing reports continue to redact
opponent health/posture and jungle truth, and the receipt debug paths still omit
the source state hash.

## Determinism, Replay, and Reproducibility Findings

The roster is constant, has no randomness or I/O, and is excluded from the
authoritative state hash. Existing transition, history, branch, coordination,
scenario, and debrief replay paths are unchanged and pass their prior tests.

## Behavior and Playtest Findings

No behavior policy or execution rule changed. The allied actor remains a
proposal-only bounded policy artifact; the roster does not grant privileged
truth or make an optimality claim. No AI playtest evidence was produced or
needed for this identity-only slice.

## Gameplay and Debrief Findings

No gameplay choice, transition outcome, or debrief calculation changed. The
slice only makes the actor set inspectable; complete vision, belief updates,
communication, pacing, and a full scenario remain open.

## Evidence and Claim Limits

The tests establish role completeness, observation equality, redaction, and
hash isolation. They do not establish human enjoyment, accessibility, trust,
behavioral validity, balance, legal clearance, or public-release readiness.

## Required Fixes

None for this bounded slice.

## Residual Risks

- The fixed roster is intentionally not extensible; multiple allied actors or
  threat sources require a separately designed scenario contract.
- The M2 exit checklist still has incomplete vision/belief, pacing, effects,
  terminal, and complete-replay inspection evidence.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 91 tests passed
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'` — 9 tests passed
- `git diff --check`
