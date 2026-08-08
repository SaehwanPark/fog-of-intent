# M6 Caller-Declared Population Composition Domain QA

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

- Does the helper accept only the two closed IDs, preserve caller order, derive
  checked sequential observation pairs, and retain the four-entry bound?
- Does safe-heavy composition produce exact explicit 3/1 frequency evidence?
- Does matched-sample composition reuse existing actor-visible validation with
  no policy, host, lane, history, replay, persistence, provider, or outcome
  authority?

## Evidence target

One focused fixture-selection regression should cover safe-heavy ordering, exact
3/1 frequency counts, complete matched-sample equality, and unknown-ID failure,
alongside the existing alternating/bounds/overflow evidence. The full gate
target is 28 focused agent tests within 241 Rust unit + 7 binary + 3 RustDoc,
15 Python tests, formatter, Clippy with warnings denied, repository checker,
and diff checks.

## Limits

The composition is explicit fixed-fixture input. It must not claim random,
representative, broader population, distributional, outcome, strategic,
persistence, provider, calibration, or human-behavior evidence.

## Required fixes

To be completed after independent review.
