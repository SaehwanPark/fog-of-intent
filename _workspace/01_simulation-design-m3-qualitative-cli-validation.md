# M3 Qualitative CLI Validation Simulation Contract

## Goal and Roadmap Milestone

Validate the existing M3 reference-client loop without changing authoritative
simulation semantics.

## Slice Boundary and Non-Goals

The public entrypoints are the lane and match command loops. A test player may
submit only commands advertised by actor-visible observations/help. This slice
does not add mechanics, alter rulesets, or infer hidden state.

## Actors and Authority

The host owns true state, legality, ordering, resolved inputs, transitions,
history, replay, and debrief generation. The CLI and test players are adapters;
they may stage intent but cannot inspect or mutate authoritative state directly.

## True State, Beliefs, Observations, and Reports

Assertions use rendered actor-visible observations, categorical reports, and
documented errors only. Opponent positions/health, state hashes, execution
traces, and private receipts must remain absent or explicitly unknown.

## Plans, Commands, and Validation

Exercise `observe`, `inspect`, `plan`, `message`, `contingency`, `commit`,
`advance`, `review`, `debrief`, `replay`, `branch`, `save`, `load`, `undo`,
`help`, and `quit` for lane sessions; exercise the advertised tactical verbs,
`commit`, `advance`, `observe`, `debrief`, `help`, and `quit` for matches.
Malformed, out-of-order, post-close, and post-conclusion commands must fail
closed with actor-safe repair text.

## Resolved Inputs and Random Streams

No test player creates randomness. Existing scenario fixtures provide explicit,
versioned resolved inputs; repeated command transcripts must be reproducible.

## Events, Effects, and Transition

The CLI observes rendered transition results only. Any code fix must leave the
existing synchronous transition boundary and event/effect attribution intact.

## History, Replay, and Branching

Successful lane transcripts must preserve append-only history and verify replay;
branch output may compare parent and counterfactual outcomes without exposing
true-state hashes. Match sessions must report committed phase counts and a
categorical debrief after conclusion.

## Debrief and Causal Explanation

Review/debrief output must distinguish committed intent, realized outcome, and
causal/coordination summaries where the current fixture exposes them. Findings
must not treat agent play as human evidence.

## Verification Contract

Record exact commands, scenario IDs, outputs, and defects in the playtest report.
Run focused tests for every fix and the full repository checks before handoff.

## Open Questions

Human keyboard-only, focus, screen-reader, enjoyment, accessibility, and trust
evidence remain open roadmap gates.
