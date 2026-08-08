# M6 Operational Event Log Request Summary

## Target slice

Define a bounded, non-authoritative operational event vocabulary and in-memory
log container that is structurally separate from committed simulation history,
reports, and decision/result artifacts.

## Required behavior

- Expose stable IDs for `batch_started`, `chunk_completed`, `checkpoint_saved`,
  `batch_resumed`, and `batch_finished`.
- Keep each event payload-free and separate from lane/domain history fields.
- Preserve append order in an in-memory log capped at 16 events.
- Reject appends after the cap without truncating or mutating existing entries.
- Expose schema, event IDs, length, emptiness, and ordered entries without I/O.

## Non-goals

This slice does not emit runtime logs, add tracing/async/network dependencies,
persist the log, reconstruct committed history, capture diagnostics, attach
decisions/results, or claim experiment scheduling, timing, or process-failure
detection.

## Verification

Cover all five literal event IDs, stable append order, empty/new state, the
inclusive 16-entry cap, overflow rejection, and unchanged entries after a
failed append. Run the pinned Rust, repository, Python, and diff gates.
