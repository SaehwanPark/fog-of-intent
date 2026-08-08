# Domain QA — M6 Matched Observation Sample

## Disposition

PASS — no actionable findings remain after the independent three-pass review
at implementation/evidence head `2b135e9`.

## Scope reviewed

The slice adds `m6-scripted-agent-matched-sample-v1`, a library-only sample
over exactly two caller-supplied, same-actor observations with distinct IDs.
It reuses the existing ordered seeded batch runner and returns only bounded
profile/rule/seed labels plus selected intents. It does not generate
populations, sample distributions, calculate outcomes or metrics, persist
samples, or acquire host/lane/provider authority.

## Evidence

One focused agent test covers visible-threat sensitivity, repeated equality,
stable observation/manifest/seed ordering, and mixed-actor, duplicate-ID,
empty-batch, and over-capacity rejection. The full evidence is 19 focused
agent tests within 232 Rust unit tests, 7 binary tests, and 3 RustDoc tests,
plus formatter, Clippy warnings denied, repository checker, 15 Python policy
tests, and diff checks; all pass at the reviewed head.

## Review limits

This evidence is fixture-sized policy-input sensitivity only. It does not claim
population generation, distributional sampling, strategic quality, outcome
balance, metrics, persistence, providers, calibration, or human behavior.
