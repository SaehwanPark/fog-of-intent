# M6 Candidate Replay Reference Domain QA

## Disposition

PASS — no actionable findings after independent three-pass review at
implementation/evidence head `b17e244`.

## Scope reviewed

- The reference binds exact schema/rule and caller-declared first-match order.
- Profile/rule/intent matching returns only candidate labels and observation
  ID after deterministic replay verification.
- Matching replay mismatch and no matching replay have separate bounded errors.
- No representative, causal, scenario-wide, host/lane, persistence, provider,
  or human-evidence authority is added.

## Evidence

One focused agent regression, 37 focused agent tests within 250 Rust unit
tests, 7 binary tests, and 3 RustDoc tests, 15 Python tests, formatter, Clippy
with warnings denied, repository checker, and diff checks all pass at
`b17e244`. The regression proves first valid ordering, mismatch-then-later-
verified selection, terminal mismatch, and no-match handling.

## Limits

This slice is caller-declared candidate-to-replay reference evidence only.
Representative-replay proof, scenario-wide replay, calibrated outlier
detection, build provenance, causality, persistence, providers, and human
evidence remain open.

## Required fixes

None. The reference remains caller-declared, reproducible, and
non-authoritative.
