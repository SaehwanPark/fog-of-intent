# Simulation Design — M2 Matched-Input Strategy Fixtures

## Goal and Boundary

This slice adds exactly three named diagnostic fixtures over the existing
one-window lane, allied-coordination, and terminal-objective contracts:

- `HappyPath`: `Contest` with accepted allied support and favorable explicit
  execution, expected to achieve `HoldLaneSpaceThroughWindow`;
- `RiskTaking`: `Contest` with rejected support and unfavorable but legal
  execution, expected to yield space while surviving the beat;
- `Conservative`: `Stabilize` with rejected support and held explicit
  execution, expected to miss the hold-space goal while surviving.

Fixtures are immutable input bundles for tests and diagnostic inspection. They
are not a scenario engine, policy population, balance model, or hidden
benchmark. They do not add a second window or change the existing transition.

## Preserved Authority

`LaneSnapshot`, `LaneResolvedInputs`, `transition_lane`, `LaneHistory`,
`LaneBranch`, `CoordinatedLaneHistory`, and terminal-objective evaluation remain
the existing authorities. A fixture chooses already-resolved inputs at the
edge; it cannot read hidden truth, create randomness, bypass validation, or
mutate a record. The same fixture input must reproduce the same player
observation, proposal/offer, response, coordination disposition, lane result,
objective review, and state hash.

## Typed Fixture Contract

```text
StrategyFixture {
    id: HappyPath | RiskTaking | Conservative,
    player_intent: LaneIntent,
    response: ProposalResponse,
    coordination_inputs: CoordinationResolutionInputs,
    lane_inputs: LaneResolvedInputs,
    expected_objective: ObjectiveDisposition,
    expected_outcome: LaneOutcome,
}
```

The fixture exposes its declared values through getters and is `Copy`/`Eq`.
Its response proposal ID is bound to the generated canonical allied proposal,
not a free-form fixture value. Construction is a pure function of the initial
snapshot and fixed observation/policy traces; a host still validates the
embedded request and resolves the coordinated record through existing APIs.

The three canonical input bundles are:

| Fixture | Intent/response | Coordination input | Lane execution | Expected review |
| --- | --- | --- | --- | --- |
| HappyPath | Contest + Accept | AllyCommitted | self 0, opponent 2, wave Advanced | Achieved / HeldSpace |
| RiskTaking | Contest + Reject | NotRequested | self 3, opponent 0, wave Lost | Missed / YieldedSpace |
| Conservative | Stabilize + Reject | NotRequested | self 0, opponent 0, wave Held | Missed / YieldedSpace |

`RiskTaking` is a legal unfavorable result, not a rejected command. The
fixture descriptions do not claim that one strategy is globally better; they
only name matched inputs and expected modeled outputs.

## Information and Causality

Fixture construction uses the public initial state and host-side receipts only
to bind the current observation/proposal ID. Ordinary actor projections remain
unchanged. No fixture exposes opponent health, posture, jungle threat, source
hashes, proposal policy internals, or private receipts as actor input.

Fixture review keeps decision, coordination, execution, objective, and
attribution separate. The expected objective is checked against the committed
`ObjectiveReviewRecord`, not used to force a transition result.

## Replay and Determinism

`run_strategy_fixture(fixture)` builds the canonical receipts/offer, creates the
declared request, appends one coordinated record, and returns the history plus
objective review. It rejects any fixture whose generated response or expected
outcome does not match the committed result. A repeated run with identical
fixture and initial state is field-equivalent. Existing ordinary history and
branch replay tests remain unchanged.

No fixture introduces a random draw. The environment, observation, policy,
coordination, and execution traces remain explicit in `LaneResolvedInputs` and
`CoordinationResolutionInputs`. Changing a fixture input is a new committed
condition and is evaluated through the same host path.

## Verification Contract

Focused tests cover:

- all three fixture IDs and declared intent/response/input values;
- host validation before append and response proposal-ID binding;
- one successful run per fixture with expected lane outcome and objective
  disposition;
- exact repeated-run equality and unchanged transition/state hash authority;
- distinct strategy inputs and outcomes under the same initial state;
- legal-unfavorable risk-taking behavior remaining distinct from invalidity;
- hidden-state and report-boundary invariants through the existing tests;
- tampering with fixture expectations or committed record data being rejected;
- preservation of ordinary history and bounded branch replay.

Evidence establishes only deterministic software fixture coverage and modeled
strategy contrast for one window. It does not establish strategy quality,
balance, optimality, human preference, enjoyment, accessibility, trust, or
behavioral validity.
