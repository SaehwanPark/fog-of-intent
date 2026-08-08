# M4 Profile Comparison Request Summary

## Requested slice

Add a second transparent scripted profile so the M4 baseline can demonstrate a
profile difference under the same actor-visible observation.

## In scope

- Add `risk-taking-laner-v1` with a versioned
  `contest-first-fixed-score-v1` evaluation rule.
- Reuse the existing actor-visible candidate generation, stable selection, and
  observer-bound host request boundary.
- Compare cautious and risk-taking profiles on one identical initial
  observation and validate both requests through the existing lane validator.
- Update M4 canonical/workspace evidence and `LESSONS.md`.

## Out of scope

- New state, scenario mechanics, execution inputs, memory, communication,
  randomness, population metrics, external adapters, strategic quality, or
  human realism.

## Success evidence

- The same observation produces `Stabilize` for cautious and `Contest` for
  risk-taking, with inspectable profile/rule IDs.
- Both requests remain actor-bound and host-validatable.
- Repeated decisions remain deterministic and repository claims stay bounded.
