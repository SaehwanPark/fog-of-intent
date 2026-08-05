# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
allied coordination, objective, and strategy-fixture contracts: add a
two-window scenario wrapper with an explicit deterministic reopen boundary and
append-only replay. Keep the existing one-window transition and branch
identities unchanged.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded two-window scenario follow-up.

## Current Evidence

- M1 is complete; the first M2 decision window, bounded branch, allied
  proposal/coordination overlay, terminal-objective review, and three strategy
  fixtures are merged on `main` through `9d0e353`; this branch advances the
  package from `0.1.8` to `0.1.9`, pinned to Rust `1.96.0`,
  with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Define a bounded two-window scenario history over existing lane records.
- Reopen the resolved first window through one explicit host-owned deterministic
  boundary before accepting the second window.
- Preserve actor-visible information limits, deterministic replay, branch
  compatibility, and existing coordination/debrief attribution.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No second decision window, full lane scenario, CLI, MCP, GUI, broad agent
  population, persistence codec, or general communication framework.
- No variable pacing, recall/gank mechanics, hidden-state strategy score,
  optimality claim, balance claim, or new lane mechanics beyond the existing
  bounded transition.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: objective evaluation is a post-commit pure
  projection; coordination and transition semantics remain host/kernel-owned.

## Source Files

- `src/lane.rs` two-window scenario history and focused tests
- existing M1 fixture tests remain unchanged
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable M2 strategy-fixture handoff snapshots

## Expected Outputs

- Two-window reopen, append, deterministic transition, objective, and replay
  tests.
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
- Focused window-boundary, actor-observation, deterministic transition, and
  replay tests.

## Evidence Limits and Open Questions

- The slice establishes only a two-window diagnostic wrapper; it does not
  establish a complete playable scenario, pacing, balance, optimality, or
  human-experience evidence.
- Portable serialization, migration, multi-window state, general communication,
  and broad agent-population claims remain deferred.
