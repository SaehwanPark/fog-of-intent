# M4 Pressure-Sensitivity Request Summary

## Requested slice

Extend the existing actor-visible scripted-agent catalog with one bounded
utility feature: the Anchor profile should use observed wave pressure when
scoring `Stabilize`, while preserving the existing candidate, request, and
host-validation contracts.

## Required boundaries

- Consume only `LanerObservation::wave_pressure()`; do not read true state.
- Keep candidate generation and stable first-maximum selection unchanged.
- Keep the risk-taking and yielding profile semantics unchanged.
- Do not add memory, communication, randomness, outcomes, populations, or an
  executable agent adapter.
- Record the exact versioned evaluation-rule identity and a monotonic low/high
  pressure regression.

## Evidence target

At observed pressure values 0 and 3, Anchor's `Stabilize` candidate scores 80
and 83 respectively, both selected requests remain `Stabilize`, and both
requests pass the existing lane validator. This is a score-sensitivity check,
not a balance, outcome, or human-behavior claim.
