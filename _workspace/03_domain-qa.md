# Domain QA

## Status

pass

This QA covers one fixed player-only FarSide opponent sighting over the
existing report, observation receipt, transition, and replay authority. It
does not promote complete vision, belief updates, memory, communication,
automatic threat timing, cooldowns, gold, experience, a playable host, human
experience, accessibility, trust, legal clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-mana-resource-request-summary.md`,
  `_workspace/01-simulation-design-m2-mana-resource.md`,
  `_workspace/03-domain-qa-m2-mana-resource.md`, and
  `_workspace/final/m2-mana-resource-handoff.md` as immutable prior-slice
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/lane.rs`, `src/kernel.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

The FarSide rule is a deterministic projection over true opponent position. It
does not add vision state, a belief store, a command, a transition branch, or
an alternate authority. Center and NearTower remain Unknown. Player health and
posture remain hidden, and the allied observation remains Unknown for all
opponent positions.

## Replay and Information Findings

The player report is captured in the existing observation receipt and replayed
from the same state/turn. Hidden health/posture substitutions at the same
FarSide position produce equal player observations. FarSide history replay
regenerates the same report and unchanged transition/state hash behavior.

## Required Fixes

None for the declared bounded opponent-report slice.

## Residual Risks

- Complete vision, beliefs, memory expiration, communication, and automatic
  threat timing remain unimplemented.
- Portable serialization and broader presentation remain deferred.
- The repository remains an internal non-playable fixture; no human-experience,
  accessibility, trust, balance, or strategy-quality claim is supported.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 60 tests passed: 19 M1 and 41 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
