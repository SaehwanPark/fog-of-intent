# Simulation Design — M2 One-Window Scenario Goal and Terminal Objective

## Goal and Boundary

This slice adds one bounded scenario goal and a host-owned terminal-objective
projection over the implemented one-window lane decision, allied proposal, and
coordination record. The goal is **hold lane space through this diagnostic
beat**. It is a review/evaluation contract, not a new transition or a second
window.

The following remain authoritative and unchanged:

- `LaneSnapshot`, its hash, `m2-lane-v1`, and hidden opponent/jungle truth;
- player and allied actor-valid observations;
- `LaneIntentRequest`, `LaneIntentCommand`, and existing validation;
- `LaneResolvedInputs`, `transition_lane`, `LaneTransitionResult`, and
  `LaneTransitionRecord`;
- `LaneHistory`, `LaneBranch`, their replay identities, and the coordination
  sidecar replay contract.

The new composition is:

```text
committed lane or coordinated record
  -> host-owned ScenarioGoal + ObjectiveEvaluationInputs
  -> deterministic TerminalObjectiveReview
  -> visible objective/debrief projection
```

Objective evaluation happens after the existing result is committed. It cannot
change state, events, effects, execution inputs, coordination disposition, or
the authoritative state hash.

## Scope and Exclusions

Included:

- one versioned goal `HoldLaneSpaceThroughWindow`;
- one objective schema `m2-terminal-objective-v1`;
- one host-owned evaluation from the next snapshot, lane outcome, player
  position/health, wave result, player intent, coordination disposition, and
  explicit execution trace;
- one explicit objective input identity containing the record replay identity,
  prior/terminal hashes, and committed outcome facts;
- typed objective result, criterion statuses, terminal disposition, and causal
  attribution limits;
- one objective review attached to an ordinary or coordinated one-window
  result, with replay and tamper tests.

Excluded are a second window, variable pacing, new mechanics, hidden-state
scoring, a utility/optimality model, win-rate or balance claims, a general
objective framework, objective changes to `LaneSnapshot`, portable
serialization, CLI/MCP/GUI, and human-experience evidence.

## Goal, Inputs, and Authority

```text
ScenarioGoal::HoldLaneSpaceThroughWindow {
    goal_id: "m2-hold-lane-space-v1"
}

ObjectiveEvaluationInputs {
    replay_id: "m2-one-lane-window-v1"
             | "m2-one-lane-coordination-v1",
    prior_state_hash: StateHash,
    terminal_state_hash: StateHash,
    outcome: LaneOutcome,
    player_position: LanePosition,
    player_health: LaneHealth,
    intent: LaneIntent,
    wave_result: LaneWaveResult,
    coordination: LaneCoordinationReview
                  | CoordinationDisposition,
    execution_trace: InputTrace,
}
```

The host derives these values from a committed lane result/record. A caller
cannot supply a different health, position, outcome, or trace independently of
the committed record. For coordinated records, the coordination disposition
is copied from the committed resolution; for ordinary records it is the
existing `NotApplicable` review. The objective evaluator receives no
`LaneSnapshot` opponent truth, jungle truth, proposal scores, source receipts,
or policy internals.

`ObjectiveInputIdentity` binds the versioned objective schema, goal identity,
replay identity, prior hash, terminal hash, and a canonical digest of all
visible evaluation facts. It is provenance, not a replacement for the lane
state hash. The evaluator is synchronous and pure over the typed goal and
inputs; it reads no clock, I/O, RNG, history, or model provider.

## Criterion and Outcome Contract

The goal has two criteria:

```text
SpaceHeld:      next player position == Center
SurvivedBeat:   next player health > zero
```

Their statuses are `Met` or `NotMet`. The closed terminal disposition is:

| SpaceHeld | SurvivedBeat | Disposition |
| --- | --- | --- |
| Met | Met | `GoalAchieved` |
| Met | NotMet | `GoalPartiallyAchieved` |
| NotMet | Met | `GoalMissed` |
| NotMet | NotMet | `GoalMissed` |

This is a diagnostic objective classification, not a universal value
judgment. `ForcedOut` cannot be called success; a `YieldedSpace` result is
classified from the committed position, not from hidden opponent truth. The
objective does not inspect whether an intent was “optimal”.

`TerminalObjectiveReview` stores the goal, objective input identity, criterion
statuses, disposition, player intent, coordination attribution, execution
trace, and a bounded `ObjectiveAttributionLimit`:

```text
ObjectiveAttributionLimit::CommittedFactsOnly
```

The visible projection may report the goal, criterion statuses, disposition,
intent, coordination disposition, and execution trace. It must not expose
hidden state, source-state hashes, proposal policy scores, or private receipts.

## Coordination and Causality

The objective observes coordination but does not rewrite it. A committed
`AcceptedOffer` or `CounterAccepted` remains a coordination fact; an
`AllyDeclined`, `CounterRejected`, or `PlayerRejected` remains distinguishable
from execution. Ordinary lane records retain `NotApplicable`.

The review attributes the criterion result only to committed facts:

- `Decision`: the existing information-consistent player intent review;
- `Coordination`: the stored coordination disposition or not-applicable value;
- `Execution`: the stored wave/health/position result and execution trace;
- `Objective`: the deterministic criterion classification.

No objective result creates an event/effect or feeds back into the lane
transition. The terminal disposition is an evaluation artifact, not a new
`LaneOutcome` and not a persistent state field.

## Replay and Compatibility

`evaluate_terminal_objective(goal, inputs)` must be deterministic for identical
typed inputs. `review_lane_objective(record)` derives inputs directly from an
ordinary `LaneTransitionRecord`; `review_coordinated_objective(record)` derives
them from its base record and coordinated result. The coordinated review keeps
the `m2-one-lane-coordination-v1` identity while the ordinary review keeps
`m2-one-lane-window-v1`.

Replay verifies the objective identity, all canonical input facts, criterion
statuses, disposition, and attribution against the already replay-verified
record. Tampering with terminal hash, outcome, position, health, intent,
coordination disposition, trace, goal identity, or objective result fails.
Existing ordinary history, old branches, and coordinated-history replay remain
valid even when no objective review is requested.

## Verification Contract

Focused tests must cover:

- canonical `Contest`/accepted-support input with Center and positive health;
- `Stabilize`/rejected-support input with NearTower;
- forced-out and yielded-space cases, including partial achievement;
- all criterion/disposition combinations and explicit attribution limits;
- ordinary versus coordinated replay identity and not-applicable coordination;
- hidden-state substitution invariance and absence of source-hash leakage;
- identical input determinism and unrelated trace isolation;
- tampering with every committed objective fact and review result;
- unchanged `LaneSnapshot::hash()` and unchanged base transition result.

Evidence establishes only deterministic objective projection and causal
bookkeeping for one window. It does not establish a complete scenario,
optimality, balance, trust, enjoyment, accessibility, behavioral validity, or
human preference.
