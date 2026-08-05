# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged one-window lane and branch
contracts: add one allied autonomous proposal and a host-owned coordination
resolution at the same decision window. Keep the proposal observation-bound and
the coordination/execution split explicit; do not add a second window yet.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, allied proposal and coordination follow-up.

## Current Evidence

- PR #6 merged the versioned replay fixture codecs to `main` as `c5d7a9d` after
  the prior M1 implementation and codec checks passed.
- M1 is complete; the first M2 decision window and bounded branch are merged on
  `main` through `3b7899b`; this branch advances the package from `0.1.5` to
  `0.1.6`, pinned to Rust `1.96.0`, with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add one allied autonomous actor that submits an observation-bound proposal for
  the current window.
- Add typed proposal acceptance/rejection or counter-proposal coordination,
  resolved separately from the existing mechanical execution inputs.
- Preserve actor-visible information limits, deterministic replay, branch
  compatibility, and current one-window debrief attribution.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No second decision window, full lane scenario, CLI, MCP, GUI, broad agent
  population, persistence codec, or general communication framework.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: proposal policy is observation-bound at the edge;
  coordination and transition semantics remain host/kernel-owned.

## Source Files

- `src/lane.rs` proposal/coordination contracts and focused tests
- new `_workspace/01_agent-ecology-design.md` bounded-policy contract
- existing M1 fixture tests remain unchanged
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable M2 window-1 handoff snapshots

## Expected Outputs

- Allied proposal, coordination resolution, and matched-input replay tests.
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
- Focused observation-bound policy, coordination, deterministic transition,
  and replay tests.

## Evidence Limits and Open Questions

- The slice establishes only a one-window allied coordination contract; it does
  not establish a playable scenario, balanced behavior, or human-experience
  evidence.
- Portable serialization, migration, multi-window state, general communication,
  and broad agent-population claims remain deferred.
