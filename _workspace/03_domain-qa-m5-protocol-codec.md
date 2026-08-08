# Domain QA — M5 Protocol Codec

## Status

Pass target for the bounded DTO codec slice, pending the single required
code-reviewer handoff.

## Review contract

- Codec/schema IDs and field sets are versioned and exact.
- Size and line bounds are enforced before DTO projection.
- Unknown, duplicate, missing, malformed, unsupported-schema, and closed-enum
  inputs fail closed with bounded errors.
- Decoding does not validate legality or gain transport, session, persistence,
  history, or transition authority.

## Claim limits

Evidence covers only in-memory codec round-trips for the bounded DTOs. It does
not establish network framing, persistence, compatibility migration, repair,
provider support, or human behavior.

## Verification target

The focused protocol suite contains eight tests. The full suite is expected to
contain 181 Rust unit tests, seven binary integration tests, and one compile-
fail RustDoc test, alongside formatting, Clippy, repository policy, Python,
and diff checks.
