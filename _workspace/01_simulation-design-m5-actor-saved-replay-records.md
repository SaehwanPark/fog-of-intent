# M5 Actor Saved Replay Records Design

## Contract

`CliScenarioHost::actor_replay_records_from_run` accepts a borrowed validated
`CliRunId` and returns the existing `m5-actor-replay-record-v1` categorical
records. It does not return the run ID, artifact text, hashes, inputs, traces,
or raw storage/replay errors.

## Verification boundary

The host loads through the injected `CliRunStore`, decodes the existing host
artifact, restores it with the host's explicit execution inputs, verifies the
reconstructed history, and projects only after all checks succeed. The current
host remains unchanged because the restored history is local to the call.

## Evidence

One focused host test proves fresh-host retrieval from a saved first-window
artifact, unchanged current observation/history, tampered-artifact rejection,
and closed-session rejection. Existing artifact codec and replay tests retain
the lower-layer coverage.

## Limits

This is injected in-process file-store evidence. Filesystem race hardening,
locking, portability, crash recovery, scenario-wide durable replay, and causal
record persistence remain open.
