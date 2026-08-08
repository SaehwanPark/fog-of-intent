# M6 Operational Event Producer Request Summary

## Target slice

Add one caller-driven producer around the deterministic in-process batch
runner. It records only the bounded lifecycle labels that the existing
operational log vocabulary already defines.

## Required behavior

- Expose a batch-runner adapter that accepts an existing mutable operational
  log and the same actor-visible observation and ordered manifests as `run`.
- Validate the batch and preflight log capacity before policy evaluation or log
  mutation.
- On success, append exactly `batch_started`, `chunk_completed`, and
  `batch_finished` in that order and return the same ordered decisions as
  `ScriptedAgentBatchRunner::run`.
- If the batch is invalid or three entries do not fit, return a bounded error
  and leave both decisions and the log unchanged.

## Non-goals

This slice does not produce checkpoint-saved or batch-resumed events, inspect
time/process state, detect failures, attach diagnostics/results, persist logs,
add tracing/transport, or change policy, transition, history, replay, provider,
population, or scheduling authority.

## Verification

Cover successful lifecycle order and decision parity, invalid-batch
nonmutation, and capacity-preflight nonmutation with a literal 16-entry log
bound. Run the pinned Rust, repository, Python, and diff gates.
