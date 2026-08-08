# M5 Actor Saved Replay-Debrief Records Request Summary

## Target slice

Expose the existing categorical actor replay-debrief projection from one
validated complete run loaded through the explicitly injected file store.

## Required behavior

- Accept a validated `CliRunId` without accepting paths or arbitrary strings.
- Load and decode the existing host artifact, verify replay against the
  receiving host's explicit execution inputs, require both bounded scenario
  windows, and only then project actor debrief records.
- Preserve the requesting host's current observation, draft, history, and
  saved bindings.
- Map missing, malformed, tampered, storage, and closed-session failures to
  bounded actor-safe errors; map an incomplete saved run to
  `debrief_unavailable/await_completion`.

## Non-goals

This slice does not add a second artifact schema, change file-store semantics,
implement locking or crash recovery, or claim scenario-wide durable replay,
causal-record persistence, transport integration, or provider behavior.
