# M5 Actor Replay Status Design

## Contract

`ActorReplayDto` uses `m5-actor-replay-v1` and carries only:

- a bounded record count from zero through two; and
- the closed `verified` replay result.

The host verifies its current `LaneScenarioHistory` before constructing the
DTO. Closed sessions and replay mismatches remain actor-safe errors.

## Authority and Limits

Replay verification remains a read-only host operation over existing immutable
history. The DTO does not contain records, hashes, resolved execution inputs,
traces, or causal explanations, and it adds no lane transition or persistence
authority.

## Verification Contract

- Codec round-trips counts 0, 1, and 2 and rejects malformed fields/counts.
- Host projection verifies empty, partial, and complete histories.
- Closed sessions reject the projection and tampered history maps to a bounded
  host-transition error without mutating record count.
