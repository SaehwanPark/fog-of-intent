# Simulation Design — M2 Delayed-Effect Origin Trace

## Goal and Boundary

Extend the existing bounded delayed-effect queue so every queued effect carries
the execution trace that originated it. The deterministic kernel remains the
authority; no randomness or I/O is added.

## Contract

`LaneDelayedEffect { delay, kind, origin_trace }` is immutable value data. The
edge-facing `LaneExecutionInputs::with_delayed_effect` binds the supplied effect
to that window's execution trace. Delay ticking preserves the original trace.
The lane state hash and record identity include the origin trace so tampering or
trace substitution fails replay.

When a queued effect resolves, `LaneEvent::DelayedEffectResolved` stores the
origin trace in its `trace` field, while
`LaneEffect::DelayedEffectResolved` uses that origin in its
`LaneEffectCause::Execution` attribution. The current window trace remains
available separately through the transition inputs and lane debrief; the
resolved event/effect projections expose the originating trace. `LaneDebrief`
and the two-window final debrief report expose the bounded list of resolved
origin traces without exposing the host state hash.

## Compatibility

Because authoritative state/hash/replay meaning changes, current internal M2
ruleset, observation, replay, profile, strategy, and branch identities advance
from v2 to v3. Unsupported v2 M2 artifacts fail closed; M1 fixtures remain
unchanged.

## Verification Contract

- Origin trace is bound on queueing and preserved after one- and two-beat ticks.
- Delayed resolution events/effects attribute the original trace.
- State hashes and record identities differ when origin trace differs.
- Replay verifies the updated v3 records and rejects tampering.
- Lane and final debrief projections expose resolved origin traces and replay
  verification covers their committed values.
