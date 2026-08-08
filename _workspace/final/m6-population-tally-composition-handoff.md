# M6 Population-to-Tally Composition Handoff

## Outcome

Implementation and evidence are pending independent three-pass review at the
implementation/evidence head.

## Verification target

The focused composition regression should cover direct tally output, pair and
observation bounds, exact 7 Stabilize/1 Withdraw counts, complete sample
equality, ordered 3/1 composition, and constructor error precedence. The full
target is 28 focused agent tests within 241 Rust unit + 7 binary + 3 RustDoc
tests, plus 15 Python tests and formatter, Clippy, repository, and diff gates.

## Limits

This is fixture-sized selected-intent composition only. Broader population
metrics, random/distributional sampling, outcomes, strategic quality,
persistence, providers/calibration, and human evidence remain open.
