# Agent Ecology Design — M2 Allied Proposal Baseline

## Goal and Roadmap Milestone

This design covers the next thin slice of M2 — One-Lane Vertical Slice, after
the implemented `m2-one-lane-window-v1` decision window and its bounded
counterfactual branch. The slice adds exactly one scripted allied autonomous
actor that observes the open window and produces one lane-intent proposal.

The actor is proposal-only. It can generate candidates, evaluate them, and
select one proposal for the host to record or present, but it cannot create a
`LaneIntentCommand`, close the window, resolve execution, or mutate
`LaneHistory`. A proposal accepted by a future host/player boundary must still
be converted into the existing player request and pass ordinary host
validation. This keeps proposal distinct from commitment and keeps the
existing `transition_lane` contract authoritative.

The slice contains one actor, one one-beat observation, one proposal decision,
and one deterministic policy profile. It does not add a second transition,
communication framework, external adapter, LLM, population model, or claim of
a playable scenario.

## Behavioral Question and Evidence Boundary

The behavioral question is: given the same actor-visible lane observation and
the same versioned policy input identity, does one transparent scripted allied
baseline produce the same legal proposal, and does it change only in the
declared direction when visible health or wave pressure changes?

The design separates four concerns:

- Policy chooses a proposal from an actor-valid observation.
- Host validation decides whether a proposal is well-formed and current.
- Execution remains an explicit `LaneResolvedInputs` value consumed by the
  deterministic transition.
- Hidden opponent and jungle truth remain host-owned causes that are not policy
  inputs.

The evidence can establish legality, deterministic policy behavior, stable
profile/input identity, invariance to hidden-state substitutions, and
matched-input regression behavior for this scripted profile. It cannot
establish optimality, balance, coordination quality, trust, enjoyment,
accessibility, human behavior, or the validity of any agent as a human model.

## Agent Families and Baselines

There is one agent family:

`scripted-allied-proposal-v1` is a deterministic, table-defined policy for the
single role `ALLIED_AUTONOMOUS_ACTOR`. The role is an allied proposal actor
with no independently modeled health, position, resources, or execution body
in this one-window slice. Its stable actor identity is therefore an agent
role identity, not a new hidden `LaneSnapshot` entity.

The profile identity is structured and recorded with every decision:

```text
profile_id:             scripted-allied-proposal-v1
ruleset:                m2-lane-v1
observation_schema:     m2-allied-proposal-observation-v1
candidate_rule:         available-intents-v1
evaluation_rule:        risk-wave-score-v1
selection_rule:         max-score-stabilize-tie-v1
```

Changing any rule, threshold, candidate ordering, or observation interpretation
requires a new profile identity. The only control condition is `NoProposal`,
meaning that no policy result is supplied; it is a harness control, not a
second agent family or a claim about an alternative behavior.

No LLM, prompt, provider, model identifier, sampling configuration, heuristic
population, opponent policy, or second allied actor is part of this contract.

## Observation, Memory, and Policy Inputs

The host creates an actor-valid `AlliedLaneObservation` from the open
`LaneSnapshot`. It is a role-specific projection, not a reuse of the
player-only `LanerObservation` receipt:

```text
AlliedLaneObservation {
  schema: m2-allied-proposal-observation-v1,
  observer: ALLIED_AUTONOMOUS_ACTOR,
  turn: Turn,
  observation_id: ObservationId,
  laner_health: LaneHealth,
  laner_position: LanePosition,
  wave_pressure: WavePressure,
  opponent: OpponentReport,       // health/posture/current position unknown
  jungle_threat: ThreatReport,    // Unknown
  available_intents: [Stabilize, Contest],
  window: OneBeat,
}
```

The proposal actor is allowed to see the public lane-laner health/position and
wave pressure for this fixture. It receives no opponent truth, jungle truth,
execution input, host prior-state hash, observation receipt, source-state
hash, parent/branch history, private report, or other actor's uncommitted
decision. Unknown values are explicit and are not converted into guesses by
the host.

The policy is memoryless for this slice. Its complete input is the observation,
the profile identity, and a host-supplied policy input trace used for identity
and replay binding. It does not read a `LaneSnapshot`, inspect a receipt, or
access a random generator. No persistent trust, reputation, communication
history, or learned parameter is introduced.

`AgentInputIdentity` is a canonical, actor-visible identity containing the
profile identity, observer, ruleset, observation schema, turn,
`observation_id`, the canonical digest of all visible observation fields, and
the existing `LaneResolvedInputs.policy()` trace when the host has resolved the
window input bundle. It excludes the true-state hash, hidden fields, execution
values, and host-only history. The digest uses a versioned field order and the
existing `fnv1a64-le-v1` byte convention; it is an agent-input identity, not
the authoritative lane state hash.

The result must carry both `profile_identity` and `input_identity`. A proposal
without either binding is not replayable evidence.

## Candidate Generation, Evaluation, and Selection

Candidate generation is separate from evaluation. The generator enumerates
only the intents advertised by `available_intents`, in the observation's
declared order, and produces at most one candidate for each existing
`LaneIntent`. It may not invent a new intent or inspect execution. Candidate
identity is stable from the profile identity, input identity, intent tag, and
candidate-rule version.

For every generated candidate, the profile records a bounded integer score and
a structured reason code. It does not emit free-form rationale or private
chain-of-thought. The baseline evaluates only two visible features:

```text
health_risk       = max(5 - laner_health, 0)
stabilize_score   = 2 * health_risk + (3 - wave_pressure)
contest_score     = 2 * wave_pressure + max(laner_health - 5, 0)
```

`laner_position` is visible but deliberately unused by this first profile;
the choice is part of the profile contract, not an invitation to infer hidden
position or opponent posture. Scores remain bounded by the observation's
health and wave limits. The score is a transparent policy feature, not a
utility claim about the game.

Selection chooses the highest score among legal candidates. Equal scores use
the fixed conservative tie-break `Stabilize`, then the declared candidate
order. The selected result is a `LaneIntentProposal` containing:

- allied actor identity, profile identity, and input identity;
- selected `LaneIntent` and stable proposal identity;
- the complete legal candidate set and scores;
- the selection-rule identity and structured tie-break/reason code.

The host may attach this artifact to the current decision review. It must not
silently treat selection as acceptance, commitment, communication, or a player
command. If a later host explicitly accepts the proposal, it creates a normal
`LaneIntentRequest` for `PLAYER_LANER`, validates it against the matching
player observation, and keeps the proposal as provenance. That acceptance path
is an integration dependency, not part of this agent slice.

For the current diagnostic observation (`laner_health = 8`, `wave_pressure =
1`), the baseline selects `Contest` with scores `2` for `Stabilize` and `5`
for `Contest`. At low health, `Stabilize` becomes more favored; at higher wave
pressure, `Contest` becomes more favored, subject to the declared tie-break.

## Communication, Trust, and Team Coordination

Communication is excluded. The proposal is a typed policy artifact at the host
boundary, not a message, speech act, broadcast, or delivery protocol. It has no
recipient, urgency, confidence, delay, missingness, overload, clarification,
confirmation, disagreement, or withdrawal semantics.

Trust and reputation are excluded. The agent does not update beliefs about a
caller, observe another actor's proposal, follow a proposal, or model team
coordination. The existing lane debrief remains `NotApplicable` for
coordination unless a separate simulation contract explicitly adds an accepted
proposal relation. No proposal may become disguised direct control of the
transition.

## Randomness and Reproducibility

The profile has no stochastic policy behavior. Candidate generation,
evaluation, tie-breaking, proposal identity, and structured reason codes are
pure functions of the declared profile and actor-visible input. The policy
trace is an explicit host input identity only; it does not cause the policy to
sample and it does not alter transition semantics.

The matched-input contract is:

```text
same profile identity
+ same AlliedLaneObservation
+ same AgentInputIdentity
=> byte/field-equivalent candidates, scores, selection, and proposal identity
```

If the host changes a visible field, observation ID, policy trace, schema, or
profile rule, the input/profile identity must change or validation must fail.
If hidden opponent health, posture, position, or jungle threat changes while
the actor-visible observation and input identity remain equal, the policy
result must remain equal. Execution damage, wave outcome, and execution trace
are resolved after policy selection and are not policy inputs.

An integration harness may hold the exact parent `LaneResolvedInputs`,
including the execution trace and values, while comparing an accepted proposal.
Matched execution supports a decision comparison only; it does not make a
proposal optimal or claim that hidden causes were known.

## Scenarios, Populations, and Metrics

The population is exactly one allied actor in the existing one-window fixture.
Use a small deterministic fixture bundle rather than a simulation population:

- baseline: health `8`, wave `1`, both intents available; expected `Contest`;
- low-health/high-pressure tie: health `2`, wave `3`; expected `Stabilize`;
- high-health/high-pressure: health `8`, wave `3`; expected `Contest`;
- low-health/low-pressure: health `2`, wave `0`; expected `Stabilize`;
- single-candidate controls for each legal intent;
- hidden-state matched pairs with identical allied observations but different
  opponent and jungle truth.

For each fixture, record:

- proposal legality rate and candidate completeness;
- exact repeat reproducibility and proposal-ID stability;
- hidden-state invariance rate;
- selector-rule consistency and tie-break consistency;
- profile/input identity collision or drift count;
- when an explicit acceptance harness is used, decision comparison under
  identical execution values and traces.

Do not report win rate, balance, action diversity, coordination success, or
strategy quality from this one profile. Repeating the same deterministic result
is reproducibility evidence, not behavioral diversity.

## Calibration or Regression Protocol

This is a scripted regression contract, not semantic calibration. No human
labels, model samples, parameter fitting, or LLM comparison is required.

The implementation should keep versioned golden records for the fixture bundle.
Each record includes the complete actor-visible observation, profile identity,
input identity, candidate scores, selected intent, proposal identity, and
structured reason code. Re-run each record at least twice and compare the
complete decision artifact, not only the selected intent.

The hidden-state matched-pair test must construct two valid host snapshots
that differ in latent opponent/jungle fields but project to the same allied
observation. It must assert equal input identity and equal policy artifact.
The matched-resolution test must reuse the exact same resolved execution input
and trace when checking any downstream accepted-proposal comparison. A change
in proposal behavior must be attributed to the visible fixture mutation, not
to regenerated execution.

Any intentional change to the feature formula, candidate order, tie-break,
observation schema, or identity encoding requires a profile/schema version
bump and updated golden records. A result that depends on hidden truth, an
unbound policy input, or an implicit random seed is a failed regression, not a
new profile variant.

## Expected Effects and Failure Signals

Expected effects are deliberately narrow:

- identical matched inputs select identically;
- lower visible laner health does not increase the Contest score and favors
  Stabilize at the declared tie boundary;
- higher visible wave pressure does not decrease the Contest score;
- hidden-state substitutions do not change candidates, scores, selection, or
  proposal identity;
- every selection is one of the observation's legal intents;
- a proposal alone does not create an event, effect, state hash, history
  record, or execution outcome.

Failure signals include hidden-state sensitivity, profile/input IDs omitted or
unstable, candidate generation that uses non-advertised intents, tie-breaking
that depends on hash-map or runtime order, scores outside the bounded rule,
non-repeatable results, execution values read during policy evaluation, a
proposal entering `transition_lane` without host acceptance and validation, or
an outcome difference being attributed to policy while execution inputs differ.

The following are not failure signals by themselves: different outcomes after
the host deliberately changes resolved execution, a proposal being unfavorable,
the baseline preferring a different intent after a visible observation change,
or the absence of proposal diversity in a deterministic single-profile test.

## Verification Contract

The production boundary must satisfy all of the following:

1. The policy accepts only `AlliedLaneObservation`, profile identity, and the
   declared input identity; it has no `LaneSnapshot`, receipt, source hash,
   history, hidden truth, execution input, I/O, clock, or RNG access.
2. The host validates actor, schema, turn, observation identity, available
   intents, and profile/input binding before recording the proposal.
3. Candidate generation, evaluation, and selection are separately inspectable
   and deterministic.
4. The proposal is not an authoritative command. Any acceptance creates the
   existing player request and passes `validate_lane_request` before
   `transition_lane`.
5. Policy identity and actor-visible input identity are preserved in the
   proposal artifact; hidden source-state hashes remain host-only.
6. Matched-input tests cover exact repetition, hidden-state swaps, visible
   health/wave directionality, candidate legality, stable tie-breaking, and
   matched execution provenance.
7. Policy evaluation cannot change `LaneResolvedInputs`, transition ordering,
   state hashing, replay semantics, or authoritative history.

The evidence check is a focused Rust test suite added with the eventual
implementation, plus the repository's existing format, clippy, and test
commands. This document-only design does not claim that those tests or the
agent actor are implemented yet.

## Open Questions

- Should the accepted-proposal presentation be attached to the player's
  existing observation/review, or should the next M2 simulation slice define a
  distinct host projection? This design deliberately does not add a message
  channel.
- Should `ALLIED_AUTONOMOUS_ACTOR` become a stable kernel `ActorId` now, or
  remain an edge-policy role until the actor has modeled state? The recommended
  choice is a stable policy identity without adding hidden world state.
- Should the host bind the proposal to the existing `LaneResolvedInputs.policy`
  trace before or after policy evaluation? Either order is acceptable only if
  the trace is explicit, stable, and included in `AgentInputIdentity`; no
  implicit default is allowed.
- What exact acceptance semantics should distinguish a player agreeing with a
  proposal from merely seeing it? That is a simulation/coordination decision
  for a later bounded slice, not a trust or communication framework here.
