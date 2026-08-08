# M5 Actor Draft Status Request Summary

## Target slice

Define one bounded actor-protocol contract for inspecting the active host draft
without delivering its free-form message, plan, or contingency values.

## Required behavior

- Version the six-line `m5-actor-draft-status-v1` DTO.
- Retain only the bound observer, observation ID, and `present`/`absent` bits
  for message, plan, and contingency.
- Expose the status only for an active, uncommitted window.
- Reject closed, complete, and committed hosts with existing actor-safe errors.
- Keep the projection read-only and preserve host draft, observation, and
  history authority.

## Non-goals

This slice does not deliver draft payloads, define communication semantics,
resolve plans or contingencies, add transport/persistence/reconnect behavior,
or support simultaneous drafts.
