# M5 Actor-Session Lifecycle Request Summary

## Requested slice

Define an immutable session lifecycle that binds one ordinary actor to one
current actor-protocol observation and prevents stale or duplicate submissions.

## Required boundaries

- Version the lifecycle as `m5-actor-session-v1`.
- Bind actor identity and current observation identity at the protocol edge.
- Reject cross-actor, stale, duplicate, no-observation, already-open, and
  closed-session operations with bounded errors.
- Leave intent legality, transition, history, replay, transport, and repair to
  existing host/adapter contracts.

## Evidence target

Immutable multi-window lifecycle, actor mismatch, stale observation, duplicate
submission, no-observation, already-open, and close regressions.

## Non-goals

No transport, reconnect/disconnect policy, simultaneous submission, validation
repair, plan/message DTOs, persistence, or provider integration.

## Verification

Focused session tests cover lifecycle and fail-closed errors. Full repository
checks remain required before handoff.
