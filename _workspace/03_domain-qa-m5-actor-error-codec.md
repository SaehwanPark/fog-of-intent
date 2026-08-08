# M5 Actor-Error Codec Domain QA

## Acceptance Checks

- [x] The codec exposes only `m5-actor-error-v1`, a closed error code, and a
  closed repair hint.
- [x] Every current error and repair ID round-trips, with exact canonical wire
  text and bounded size/line parsing.
- [x] Unknown IDs, extra lines, and malformed fields fail closed without raw
  domain detail.
- [x] Repair remains advisory and no codec path gains host legality, transition,
  retry, or history authority.

## Verification Snapshot

Focused evidence includes 13 protocol tests, 5 session tests, and 18 host tests
within 192 Rust unit tests, 7 binary integration tests, and 1 RustDoc test,
plus format, Clippy, repository, Python, and diff gates.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review.

## Non-Claims

This is a pure actor-safe error codec, not automatic repair, transport/MCP
framing, persistence, or a raw diagnostic channel.
