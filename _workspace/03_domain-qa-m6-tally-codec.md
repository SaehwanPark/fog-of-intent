# Domain QA — M6 Selected-Intent Tally Codec

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Scope reviewed

The slice adds bounded encode/decode support for
`m6-scripted-agent-matched-scenario-tally-v1`. The codec preserves observer,
pair/observation counts, ordered profile/rule rows, and five intent counters;
it carries no observations, state, seeds, inputs, outcomes, traces, paths,
providers, or history and owns no persistence or execution authority.

## Evidence target

The focused matched-scenario sample-set test covers canonical text, round trip,
wrong schema, unknown/duplicate/missing fields, malformed rows, wrong rules,
count mismatch, extra lines, and oversized input. The expected full evidence is
21 focused agent tests within 234 Rust unit tests, 7 binary tests, and 3 RustDoc
tests, plus formatter, Clippy warnings denied, repository checker, 15 Python
policy tests, and diff checks.

## Review limits

This is bounded machine-readable evidence only. It does not claim durable
export, report pipelines, population/distributional sampling, outcomes,
strategic metrics, providers, calibration, or human behavior.
