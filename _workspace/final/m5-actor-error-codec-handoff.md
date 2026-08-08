# M5 Actor-Error Codec Handoff

## Outcome

`ActorProtocolError::encode/decode` now provides exact bounded
`m5-actor-error-v1` wire text for the closed error and repair ID vocabularies.
Unknown IDs, malformed fields, and extra lines fail closed; the codec retains no
raw payload or domain detail and cannot authorize repair or host work.

## Verification

- One focused protocol regression round-trips every error and repair ID, pins
  canonical wire text, and rejects unknown IDs and extra lines.
- 192 Rust unit tests, 7 binary integration tests, and 1 RustDoc test.
- Format, Clippy with warnings denied, repository checker, 14 Python checks,
  and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff.

## Limits and Next Dependencies

Automatic repair, transport/MCP framing, persistence, provider compatibility,
and broader session coordination remain open. Host legality, transition,
execution, and history authority remain unchanged.
