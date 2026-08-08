# M6 Profile-Aware Tally Codec Handoff

## Outcome

Implementation and evidence are complete at head `0d2a323`; the independent
three-pass review passed with no actionable findings. The slice binds the
profile-aware tally to its existing evidence codec without adding persistence.

## Verification

One focused profile-aware tally codec regression covers canonical schema and
rows, verified round-trip, and tampered-row rejection. The full evidence is 30
focused agent tests within 243 Rust unit + 7 binary + 3 RustDoc tests, plus 15
Python tests; formatter, Clippy, repository, and diff gates pass at `0d2a323`.

## Limits

This is bounded evidence transport only. Durable export, broader metrics and
distributions, outcomes, calibration, persistence, providers, and human
evidence remain open.
