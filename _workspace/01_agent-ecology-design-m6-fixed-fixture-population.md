# M6 Fixed-Fixture Population Design

## Goal and evidence boundary

Provide a small deterministic population-shaped input for composing existing
actor-visible sample evidence without implying that the repository can sample
representative scenarios or populations.

## Contract

`ScriptedAgentFixtureScenarioPopulation::generate` defines
`m6-scripted-agent-fixture-population-v1`. It accepts one to four entries,
alternates the closed safe and RiverSide-threat fixture IDs, and derives each
entry's two observation IDs sequentially from one caller-supplied starting ID.
Checked arithmetic rejects an observation-ID overflow. The resulting
population exposes its closed selection and composes
`ScriptedAgentMatchedScenarioSample`, preserving the existing shared-observer,
global-ID, and actor-visible validation path.

## Authority boundary

The generator constructs only deterministic fixture snapshots and observation
identity metadata. It does not read true state, draw randomness, choose among
scenarios, execute policy decisions, resolve transitions, mutate history, load
or persist artifacts, or add provider/report authority.

## Verification contract

The existing fixture-selection test proves the literal schema, alternating
four-entry output, ordered observation IDs, repeated construction equality,
verified matched-sample composition, empty/over-capacity rejection, and the
inclusive maximum observation-ID boundary plus overflow rejection. Full Rust,
RustDoc, formatter, Clippy, repository, Python, and diff gates are required.

## Open boundaries

Broader/random scenario generation, population diversity, distributional
sampling, representative replays, outcomes, strategic metrics, persistence,
providers/calibration, and human evidence remain open.
