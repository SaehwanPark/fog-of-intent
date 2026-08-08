# M6 Sample Tally Report Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentMatchedScenarioTallyReport` exposes the bounded
`m6-scripted-agent-matched-scenario-tally-v1` report over a verified sample
set. It preserves shared observer, pair/observation counts, profile/rule order,
and counts for the five closed intents, with totals bounded by eight
observations. It is pure in-process aggregation and has no policy, scenario,
population, outcome, persistence, or provider authority.

## Verification target

The focused matched-scenario sample-set test now covers exact tally rows and
repeatability, alongside the full 21-agent-focused / 234-unit, 7-binary,
3-RustDoc suite, formatter, Clippy warnings denied, repository checker, 15
Python policy tests, and diff checks.

## Open boundaries

Population/distributional sampling, scenario generation, outcomes, strategic
metrics, representative replays, persistence, providers, calibration, and
human evidence remain open.
