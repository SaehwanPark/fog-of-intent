# M6 Replay-Sequence Evidence Domain QA

## Disposition

Pending independent three-pass review.

## Scope to review

- The report binds exact schema/rule and independent replay/sequence IDs.
- Complete and incomplete caller-declared operational logs remain read-only.
- A tampered recorded decision returns `decision_mismatch` without changing
  the sequence status.
- No causal, runtime, persistence, provider, or host/lane authority is added.

## Evidence target

One focused agent regression, 35 focused agent tests within 248 Rust unit
tests, 7 binary tests, and 3 RustDoc tests, 15 Python tests, formatter, Clippy
with warnings denied, repository checker, and diff checks must pass.

## Limits

This slice is bounded decision-replay and operational-label evidence only. It
does not establish causal-trace completeness, runtime event production,
scenario-wide replay identity, persistence, providers, or human evidence.

## Required fixes

To be determined by independent review.
