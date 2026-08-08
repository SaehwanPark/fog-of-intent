# M6 Fixed-Fixture Population Domain QA

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

- Does the generator bind the exact population schema, closed alternating
  scenario IDs, the caller-supplied starting ID and checked derived pairs, and
  the inclusive four-entry bound?
- Are empty, over-capacity, and observation-ID overflow cases rejected without
  introducing randomness or hidden inputs?
- Does matched-sample composition reuse existing actor-visible validation while
  keeping policy, transition, history, replay, persistence, provider, outcome,
  and population authority outside this adapter?

## Evidence target

One focused fixture-selection regression should cover literal schema and
alternating order, exact observation IDs, deterministic repetition, successful
matched-sample composition, empty/over-capacity failures, and the inclusive
maximum observation-ID boundary plus overflow rejection. The full gate target
is 27 focused agent tests within 240 Rust unit + 7 binary + 3 RustDoc tests,
15 Python tests, formatter, Clippy with warnings denied, repository checker,
and diff checks.

## Limits

The slice must not claim random/broad population sampling, representative
coverage, distributions, outcomes, strategic quality, persistence, provider
behavior, or human realism.

## Required fixes

To be completed after independent review.
