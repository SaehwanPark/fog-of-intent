# M6 Fixed-Fixture Scenario-Frequency Report Design

## Goal and roadmap milestone

Add the smallest aggregate evidence over the new fixed-fixture selector without
turning explicit selection counts into a population or distribution authority.

## Report contract

`ScriptedAgentFixtureScenarioFrequencyReport` uses
`m6-scripted-agent-fixture-frequency-v1`. It stores the bounded selection count
and two ordered rows: `safe-fixture-v1` and `river-side-threat-v1`, each with a
count in the closed selection. Since the input selection is capped at four,
all counts fit in `u8` and the row sum equals the total.

## Construction and authority

`from_selection` accepts only the private-field,
constructor-validated `ScriptedAgentFixtureScenarioSelection`. It counts the
selection's explicit closed IDs without rerunning policies or regenerating
observations. The report owns no scenario, population, distribution, outcome,
transition, history, replay, persistence, provider, or calibration authority.

## Verification contract

One focused agent test builds a four-entry safe/threat/threat/safe selection,
binds the literal schema and row IDs, asserts exact 2/2 counts and total 4,
checks stable catalog order and row-sum equality, and repeats construction for
equality. The full repository gates remain the evidence boundary.

## Open boundaries

Population generation, random/distributional sampling, durable export,
outcome/strategic metrics, representative replays, providers, calibration, and
human behavior remain open.
