# M6 Profile-Aware Tally Codec Domain QA

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

- Does the focused integration bind canonical schema and all three row IDs/counts?
- Does verified round-trip succeed and tampered-row decode fail as
  `InputMismatch`?
- Does the codec remain bounded evidence transport with no persistence or
  policy/host/lane/history/replay/provider authority?

## Evidence target

One focused profile-aware tally codec regression should cover canonical rows,
verified round-trip, and tampered-row rejection. The full gate target is 30
focused agent tests within 243 Rust unit + 7 binary + 3 RustDoc, 15 Python
tests, formatter, Clippy with warnings denied, repository checker, and diff
checks.

## Limits

The codec is fixture-sized evidence transport only. Durable export, broader
metrics/distributions, outcomes, calibration, persistence, providers, and
human evidence remain open.

## Required fixes

To be completed after independent review.
