# M5 Actor Replay-Records Request Summary

## Requested slice

Add a bounded actor-visible projection for the records already verified by the
host replay path. Each record may expose only its first/second window, closed
intent ID, categorical outcome ID, and a `verified` marker.

## Required boundary

- Define `m5-actor-replay-record-v1` with an exact five-line codec and closed
  window, intent, outcome, and verification IDs.
- Return at most the two records supported by the fixture; keep hashes,
  resolved inputs, execution traces, record identity, and causal detail out of
  the DTO.
- Verify immutable host history before projection and preserve closed-session
  and tampered-history actor-safe errors.
- Keep the API read-only and separate from persistence, transport, and replay
  authority.

## Evidence target

One focused protocol codec test and one focused host test should prove exact
round trips, malformed-input rejection, empty/partial/complete projections,
closed-session rejection, tampered-history rejection, and actor-safe fields.
