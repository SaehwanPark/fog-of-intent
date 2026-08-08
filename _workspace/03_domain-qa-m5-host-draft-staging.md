# M5 Host-Draft Staging Domain QA

## Acceptance Checks

- [x] The host binds each draft to the current actor and observation before
  replacing the selected message, plan, or contingency field.
- [x] Repeated staging replaces one field without changing the current
  observation or appending history; the existing commit path still consumes the
  staged closed intent.
- [x] Stale, wrong-actor, committed, complete, and closed drafts fail closed
  with bounded actor-safe protocol errors.
- [x] Staging does not invoke lane legality, transition, execution, or
  communication authority; commit and advance remain the existing host boundary.

## Verification Snapshot

The focused host regression covers all three field mappings, replacement,
wrong-actor/stale/committed/complete/closed rejection, and unchanged history
and observation. The full suite contains 188 Rust unit tests, 7 binary
integration tests, and 1 Rustdoc test, plus format, Clippy, repository, Python,
and diff gates.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review.

## Non-Claims

This is a synchronous two-window host boundary. It does not establish metadata
delivery, communication semantics, free-form plan language, simultaneous
drafts, transport, persistence, provider metadata, or complete MCP behavior.
