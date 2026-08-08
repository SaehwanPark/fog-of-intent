# M6 Operational-Log Segments Request Summary

## Target slice

Add a bounded storage primitive for caller-declared operational-log segments
without changing the existing payload-free codec or inventing automatic
rotation and crash recovery.

## Required behavior

- Store and load segment indices `0..=3` using distinct
  `.foi-operational-log.segment-*` paths and the existing atomic injected
  replacement boundary.
- Reject segment `4` and above before filesystem I/O with a bounded generic
  error.
- Keep base operational logs, host artifacts, checkpoints, and each segment
  independently loadable under the same root and run ID.
- Preserve the existing closed codec, 4096-byte bound, 16-event bound, and
  payload-free event vocabulary.

## Non-goals

The caller owns segment ordering and rotation policy. This slice does not add
automatic rotation, crash recovery, locking, fsync, retention, external
export, runtime diagnostics, tracing, scheduling, transport, providers, or
scenario/history authority.

## Verification

Cover two segment round trips, literal suffixes and inclusive/exclusive index
bounds, same-root/same-run-ID coexistence with base host/checkpoint/log files,
and no filesystem access for invalid indices. Run all pinned repository gates.
