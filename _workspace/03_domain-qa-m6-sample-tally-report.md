# Domain QA — M6 Sample Tally Report

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope reviewed

The slice adds `m6-scripted-agent-matched-scenario-tally-v1`, an aggregation
over an already verified caller-supplied matched-scenario sample set. It
retains only shared observer, pair/observation counts, ordered profile/rule
rows, and bounded selected-intent counts. It does not rerun policies, generate
scenarios or populations, sample distributions, inspect outcomes, persist
reports, or acquire transition/history/provider authority.

## Evidence target

The existing focused matched-scenario sample-set test now asserts exact
cautious/yielding profile and rule counts plus repeated tally equality. The
expected full evidence is 21 focused agent tests within 234 Rust unit tests, 7
binary tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied,
repository checker, 15 Python policy tests, and diff checks.

## Review limits

This is fixture-sized selected-intent aggregation only. It does not claim
population/distributional metrics, outcomes, strategic quality, persistence,
providers, calibration, or human behavior.
