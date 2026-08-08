# Domain QA — M6 Build-Labeled Comparison

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

The slice adds optional distinct caller-declared numeric build labels to the
existing verified fixed-fixture frequency comparison. It must preserve the
unlabeled constructor, ordered deltas, and fixed no-change gate while adding no
source/package verification, causality, policy, scenario, transition,
history, replay, persistence, provider, population, or outcome authority.

## Evidence target

One focused comparison test must bind the build-label schema, prove distinct
baseline/candidate IDs survive construction, retain safe-then-RiverSide row
order and signed deltas, repeat deterministically, and reject matching IDs.
The expected full evidence is 25 focused agent tests within 238 Rust unit
tests, 7 binary tests, and 3 RustDoc tests, plus formatter, Clippy warnings
denied, repository checker, 15 Python policy tests, and diff checks.

## Review limits

Labels are caller declarations only. Independent binary/source/package
verification, causal attribution, durable export, population/distributional
sampling, outcome/strategic metrics, providers, calibration, and human
evidence remain open.
