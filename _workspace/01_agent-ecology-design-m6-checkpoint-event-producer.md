# M6 Checkpoint Event Producer Design

## Goal and roadmap milestone

Complete the caller-driven operational trace around the existing injected
cursor store without treating filesystem activity as automatic runtime
detection or authoritative experiment history.

## Producer contract

`ScriptedAgentBatchRunStore::save_with_operational_log` preflights one slot,
delegates to the existing bounded atomic checkpoint save, and appends exactly
`checkpoint_saved` only after success. `load_with_operational_log` preflights
one slot, delegates to the existing bounded read/decode, and appends exactly
`batch_resumed` only after success. Storage-unavailable and invalid-checkpoint
errors do not append events; a full log returns a bounded capacity error before
I/O.

## Construction and authority

The store adapters reuse the existing injected filesystem boundary and the
caller-owned non-authoritative log. They do not emit process diagnostics,
infer timing or failure, persist the operational log, or own policy, transition,
history, replay, provider, population, or scheduling authority. Direct save
and load APIs remain compatible.

## Verification contract

The existing focused checkpoint/store test proves host-artifact coexistence,
successful save/load event labels, and full-log nonmutation when capacity is
preflighted. The existing operational-log test continues to bind all five
literal event IDs and the inclusive 16-entry cap.

## Open boundaries

Automatic failure detection, diagnostics, event-log persistence, tracing/
transport, scheduling, decision/result attachment, richer checkpoint/replay
semantics, population experiments, providers, and human evidence remain open.
