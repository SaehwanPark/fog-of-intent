# M5 Actor-Authorization Matrix Design

## Contract

The ordinary actor adapter accepts only requests bound to the current actor
receipt. A wrong observer is rejected before any staging, commit, legality,
transition, or history work, using the existing bounded
`actor_mismatch`/`use_bound_actor` pair.

## Redaction Matrix

Actor-visible observation, history, action-result, commit-result, and draft
receipt values may contain only their documented labels, closed IDs, and
bounded lifecycle/receipt metadata. The regression rejects state hashes,
health/position/wave values, execution inputs/traces, and raw provenance
markers in encoded or debug-visible values.

## Authority and Limits

This is evidence only; it adds no authorization service or new protocol
vocabulary. The host remains the lifecycle and actor-binding authority, the
lane remains the legality/transition/history authority, and the protocol owns
only pure DTO shapes. Transport authentication, privileged tools, simultaneous
privacy, persistence, and provider compatibility remain open.

## Verification Contract

- Wrong-actor action, draft, commit, and draft-receipt attempts all fail with
  the same actor-safe mapping.
- Each rejection preserves the observation and record count.
- Every matrix DTO remains free of hidden-state/hash/execution/provenance
  fields.
