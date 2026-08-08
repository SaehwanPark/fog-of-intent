# M6 Operational-Log Sequence Status Request Summary

## Requested outcome

Define one bounded deterministic status over a caller-declared operational log:
whether the payload-free labels contain the fixed `batch_started`,
`chunk_completed`, `batch_finished` lifecycle in order. The status must remain
separate from causal-trace completeness and replay identity.

## Roadmap milestone

M6 — Automated Behavioral Validation. This slice advances a narrow event-log
sequence check while leaving causal tracing, replay identity, runtime
production, and recovery open.

## Behavioral question and evidence boundary

Can a bounded operational log classify a complete ordered lifecycle and reject
missing or reordered lifecycle labels deterministically? Evidence is limited to
closed event IDs and a categorical status; it does not infer causality,
runtime success, replay identity, or process behavior.

## In scope

- A closed `m6-scripted-agent-operational-log-sequence-v1` status identity.
- A fixed `m6-operational-start-chunk-finish-v1` sequence rule.
- Categorical statuses for complete, missing-start, missing-chunk,
  missing-finish, and invalid-order logs.
- Pure `&ScriptedAgentOperationalLog` inspection without mutation or I/O.
- Focused evidence for canonical complete, missing, reordered, repeated, and
  deterministic status cases.

## Non-goals and stop conditions

- Do not inspect true state, hashes, traces, timing, providers, or replay data.
- Do not claim causal-trace completeness, replay identity, runtime failure
  detection, process success, persistence, rotation, or recovery.
- Do not add event production, scheduling, transport, host/lane/history, or
  scenario authority.
- Stop if classification requires hidden input or a runtime producer.

## Expected files and verification

Likely targets are `src/agent.rs` for the closed status type, helper, and
focused test, plus canonical docs, LESSONS, and workspace QA/handoff artifacts.
Run all pinned Rust, repository, Python, and diff gates.
