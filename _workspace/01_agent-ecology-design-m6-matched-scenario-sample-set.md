# M6 Matched Scenario Sample Set Design

## Goal and Roadmap Milestone

Advance M6 with a small composition boundary for caller-supplied matched
scenario observations, without adding a scenario generator or population
sampler.

## Input and bounds

`ScriptedAgentMatchedScenarioSample::from_observations` accepts one to four
`[LanerObservation; 2]` pairs. Every observation must belong to the same actor
and every observation ID must be unique across the complete ordered set.

## Evaluation and output

Each pair delegates to `ScriptedAgentMatchedSample`, which delegates each
observation to the existing seeded batch runner. The returned report stores the
fixed schema, shared observer, and nested pair reports in caller order. It does
not expose true state, execution inputs, histories, outcomes, or providers.

## Authority and reproducibility

The sample set is pure in-process metadata. It owns no scenario selection,
transition, history, replay, persistence, population, or distributional
authority. Pair order, observation order, manifest order, and explicit policy
seeds are the only reproducibility inputs.

## Verification contract

One focused agent test covers two ordered pairs, repeated equality, nested
observation-ID order, and empty/over-capacity/mixed-actor/global-duplicate-ID
rejection. The full repository gates remain the evidence boundary.

## Open boundaries

Scenario generation, population sampling, distributional metrics, outcomes,
representative replays, persistence, providers, calibration, and human
behavior remain open.
