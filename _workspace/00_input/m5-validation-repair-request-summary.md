# M5 Validation-Error and Bounded-Repair Request Summary

## Requested Outcome

Define a versioned, actor-safe protocol-edge error contract for malformed
codec input and immutable session-freshness failures, including deterministic
repair hints that a caller can follow without seeing hidden state.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded library adapter slice.

## Current Evidence

- `src/protocol.rs` exposes versioned observation/action DTOs and a bounded
  `m5-actor-codec-v1` parser.
- `src/session.rs` exposes immutable actor binding and observation/action
  freshness checks.
- Codec and session failures are typed, but they do not yet share a stable
  actor-facing error code and recovery vocabulary.

## In Scope

- A versioned `m5-actor-error-v1` identity with closed error-code and repair
  hint enums.
- Pure mappings from codec failures and session freshness failures to bounded
  error projections.
- Stable IDs and focused exhaustive mapping tests.
- Documentation of the no-auto-repair and no-host-authority boundary.

## Non-Goals

- Automatic payload rewriting, retries, or transport framing.
- Mapping raw lane/host validation errors that may contain authoritative
  values; that projection needs its own host-boundary contract.
- Plan, message, contingency, outcome, replay, or debrief DTOs.
- MCP, async runtime, persistence, reconnect, or provider integration.

## Project Boundaries Touched

- Protocol adapter: owns bounded error categorization and recovery hints.
- Session adapter: exposes freshness failures through the protocol error
  vocabulary without gaining legality or transition authority.
- Host/kernel: unchanged and still sole legality, transition, and history
  authority.

## Source Files

- `src/protocol.rs`
- `src/session.rs`
- `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`,
  `LESSONS.md`

## Expected Outputs

- Versioned error/recovery DTO metadata with no dynamic hidden-state payload.
- Focused tests for every codec/session error mapping and stable IDs.
- Synchronized canonical/workspace handoff documents with explicit limits.

## Verification

- Focused protocol/session tests.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- Repository checker, Python checks, and `git diff --check`.

## Evidence Limits and Open Questions

This slice proves only deterministic categorization and caller-visible repair
hints for local codec/session failures. It does not prove a network protocol,
automatic recovery, host-legality error redaction, reconnect behavior, or
complete MCP compatibility.
