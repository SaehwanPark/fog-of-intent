# M6 Operational-Log Segment Inventory Domain QA

## Disposition

Pending independent three-pass review at implementation head `HEAD`.

## Scope to review

- Does the inventory validate run IDs before I/O and return only the closed
  segment range in sorted, deduplicated order?
- Does it ignore unrelated/temp/malformed/out-of-range files without decoding
  or exposing payloads?
- Does it remain observational, generic on failures, and separate from
  rotation, crash recovery, policy, transition, history, replay, and provider
  authority?

## Evidence target

One focused batch/store regression should cover stable `[0, 1, 3]` discovery,
base/segment coexistence, missing-root and invalid-run failures, and the
existing segment bounds. The current suite is expected to remain 27 focused
agent tests within 240 unit + 7 binary + 3 RustDoc tests, plus 15 Python tests
and all formatter, Clippy, repository, and diff gates.

## Limits

The scan is not a race-hard snapshot and does not infer rotation, completeness,
crash state, retention, export, diagnostics, transport, scheduling, providers,
durable scenario-wide replay, or human operational evidence.

## Required fixes

To be determined by the independent review.
