# Simulation Design — M2 Bounded Effect Provenance

## Goal and Boundary

This slice makes the existing lane effects explicit about relationship and
timing. Every currently emitted effect is `Immediate`; explicit health, wave,
and intent position changes are `Direct`, while Contest fallback position
movement is `Indirect` because the fallback rule mediates the movement.

The existing `LaneEffectCause` remains unchanged and continues to carry intent,
fallback, or execution trace attribution. No delayed queue, future event, new
state field, or transition authority is added.

## Provenance Contract

```text
LaneEffectProvenance {
    relation: Direct | Indirect,
    timing: Immediate | Delayed,
}
```

Current emission mapping:

- health changed by explicit execution: `Direct + Immediate`;
- wave pressure changed by explicit execution: `Direct + Immediate`;
- position changed by Stabilize, Recall, or Withdraw intent:
  `Direct + Immediate`;
- position changed by Contest fallback: `Indirect + Immediate`.

`Delayed` is a declared vocabulary value only in this slice; no delayed effect
is emitted or stored. Existing event ordering, effect cause/trace, state hash,
history, branch, objective, scenario, and final-debrief contracts remain
unchanged.

## Read Models and Replay

`LaneEffect` exposes provenance accessors without exposing hidden truth. The
transition constructs the labels from the same explicit command/execution
causes used by the prior implementation. Replay regenerates the same effects
and provenance; objective/debrief projections can retain their existing
committed-facts boundary.

## Verification Contract

Focused tests cover direct immediate health/wave/intent effects, indirect
immediate Contest fallback movement, absence of delayed effects, and replay
equality. Existing M1/M2 tests remain passing.

Evidence establishes provenance labels for current immediate effects only. It
does not establish delayed effects, causal completeness, strategy quality,
balance, or a complete playable scenario.
