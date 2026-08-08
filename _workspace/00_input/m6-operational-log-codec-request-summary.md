# M6 Operational-Log Codec Request Summary

## Target slice

Persist the existing bounded, payload-free operational event log through a
versioned codec and an injected file-store namespace that cannot collide with
host artifacts or batch checkpoints.

## Required behavior

- Encode and decode the exact `m6-scripted-agent-operational-log-v1` shape:
  `schema`, `entries`, then one `event` line per ordered record.
- Keep the codec bounded to 4096 bytes and at most 16 event records, with the
  closed five-ID vocabulary: `batch_started`, `chunk_completed`,
  `checkpoint_saved`, `batch_resumed`, and `batch_finished`.
- Reject unknown, duplicate, missing, unsupported, invalid, over-count,
  over-line, and oversized input before constructing trusted evidence.
- Save and load through the injected store using the distinct
  `.foi-operational-log` and `.foi-operational-log.tmp` suffixes. The same
  root/run ID must continue to support the host artifact and batch checkpoint
  independently.

## Non-goals

This slice does not add runtime event production, automatic crash or timeout
detection, tracing, durations, diagnostics, rotation, crash recovery,
external export, transport, scheduling, provider/model metadata, or scenario
history authority. The operational log remains a non-authoritative label
record, not committed simulation history or a decision/report archive.

## Verification

Cover exact canonical encoding and round-trip decoding, all closed malformed
branches, the inclusive byte and entry limits, same-root/same-run-ID
coexistence with host/checkpoint artifacts, and storage/decode failures that
do not mutate caller-owned logs. Run the pinned Rust, repository, Python, and
diff gates.
