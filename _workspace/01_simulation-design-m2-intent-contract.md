# Simulation Design — M2 Bounded Intent Contract

## Goal and Boundary

Record the current typed intent boundary for one lane decision window. Intent
is distinct from command validation and mechanical execution.

## Typed Contract

- `LaneIntent` selects the bounded lane posture/action.
- `LaneCommitment` records cautious, standard, or aggressive commitment.
- `LaneTargetFocus` selects minions, opposing laner, or tower.
- `LanePingSignal` is the bounded communication surface: none, danger, on-my-way,
  assist, or enemy-missing.
- `LaneAbortCondition` and `LaneFallbackBehavior` define conditional exit and
  recovery behavior without executing them at request construction time.

Player observations advertise legal options; host validation binds requests to
the current actor and observation. The transition evaluates intent/fallback and
separately resolves explicit damage, wave, and resource inputs.

## Non-Goals and Compatibility

No free-form message transport, trust, negotiation, or communication population
is added. Existing v2 record identities and replay semantics remain unchanged.

## Verification Contract

Use the existing focused intent tests, validation/replay tests, hidden-state
tests, and full repository checks. Keep the bounded communication surface
explicitly separate from future M3/M4/M8 communication work.
