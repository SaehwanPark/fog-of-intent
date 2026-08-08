# M5 Actor Draft Status Design

## Contract

`ActorDraftStatusDto` uses schema `m5-actor-draft-status-v1` and six newline-
terminated fields: schema, observer, observation ID, and aggregate presence
for message, plan, and contingency. It never contains or derives a payload.

## Host boundary

`CliScenarioHost::actor_draft_status` checks closed, complete, and committed
states before projecting the active observation binding and internal draft
field presence. It does not mutate the draft, observation, or history. The
existing staging and commit methods retain all validation and transition
authority.

## Evidence

The focused protocol test covers canonical encoding, round-trip decoding, the
closed field vocabulary, malformed input, and payload absence. The focused host
test covers empty/present status, unchanged host state, committed-boundary
rejection, completion, and closed-session errors.

## Limits

Presence is host acceptance metadata only. Delivery to another actor, transport,
persistence, reconnect, simultaneous drafts, and free-form plan semantics stay
outside this slice.
