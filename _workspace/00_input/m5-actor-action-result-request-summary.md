# M5 Actor-Action Result Request Summary

## Requested Outcome

Return successful host actor submissions through a bounded
`m5-actor-action-result-v1` DTO containing only fixture window and categorical
outcome.

## In Scope

- Closed first/second window and held/yielded/forced-out outcome IDs.
- Exact line-oriented codec and host projection after existing submission.
- Actor-safe redaction and two-window result regression coverage.

## Non-Goals

- New validation or transition logic, detailed debrief, persistence, transport,
  simultaneous actors, or provider-specific behavior.

## Authority

The host remains the sole validator, transition, execution, and history owner.
The result DTO is a success projection only and cannot authorize host work.

## Verification

One focused protocol test covers all six closed window/outcome combinations and
malformed IDs; one focused host test covers both successful fixture windows.
