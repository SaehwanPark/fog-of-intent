# M6 Operational-Log Sequence Status Ecology Design

## Goal and roadmap milestone

Expose a reproducible categorical check for the fixed operational event
sequence `batch_started → chunk_completed → batch_finished`. This is a
metadata-only status projection, not causal analysis or replay verification.

## Behavioral question and evidence boundary

Does a caller-declared log contain one ordered lifecycle skeleton? A complete
status requires the first required label before the chunk and the finish after
it. Optional checkpoint/resume labels may remain between chunk and finish.
Missing labels and labels that violate the required order receive closed status
IDs. Repeated calls return equal values.

## Agent families and baselines

No agent family or policy changes. The helper reads the existing operational-log
container only.

## Observation, memory, and policy inputs

Only the log's closed event IDs and order are read. No observations, true state,
resolved inputs, hashes, replay records, traces, wall-clock values, or provider
data enter the status.

## Candidate generation, evaluation, and selection

The fixed rule is `m6-operational-start-chunk-finish-v1`. Scan the payload-free
events in order, classify the first violated required phase, and return one
closed status ID. No event is appended, reordered, or repaired.

## Communication, trust, and team coordination

No communication, trust, or coordination behavior is modeled. The status has
no message or actor payload fields.

## Randomness and reproducibility

No randomness is used. Equal logs produce equal statuses; status construction
is pure and read-only.

## Scenarios, populations, and metrics

The status describes only a caller-declared event-label sequence. It is not a
causal trace, replay identity check, runtime health signal, outcome metric, or
representative operational run.

## Calibration or regression protocol

Bind literal schema/rule/status IDs; test complete, missing-start, missing-
chunk, missing-finish, reordered, repeated, and optional checkpoint/resume
sequences without mutating the log.

## Expected effects and failure signals

Expected output is a bounded categorical status only. Any request for causal
links, replay hashes, runtime diagnostics, persistence, or event production is
a stop condition.

## Verification contract

One focused agent regression must bind the closed IDs, prove canonical and
malformed statuses, permit optional checkpoint/resume labels, preserve order,
and prove read-only repeatability. Full Rust, RustDoc, formatter, Clippy,
repository, Python, and diff gates are required.

## Open questions

Causal trace completeness, replay identity, runtime producers, diagnostics,
rotation, crash recovery, persistence, transport, providers, and human
operational evidence remain open.
