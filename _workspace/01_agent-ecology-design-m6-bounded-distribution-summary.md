# M6 Bounded Distribution Summary Design

## Goal and evidence boundary

Make the existing caller-declared fixed-fixture frequency evidence easier to
inspect as a bounded distribution summary without implying that the library
samples a population or estimates a real-world distribution.

## Contract

`ScriptedAgentFixtureScenarioFrequencyReport::distribution_basis_points`
returns two ordered `u16` shares in the fixed catalog order
`safe-fixture-v1`, `river-side-threat-v1`. The selection count is already
validated to be one through four. The first row uses integer floor division of
its count by the selection count at a 10,000-point scale; the second row gets
the remainder, so shares always sum to exactly 10,000.

`to_distribution_markdown` is a pure in-process projection containing only the
schema, selection count, ordered scenario IDs, counts, and basis-point shares.

## Authority boundary

The summary reads only private fields of a constructor-verified frequency
report. It does not rerun policy, inspect true state, choose scenarios, draw
randomness, write files, mutate history, or add host/lane/transition/replay,
provider, outcome, or calibration authority.

## Verification contract

The focused frequency regression binds the literal scale and exact Markdown
for a 1-safe/3-RiverSide composition, a balanced 2/2 composition, and an
all-safe 4/0 composition. It checks stable row order, exact counts, exact
shares, and the 10,000-point sum. Full Rust, RustDoc, formatter, Clippy,
repository, Python, and diff gates are required.

## Open boundaries

Random or representative sampling, broader scenario generation, population
diversity, outcome/strategic metrics, durable export, persistence,
providers/calibration, and human evidence remain open.
