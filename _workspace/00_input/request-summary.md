# Request Summary

## Requested Outcome

Implement the next bounded M2 information slice: project one fixed,
player-visible opponent sighting into the existing `OpponentReport::LastKnown`
boundary while leaving Center and NearTower unknown. Preserve hidden opponent
health/posture, allied uncertainty, state hashes, transition authority, and
replay behavior.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded opponent last-known report follow-up.

## Current Evidence

- M1 and the prior M2 slices are merged on `main` through `e079cea`; this branch
  advances the package from `0.1.16` to `0.1.17`, pinned to Rust `1.96.0`, with
  no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.
- `OpponentReport` already carries optional last-known position/turn plus hidden
  health/posture fields; player threat reporting already has an explicit
  last-known boundary.

## In Scope

- Define one deterministic player-vision rule: opponent `FarSide` projects as
  `LastKnown { position: FarSide, last_seen_turn: state.turn() }`.
- Keep opponent `Center` and `NearTower` as `Unknown` in the player report.
- Keep opponent health/posture unknown in every player projection and leave the
  allied observation's opponent report unknown.
- Preserve report wording, observation receipt bindings, state hashes,
  transition inputs, history/replay identities, and actor authority.
- Add focused projection, hidden-state, and replay tests and synchronize the
  canonical and SDD/domain-QA artifacts.

## Non-Goals

- No vision graph, ward/line-of-sight state, belief update, memory decay,
  opponent current-state claim, communication, new command, state/hash field,
  resource mechanic, serialization, CLI, MCP, GUI, or playable-scenario claim.
- No player or allied policy change based on opponent position in this slice.
- No claim that one sighting establishes complete vision, strategic quality, or
  human experience.

## Project Boundaries Touched

- Player observation projection and report wording.
- Existing hidden-state information boundary.
- History replay observation regeneration without transition changes.

## Source Files

- `src/lane.rs` opponent report projection and focused tests
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable mana-resource handoff snapshots

## Expected Outputs

- FarSide-only player last-known opponent position projection.
- Explicit unknown behavior for other bounded positions and hidden values.
- Replay and information-boundary tests with no transition/state-hash changes.
- Passing local checks, one-code-reviewer PR handoff, hosted CI, merged PR, and
  temporary branch cleanup.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

- The slice establishes one player-facing FarSide sighting only; complete
  vision, beliefs, memories, and updates remain open.
- Future vision work must keep true position, actor belief, report wording, and
  research inspection distinct and must not add a second transition authority.
