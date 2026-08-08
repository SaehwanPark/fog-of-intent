# M5 Actor-Draft DTO Domain QA

## Acceptance Checks

- [x] Message, plan, and contingency use closed field IDs under
  `m5-actor-draft-v1`.
- [x] Values are non-empty, control-free, and capped at 256 UTF-8 bytes.
- [x] Plan values are restricted to the existing closed actor intent IDs.
- [x] Codec round-trips preserve observer/observation binding and field/value
  order without hidden state or hash fields.
- [x] The DTO does not stage a host draft, communicate, validate legality, or
  mutate history.

## Verification Snapshot

Focused protocol tests cover all three fields and malformed/size/control/plan
cases. The full suite is expected to contain 187 Rust unit tests, 7 binary
integration tests, and 1 Rustdoc test, plus format, Clippy, repository,
Python, and diff gates.

## Disposition

Preliminary pass pending the required independent three-pass code/domain/docs
review.

## Non-Claims

This is a bounded metadata codec, not host draft staging, a communication
system, a prompt schema, or a complete MCP compatibility contract.
