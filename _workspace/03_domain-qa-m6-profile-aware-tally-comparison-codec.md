# M6 Profile-Aware Tally Comparison Codec Domain QA

## Disposition

PASS — independent three-pass review found no actionable findings at
implementation/evidence head `d9576c1`.

## Scope reviewed

- The comparison codec must retain the exact versioned schema, seven metadata
  lines, ordered profile/rule rows, and bounded baseline/candidate counts.
- Decode must enforce closed profiles/rules and row totals, then compare its
  private candidate with the expected verified comparison before returning it.
- Malformed, oversized, reordered, and sum-preserving tampered text must fail
  without policy, host, lane, persistence, or provider authority.

## Evidence

The focused profile-aware comparison regression covers canonical encoding,
verified round-trip, strict positional/closed parsing, swapped metadata and
row rejection, unknown profile/rule rejection, nonnumeric and out-of-range
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

## Required fixes

None. The codec remains bounded, positional, closed, and bound to a verified
comparison.
