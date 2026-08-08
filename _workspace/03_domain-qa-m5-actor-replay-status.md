# M5 Actor Replay Status Domain QA

## Status

Implementation complete; pending the required independent three-pass review.

## Scope and Authority

This slice adds only an actor-visible replay status DTO and a read-only host
projection over existing immutable history. It does not add replay transport,
persistence, causal debrief, lane transitions, or a second history authority.

## Evidence and Claim Limits

One focused protocol codec test and one focused host projection test cover
bounded counts, malformed input, successful empty/partial/complete history,
closed sessions, and tampered history. The evidence is fixture-sized status
verification, not replay records, traces, persistence, or complete MCP behavior.

## Required Fixes

Pending implementation and review.

## Verification Evidence

Focused evidence is 1 protocol test and 1 host test within 20 protocol, 12
session, and 25 host tests; the full suite is 213 Rust unit tests, 7 binary
integration tests, and 3 RustDoc compile-fail tests. Formatter, Clippy with
warnings denied, repository checker, 15 Python policy tests, and
`git diff --check` must pass.
