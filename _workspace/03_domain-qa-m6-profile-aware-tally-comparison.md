# M6 Profile-Aware Tally Comparison Domain QA

## Disposition

PASS: the independent three-pass review found no actionable findings at
implementation/evidence head `f9b7cde`.

## Scope reviewed

- The focused comparison binds the literal schema and preserves cautious,
  risk-taking, and yielding profile/evaluation-rule row order.
- It retains baseline/candidate pair and observation counts, exposes exact
  bounded intent counts, and computes signed candidate-minus-baseline deltas.
- It rejects mismatched observers and differently ordered profile/rule rows
  without rerunning policy evaluation or adding host/lane authority.

## Evidence

One focused profile-aware tally comparison regression covers schema, ordered
profile/rule rows, exact counts, signed/reversed deltas, repeatability, and
mismatch errors. The full evidence is 31 focused agent tests within 244 Rust
unit tests, 7 binary tests, 3 RustDoc tests, 15 Python tests, formatter,
Clippy with warnings denied, repository checker, and diff checks, all passing
at `f9b7cde`.

## Limits

This is caller-declared verified selected-intent comparison evidence only. It
does not establish build/source provenance, causality, broader/random sampling,
population distributions, outcomes, strategic quality, persistence, providers,
calibration, durable export, or human evidence.

## Required fixes

None. The bounded comparison retains its shared-observer and ordered-row
identity checks.
