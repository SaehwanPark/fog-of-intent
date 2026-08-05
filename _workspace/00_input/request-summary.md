# Request Summary

## Requested Outcome

Implement the first bounded M2 lane decision-window slice after the completed
M1 kernel and fixture merge. Keep the work small enough to prove one typed lane
state, one actor-valid observation, one host-validated intent command, explicit
execution input, deterministic transition output, and replay identity before
adding the rest of the scenario.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, first decision-window boundary.

## Current Evidence

- PR #6 merged the versioned replay fixture codecs to `main` as `c5d7a9d` after
  the prior M1 implementation and codec checks passed.
- M1 is now promoted to complete in `ROADMAP.md` and `SPEC.md`; the package is
  version `0.1.3`, remains pinned to Rust `1.96.0`, and has no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add the smallest lane state and actor-specific observation contract needed for
  one deterministic decision window.
- Add one host-validated intent command with explicit execution resolution and
  visible event/effect output, preserving the existing kernel invariants.
- Add replay-backed fixtures and focused tests for state/output determinism,
  invalid command rejection, hidden-state omission, and legal unfavorable
  execution.
- Synchronize the M2 design, roadmap, SPEC, architecture, changelog, and domain
  QA evidence after verification.

## Non-Goals

- No full lane scenario, CLI, MCP, GUI, autonomous policy population, branching,
  or arbitrary scripting format.
- No change to the placeholder binary or claim of a playable simulation.

## Project Boundaries Touched

- Host-owned history/replay authority from ADR-0001.
- Functional core boundary: lane observation and intent remain typed domain
  contracts; replay and transition semantics remain owned by the kernel.

## Source Files

- `src/kernel.rs`, `src/lib.rs`, and the new M2 lane/observation modules
- `tests/fixtures/` versioned text fixtures and focused M2 tests
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`

## Expected Outputs

- Typed lane snapshot, actor-visible observation, and one intent transition.
- Versioned checked-in fixtures and focused tests.
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
- Focused observation, command, transition, and replay tests.

## Evidence Limits and Open Questions

- The M2 slice establishes only the first lane decision-window boundary; it does
  not establish a playable scenario or human-experience evidence.
- Scenario-specific serialization and migration policy remain deferred until
  the lane contract proves stable.
