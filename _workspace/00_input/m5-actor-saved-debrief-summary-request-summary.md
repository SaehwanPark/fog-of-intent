# M5 Actor Saved Debrief Summary Request Summary

## Target slice

Expose the existing bounded `m5-actor-debrief-v1` summary from one validated
complete run loaded through the explicitly injected file store.

## Required behavior

- Accept a validated `CliRunId` without accepting paths or arbitrary strings.
- Load and decode the existing host artifact, verify replay against the
  receiving host's explicit execution inputs, require both scenario windows,
  and only then project the existing actor debrief summary.
- Preserve the requesting host's current observation, draft, history, commit,
  and saved bindings.
- Map missing, malformed, tampered, storage, and closed-session failures to
  bounded actor-safe errors; map an incomplete saved run to
  `debrief_unavailable/await_completion`.

## Non-goals

This slice does not add a new DTO or artifact schema, change file-store
semantics, implement locking or crash recovery, or claim scenario-wide durable
replay, detailed causal review, transport integration, or provider behavior.
