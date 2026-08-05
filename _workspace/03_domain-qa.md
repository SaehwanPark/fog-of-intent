# Domain QA

## Status

pass

This QA covers the bounded player-facing Recall intent over the existing
one-window lane authority and replay/debrief projections. It does not promote
the complete M2 scenario and does not validate variable pacing, gank response,
communication, a playable host, human experience, accessibility, trust, legal
clearance, or research validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/02_design-synthesis.md`
- `_workspace/00_input/m2-final-debrief-request-summary.md`,
  `_workspace/01-simulation-design-m2-final-debrief.md`,
  `_workspace/03-domain-qa-m2-final-debrief.md`, and
  `_workspace/final/m2-final-debrief-handoff.md` as immutable prior-slice
  evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `src/lane.rs`, `src/kernel.rs`, `src/lib.rs`, and focused test output

## Scope and Authority Findings

Recall is represented as an existing `LaneIntent` value. The player observation
advertises it, the allied observation and scripted proposal remain limited to
the two prior candidates, and host validation binds the request to the current
actor-visible observation. No policy output becomes a command and no new
authoritative state or hidden-state fact is introduced.

Recall resolves synchronously through `transition_lane`: NearTower movement is
intent-attributed, explicit wave and execution inputs remain authoritative, and
the existing YieldedSpace/ForcedOut outcome boundary is retained. Contest-only
fallback behavior remains isolated from Recall.

## Attribution and Replay Findings

Recall uses the existing command, record, branch, objective, scenario, and final
debrief paths. Its intent tag is distinct while Stabilize/Contest tags remain
stable. The visible observation contains no latent opponent values, threat
truth, source hash, or execution result. Legal unfavorable Recall execution is
distinct from invalid command rejection and remains replay-verifiable.

## Required Fixes

None for the declared bounded Recall slice.

## Residual Risks

- Recall has no timing, resource restoration, or pacing semantics yet.
- Portable serialization, multi-window coordinated debrief, communication,
  gank response, and broader presentation remain deferred.
- The repository remains an internal non-playable fixture; no human-experience,
  accessibility, trust, balance, or strategy-quality claim is supported.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 48 tests passed: 19 M1 and 29 M2 tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
