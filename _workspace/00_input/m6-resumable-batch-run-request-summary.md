# M6 Resumable Batch Run Request Summary

## Target slice

Add a bounded checkpoint and injected run-directory adapter for the existing
scripted-agent batch runner. A caller can persist a cursor after a deterministic
chunk, reload it, and continue the same ordered manifest batch.

## Required behavior

- Bind a checkpoint to one actor-visible observation and the ordered manifest
  list through a versioned, deterministic input fingerprint.
- Keep the cursor bounded to the existing 16-manifest batch cap and reject
  mismatched, malformed, incomplete, or over-capacity checkpoints fail closed.
- Evaluate only the requested remaining chunk with the existing seeded policy
  path and return the next cursor without changing host, lane, history, or
  transition state.
- Persist only the bounded checkpoint through the existing injected file-store
  boundary; storage errors remain generic to the batch adapter.

## Non-goals

This slice does not persist decisions, metrics, reports, population samples,
crash diagnostics, provider/model metadata, scenario-wide replay, or cloud/CI
experiment scheduling. A caller retains the observation and manifests needed
to resume and may recompute decisions deterministically from the cursor.

## Verification

Cover exact checkpoint encoding/decoding, malformed fields, input mismatch,
one-chunk resume across save/load, completion, and the inclusive 16-manifest
bound. Run the pinned Rust, repository, Python, and diff gates.
