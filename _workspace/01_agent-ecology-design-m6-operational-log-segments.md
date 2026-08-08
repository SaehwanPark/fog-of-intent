# M6 Operational-Log Segments Design

## Goal and evidence boundary

Expose a small caller-declared segment namespace for the existing operational
log. Evidence is limited to independent bounded files; it does not claim an
automatic rotation policy or crash-safe event pipeline.

## Contract

`ScriptedAgentOperationalLogStore::save_segment` and `load_segment` accept only
segment indices `0..=3`. They reuse the existing run-ID validation, bounded
codec bytes, same-directory replacement, and symlink/file confinement. Segment
files use `.foi-operational-log.segment-0` through
`.foi-operational-log.segment-3`, with matching temporary suffixes.

The base `.foi-operational-log`, `.foi-artifact`, and `.foi-batch-run` paths
remain separate. A segment is just a caller label around one closed log; the
store does not merge segments, infer order, or mutate any host/lane/history
state.

## Verification contract

One focused batch/store regression proves segment 0 and 1 round trips, the
literal four-segment bound, invalid-index rejection before I/O, and coexistence
with the base operational log, host artifact, and checkpoint under one root and
run ID. The full Rust, RustDoc, formatter, Clippy, repository, Python, and diff
gates are the evidence boundary.

## Open boundaries

Automatic rotation, crash recovery, locking/fsync, retention, external export,
runtime diagnostics, tracing/transport, scheduling, providers/models, durable
scenario-wide replay, and human operational evidence remain open.
