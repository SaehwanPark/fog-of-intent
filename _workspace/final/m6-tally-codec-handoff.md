# M6 Selected-Intent Tally Codec Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentMatchedScenarioTallyReport::encode/decode` uses the bounded
`m6-scripted-agent-matched-scenario-tally-v1` line-oriented shape with a
4096-byte bound, fixed top-level metadata, and ordered pipe-delimited rows.
Decode validates closed profile/rule identities, count totals, exact line
count, and malformed/unknown/duplicate/missing/extra fields, then compares the
candidate with an already verified report before returning actor-safe evidence.
It is not durable export or a report pipeline.

## Verification target

The focused matched-scenario sample-set test covers canonical codec text,
round trip, malformed-input classes, and oversized input, alongside the full
21-agent-focused / 234-unit, 7-binary, 3-RustDoc suite, formatter, Clippy
warnings denied, repository checker, 15 Python policy tests, and diff checks.

## Open boundaries

Durable export/report pipelines, population/distributional sampling, outcomes,
strategic metrics, persistence, providers, calibration, representative
replays, and human evidence remain open.
