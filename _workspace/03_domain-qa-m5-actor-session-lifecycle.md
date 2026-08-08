# Domain QA — M5 Actor-Session Lifecycle

## Status

Pass target for the bounded immutable session slice, pending the single
required code-reviewer handoff.

## Review contract

- Session identity and phases are versioned and immutable.
- Actor and observation freshness checks are fail-closed and bounded.
- Duplicate and closed operations cannot produce a host request or mutation.
- Legality, transition, history, replay, transport, and repair authority remain
  outside the session module.

## Claim limits

Evidence covers one ordinary actor and fixture-sized observation IDs. It does
not establish reconnect semantics, network delivery, simultaneous decisions,
provider compatibility, or complete MCP session behavior.

## Verification target

The focused session suite contains four tests. The full suite is expected to
contain 177 Rust unit tests, seven binary integration tests, and one compile-
fail RustDoc test, alongside formatting, Clippy, repository policy, Python,
and diff checks.
