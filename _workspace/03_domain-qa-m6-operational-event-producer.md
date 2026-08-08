# Domain QA — M6 Operational Event Producer

## Disposition

Pending independent three-pass review at the implementation/evidence head.

## Scope to review

The slice adds a caller-driven producer around one complete deterministic
in-process batch. It validates inputs and preflights capacity before policy
evaluation, appends exactly `batch_started`, `chunk_completed`, and
`batch_finished` on success, and preserves the existing ordered decisions.
Invalid batches and insufficient log capacity must leave the caller-owned log
unchanged. It must not produce checkpoint/resume events, inspect time/process
state, detect failures, persist data, or add policy, transition, history,
replay, provider, population, or scheduling authority.

## Evidence target

One focused agent test must prove decision parity and exact lifecycle order,
invalid-batch nonmutation, and capacity-preflight nonmutation with the literal
16-entry bound. The expected full evidence is 27 focused agent tests within
240 Rust unit tests, 7 binary tests, and 3 RustDoc tests, plus formatter,
Clippy warnings denied, repository checker, 15 Python policy tests, and diff
checks.

## Review limits

This is a caller-driven in-process producer only. Checkpoint/resume producers,
runtime failure detection, diagnostics, tracing/transport, persistence,
scheduling, decision/result attachment, replay, population experiments,
providers, and human evidence remain open.
