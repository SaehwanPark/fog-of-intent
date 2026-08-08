# M6 Profile-Aware Tally Comparison Domain QA

## Disposition

Pending independent three-pass review of implementation head.

## Scope to review

- The focused comparison must bind the literal comparison schema and preserve
  cautious, risk-taking, and yielding profile/evaluation-rule row order.
- It must retain baseline/candidate pair and observation counts, expose exact
  bounded intent counts, and compute signed candidate-minus-baseline deltas.
- It must reject mismatched observers and differently ordered profile/rule rows
  without rerunning policy evaluation or adding host/lane authority.

## Evidence target

One focused profile-aware tally comparison regression is expected within 31
focused agent tests and 244 Rust unit tests, 7 binary tests, 3 RustDoc tests,
15 Python tests, formatter, Clippy with warnings denied, repository checker,
and diff checks.

## Limits

This is caller-declared verified selected-intent comparison evidence only. It
does not establish build/source provenance, causality, broader/random sampling,
population distributions, outcomes, strategic quality, persistence, providers,
calibration, durable export, or human evidence.

## Required fixes

To be determined by independent review. The bounded comparison must retain its
shared-observer and ordered-row identity checks.
