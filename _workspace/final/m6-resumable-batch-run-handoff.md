# M6 Resumable Batch Run Handoff

## Outcome

Implementation is complete; independent three-pass review is pending.
The checkpoint binds one ordered actor-visible batch to a bounded cursor, and
the injected run-directory adapter persists only that cursor for deterministic
chunk resume.

## Verification target

The implementation provides one focused agent checkpoint/store test. The
expected full evidence is 231 Rust unit tests, 7 binary tests, and 3 RustDoc
tests, with the focused agent suite at 18 tests; formatter, Clippy with
warnings denied, repository checker, 15 Python policy tests, and diff checks
must pass at the reviewed head.

## Limits

This is bounded in-process decision continuity with injected cursor storage.
Decision/result persistence, crash diagnostics, populations, sampling, metrics,
report export, providers/models, scenario-wide replay, and calibration remain
open.
