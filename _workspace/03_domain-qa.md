# Domain QA

## Status

pass

This QA covers the first bounded M2 lane decision-window implementation. It
does not promote the complete M2 scenario and does not validate a playable
host, human experience, accessibility, trust, legal clearance, or research
validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `_workspace/01_simulation-design-m1.md` and `_workspace/03_domain-qa-m1.md`
  as immutable prior-slice evidence
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `docs/harness/fog-of-intent/team-spec.md`, `docs/TERMINOLOGY.md`, and
  `docs/adr/0001-authoritative-transition-boundary.md`
- `src/kernel.rs`, `src/lane.rs`, `src/lib.rs`, and focused test output
- Locked Rust 1.96.0 format, clippy, tests, repository checks, and diff checks

## Scope and Roadmap Findings

The implementation matches the declared first M2 slice: one typed lane
snapshot, one player-laner observation, two legal intent variants, explicit
execution inputs, one deterministic transition, one-window debrief data, and
append-only replay. The overarching M2 checklist remains open for multiple
windows, allied behavior, communication, pacing, branching, and a complete
scenario. No CLI, MCP, persistence codec, or general scenario framework was
added.

## Authority and Information-Boundary Findings

`LaneSnapshot` is true host-owned state. `observe_player` returns only the
player's health/position, wave pressure, explicit unknown reports, legal intent
set, and window identity. Opponent health/posture/position and jungle threat
remain outside the actor-visible projection. `LaneObservationReceipt` keeps
the exact source-state hash private to the host validation boundary.

Requests are not authoritative commands. Host validation creates a
`LaneIntentCommand` bound to actor, turn, ruleset, observation receipt, and
prior hash; `ValidatedLaneIntent` is required by the pure transition. Invalid
metadata is rejected before events, effects, or history mutation.

## Determinism, Replay, and Reproducibility Findings

`transition_lane` consumes only owned state, validated intent, and explicit
resolved execution inputs. It generates no random values, reads no clock or
I/O, and keeps neutral environment/observation/policy/coordination traces
separate from the consumed execution trace. Lane state hashing uses the
declared stable field order and the existing FNV-1a little-endian representation.

`LaneHistory` stores the actor-visible observation, host command, prior hash,
resolved inputs, complete result, and terminal state. Replay regenerates the
observation, revalidates the command, reevaluates the inputs, compares the
result, and checks the terminal snapshot. The M1 serialization fixtures remain
unchanged; lane serialization is explicitly deferred.

## Behavior and Playtest Findings

No autonomous policy or agent population was added. `Stabilize` and `Contest`
are two observation-available strategies; execution damage and wave outcomes
are resolved inputs, not policy decisions. The test suite covers a legal but
unfavorable contest with fallback and does not claim human-like behavior.

## Gameplay and Debrief Findings

The one-window diagnostic preserves a meaningful conservative/risk-taking
choice and separates intent, coordination-not-applicable, execution, and luck
trace data. Its terminal result is `HeldSpace`, `YieldedSpace`, or `ForcedOut`,
not a binary win/loss judgment. This is a technical debrief contract, not
evidence of enjoyment, balance, or an understandable human experience.

## Evidence and Claim Limits

The evidence establishes typed software boundaries, hidden-state omission,
validation ordering, legal unfavorable execution, boundedness, deterministic
outputs, stream isolation, append-only replay, and a one-window debrief shape.
It does not establish a playable simulation, human enjoyment, accessibility,
trust, behavioral validity, legal clearance, public-release readiness, or
research validity.

## Required Fixes

None for the declared first M2 slice.

## Residual Risks

- Lane snapshots/history are not serialized; external compatibility and
  migration policy remain deferred.
- The host/application layer and player-facing adapter are not implemented;
  the binary remains a placeholder.
- Branching, multiple windows, allied autonomous behavior, communication,
  variable pacing, and full terminal debrief remain unimplemented.
- The two strategies and diagnostic values are fixtures, not balance or human
  experience evidence.

## Verification Evidence

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked` — 28 tests passed: 19 M1 and 9 M2 lane tests.
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
