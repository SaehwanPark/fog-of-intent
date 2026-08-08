# M6 Profile-Aware Tally Regression-Gate Domain QA

## Disposition

PASS: the independent three-pass review found no actionable findings at
implementation/evidence head `aa2d878`.

## Scope reviewed

- The comparison exposes the literal
  `m6-fixed-profile-tally-no-change-v1` rule ID.
- The gate passes unchanged verified reports and rejects both the 4/8→3/6
  changed-total and 4/8→4/8 same-total row-redistribution comparisons.
- The predicate compares top-level pair/observation counts and every ordered
  profile row's five intent counts without adding policy or host authority.

## Evidence

One focused profile-aware tally comparison regression covers the literal rule,
unchanged success, 4/8→3/6 changed-total failure, 4/8→4/8 same-total
redistribution failure, and prior identity checks. The full evidence is 31
focused agent tests within 244 Rust unit tests, 7 binary tests, 3 RustDoc
tests, 15 Python tests, formatter, Clippy with warnings denied, repository
checker, and diff checks, all passing at `aa2d878`.

## Limits

This is a provisional fixed-fixture equality signal only. It does not establish
broader thresholds, balance, build/source provenance, causality, random or
representative sampling, population distributions, outcomes, strategic
quality, persistence, providers, calibration, durable export, or human
evidence.

## Required fixes

None. The gate remains a pure equality predicate over verified comparison
fields.
