# Simulation Design — M1 Bounded Deterministic Transition Fixture

## Goal and Roadmap Milestone

Prove the smallest host-owned deterministic transition boundary for M1. The
fixture has one actor, one bounded energy resource, one score counter, and two
commands. It establishes typed validation, explicit resolved inputs, attributed
events/effects, stable state hashing, append-only in-memory replay, and a strict
versioned text fixture codec before lane mechanics or persistence are
introduced.

## Slice Boundary and Non-Goals

In scope:

- `WorldState` containing one actor, a ruleset identifier, and a turn;
- bounded `Units` for energy, requested spend, and resolved yield;
- `Hold` and `Gather { spend }` commands;
- a separate validation result before transition evaluation;
- explicit environment, observation, policy, coordination, and execution input
  categories;
- a legal `Gather` whose resolved yield is zero, demonstrating an unfavorable
  execution outcome distinct from invalid command rejection;
- ordered events, attributed effects, a deterministic hash, committed records,
  and replay verification.

Out of scope: actor beliefs or reports, lane/scenario rules, multiple actors,
communication, branching APIs, external persistence, CLI/MCP adapters, random
generation, async execution, migrations, and debrief presentation.

## Actors and Authority

The host owns the true `WorldState`, command-window ordering, ruleset selection,
resolved inputs, transition invocation, committed history, and replay authority.
The kernel is a pure evaluator over owned values. `ActorId` identifies the one
fixture actor but does not grant authority or expose a projection of hidden
state. The binary and future adapters remain outside this slice.

## True State, Beliefs, Observations, and Reports

`WorldState` is true authoritative state and contains the actor's energy, score,
turn, and ruleset. This fixture does not create an actor belief, observation, or
report type because it has no information boundary to exercise yet. Execution
inputs are resolved inputs, not observations or reports, and are never inferred
inside the transition.

## Plans, Commands, and Validation

`Command` carries actor, turn, ruleset, expected prior hash, and an action.
`validate_command` checks actor identity, exact turn ordering, ruleset equality,
expected prior hash, nonzero gather spend, and available energy. It returns a
`ValidatedCommand` that cannot be constructed by callers without passing this
boundary. Invalid commands return typed errors and do not produce events or
effects.

`Hold` is valid and has no state effect. `Gather` spends the requested energy
and applies the resolved yield. A zero resolved yield is valid but unfavorable:
the command is accepted, energy is spent, a gathered event is recorded, and no
score is awarded.

## Resolved Inputs and Random Streams

`ResolvedInputs` has five named categories: environment, observation, policy,
coordination, and execution. Each category carries a stable `StreamId` and
`DrawId`; execution additionally carries a bounded resolved yield. The kernel
never creates random values or selects draws. The first fixture uses only the
execution yield and attributes the score award to its execution stream/draw.
Changing unrelated category identities leaves the transition result unchanged,
which makes stream isolation explicit.

## Events, Effects, and Transition

The pure transition boundary is:

```text
WorldState + ValidatedCommand + ResolvedInputs
  -> TransitionResult | TransitionError
```

`TransitionResult` returns the next state, ordered events, ordered attributed
effects, and its authoritative next-state hash. `Gathered` records requested and
resolved yield. `EnergySpent` is attributed to the command, while positive
`ScoreAwarded` effects name the execution stream and draw that caused the
resolved award. State updates conserve energy by
subtracting exactly the requested bounded spend and increase score by exactly
the resolved bounded yield.

## History, Replay, and Branching

`History` owns an initial state, current state, and append-only vector of
`TransitionRecord` values. Appending validates and evaluates one command, then
stores the raw command, resolved inputs, prior hash, and complete transition
result. Replay starts at the initial state, checks each prior hash, revalidates
and reevaluates each record, compares the stored result, and returns the
terminal state or a typed divergence error. Branching and record removal remain
deferred; no mutation API removes committed records.

The M1 codec serializes snapshots and histories as canonical line-oriented text
with schema version `1.0.0` and hash representation `fnv1a64-le-v1`. It records
the initial state, exact commands, all five input-category identities, prior
hashes, ordered events/effects, next state, and next hash. Deserialization
reconstructs history through the kernel and fails closed on unsupported versions,
unknown fields, malformed values, or mismatched replay results.

## Debrief and Causal Explanation

The fixture does not implement a debrief surface. Its event/effect separation
and execution attribution are the causal data needed by a later debrief; this
slice makes no claim that a player-facing explanation exists.

## Verification Contract

Tests must establish:

- identical state, validated command, resolved inputs, and ruleset yield equal
  events, effects, next state, and hash;
- invalid zero-spend, overspend, stale-hash, wrong-turn, wrong-actor, and
  wrong-ruleset commands fail before transition evaluation;
- zero-yield gather is legal, spends energy, and leaves score unchanged;
- energy and score bounds/conservation hold for valid gathers;
- history replay verifies every transition and reaches the terminal state;
- versioned snapshot/history fixtures round-trip and reject tampered or
  unsupported records;
- duplicate/out-of-order commands are rejected by exact turn validation;
- changing unrelated input streams does not change the result;
- exhaustive bounded spend/yield checks preserve energy bounds, conservation,
  and score/yield invariants;
- no kernel path reads I/O, wall clock, environment, async services, or RNG.

## Open Questions

- Migration support and externally supported replay bundles need a later M1
  design once this local 1.0.0 fixture shape proves stable.
- A future scenario will decide whether energy and score remain generic units or
  become scenario-specific resources.
- Actor-visible observations and reports begin with the first information-
  asymmetric scenario, not this authority fixture.
