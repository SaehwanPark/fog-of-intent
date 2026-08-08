# M6 Fixed-Fixture Population Handoff

## Outcome

Implementation and evidence are pending independent three-pass review at the
implementation/evidence head.

## Verification target

One focused fixture-selection regression should cover the literal population
schema, deterministic alternating four-entry output, ordered IDs derived from
the caller-supplied starting ID, matched-sample composition,
empty/over-capacity failures, and inclusive observation-ID bounds. The full
target is 27 focused agent tests within 240
Rust unit + 7 binary + 3 RustDoc tests, plus 15 Python tests and formatter,
Clippy, repository, and diff gates.

## Limits

This is a fixed-fixture, in-process generator only. Broader/random population
sampling, distributional evidence, representative replays, outcomes,
strategic metrics, persistence, providers/calibration, and human evidence
remain open.
