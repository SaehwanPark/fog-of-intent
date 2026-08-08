# M6 Run Disposition Envelope Design

## Goal and roadmap milestone

Advance M6 by preserving bounded caller-declared run outcomes without making
the deterministic agent library responsible for detecting or interpreting
external failures.

## Disposition contract

`ScriptedAgentRunDisposition` is a closed enum with these stable IDs:
`completed`, `crashed`, `timed_out`, `missing_branch`, and `inconclusive`.
`ScriptedAgentRunDispositionRecord` carries only the versioned schema and one
disposition. It is a status envelope, not a decision, result, report, or
diagnostic record.

## Construction and authority

The caller declares the disposition through a constructor. The record does not
run an agent, inspect true state, schedule time, access a process, read a
stack, resolve execution, or own host, lane, transition, history, replay,
persistence, provider, or population authority.

## Codec boundary

The `m6-scripted-agent-run-disposition-v1` codec is exactly two newline-
terminated fields: `schema` and `disposition`. A 4096-byte pre-parse bound,
closed field set, and closed status parser reject malformed text before a
trusted record is returned. The codec carries no payload or raw failure detail.

## Verification contract

One focused agent test binds the five literal IDs, round-trips every status,
asserts the canonical wire form, and rejects unknown, duplicate, missing,
wrong-schema, invalid-status, extra-line, and oversized input. The full
repository gates remain the evidence boundary.

## Open boundaries

Automatic crash/timeout detection, process diagnostics, result/decision
attachment, durable export, build provenance, causal attribution, population
sampling, provider execution, outcome metrics, and human evidence remain open.
