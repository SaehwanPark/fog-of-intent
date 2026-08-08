# M5 Actor-History Status Design

## Contract

`ActorHistoryDto` encodes the bounded record count and one of `open`, `complete`,
or `closed` for the two-window host fixture. It uses the exact
`m5-actor-history-v1` line format and rejects impossible open/complete counts.

## Boundary

`CliScenarioHost::actor_history` derives status from host lifecycle and history
without exposing hashes, snapshots, detailed records, or replay inputs. The
projection is actor-visible metadata only; host transition and history remain
authoritative.

## Verification

Protocol coverage round-trips all statuses, pins the schema, rejects impossible
counts, unknown status, and extra lines. Host coverage compares open/complete/
closed projections and checks no hidden hash text is reachable.

## Deferred Work

Detailed history, replay, causal debrief, persistence, transport, and broader
MCP/session coordination remain separate.
