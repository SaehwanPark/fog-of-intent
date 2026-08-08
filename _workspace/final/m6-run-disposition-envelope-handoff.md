# M6 Run Disposition Envelope Handoff

## Outcome

Pending independent domain QA and final handoff review.

## Delivered contract

`ScriptedAgentRunDispositionRecord` exposes the exact
`m6-scripted-agent-run-disposition-v1` two-line codec for the five closed
caller-declared IDs `completed`, `crashed`, `timed_out`, `missing_branch`, and
`inconclusive`. The record is payload-free and does not inspect processes,
diagnostics, decisions, results, true state, or execution.

## Verification target

One focused agent test covers all five IDs, canonical round-trips, exact wire
text, malformed-field/status/schema/line cases, and the 4096-byte bound. The
expected full evidence is 25 focused agent tests within 238 unit tests, 7
binary tests, and 3 RustDoc tests, plus formatter, Clippy warnings denied,
repository checker, 15 Python policy tests, and diff checks.

## Open boundaries

Automatic crash/timeout detection, process diagnostics, decision/result
attachment, durable export, independent build provenance, causal attribution,
population sampling, provider execution, outcome metrics, and human evidence
remain open.
