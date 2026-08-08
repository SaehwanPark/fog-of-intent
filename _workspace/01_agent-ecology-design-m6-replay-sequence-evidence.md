# M6 Replay-Sequence Evidence Ecology Design

## Goal and roadmap milestone

Bind one deterministic decision replay identity to one caller-declared
operational start/chunk/finish sequence status. This is a metadata-only M6
evidence composition.

## Behavioral question and evidence boundary

Does the recorded actor-visible decision still reproduce, and what categorical
status does the separate operational label sequence have? The two results are
reported independently so a complete label sequence cannot mask a decision
mismatch, and a verified decision cannot imply causal-trace completeness.

## Inputs and authority

The report reads only `ScriptedAgentReplayRecord::replay()` and
`ScriptedAgentOperationalLogSequenceReport::from_log`. It has no true state,
resolved inputs, hashes, traces, wall-clock values, I/O, host/lane/history,
provider, or runtime authority, and it does not append or repair events.

## Versioned contract

- Schema: `m6-scripted-agent-replay-sequence-evidence-v1`.
- Rule: `m6-replay-identity-operational-sequence-v1`.
- Replay identity IDs: `verified`, `decision_mismatch`.
- Sequence IDs remain `complete`, `missing_start`, `missing_chunk`,
  `missing_finish`, and `invalid_order`.

## Verification contract

One focused agent regression must bind the schema/rule and replay IDs, prove a
complete sequence, prove an incomplete sequence, prove a tampered decision is
classified as `decision_mismatch`, and prove the sequence remains complete in
that tampered case. Full Rust, RustDoc, formatter, Clippy, repository, Python,
and diff gates are required.

## Open boundaries

Causal-trace completeness, runtime production/detection, scenario-wide replay
identity, persistence, recovery, providers, and human operational evidence
remain open.
