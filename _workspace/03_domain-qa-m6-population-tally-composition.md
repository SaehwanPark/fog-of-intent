# M6 Population-to-Tally Composition Domain QA

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

- Does `matched_tally` reuse the verified sample/tally path without policy
  reruns or new authority?
- Does safe-heavy composition produce pair count 4, observation count 8, one
  row, and exact 7 Stabilize/1 Withdraw counts?
- Are docs explicit that this is fixture-sized selected-intent evidence rather
  than broader population metrics or outcomes?

## Evidence target

The existing focused composition regression should cover direct tally output,
full matched-sample equality, ordered 3/1 composition, and the existing public
constructor error precedence. The full gate target is 28 focused agent tests
within 241 Rust unit + 7 binary + 3 RustDoc, 15 Python tests, formatter,
Clippy with warnings denied, repository checker, and diff checks.

## Limits

No broader/random population sampling, distributional/outcome/strategic metric,
persistence, provider/calibration, or human-behavior claim is allowed.

## Required fixes

To be completed after independent review.
