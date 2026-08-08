# M5 Session Edge Matrix Design

## Contract

`ActorSession` is versioned as `m5-actor-session-v2` and retains a bounded
optional close reason:

- `client_requested` from `close()`;
- `timed_out` from an explicit caller `timeout()` event;
- `disconnected` from an explicit caller `disconnect()` event.

`accept_encoded_action` decodes the existing bounded action DTO first. Unknown,
duplicate, missing, wrong-schema, invalid, oversized, and extra-line input maps
to the existing actor-safe codec IDs; valid decoded actions then use the normal
actor-first freshness and duplicate checks.

## Authority and Limits

No clock, async runtime, transport, reconnect, legality, transition, history,
or replay authority is introduced. Closure reasons are metadata for an already
closed immutable session, and repair hints remain advisory.

## Verification Contract

- Valid encoded action reaches `Submitted`.
- Malformed encoded input maps to bounded codec errors.
- Encoded stale and duplicate actions map to the existing session errors.
- All three explicit close reasons fail later actions closed.
