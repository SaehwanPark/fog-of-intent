# M6 Selected-Intent Tally Codec Handoff

## Outcome

PASS — independent domain QA and final handoff review found no actionable
findings at implementation/evidence head `af504cb`.

## Delivered contract

`ScriptedAgentMatchedScenarioTallyReport::encode/decode` uses the bounded
`m6-scripted-agent-matched-scenario-tally-v1` line-oriented shape with a
4096-byte bound, fixed top-level metadata, and ordered pipe-delimited rows.
Decode validates closed profile/rule identities, count totals, exact line
count, and malformed/unknown/duplicate/missing/extra fields, then compares the
candidate with an already verified report before returning actor-safe evidence.
It is not durable export or a report pipeline.

## Verification

The focused matched-scenario sample-set test covers canonical codec text,
round trip, malformed-input classes, oversized input, observer/count provenance
tampering, and both inclusive pair/row maxima. The full evidence is one
focused sample-set test within 21 focused agent tests, 234 unit tests, 7 binary
tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks; all pass at reviewed head
`af504cb`.

## Open boundaries

Durable export/report pipelines, population/distributional sampling, outcomes,
strategic metrics, persistence, providers, calibration, representative
replays, and human evidence remain open.
