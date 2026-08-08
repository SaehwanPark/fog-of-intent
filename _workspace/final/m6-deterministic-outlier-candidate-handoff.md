# M6 Deterministic Outlier-Candidate Handoff

## Outcome

PASS — no actionable findings remain after independent three-pass review at
implementation/evidence head `90f0201`.

## Verification

The focused agent regression binds the literal candidate schema and rule,
selects the first largest absolute signed delta with stable row/intent ties,
preserves positive and negative signs, proves magnitude equality and repeated
construction, and returns no candidate for an unchanged comparison. The full
evidence is 33 focused agent tests within 246 Rust unit tests, 7 binary tests,
3 RustDoc tests, 15 Python tests, formatter, Clippy, repository, and diff
gates; all pass at `90f0201`.

## Limits

This is deterministic caller-declared metric evidence only. Actual outlier
detection, threshold calibration, representative replay selection, causal
attribution, population inference, persistence, providers, and human evidence
remain open.
