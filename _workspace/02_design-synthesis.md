# Design Synthesis — M2 Bounded Last-Known Threat Report

## Decision

Add a bounded `LastKnown` threat report to the player observation without
adding a new intent or transition mechanic. The report is generated from the
existing host-owned snapshot: `RiverSide` is reportable at the observation turn;
`Absent` and hidden current `InLane` remain `Unknown`.

This is the smallest useful vision/last-known slice before gank response. It
keeps the authoritative state, transition result, state hash, player intent
set, allied policy artifact, and all replay identities unchanged.

## Resolved Contract

`ThreatReport` exposes:

```text
Unknown
LastKnown { region: RiverSide, last_seen_turn: Turn }
```

The player can inspect the bounded report region and observation turn, but not
the source hash, exact threat entity, current movement, hidden InLane truth,
opponent truth, or execution values. The allied actor continues to receive
the existing unknown threat projection in this slice.

`LaneHistory::verify_replay` regenerates the observation from the replay state,
so a RiverSide observation is committed and replay-checked through the same
authority as every prior record. No new state field or transition input is
needed.

## Evidence and Limits

Focused tests cover RiverSide last-known wording, Absent/InLane unknown
behavior, public report accessors, source-hash boundaries, and replay of a
RiverSide history record. The full suite passes with 50 Rust tests.

This establishes one bounded last-known threat report only. It does not
establish complete vision, belief updates, gank response, variable pacing,
communication, strategy quality, balance, or a complete playable lane
scenario.
