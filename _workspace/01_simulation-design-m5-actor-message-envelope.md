# M5 Actor Message Envelope Design

## Goal and Roadmap Milestone

Advance the bounded M5 actor protocol by defining a recipient-scoped message
metadata envelope without claiming a communication system or MCP transport.

## Slice Boundary and Non-Goals

`ActorMessageDto` carries one bounded actor-authored text value, sender,
recipient, and observation binding. It is a pure protocol value. No host method
stores, routes, retries, or delivers it; queueing, ordering, trust, and
communication semantics remain open.

## Actors and Authority

The sender and recipient are actor identifiers supplied by the caller. The
envelope does not authenticate either actor and does not authorize a host
operation. Host validation, delivery, and simulation authority remain outside
the DTO.

## True State, Beliefs, Observations, and Reports

The message contains no true-state, belief, execution, hash, or report fields.
The observation ID only binds the message to the actor-visible decision window;
it does not reveal the observation itself.

## Plans, Commands, and Validation

Construction rejects zero IDs, self-delivery, empty text, control characters,
and text above `MAX_ACTOR_DRAFT_VALUE_BYTES`. Codec parsing additionally
requires the exact schema, fields, line count, and numeric bounds.

## Resolved Inputs and Random Streams

There are no resolved inputs or random streams. The envelope is immutable
metadata and cannot select, validate, or execute a lane intent.

## Events, Effects, and Transition

No event, effect, transition, or history record is produced. Delivery remains a
future host/session contract.

## History, Replay, and Branching

The envelope is not persisted, replayed, or included in record identity. Future
delivery and retention contracts must define those relationships explicitly.

## Debrief and Causal Explanation

The message is not a debrief or causal explanation and carries no outcome or
attribution data.

## Verification Contract

One focused protocol test proves canonical round-trip, literal wire shape,
sender/recipient/observation binding, valid bounded text, and rejection of
unknown/duplicate/missing/wrong-schema/extra-line, numeric, self-delivery,
empty, control, and overlong cases.

## Open Questions

Transport framing, authenticated actor sessions, recipient visibility,
delivery ordering, retries, trust, and communication-quality evidence remain
open.
