# M5 Actor-History Status Handoff

## Outcome

`ActorHistoryDto` now defines `m5-actor-history-v1` with only bounded record
count and open/complete/closed status. `CliScenarioHost::actor_history`
projects that lifecycle summary without hashes, snapshots, detailed records, or
replay authority.

## Verification

- One focused protocol test covers all statuses, exact round-trips, impossible
  counts, unknown status, and extra lines.
- One focused host test covers open, complete, and closed projections plus
  hidden-field absence.
- 191 Rust unit tests, 7 binary integration tests, and 1 RustDoc test.
- Format, Clippy with warnings denied, repository checker, 14 Python checks,
  and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff.

## Limits and Next Dependencies

Detailed history, replay, causal debrief, persistence, transport/MCP framing,
simultaneous actors, and broader session coordination remain open. The host
continues to own lifecycle, transition, and history authority.
