# M6 Fixed-Fixture Scenario Selection Request Summary

## Target slice

Add a closed, versioned catalog for the two actor-visible fixture variants
already used by the matched-scenario evidence: a no-threat pair and a
RiverSide-threat pair. Select at most four catalog entries by their stable IDs,
bind each selected entry to caller-supplied observation IDs, and compose the
existing matched-sample pipeline.

## Required behavior

- Expose exact scenario IDs for the safe and RiverSide-threat fixture variants.
- Reject unknown IDs, empty selections, more than four selections, mismatched
  scenario/ID lengths, and globally duplicate observation IDs before policy
  evaluation. Repeated closed IDs are allowed as explicit ordered samples.
- Generate only actor-visible `LanerObservation` pairs from the fixed fixture
  states, preserving selection order and each pair's two caller-supplied IDs.
- Return the existing bounded matched-scenario sample so tally/report behavior
  remains unchanged; repeated selection with the same IDs is equal.

## Non-goals

This slice does not sample random scenarios, generate populations, estimate
distributions, resolve transitions, expose true state, persist selections, or
claim outcome, strategic, provider, or human-behavior evidence.

## Verification

Use both catalog IDs in ordered selections, repeat the same selection, assert
the generated observation IDs and visible-threat difference, and cover every
bounded rejection branch plus the four-selection inclusive cap. Run the pinned
Rust, repository, Python, formatter, Clippy, and diff gates.
