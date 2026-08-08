# M6 Scenario-Frequency Markdown Evidence Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentFixtureScenarioFrequencyReport::to_markdown` renders a stable
heading, schema, bounded selection count, and the two catalog rows in
safe-then-RiverSide order. It accepts only `&self` on a verified report, does
no I/O, and owns no export, scenario, policy, transition, history, replay,
persistence, provider, outcome, or population authority.

## Verification target

The focused frequency-report test covers the exact four-selection 2/2 Markdown
output and the singleton zero-row boundary. The expected full evidence is one
focused report test within 23 focused agent tests, 236 unit tests, 7 binary
tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks.

## Open boundaries

Durable export, arbitrary report construction, population generation,
random/distributional sampling, outcomes, strategic metrics, persistence,
providers, calibration, and human evidence remain open.
