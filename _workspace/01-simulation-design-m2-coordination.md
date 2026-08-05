# Simulation Design — M2 One Allied Proposal at the One-Window Decision

## Goal and Roadmap Milestone

This is the next bounded M2 slice after the implemented window-1 lane contract
and the record-0 counterfactual branch. It adds one allied autonomous proposal
and one host-owned coordination resolution at the existing `Open -> Resolved`
lane decision. It does not add another decision window or a general
communication system.

The existing lane and branch contracts remain the base authority:

- `LaneSnapshot`, its fields, state hash, and `m2-lane-v1` ruleset do not
  change;
- `LaneIntentRequest`, `LaneIntentCommand`, `ValidatedLaneIntent`, and the
  existing command validation remain the player command boundary;
- `LaneResolvedInputs`, `transition_lane`, `LaneTransitionResult`,
  `LaneTransitionRecord`, `LaneHistory`, and the existing lane events/effects
  remain valid for a window with no allied proposal;
- `m2-one-lane-window-v1` and `m2-one-lane-window-branch-v1` replay identities
  remain valid for existing records and branches.

The new path composes those values in a narrow coordination envelope:

```text
open LaneSnapshot
  -> player LanerObservation + allied AlliedLaneObservation
  -> one LaneIntentProposal and host support offer
  -> player CoordinatedLanerObservation
  -> CoordinatedLaneRequest { existing intent request + response }
  -> host validation
  -> explicit CoordinationResolutionInput
  -> explicit existing LaneResolvedInputs for execution
  -> existing transition_lane
  -> CoordinatedTransitionResult and one-record coordinated replay
```

The policy proposal is an actor artifact, not a command or state mutation. The
host presents it as one typed coordination offer. Coordination decides whether
that offer is committed, declined, or countered; execution remains a separate
already-resolved input. The canonical initial profile selects `Contest` and
offers `AssistContest`; the player can contest and accept it, contest and
reject it, or stabilize and counter with a bounded cover request.

## Slice Boundary and Non-Goals

Included is exactly one existing lane window with:

- the current player, opponent, wave, and hidden threat snapshot unchanged;
- one ephemeral allied actor `ALLIED_AUTONOMOUS_ACTOR` whose persistent health,
  position, resources, and policy population are not modeled;
- one fixed, bounded autonomous proposal generated from an actor-valid allied
  observation;
- one actor-visible proposal report attached as a coordination overlay to the
  existing player observation;
- one player response: accept, reject, or one closed counter-proposal;
- one host-owned coordination resolution with an explicit stable input trace;
- one explicit execution input resolved after coordination, passed to the
  existing deterministic lane transition;
- one coordinated result, coordination-specific events/effects/debrief, and
  one append-only in-memory coordinated record that replays the base lane
  transition and coordination envelope.

The current two player strategies remain meaningful:

- `Contest + Accept(AssistSelectedIntent)` risks exposure but can receive allied
  support for the wave/opponent;
- `Stabilize + Reject` yields space deliberately and receives no support;
- `Stabilize + Counter(RequestIntent)` asks for the corresponding lower-risk
  or alternate allied cover plan.

The counter is included to exercise the response contract; it is not a second
allied policy. A host may resolve the counter as accepted or declined through
the explicit coordination input. The policy artifact remains proposal-only;
host presentation and player response are the new coordination integration.

Explicitly excluded are a second window, variable pacing, allied movement or
health state, multiple allied proposals, proposal queues, free-form messages,
trust models, negotiation rounds, autonomous population behavior, opponent
policy, recall, gank-response mechanics, CLI, MCP, GUI, persistence codecs,
portable coordination bundles, branch support for coordinated records, merge
or tree operations, and human-experience or behavioral-validity claims.

## Actors and Authority

The ordinary actors are still `PLAYER_LANER` and the hidden `OPPONENT_LANER`;
`JungleThreatTruth` remains a hidden field. `ALLIED_AUTONOMOUS_ACTOR` is a
single proposal-only role, represented by its actor identity, observation,
versioned policy artifact, and coordination outcome. It has no independently
advancing world state and cannot create a `LaneIntentCommand`, close the
window, resolve execution, or mutate history.

The allied policy is the bounded `scripted-allied-proposal-v1` profile from the
agent-ecology contract. Candidate generation, evaluation, and selection are
separate and transparent. It enumerates only the two advertised existing
intents and uses:

```text
health_risk       = max(5 - laner_health, 0)
stabilize_score   = 2 * health_risk + (3 - wave_pressure)
contest_score     = 2 * wave_pressure + max(laner_health - 5, 0)
```

Equal scores use the fixed conservative `Stabilize` tie-break. At the
canonical visible `(health=8, wave=1)` observation, the policy selects
`Contest` with score `5` over `Stabilize` with score `2`; low visible health
favors `Stabilize`, and higher visible wave pressure favors `Contest`. The
policy reads only `AlliedLaneObservation`, profile identity, and a bound input
identity. It never reads opponent health, posture, current threat, true hashes,
history, or research inspection.

The host owns true state, both actor observations and the player overlay,
proposal validity, response
validity, coordination resolution, execution-input resolution, command
validation, event/effect ordering, history/replay, and debrief projection. The
transition remains a synchronous pure evaluator over the existing lane state,
validated intent, and explicit lane execution input. It does not call the
allied policy, resolve a proposal, or generate a draw.

## True State, Beliefs, Observations, and Reports

`LaneSnapshot` remains exactly the existing typed `m2-lane-v1` snapshot. It
contains no allied actor, proposal, response, coordination disposition, or
branch metadata. The lane state hash therefore remains unchanged. A
coordination record may explain why a particular execution input was supplied,
but coordination metadata never becomes hidden state or a new hash field.

The allied actor receives one new actor-valid projection:

```text
AlliedLaneObservation {
  schema: m2-allied-proposal-observation-v1,
  observer: ALLIED_AUTONOMOUS_ACTOR,
  turn: Turn,
  observation_id: ObservationId,
  laner_health: LaneHealth,
  player_position: LanePosition,
  wave_pressure: WavePressure,
  opponent: OpponentReport,       // health/posture/current position unknown
  jungle_threat: ThreatReport,     // Unknown
  available_intents: [Stabilize, Contest],
  window: OneBeat,
}
```

The policy input identity also records the profile
`scripted-allied-proposal-v1`, actor and ruleset identity, observation schema,
turn, observation ID, canonical digest of the visible fields, and the existing
`LaneResolvedInputs::policy()` trace. It excludes true-state hashes, hidden
fields, execution values, and history.

The player keeps the existing `LanerObservation` byte-for-byte as the base
projection. The host adds a separate coordination overlay rather than adding
proposal fields to the base observation:

```text
CoordinatedLanerObservation {
  lane: LanerObservation,          // existing schema and ObservationId
  allied_proposal: AlliedProposalReport,
}
```

`AlliedProposalReport` exposes the selected intent, proposal ID, proposer,
target, bounded candidate scores/reason codes, profile identity, commitment,
focus, abort condition, and fallback. These are the policy artifact and the
host's typed support offer, not hidden state. It does not expose the ally's
source-state hash, hidden state, execution input, or future execution result.
The player can choose an intent and response from this overlay without
knowing opponent truth.

The proposal ID is deterministically derived from the profile identity, agent
input identity, selected intent, and one proposal ordinal. It changes when a
versioned policy/input identity changes and is not a random value. The full
`LaneIntentProposal` artifact and player-facing report are recorded in the
coordination envelope so replay can prove what the player saw.

No new belief update is inferred from acceptance or rejection. The player may
believe the ally will follow through, but the committed coordination result
records only the host resolution. Research inspection may see the true
coordination input through a separate privileged projection; ordinary actor
observations do not.

## Plans, Commands, and Validation

The existing player plans remain unchanged:

- `Stabilize`: commit for this beat, focus on the wave, send no further
  message, do not contest, and fall back to yielding space near the tower.
- `Contest`: commit for this beat, focus on opponent and wave, send no further
  message, abort on self damage at least `2`, and fall back near the tower.

The autonomous policy produces a closed, proposal-only artifact:

```text
LaneIntentProposal {
  id: ProposalId,
  actor: ALLIED_AUTONOMOUS_ACTOR,
  profile_id: scripted-allied-proposal-v1,
  input_identity: AgentInputIdentity,
  candidates: [
    { intent: Stabilize, score, reason_code },
    { intent: Contest, score, reason_code },
  ],
  selected_intent: Contest | Stabilize,
  selection_rule: max-score-stabilize-tie-v1,
}
```

The host turns the selected policy artifact into one bounded coordination offer
without treating the policy result as acceptance or commitment:

```text
AlliedProposalOffer {
  proposal: LaneIntentProposal,
  target: PLAYER_LANER,
  support: AssistSelectedIntent | CoverSelectedIntent,
  commitment: UntilWindowEnd,
  focus: OpponentAndWave | Wave,
  abort: IfPlayerYields | IfPlayerHealthAtMost(2),
  fallback: HoldPosition,
}
```

`AssistSelectedIntent` is `AssistContest` when the policy selects `Contest`;
`CoverSelectedIntent` is `CoverStabilize` when it selects `Stabilize`. The
canonical initial policy therefore offers `AssistContest`, while low-health
fixtures offer `CoverStabilize`. The offer is the only new coordination
presentation type.

The player response is a closed coordination message, not a second lane
command:

```text
ProposalResponse::Accept { proposal_id }
ProposalResponse::Reject { proposal_id }
ProposalResponse::Counter {
  proposal_id,
  counter: CounterProposal::RequestIntent {
    requested_intent: Stabilize | Contest,
    target: PLAYER_LANER,
    commitment: UntilWindowEnd,
    focus: Wave | OpponentAndWave,
    abort: IfPlayerHealthAtMost(2) | IfPlayerYields,
    fallback: HoldPosition,
  },
}
```

The counter-proposal is the only counter shape. It may request the other
existing intent and its corresponding bounded support plan; it does not create
free-form negotiation. The response is presented as a coordination message in
this slice, while the policy artifact itself remains a proposal-only result.

The new request envelope preserves the existing intent request:

```text
CoordinatedLaneRequest {
  intent: LaneIntentRequest,       // existing actor/observation/intent
  response: ProposalResponse,
}
```

Host validation checks, in order:

1. the allied observation and proposal were generated for the current open
   snapshot and the canonical one-proposal slot;
2. the proposal ID, proposer, target, and plan match the recorded report;
3. the embedded existing `LaneIntentRequest` passes the current lane request
   actor and observation checks;
4. `Accept` is allowed only when the embedded player intent equals the
   proposal's `selected_intent`;
5. `Counter(RequestIntent)` is allowed only when its requested intent equals
   the embedded player intent and differs from the proposal's selected intent;
6. `Reject` is allowed for either player intent;
7. the response proposal ID matches exactly and there is only one response.

Typed failures include `StaleAlliedObservation`, `ProposalNotForWindow`,
`ProposalIdMismatch`, `WrongProposer`, `WrongTarget`,
`ResponseProposalMismatch`, `AcceptIntentMismatch`,
`CounterIntentMismatch`, `UnsupportedCounter`, `DuplicateResponse`, and the
existing `LaneValidationError` values. They fail before coordination or lane
transition and add no history record.

An accepted proposal is not a guarantee of execution success. A valid
`Accept` may resolve to `AllyDeclined`; a valid `Reject` may be followed by a
good execution result. Those are modeled coordination/execution outcomes, not
invalid commands.

## Resolved Inputs and Random Streams

The proposal and coordination boundaries resolve before `transition_lane`:

```text
AlliedProposalInputs {
  profile_id: scripted-allied-proposal-v1,
  policy_trace: LaneResolvedInputs.policy(),
  input_identity: AgentInputIdentity,
  proposal: LaneIntentProposal,
}

CoordinationResolutionInputs {
  coordination_trace: InputTrace,   // stream 7, draw 0 in the fixture
  follow_through: NotRequested | AllyCommitted | AllyDeclined,
}
```

The proposal policy does not sample randomness in this slice. Its profile and
agent-input identity are nevertheless recorded, and the existing policy trace
is part of that identity. `follow_through` is already resolved host input; the
coordination resolver does not create an RNG or infer it from true state.

The pure host-owned resolver is:

```text
resolve_coordination(
  offer: &AlliedProposalOffer,
  request: &CoordinatedLaneRequest,
  inputs: &CoordinationResolutionInputs,
) -> Result<CoordinationResolution, CoordinationError>
```

Its closed mapping is:

| Player response | Required follow-through input | Resolution |
| --- | --- | --- |
| `Reject` | `NotRequested` | `PlayerRejected`, no support |
| `Accept` | `AllyCommitted` | `AcceptedOffer`, with offer support |
| `Accept` | `AllyDeclined` | `AllyDeclined`, no support |
| `Counter(RequestIntent)` | `AllyCommitted` | `CounterAccepted`, with requested-intent support |
| `Counter(RequestIntent)` | `AllyDeclined` | `CounterRejected`, no support |

Any other pair is a typed malformed coordination input. The resolution stores
the proposal ID, response, disposition, support plan or `None`, and
coordination trace.

The edge then resolves the existing `LaneResolvedInputs` execution payload
under that explicit disposition. The four existing lane traces remain
explicit; `LaneResolvedInputs::coordination()` must equal the coordination
trace stored in `CoordinationResolutionInputs`. The final
`LaneExecutionInputs` contains the existing self damage, opponent damage, wave
result, and execution trace. Coordination can influence those resolved values
at the edge—for example, accepted contest support may resolve to opponent
damage `2` and `Advanced`, while a declined support may resolve to self damage
`3` and `Lost`—but the transition never reconstructs that causal relationship.

The coordinated record stores both coordination inputs and final execution
inputs. Replaying with identical prior state, validated intent, proposal,
coordination inputs, execution inputs, and ruleset is deterministic. Changing
the coordination disposition or execution input is a new committed condition,
not an implicit draw shift.

## Events, Effects, and Transition

The existing `transition_lane` remains the sole authority for lane state,
health, wave, position, terminal outcome, lane events, lane effects, and the
authoritative state hash. It is called once:

```text
lane_result = transition_lane(
  prior_snapshot,
  validated_intent,
  resolved_lane_inputs,
)
```

The coordinated host wrapper composes the result with coordination facts; it
does not add coordination metadata to `LaneSnapshot` or call a second lane
transition:

```text
CoordinatedTransitionResult {
  lane: LaneTransitionResult,
  events: Vec<CoordinatedEvent>,
  effects: Vec<CoordinatedEffect>,
  debrief: CoordinatedDebrief,
}
```

The ordered event stream is:

```text
  ProposalOffered { proposal_id, proposer: ALLIED_AUTONOMOUS_ACTOR, target, plan }
ProposalResponded { proposal_id, response }
CoordinationResolved { proposal_id, disposition, trace }
Lane(LaneEvent::IntentCommitted { ... })
Lane(other existing LaneEvent values in existing order)
```

The existing `LaneEvent` values are not renamed or reordered. Coordination
events are envelope-level facts that make proposal and resolution inspectable.

Coordination effects are likewise a small envelope type:

```text
CoordinatedEffect::SupportCommitted {
  proposal_id,
  proposer: ALLIED_AUTONOMOUS_ACTOR,
  target: PLAYER_LANER,
  support: AssistContest | CoverStabilize,
  cause: Coordination(coordination_trace),
}
CoordinatedEffect::SupportUnavailable {
  proposal_id,
  disposition: PlayerRejected | AllyDeclined | CounterRejected,
  cause: Coordination(coordination_trace),
}
CoordinatedEffect::Lane(LaneEffect)
```

`SupportCommitted` is a semantic coordination effect, not a new persistent
ally state. Any health, wave, or position change remains an existing
`LaneEffect` caused by the explicit execution trace. The debrief can say that
execution was resolved under committed support without mislabeling execution
as a direct coordination state mutation.

The next snapshot is exactly `lane_result.next_state()`: phase `Resolved`,
one advanced turn, updated player/opponent health and position, updated wave,
unchanged hidden threat truth, and existing `LaneOutcome`. The state hash is
exactly `lane_result.state_hash()`.

## History, Replay, and Branching

The existing no-proposal `LaneHistory` remains authoritative for old records.
Add only a narrow one-record `CoordinatedLaneHistory` wrapper that owns:

```text
CoordinatedLaneRecord {
  replay_id: "m2-one-lane-coordination-v1",
  player_observation: LanerObservation,
  allied_observation: AlliedLaneObservation,
  allied_proposal: LaneIntentProposal,
  proposal_report: AlliedProposalReport,
  request: CoordinatedLaneRequest,
  coordination_inputs: CoordinationResolutionInputs,
  resolution: CoordinationResolution,
  base_lane_record: LaneTransitionRecord,
  coordinated_result: CoordinatedTransitionResult,
}
```

The base lane record is produced through the existing `LaneHistory::append`
path with the existing `LaneResolvedInputs`; the coordinated wrapper stores
the proposal and resolution sidecar and the composed result. This keeps the
old lane record's prior hash, state hash, events, effects, and replay behavior
unchanged. The coordinated history allows exactly one record and no delete or
edit operation.

`CoordinatedLaneHistory::verify_replay` must:

1. verify the base one-record `LaneHistory` from its initial snapshot;
2. regenerate the allied observation and deterministic proposal from the
   actor-valid allied input boundary;
3. compare the player base observation and proposal overlay with the recorded
   reports;
4. revalidate the embedded existing lane intent and response;
5. rerun `resolve_coordination` with the exact stored resolution input;
6. verify the lane input coordination trace matches the resolution trace;
7. rerun the existing `transition_lane` once; and
8. compare the coordinated events, effects, debrief, base result, next state,
   and state hash.

The new coordinated replay identity is
`m2-one-lane-coordination-v1`, with parent/base replay identity
`m2-one-lane-window-v1`, proposal schema `m2-allied-proposal-observation-v1`,
and policy profile `scripted-allied-proposal-v1`, using the existing hash
representation. Proposal IDs, candidate scores/reasons, policy/input
identities, coordination traces, responses, dispositions, and execution values
are committed inputs; runtime logs are not replay authority.

Compatibility with the existing bounded branch is explicit:

- old `LaneHistory` records with no coordination overlay continue to verify;
- old `LaneBranch` artifacts continue to use
  `m2-one-lane-window-branch-v1`, record-0 boundaries, matched/regenerated
  execution identities, and their existing `parent_record_identity`;
- branch metadata and coordination metadata do not enter the lane state hash;
- `branch_from_window` must not silently discard a coordinated overlay. This
  slice rejects a coordinated history as an input to the old branch API;
  branching from a coordinated record requires a future versioned branch
  identity that includes the proposal/response/resolution envelope.

Thus the allied proposal slice preserves old replay compatibility without
pretending that the old branch identity can prove a coordination-aware branch.
No branch tree, merge, persistence format, or second transition is added here.

## Debrief and Causal Explanation

The existing `LaneDebrief` remains embedded and continues to describe the
player intent, execution facts, fallback, coordination-not-applicable base
view, and execution trace for the lane transition. The new
`CoordinatedDebrief` adds the proposal/coordination attribution:

```text
CoordinatedDebrief {
  lane: LaneDebrief,
  decision: IntentAndResponseInformationConsistent | Invalid,
  proposal: Offered,
  player_response: Accepted | Rejected | Countered,
  coordination: PlayerRejected
               | AcceptedOffer
               | AllyDeclined
               | CounterAccepted
               | CounterRejected,
  execution: ConditionalOnCoordination { execution_trace },
  luck: ExplicitExecutionInput { execution_trace },
}
```

Attribution rules are strict:

- `Decision` assesses whether the player intent and response were valid from
  the recorded `CoordinatedLanerObservation`; it does not inspect hidden
  opponent truth or declare the response optimal.
- `Coordination` reports whether the player rejected, the ally committed, or
  the ally declined/counter-rejected. A valid acceptance followed by
  `AllyDeclined` is a coordination failure, not a bad command.
- `Execution` reports the explicit damage, wave result, fallback, and trace
  resolved after coordination. It may say the result was conditioned on
  committed support, but it does not infer an unrecorded causal bonus.
- `Luck` identifies the committed execution input and coordination
  follow-through identity. It does not estimate luck or compare a branch.

The immediate review is emitted after this one transition. Its terminal review
is still a one-window diagnostic, not a match debrief. There is no delayed
effect or second observation. Player-facing review redacts hidden state and
private policy/input details; privileged research review may inspect them only
through an explicitly separate surface.

## Verification Contract

Focused tests must cover the coordinated path while retaining all existing
window and branch tests:

- **Proposal generation:** from the canonical initial snapshot, generate the
  `m2-allied-proposal-observation-v1` observation and the
  `scripted-allied-proposal-v1` artifact with candidates `Stabilize=2`,
  `Contest=5`, selected `Contest`, and one `AssistContest` offer; changing
  hidden opponent health, posture, or threat while holding visible fields and
  the agent input identity fixed does not change the policy artifact or report.
- **Observation boundary:** assert the base `LanerObservation` is unchanged,
  the player sees only the proposal overlay, and neither actor receives true
  state, source hashes, policy scores, or execution values.
- **Response examples:** accept with the selected `Contest`, reject with
  `Contest`, and counter with `Stabilize`; assert proposal IDs, candidate
  identity, and plan/intent compatibility.
- **Coordination mapping:** prove the five response/follow-through mappings,
  including accepted support, player rejection, ally decline, counter accept,
  and counter rejection; resolution is deterministic and trace-attributed.
- **Validation:** reject stale proposal, wrong proposer/target, mismatched
  proposal ID, accept-plus-stabilize, counter-plus-contest, unsupported
  counter, duplicate response, wrong actor, stale observation, and wrong turn
  before coordination or lane transition; assert no record is committed.
- **Malformed coordination input:** reject `AllyCommitted` for `Reject`,
  `NotRequested` for `Accept` or `Counter`, mismatched coordination trace,
  malformed proposal identity, and execution input outside existing lane
  bounds. Keep these distinct from a valid ally decline or unfavorable result.
- **Execution separation:** with the same valid response, accept two explicit
  execution outcomes and prove coordination resolution is unchanged while
  execution events/effects and state hash follow the supplied input. With
  accepted versus declined support, record that the edge supplied different
  execution values rather than deriving them in the transition.
- **Events/effects:** assert canonical proposal, response, resolution, and
  existing lane event ordering; assert support committed/unavailable effects
  carry the coordination trace and lane effects retain their existing causes.
- **State invariant:** the next state and authoritative hash equal the base
  `transition_lane` result; no proposal or allied actor field enters
  `LaneSnapshot::hash()`.
- **Determinism and input isolation:** identical observations, proposal,
  response, coordination inputs, lane inputs, and ruleset produce identical
  coordinated events, effects, debrief, next state, and hash; unrelated
  traces do not shift existing execution identities.
- **Replay:** append and verify the one-record coordinated history; reject
  tampered proposal, actor observation, response, coordination input,
  disposition, execution input, base record, coordinated result, or terminal
  hash. Verify old `LaneHistory` and old matched/regenerated `LaneBranch`
  fixtures still replay unchanged.
- **Debrief attribution:** distinguish player decision, proposal response,
  coordination follow-through, execution, and explicit luck; assert no
  optimality, full-scenario, or hidden-state claim is emitted.
- **Strategy diversity:** show conservative `Stabilize + Reject/Counter` and
  risk-taking `Contest + Accept/Reject` are both valid and can lead to
  different modeled outcomes under explicit execution inputs.

These tests establish software and modeled coordination properties only. They
do not establish broad agent behavior, human trust, enjoyment, accessibility,
balance, or behavioral validity.

## Open Questions

- Should a future player-facing surface display the proposal overlay as part of
  the base observation schema, or preserve the separate overlay to keep old
  observations and branches byte-compatible? This slice chooses the overlay.
- Should an accepted support disposition constrain the execution resolver with
  a formal effect envelope, or remain an explicit causal context as here until
  a second scenario demonstrates the need? This slice keeps final execution
  values authoritative and explicit.
- Should the allied proposal policy later be owned by the M4 agent-ecology
  contract, or remain scenario content with a profile version? This slice uses
  one fixed policy and makes no population claim.
- When branching from a coordination-aware record is authorized, should it
  match/regenerate coordination inputs separately from execution inputs? A
  future versioned branch identity must decide; the old branch API rejects
  coordinated records here.
- What original-setting vocabulary should replace `AssistContest`,
  `CoverStabilize`, and `AllyDeclined`? The typed semantics are fixed before
  presentation wording.
- Portable proposal, coordination, and replay serialization remain deferred;
  no CLI, MCP, persistence, or general communication framework is authorized
  by this slice.
