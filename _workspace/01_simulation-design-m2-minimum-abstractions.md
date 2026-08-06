# Simulation Design — M2 Minimum State Abstractions

## Goal and Boundary

Record the current M2 state contract needed to evaluate one lane decision
window. The contract is already implemented; this artifact makes its scope and
limits explicit for the roadmap promotion.

## Typed Contract

- `LanePosition` identifies `NearTower`, `Center`, or `FarSide`.
- `LaneHealth` is bounded and belongs to player/opponent lane state.
- `WavePressure` is bounded and belongs to `WaveState`.
- `LaneResources` groups bounded `LaneMana`, `LaneCooldown`, `LaneGold`, and
  `LaneExperience` for the player; execution deltas remain explicit inputs.
- `LaneSnapshot` owns the host-visible truth and state hash; observations expose
  only authorized player/allied projections.

## Authority and Compatibility

No new state or hash field is introduced by this documentation slice. Existing
M2 v2 ruleset, observation, replay, and record identities remain unchanged.
The current implementation's resource aggregation and lifecycle invariants are
the evidence for the checklist item.

## Verification Contract

Use the existing state/resource/transition/observation/history/replay tests and
the full locked repository checks. Do not interpret this promotion as evidence
of a complete economy, balance, or playable scenario.
