# Simulation Design — M2 Bounded Two-Window Final Debrief

## Goal and Boundary

This slice adds one final debrief projection over a completed
`LaneScenarioHistory` containing exactly two existing ordinary lane records.
It aggregates committed facts; it is not another transition, objective engine,
belief update, or hidden-state research view.

```text
verified two-window history
  -> per-window intent/coordination/execution/objective summaries
  -> final terminal-state summary
  -> bounded visible ScenarioDebriefReport
```

The existing lane transition, window reopen, ordinary/coordinated history,
branch, objective, fixture, and state-hash contracts remain unchanged. The
final debrief cannot mutate history or infer facts that are not committed.

## Scope and Exclusions

Included:

- versioned `m2-two-window-final-debrief-v1` identity;
- a debrief record derived only from a replay-verified two-window history;
- two per-window summaries containing player intent, lane outcome, health/
  position result, wave result, execution trace, and `NotApplicable`
  coordination for this ordinary scenario wrapper;
- per-window objective reviews using the existing goal/evaluation contract;
- final terminal state hash, terminal outcome, goal dispositions, and an
  attribution limit;
- a visible debrief report that omits hidden opponent/jungle truth, source
  receipts, private hashes, policy internals, and uncommitted choices;
- deterministic replay/tamper verification of the debrief record.

Excluded are new mechanics, automatic pacing, recall/gank response,
communication, coordinated records inside the multi-window wrapper, hidden
state inspection, optimality/balance scoring, portable serialization, CLI/MCP/
GUI, and human-experience claims.

## Typed Contract

```text
WindowDebriefSummary {
    window: First | Second,
    intent: LaneIntent,
    outcome: LaneOutcome,
    player_health: LaneHealth,
    player_position: LanePosition,
    wave_result: LaneWaveResult,
    coordination: NotApplicable,
    execution_trace: InputTrace,
    objective: TerminalObjectiveReview,
}

ScenarioDebrief {
    replay_id: "m2-two-window-final-debrief-v1",
    source_replay_id: "m2-two-window-scenario-v1",
    source_terminal_state_hash: StateHash,
    windows: [WindowDebriefSummary; 2],
    final_objective: ObjectiveDisposition,
    attribution_limit: CommittedHistoryFactsOnly,
}
```

The final objective is `GoalAchieved` only if both window objective reviews are
achieved; otherwise it is `GoalMissed` for this bounded report. This is an
explicit aggregation rule, not a new `LaneOutcome` or a global value score.
The visible `ScenarioDebriefReport` includes window intents/outcomes,
coordination-not-applicable, objective dispositions, final disposition, and the
attribution limit, but not source hashes or private receipts.

## Authority, Causality, and Evidence

`build_scenario_debrief(history)` first calls `history.verify_replay()` and
requires exactly two records. It derives every summary from each committed
`LaneScenarioRecord::transition()` and calls `review_lane_objective` for each
record. It never reads opponent truth or changes the stored history.

The debrief distinguishes:

- intent: the committed player strategic choice;
- coordination: `NotApplicable` because this wrapper uses ordinary records;
- execution: committed health/position/wave result and trace;
- objective: existing per-window criterion/disposition;
- final aggregation: a bounded report over those facts.

No summary says an intent was optimal, that hidden state was known, or that a
result generalizes beyond this two-window fixture.

## Replay and Tamper Contract

`ScenarioDebriefRecord` stores the source terminal state hash, source record
identities, summaries, and final report. `verify_replay(history)` reruns the
source history verification, regenerates both objective reviews and summaries,
then compares the complete debrief record. Tampering with source identity,
window order, intent/outcome, execution trace, objective review, terminal hash,
final disposition, or report fails.

The visible report does not expose the privileged debrief identity. Existing
one-window, branch, coordination, objective, fixture, and two-window replay
tests remain passing.

## Verification Contract

Focused tests must cover:

- debrief build only from a complete two-window history;
- per-window attribution and objective preservation;
- final achieved versus missed aggregation;
- visible report hash/receipt redaction;
- deterministic repeated build and unchanged history/current state;
- reject incomplete history, tampered source record, window order, objective,
  terminal hash, final disposition, or report;
- preserve all existing M1/M2 replay and information-boundary tests.

Evidence establishes only a deterministic, committed-facts final debrief for
two ordinary windows. It does not establish a complete scenario, pacing,
strategy quality, balance, optimality, trust, accessibility, or human behavior.
