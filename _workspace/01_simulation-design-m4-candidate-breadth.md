# M4 Candidate-Breadth Simulation Design

## Boundary

Candidate generation remains a pure function of copied actor-visible
`LanerObservation` data. It copies the advertised four legal intents and adds
the optional visible threat response only when it is not already present.
Selection, request construction, host validation, and transition authority are
unchanged.

## Evidence contract

- Safe fixture: four unique candidates from `available_intents`.
- RiverSide fixture: five unique candidates, including `Withdraw` from the
  visible threat response.
- Every candidate is present in the observation's advertised fields.
- Stable selection remains `Stabilize` for safe and `Withdraw` for RiverSide
  under the cautious profile.

## Limits

This measures candidate breadth only. It does not establish creativity,
strategic diversity, transformed candidates, random sampling, population
variation, outcomes, or human behavior. Memory, communication, and execution
remain deferred.
