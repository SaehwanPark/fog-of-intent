# M6 Replay-Sequence Evidence Request Summary

## Requested slice

Add one bounded report that joins an existing actor-visible scripted decision
replay check with the existing caller-declared operational sequence status.
This is a pure library composition for sampled-run evidence; it must not claim
causal-trace completeness or scenario-wide replay.

## Required contract

- Use the closed identity `m6-scripted-agent-replay-sequence-evidence-v1`.
- Use the rule `m6-replay-identity-operational-sequence-v1`.
- Report `verified` or `decision_mismatch` for the recorded decision replay.
- Preserve the existing operational sequence status without repairing or
  mutating its event log.
- Keep inputs limited to the existing replay record and operational log.

## Explicit limits

Do not add runtime event production, causal links, hashes, persistence,
scenario-wide replay, provider behavior, or human-evidence claims.
