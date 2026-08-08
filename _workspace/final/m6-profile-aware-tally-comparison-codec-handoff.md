# M6 Profile-Aware Tally Comparison Codec Handoff

## Outcome

Implementation is ready for independent review at the current branch head.

## Verification target

The focused comparison regression is expected to cover canonical encoding,
verified round-trip, malformed/oversized branches, and sum-preserving
`InputMismatch` tampering within 31 focused agent tests and 244 Rust unit tests,
7 binary tests, 3 RustDoc tests, 15 Python tests, formatter, Clippy,
repository, and diff gates.

## Limits

This is provenance-bound evidence transport only. Durable export, arbitrary
report pipelines, broader metrics/distributions, outcomes, persistence,
providers, calibration, build provenance, causality, and human evidence remain
open.
