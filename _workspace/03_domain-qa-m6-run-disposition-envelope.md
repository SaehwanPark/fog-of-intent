# Domain QA — M6 Run Disposition Envelope

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

The slice adds one caller-declared, payload-free disposition record with the
closed statuses `completed`, `crashed`, `timed_out`, `missing_branch`, and
`inconclusive`. It must not detect process failures, retain diagnostics, attach
decisions or results, or add execution, transition, history, replay,
persistence, provider, population, or outcome authority.

## Evidence target

One focused agent test must bind all five literal IDs, prove canonical
round-trips and wire text, and reject unknown, duplicate, missing, wrong-schema,
invalid-status, extra-line, and oversized input. The expected full evidence is
25 focused agent tests within 238 Rust unit tests, 7 binary tests, and 3
RustDoc tests, plus formatter, Clippy warnings denied, repository checker, 15
Python policy tests, and diff checks.

## Review limits

The envelope is caller-declared status preservation only. Automatic runtime
failure detection, process diagnostics, decision/result attachment, durable
export, build provenance, causality, population sampling, provider execution,
outcome metrics, and human evidence remain open.
