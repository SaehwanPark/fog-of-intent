# M6 Caller-Declared Population Composition Request Summary

## Target slice

Allow a caller to express an ordered composition of the already closed
fixed-fixture scenario catalog while retaining the population bound and the
existing actor-visible sample/frequency validation paths.

## Required behavior

- Keep the exact `m6-scripted-agent-fixture-population-v1` schema and one-to-four
  entry bound.
- Accept only `safe-fixture-v1` and `river-side-threat-v1` IDs in caller order.
- Derive globally distinct observation-ID pairs from one checked starting ID.
- Preserve skewed compositions as explicit input and compose them through the
  verified matched-sample and scenario-frequency paths.

## Non-goals

This does not add random draws, weights, broad scenarios, representative
sampling, population diversity claims, outcomes, strategic metrics, persistence,
providers, or human-behavior evidence.

## Verification

Extend the existing population regression with a safe-heavy ordered composition,
exact 3/1 frequency counts, complete sample composition, and unknown-ID failure.
Run all pinned gates.
