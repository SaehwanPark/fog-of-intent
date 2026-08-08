# M4 Profile-Comparison Report Request Summary

## Requested slice

Expose a versioned, actor-safe comparison report for the three deterministic
scripted profiles over one `LanerObservation`.

## In scope

- Add `m4-scripted-agent-metrics-v1` with three bounded profile rows.
- Include profile/rule identity, selected intent/score, candidate count, and
  observer/observation identity only.
- Add reproducibility and row-order assertions over the existing catalog.
- Synchronize M4/core docs, QA/handoff, changelog, and `LESSONS.md`.

## Out of scope

- True-state hashes, execution inputs, raw domain errors, outcome metrics,
  population distributions, randomness, memory, communication, or external
  adapters.

## Success evidence

- The same observation yields three stable rows in catalog order with bounded
  counts and actor-safe values.
- Rebuilding the report from the same observation produces an equal report.
