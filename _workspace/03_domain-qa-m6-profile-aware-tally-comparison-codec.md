# M6 Profile-Aware Tally Comparison Codec Domain QA

## Disposition

Pending independent three-pass review of implementation head.

## Scope to review

- The comparison codec must retain the exact versioned schema, seven metadata
  lines, ordered profile/rule rows, and bounded baseline/candidate counts.
- Decode must enforce closed profiles/rules and row totals, then compare its
  private candidate with the expected verified comparison before returning it.
- Malformed, oversized, reordered, and sum-preserving tampered text must fail
  without policy, host, lane, persistence, or provider authority.

## Evidence target

The existing focused profile-aware comparison regression is expected to cover
canonical encoding, verified round-trip, malformed/oversized branches, and
InputMismatch tampering within 31 focused agent tests and 244 Rust unit tests,
7 binary tests, 3 RustDoc tests, 15 Python tests, formatter, Clippy with
warnings denied, repository checker, and diff checks.

## Limits

This is provenance-bound evidence transport only. Durable export, arbitrary
report pipelines, broader metrics/distributions, outcomes, persistence,
providers, calibration, build provenance, causality, and human evidence remain
open.

## Required fixes

To be determined by independent review. The codec must remain bounded,
positional, closed, and bound to a verified comparison.
