# M6 Scenario-Frequency Markdown Evidence Design

## Goal and roadmap milestone

Complete the bounded evidence-report portion of M6 for the fixed-fixture
frequency report without treating a pure projection as durable export or a
population metric.

## Projection contract

`ScriptedAgentFixtureScenarioFrequencyReport::to_markdown` emits a stable
heading, the versioned schema, the bounded selection count, and a two-row
Markdown table. Rows remain in the catalog's safe-then-RiverSide order and
carry only the existing scenario IDs and counts.

## Construction and authority

The method accepts `&self` on a report that can only be constructed from a
validated fixed-fixture selection or a verified codec match. It performs no
policy evaluation, scenario generation, filesystem I/O, transition, history,
replay, persistence, provider, or outcome work. The Markdown string is an
in-process evidence projection, not a saved artifact or transport protocol.

## Verification contract

The existing focused agent test binds the exact canonical Markdown for the
four-selection 2/2 report and checks that the singleton 1/0 report retains the
zero RiverSide row. The full repository gates remain the evidence boundary.

## Open boundaries

Durable export, arbitrary report construction, population generation,
random/distributional sampling, outcomes, strategic metrics, persistence,
providers, calibration, and human evidence remain open.
