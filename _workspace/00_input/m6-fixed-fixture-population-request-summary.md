# M6 Fixed-Fixture Population Request Summary

## Target slice

Add a deterministic population-shaped input over the already closed
fixed-fixture scenario catalog, bound to a caller-supplied starting ID.

## Required behavior

- Expose the exact `m6-scripted-agent-fixture-population-v1` identity.
- Accept one through four entries and reject empty or over-capacity requests
  before constructing the population.
- Alternate `safe-fixture-v1` and `river-side-threat-v1` in stable order.
- Derive each pair's globally distinct caller-visible observation IDs in stable
  order from the caller-supplied starting ID with checked arithmetic and reject
  overflow.
- Compose the existing verified matched-sample path without adding policy,
  transition, history, replay, persistence, provider, or outcome authority.

## Non-goals

This is not random or broad population generation, scenario discovery,
distributional sampling, representative replay selection, outcome/strategic
measurement, or human-realism evidence.

## Verification

Extend the existing fixture-selection regression to bind the literal schema and
inclusive four-entry cap, assert alternating IDs and observation pairs,
reproducibility, matched-sample composition, empty/over-capacity failures, and
the inclusive `u64` observation-ID boundary plus overflow failure. Run all
pinned gates.
