# M6 Sample Tally Report Design

## Goal and Roadmap Milestone

Advance M6 with one bounded aggregate selected-intent report over a verified
caller-supplied matched-scenario sample set.

## Input and provenance

`ScriptedAgentMatchedScenarioTallyReport::from_sample` accepts only the
already validated sample-set value. It does not reconstruct observations,
rerun policies, or accept independent counts.

## Output and bounds

The `m6-scripted-agent-matched-scenario-tally-v1` report retains the shared
observer, pair/observation counts, and ordered profile/rule rows. Each row
contains only bounded counts for the five closed intents; totals are at most
eight observations and must equal the report observation count.

## Authority and limits

The tally is a pure actor-safe aggregation of selected intents. It owns no
scenario generation, population/distribution sampling, outcomes, transition,
history, replay, persistence, provider, or calibration authority.

## Verification contract

The existing sample-set test asserts exact counts for cautious and yielding
rows and repeated report equality. This is fixture-sized aggregate evidence,
not a population distribution or strategic-quality metric.
