# M5 Actor-Authorization Matrix Domain QA

## Status

Implementation is complete; pending the required independent three-pass review.

## Scope and Authority

The slice adds only table-driven evidence over existing actor adapter methods.
No production authority, protocol vocabulary, transport, or lifecycle behavior
changes. Wrong-actor requests fail before host draft, commit, legality,
transition, or history work.

## Information Boundary

The matrix inspects only actor-visible DTOs and bounded errors. It must reject
state/hash/execution/provenance markers without asserting unsupported human or
network privacy claims.

## Evidence and Claim Limits

Evidence is one deterministic host test over four wrong-actor requests and
five actor-visible DTO/result values. It does not establish transport
authentication, simultaneous actors, persistence, accessibility, or complete
MCP behavior.

## Required Fixes

Pending implementation and review.

## Verification Evidence

The focused matrix is one host test. Current protocol/session/host evidence is
17 protocol, 5 session, and 23 host tests within 201 Rust unit tests, 7 binary
integration tests, and one RustDoc compile-fail test. Formatter, Clippy with
warnings denied, repository checker, 14 Python policy tests, and
`git diff --check` all pass.
