# M6 Fixed-Fixture Scenario Selection Design

## Goal and roadmap milestone

Advance M6 with the smallest scenario-selection boundary that can feed the
existing matched sample and tally contracts without becoming a population
sampler.

## Closed catalog and input binding

`ScriptedAgentFixtureScenario` owns exactly two IDs:
`safe-fixture-v1` and `river-side-threat-v1`. A selection accepts an ordered
list of those IDs plus one caller-supplied `[ObservationId; 2]` pair per ID.
The selection is non-empty, capped at four entries, permits repeated IDs as
explicit ordered samples, rejects globally duplicate observation IDs, and
requires the two input lists to have equal lengths.

## Generated observations and composition

The safe case observes the initial fixture state twice with distinct IDs. The
RiverSide case observes the same initial state followed by the existing fixed
RiverSide-threat state. Both are projected through `observe_player`; no true
state, execution input, or transition result crosses the agent boundary. The
ordered generated pairs are passed directly to
`ScriptedAgentMatchedScenarioSample::from_observations` with caller-supplied
manifests.

## Authority and reproducibility

The catalog is closed metadata plus deterministic fixture construction. It owns
no randomness, scenario search, distribution, transition, history, replay,
persistence, provider, or outcome authority. Stable scenario order, explicit
observation IDs, and ordered manifests are the complete reproducibility input.

## Verification contract

One focused agent test binds both literal IDs, proves visible-threat-sensitive
observations and stable IDs/order, repeats the selection, covers unknown/empty,
length, duplicate-ID, and over-capacity errors, and accepts exactly four
entries including repeated IDs.
The full repository gates remain the evidence boundary.

## Open boundaries

Population generation, random/distributional sampling, outcome and strategic
metrics, persistence, representative replays, providers, calibration, and
human behavior remain open.
