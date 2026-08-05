# Simulation Design — M2 Bounded Counterfactual Branch

## Goal and Roadmap Milestone

This slice advances M2 — One-Lane Vertical Slice from the implemented
`m2-one-lane-window-v1` diagnostic window to one bounded counterfactual at that
window's pivotal decision. It is a host/research comparison artifact, not a
second playable window and not a general branching framework.

The branch must reuse the current `src/lane.rs` contract wherever possible:
the existing `LaneSnapshot`, `LanerObservation`, `LaneObservationReceipt`,
`LaneIntentRequest`, `LaneIntentCommand`, validation functions,
`LaneResolvedInputs`, `transition_lane`, `LaneTransitionResult`,
`LaneTransitionRecord`, `LaneHistory`, and `LaneDebrief` retain their meaning.
The branch composes those values in a small `LaneBranch` envelope.

The end-to-end boundary is:

```text
verified parent LaneHistory with exactly one committed window
  -> record 0 pre-transition snapshot and actor observation
  -> alternate existing LaneIntentRequest
  -> matched parent execution OR explicitly regenerated execution
  -> existing host validation and transition
  -> one LaneBranch result + branch replay identity + bounded comparison
```

The branch isolates one decision only. It does not continue the branch into a
second observation, another command, or another transition.

## Slice Boundary and Non-Goals

Included:

- a parent `LaneHistory` containing one valid, replayable window-1 record;
- a fixed branch point at record index `0`, immediately before the parent
  command was evaluated, with the parent initial state still `Open`;
- the same actor-valid `LanerObservation` that existed at the parent decision;
- one alternate `LaneIntentRequest`, limited to the other existing intent
  (`Stabilize` versus `Contest`);
- one execution-selection mode: reuse the parent's exact execution input, or
  use one new execution input supplied by the edge resolver with a stable
  branch-scoped trace;
- one branch transition using the existing lane transition and one
  branch-local replay verification;
- a bounded comparison that separates decision, execution condition, and luck
  without claiming an optimal action or a full-scenario result.

The parent and branch each have one resolved window. The branch has no new
`LaneSnapshot` schema, no new actor, no new intent, and no new event/effect
kind. Branch metadata is not authoritative world state and is not included in
the lane state hash.

Explicit exclusions are second-window mechanics, variable pacing, allied
policy, communication, proposals, autonomous opponent policy, recall, gank
response, CLI, MCP, GUI, persistence codecs, portable branch bundles, branch
trees, arbitrary branch points, branch merge, general branching frameworks,
and human-experience or balance claims.

## Actors and Authority

The ordinary actors remain unchanged: `PLAYER_LANER` is the only decision-maker;
`OPPONENT_LANER` and `JungleThreatTruth` remain host-owned true-state fields
with no policy interface. The branch is not an action available during
ordinary play. A host-owned experiment/controller boundary requests a branch
after the parent record is committed and verified.

The host owns:

- the parent `LaneHistory` and its true snapshots;
- the branch-point selection and parent replay verification;
- the actor observation and its private source-state binding;
- conversion of the alternate request into a host-created command;
- the choice to match execution or accept an explicitly resolved regenerated
  execution input;
- branch identity, provenance, result comparison, and replay verification.

The deterministic kernel still owns only validation and transition evaluation.
It never discovers a branch, reads parent history by itself, generates a new
draw, or mutates either history.

The branch entry point should be a narrow composition function, conceptually:

```text
branch_from_window(
  parent: &LaneHistory,
  alternate: &LaneIntentRequest,
  execution: BranchExecutionSelection,
) -> Result<LaneBranch, LaneBranchError>
```

It borrows the parent immutably. `LaneBranch` owns a copied branch record and
metadata sufficient to verify it later; it does not own a mutable reference to
or a mutation capability for the parent.

## True State, Beliefs, Observations, and Reports

The branch point is the parent record's `prior_state_hash` and corresponding
initial `LaneSnapshot`. For the required one-record parent, this is exactly:

```text
parent.initial_state() == parent.records()[0].prior state
parent.initial_state().phase() == LanePhase::Open
parent.records()[0].observation() ==
  observe_player(&parent.initial_state(), parent.records()[0].command().observation_id())
```

The snapshot remains the existing `m2-lane-v1` type: bounded player and
opponent health, position, wave pressure, phase, hidden opponent posture,
hidden jungle threat, and optional one-window outcome. The branch does not add
branch ID, parent outcome, counterfactual status, or branch metadata to the
snapshot. State hashes therefore remain hashes of authoritative lane state
only, using the existing field order and `fnv1a64-le-v1` representation.

The branch reuses the recorded `LanerObservation` exactly. It contains the
same player health/position, visible wave pressure, two available intents,
`Unknown` opponent health/posture, `Unknown` jungle threat, one-beat window,
schema `m2-lane-observation-v1`, and the same `ObservationId`. The branch must
not create a new observation after the parent window or reveal:

- the parent's command or outcome before the alternate request is accepted;
- the true opponent health, posture, current position, or jungle threat;
- the host source-state hash stored in `LaneObservationReceipt`;
- matched or regenerated execution values before transition evaluation.

The host reconstructs a private receipt from the branch-point snapshot and the
recorded observation ID, then compares its actor-visible observation with the
parent record. The receipt's source hash stays host-only. A branch result may
be projected to the actor using the same redaction rules as window 1; a
privileged research/controller inspection may see provenance and true hashes
through a separately authorized surface, but that is not an actor observation
or an ordinary policy input.

No new belief update or report wording is introduced. This branch compares
what could have followed from the same information available at the pivotal
decision; it does not pretend that the actor observed the parent outcome before
choosing the alternate.

## Plans, Commands, and Validation

The two existing `LaneIntent` variants and their plan semantics are unchanged:

- `Stabilize`: commit for the beat, focus on the wave, send `NoMessage`, have
  no contest abort, and fall back to yielding space near the tower.
- `Contest`: commit for the beat, focus on opponent and wave, send `NoMessage`,
  abort when self damage reaches `2`, and fall back to yielding space near the
  tower.

The alternate is still a normal actor-facing `LaneIntentRequest`. It must use
the recorded observation ID and player actor, and its intent must differ from
the parent record's intent. This makes the bounded branch a decision
counterfactual rather than an arbitrary replay edit. The host then creates the
same internal `LaneIntentCommand` shape already used by `validate_lane_request`:
actor, current turn, `m2-lane-v1`, observation ID, host prior-state hash, and
one closed intent variant.

Branch validation occurs before transition evaluation:

1. `parent.verify_replay()` succeeds, so the source history is not trusted
   merely because it has a terminal snapshot;
2. the parent has exactly one record, at index `0`, and its prior state is the
   parent initial state with phase `Open`;
3. the parent record's observation equals a fresh projection from that prior
   state using the recorded observation ID;
4. the alternate actor and observation ID match the recorded player observation;
5. the alternate intent is different from the parent intent and is one of the
   two existing variants;
6. the existing `validate_lane_request` / `validate_lane_command` checks pass;
7. the execution-selection contract below is satisfied.

Branch-specific failures are typed separately from ordinary command and
execution failures:

```text
ParentNotReplayable
ParentNotExactlyOneWindow
InvalidBranchPoint
ObservationMismatch
NotAnAlternateIntent
BranchActorMismatch
BranchObservationMismatch
NonExecutionInputsChanged
InvalidBranchExecutionIdentity
ParentExecutionUnavailable
Validation(LaneValidationError)
Transition(LaneTransitionError)
```

No branch record is created for any failure. In particular, a wrong actor,
stale observation, wrong turn/ruleset, stale host hash, or same-intent request
is not a modeled unfavorable outcome. A valid alternate command paired with
bad but bounded execution remains a legal branch result; malformed damage or
wave inputs remain `LaneTransitionError` values and do not commit.

## Resolved Inputs and Random Streams

`LaneResolvedInputs` remains unchanged. The branch reuses its four existing
non-execution traces exactly: environment, observation, policy, and
coordination. There is no communication stream because the existing slice has
no allied recipient and its plan metadata remains `NoMessage`.

The branch adds only a selection envelope around the existing
`LaneExecutionInputs`:

```text
BranchExecutionSelection::MatchedParent {
  source_record: 0,
}

BranchExecutionSelection::Regenerated {
  branch_id: BranchId,                 // explicit, stable, 0..=127
  execution: LaneExecutionInputs,      // already resolved at the edge
}
```

Matched execution copies the parent record's entire `LaneResolvedInputs`,
including its exact execution values and `InputTrace`. If the parent used
`InputTrace { stream: 5, draw: 0 }`, the branch uses that same identity; it does
not relabel or resample it. This is a controlled comparison of the alternate
intent under the same resolved outcome, not a claim that the physical outcome
would necessarily be identical in an unmodeled world.

Regenerated execution copies the parent's environment, observation, policy,
and coordination traces and replaces only its `LaneExecutionInputs`. The edge
resolver supplies the new bounded damage and wave result before the transition
is called. The transition never creates or samples this value.

For this one-window branch contract, regenerated execution must use the
following stable trace namespace:

```text
branch_execution_trace(branch_id) =
  InputTrace { stream: StreamId(128 + branch_id), draw: DrawId(0) }
```

`BranchId` is explicitly supplied by the host and is restricted to `0..=127`
so the mapping is total for the current `u8` `StreamId`. The branch identity
records the branch ID and resulting trace. Repeating the same branch identity
with the same explicit execution value reproduces the same result. A future
need for more branch IDs requires a versioned contract, not silent reuse.

The branch must reject regenerated inputs whose non-execution traces differ
from the parent or whose execution trace is not the derived branch trace.
Adding unrelated streams cannot alter the existing lane result. The branch
does not implement an RNG, draw allocation service, or stream scheduler.

## Events, Effects, and Transition

The branch invokes the unchanged transition boundary:

```text
transition_lane(
  branch_point_state,
  validated_alternate,
  branch_resolved_inputs,
) -> LaneTransitionResult
```

The existing deterministic steps remain authoritative: validate the exact
snapshot binding and phase, validate damage and wave bounds, subtract health,
apply wave movement, apply the intent/fallback position rule, classify
`HeldSpace`, `YieldedSpace`, or `ForcedOut`, close the window, advance the turn,
preserve hidden truth, and hash the next snapshot.

The branch emits the existing `LaneEvent` and `LaneEffect` values in the same
order. `IntentCommitted` names the alternate intent. Damage and wave effects
retain `LaneEffectCause::Execution(trace)`, so matched branches retain the
parent trace while regenerated branches carry the derived branch trace.
`FallbackActivated`, `WindowResolved`, and all existing provenance rules are
unchanged. No `BranchStarted` event is inserted into the authoritative lane
transition, because branch provenance belongs to the branch envelope rather
than the scenario state.

The branch result's state hash is computed exactly as before and excludes
branch ID, branch mode, parent hash, and comparison labels. A branch can
therefore have the same next-state hash as its parent when the alternate
intent and matched execution happen to produce the same state; the distinct
branch replay identity still distinguishes the artifacts.

The branch envelope records the parent relationship separately:

```text
LaneBranch {
  identity: LaneBranchReplayIdentity,
  execution_selection: BranchExecutionSelection,
  record: LaneTransitionRecord,
}
```

The record's observation, host command, resolved inputs, prior hash, events,
effects, debrief, next snapshot, and next hash are the same typed values used
by `LaneHistory`. No second transition is allowed.

## History, Replay, and Branching

The branch boundary is the immutable prefix before parent record `0`. The
parent must be a verified one-record `LaneHistory`:

```text
parent initial state: Open lane snapshot
parent record 0: original observation + original command + original inputs
parent current state: Resolved lane snapshot
```

`LaneBranchReplayIdentity` is a versioned in-memory value:

```text
LaneBranchReplayIdentity {
  replay_id: "m2-one-lane-window-branch-v1",
  parent_replay_id: "m2-one-lane-window-v1",
  parent_record_index: 0,
  parent_initial_state_hash: StateHash,
  parent_terminal_state_hash: StateHash,
  parent_record_identity: StateHash, // command + observation/input identity
  branch_id: BranchId,
  alternate_intent: LaneIntent,
  execution_mode: MatchedParent | Regenerated,
  execution_trace: InputTrace,
}
```

The parent terminal hash binds the branch to the verified parent artifact;
the parent initial hash binds the exact branch point; and
`parent_record_identity` hashes the parent command, prior hash, all five input
traces, and resolved execution values. Branch identity is history metadata and
is not included in `LaneSnapshot::hash()`.

`LaneBranch::verify_replay(parent)` must:

1. verify the parent history and compare its current hash with
   `parent_terminal_state_hash`;
2. confirm the parent has one record at index `0` and that its prior hash and
   observation identify the declared branch point;
3. regenerate the actor observation from the parent initial state and compare
   it with both the parent and branch records;
4. validate the branch command with the existing lane validation boundary;
5. re-derive the branch inputs according to matched or regenerated mode,
   including exact neutral traces and the stable execution trace;
6. rerun `transition_lane` from the parent initial state; and
7. compare the stored branch record's observation, command, inputs, events,
   effects, debrief, next snapshot, and state hash.

The verifier does not trust a branch terminal snapshot alone. It also rejects
tampered parent hashes, branch mode, branch ID, execution trace, input values,
command, observation, or result. Parent history remains byte-/value-equivalent
before and after branch creation and continues to verify independently.

This is one branch value, not a branch tree. There is no API for deleting,
merging, recursively branching, selecting arbitrary record indices, or
continuing a branch. Persistence and external replay schema are deferred.

## Debrief and Causal Explanation

The parent `LaneDebrief` and branch `LaneDebrief` retain the existing four-way
separation:

- `Decision`: whether the alternate intent was information-consistent with the
  same recorded observation; it is not an optimality or hindsight score.
- `Coordination`: `NotApplicable`, because the window has no allied actor or
  message.
- `Execution`: the actual damage, wave result, fallback activation, and trace
  used by that transition.
- `Luck`: whether the comparison matched the parent execution or used an
  explicitly regenerated execution input.

The branch adds a bounded comparison projection, not a new transition result:

```text
CounterfactualReview {
  parent_outcome: LaneOutcome,
  branch_outcome: LaneOutcome,
  parent_intent: LaneIntent,
  branch_intent: LaneIntent,
  execution_relation: Matched | Regenerated,
  decision_comparison: InformationConsistent,
  coordination: NotApplicable,
  attribution_limit: MatchedDecisionOnly | DecisionAndExecutionChanged,
}
```

For `Matched`, a difference in outcome is attributable only to the changed
intent under this fixed-input comparison. For `Regenerated`, the review must
say that both decision and execution changed; it cannot attribute the outcome
difference to the alternate intent alone. Neither mode estimates luck,
declares a best action, reveals hidden opponent truth to the actor, or claims
that the branch would have occurred in the actual run.

The branch's immediate review uses the pre-decision observation and both
committed one-window results. Its terminal review is the same one-window
diagnostic projection; there is no cross-window or full-match debrief. Any
research-only causal explanation of hidden truth must be clearly separated
from the actor-visible review.

## Verification Contract

The branch implementation must add focused tests while leaving existing
window-1 tests unchanged:

- **Boundary:** build the canonical one-record `LaneHistory` through the
  existing append path; accept only branch point `0` and reject an empty,
  multi-record, invalid, or unreplayable parent.
- **Two strategies:** branch a parent `Contest` into `Stabilize` and a parent
  `Stabilize` into `Contest`, proving both existing intents remain legal from
  the same actor observation.
- **Parent immutability:** snapshot parent records, current state, and terminal
  hash; create and verify a branch; assert all parent values and replay results
  are unchanged.
- **Matched execution:** assert branch non-execution inputs, execution values,
  and execution trace equal parent record `0`; assert the transition uses
  existing event/effect types and the matched trace; verify the branch identity.
- **Regenerated execution:** supply bounded new execution damage/wave values
  with a valid `BranchId`; assert neutral traces match parent, the execution
  trace is exactly `StreamId(128 + branch_id), DrawId(0)`, and the result is
  deterministic across repeated runs.
- **Input isolation:** change a neutral parent trace and reject it as a branch
  input change; separately prove that matching all neutral traces while
  changing only regenerated execution values changes only the expected
  transition result and provenance.
- **Validation:** reject wrong actor, stale observation ID, same-intent request,
  parent hash mismatch, wrong branch point, invalid branch ID, non-derived
  regenerated trace, and changed non-execution inputs before transition.
- **Malformed execution versus failure:** reject damage above available health
  and wave overflow/underflow as transition errors with no branch record;
  accept bounded but unfavorable regenerated execution as a legal result.
- **Hidden-state boundary:** assert that parent and branch use the identical
  actor-visible observation and that branch metadata, parent hashes, execution
  values, and hidden truth are absent from the actor projection.
- **Determinism:** identical branch point, alternate command, execution
  selection, parent history, and ruleset produce identical branch events,
  effects, next state, debrief, and hash; branch identity does not perturb the
  state hash.
- **Replay:** verify matched and regenerated branches from the parent prefix;
  reject tampered parent terminal hash, branch-point hash, branch command,
  observation, mode, trace, input value, result, and terminal state.
- **Debrief limits:** assert matched review reports `MatchedDecisionOnly`,
  regenerated review reports `DecisionAndExecutionChanged`, coordination stays
  `NotApplicable`, and neither review claims optimality or a full-scenario
  result.

These tests establish deterministic software and modeled-causality properties
only. They do not establish that counterfactual branches are understandable,
enjoyable, balanced, accessible, human-like, or scientifically valid.

## Open Questions

- Should the host allocate branch IDs from a per-parent counter or require the
  caller to provide them? This design requires an explicit ID for reproducible
  identity but does not choose an allocator.
- Should a later branch review expose the parent command to the ordinary actor,
  or remain a controller/research projection? This slice keeps it privileged
  and actor-safe.
- Is the reserved `StreamId(128..=255)` namespace sufficient for the eventual
  scenario, or should a future version widen `InputTrace` before persistence?
- Should matched execution reuse the four neutral input values as well as
  their identities in a future branch with communication or allied policy?
  This slice requires exact reuse and has no such actors.
- When the next M2 slice is selected, should it add a second window after this
  branch contract or add richer causal debrief fields first? Neither is
  authorized here.
- Portable branch serialization, migration, branch trees, and compatibility
  policy remain deferred until the in-memory contract is proven stable.
