# M5 Host-Observation DTO Domain QA

## Acceptance Checks

- [x] The host exposes the current receipt through the exact
  `m5-actor-observation-v1` DTO projection.
- [x] Projection is parity-checked before and after a fixture transition and
  preserves the observation ID used by later actor requests.
- [x] Projection leaves record count/history unchanged and exposes no hash or
  resolved-input field.
- [x] Observation, transition, legality, execution, and history authority stay
  on the host/lane boundary; no transport or simultaneous actor behavior is
  introduced.

## Verification Snapshot

The focused host regression covers initial/next-window parity, actor-visible
schema and intent, hidden-field absence, observation identity change, and
non-mutation. The full suite contains 189 Rust unit tests, 7 binary integration
tests, and 1 RustDoc test, plus format, Clippy, repository, Python, and diff
gates.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review.

## Non-Claims

This is a synchronous host projection for the bounded two-window fixture. It
does not establish transport, simultaneous actors, reconnect, persistence,
provider compatibility, or complete MCP/session behavior.
