# M6 Profile-Aware Population Tally Domain QA

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

- Does the regression preserve cautious/risk-taking/yielding row order?
- Does it bind exact 7/1 cautious, 8 contest, and 8 yield counts with row sums
  of eight?
- Does the adapter remain pure aggregation over actor-visible verified samples?

## Evidence target

One focused profile-aware population tally regression should cover row IDs,
exact counts, row sums, and existing safe-heavy composition. The full gate
target is 29 focused agent tests within 242 Rust unit + 7 binary + 3 RustDoc,
15 Python tests, formatter, Clippy with warnings denied, repository checker,
and diff checks.

## Limits

The evidence is fixture-sized selected-intent plumbing only; no profile
calibration, population metric, random/distributional, outcome, strategic,
persistence, provider, or human-behavior claim is allowed.

## Required fixes

To be completed after independent review.
