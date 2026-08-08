# M5 Actor-Draft DTO Request Summary

## Requested Outcome

Define a versioned, bounded actor-protocol DTO for one message, plan, or
contingency metadata value, bound to an actor observation and safe to carry
through a future host adapter.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded plan/message metadata contract.

## In Scope

- `m5-actor-draft-v1` identity with observer and observation binding.
- Closed `message`/`plan`/`contingency` field IDs.
- Bounded UTF-8 payloads with control-character and empty-value rejection;
  plan values must use the existing closed intent IDs.
- Stable line-oriented round-trip and malformed-input tests.

## Non-Goals

- Host draft staging, commit, transition, or history mutation.
- Free-form plan language, prompt/provider metadata, transport framing, or
  persistence.
- Messages/contingencies as communication, coordination, or outcome evidence.

## Boundaries

The protocol module owns only schema/shape validation. The host remains the
sole authority for accepting metadata, legality, and transitions; this DTO
does not authorize or submit a request.

## Verification

Focused protocol tests cover all three fields, exact plan vocabulary, size and
control bounds, round trips, and malformed rejection. Full repository gates
remain required.
