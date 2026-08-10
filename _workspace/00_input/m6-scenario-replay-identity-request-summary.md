# M6 Scenario Replay Identity Request Summary

## Requested slice

Add one bounded report that verifies deterministic replay across a sequence of
one to sixteen caller-declared actor decision replay records from a sampled
scenario run. This provides pure library-side scenario-wide replay identity
evidence without claiming runtime event production, causal-trace completeness,
or external persistence.

## Required contract

- Use the closed schema identity `m6-scripted-agent-scenario-replay-identity-v1`.
- Use the rule `m6-scenario-replay-identity-v1`.
- Accept a slice of 1 to 16 `ScriptedAgentReplayRecord`s.
- Reject empty slices, oversized slices (>16), and duplicate observation IDs.
- Report `AllVerified` when all records replay exactly, or `DecisionMismatch`
  when any record fails deterministic replay.
- Retain record count, verified count, start observation ID, and end
  observation ID.
- Keep inputs strictly bounded to caller-supplied replay records.

## Explicit limits

Do not add runtime trace production, causal graph reconstruction, hashes,
filesystem persistence, scenario scheduling, provider behavior, or human
evidence claims.
