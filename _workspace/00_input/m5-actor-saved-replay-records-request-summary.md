# M5 Actor Saved Replay Records Request Summary

## Target slice

Expose the existing categorical actor replay-record projection from one
validated run loaded through the explicitly injected file store.

## Required behavior

- Accept a validated `CliRunId` without accepting paths or arbitrary strings.
- Load and decode the existing host artifact, verify replay against the
  current explicit execution inputs, and only then project actor records.
- Preserve the requesting host's current observation, draft, and history.
- Map missing, malformed, tampered, storage, and closed-session failures to
  bounded actor-safe errors.

## Non-goals

This slice does not add a second artifact schema, change file-store semantics,
implement locking or crash recovery, or claim scenario-wide durable replay or
causal-record persistence.
