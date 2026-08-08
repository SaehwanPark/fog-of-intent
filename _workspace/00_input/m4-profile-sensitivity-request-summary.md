# M4 Profile-Sensitivity Request Summary

## Requested slice

Test whether the three deterministic profiles respond to a visible RiverSide
threat while keeping the same candidate and host-validation boundaries.

## In scope

- Compare safe and visible-RiverSide `LanerObservation` values for all three
  profiles.
- Assert cautious changes from `Stabilize` to `Withdraw`, while risk-taking
  remains `Contest` and yielding remains `Yield`.
- Assert the visible response candidate is present for every profile and all
  six requests pass the existing lane validator.
- Synchronize M4/core docs, QA/handoff, changelog, and `LESSONS.md`.

## Out of scope

- New policy inputs, scenario outcomes, execution changes, memory,
  communication, randomness, role populations, strategic quality, or human
  realism.

## Success evidence

- Selection differences are tied to an actor-visible threat change rather than
  hidden state or execution inputs.
- The host remains the sole legality and transition authority.
