# Request Summary

## Requested Outcome

Implement the bounded M3 `observe`, `inspect`, and contextual `help` adapter
contract on top of the stable in-session grammar. Return typed read requests and
static command metadata; do not render terminal output or inspect hidden state.

## Roadmap Milestone

M3 — CLI Reference Experience, read-only adapter foundation.

## In Scope

- Map `observe`, `inspect`, and `help` to typed `CliReadRequest` values.
- Restrict inspect targets to actor-visible `state` and committed `history`.
- Provide static contextual help metadata for all stable grammar verbs.
- Add typed read errors and transcript-style tests; preserve the domain boundary.

## Non-Goals

- No terminal rendering, host lifecycle, state serialization, privileged
  provenance inspection, or transition invocation.
- No claim of a complete CLI flow or playable scenario.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and checker unit tests
- `git diff --check`
