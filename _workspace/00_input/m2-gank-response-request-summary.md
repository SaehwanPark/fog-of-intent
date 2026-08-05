# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
allied coordination, objective, strategy-fixture, two-window, debrief, Recall,
and last-known threat-report contracts: add one conditional `Withdraw` gank
response that is available only when the player sees the bounded RiverSide
last-known threat report. Preserve the existing command, transition, replay,
and hidden-state boundaries.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded gank-response follow-up.

## Current Evidence

- M1 and the prior M2 slices are merged on `main` through `e625304`; this
  branch advances the package from `0.1.12` to `0.1.13`, pinned to Rust
  `1.96.0`, with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add `Withdraw` as a player command intent that is advertised only through a
  current `LastKnown { region: RiverSide, ... }` observation.
- Resolve Withdraw through the existing deterministic lane transition as a
  one-beat NearTower response, preserving explicit wave/execution inputs and
  intent attribution without activating Contest fallback.
- Reject Withdraw for Unknown threat reports, stale observations, resolved
  windows, wrong actors, and malformed command bindings.
- Preserve the allied two-intent policy, existing player strategic intents,
  state-hash inputs, replay identities, objective/debrief paths, and hidden
  current InLane truth.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and
  domain-QA handoff artifacts after verification.

## Non-Goals

- No automatic threat damage rule, full vision/belief system, variable pacing,
  communication, resource mechanic, CLI, MCP, GUI, or playable scenario.
- No policy-generated Withdraw proposal or counter shape; the allied policy
  remains limited to Stabilize and Contest.
- No claim that last-known information is current truth or that Withdraw is
  optimal, balanced, or generally safe.

## Project Boundaries Touched

- Actor-visible conditional intent availability and host validation.
- Existing functional transition/replay boundary with no second authority.

## Source Files

- `src/lane.rs` Withdraw intent availability, transition, and focused tests
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable last-known-threat handoff snapshots

## Expected Outputs

- Conditional Withdraw API and deterministic NearTower response behavior.
- Tests for availability, unknown/stale/resolved rejection, explicit execution
  preservation, attribution, and replay/objective compatibility.
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

- The slice establishes one conditional Withdraw response only. It does not
  establish complete vision, current threat tracking, automatic execution
  outcomes, pacing, strategy quality, balance, or human-experience evidence.
- Future gank-response work may add richer threat timing only after it can keep
  last-known reports distinct from hidden current truth.
