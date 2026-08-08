# M5 Actor Saved Replay-Debrief Records Design

## Contract

`CliScenarioHost::actor_replay_debrief_records_from_run` accepts a borrowed
validated `CliRunId` and returns the existing
`m5-actor-replay-debrief-record-v1` categorical records for one complete saved
two-window run. It does not return the run ID, artifact text, paths, hashes,
inputs, traces, or raw storage/replay errors.

## Verification boundary

The host loads through the injected `CliRunStore`, decodes the existing host
artifact, restores it with the receiving host's explicit execution inputs,
verifies the reconstructed history, requires exactly two records, and invokes
the existing debrief builder before projection. The restored history is local
to the call, so the current host remains unchanged.

## Evidence

One focused host test proves fresh-host retrieval from a saved complete run,
incomplete-run gating, categorical output, unchanged current
observation/history, tampered-artifact rejection, and closed-session
rejection. Existing artifact, replay, and debrief tests retain lower-layer
coverage.

## Limits

This is injected in-process file-store evidence. Filesystem race hardening,
locking, portability, crash recovery, scenario-wide durable replay, and
detailed causal-record persistence remain open.
