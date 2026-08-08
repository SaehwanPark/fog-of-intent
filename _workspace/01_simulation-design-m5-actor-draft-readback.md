# M5 Actor Draft Readback Design

## Contract

`CliScenarioHost::actor_draft` returns zero or more existing
`ActorDraftDto` values for the current actor's staged fields. Values are
returned only for the active observation and in the fixed
message/plan/contingency order; the DTO's existing bounded validation and
codec remain the compatibility contract.

## Authority and lifecycle boundary

The host reads its own draft after closed, complete, and committed-boundary
checks. It does not stage, clear, commit, advance, deliver, or mutate any
state. The returned metadata is an actor-owned readback, not communication to
another actor and not a lane or history command.

## Evidence

One focused host test covers empty and populated readback, exact observation
binding and field order, unchanged observation/history/commit state, and
committed/complete/closed rejection. Existing draft codec and staging tests
retain malformed-value and replacement coverage.

## Limits

Communication delivery, recipient visibility, simultaneous drafts, transport,
persistence, reconnect, provider behavior, and richer plan semantics remain
open.
