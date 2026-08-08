# M6 Population-to-Tally Composition Domain QA

## Disposition

PASS: the independent three-pass review found no actionable findings at
implementation/evidence head `d4b535b`.

## Scope reviewed

- `matched_tally` reuses the verified sample/tally path without policy reruns or
  new authority.
- Safe-heavy composition produces pair count 4, observation count 8, one row,
  and exact 7 Stabilize/1 Withdraw counts.
- The docs explicitly limit this to fixture-sized selected-intent evidence
  rather than broader population metrics or outcomes.

## Evidence

The existing focused composition regression covers direct tally output, full
matched-sample equality, ordered 3/1 composition, and the existing public
constructor error precedence. The full evidence is 28 focused agent tests
within 241 Rust unit + 7 binary + 3 RustDoc, 15 Python tests, formatter,
Clippy with warnings denied, repository checker, and diff checks, all passing
at `d4b535b`.

## Limits

No broader/random population sampling, distributional/outcome/strategic metric,
persistence, provider/calibration, or human-behavior claim is allowed.

## Required fixes

None. Broader population metrics, random/distributional sampling, outcomes,
strategic quality, persistence, providers/calibration, and human evidence
remain open.
