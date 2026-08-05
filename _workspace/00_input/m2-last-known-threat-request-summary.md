# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
allied coordination, objective, strategy-fixture, two-window, debrief, and
Recall contracts: add one explicit player-visible last-known jungle-threat
report without exposing current hidden threat truth or adding a new transition
mechanic. Preserve the existing player intent set, allied policy boundary, and
replay identities.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded last-known threat-report follow-up.

## Current Evidence

- M1 and the prior M2 slices are merged on `main` through `20935a0`; this
  branch advances the package from `0.1.11` to `0.1.12`, pinned to Rust
  `1.96.0`, with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add an actor-visible `LastKnown` jungle-threat report for the bounded
  `RiverSide` truth case, including the observation turn.
- Keep `Absent` and current `InLane` threat truth reported as `Unknown`; no
  current hidden threat location or source state hash may cross the boundary.
- Preserve the existing `Stabilize`/`Contest`/`Recall` player intents, allied
  two-intent candidate policy, transition authority, and replay behavior.
- Synchronize the M2 design, roadmap, SPEC, architecture, and domain-QA
  handoff artifacts after verification.

## Non-Goals

- No gank-response intent, threat damage rule, variable pacing, vision system,
  communication, resource mechanic, CLI, MCP, GUI, or playable scenario.
- No change to authoritative `LaneSnapshot` fields, transition outputs, state
  hash inputs, or replay IDs.
- No claim that a last-known report is current truth, complete vision, or a
  model of human threat perception.

## Project Boundaries Touched

- Actor-specific observation projection and explicit unknown/last-known wording.
- Host-owned replay regeneration of the exact observation from committed state.

## Source Files

- `src/lane.rs` threat report type, player projection, and focused tests
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable Recall handoff snapshots

## Expected Outputs

- Last-known report API with RiverSide-only projection.
- Tests for unknown current/absent threats, RiverSide last-known wording,
  observation replay, and hidden-state/source-hash boundaries.
- Passing local checks, one code-reviewer PR handoff, hosted CI, and a merged
  PR with temporary branch cleanup.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Open Questions

- The slice establishes only one bounded last-known report. It does not
  establish vision completeness, threat timing, gank response, pacing,
  balance, strategy quality, or human-experience evidence.
- The next gank-response slice must define what an actor can act on without
  treating the last-known report as current hidden truth.
