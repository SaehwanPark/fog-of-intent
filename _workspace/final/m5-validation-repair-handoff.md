# M5 Validation-Error and Bounded-Repair Handoff

## Outcome

The protocol edge now projects codec and immutable actor-session freshness
failures through the versioned `m5-actor-error-v1` contract. Each bounded
error exposes a stable code and deterministic repair hint without retaining
raw payload or authoritative values.

## Changed Files

- `src/protocol.rs`: error schema, closed code/repair enums, and total codec
  mappings.
- `src/session.rs`: total session-error mappings into the actor-safe contract.
- `Cargo.toml`, `Cargo.lock`: package `0.1.97`.
- Canonical docs and `LESSONS.md`.

## Verification

- 9 focused protocol tests and 5 focused session tests.
- 183 Rust unit tests, 7 binary integration tests, and 1 Rustdoc test.
- Pinned format, Clippy with warnings denied, repository checker, 14 Python
  checks, and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff. The intended
disposition is pass only if the reviewer confirms the closed mappings,
actor-safe payload, deterministic hints, and authority boundaries.

## Canonical State Updates

ROADMAP and SPEC mark protocol-edge validation-error/repair behavior delivered
while leaving host-legality projection, automatic repair, transport retry,
reconnect, authorization, persistence, provider transcripts, and complete MCP
compatibility open. ARCHITECTURE and LESSONS record the same boundary.

## Known Limits and Next Dependencies

Hints are advisory and cannot rewrite or resubmit payloads. A future host-edge
contract must redact domain legality failures before any transport adapter
uses them. Transport framing, reconnect, and broader DTOs remain separate M5
slices.
