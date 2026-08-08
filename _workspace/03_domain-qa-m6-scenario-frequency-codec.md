# Domain QA — M6 Scenario-Frequency Codec

## Disposition

Pending independent three-pass review of implementation and evidence head
`248a669`.

## Scope reviewed

The slice adds a bounded line-oriented codec for the verified
`m6-scripted-agent-fixture-frequency-v1` report. Decoding accepts only text
that matches a caller-supplied constructor-validated report, so the codec does
not turn arbitrary text into trusted frequency evidence. It owns no scenario
generation, policy evaluation, transition, history, replay, persistence,
provider, or outcome authority.

## Evidence target

One focused frequency-report test covers the canonical five-line wire shape,
four-selection and singleton round trips, closed malformed-field cases,
inclusive count handling, oversized input, and verified-report tamper
rejection. The expected full evidence is one focused report test within 23
focused agent tests, 236 Rust unit tests, 7 binary tests, and 3 RustDoc tests,
plus formatter, Clippy warnings denied, repository checker, 15 Python policy
tests, and diff checks.

## Review limits

This is bounded in-process codec evidence only. It does not claim durable
export, arbitrary report construction, population generation,
random/distributional sampling, outcomes, strategic metrics, persistence,
providers, calibration, or human evidence.
