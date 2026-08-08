# M6 Operational Event Log Design

## Goal and roadmap milestone

Advance M6 by defining a non-authoritative operational stream boundary beside
the committed simulation history and evidence reports.

## Event contract

`ScriptedAgentOperationalEvent` is a closed vocabulary with stable IDs for
`batch_started`, `chunk_completed`, `checkpoint_saved`, `batch_resumed`, and
`batch_finished`. `ScriptedAgentOperationalLog` stores only ordered event
records and a fixed maximum of 16 entries. Records contain no payload, path,
duration, raw error, decision, result, state, hash, or trace fields.

## Construction and authority

The caller appends non-authoritative metadata to an in-memory log. The log does
not emit events, inspect the clock/process, schedule work, persist data,
reconstruct history, or own policy, transition, replay, provider, or experiment
authority. Committed simulation records remain the only source of authoritative
history.

## Verification contract

One focused agent test binds all five literal IDs, proves empty/new behavior,
stable append order, the inclusive 16-entry cap, overflow rejection, and
non-mutation after a rejected append. The full repository gates remain the
evidence boundary.

## Open boundaries

Runtime log producers, tracing/transport adapters, durations, process
diagnostics, persistence, scheduling, decision/result attachment, replay,
population experiments, provider execution, and human evidence remain open.
