# Domain QA — M6 Matched Observation Sample

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope reviewed

The slice adds `m6-scripted-agent-matched-sample-v1`, a library-only sample
over exactly two caller-supplied, same-actor observations with distinct IDs.
It reuses the existing ordered seeded batch runner and returns only bounded
profile/rule/seed labels plus selected intents. It does not generate
populations, sample distributions, calculate outcomes or metrics, persist
samples, or acquire host/lane/provider authority.

## Evidence target

One focused agent test covers visible-threat sensitivity, repeated equality,
stable observation/manifest/seed ordering, and mixed-actor, duplicate-ID,
empty-batch, and over-capacity rejection. The expected full evidence is 19
focused agent tests within 232 Rust unit tests, 7 binary tests, and 3 RustDoc
tests, plus formatter, Clippy warnings denied, repository checker, 15 Python
policy tests, and diff checks.

## Review limits

This evidence is fixture-sized policy-input sensitivity only. It does not claim
population generation, distributional sampling, strategic quality, outcome
balance, metrics, persistence, providers, calibration, or human behavior.
