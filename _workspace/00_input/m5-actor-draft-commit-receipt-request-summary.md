# M5 Actor Draft-Commit Receipt Request Summary

## Requested slice

Define one bounded actor-protocol receipt for a successful draft commit. The
receipt must identify the bound observer, observation, and committed intent,
then report only whether message, plan, and contingency fields were present at
the commit boundary.

## Required boundary

- Use a new versioned `m5-actor-draft-commit-receipt-v1` DTO and exact bounded
  line codec.
- Keep values, prompts, delivery, transport, persistence, and simultaneous
  draft ordering out of the receipt.
- Capture field presence before delegating to the existing host commit method;
  preserve its actor, freshness, lifecycle, and staged-plan checks unchanged.
- A failed commit returns the existing actor-safe error and leaves the draft
  repairable; a successful commit clears the internal draft as before.

## Evidence target

One focused protocol codec test and one focused host test should prove exact
fields, closed `present`/`absent` IDs, malformed-input rejection, payload-free
encoding/debug output, successful field-presence reporting, and unchanged
observation/history boundaries. Communication delivery and richer plan
semantics remain intentionally open.
