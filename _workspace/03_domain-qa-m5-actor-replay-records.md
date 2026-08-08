# M5 Actor Replay-Records Domain QA

## Disposition

Implementation complete; independent three-pass review is pending.

## Evidence

One focused protocol codec test and one focused host projection test cover the
exact five-line `m5-actor-replay-record-v1` shape, closed IDs, malformed input,
empty/partial/complete histories, payload-free output, closed-session
rejection, and tampered-history rejection. The full evidence target is 22
protocol, 12 session, and 27 host focused tests within 217 Rust unit tests, 7
binary tests, and 3 RustDoc tests; 15 Python policy tests and repository gates
remain required.

## Boundary assessment

The host verifies immutable history before mapping at most two categorical
records. No record identity, hash, resolved input, trace, causal detail,
transition, persistence, transport, or provider authority crosses the actor
boundary. The projection is read-only and preserves actor-safe closed and
tamper errors.
