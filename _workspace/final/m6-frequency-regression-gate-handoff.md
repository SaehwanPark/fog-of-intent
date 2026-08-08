# M6 Fixed-Fixture Frequency Regression Gate Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentFixtureScenarioFrequencyComparisonReport` exposes the fixed rule
`m6-fixed-frequency-no-change-v1`. It passes only when baseline and candidate
selection totals and both ordered safe/threat counts are identical. The
rationale is limited to deterministic fixed-fixture baseline mismatch
detection; no independent build or causal claim is made.

## Verification target

The focused comparison test covers the exact rule ID, changed 1/1-to-2/2 gate
failure, and unchanged gate success. The expected full evidence is one focused
comparison test within 24 focused agent tests, 237 unit tests, 7 binary tests,
and 3 RustDoc tests, plus formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks.

## Open boundaries

Independent build provenance, causal attribution, broader threshold rationale,
population generation, random/distributional sampling, outcomes, strategic
metrics, durable export, persistence, providers, calibration, and human
evidence remain open.
