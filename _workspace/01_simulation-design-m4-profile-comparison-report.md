# M4 Profile-Comparison Report Design

## Boundary

`ScriptedAgentComparisonReport::from_observation` runs the existing three pure
profiles over one copied `LanerObservation`. It reports policy outputs only;
the host remains the sole legality, transition, execution, replay, and history
authority.

## Versioned schema

`m4-scripted-agent-metrics-v1` contains the observer and observation ID plus
three ordered rows. Each row carries profile ID, evaluation-rule ID, selected
intent, selected score, and a `u8` candidate count. No row carries state,
hashes, execution inputs, or raw failures.

## Evidence and limits

The report is reproducible for an identical observation and preserves catalog
order. It is a bounded metric/schema plumbing result, not an action-distribution
study, outcome report, strategic evaluation, population comparison, or human
behavioral result.
