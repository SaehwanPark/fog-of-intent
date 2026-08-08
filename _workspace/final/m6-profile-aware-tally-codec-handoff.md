# M6 Profile-Aware Tally Codec Handoff

## Outcome

Implementation and evidence are pending independent three-pass review at the
implementation/evidence head.

## Verification target

One focused profile-aware tally codec regression should cover canonical schema
and rows, verified round-trip, and tampered-row rejection. The full target is
30 focused agent tests within 243 Rust unit + 7 binary + 3 RustDoc tests, plus
15 Python tests and formatter, Clippy, repository, and diff gates.

## Limits

This is bounded evidence transport only. Durable export, broader metrics and
distributions, outcomes, calibration, persistence, providers, and human
evidence remain open.
