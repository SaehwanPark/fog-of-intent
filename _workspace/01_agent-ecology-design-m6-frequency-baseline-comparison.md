# M6 Fixed-Fixture Frequency Baseline Comparison Design

## Goal and roadmap milestone

Advance M6 regression evidence with the smallest declared-baseline comparison
that can be derived from already verified fixed-fixture frequency reports.

## Comparison contract

`ScriptedAgentFixtureScenarioFrequencyComparisonReport` uses
`m6-scripted-agent-fixture-frequency-compare-v1`. It stores baseline and
candidate selection totals and two ordered rows containing each count and a
signed candidate-minus-baseline delta. The counts remain bounded by the
underlying four-selection report.

## Construction and authority

`from_reports` accepts only report values that were constructed from validated
selections or matched verified codec input. It compares stored metadata without
rerunning policies, generating scenarios, or inspecting true state. The report
does not prove independent build identity or causal attribution; it is a
caller-declared baseline comparison and owns no transition, history, replay,
persistence, provider, population, or outcome authority.

## Verification contract

One focused agent test compares a 1/1 baseline with a 2/2 candidate, binds the
literal comparison schema and row IDs, asserts both positive deltas and stable
order, repeats construction, and checks the reversed negative deltas.

## Open boundaries

Independent build provenance, causal attribution, population generation,
random/distributional sampling, outcomes, strategic metrics, durable export,
persistence, providers, calibration, and human evidence remain open.
