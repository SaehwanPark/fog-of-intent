# M6 Selected-Intent Tally Codec Design

## Contract

`ScriptedAgentMatchedScenarioTallyReport::encode/decode` uses the existing
`m6-scripted-agent-matched-scenario-tally-v1` identity and a bounded
line-oriented shape: fixed top-level metadata followed by one pipe-delimited
row per ordered profile. The row grammar contains only closed profile/rule IDs
and five numeric intent counters.

Decode accepts an already verified report as provenance, parses wire data into
a private candidate, and returns `InputMismatch` unless the candidate exactly
matches that verified report.

## Bounds and validation

The decoder rejects input above 4096 bytes before parsing, requires the exact
line count implied by the row count, caps rows at the existing 16-manifest
limit, and verifies each row's profile/rule pairing and count total against
the report observation count. Unknown, duplicate, missing, malformed, and
extra fields fail closed.

## Authority and information boundary

The codec carries only actor-safe tally metadata. It does not include
observations, state, seeds, inputs, outcomes, traces, paths, providers, or
history and cannot invoke policy, transition, persistence, or replay behavior.

## Verification contract

One focused codec test covers canonical text/round trip, exact row order,
oversize/wrong-schema/unknown/duplicate/missing/extra-line errors, malformed
rows, wrong rule identity, and count-total mismatch. Full repository gates are
the evidence boundary.
