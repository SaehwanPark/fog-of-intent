# Simulation Design — M2 Bounded Two-Window Scenario Wrapper

## Goal and Boundary

This slice composes exactly two sequential one-beat lane windows over the
implemented `LaneSnapshot` transition. It adds no new lane mechanics. The
first committed window produces the existing resolved result; one explicit
host-owned reopen boundary prepares a second open window; the second commit
finishes the bounded scenario history.

```text
open snapshot 0
  -> existing observe/validate/transition
  -> resolved result 0
  -> reopen_lane_window(result 0)
  -> open snapshot 1
  -> existing observe/validate/transition
  -> resolved result 1 / scenario terminal state
```

The existing one-window `LaneHistory`, `LaneBranch`,
`CoordinatedLaneHistory`, replay IDs, `LaneSnapshot::hash()`, and
`transition_lane` remain unchanged and independently valid. The new wrapper
owns only the two-window sequence and the deterministic reopen boundary.

## Scope and Exclusions

Included:

- `m2-two-window-scenario-v1` identity;
- a `LaneScenarioHistory` that accepts at most two sequential ordinary lane
  records;
- one `reopen_lane_window` operation from resolved result 0 to open window 1,
  preserving player/opponent/wave/hidden-threat values and the incremented
  turn while clearing only phase/terminal-window status;
- append-only scenario records that retain each window's start state, base
  transition record, and optional reopened state;
- deterministic replay of both transitions and the reopen boundary;
- a final terminal state after window 2, plus objective review of either
  committed record through the existing objective API.

Excluded are variable-duration windows, automatic pacing, recall, gank
response, new resources, communication, multiple allied proposals, scenario
serialization, branching across scenario windows, merge/delete operations,
CLI/MCP/GUI, and human-experience claims.

## Reopen Contract

`reopen_lane_window(result)` accepts only the opaque committed
`LaneTransitionResult`; it verifies the result hash/outcome against its
resolved next state and then requires `phase == Resolved` with
`terminal_outcome.is_some()`. It returns:

```text
LaneSnapshot::new(
    same ruleset,
    same turn,
    Open,
    same player/opponent/wave/jungle values,
    None,
)
```

It does not call `transition_lane`, create randomness, or mutate the prior
result. Its output hash is an explicit scenario-boundary state and is stored
in the scenario record/replay stream. The raw snapshot helper is private; only
the scenario wrapper may accept the open state as the next window's starting
point.

## History and Authority

```text
LaneScenarioRecord {
    window: ScenarioWindow,
    start_state: LaneSnapshot,
    transition: LaneTransitionRecord,
    reopened_state: Option<LaneSnapshot>,
}
```

Window 0 stores `reopened_state = Some(...)`; window 1 stores `None`. The
wrapper rejects a third append, a non-open current state, an invalid initial
state, or a record whose observation/command does not validate against the
current window. It never accepts an actor action against the resolved window.

The scenario host can use existing ordinary lane records in this slice. The
allied coordination and fixture APIs remain available for one-window cases;
scenario-aware coordination is deferred until a versioned composition is
needed.

## Replay and Determinism

`LaneScenarioHistory::verify_replay` starts from the initial open snapshot. For
each record it checks the window index and exact `start_state`, regenerates the
player observation, validates the stored command, reruns `transition_lane`,
compares the complete `LaneTransitionRecord`, and compares the stored reopen
state when present. It then uses the reopened state as the next start. The
terminal scenario state must equal the wrapper's current state.

Changing a prior outcome, start-state phase, reopened state, command,
observation, input trace, or terminal result is replay failure. Identical
states/commands/inputs reproduce the same window results and hashes. The
wrapper does not infer a second transition from runtime logs.

## Verification Contract

Focused tests must cover:

- valid reopen preserving all domain values while changing only window phase;
- reject reopen from open/invalid states;
- append first window, reopen, append second window, and reach terminal state;
- reject third append and actions against the resolved final window;
- preserve first-window outcome and objective facts across the reopen;
- deterministic repeated two-window replay and unchanged base results/hashes;
- tamper detection for window index, start state, reopened state, command,
  observation, input traces, result, and terminal state;
- existing one-window history, coordinated history, branch, fixture, and M1
  replay tests remain passing.

Evidence establishes only a two-window deterministic composition and replay
boundary. It does not establish variable pacing, a complete lane scenario,
strategy quality, balance, optimality, or human behavior.
