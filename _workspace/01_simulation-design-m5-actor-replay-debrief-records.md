# M5 Actor Replay-Debrief Records Design

## Delivered contract

`ActorReplayDebriefRecordDto` uses schema
`m5-actor-replay-debrief-record-v1` and seven newline-terminated fields:

```text
schema=m5-actor-replay-debrief-record-v1
window=first|second
intent=<closed intent id>
outcome=held_space|yielded_space|forced_out
objective=goal_achieved|goal_partially_achieved|goal_missed
attribution=committed_facts_only
verification=verified
```

`CliScenarioHost::actor_replay_debrief_records` rejects closed and incomplete
hosts, rebuilds the existing replay-verified debrief, and maps exactly two
categorical records. It does not expose causal, hash, input, trace, or
record-identity detail.

## Verification target

The implementation evidence is one focused protocol codec test and one focused
host projection test within the full 219-unit, 7-binary, and 3-RustDoc suite.
The focused tests cover canonical records, malformed closed fields,
completion/closed gating, exact ordering and labels, payload-free output, and
tampered-history rejection without mutation.

## Open boundaries

This is an in-process committed-facts projection only. Detailed causal review,
durable/scenario replay records, transport framing, persistence, reconnect,
provider/MCP integration, and broader scenario debriefs remain deferred.
