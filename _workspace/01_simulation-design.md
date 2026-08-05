# Simulation Design — M2 Bounded Last-Known Threat Report

## Goal and Boundary

This slice adds one actor-visible last-known report to the existing player
observation. It is a projection of a bounded reportable fact, not a new source
of truth, intent, transition rule, or gank mechanic. The current hidden threat
truth remains in `LaneSnapshot` and is never copied into the player report
except for the deliberately reportable RiverSide last-known case.

The existing player intent set remains `[Stabilize, Contest, Recall]`. The
allied observation and scripted proposal remain unchanged and continue to use
unknown threat wording.

## Threat-Report Contract

`ThreatReport` has two forms:

```text
Unknown
LastKnown { region: RiverSide, last_seen_turn: Turn }
```

`observe_player` maps `JungleThreatTruth::RiverSide` to `LastKnown` at the
current observation turn. `JungleThreatTruth::Absent` and
`JungleThreatTruth::InLane` both map to `Unknown`: absence is not proof of
complete vision, and an in-lane threat remains hidden current truth rather than
an actor-visible fact.

The report exposes only the bounded region and last-seen turn. It does not
expose a source state hash, exact threat entity, current movement, hidden
opponent values, execution input, or whether the report remains current after
the observation.

## Authority and Replay

The projection remains synchronous and deterministic. `LaneHistory::verify_replay`
regenerates the player observation from the replay state, so a RiverSide report
is replay-checked without adding a new authoritative state field or changing
the transition/state-hash contract. Existing command, branch, coordination,
objective, scenario, and final-debrief identities remain unchanged.

The allied policy remains bound to its existing visible artifact and does not
learn or emit a RiverSide report in this slice. A future gank-response slice
must define how an intent acts on last-known information without treating it as
current truth.

## Verification Contract

Focused tests cover:

- RiverSide projects to a last-known report with the current observation turn;
- Absent and InLane project to Unknown;
- player intent availability and hidden opponent/source-hash boundaries remain
  unchanged;
- a history containing a RiverSide observation replays exactly;
- existing M1/M2 tests remain passing.

Evidence establishes one bounded last-known threat report only. It does not
establish complete vision, belief updates, gank response, variable pacing,
communication, strategy quality, balance, or human behavior.
