# M6 Fixed-Fixture Scenario Selection Handoff

## Outcome

PASS — independent domain QA and final handoff review found no actionable
findings at implementation/evidence head `bcd5b4d`.

## Delivered contract

`ScriptedAgentFixtureScenarioSelection` exposes the exact
`safe-fixture-v1` and `river-side-threat-v1` IDs under
`m6-scripted-agent-fixture-scenarios-v1`. It binds one caller-supplied pair of
observation IDs per ordered selection, permits repeated IDs as explicit
samples, rejects unknown/empty/length-mismatch/duplicate-ID/over-capacity
inputs, and projects the resulting actor-visible pairs through the existing
matched-scenario sample pipeline. It is not a population generator or random
sampler and owns no transition, history, replay, persistence, provider, or
outcome authority.

## Verification

The focused selection test covers literal catalog IDs, stable IDs/order,
visible-threat difference, repeated equality, all malformed/boundary cases, and
the four-selection cap. The full evidence is one focused selector test within
22 focused agent tests, 235 unit tests, 7 binary tests, and 3 RustDoc tests,
plus formatter, Clippy warnings denied, repository checker, 15 Python policy
tests, and diff checks; all pass at reviewed head `bcd5b4d`.

## Open boundaries

Population generation, random/distributional sampling, outcome and strategic
metrics, persistence, representative replays, providers, calibration, and
human evidence remain open.
