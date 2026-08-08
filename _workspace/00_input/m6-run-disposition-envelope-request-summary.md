# M6 Run Disposition Envelope Request Summary

## Target slice

Preserve caller-declared outcomes that a bounded experiment may otherwise
drop: completed, crashed, timed out, missing branch, and inconclusive.
Represent only the closed categorical disposition in a small versioned codec;
do not add runtime crash detection, scheduling, persistence, or experiment
execution.

## Required behavior

- Expose a closed five-value disposition enum with stable IDs.
- Encode and decode one disposition through a bounded, exact line-oriented
  record under a new M6 schema identity.
- Reject unknown fields, duplicate/missing fields, wrong schema, unknown status,
  extra lines, and oversized input before returning a trusted record.
- Keep the record caller-declared and actor-safe; it must not contain paths,
  raw errors, stack traces, true state, policy inputs, decisions, or results.

## Non-goals

This slice does not detect crashes or timeouts, schedule work, preserve process
diagnostics, attach a disposition to a decision/result artifact, persist a
population run, or claim build provenance, causal attribution, or outcome
quality. Automatic runtime detection and richer result records remain open.

## Verification

Cover all five closed IDs, canonical round trips, exact wire text, every listed
malformed branch, the 4096-byte bound, and actor-safe field limits. Run the
pinned Rust, repository, Python, and diff gates.
