# M5 Host-Draft Staging Request Summary

## Requested Outcome

Connect the bounded `m5-actor-draft-v1` DTO to a host-owned, observation-bound
draft staging method for message, plan, and contingency metadata.

## In Scope

- `CliScenarioHost::stage_actor_draft` with actor/observation freshness checks.
- Replacement semantics matching the existing CLI draft fields.
- Bounded committed-boundary, stale, closed, and complete-session rejection.
- Read-only history/observation preservation tests and canonical updates.

## Non-Goals

- Commit/advance through metadata, free-form plan interpretation, transport,
  communication delivery, persistence, or provider prompt metadata.

## Authority

The host owns draft mutation and commit boundaries; the DTO owns only shape and
payload bounds. Lane legality, transition, execution, and history remain
unchanged.

## Verification

Focused host tests cover all three field mappings, replacement, wrong actor,
stale/committed/complete/closed rejection, and unchanged history.
