# Simulation Design — M2 One-Lane Decision Window

## Goal and Roadmap Milestone

This is the next bounded slice for M2 — One-Lane Vertical Slice. It turns the
M1 transition boundary into one information-asymmetric, testable lane decision
window. It is a host/kernel contract, not a playable CLI or a complete lane
scenario.

The fixture goal is: the human laner chooses a one-window posture for a six-
second abstract lane beat, then delegated execution resolves the beat. The
terminal result describes whether the laner held space, yielded space, or was
forced out; it is not a win/loss judgment.

The implementation must use synchronous, deterministic evaluation:

```text
prior LaneSnapshot
  + host-validated LaneIntentCommand
  + LaneResolvedInputs with explicit execution outcome
  + m2-lane-v1 ruleset
  -> LaneEvents + LaneEffects + next LaneSnapshot + StateHash
```

The existing M1 `WorldState`, `Command`, `ResolvedInputs`, hash convention, and
replay properties remain valid. Lane-specific types may be added beside them in
the kernel; M1 types and fixtures must not be generalized or broken to make the
lane slice fit.

## Slice Boundary and Non-Goals

Included is exactly one scenario decision window with:

- one human-controlled laner, one opposing laner, one visible lane wave, and
  one abstract hidden jungle-threat field in one typed authoritative
  `LaneSnapshot`;
- one actor-valid `LanerObservation` projected for the human laner;
- one host-validated `LaneIntentCommand` type, with two intent variants:
  `Stabilize` and `Contest`;
- one explicit `LaneExecutionInputs` value containing the already-resolved
  self damage, opposing damage, wave result, and stable execution trace;
- one deterministic transition that closes the window and emits ordered events,
  attributed effects, a terminal outcome, and a next-state hash;
- one append-only `LaneHistory` record and replay verification tests.

`Stabilize` and `Contest` are two strategies represented by one command schema,
not two command systems. On the same observation, both are legal. A conservative
fixture preserves health by yielding space; a risk-taking fixture contests the
wave and opponent. No strategy is declared globally optimal.

Explicitly excluded from this slice are multiple windows, variable-duration
windows, an allied autonomous actor, communication or proposals, autonomous
opponent policy, a full jungle model, gank response, recall, a champion/item
catalog, mana/cooldowns/gold/experience, a three-lane map, CLI or GUI work,
MCP/external adapters, persistence codecs, general scenario scripting,
counterfactual branch execution, and human-experience claims. These exclusions
keep the lane boundary dependency-complete without creating a second engine.

## Actors and Authority

The only ordinary decision-maker in this slice is `PlayerLaner`. The
`OpponentLaner` is represented in true state but has no policy or command
interface yet. `JungleThreat` is a hidden host-owned fact used only to motivate
uncertain execution outcomes; it is not an actor and cannot issue commands.

The host owns the true `LaneSnapshot`, scenario/ruleset identity, observation
projection, observation receipt, window ordering, command validation, resolved
execution input, transition invocation, history commit, replay verification,
and debrief projection. The transition function is a pure evaluator and does
not own authority, inspect I/O, generate randomness, or fetch observations.

The actor-facing submission is a request, not an authoritative command:

```text
LaneIntentRequest {
  actor: PlayerLaner,
  observation_id: ObservationId,
  intent: LaneIntent,
}
```

The host converts a current, valid request into the one internal command:

```text
LaneIntentCommand {
  actor: PlayerLaner,
  turn: Turn,
  ruleset: RulesetId,
  observation_id: ObservationId,
  host_prior_state_hash: StateHash,
  intent: LaneIntent,
}
```

`host_prior_state_hash` is added by the host and is never required in the
actor-visible request. This preserves the M1 stale-state guard without making a
player know a hash that may encode hidden opponent state. `ValidatedLaneIntent`
is constructible only through host validation and is bound to the exact prior
`LaneSnapshot`.

## True State, Beliefs, Observations, and Reports

`LaneSnapshot` is the one authoritative typed snapshot. Its minimal fields are:

```text
LaneSnapshot {
  ruleset: RulesetId,             // m2-lane-v1
  turn: Turn,                     // 0 in the fixture
  phase: Open | Resolved,
  player: PlayerLaneState {
    id: ActorId,
    health: LaneHealth,           // 0..10
    position: LanePosition,       // NearTower | Center | FarSide
  },
  opponent: OpponentTruth {
    id: ActorId,
    health: LaneHealth,            // hidden from the player
    position: LanePosition,        // current truth; not necessarily visible
    posture: OpponentPosture,      // Aggressive | Passive; hidden
  },
  wave: WaveState {
    pressure: WavePressure,       // 0..3
  },
  jungle_threat: JungleThreatTruth, // Absent | RiverSide | InLane; hidden
  terminal_outcome: Option<LaneOutcome>,
}
```

`LaneHealth` and `WavePressure` are scenario-specific bounded value types;
damage is a separate bounded `LaneDamage` type. `LanePosition`, posture,
threat, phase, and outcome are closed enums. No floating-point or wall-clock
value enters authoritative state. The snapshot hash encodes the ruleset, turn,
phase, all player fields, all opponent truth fields, wave pressure, threat, and
outcome in that declared order using the existing `fnv1a64-le-v1`
representation. Observation IDs and reports are not state fields.

The initial diagnostic snapshot is fixed for tests: player `(health=8,
position=Center)`, opponent `(health=7, position=Center, posture=Aggressive)`,
wave pressure `1`, hidden threat `InLane`, phase `Open`, and no outcome. The
exact hidden values are test fixtures, not player inputs.

`observe_player(snapshot)` returns the only actor-valid observation:

```text
LanerObservation {
  schema: m2-lane-observation-v1,
  observer: PlayerLaner,
  turn: Turn,
  observation_id: ObservationId,
  self: { health: LaneHealth, position: LanePosition },
  wave: { pressure: WavePressure },
  opponent: OpponentReport {
    last_known_position: Option<LanePosition>,
    last_seen_turn: Option<Turn>,
    health: Unknown,
    posture: Unknown,
  },
  jungle_threat: ThreatReport { status: Unknown | LastKnown { ... } },
  available_intents: [Stabilize, Contest],
  window: OneBeat,
}
```

The report wording must label `Unknown` and `LastKnown` explicitly. The
observation contains no opponent health, current posture, current threat,
execution input, true-state hash, research inspection, or private policy data.
The host may retain a private binding from `ObservationId` to the exact prior
snapshot, but the actor receives only the actor-valid projection. Belief state
and player/agent reports are not silently treated as true state; this slice
uses the observation and its explicit unknown/last-known reports as the only
decision input.

## Plans, Commands, and Validation

`LaneIntent` is a closed enum and is the only command payload:

- `Stabilize`: commitment `UntilWindowEnd`; focus `Wave`; communication
  `NoMessage`; abort condition `None`; fallback `YieldSpaceNearTower`.
- `Contest`: commitment `ContestThisBeat`; focus `OpponentAndWave`;
  communication `NoMessage`; abort condition `SelfDamageAtLeast(2)`; fallback
  `YieldSpaceNearTower`.

The plan metadata is a semantic property of the variant, not a second command
or an arbitrary string. There is no allied recipient in this slice, so
`NoMessage` is explicit and coordination is not modeled. The transition keeps
intent/fallback semantics distinct from the mechanical execution outcome.

Host validation checks, in order:

1. the request actor is the player laner;
2. the observation receipt belongs to that actor, the current turn, the exact
   current snapshot, and `m2-lane-observation-v1`;
3. the snapshot phase is `Open` and the request turn is current;
4. the ruleset is `m2-lane-v1` and the host prior hash matches the snapshot;
5. the intent is one of the two closed variants.

Failures are typed as `WrongActor`, `WrongTurn`, `WrongRuleset`,
`StaleObservation`, `StateHashMismatch`, `WindowAlreadyResolved`, or
`UnsupportedIntent`. They produce no transition, events, effects, or history
record. A valid command does not promise a favorable result: a `Contest` with
bad resolved execution remains a legal command.

`LaneExecutionInputs` are validated separately at the transition boundary.
Damage may not exceed the corresponding prior health; wave resolution must be
able to apply without leaving pressure outside `0..3`; and its trace must have
a stable stream and draw identity. These are malformed resolved inputs, not
player-command rejection. The transition returns typed execution errors and
does not commit a record when they fail.

## Resolved Inputs and Random Streams

The host or an edge resolver produces one immutable `LaneResolvedInputs` value
before transition evaluation:

```text
LaneResolvedInputs {
  environment: InputTrace,
  observation: InputTrace,
  policy: InputTrace,
  coordination: InputTrace,
  execution: LaneExecutionInputs {
    trace: InputTrace,
    self_damage: LaneDamage,
    opponent_damage: LaneDamage,
    wave_result: Advanced | Held | Lost,
  },
}
```

The four non-execution traces preserve M1's explicit categories. They are
neutral in this one-actor slice and have no transition effect. The execution
trace is the only consumed stochastic identity; it must be recorded even when
the resolved damage is zero. A canonical fixture may use stream `5`, draw
`0` for the first window, but the transition treats the identity as data and
never derives values from it.

The resolver may use host-only environment facts and bounded policy logic at
the edge to produce the execution outcome. It must not pass hidden state to the
player or let the transition infer a draw. Adding or changing an unrelated
environment, observation, policy, or coordination trace cannot alter the lane
result. If a future branch regenerates execution, it receives a new
branch-scoped identity; matched replay reuses the exact original value and
identity.

## Events, Effects, and Transition

The pure lane transition is:

```text
transition_lane(
  prior: &LaneSnapshot,
  command: &ValidatedLaneIntent,
  inputs: &LaneResolvedInputs,
) -> Result<LaneTransitionResult, LaneTransitionError>
```

It performs these deterministic steps:

1. confirm validation is bound to the exact prior snapshot;
2. validate damage and wave bounds;
3. subtract player and opponent damage from their health;
4. apply `Advanced`, `Held`, or `Lost` to wave pressure by `+1`, `0`, or `-1`;
5. set player position to `NearTower` for `Stabilize`, or `Center` for
   `Contest` unless self damage is at least `2`, which activates the declared
   fallback and sets `NearTower`;
6. classify the terminal outcome as `HeldSpace` (contest remains in Center),
   `YieldedSpace` (fallback or stabilize yields NearTower), or
   `ForcedOut` (player health reaches zero);
7. set phase to `Resolved`, advance the turn once, preserve the hidden threat
   truth, and compute the next-state hash.

Events are ordered and contain causal facts, not replacement state:

```text
IntentCommitted { actor, intent }
PlayerDamaged { target: PlayerLaner, amount, execution_trace }   // only if nonzero
OpponentDamaged { target: OpponentLaner, amount, execution_trace } // only if nonzero
WaveResolved { before, after, execution_trace }
FallbackActivated { actor, intent, reason }             // if threshold met
WindowResolved { outcome }
```

Effects are ordered state changes with explicit provenance:

```text
HealthChanged { actor, before, after, cause: Execution(trace), immediate }
WavePressureChanged { before, after, cause: Execution(trace), immediate }
PositionChanged { before, after, cause: Intent | Fallback, immediate }
```

Opponent effects remain host/research data. An actor-facing result projection
may redact opponent amounts or hidden fields; it must not expose the raw true
snapshot merely because the raw history is replayable. The transition may
return a legal unfavorable result such as `Contest` plus self damage `3`, wave
`Lost`, and `YieldedSpace`. That result is distinct from every command or input
validation error.

## History, Replay, and Branching

Add a narrow `LaneHistory`, not a generic history framework. It contains one
initial `LaneSnapshot`, the current snapshot, and an append-only vector of
`LaneTransitionRecord` values. Each record stores the actor-valid observation,
the host-created `LaneIntentCommand`, prior true-state hash, exact
`LaneResolvedInputs`, complete transition result, ruleset/scenario identity,
and the next-state hash.

Appending validates and evaluates exactly one record before mutating current
history. An invalid command or malformed execution input leaves history
unchanged. There is no delete or in-place edit operation. The replay verifier
starts from the initial snapshot and, for every record:

- checks the recorded prior hash;
- regenerates the player observation and compares it with the recorded
  actor-valid observation;
- revalidates the stored command against the current snapshot;
- reevaluates the exact stored resolved inputs;
- compares ordered events, effects, next snapshot, and state hash.

Replay succeeds only when the reconstructed terminal snapshot equals the
committed current snapshot. The replay identity is
`m2-one-lane-window-v1` plus ruleset `m2-lane-v1`, observation schema
`m2-lane-observation-v1`, and the existing hash representation. The M1 text
codec fixtures remain unchanged; lane serialization is excluded from this
slice.

Branching has no implementation or test here. Its future contract is explicit:
a branch starts from an immutable record boundary and copies the parent prefix;
matched exogenous inputs reuse exact values and trace identities, while
regenerated inputs use new branch-scoped identities. Parent history is never
mutated, and a branch cannot alter the original replay result.

## Debrief and Causal Explanation

The transition's result supplies a small immediate and terminal debrief
projection for this one-window fixture. It is derived from the recorded
pre-decision `LanerObservation`, command, committed events/effects, and
resolved execution result; it does not use hidden truth to judge the decision.

The projection has four separate fields:

- `Decision`: records `Stabilize` or `Contest`, the observed focus, commitment,
  abort rule, and whether the intent was valid for the observed information.
  It says `information-consistent` or `invalid`, not `optimal`.
- `Coordination`: `NotApplicable(NoMessage)` because there is no allied actor
  or communication channel in this slice.
- `Execution`: reports the resolved damage, wave result, fallback activation,
  and execution trace as what happened after commitment. It does not relabel
  a bad outcome as a bad command.
- `Luck`: identifies that the result depended on the explicit execution input
  and its trace. It does not estimate luck, compare an unrun counterfactual, or
  claim that hidden opponent behavior was known at decision time.

The immediate projection is available after the single transition. The same
shape is the terminal projection because this fixture ends after one window;
it must be labeled `one-window diagnostic`, not a complete-match debrief. Raw
host/research inspection can explain hidden causes separately, but no such
privileged view is exposed through the actor observation or ordinary replay
projection.

## Verification Contract

Implementation must name and pass focused tests before this slice is treated as
M2 evidence:

- **Example / end-to-end:** project the initial snapshot, accept both
  `Stabilize` and `Contest` requests from the same observation, resolve one
  transition, and assert the exact next state, ordered events, effects, outcome,
  and hash for conservative and risk-taking fixtures.
- **Legal unfavorable outcome:** accept `Contest` with self damage `3` and a
  lost wave, produce `YieldedSpace` or `ForcedOut` as applicable, and prove
  this is not a validation error.
- **Validation:** reject wrong actor, wrong turn, wrong ruleset, stale
  observation, stale host hash, and already-resolved-window requests before
  transition evaluation; assert no history record is added.
- **Malformed execution:** reject damage above available health and wave
  underflow/overflow without mutating state or history.
- **Invariant:** player and opponent health remain `0..10`, wave pressure
  remains `0..3`, damage conservation is exact, phase changes only
  `Open -> Resolved`, and the result hash equals the next snapshot hash.
- **Hidden-state boundary:** create snapshots with different opponent health,
  posture, and jungle threat but the same visible fields; assert the player
  observations contain no latent values and actor policies can choose only
  from the observation.
- **Determinism and stream isolation:** identical prior snapshot, validated
  command, resolved inputs, and ruleset yield equal events, effects, next state,
  and hash; changing neutral input traces does not change the result.
- **Replay:** append the canonical one-record history and verify it from the
  initial snapshot; reject tampered command, execution value, prior hash,
  observation, result, and terminal hash; verify stable stream/draw identity.
- **Authority boundary:** prove ordinary request construction cannot create a
  `ValidatedLaneIntent` and that raw true-state access is not a prerequisite
  for choosing either intent.

These tests establish software properties only. They do not establish that
the lane abstraction is enjoyable, accessible, human-like, balanced, or
behaviorally valid.

## Open Questions

- Should the first player-facing wording call `Contest` a trade, pressure, or
  probe once the original-setting vocabulary is selected?
- Should the actor-visible opponent report expose a last-known position in the
  initial fixture, or begin as entirely unknown? Either choice must remain
  explicit and must not reveal current latent state.
- What execution resolver supplies damage and wave outcomes at the edge, and
  how should its scenario/profile version be recorded when it becomes real?
- Should player-facing replay reveal the amount of opponent damage, or only a
  redacted visible event, in the original-setting adaptation?
- After this diagnostic window passes, should the next M2 slice add a second
  window, an allied proposal, or a bounded branch first? That choice requires
  evidence from this contract and is not assumed here.
- The lane snapshot hash field order and schema are local M2-v1 contracts;
  external migration and portable lane replay compatibility remain deferred.
