# M6 Matched Scenario Sample Set Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentMatchedScenarioSample` exposes the bounded
`m6-scripted-agent-matched-scenarios-v1` schema for one to four
caller-supplied matched pairs. It requires a shared actor and globally unique
observation IDs, composes existing matched reports in pair order, and remains
in-process metadata with no scenario, population, distribution, outcome,
transition, history, provider, or persistence authority.

## Verification target

One focused matched-scenario sample-set test plus the full 21-agent-focused /
234-unit, 7-binary, 3-RustDoc suite, formatter, Clippy warnings denied,
repository checker, 15 Python policy tests, and diff checks.

## Open boundaries

Scenario generation/selection, population and distributional sampling,
outcomes, metrics, representative replays, persistence, providers,
calibration, and human evidence remain open.
