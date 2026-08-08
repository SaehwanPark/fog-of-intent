# M6 Operational Event Producer Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentBatchRunner::run_with_operational_log` accepts the existing
actor-visible batch inputs and caller-owned `ScriptedAgentOperationalLog`.
After validation and capacity preflight it appends the exact ordered
`batch_started`, `chunk_completed`, and `batch_finished` labels and returns the
same decisions as the direct batch runner. Failed validation or insufficient
capacity leaves the log unchanged.

## Verification target

One focused agent test covers decision parity, lifecycle order, invalid-batch
nonmutation, capacity-preflight nonmutation, and the literal 16-entry bound.
The expected full evidence is 27 focused agent tests within 240 unit tests, 7
binary tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied,
repository checker, 15 Python policy tests, and diff checks.

## Open boundaries

Checkpoint/resume event production, runtime failure detection, diagnostics,
tracing/transport, persistence, scheduling, decision/result attachment,
replay, population experiments, providers, and human evidence remain open.
