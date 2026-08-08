# M3 Terminal-Rendering Boundary Design

## Goal and Roadmap Milestone

Make the terminal-rendering ownership rule explicit and inspectable: the
authoritative kernel owns state and transitions; a future CLI host may render
host-projected values at the edge.

## Slice Boundary and Non-Goals

This is a documentation reconciliation. No runtime renderer or host is added.
The current pure CLI adapter remains a request/projection contract only.

## Actors and Authority

The kernel and host own true state, legality, transition ordering, history,
replay, and debrief facts. Terminal rendering belongs to an outer adapter and
must not infer hidden state, authorize commands, or mutate history.

## True State, Beliefs, Observations, and Reports

Rendering may display only actor-valid projections and explicit provenance
labels. Research inspection remains separately privileged; the renderer cannot
turn a presentation convenience into playable truth.

## Plans, Commands, and Validation

The renderer consumes typed read results and staged/committed adapter values. It
does not validate domain legality or construct authoritative commands.

## Resolved Inputs and Random Streams

No randomness, I/O, or asynchronous work enters the transition because of
rendering. Formatting must be deterministic for a given projection.

## Events, Effects, and Transition

No event, effect, state, hash, or ruleset changes are introduced.

## History, Replay, and Branching

Rendering reads committed projections and cannot rewrite history, replay
identity, or branch inputs.

## Debrief and Causal Explanation

Future debrief text remains a projection over committed facts; presentation
formatting must not alter causal attribution or claim stronger evidence.

## Verification Contract

- Current core and CLI source contain no terminal or I/O implementation.
- Architecture and roadmap explicitly assign rendering to an outer adapter.
- No package, API, hash, replay, or test behavior changes.
- Human keyboard-only and screen-reader inspection remain unchecked.

## Open Questions

- A future host must choose a text format and rendering abstraction only after
  a complete run demonstrates the need.
- Accessibility and discoverability require later human-oriented validation.

No renderer is implemented by this slice.
