# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
allied coordination, objective, strategy-fixture, two-window, debrief, Recall,
last-known threat-report, and conditional Withdraw contracts: add one explicit
two-beat decision-window duration to the existing deterministic transition.
Preserve the one-beat hash/identity behavior, command/replay authority, allied
policy bounds, and hidden-state boundaries.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded variable-duration-window follow-up.

## Current Evidence

- M1 and the prior M2 slices are merged on `main` through `68cfee2`; this
  branch advances the package from `0.1.13` to `0.1.14`, pinned to Rust
  `1.96.0`, with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add `LaneWindow::TwoBeats` to the authoritative snapshot and current
  actor-visible observations.
- Advance a committed two-beat transition by exactly two turns and close the
  window automatically on the existing transition commit.
- Keep one-beat state hashes and prior replay/identity behavior stable while
  making two-beat state hashes distinct and replay-verifiable.
- Preserve the allied two-intent policy, existing player intents and threat
  response, objective/debrief paths, and hidden current InLane truth.

## Non-Goals

- No third duration, adaptive pacing, manual tick command, automatic threat
  damage rule, full vision/belief system, communication, resource mechanic,
  CLI, MCP, GUI, or playable scenario.
- No change to policy candidate selection or allied support semantics.
- No claim that a two-beat window establishes pacing quality, balance,
  optimality, or human experience.

## Project Boundaries Touched

- Variable-duration window state and automatic close-on-commit behavior.
- Existing functional transition/replay boundary with no second authority.

## Source Files

- `src/lane.rs` LaneWindow duration, snapshot/hash/observation propagation,
  transition advancement, and focused tests
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable gank-response handoff snapshots

## Expected Outputs

- Two-beat window API and deterministic turn/hash/observation behavior.
- Tests for duration propagation, automatic close, distinct state hashing,
  allied observation compatibility, and replay compatibility.
- Passing local checks, one-code-reviewer PR handoff, hosted CI, and merged PR
  with temporary branch cleanup.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

- The slice establishes one two-beat duration only. It does not establish
  adaptive pacing, automatic execution outcomes, strategy quality, balance, or
  human-experience evidence.
- Future pacing work may add additional durations only after preserving
  deterministic transition and replay identities.
