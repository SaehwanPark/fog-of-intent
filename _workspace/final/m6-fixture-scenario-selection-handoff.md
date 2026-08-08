# M6 Fixed-Fixture Scenario Selection Handoff

## Outcome

Pending independent domain QA and final handoff review.

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

## Verification target

The focused selection test covers literal catalog IDs, stable IDs/order,
visible-threat difference, repeated equality, all malformed/boundary cases, and
the four-selection cap, alongside the expected 22 focused agent tests,
235-unit, 7-binary, and 3-RustDoc suite, formatter, Clippy warnings denied,
repository checker, 15 Python policy tests, and diff checks.

## Open boundaries

Population generation, random/distributional sampling, outcome and strategic
metrics, persistence, representative replays, providers, calibration, and
human evidence remain open.
