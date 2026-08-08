# M6 Deterministic Outlier-Candidate Domain QA

## Disposition

PASS — no actionable findings after independent three-pass review at
implementation/evidence head `90f0201`.

## Scope reviewed

- The candidate retains the literal schema and selection-rule IDs.
- Largest absolute signed deltas use stable profile-row order followed by
  `[Stabilize, Contest, Yield, Recall, Withdraw]` intent order for ties.
- Zero-delta comparisons return no candidate, and the projection remains
  metric-side caller-declared metadata with no replay or outlier authority.

## Evidence

One focused agent regression proves positive and negative signed deltas, the
tied first candidate across both duplicate profile rows and intent positions,
magnitude equality, repeated construction, and the all-zero `None` result.
The full evidence is 33 focused agent tests within 246 Rust unit tests, 7
binary tests, 3 RustDoc tests, 15 Python tests, formatter, Clippy with warnings
denied, repository checker, and diff checks; all pass at `90f0201`.

## Limits

This is deterministic fixed-fixture comparison evidence only. Actual outlier
definitions, threshold calibration, representative replay selection, causal
importance, population inference, persistence, providers, and human evidence
remain open.

## Required fixes

None. The candidate remains bounded, reproducible, and non-authoritative.
