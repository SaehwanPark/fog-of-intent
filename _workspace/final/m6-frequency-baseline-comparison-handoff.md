# M6 Fixed-Fixture Frequency Baseline Comparison Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentFixtureScenarioFrequencyComparisonReport` defines
`m6-scripted-agent-fixture-frequency-compare-v1` with baseline/candidate
selection totals and ordered safe/threat count deltas. It compares only
caller-declared verified reports, performs no policy execution or I/O, and
does not claim independent build provenance or causal change attribution.

## Verification target

The focused comparison test covers the exact schema and row order, 1/1 versus
2/2 totals, positive signed deltas, repeated construction, and reversed
negative deltas. The expected full evidence is one focused comparison test
within 24 focused agent tests, 237 unit tests, 7 binary tests, and 3 RustDoc
tests, plus formatter, Clippy warnings denied, repository checker, 15 Python
policy tests, and diff checks.

## Open boundaries

Independent build provenance, causal attribution, population generation,
random/distributional sampling, outcomes, strategic metrics, durable export,
persistence, providers, calibration, and human evidence remain open.
