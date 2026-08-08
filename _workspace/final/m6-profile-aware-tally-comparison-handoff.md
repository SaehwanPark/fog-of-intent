# M6 Profile-Aware Tally Comparison Handoff

## Outcome

Implementation and evidence are complete at head `f9b7cde`; the independent
three-pass review passed with no actionable findings.

## Verification

One focused comparison regression binds the schema, ordered profile/rule rows,
exact counts, signed and reversed deltas, repeatability, and mismatch errors.
The full evidence is 31 focused agent tests within 244 Rust unit tests, 7
binary tests, 3 RustDoc tests, and 15 Python tests; formatter, Clippy,
repository, and diff gates pass at `f9b7cde`.

## Limits

This is caller-declared verified selected-intent comparison evidence only. Build
provenance, causality, broader/random sampling, population distributions,
outcomes, strategic quality, persistence, providers, calibration, durable
export, and human evidence remain open.
