# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
and allied coordination contracts: define one scenario goal and a
host-owned terminal-objective projection for the existing one-window record.
Keep objective evaluation separate from lane mechanics and do not add a second
window yet.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, scenario goal and terminal-objective follow-up.

## Current Evidence

- M1 is complete; the first M2 decision window, bounded branch, and allied
  proposal/coordination overlay are merged on `main` through `ae87d0d`; this
  branch advances the package from `0.1.6` to `0.1.7`, pinned to Rust `1.96.0`,
  with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Define one bounded scenario goal for the current diagnostic lane window.
- Evaluate a terminal-objective projection from committed lane/coordination
  facts without changing `LaneSnapshot`, its hash, or transition authority.
- Preserve actor-visible information limits, deterministic replay, branch
  compatibility, and the existing coordination/debrief attribution.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No second decision window, full lane scenario, CLI, MCP, GUI, broad agent
  population, persistence codec, or general communication framework.
- No hidden-state objective score, optimality claim, balance claim, or new
  terminal mechanics beyond the bounded projection.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: objective evaluation is a post-commit pure
  projection; coordination and transition semantics remain host/kernel-owned.

## Source Files

- `src/lane.rs` objective projection and focused tests
- existing M1 fixture tests remain unchanged
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable M2 coordination handoff snapshots

## Expected Outputs

- Scenario-goal, terminal-objective, attribution, and replay tests.
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
- Focused objective-boundary, coordination attribution, deterministic
  projection, and replay tests.

## Evidence Limits and Open Questions

- The slice establishes only a one-window scenario-goal/objective projection;
  it does not establish a playable scenario, balance, optimality, or
  human-experience evidence.
- Portable serialization, migration, multi-window state, general communication,
  and broad agent-population claims remain deferred.
