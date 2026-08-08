# Domain QA — M4 Selection Tiebreak

## Status

Pass for the bounded deterministic selection contract.

## Reviewed inputs

- `ScriptedAgent::select_candidate` and equal-score focused regression.
- Profile selection-rule IDs and existing actor-visible candidate generation.
- Host/lane validation boundary and current M4 canonical claims.

## Findings

- Equal scores preserve the first advertised candidate; strictly higher scores
  still replace the current best.
- All three profile constructors bind `max-score-stable-order-v1` exactly.
- Selection receives no hidden state, execution inputs, randomness, or host
  authority and returns the existing request shape.

## Claim limits

This proves deterministic top-1 tie behavior for the fixture. It does not prove
top-k/nucleus sampling, random policies, population diversity, outcomes,
strategic quality, or human realism.

## Verification evidence

The focused agent suite has eleven tests. The full repository target is 165
Rust unit tests, seven binary integration tests, and one compile-fail RustDoc
test.
