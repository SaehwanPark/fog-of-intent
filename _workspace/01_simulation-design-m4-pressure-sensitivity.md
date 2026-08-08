# M4 Pressure-Sensitivity Simulation Design

## Boundary

The policy remains a pure adapter over a copied `LanerObservation`. The Anchor
profile reads the observation's bounded wave-pressure value as one utility
feature. The host remains the sole authority for freshness, legality,
transition, execution, history, replay, and debrief.

## Versioned rule

- Profile: `cautious-laner-v1`
- Role: `anchor-v1`
- Evaluation rule: `threat-first-pressure-aware-fixed-score-v1`
- Pressure domain: the lane observation's bounded values 0 through 3

## Evaluation contract

The visible threat response remains score 100. Anchor's `Stabilize` score is
`80 + wave_pressure`; the other non-threat fixture scores remain unchanged.
The policy selects the stable maximum in advertised order and returns the same
observer-bound `LaneIntentRequest` shape as the catalog profiles.

## Expected effect and limits

Increasing observed pressure increases only the Anchor `Stabilize` score in
this slice. At pressures 0 and 3 the selected intent remains `Stabilize` and
both requests validate. This directional score effect is not evidence of
strategic quality, balance, outcomes, or human behavior. Memory, randomness,
communication, populations, and complete role heuristics remain deferred.
