# M6 Caller-Declared Population Composition Handoff

## Outcome

Implementation and evidence are pending independent three-pass review at the
implementation/evidence head.

## Verification target

One focused fixture-selection regression should cover safe-heavy ordered
composition, exact 3/1 frequency counts, complete matched-sample composition,
unknown-ID failure, and the existing alternating/bounds/overflow evidence. The
full target is 28 focused agent tests within 241 Rust unit + 7 binary + 3
RustDoc tests, plus 15 Python tests and formatter, Clippy, repository, and diff
gates.

## Limits

This is caller-declared fixed-fixture composition only. Random/representative
sampling, broader distributions, outcomes, strategic metrics, persistence,
providers/calibration, and human evidence remain open.
