# M5 Actor-Error Codec Design

## Contract

`ActorProtocolError::encode/decode` uses exactly three lines:
`schema=m5-actor-error-v1`, one closed `code`, and one closed `repair` ID. The
bounded parser preserves the existing size and line-count limits.

## Boundary

The codec carries no raw payload, actor ID, state hash, domain error, or
transport detail. Decoding reconstructs only the actor-safe projection and
cannot authorize repair, retry, legality, or host work.

## Verification

The focused regression round-trips all closed error and repair IDs, pins the
canonical wire text, rejects unknown IDs and extra lines, and checks debug output
contains no hidden hash detail.

## Deferred Work

Automatic repair, transport framing, persistence, provider compatibility, and
broader MCP/session orchestration remain separate.
