# M5 Actor Replay-Debrief Records Request Summary

## Requested slice

Expose the two complete fixture windows as replay-linked actor-safe debrief
records. Each record may contain only window, closed intent, categorical
outcome, categorical objective disposition, `committed_facts_only` attribution,
and `verified` status.

## Required boundary

- Define `m5-actor-replay-debrief-record-v1` with an exact seven-line codec and
  closed IDs.
- Require an active complete host and rebuild the existing replay-verified
  debrief before projection.
- Keep health, position, wave, coordination, delayed origins, hashes, inputs,
  traces, record identities, causal explanations, and persistence out of the
  DTO.
- Preserve bounded incomplete, closed, and tampered-history errors without
  adding transition, replay, transport, or provider authority.

## Evidence target

One focused protocol codec test and one focused host test should prove exact
round trips, malformed input rejection, completion gating, two-record ordering,
payload-free output, closed-session rejection, and tampered-history rejection.
