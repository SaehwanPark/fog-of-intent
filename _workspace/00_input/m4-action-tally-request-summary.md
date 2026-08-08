# M4 Action-Tally Request Summary

## Requested slice

Add a bounded actor-safe selected-action tally over the existing safe and
visible-RiverSide `LanerObservation` fixtures for the three scripted profiles.

## Required boundaries

- Aggregate exactly two observations and require a shared observer identity.
- Expose only versioned profile/rule IDs, the observer, observation count, and
  selected-intent counts.
- Validate all six underlying observer-bound requests through the existing lane
  validator.
- Do not add population sampling, outcomes, execution metrics, randomness,
  memory, communication, or host authority.

## Evidence target

The tally should record cautious `Stabilize` once and `Withdraw` once,
risk-taking `Contest` twice, and yielding `Yield` twice. A mixed-observer
input must fail with a bounded metric error.
