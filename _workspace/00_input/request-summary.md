# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane contract:
support one counterfactual branch at the pivotal decision, with explicit policy
for reusing or regenerating execution inputs, parent immutability, and replay
identity. Keep the branch local to the existing one-window history contract.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded counterfactual branch follow-up.

## Current Evidence

- PR #6 merged the versioned replay fixture codecs to `main` as `c5d7a9d` after
  the prior M1 implementation and codec checks passed.
- M1 is complete and the first M2 decision-window slice is merged on `main` as
  `986958c`; this branch advances the package from `0.1.4` to `0.1.5`, pinned
  to Rust `1.96.0`, with no
  dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add a bounded branch from the immutable initial record boundary of the
  existing one-window lane history.
- Define matched-input and regenerated-input branch policies with stable trace
  identities and separate replay identities.
- Add branch replay and parent-immutability tests while preserving the existing
  observation, intent, transition, and debrief contracts.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No second decision window, full lane scenario, CLI, MCP, GUI, autonomous
  policy population, persistence codec, or general branching framework.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: branch policy is explicit data at the history edge;
  observation, transition, and replay semantics remain host/kernel-owned.

## Source Files

- `src/lane.rs` history/branch contracts and focused tests
- existing M1 fixture tests remain unchanged
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable M1 handoff snapshots

## Expected Outputs

- Bounded branch policy, branch history, and focused replay tests.
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
- Focused branch, input-policy, parent-immutability, and replay tests.

## Evidence Limits and Open Questions

- The branch establishes only a local one-window counterfactual contract; it
  does not establish a playable scenario, balance, or human-experience evidence.
- Portable branch serialization, migration, multi-window state, and general
  branch graphs remain deferred.
