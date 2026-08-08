# Domain QA — M4 Policy-Decision Replay

## Status

Pass target for the bounded policy-replay inspection slice, pending the single
required code-reviewer handoff.

## Review contract

- The record schema, disposition, and mismatch error are versioned and
  bounded.
- Replay consumes only actor-visible observation and explicit policy seed data.
- Expected and declared-anomalous cases remain inspectable without claiming a
  degenerate population or strategic outcome.
- Tampering fails closed, and no host history, transition, execution, or
  durable persistence authority moves into the agent module.

## Claim limits

This is a library-only decision replay set. It does not prove population
behavior, strategic quality, scenario-level replay, durable artifact support,
or human behavioral realism.

## Verification target

The focused agent suite grows from thirteen to fifteen tests. The full suite is
expected to contain 169 Rust unit tests, seven binary integration tests, and
one compile-fail RustDoc test, alongside formatting, Clippy, repository
policy, Python, and diff checks.
