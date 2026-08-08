# M6 Experiment Version Catalog Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentExperimentVersionCatalog` exposes the fixed
`m6-experiment-version-catalog-v1` identity for applicable ruleset, scenario,
policy-schema, and profile IDs. Prompt, model, tool-schema, and extractor
fields are explicit `not-applicable` labels for this deterministic library
slice. The catalog is pure metadata and does not alter manifest codecs, policy
execution, persistence, or simulation authority.

## Verification target

One focused agent catalog test plus the full 20-agent-focused / 233-unit,
7-binary, 3-RustDoc suite, formatter, Clippy warnings denied, repository
checker, 15 Python policy tests, and diff checks.

## Open boundaries

Provider/model and prompt integration, tool registration, extractor versions,
population and matched-scenario sampling, metrics, persistence, calibration,
representative replays, and human evidence remain open.
