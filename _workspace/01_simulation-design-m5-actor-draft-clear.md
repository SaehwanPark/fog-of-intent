# M5 Actor Draft Clear Design

## Contract

`ActorDraftClearDto` uses `m5-actor-draft-clear-v1` and carries only the bound
observer and observation ID. A successful clear returns the six-line
`m5-actor-draft-clear-receipt-v1` acknowledgement with field-presence bits
captured before the clear.

## Host boundary

`CliScenarioHost::clear_actor_draft` checks closed, actor, complete, committed,
and observation freshness in that order. It then clears only the host-owned
draft and leaves observation/history unchanged. Empty clears are idempotent.

## Evidence

The focused protocol test covers both codecs, canonical fields, round trips,
malformed input, and payload absence. The focused host test covers empty and
present clears, pre-clear presence reporting, wrong/stale/committed/complete/
closed failures, and unchanged observation/history.

## Limits

The receipt is acceptance metadata, not delivery. Communication, transport,
persistence, reconnect, simultaneous drafts, and free-form plan semantics stay
outside this slice.
