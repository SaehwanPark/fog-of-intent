# M6 Candidate Replay Reference Handoff

## Outcome

Pending independent review.

## Verification target

The focused agent regression should bind exact reference identities, prove
first matching order, candidate-label/observation projection, decision-mismatch
handling, and no-match handling without mutation. The expected full evidence
is 37 focused agent tests within 250 Rust unit tests, 7 binary tests, and 3
RustDoc tests, 15 Python tests, formatter, Clippy, repository, and diff gates.

## Limits

This is pure caller-declared candidate-to-replay reference evidence. It does
not prove representative replay, scenario-wide replay, calibrated outlier
detection, build provenance, causality, persistence, providers, or human
evidence.
