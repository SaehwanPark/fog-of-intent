# M6 Operational-Log Segment Inventory Request Summary

## Target slice

Expose a bounded observational directory scan for caller-declared operational
log segments after the segment save/load boundary is established.

## Required behavior

- Validate the requested run ID before filesystem access.
- Recognize only `.foi-operational-log.segment-0` through
  `.foi-operational-log.segment-3` and return indices in sorted, deduplicated
  order.
- Ignore unrelated, malformed, temporary, and out-of-range names.
- Map missing roots, read errors, and malformed run IDs to the existing generic
  storage boundary; do not merge, load, or decode segment payloads.

## Non-goals

The scan does not infer ordering, rotation, crash state, retention, or segment
completeness. Race-hard directory snapshots, locks/fsync, crash recovery,
external export, diagnostics, transport, providers, and scenario authority
remain open.

## Verification

Cover stable `[0, 1, 3]` discovery, ignored names, missing-root and invalid-run
failures, and unchanged base/host/checkpoint coexistence. Run all pinned gates.
