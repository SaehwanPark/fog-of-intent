# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
allied coordination, objective, strategy-fixture, two-window, and debrief
contracts: add one explicit low-risk `Recall` player intent to the existing
one-window command/transition boundary. Keep allied proposals limited to their
existing two-intent policy and preserve all replay identities.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded Recall-intent follow-up.

## Current Evidence

- M1 is complete; the first M2 decision window, bounded branch, allied
  proposal/coordination overlay, terminal-objective review, three strategy
  fixtures, two-window wrapper, and final debrief are merged on `main` through
  `7491bca`; this branch advances the package from `0.1.10` to `0.1.11`,
  pinned to Rust `1.96.0`,
  with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add `Recall` to the player-visible legal intent set and host validation.
- Resolve Recall as a deterministic low-risk plan that moves the player near
  tower, holds the wave, and preserves explicit execution inputs/provenance.
- Preserve allied policy candidate bounds, deterministic replay, branch
  compatibility, objective attribution, and final-debrief contracts.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No second decision window, full lane scenario, CLI, MCP, GUI, broad agent
  population, persistence codec, or general communication framework.
- No variable pacing, gank mechanics, hidden-state score, optimality claim,
  balance claim, or new lane mechanics beyond this bounded Recall plan.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: objective evaluation is a post-commit pure
  projection; coordination and transition semantics remain host/kernel-owned.

## Source Files

- `src/lane.rs` Recall intent and focused tests
- existing M1 fixture tests remain unchanged
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable M2 final-debrief handoff snapshots

## Expected Outputs

- Recall legality, transition, objective attribution, and replay tests.
- Updated M2 checklist and SDD/domain handoff artifacts.
- Passing local checks, one code-reviewer’s three-pass review, hosted CI, and a
  merged PR.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`
- Focused Recall-boundary, actor-observation, deterministic transition, and
  replay tests.

## Evidence Limits and Open Questions

- The slice establishes only one bounded Recall plan; it does not establish a
  complete playable scenario, pacing, gank response, balance, optimality, or
  human-experience evidence.
- Portable serialization, migration, multi-window state, general communication,
  and broad agent-population claims remain deferred.
