# M5 Actor Message Envelope Request Summary

## Target slice

Define the bounded `m5-actor-message-v1` recipient-scoped envelope for
actor-authored text at the protocol edge.

## Required behavior

- Bind sender, recipient, and observation ID in a closed, exact codec.
- Accept only non-empty UTF-8 text within the existing actor payload bound.
- Reject self-delivery and zero actor IDs before producing an envelope.
- Preserve the distinction between message metadata and actual delivery.

## Non-goals

This slice adds no host routing, queue, transport, ordering, trust,
communication-quality, simultaneous-delivery, persistence, or transition/
history authority. It does not expose hidden state or reinterpret existing
`ActorDraftDto` values.
