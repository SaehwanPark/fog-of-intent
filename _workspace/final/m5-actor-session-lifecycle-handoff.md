# M5 Actor-Session Lifecycle Handoff

## Delivered

- Added the immutable `m5-actor-session-v1` lifecycle over actor protocol DTOs.
- Bound one ordinary actor to one current observation and one action per
  window; added stale, cross-actor, duplicate, no-observation, already-open,
  and closed-session errors.
- Preserved host legality, transition, history, replay, and transport
  authority boundaries.
- Synchronized canonical/workspace docs, CHANGELOG, ARCHITECTURE, and
  LESSONS.md.

## Verification

The focused session suite contains four tests. The full suite contains 177
Rust unit tests, seven binary integration tests, and one compile-fail RustDoc
test, plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Domain QA disposition

Pass for the immutable library lifecycle slice. Session transitions only track
actor/observation freshness and phase; they do not validate or commit intent.

## Open boundaries

Transport, reconnect/disconnect behavior, simultaneous submission, validation
repair, plan/message metadata, persistence, provider-neutral transcripts, and
complete MCP session behavior remain open.
