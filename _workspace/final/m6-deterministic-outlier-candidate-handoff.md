# M6 Deterministic Outlier-Candidate Handoff

## Outcome

Implementation is ready for independent review at the current branch head.

## Verification target

The focused agent regression should bind the literal candidate schema and rule,
select the first largest absolute signed delta with stable row/intent ties,
preserve positive and negative signs, prove magnitude equality and repeated
construction, and return no candidate for an unchanged comparison within 33
focused agent tests and 246 Rust unit tests, 7 binary tests, 3 RustDoc tests,
15 Python tests, formatter, Clippy, repository, and diff gates.

## Limits

This is deterministic caller-declared metric evidence only. Actual outlier
detection, threshold calibration, representative replay selection, causal
attribution, population inference, persistence, providers, and human evidence
remain open.
