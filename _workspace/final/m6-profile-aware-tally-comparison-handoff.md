# M6 Profile-Aware Tally Comparison Handoff

## Outcome

Implementation is ready for independent review at the current branch head.

## Verification target

The focused comparison regression is expected to bind the schema, ordered
profile/rule rows, exact counts, signed deltas, repeatability, and mismatch
errors within 31 focused agent tests and 244 Rust unit tests, 7 binary tests,
3 RustDoc tests, 15 Python tests, formatter, Clippy, repository, and diff
gates.

## Limits

This is caller-declared verified selected-intent comparison evidence only. Build
provenance, causality, broader/random sampling, population distributions,
outcomes, strategic quality, persistence, providers, calibration, durable
export, and human evidence remain open.
