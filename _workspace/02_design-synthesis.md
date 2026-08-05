# Design Synthesis — M2 Bounded Effect Provenance

## Decision

Add relationship and timing provenance to the existing `LaneEffect` values
without changing the authoritative transition inputs or state hash. The
existing transition continues to evaluate a selected window from explicit
resolved inputs and emits only immediate effects in this slice.

The authoritative transition, command shape, allied candidate policy, state
hashes, and replay identities remain stable. Existing effect causes and trace
attribution remain available alongside the new labels.

## Resolved Contract

`LaneEffectProvenance` distinguishes `Direct` from `Indirect` relationship and
`Immediate` from `Delayed` timing. Explicit health, wave, and intent-position
effects are direct/immediate; Contest fallback movement is indirect/immediate.
`Delayed` is vocabulary only: no delayed queue, future event, or stored delayed
effect is added. Existing history, branch, objective, scenario, and debrief
contracts remain intact.

## Evidence and Limits

Focused tests cover direct/immediate explicit effects, indirect/immediate
fallback movement, absence of delayed emissions, and replay-preserved
provenance. The full suite passes with 55 Rust tests.

This establishes labels for current immediate effects only. It does not
establish delayed effects, causal completeness, adaptive pacing, communication,
strategy quality, balance, or a complete playable scenario.
