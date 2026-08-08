# M5 Core Boundary Guard Domain QA

## Status

Implementation complete; pending the required independent three-pass review.

## Scope and Authority

This slice adds only repository-policy evidence that deterministic core modules
exclude async runtime/syntax, wall-clock, and network transport primitives. It
does not add transport, reconnect, provider, host, lane, transition, history,
or replay behavior.

## Evidence and Claim Limits

One focused checker test exercises forbidden and clean fixture paths. The
repository check scans the explicit core module list. This proves source
ownership boundaries only, not complete transport framing or MCP behavior.

## Required Fixes

Pending implementation and review.

## Verification Evidence

The focused checker suite is 1 test within 15 Python policy tests. The standard
Rust format, Clippy with warnings denied, 211 unit tests, 7 binary integration
tests, 3 RustDoc compile-fail tests, repository checker, and `git diff --check`
must pass.
