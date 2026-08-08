# M6 Fixed-Fixture Scenario-Frequency Report Request Summary

## Target slice

Summarize the frequency of each closed fixture scenario ID in a validated,
ordered `ScriptedAgentFixtureScenarioSelection`. Preserve the catalog order and
selection count in a bounded actor-safe report that can feed later evidence
without claiming a generated population or general distribution.

## Required behavior

- Return exact `safe-fixture-v1` and `river-side-threat-v1` rows in stable order.
- Count repeated explicit selections without rerunning policy evaluation.
- Retain a bounded total selection count and row counts that sum to it.
- Repeat construction with the same validated selection identically.

## Non-goals

This slice does not generate scenarios, sample randomly, estimate population or
outcome distributions, persist reports, encode transport data, or add host,
transition, history, replay, provider, or calibration authority.

## Verification

Use a four-entry repeated selection, assert literal schema and row IDs, exact
counts/order/total, repeated equality, and the bounded row-sum invariant. Run
the pinned Rust, repository, Python, formatter, Clippy, and diff gates.
