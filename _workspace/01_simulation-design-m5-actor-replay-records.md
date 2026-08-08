# M5 Actor Replay-Records Design

## Delivered contract

`ActorReplayRecordDto` uses schema `m5-actor-replay-record-v1` and five
newline-terminated fields:

```text
schema=m5-actor-replay-record-v1
window=first|second
intent=<closed intent id>
outcome=held_space|yielded_space|forced_out
verification=verified
```

`CliScenarioHost::actor_replay_records` first verifies the existing immutable
history, then maps at most two records into categorical DTOs. The projection is
read-only and does not expose hashes, resolved inputs, execution traces,
record identity, or causal detail.

## Verification target

The implementation evidence is one focused protocol codec test and one focused
host projection test within the full 217-unit, 7-binary, and 3-RustDoc suite.
The focused tests cover canonical first/second records, malformed closed
fields, empty/partial/complete histories, payload-free output, closed sessions,
and tampered-history rejection without mutation.

## Open boundaries

This is an in-process categorical projection only. Durable/scenario replay
records, transport framing, reconnect, persistence, causal debrief detail,
provider/MCP integration, and broader scenario history remain deferred.
