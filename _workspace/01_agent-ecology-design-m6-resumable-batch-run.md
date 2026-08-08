# M6 Resumable Batch Run Design

## Goal and Roadmap Milestone

Advance M6 from an in-process batch runner to a bounded resumable execution
cursor backed by the existing injected run-directory boundary.

## Behavioral Question and Evidence Boundary

Can a deterministic scripted-agent batch be split into bounded chunks, persisted
after a cursor advance, and resumed without changing the ordered decisions?
This establishes execution continuity only; it does not establish sampling,
metrics, strategic quality, or human behavior.

## Agent Families and Baselines

Use the existing cautious, risk-taking, and yielding profiles and their
constructor-owned manifest metadata. No new policy family is introduced.

## Observation, Memory, and Policy Inputs

The caller supplies one actor-visible `LanerObservation` and the same ordered
manifest list on each resume. The checkpoint stores only actor/observation
binding, manifest count, cursor, and a deterministic fingerprint of those
inputs; it does not store true state, hidden values, or policy memory.

## Candidate Generation, Evaluation, and Selection

Each resumed chunk delegates to `ScriptedAgentBatchRunner`'s existing seeded
selection path. The checkpoint adapter sequences work and owns no candidate,
legality, transition, execution, history, or replay authority.

## Communication, Trust, and Team Coordination

No communication, trust, coordination, or delivery is executed.

## Randomness and Reproducibility

Each manifest's explicit seed bundle remains the only policy randomness input.
The checkpoint fingerprint binds the ordered manifest metadata and actor-visible
observation identity; equal inputs and cursor produce equal remaining decisions.

## Scenarios, Populations, and Metrics

The run is one bounded observation and at most 16 manifests. The persisted
artifact is a cursor, not a population sample, aggregate metric, report, or
decision archive.

## Calibration or Regression Protocol

Focused evidence starts a two-manifest run, advances one decision, saves and
reloads the checkpoint, resumes the remaining decision, and compares it with a
fresh full batch. Mismatched observation/manifest input and malformed cursors
fail closed.

## Expected Effects and Failure Signals

Valid chunks return ordered decisions and an advanced cursor. Empty/over-capacity
batches, invalid cursors, input mismatches, and storage/decode failures return
bounded errors before policy execution or state mutation.

## Verification Contract

One focused agent test covers checkpoint codec and cursor progression; one
focused store test covers save/load across a chunk boundary. The full suite and
repository gates remain the evidence boundary.

## Open Questions

Decision/result persistence, crash diagnostics, population sampling, metrics,
report export, provider/model execution, scenario-wide replay, and scheduling
remain open.
