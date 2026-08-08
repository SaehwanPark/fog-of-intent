# M4 Selection-Tiebreak Request Summary

## Requested slice

Make the existing deterministic top-1 selection contract explicit and
regression-bound for all three scripted profiles.

## Required boundaries

- Preserve `max-score-stable-order-v1` as the shared selection identity.
- Replace the current best only on a strictly higher score; equal scores keep
  the first advertised candidate.
- Keep candidate generation, scoring, host validation, and transition
  authority unchanged.
- Do not introduce top-k/nucleus sampling, randomness, or population claims.

## Evidence target

All three profiles expose the exact selection rule ID, and a synthetic equal
score pair selects the first advertised candidate.
