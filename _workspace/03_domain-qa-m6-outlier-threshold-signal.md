# M6 Outlier-Threshold Signal Domain QA

## Disposition

Pending independent three-pass review.

## Scope to review

- The report binds exact schema, rule, threshold, and three status IDs.
- Magnitudes 2 and 1 classify above and below the inclusive threshold.
- An unchanged verified comparison returns `no_candidate`.
- The projection is pure, fixed-fixture, and adds no outlier or replay
  authority.

## Evidence target

One focused agent regression, 36 focused agent tests within 249 Rust unit
tests, 7 binary tests, and 3 RustDoc tests, 15 Python tests, formatter, Clippy
with warnings denied, repository checker, and diff checks must pass.

## Limits

This slice is provisional signed-delta threshold evidence only. Calibrated
outlier detection, representative replay selection, causal attribution,
population inference, persistence, providers, and human evidence remain open.

## Required fixes

To be determined by independent review.
