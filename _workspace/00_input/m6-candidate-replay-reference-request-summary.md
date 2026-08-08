# M6 Candidate Replay Reference Request Summary

## Requested slice

Select a bounded caller-declared replay reference for one verified
largest-delta tally candidate. Matching uses profile ID, evaluation rule, and
selected intent, then requires the chosen decision to replay exactly.

## Required contract

- Use `m6-scripted-agent-tally-replay-reference-v1`.
- Use `m6-first-verified-candidate-replay-v1`.
- Preserve caller-declared record order and choose the first verified match.
- Return only candidate labels and the matched observation ID.
- Distinguish no matching record from a matching record whose replay mismatches.

## Explicit limits

This is a reproducible reference, not representative-replay proof, scenario-
wide replay, build provenance, causal attribution, persistence, provider
behavior, or human evidence.
