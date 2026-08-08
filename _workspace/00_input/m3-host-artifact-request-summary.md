# M3 Host Artifact Request Summary

## Requested slice

Make the bounded host's existing `save`/`load` behavior an explicit,
versioned, replay-verifiable text artifact contract.

## In scope

- Encode a validated run identifier, the bounded host replay identity, each
  committed intent, the prior/result state hashes, and the full lane-record
  identity for the two-window fixture.
- Decode the artifact with bounded, fail-closed syntax and version checks.
- Enforce the documented byte and line bounds before allocating parsed lines.
- Rebuild a host history from the explicit fixture inputs and reject artifacts
  whose replay hashes do not match the current fixture inputs.
- Keep artifact handling pure and dependency-free; the command loop and binary
  remain free of filesystem I/O in this slice.
- Add round-trip, malformed-input, and divergent-input regression evidence.

## Out of scope

- Choosing a filesystem location, atomic file writes, deletion, locking, or
  cross-process resume.
- Scenario selection, arbitrary execution-input serialization, branch
  execution, prompt styling, or keyboard/screen-reader inspection.
- Exposing authoritative snapshots, state hashes, or artifact contents through
  actor-facing terminal output.

## Success evidence

- A saved one-window host artifact decodes and restores one committed record
  after the current host has advanced to a second window.
- Tampered schema, run ID, record ordering, or hashes fail closed.
- A valid-intent substitution fails through the lane-record identity even when
  the resulting state hash happens to be unchanged.
- The artifact cannot load under divergent resolved fixture inputs.
- Existing actor-visible host and command-loop transcripts remain unchanged.
