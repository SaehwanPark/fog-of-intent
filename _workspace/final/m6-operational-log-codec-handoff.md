# M6 Operational-Log Codec Handoff

## Outcome

Implementation is pending independent review at implementation head `ac32163`.
The slice adds the bounded `m6-scripted-agent-operational-log-v1` codec and a
distinct `.foi-operational-log` injected store namespace; event records remain
payload-free and non-authoritative.

## Verification target

The expected evidence is one focused agent codec/store regression within 27
focused agent tests, 240 Rust unit tests, 7 binary tests, and 3 RustDoc tests,
plus 15 Python policy tests, formatter, Clippy with warnings denied, repository
checker, and diff checks.

## Limits

This is an in-process bounded storage edge. Automatic runtime failure
detection, crash recovery, rotation, tracing/transport, durations,
diagnostics, external export, scheduling, durable scenario-wide replay,
providers/models, and human operational evidence remain open.
