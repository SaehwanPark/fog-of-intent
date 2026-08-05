# Request Summary

## Requested Outcome

Implement the next bounded M2 slice after the merged lane window, branch,
coordination, objective, fixture, scenario, debrief, Recall, threat-report,
Withdraw, and variable-duration contracts: make existing lane effects expose
explicit direct/indirect and immediate timing provenance while retaining their
existing cause/trace attribution. Do not add delayed mechanics in this slice.

## Roadmap Milestone

M2 — One-Lane Vertical Slice, bounded effect-provenance follow-up.

## Current Evidence

- M1 and the prior M2 slices are merged on `main` through `f76c584`; this
  branch advances the package from `0.1.14` to `0.1.15`, pinned to Rust
  `1.96.0`, with no dependencies.
- The binary remains a placeholder and the M2 scenario is not yet playable.

## In Scope

- Add public effect provenance distinguishing `Direct` from `Indirect` and
  `Immediate` timing for all currently emitted lane effects.
- Mark explicit health/wave/intent changes as direct immediate effects and
  Contest fallback position movement as an indirect immediate effect.
- Preserve existing cause/trace fields, event ordering, state hashes,
  transition authority, replay identities, and actor-visible information.
- Synchronize canonical M2 documents and SDD/domain-QA handoff artifacts.

## Non-Goals

- No delayed effect queue, automatic future event, resource mechanic,
  communication, serialization change, CLI, MCP, GUI, or playable scenario.
- No new lane effect categories or changes to transition state/hash inputs.
- No claim that immediate provenance establishes causal completeness, balance,
  optimality, strategy quality, or human experience.

## Project Boundaries Touched

- Attributed effect projection at the deterministic transition boundary.
- Causal debrief/read-model access to direct/indirect and timing labels.

## Source Files

- `src/lane.rs` effect provenance types, emitted effects, and focused tests
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`
- `_workspace/01_simulation-design.md`, `_workspace/03_domain-qa.md`, and
  immutable variable-duration handoff snapshots

## Expected Outputs

- Effect provenance API and direct/indirect immediate labels for current effects.
- Tests for explicit, fallback, wave, health, and replay-preserved provenance.
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

- The slice establishes provenance labels for existing immediate effects only.
  Delayed effects, indirect chains beyond fallback, and causal completeness
  remain open.
- Future delayed-effect work must preserve committed history and keep timing
  inputs explicit at the authoritative transition boundary.
