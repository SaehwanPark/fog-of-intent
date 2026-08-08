# M3 Run-Identifier Design

## Goal and Roadmap Milestone

Give save/load/replay/export adapter requests a bounded, human-readable run-ID
type while leaving persistence and host lifecycle outside the current slice.

## Slice Boundary and Non-Goals

`CliRunId<'a>` borrows the original argument and accepts 1–64 ASCII bytes. The
first character must be alphanumeric; subsequent characters may be alphanumeric,
`-`, `_`, or `.`. No normalization or allocation occurs. Branch point IDs and
filesystem paths are separate values and remain out of scope.

## Actors and Authority

The application host remains the sole owner of session lifecycle, persistence,
history, replay, and authorization. The CLI adapter only validates syntax and
passes a typed borrowed identifier to that host.

## True State, Beliefs, Observations, and Reports

Run IDs carry no simulation state or actor information. They identify a future
host artifact and cannot expose hidden state.

## Plans, Commands, and Validation

`CliSessionRequest::Save/Load`, `CliProcessRequest::Replay`, and
`CliTopLevelRequest::Replay/Export` carry `CliRunId`. Empty values retain their
existing empty-input errors; non-empty malformed IDs fail with a typed
`CliRunIdError` before host execution.

## Resolved Inputs and Random Streams

No randomness, resolved input, or transition behavior changes.

## Events, Effects, and Transition

No domain event, effect, state, hash, or ruleset changes.

## History, Replay, and Branching

The ID is only an adapter reference. It does not load, save, mutate, or
authorize a history record, and it is not a replay identity or branch point.

## Debrief and Causal Explanation

No debrief behavior changes. A future debrief exporter may use a validated run
ID after host authorization and persistence are implemented.

## Verification Contract

- Stable `m3-cli-run-id-v1` schema identifier.
- Valid readable forms, length bounds, and invalid-character rejection are
  covered by tests.
- All affected request mappers return typed IDs and reject malformed values.
- Empty-command behavior and unaffected grammar remain unchanged.

## Open Questions

- Persistence must define collision handling, filesystem encoding, and run-ID
  generation before save/load execution is implemented.
- Human discoverability and transcript wording remain open.

No host persistence is implemented by this slice.
