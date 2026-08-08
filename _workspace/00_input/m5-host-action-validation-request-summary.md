# M5 Host-Action Validation Request Summary

## Requested Outcome

Add a read-only host adapter for actor action DTOs. It must bind the action to
the current actor-visible observation, invoke the existing lane validator, and
return bounded actor-safe rejection categories without exposing raw domain
errors or mutating authoritative state.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded host-legality projection slice.

## Current Evidence

- `ActorActionDto` converts to an observer-bound `LaneIntentRequest`.
- `CliScenarioHost` owns the current lane state and history but has no public
  actor-action validation entry point.
- `m5-actor-error-v1` already categorizes protocol-edge failures and can carry
  host-safe rejection categories.

## In Scope

- A host method that validates one `ActorActionDto` against the current
  observation and lane state without appending history or closing a window.
- Stable actor-safe categories for actor mismatch, stale observation, closed
  scenario window, and generic lane-validator rejection.
- Focused regressions proving valid acceptance, each rejection category, and
  unchanged host observation/history on every call.

## Non-Goals

- Applying or committing the action, advancing a window, or changing history.
- Raw `LaneValidationError`/state hash/domain payload exposure.
- Transport, retry, reconnect, authorization, simultaneous submission, or
  privileged controller tools.

## Project Boundaries Touched

- Host: sole caller of lane validation; read-only adapter method only.
- Protocol: adds bounded host rejection codes and repair hints.
- Lane/kernel: unchanged validation and transition authority.

## Verification

- Focused host/protocol tests and full repository gates.
- Explicit assertions that successful and rejected calls preserve host state
  and do not append records.

## Evidence Limits

This proves one fixture host-bound validation path, not transport integration,
submission/closure, multi-actor simultaneity, or broad MCP/client behavior.
