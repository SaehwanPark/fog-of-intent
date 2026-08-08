# M5 Protocol Domain-Visibility Request Summary

## Requested Outcome

Keep authoritative lane observation and action-request conversions behind the
crate-private protocol edge so public protocol compatibility exposes DTOs,
not internal domain types.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- Make the lane observation projection and actor-action request conversion
  crate-private implementation adapters.
- Add two independent compile-fail RustDoc checks proving public consumers
  cannot call those domain conversions.
- Synchronize the current package and canonical/workspace boundary claims.

## Non-Goals

- Hiding the lane module from the host/lane implementation.
- Changing DTO fields, codecs, host authority, transport, authentication,
  persistence, or provider compatibility.

## Expected Outputs

- Public DTO-only protocol surface for observation and action construction.
- Two independent compile-fail RustDoc boundary tests and synchronized
  documentation.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

This is a Rust visibility/API boundary only. It does not authenticate callers
or prove transport, persistence, provider, or complete MCP behavior.
