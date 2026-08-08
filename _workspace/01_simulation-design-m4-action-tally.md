# M4 Action-Tally Simulation Design

## Boundary

`ScriptedAgentActionTallyReport` is a pure metric adapter over two copied
`LanerObservation` values. It invokes the existing policy choices but does not
resolve execution, validate transitions, mutate history, or access true state.
The host remains the sole authority for all simulation lifecycle operations.

## Versioned contract

- Schema: `m4-scripted-agent-action-tally-v2` (v1 is the historical
  count-only contract)
- Observation count: exactly two, with distinct retained observation IDs
- Observer contract: both observations must have the same actor identity
- Fields: profile ID, evaluation-rule ID, observer, observation count, the two
  observation IDs, and counts for each of the five lane intents

## Expected fixture tally

Across safe and RiverSide observations, the cautious profile selects
`Stabilize` once and `Withdraw` once; risk-taking selects `Contest` twice; and
yielding selects `Yield` twice. Each underlying request remains host-valid.

## Evidence limits

This is a two-observation action tally with visible input-ID binding, not a
population distribution, outcome metric, strategic-quality measure, or
human-behavior result. Memory,
communication, randomness, execution metrics, and broader scenario sampling
remain deferred.
