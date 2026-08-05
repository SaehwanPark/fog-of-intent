# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane, branch,
allied coordination, and objective contracts: add three explicit matched-input
diagnostic strategy fixtures (happy-path, risk-taking, conservative) over the
existing one-window mechanics. Keep fixture content separate from simulation
authority and do not add a second window yet.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, strategy-fixture follow-up.

## Current Evidence

- M1 is complete; the first M2 decision window, bounded branch, allied
  proposal/coordination overlay, and terminal-objective review are merged on
  `main` through `31836e0`; this branch advances the package from `0.1.7` to
  `0.1.8`, pinned to Rust `1.96.0`,
  with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Define three named strategy fixtures: happy-path, risk-taking, and
  conservative.
- Bind each fixture to explicit player intent/response and resolved execution
  inputs, then evaluate the existing terminal objective.
- Preserve actor-visible information limits, deterministic replay, branch
  compatibility, and the existing coordination/debrief attribution.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No second decision window, full lane scenario, CLI, MCP, GUI, broad agent
  population, persistence codec, or general communication framework.
- No hidden-state strategy score, optimality claim, balance claim, or new
  mechanics beyond the existing bounded projection.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: objective evaluation is a post-commit pure
  projection; coordination and transition semantics remain host/kernel-owned.

## Source Files

- `src/lane.rs` fixture descriptors and focused tests
- existing M1 fixture tests remain unchanged
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable M2 objective handoff snapshots

## Expected Outputs

- Happy-path, risk-taking, conservative, objective, and replay tests.
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
- Focused fixture-boundary, objective attribution, deterministic projection,
  and replay tests.

## Evidence Limits and Open Questions

- The slice establishes only three one-window matched-input fixture cases; it
  does not establish a playable scenario, balance, optimality, or
  human-experience evidence.
- Portable serialization, migration, multi-window state, general communication,
  and broad agent-population claims remain deferred.
