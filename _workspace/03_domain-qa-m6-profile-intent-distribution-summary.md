# M6 Profile Intent Distribution Summary Domain QA

## Disposition

Pending independent three-pass review of implementation head.

## Scope to review

- Existing tally schema, profile/rule order, counts, and codec behavior remain
  unchanged.
- Each verified row exposes five ordered intent shares at the literal
  10,000-point scale, with a deterministic Withdraw remainder.
- Markdown contains only actor-safe profile/rule labels, bounded counts, and
  ordered shares; it adds no policy, host, lane, history, persistence, or
  provider authority.

## Evidence target

One focused profile-aware tally regression should cover exact cautious 7/1,
risk-taking 8/0, and yielding 8/0 rows, stable profile/rule order, exact
shares, 10,000-point row sums, and complete exact Markdown. Expected full
evidence is 31 focused agent tests within 244 Rust unit tests, 7 binary tests,
3 RustDoc tests, 15 Python tests, formatter, Clippy with warnings denied,
repository checker, and diff checks.

## Limits

This is a bounded selected-intent projection over verified fixture tallies.
Broader population generation/distributions, outcomes, strategic metrics,
durable export, persistence, providers, calibration, and human evidence remain
open.

## Required fixes

To be determined by independent review. The projection must remain pure,
deterministic, ordered, and denominator-bound.
