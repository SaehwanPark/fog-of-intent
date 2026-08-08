# M6 Profile-Aware Tally Comparison Codec Handoff

## Outcome

PASS — independent domain QA and final handoff review found no actionable
findings at implementation/evidence head `d9576c1`.

## Delivered contract

The codec retains the exact versioned schema, seven positional metadata lines,
ordered profile/rule rows, bounded baseline/candidate counts, closed profiles
and rules, and private expected-comparison binding. Malformed, oversized,
reordered, and sum-preserving tampered text is rejected without policy,
host/lane, persistence, or provider authority.

## Verification

The focused comparison regression covers canonical encoding, verified
round-trip, strict positional/closed parsing, swapped metadata and row
rejection, unknown profile/rule rejection, nonnumeric and out-of-range
metadata rejection, malformed/oversized branches, and sum-preserving
`InputMismatch` tampering. The full evidence is one focused comparison test
within 31 focused agent tests and 244 Rust unit tests, 7 binary tests, and 3
RustDoc tests, plus 15 Python tests, formatter, Clippy with warnings denied,
repository checker, and diff checks; all pass at reviewed head `d9576c1`.

## Limits

This is provenance-bound evidence transport only. Durable export, arbitrary
report pipelines, broader metrics/distributions, outcomes, persistence,
providers, calibration, build provenance, causality, and human evidence remain
open.
