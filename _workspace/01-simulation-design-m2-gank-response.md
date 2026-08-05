# Simulation Design — M2 Bounded Gank Response

## Goal and Boundary

This slice adds one conditional player `Withdraw` response to the existing
lane command and transition boundary. It is available only when the current
player observation reports a RiverSide `LastKnown` threat. It is not available
when threat information is `Unknown`, and it does not turn last-known data into
current hidden truth.

The existing strategic intents remain `[Stabilize, Contest, Recall]`. Withdraw
is a conditional threat response exposed separately as
`available_threat_response()`. The allied observation and scripted policy
remain limited to `Stabilize` and `Contest`.

## Withdraw Contract

```text
ThreatReport::LastKnown { region: RiverSide, last_seen_turn: Turn }
  -> available_threat_response() == Some(Withdraw)

ThreatReport::Unknown
  -> available_threat_response() == None
```

With a current RiverSide report, a valid Withdraw command commits one beat and
moves the player to `NearTower`. The explicit `LaneWaveResult`, damage, and
execution trace remain authoritative inputs; a legal execution can still make
the response unfavorable. The outcome is `YieldedSpace` while health remains
positive and `ForcedOut` when explicit self damage reaches zero. Position
movement is intent-attributed and Withdraw never activates the Contest-only
fallback.

## Validation and Information

Host validation accepts Withdraw only when the current observation advertises
it through `available_threat_response()`. The same actor, observation ID,
turn, ruleset, source-state, prior-hash, phase, and state-validity checks remain
in force. A stale observation from a RiverSide state cannot authorize Withdraw
against a different state, and an Unknown current InLane/Absent state cannot
authorize it.

No opponent truth, exact threat entity, current threat movement, source hash,
or execution result is exposed. The allied policy does not receive or emit a
Withdraw proposal or counter shape in this slice.

## Replay, Objective, and Attribution

Withdraw uses the existing `LaneIntent` field and receives a stable identity tag
after the existing Stabilize/Contest/Recall tags. `LaneHistory`, branch,
objective, scenario, and final debrief paths remain unchanged; history replay
regenerates the same conditional observation before validating the command.
Objective and debrief projections report Withdraw as the committed intent and
retain the distinction between intentional withdrawal, explicit execution,
and ForcedOut. No optimality or balance judgment is inferred.

## Verification Contract

Focused tests cover:

- Withdraw availability only with a current RiverSide last-known report;
- Unknown, stale, resolved, wrong-actor, and malformed Withdraw rejection;
- NearTower movement, explicit wave/execution preservation, intent attribution,
  and no fallback activation;
- legal unfavorable/ForcedOut execution and history/objective replay;
- unchanged allied candidate bounds, hidden-state boundary, and prior tests.

Evidence establishes one bounded conditional Withdraw response only. It does
not establish complete vision, current threat tracking, variable pacing,
communication, strategy quality, balance, or a complete playable scenario.
