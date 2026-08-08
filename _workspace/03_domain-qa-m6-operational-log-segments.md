# M6 Operational-Log Segments Domain QA

## Disposition

Pending independent three-pass review at implementation head `HEAD`.

## Scope to review

- Are only segment indices `0..=3` accepted, with invalid values rejected
  before filesystem effects?
- Do segment files use their own closed suffixes and remain independent from
  the base operational log, host artifact, and batch checkpoint?
- Do the segment methods remain bounded payload-free storage edges with no
  rotation, crash recovery, diagnostics, policy, transition, history, replay,
  provider, or transport authority?

## Evidence target

One focused batch/store regression should cover segment 0/1 round trips,
literal suffixes, the inclusive/exclusive segment bounds, same-root/run-ID
coexistence, and invalid-index non-I/O behavior. The current suite is expected
to remain 27 focused agent tests within 240 unit + 7 binary + 3 RustDoc tests,
plus 15 Python tests and all formatter, Clippy, repository, and diff gates.

## Limits

The caller still owns segment ordering and rotation. Automatic rotation, crash
recovery, locking/fsync, retention, external export, runtime diagnostics,
tracing/transport, scheduling, providers/models, durable scenario-wide replay,
and human operational evidence remain open.

## Required fixes

To be determined by the independent review.
