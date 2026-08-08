# M5 Actor Replay Status Handoff

## Outcome

Implementation complete; pending the required independent three-pass review.

## Intended Contract

`m5-actor-replay-v1` reports only verified status and bounded record count after
the host replays current immutable history. Records, hashes, resolved inputs,
traces, and causal details remain private.

## Verification

One focused protocol codec test and one host projection test complement 20
protocol, 12 session, and 25 host tests within 213 Rust unit tests, 7 binary
integration tests, and 3 RustDoc compile-fail tests. Formatter, Clippy with
warnings denied, repository checker, 15 Python checks, and `git diff --check`
are the required gates.

## Limits

This is bounded in-memory status evidence only. Replay records, durable/scenario
replay integration, detailed causal review, messages, contingencies, reconnect,
and complete MCP transport remain open.
