# M6 Matched Observation Sample Request Summary

## Target slice

Add a deterministic, caller-supplied two-observation matched sample over the
existing scripted-agent manifests. The sample preserves manifest order and
returns actor-safe selected-intent rows for each observation.

## Required behavior

- Require two observations from the same actor with distinct observation IDs.
- Evaluate a non-empty ordered manifest list through the existing seeded batch
  runner and retain profile, evaluation-rule, seed, and selected-intent labels.
- Return a versioned, bounded report with deterministic repeated output and no
  true state, execution, history, transition, or provider data.
- Reject invalid observation pairing and empty/over-capacity manifest lists
  before policy evaluation.

## Non-goals

This slice does not generate populations, choose scenarios, sample from a
distribution, persist samples, aggregate outcomes, calculate metrics, or claim
strategic quality, balance, or human behavior.

## Verification

Use one initial and one visible-threat observation with matched actor identity,
two explicit manifests, repeated-run equality, stable row/seed ordering, and
bounded rejection tests. Run the pinned Rust, repository, Python, and diff
gates.
