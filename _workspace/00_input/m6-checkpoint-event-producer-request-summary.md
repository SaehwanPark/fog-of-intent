# M6 Checkpoint Event Producer Request Summary

## Target slice

Add caller-driven `checkpoint_saved` and `batch_resumed` production at the
existing injected checkpoint-store boundary.

## Required behavior

- Keep the existing direct checkpoint `save` and `load` behavior unchanged.
- Preflight one operational-log slot before any storage I/O.
- Append `checkpoint_saved` only after a successful bounded cursor save.
- Append `batch_resumed` only after successful bounded cursor load and decode.
- Map storage/codec failures without appending an event; capacity failure must
  happen before I/O and preserve the complete log.

## Non-goals

This slice does not detect crashes/timeouts, inspect process state, attach
decisions/results/diagnostics, persist event logs, add tracing/transport, or
change cursor, policy, transition, history, replay, provider, population, or
scheduling authority.

## Verification

Extend the existing checkpoint/store regression with successful save/load event
assertions and a full-capacity preflight rejection that leaves the log intact.
Run the pinned Rust, repository, Python, and diff gates.
