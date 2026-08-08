# M5 Actor-History Status Domain QA

## Acceptance Checks

- [x] `m5-actor-history-v1` exposes only bounded record count and a closed
  open/complete/closed status vocabulary.
- [x] Codec round-trips all statuses and rejects impossible counts, unknown
  status, and extra lines.
- [x] Host projection matches open, complete, and closed lifecycle states
  without mutating history or exposing hashes/snapshots.
- [x] Detailed history, replay, debrief, persistence, and transition authority
  remain outside this DTO.

## Verification Snapshot

Focused tests include 12 protocol tests, 5 session tests, and 18 host tests
within 191 Rust unit tests, 7 binary integration tests, and 1 RustDoc test,
plus format, Clippy, repository, Python, and diff gates.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review.

## Non-Claims

This is a bounded status summary for one synchronous two-window fixture. It does
not establish detailed history, replay/debrief compatibility, persistence,
transport, simultaneous actors, or complete MCP behavior.
