# M6 Operational-Log Segment Inventory Design

## Goal and evidence boundary

Let a caller observe which bounded segment suffixes exist without making the
filesystem listing a rotation controller or a crash-recovery report.

## Contract

`ScriptedAgentOperationalLogStore::list_segments` validates a constructed run
ID, scans only the configured root, recognizes the closed suffix family
`.foi-operational-log.segment-0` through `-3`, sorts and deduplicates the
indices, and returns them as bounded `u8` values. Unrelated, malformed,
temporary, and out-of-range names are ignored. Read and validation failures
map to the existing generic storage error.

The method does not decode files, inspect payloads, merge segments, infer
ordering, or mutate any base log, host artifact, checkpoint, history, or lane
state. Directory races remain explicitly outside the guarantee.

## Verification contract

One focused batch/store regression proves stable `[0, 1, 3]` discovery,
base/segment coexistence, missing-root and invalid-run failures, and the
existing segment round trips and bounds. Full Rust, RustDoc, formatter, Clippy,
repository, Python, and diff gates remain the evidence boundary.

## Open boundaries

Race-hard directory snapshots, automatic rotation, crash recovery, locking,
fsync, retention, export, runtime diagnostics, tracing/transport, scheduling,
providers/models, durable scenario-wide replay, and human evidence remain open.
