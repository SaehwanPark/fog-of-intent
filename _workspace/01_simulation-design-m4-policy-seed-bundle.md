# M4 Policy Seed-Bundle Simulation Design

## Boundary

`ScriptedAgentSeedBundle` is an explicit policy-edge input. It contains a
versioned contract identity, a caller-provided seed, and one policy
`InputTrace` composed of `StreamId` and `DrawId`. It is not derived from true
state, wall time, an implicit global generator, or transition execution.

## Contract

- `m4-scripted-agent-random-v1` identifies the seed-bundle contract.
- `max-score-seeded-tie-v1` is used only by opt-in
  `ScriptedAgent::choose_with_seed`.
- The seeded path computes a deterministic SplitMix64-derived index from the
  seed and policy stream/draw, then selects only among equal top-score
  candidates.
- The default `ScriptedAgent::choose` path remains
  `max-score-stable-order-v1`, retaining the first advertised equal maximum.
- Seeded decisions retain the bundle and seeded selection-rule identity so a
  caller can preserve the exact policy input for later reproduction.

## Authority and information

The agent still consumes only `LanerObservation` and returns an
observer-bound `LaneIntentRequest`. The host validates freshness and legality;
the lane/kernel evaluates transitions; neither the seed bundle nor the agent
selects execution inputs or mutates history.

## Limits

The slice demonstrates reproducible tie resolution only. It does not establish
random policy quality, broad distributions, top-k/nucleus behavior, population
sampling, scenario outcomes, strategic diversity, or human realism.
