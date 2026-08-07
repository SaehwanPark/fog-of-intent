# Simulation Design — M3 CLI Command Grammar

## Goal and Boundary

Define a stable adapter-level grammar whose parsed values can later be mapped
to host operations. The parser does not inspect simulation state and cannot
authorize a domain command.

## Contract

In-session commands use lowercase ASCII verbs. Read-only verbs are `help`, `observe`,
`inspect`, `review`, `debrief`, and `replay`; planning verbs are `message`,
`plan`, `contingency`, `commit`, and `advance`; history/session verbs are
`branch`, `save`, `load`, `undo`, and `quit`. `message`, `plan`,
`contingency`, `save`, and `load` require one non-empty payload; `inspect`,
`replay`, and `branch` accept an optional non-empty identifier. All other
verbs reject trailing arguments. Unknown verbs, missing payloads, and extra
arguments return typed parse errors.

The parser returns borrowed payloads and no domain types. Terminal rendering,
I/O, authorization, hidden-state filtering, and transition invocation remain
outside this module.

## Verification Contract

- Every stable verb has a typed identity and canonical name.
- Valid payload and no-payload forms parse deterministically.
- Missing, extra, and unknown tokens fail with bounded errors.
- Parsing never changes lane state or claims a CLI flow is implemented.
