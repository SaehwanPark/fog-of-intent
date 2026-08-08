# M6 Profile-Aware Tally Regression-Gate Handoff

## Outcome

Implementation and evidence are complete at head `aa2d878`; the independent
three-pass review passed with no actionable findings.

## Verification

One focused comparison regression binds the literal rule ID, proves unchanged
success, rejects the 4/8→3/6 changed-total comparison, and rejects the 4/8→4/8
same-total redistribution comparison. The full evidence is 31 focused agent
tests within 244 Rust unit tests, 7 binary tests, 3 RustDoc tests, and 15
Python tests; formatter, Clippy, repository, and diff gates pass at `aa2d878`.

## Limits

This is a provisional fixed-fixture equality signal only. Broader thresholds,
balance, build/source provenance, causality, random or representative
sampling, population distributions, outcomes, strategic quality, persistence,
providers, calibration, durable export, and human evidence remain open.
