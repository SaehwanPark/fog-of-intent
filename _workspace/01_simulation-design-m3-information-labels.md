# M3 Information-Label Design

## Goal and Roadmap Milestone

Define the smallest CLI-facing vocabulary that lets a future host label actor-
visible values as `observed`, `believed`, `inferred`, `reported`, or `unknown`
without changing simulation authority. This is an M3 adapter contract slice.

## Slice Boundary and Non-Goals

The contract is a dependency-free Rust type in `src/cli.rs`. It is not a
renderer, parser command, belief engine, report transport, persistence format,
or authoritative state field. The existing M2 lane contracts remain unchanged.

## Actors and Authority

The host and kernel remain the sole owners of true state, legality, transition,
history, replay, and debrief facts. A CLI consumer may receive a typed value
only after the host has projected it through the actor-visible boundary. The
label describes the provenance available to that consumer; it does not grant
additional authority.

## True State, Beliefs, Observations, and Reports

`CliInformation<T>` has five disjoint forms:

- `Observed(T)`: directly available to the actor through its current
  observation.
- `Believed(T)`: the actor's current belief, which may be stale and is not true
  state.
- `Inferred(T)`: a conclusion derived from available information rather than a
  direct observation.
- `Reported(T)`: a value attributed to another actor or communication source;
  the adapter does not validate its truth.
- `Unknown`: unavailable or intentionally redacted information with no payload.

`CliInformationLabel` supplies the stable vocabulary and canonical names.
`CliInformation<T>::label()` exposes the label without exposing hidden state;
`Unknown` cannot contain a `T`. Borrowed projections preserve the same label.

## Plans, Commands, and Validation

No new command or domain validation is introduced. Labels are metadata on
future read projections and cannot become intents, commands, or execution
inputs.

## Resolved Inputs and Random Streams

No resolved input or random stream is added. Label selection must be made by the
host projection that already owns the relevant actor-valid evidence.

## Events, Effects, and Transition

No state transition, event, effect, or hash changes. The type is presentation-
neutral and pure.

## History, Replay, and Branching

No history or replay identity changes. A future serialized projection may adopt
the schema identifier `m3-cli-information-labels-v1`, but this slice does not
create an external artifact or compatibility promise.

## Debrief and Causal Explanation

The labels support later debrief rendering but do not classify decision,
coordination, execution, or luck themselves. A debrief must continue to use
committed facts and actor-valid evidence.

## Verification Contract

- Every label has a stable canonical name.
- `Unknown` is marked redacted and has no value payload.
- Each valued form returns its matching label.
- Borrowing a valued form preserves its label and value.
- The schema identifier is stable and explicitly versioned.
- Existing M1/M2 tests, hashes, replay fixtures, and CLI grammar behavior remain
  unchanged.

## Open Questions

- The future host must decide how labels are rendered in guided and expert text.
- A later protocol may need source/turn metadata for reported or believed
  values; adding it is outside this slice.
- Human distinction, screen-reader clarity, and accessibility remain untested.
