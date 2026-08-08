# M6 Profile-Aware Population Tally Domain QA

## Disposition

PASS: the independent three-pass review found no actionable findings at
implementation/evidence head `ea01976`.

## Scope reviewed

- The regression preserves cautious/risk-taking/yielding row order.
- It binds exact 7/1 cautious, 8 contest, and 8 yield counts with row sums of
  eight.
- The adapter remains pure aggregation over actor-visible verified samples.

## Evidence

One focused profile-aware population tally regression covers row IDs, exact
counts, row sums, and existing safe-heavy composition. The full evidence is 29
focused agent tests within 242 Rust unit + 7 binary + 3 RustDoc, 15 Python
tests, formatter, Clippy with warnings denied, repository checker, and diff
checks, all passing at `ea01976`.

## Limits

The evidence is fixture-sized selected-intent plumbing only; no profile
calibration, population metric, random/distributional, outcome, strategic,
persistence, provider, or human-behavior claim is allowed.

## Required fixes

None. Profile calibration, broader population metrics, random/distributional
sampling, outcomes, strategic quality, persistence, providers, and human
evidence remain open.
