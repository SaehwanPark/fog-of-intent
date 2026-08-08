# M5 Actor Draft Clear Request Summary

## Target slice

Define one bounded actor-protocol command that clears staged message, plan, and
contingency metadata without echoing values or implying communication delivery.

## Required behavior

- Version the observation-bound `m5-actor-draft-clear-v1` command.
- Return `m5-actor-draft-clear-receipt-v1` with only observer, observation ID,
  and pre-clear `present`/`absent` bits for each draft field.
- Permit an empty clear as an idempotent no-op.
- Reject wrong-actor, stale, committed, complete, and closed requests through
  existing actor-safe errors.
- Preserve host observation/history and clear draft state only after checks.

## Non-goals

This slice does not deliver draft payloads, define communication semantics,
resolve plans or contingencies, or add transport, persistence, reconnect, or
simultaneous-draft behavior.
