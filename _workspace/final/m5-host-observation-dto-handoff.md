# M5 Host-Observation DTO Handoff

## Outcome

`CliScenarioHost::actor_observation` now returns the active actor-visible
receipt through `m5-actor-observation-v1`, preserving the identity consumed by
actor action and draft DTOs without exposing internal lane types. Projection is
read-only, rejects complete/closed hosts through actor-safe errors, and does not
mutate host history or transition state.

## Verification

- One focused host regression proves exact mapper parity before and after the
  first fixture advance, actor-visible fields, hidden-field absence, unchanged
  history, and complete/closed lifecycle errors.
- 189 Rust unit tests, 7 binary integration tests, and 1 RustDoc test.
- Format, Clippy with warnings denied, repository checker, 14 Python checks,
  and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff.

## Limits and Next Dependencies

Transport/MCP framing, simultaneous actors, reconnect, persistence, provider
compatibility, and broader session coordination remain open. The host/lane
continues to own observation freshness, legality, transition, execution, and
history.
