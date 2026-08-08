# M6 Outlier-Threshold Signal Handoff

## Outcome

Pending independent review.

## Verification target

The focused agent regression should bind the exact threshold contract and
prove magnitude-2 acceptance, magnitude-1 rejection, and no-candidate
handling without mutating the verified comparison. The expected full evidence
is 36 focused agent tests within 249 Rust unit tests, 7 binary tests, and 3
RustDoc tests, 15 Python tests, formatter, Clippy, repository, and diff gates.

## Limits

This is pure in-process fixed-fixture threshold evidence. Calibrated outlier
detection, representative replay selection, causal attribution, population
inference, persistence, providers, and human evidence remain open.
