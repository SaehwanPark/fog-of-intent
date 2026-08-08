# M5 Host-Action Submission Domain QA

## Acceptance Checks

- [x] The host validates actor identity, current observation, and lane legality
  before appending any actor action.
- [x] Two valid fixture actions close the first and second windows through the
  existing deterministic host/lane transition path.
- [x] Reused/stale actions and complete-window actions fail closed without a
  new record.
- [x] Malformed execution is mapped to `host_transition_rejected` without raw
  domain values and leaves history unchanged.
- [x] No transport, async runtime, randomness, or second transition engine is
  introduced.

## Verification Snapshot

The focused submission regression covers successful first/second-window
closure, reused action, complete window, and malformed execution. Expected
repository evidence is 185 Rust unit tests, 7 binary integration tests, and
1 Rustdoc test, plus format, Clippy, repository, Python, and diff gates.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review.

## Non-Claims

This is a synchronous two-window fixture path. It does not establish network
submission, simultaneous decisions, reconnect semantics, privileged control,
or broad MCP/client compatibility.
