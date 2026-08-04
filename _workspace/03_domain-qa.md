# Domain QA

## Status

`pass`

This QA covers the first M1 bounded deterministic-kernel fixture. It does not
validate a playable scenario, human experience, legal clearance, or research
validity.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `_workspace/01_simulation-design.md`
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `docs/TERMINOLOGY.md`, `docs/COMPATIBILITY.md`, and ADR-0001
- `src/lib.rs` and `src/kernel.rs`
- Local Rust 1.96.0 locked metadata, formatting, clippy, tests, repository
  currentness/link checks, focused checker tests, and diff checks
- One code-reviewer’s three-pass report; the identified state-binding,
  provenance, version, and stale-handoff issues were corrected before PR
  handoff

## Scope and Roadmap Findings

The implementation is limited to the active M1 fixture: one actor, one bounded
energy resource, score, `Hold`/`Gather`, explicit resolved-input categories,
events/effects, state hashes, and in-memory replay. M1 serialization and
property-style tests remain unchecked and are not claimed complete. No M2 lane
mechanics or adapter surface was added.

## Authority and Information-Boundary Findings

`WorldState` remains host-owned true state. `validate_command` is separate from
`transition`, and `ValidatedCommand` is bound to the exact prior state rather
than only its hash. The kernel accepts resolved inputs but does not generate
randomness, infer observations, or expose a player-facing projection.

## Determinism, Replay, and Reproducibility Findings

The transition is synchronous and I/O-free. Stable IDs and five named input
categories are explicit. The state hash uses a documented FNV-1a calculation
over ruleset, turn, actor ID, bounded energy, and score in declared byte order.
History stores prior hashes, commands, resolved inputs, events, effects, next
state, and next hash; replay revalidates and compares every stored result.
Unrelated input-stream identity changes do not affect the evaluated result.

## Behavior and Playtest Findings

No actor policy, playtest, or behavioral claim was added. A zero-yield gather is
tested as a legal but unfavorable execution result, distinct from malformed or
illegal command rejection.

## Gameplay and Debrief Findings

No gameplay or debrief surface was added. Command and execution provenance in
the effects provide a causal seam for later debrief work without claiming that a
player-facing explanation exists.

## Evidence and Claim Limits

The fixture establishes software properties only: typed validation, exact-state
binding, boundedness, energy conservation, deterministic output, provenance,
and replay verification. It does not establish human enjoyment, accessibility,
trust, legal clearance, public-release readiness, or scientific validity.

## Required Fixes

None for the bounded M1 slice after the reviewer corrections.

## Residual Risks

- Snapshot/history serialization and property-style tests remain open M1 work.
- The binary remains a placeholder; no user-facing host or playable simulation
  exists.
- A future non-empty dependency graph still requires the approved advisory and
  license policy tooling or an exact machine-readable defer record.

## Verification Evidence

- Eleven focused Rust kernel tests pass, including exact-state validation,
  malformed/illegal commands, legal unfavorable outcomes, bounds,
  conservation, ordering, repeated runs, stream isolation, and replay.
- `cargo +1.96.0 test --locked` passes.
- `cargo +1.96.0 fmt --check` passes.
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings` passes.
- `python3 scripts/check_repository.py` passes.
- Seven focused repository-checker tests pass.
- `git diff --check` passes.
