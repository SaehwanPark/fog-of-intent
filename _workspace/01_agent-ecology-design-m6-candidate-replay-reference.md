# M6 Candidate Replay Reference Ecology Design

## Goal and roadmap milestone

Bind one verified largest-delta candidate to the first caller-declared replay
record with matching profile, evaluation rule, and selected intent.

## Behavioral question and evidence boundary

Is there a caller-declared replay record that matches the metric candidate and
still replays exactly? The output is a reference label, not a claim that the
record is representative or causally explanatory.

## Inputs and authority

The selector reads only the existing candidate and replay records. It checks
the existing deterministic replay function and preserves input order. It does
not inspect true state, hashes, traces, runtime/process data, I/O, host/lane/
history, providers, or persistence, and it does not rerun policy evaluation.

## Versioned contract

- Schema: `m6-scripted-agent-tally-replay-reference-v1`.
- Rule: `m6-first-verified-candidate-replay-v1`.
- Match keys: profile ID, evaluation rule, selected intent.
- Errors: `no_matching_replay`, `decision_mismatch`.

## Verification contract

One focused agent regression must prove first-match ordering, exact candidate
labels and observation ID, a matching decision mismatch, and no matching
record. Full Rust, RustDoc, formatter, Clippy, repository, Python, and diff
gates are required.

## Open boundaries

Representative-replay proof, scenario-wide replay, calibrated outlier
detection, build provenance, causal attribution, persistence, providers, and
human evidence remain open.
