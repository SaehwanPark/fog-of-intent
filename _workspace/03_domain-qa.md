# Domain QA

## Status

pass

This QA covers the bounded conditional Withdraw response over the existing
player observation, lane transition, and replay authority. It does not promote
complete vision/belief updates, automatic threat damage, variable pacing, a
playable host, human experience, accessibility, trust, legal clearance, or
research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-last-known-threat-request-summary.md`,
  `_workspace/01-simulation-design-m2-last-known-threat.md`,
  `_workspace/03-domain-qa-m2-last-known-threat.md`, and
  `_workspace/final/m2-last-known-threat-handoff.md` as immutable prior-slice
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/lane.rs`, `src/kernel.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

Withdraw is a host-validated `LaneIntent` that is conditionally advertised by
the current player observation. Only a RiverSide `LastKnown` report authorizes
it; Unknown current InLane/Absent states do not. No policy output becomes a
command, and no new source of transition authority or hidden current truth was
introduced.

The existing transition consumes explicit wave, damage, and execution-trace
inputs. Withdraw moves NearTower with intent attribution and does not activate
the Contest fallback. A legal unfavorable execution remains distinct from
invalid command rejection.

## Replay and Information Findings

Withdraw history replay regenerates the conditional observation, validates the
command, reruns the transition, and reproduces objective intent attribution.
The allied candidate artifact remains exactly Stabilize/Contest. No opponent
truth, exact threat entity, current movement, source hash, or execution result
is exposed to the player.

## Required Fixes

None for the declared bounded Withdraw slice.

## Residual Risks

- Automatic threat damage, current threat tracking, complete vision/belief
  updates, and variable pacing remain unimplemented.
- Portable serialization, communication, and broader presentation remain
  deferred.
- The repository remains an internal non-playable fixture; no human-experience,
  accessibility, trust, balance, or strategy-quality claim is supported.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 53 tests passed: 19 M1 and 34 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
