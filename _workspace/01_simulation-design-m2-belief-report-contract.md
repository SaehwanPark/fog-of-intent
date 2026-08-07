# Simulation Design — M2 Belief and Report Contract

## Goal and Boundary

Define a report-derived belief value without making belief a second simulation
state. Actor observations remain the only source of information available to an
ordinary actor.

## Contract

`LaneBelief<T>` has three explicit states: `Unknown`, `Observed { value,
observed_turn }`, and `LastKnown { value, last_seen_turn }`. Updating from a
report with a value marks it `Observed` when the sighting turn equals the
observation turn and `LastKnown` otherwise. Updating from an unknown report
retains the prior belief; memory decay is deliberately not modeled in this
bounded slice. Malformed value/turn pairs fail closed to `Unknown`.

Opponent position and jungle-threat region beliefs are derived only from their
actor-authorized reports. Health, posture, exact threat truth, state hashes, and
other latent values cannot enter the helper through the report boundary.

## Verification Contract

- Far-side opponent and river-side threat reports produce observed beliefs.
- An older explicit sighting produces a last-known belief.
- Unknown reports preserve prior belief without inventing new information.
- Player and allied redaction tests remain unchanged and observation/replay
  schemas and hashes remain stable.
