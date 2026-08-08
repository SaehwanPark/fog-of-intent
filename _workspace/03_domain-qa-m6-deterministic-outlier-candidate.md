# M6 Deterministic Outlier-Candidate Domain QA

## Disposition

Pending independent three-pass review of implementation head.

## Scope to review

- The candidate must retain the literal schema and selection-rule IDs.
- Largest absolute signed deltas must use stable profile-row order followed by
  `[Stabilize, Contest, Yield, Recall, Withdraw]` intent order for ties.
- Zero-delta comparisons must return no candidate, and the projection must
  remain metric-side caller-declared metadata with no replay or outlier
  authority.

## Evidence target

One focused agent regression should prove positive and negative signed deltas,
the tied first candidate, magnitude equality, repeated construction, and the
all-zero `None` result. Expected full evidence is 33 focused agent tests within
246 Rust unit tests, 7 binary tests, 3 RustDoc tests, 15 Python tests,
formatter, Clippy with warnings denied, repository checker, and diff checks.

## Limits

This is deterministic fixed-fixture comparison evidence only. Actual outlier
definitions, threshold calibration, representative replay selection, causal
importance, population inference, persistence, providers, and human evidence
remain open.

## Required fixes

To be determined by independent review. The candidate must remain bounded,
reproducible, and non-authoritative.
