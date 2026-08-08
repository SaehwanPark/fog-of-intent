# M6 Scenario-Frequency Codec Handoff

## Outcome

Pending independent domain QA and final handoff review at implementation head
`248a669`.

## Delivered contract

`ScriptedAgentFixtureScenarioFrequencyReport` now has a bounded five-line,
4096-byte codec with closed fields and stable safe/threat row IDs. Decoding
parses an unverified value and returns it only when it exactly matches the
caller-supplied constructor-validated report, preserving verified-report
provenance. The codec owns no scenario generation, policy evaluation,
transition, history, replay, persistence, provider, or outcome authority.

## Verification target

The focused frequency-report test covers canonical text, four-selection and
singleton round trips, unknown/duplicate/missing/wrong-schema/wrong-row,
malformed/count-mismatch/extra-line/oversized inputs, inclusive count values,
and structurally valid count tampering. The expected full evidence is one
focused report test within 23 focused agent tests, 236 unit tests, 7 binary
tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks.

## Open boundaries

Durable export, arbitrary report construction, population generation,
random/distributional sampling, outcomes, strategic metrics, persistence,
providers, calibration, and human evidence remain open.
