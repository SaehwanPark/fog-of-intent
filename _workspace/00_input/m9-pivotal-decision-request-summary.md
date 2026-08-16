# Request Summary — M9 Match-Level Pivotal-Decision Detection

## Requested outcome

Implement the next open M9 scope item from `ROADMAP.md`:

> - [ ] Add match-level pivotal-decision detection.

Deliver a bounded, deterministic, pure-evaluation boundary that identifies
which declared match decisions were pivotal (largest value swings, lead
changes) for match-level debriefing, following the established M9 pattern of
explicit caller-supplied inputs with no authoritative match-state access.

## Current milestone

M9 — Bounded Multi-Lane Match Prototype (Phase 9). The last merged slice was
`m9-comeback-mechanics-v1` (PR #204, commit `a8d9329`), which explicitly
deferred match-level pivotal-decision detection.

## Scope

- One new module `src/map/pivotal.rs` (core detection contract).
- One new module `src/map/pivotal_catalog.rs` (canonical benchmark scenarios).
- One new test module `src/map/tests/pivotal.rs`.
- `src/map/mod.rs` and `src/map/tests/mod.rs` wiring.
- Doc reconciliation: `CHANGELOG.md`, `ROADMAP.md`, `SPEC.md`, `README.md` if
  its claims change.
- Version bump per `README.md` versioning policy.

## Non-goals

- No automatic detection from true authoritative match state (remains open,
  as with comeback evaluation).
- No host/CLI/MCP integration of pivotal reporting into the runnable fixture.
- No counterfactual branch execution from a pivotal decision (M2 already has
  a bounded branch; match-level branching stays deferred).
- No floating-point math, randomness, I/O, async, wall clock, or hidden state.
- No decision-density optimization (separate open item).

## Source files informing the slice

- `src/map/comeback.rs`, `src/map/comeback_catalog.rs`,
  `src/map/tests/comeback.rs` — established M9 pure-evaluation pattern.
- `src/map/composition.rs` (`MatchPhase`), `src/map/topology.rs` (`TeamSide`).
- `ROADMAP.md` Phase 9 scope and evidence sections; `SPEC.md` M9 evidence.

## Expected outputs

- Versioned `m9-pivotal-decision-v1` contract and `m9-pivotal-catalog-v1`
  benchmark scenarios with focused Rust tests.
- Updated project-state documents with bounded evidence claims.

## Validation

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `fog-intent-domain-qa` review before handoff.

## Evidence limits

This slice establishes deterministic detection over caller-declared value
trajectories only. It does not establish automatic trajectory derivation from
authoritative state, decision quality/optimality, human debrief usefulness, or
complete match debrief integration.
