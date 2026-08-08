# M6 Scenario-Frequency Codec Design

## Goal and roadmap milestone

Make the fixed-fixture frequency evidence machine-readable without widening it
into a report-export or population-distribution contract.

## Wire shape and bounds

`ScriptedAgentFixtureScenarioFrequencyReport::encode/decode` uses the same
`m6-scripted-agent-fixture-frequency-v1` identity, a 4096-byte pre-parse bound,
fixed top-level `schema`, `selection_count`, and `entries` fields, and two
ordered `row=scenario-id|count` lines. Only the two closed catalog IDs are
accepted, each count is bounded by the selector's cap, and row counts must sum
to the selection count.

## Provenance and authority

Decoding first builds a private unverified value, then compares it with a
caller-supplied constructor-validated report before returning a trusted report.
This prevents observer-free text from becoming evidence and keeps the codec
actor-safe. The codec owns no scenario generation, policy evaluation,
transition, history, replay, persistence, provider, or outcome authority.

## Verification contract

One focused agent test covers canonical five-line text, four-entry and
singleton round trips, unknown/duplicate/missing/wrong-schema/wrong-row/
malformed/count-mismatch/extra-line/oversized input, and structurally valid
count tampering rejected as `InputMismatch`.

## Open boundaries

Durable export, arbitrary report construction, population generation,
random/distributional sampling, outcomes, strategic metrics, persistence,
providers, calibration, and human evidence remain open.
