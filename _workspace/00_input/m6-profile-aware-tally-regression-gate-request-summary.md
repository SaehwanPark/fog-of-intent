# M6 Profile-Aware Tally Regression-Gate Request Summary

## Goal

Add one versioned fixed-fixture equality gate over the existing
profile-aware tally comparison without turning it into a broad threshold,
build, balance, or causal metric.

## In scope

- A closed regression-rule ID on the verified tally comparison.
- Equality of baseline/candidate pair and observation counts and every ordered
  profile row's five intent counts.
- Focused evidence for unchanged, changed-total, and same-total row
  redistribution comparisons.

## Out of scope

- Choosing or sampling scenarios/populations, rerunning policies, or accepting
  free-form thresholds/counts.
- Build/source provenance, causal attribution, balance, outcomes, strategic
  interpretation, persistence, provider behavior, or human evidence.

## Acceptance evidence

One focused agent regression must bind the literal rule ID, prove the unchanged
comparison passes, and prove both changed-total and same-total row changes
fail the gate while preserving ordered profile-aware comparison semantics.
