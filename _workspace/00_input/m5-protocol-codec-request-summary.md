# M5 Protocol Codec Request Summary

## Requested slice

Add a pure, versioned line-oriented codec for the bounded actor observation and
intent-action DTOs.

## Required boundaries

- Use `m5-actor-codec-v1` with exact observation/action field sets.
- Reject input above 4096 bytes before projection.
- Reject unknown, duplicate, missing, malformed, unsupported-schema, extra-line,
  and closed-intent values with bounded errors.
- Keep decoding separate from transport, persistence, session framing, and host
  legality.

## Evidence target

Observation/action round-trips, malformed-field rejection, size/line bounds,
closed-intent rejection, and host validation after action decoding.

## Non-goals

No stdin/stdout integration, network framing, persistence, plan/message payloads,
repair protocol, provider-neutral transcript, or MCP client support.

## Verification

Focused protocol tests cover codec round-trips and failure classes. Full
repository checks remain required before handoff.
