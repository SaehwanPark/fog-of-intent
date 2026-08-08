# M5 Actor-History Status Request Summary

## Requested Outcome

Add a bounded `m5-actor-history-v1` DTO and host projection that lets an actor
see record count plus open/complete/closed lifecycle status without receiving
hashes, snapshots, or detailed replay data.

## In Scope

- Closed `ActorHistoryStatus` values and exact line-oriented codec.
- `CliScenarioHost::actor_history` projection for the two-window fixture.
- Codec bounds, lifecycle parity, hidden-field, and non-mutation regressions.

## Non-Goals

- Detailed records, replay, debrief, persistence, transport, or simultaneous
  actor coordination.

## Authority

The host owns lifecycle and history; the DTO is a status-only actor projection
and has no transition, replay, or persistence authority.

## Verification

One focused protocol test covers all three statuses and malformed/boundary
payloads; one focused host test covers open, complete, and closed projections.
