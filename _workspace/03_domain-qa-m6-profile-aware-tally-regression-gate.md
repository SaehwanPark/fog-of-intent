# M6 Profile-Aware Tally Regression-Gate Domain QA

## Disposition

Pending independent three-pass review of implementation head.

## Scope to review

- The comparison exposes the literal
  `m6-fixed-profile-tally-no-change-v1` rule ID.
- The gate passes unchanged verified reports and rejects both changed-total and
  same-total row-redistribution comparisons.
- The predicate compares top-level pair/observation counts and every ordered
  profile row's five intent counts without adding policy or host authority.

## Evidence target

One focused profile-aware tally comparison regression is expected within 31
focused agent tests and 244 Rust unit tests, 7 binary tests, 3 RustDoc tests,
15 Python tests, formatter, Clippy with warnings denied, repository checker,
and diff checks.

## Limits

This is a provisional fixed-fixture equality signal only. It does not establish
broader thresholds, balance, build/source provenance, causality, random or
representative sampling, population distributions, outcomes, strategic
quality, persistence, providers, calibration, durable export, or human
evidence.

## Required fixes

To be determined by independent review. The gate must remain a pure equality
predicate over verified comparison fields.
