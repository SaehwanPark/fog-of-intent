# M3 Host Artifact Design

## Boundary

`src/host_artifact.rs` is a pure edge codec. It serializes only a bounded host
artifact contract; it does not own simulation authority, choose resolved
inputs, inspect hidden state, or perform filesystem I/O. `src/host.rs` remains
the sole lifecycle and transition authority and validates decoded artifacts by
replaying the current explicit inputs through the lane contract.

## Artifact contract

The versioned schema is `m3-cli-host-artifact-v1`. The header records the
artifact schema, the fixed `m2-two-window-scenario-v3` replay identity, the
validated run identifier, and the number of committed records. Each record
contains its contiguous index, intent, prior-state hash, and resulting
state-hash. Only the bounded two-window fixture is accepted.

Encoding is deterministic and uses one space-delimited header followed by one
record per committed window. Run IDs use the existing 1–64-byte ASCII
identifier contract. Unknown fields, duplicate fields, invalid enums, extra
lines, unsupported versions, and hash mismatches fail closed.

## Restore contract

Decoding produces an owned artifact value. The host reconstructs a fresh
scenario history from its already-resolved inputs and the artifact intents,
then compares every recorded prior/result hash and the replay identity. A
different input fixture cannot silently produce a different restored run.

## Evidence and limits

This slice proves deterministic artifact encoding, validation, and in-process
restore only. It does not prove persistence across process restarts because no
filesystem store is introduced. File placement/atomicity, scenario selection,
branch execution, and human accessibility inspection remain open.
