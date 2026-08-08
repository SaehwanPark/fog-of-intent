# M6 Scenario-Frequency Markdown Evidence Request Summary

## Target slice

Add a concise Markdown projection for the already verified
`m6-scripted-agent-fixture-frequency-v1` report. Keep the projection pure and
in-process so the roadmap's evidence-report item advances without creating a
durable export, population, or outcome-metrics pipeline.

## Required behavior

- Render a stable heading, schema, selection count, and the two catalog rows in
  safe-then-RiverSide order.
- Preserve the report's bounded actor-safe fields only; do not add paths,
  timestamps, inputs, hashes, traces, or raw errors.
- Render both the four-selection 2/2 fixture and the singleton 1/0 boundary in
  focused evidence.

## Non-goals

This slice does not add filesystem writes, arbitrary report construction,
population/distributional sampling, outcome or strategic metrics, providers,
transport, or presentation beyond the pure Markdown string.

## Verification

Extend the existing focused frequency-report test with an exact canonical
Markdown assertion and a singleton zero-row assertion. Run the pinned Rust,
repository, Python, formatter, Clippy, and diff gates.
