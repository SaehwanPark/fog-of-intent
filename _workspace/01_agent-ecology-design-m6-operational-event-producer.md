# M6 Operational Event Producer Design

## Goal and roadmap milestone

Advance M6 by producing a tiny deterministic lifecycle trace around one
complete in-process batch without turning operational metadata into simulation
history or runtime diagnostics.

## Producer contract

`ScriptedAgentBatchRunner::run_with_operational_log` accepts the same
actor-visible observation and ordered manifest slice as `run`, plus a mutable
`ScriptedAgentOperationalLog`. It validates the batch first and requires room
for exactly three events before evaluating any policy. A successful call
appends `batch_started`, `chunk_completed`, and `batch_finished` in that order
and returns the same ordered decisions as `run`.

The bounded producer error distinguishes an invalid batch from an operational
log capacity failure. Both failures leave the caller's log unchanged. The
producer is caller-driven and deterministic; it does not infer timing,
process failure, checkpoint state, or result meaning.

## Construction and authority

The adapter delegates decision evaluation to the existing batch runner and
appends payload-free labels to the existing non-authoritative log. It owns no
policy, legality, transition, committed history, replay, provider, population,
transport, persistence, or scheduling authority. Checkpoint and resume event
production remain separate future boundaries.

## Verification contract

One focused agent test proves success decision parity and exact three-event
order, rejects an empty batch without mutating a pre-existing log, and rejects
an over-capacity call before evaluation while preserving the full log. The
tests bind the literal 16-entry cap and the five-event vocabulary remains
covered by the existing operational-log test.

## Open boundaries

Checkpoint-saved and batch-resumed producers, runtime/process diagnostics,
tracing/transport, persistence, scheduling, decision/result attachment,
replay, population experiments, providers, and human evidence remain open.
