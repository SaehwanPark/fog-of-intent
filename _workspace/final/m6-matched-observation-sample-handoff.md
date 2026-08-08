# M6 Matched Observation Sample Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentMatchedSample` exposes the versioned
`m6-scripted-agent-matched-sample-v1` schema for exactly two same-actor,
distinct-ID observations and an ordered, bounded manifest list. It delegates
policy evaluation to the existing seeded batch runner and returns only
actor-safe observer/observation IDs, profile/evaluation labels, explicit seeds,
and selected intents. The adapter is in-process and has no transition,
history, provider, population, metrics, or persistence authority.

## Verification target

One focused agent sample test plus the full 19-agent-focused / 232-unit,
7-binary, 3-RustDoc suite, formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks.

## Open boundaries

Population generation and distribution sampling, outcome and metric reports,
durable persistence, providers, calibration, representative replays, and
human-behavior evidence remain open.
