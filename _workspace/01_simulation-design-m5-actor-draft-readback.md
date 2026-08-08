# M5 Actor Draft Readback Design

## Contract

`CliScenarioHost::actor_draft` returns zero or more existing
`ActorDraftDto` values for the current actor's protocol-staged fields. Values
are returned only for the active observation and in the fixed
message/plan/contingency order; the DTO's existing bounded validation and
codec remain the compatibility contract.

## Authority and lifecycle boundary

The host reads its actor-protocol draft after closed, complete, and
committed-boundary checks. Legacy CLI draft text remains on its existing host
draft path and is not reinterpreted as a bounded actor DTO. The readback does
not stage, clear, commit, advance, deliver, or mutate any state; it is not
communication to another actor or a lane/history command.

## Evidence

One focused host test covers empty and populated actor-protocol readback,
exact observation binding and field order, unchanged observation/history/
commit state, committed/complete/closed rejection, and CLI-only malformed or
oversized draft text remaining outside this DTO projection. Existing draft
codec and staging tests retain malformed-value and replacement coverage.

## Limits

Communication delivery, recipient visibility, simultaneous drafts, transport,
persistence, reconnect, provider behavior, and richer plan semantics remain
open.
