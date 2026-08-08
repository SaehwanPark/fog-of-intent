# Project Specification

**Status:** Active project-state index
**Last reviewed:** 2026-08-08

This file records verified past, the small active slice, and intentionally
deferred future work. It is not the product proposal, roadmap, issue tracker, or
per-commit journal.

Canonical direction and state live in:

- `README.md` — project entry point and current status;
- `ROADMAP.md` — authoritative milestone order and promotion gates;
- `ARCHITECTURE.md` — verified current structure and target boundaries;
- `CHANGELOG.md` — meaningful contributor- and user-visible history;
- `docs/project-proposal.md` — detailed product and research vision;
- `docs/tech-stack-consideration.md` — proposed technology choices.

## Maintenance Rule

Keep `Present` small. Every active item states what is done, not yet done,
verification, and deferrals. Move work to `Past` only after the named evidence
exists. Planned proposal or roadmap text is never implementation evidence.

## Past

### Repository inception — 2026-08-04

- A Rust 2024 binary package named `fog-of-intent` was initialized at version
  `0.1.0`.
- The executable was initialized as a placeholder that printed `Hello, world!`.
- A comprehensive proposal established the turn-based, AI-native team-strategy
  thesis, initial one-lane slice, bounded-rationality direction, deterministic
  authority boundary, replay/debrief goals, and evidence limits.
- A technology analysis recommended a Rust-authoritative core with CLI and MCP
  adapters, artifact-first persistence, optional Python research tooling, and an
  optional later GUI. Those recommendations remain unadopted until implemented
  or recorded as architecture decisions.

### M0 — Governed repository baseline — 2026-08-04

**Status:** Complete

#### Delivered

- Canonical roadmap, specification, architecture, changelog, repo guidance, and
  domain harness were established.
- MIT source licensing, contribution/conduct policy, unofficial project notice,
  original-setting fallback, and distribution boundaries were documented without
  claiming legal clearance.
- Authoritative terminology and ADR-0001 established host-owned simulation
  authority, explicit resolved inputs, and adapter boundaries.
- ADR-0002 kept M1 in one Cargo package; Rust `1.96.0`, `rustfmt`, `clippy`,
  `Cargo.lock`, MIT package metadata, artifact/replay compatibility policy, and
  dependency/security/license policy were recorded.
- `.github/workflows/ci.yml` and `scripts/check_repository.py` established
  clean-checkout format, lint, test, metadata, link, currentness, and
  dependency-free package checks with focused checker tests.

#### Verification

- PR #4 hosted GitHub Actions `verify` passed from a clean Ubuntu checkout.
- The exact Rust `1.96.0` toolchain, locked metadata, formatting, clippy, and
  Rust tests passed.
- Seven focused repository-checker tests passed, including stale milestone,
  M1/M10, collapsed-reference, image/outside-root, and dependency-defer cases.
- The initial authority ADR identifies one host-owned transition authority and
  forbids adapters from owning simulation truth.
- License and fan-project notices state contributor and user boundaries while
  preserving the evidence limit that they are not legal clearance.

#### Deferred / Non-Goals

- No simulation mechanic, playable decision window, CLI command loop, MCP
  server, replay engine, research package, or GUI shipped in M0.
- A future non-empty dependency graph still requires an approved advisory/license
  scanner or a complete machine-readable defer record.
- M0 does not establish intellectual-property clearance, public-release
  readiness, accessibility, enjoyment, or research validity.

### M1 — Bounded deterministic transition fixture — 2026-08-04

**Status:** Complete
**Started:** 2026-08-04
**Selected after:** M0 hosted CI promotion

#### Delivered

- `src/kernel.rs` provides stable identifiers, bounded `Units`, immutable
  `WorldState`, a versioned ruleset identifier, host validation, and the pure
  `Hold`/`Gather` transition boundary.
- Resolved inputs carry distinct environment, observation, policy,
  coordination, and execution categories with stable stream/draw identities.
- Events, command- and execution-attributed effects, authoritative FNV-1a state
  hashes, and append-only in-memory transition records are implemented.
- Replay revalidates and reevaluates every committed transition and compares
  each stored result and hash.
- Nineteen focused Rust tests cover invalid and unfavorable outcomes, bounds,
  conservation, repeated runs, input-stream isolation, ordering, replay,
  versioned fixtures, round trips, and fail-closed codec rejection.
- `src/serialization.rs` and two checked-in `1.0.0` text fixtures serialize and
  deserialize snapshots and histories through the kernel replay contract.

#### Verification

- Repeated runs with identical prior state, validated commands, resolved inputs,
  and ruleset produce equivalent events, effects, next state, and hash.
- Replay reconstructs the terminal state and verifies every committed transition
  hash from the initial state.
- Tests cover malformed/illegal commands, legal unfavorable outcomes, bounds,
  conservation, ordering, and unrelated input-stream isolation as implemented.
- Core dependency inspection confirms no I/O, async runtime, wall clock,
  terminal, database, MCP, model-provider, or hidden RNG dependency.
- The M1 checklist and exit evidence in `ROADMAP.md` are complete, and the
  merged `0.1.3` implementation passes the locked project checks.

#### Deferred

- No lane model, full scenario, interactive CLI, MCP transport, general
  entity-component system, arbitrary scenario scripting, migration support, or
  richer external replay bundle is implemented.
- The codec remains a local fixture contract; it does not claim external
  compatibility, human experience, or a playable simulation.

## Present

### M2 — First bounded one-lane decision window

**Status:** Active
**Started:** 2026-08-04
**Selected after:** M1 replay and codec promotion

#### Target slice

- Define the smallest typed lane snapshot needed for one decision window.
- Project actor-valid observation text/data for the human laner without latent
  opponent state or research-only inspection.
- Accept one host-validated intent command and explicit resolved execution
  input, then return deterministic events, effects, next state, and hash through
  the existing kernel boundary.
- Preserve append-only history and replay identity while leaving the binary,
  CLI, full scenario, and external adapters deferred.

#### Current M2 v3 contract — 2026-08-06

- The authoritative lane snapshot stores `LaneStatus::{Open,
  Resolved(LaneOutcome)}` rather than correlated phase/outcome fields.
- The authoritative lane state retains only health, mana, gold, experience,
  cooldown, wave, position, threat-backed state, and delayed effects;
  `LaneResources` groups the player mana/gold/experience/cooldown fields, and
  `LaneResourceInputs` is the execution aggregate for their deltas.
- `LaneDelay` rejects zero-beat delayed effects, cooldown ticking saturates for
  every `u32`, and histories must begin from `LaneStatus::Open`.
- Ruleset `4`, the v3 player/allied observation schemas, v3 replay/profile/
  strategy/scenario/debrief/branch identities, and the v3 base-record replay
  identity are current internal identifiers. M2 v1/v2 have no release, tag,
  external codec, or supported artifact; old M2 inputs fail closed and have no
  migration.
- The fixed `LaneActorRoster` identifies the human laner, opposing laner, allied
  autonomous actor, and abstract opposing jungle threat. Player and allied
  observations expose those role identities while retaining their existing
  hidden-state redaction; the roster is scenario metadata and is not hashed into
  mutable lane state.
- The minimum diagnostic state is bounded `LanePosition`, `LaneHealth`,
  `WavePressure`, and the `LaneResources` aggregate containing mana, cooldown,
  gold, and experience. These values remain host-owned and are represented in
  the snapshot, state hash, and replay; projections expose only authorized
  player fields and bounded reports, while explicit execution inputs carry
  resolved damage, wave, and resource changes. Position follows authoritative
  intent/fallback evaluation; health follows validated damage/delayed-effect
  resolution; terminal outcome is evaluated from the resulting values, and
  hidden opponent values remain redacted.
- The bounded intent contract carries `LaneIntent`, `LaneCommitment`,
  `LaneTargetFocus`, `LanePingSignal`, `LaneAbortCondition`, and
  `LaneFallbackBehavior`. Observations advertise legal alternatives and host
  validation/replay bind them to the current actor-visible receipt. The ping
  field is a communication signal, not a free-form message system.
- The v3 transition records explicit effect relation/timing labels and retains
  each delayed effect's originating execution trace through queue ticking, state
  hashing, branch/history identity, replay, events, effects, lane debriefs, and
  final debrief reports. `LaneOutcome` and objective review remain distinct
  from binary win/loss scoring.
  Observation/replay tests cover hidden-state redaction, report completeness,
  receipt privacy, and a complete two-window debrief path.
- `LaneAdvanceCondition` and `LaneAdvanceDecision` define deterministic
  commit-required and no-legal-intent outcomes. Current one- and two-beat
  windows use the commit-required condition; host integration for a genuine
  no-choice automatic path remains deferred.
- `LaneBelief<T>` is a report-derived value with `Unknown`, `Observed`, and
  `LastKnown` states. Unknown reports retain prior belief under the explicit
  no-decay rule; only actor-authorized opponent-position and threat-region
  reports can update it. No belief becomes authoritative lane state.
- M1 ruleset, codec, fixtures, hashes, and test behavior remain unchanged. The
  complete M2 exit criteria below remain unchecked.

#### Delivered in the bounded actor-roster follow-up

- Added `LaneActorRole`, `LaneActorRoster`, and the stable
  `OPPOSING_JUNGLE_THREAT_ACTOR` identity for the four actors named by the M2
  scope.
- Added roster accessors to `LanerObservation` and `AlliedLaneObservation`
  without exposing latent opponent or jungle values.
- Added focused roster/completeness/redaction/hash-boundary tests; the locked
  Rust suite now passes 91 tests.

This establishes actor-role identity and observation completeness only. It does
not establish a complete vision/belief model, communication, pacing, balance,
playability, or human-experience evidence.

#### Reconciled in the bounded minimum-abstraction follow-up

- Promoted the M2 minimum lane/wave/position/health/resource checklist item from
  existing implementation evidence; no runtime code or package version changed.
- Kept the current contract bounded to the diagnostic window and explicitly
  deferred a complete economy, balance, and playable scenario.

#### Reconciled in the bounded intent-contract follow-up

- Promoted the M2 bounded intent/communication definition from existing v2
  request, observation, validation, record-identity, and replay evidence.
- Kept free-form messaging, delivery, trust, negotiation, and a complete
  communication system explicitly deferred.

#### Reconciled in the bounded causal-information follow-up

- Promoted non-binary terminal outcome, hidden-state/report coverage, and
  complete-replay inspection from existing v3 tests and source/fixture
  inspection; delayed-origin provenance is now complete for the bounded queue.
- Kept vision/belief updates, automatic host scheduling, communication
  transport, balance, and playability explicitly deferred; the bounded
  advance-condition contract is defined separately.

#### Defined in the bounded automatic-advance contract follow-up

- Added explicit `LaneAdvanceCondition` and `LaneAdvanceDecision` values for
  commit-required and no-legal-intent evaluation using only declared inputs.
- Kept the current one- and two-beat transition, state hash, observation, and
  replay contracts unchanged; automatic execution and scheduling remain open.

#### Defined in the bounded belief/report contract follow-up

- Added report-derived `LaneBelief<T>` updates for unknown, observed, and
  last-known information with malformed value/turn pairs failing closed.
- Kept hidden health/posture, exact threat truth, state hashes, observation
  schemas, replay identities, and authoritative state outside the belief helper.

#### Delivered in the bounded delayed-origin-trace follow-up

- Added delayed-effect origin traces, preserved them through queue ticking and
  hashing, and attributed resolution events/effects to the originating trace.
- Advanced current internal M2 identities from v2 to v3 with fail-closed
  compatibility; M1 fixtures and behavior remain unchanged.

#### Historical M2 v1 slices (retired; preserved as changelog evidence)

- `src/lane/` defines bounded lane health, damage, wave pressure, positions,
  phase, opponent truth, hidden threat truth, terminal outcome, and the fixed
  `m2-lane-v1` ruleset.
- `observe_player` returns an actor-valid `m2-lane-observation-v1` projection
  with explicit unknown opponent/threat reports and no latent state or hash.
- `Stabilize` and `Contest` requests become host-created validated commands;
  command metadata, observation receipts, and exact prior-state binding reject
  wrong actor, turn, ruleset, stale observation, and stale hash inputs.
- `transition_lane` consumes explicit execution damage and wave results,
  preserves intent/execution separation, emits ordered attributed events and
  effects, produces a one-window debrief, and computes the next-state hash.
- `LaneHistory` commits one append-only record and verifies its observation,
  command, resolved inputs, result, and terminal snapshot through replay.
- Nine focused lane tests plus the nineteen M1 tests pass, covering the
  information boundary, both legal intents, an unfavorable fallback outcome,
  malformed execution, validation, determinism, stream isolation, and replay.

#### Delivered in the bounded branch follow-up

- `branch_from_window` accepts only a verified one-record parent and the
  immutable record-0 decision boundary; the parent remains independently
  replayable and unchanged.
- `BranchExecutionSelection` distinguishes exact parent-input reuse from an
  explicitly resolved branch execution with a stable branch-scoped trace.
- `LaneBranchReplayIdentity` and `CounterfactualReview` preserve parent and
  branch hashes, intent, execution relation, and attribution limits outside the
  authoritative lane-state hash.
- Branch replay re-derives the observation, command, input selection,
  transition result, and terminal hash; tampered branch metadata/results are
  rejected.
- Thirteen focused M2 lane tests plus the nineteen M1 tests pass, including
  matched/regenerated branches, parent immutability, invalid selection,
  branch identity, replay tamper, and causal-review limits.

#### Delivered in the allied proposal and coordination follow-up

- `observe_allied` projects one proposal-only allied actor from visible lane
  fields and explicit unknown reports; hidden truth, source hashes, history,
  and execution values remain outside the actor input.
- `scripted-allied-proposal-v1` records profile/input identities, bounded
  candidate scores, stable selection, and deterministic proposal identity;
  matched hidden-state substitutions produce the same artifact.
- `AlliedProposalOffer`, `ProposalResponse`, and
  `CoordinationResolutionInputs` define one host-owned support offer and the
  accept/reject/counter boundary without turning policy output into a command.
- `resolve_coordinated_lane` emits coordination envelope events/effects and a
  causal debrief around one unchanged `transition_lane` result; proposal and
  coordination metadata do not enter `LaneSnapshot::hash()`.
- `CoordinatedLaneHistory` appends and replays one sidecar record with tamper
  detection while existing no-proposal history and branch replay remain
  valid.
- Five focused coordination/policy/history tests plus the prior thirty-two
  tests pass, for thirty-seven Rust tests total.

#### Delivered in the scenario-goal and terminal-objective follow-up

- `ScenarioGoal::HoldLaneSpaceThroughWindow` defines one bounded diagnostic
  goal without adding a new lane mechanic, state field, event, or transition.
- `ObjectiveEvaluationInputs` and `ObjectiveInputIdentity` derive committed
  result facts, source replay identity, and provenance for ordinary and
  coordinated records while keeping hidden truth outside evaluation.
- `TerminalObjectiveReview` evaluates `SpaceHeld` and `SurvivedBeat` into
  achieved, partial, or missed dispositions and preserves coordination and
  execution attribution without claiming optimality.
- `ObjectiveReport` is a visible projection that omits source-state hashes and
  private receipts; `ObjectiveReviewRecord` verifies source identity, facts,
  review, and tampering for both record types.
- Three focused objective tests plus the prior thirty-seven tests pass, for
  forty Rust tests total.

#### Delivered in the matched-input strategy-fixture follow-up

- `StrategyFixture` defines named `HappyPath`, `RiskTaking`, and
  `Conservative` bundles with explicit intent/response, coordination input,
  lane execution input, and expected modeled outcome.
- `run_strategy_fixture` binds each response to the canonical proposal ID and
  runs through host validation, coordinated history append, and terminal
  objective review; expectations do not alter transition authority.
- Repeated runs are equivalent, the three cases produce distinct declared
  input/output contrasts, legal-unfavorable risk-taking remains valid, and
  tampered expected outcomes are rejected.
- One focused strategy-fixture test plus the prior forty tests pass, for
  forty-one Rust tests total.

#### Delivered in the bounded two-window scenario follow-up

- `LaneScenarioHistory` composes two sequential ordinary lane records under
  `m2-two-window-scenario-v1` while preserving the one-window transition,
  branch, coordination, and objective identities.
- `reopen_lane_window` accepts only a valid resolved result, preserves its
  domain values and advanced turn, and deterministically clears only the
  per-window phase/outcome status for the next open window.
- Scenario replay stores and compares exact window start states, complete base
  records, reopened state, terminal state, and tamper-sensitive sequencing;
  third append and invalid reopen cases fail.
- Two focused scenario tests plus the prior forty-one tests pass, for
  forty-three Rust tests total.

#### Delivered in the bounded final-debrief follow-up

- `build_scenario_debrief` requires a replay-verified two-window history and
  derives two per-window intent/coordination/execution/objective summaries plus
  a final committed-facts objective disposition.
- `ScenarioDebriefRecord` preserves source replay/record identities and
  terminal hash for privileged verification without changing scenario state,
  lane outcomes, events, or effects.
- `ScenarioDebriefReport` uses a separate redacted window-summary type and
  omits source hashes, receipts, full objective identities, policy internals,
  and uncommitted choices.
- One focused final-debrief test plus the prior forty-three tests pass, for
  forty-four Rust tests total.

#### Delivered in the bounded Recall-intent follow-up

- `LanerObservation` advertises `Stabilize`, `Contest`, and `Recall` to the
  player while the allied proposal policy remains limited to its existing
  two-intent candidate set.
- Host validation rejects a Recall request when the current actor-visible
  observation omits it, while current valid, stale, and resolved-window
  behavior remains explicit.
- `transition_lane` resolves Recall to `NearTower` with intent-attributed
  position effects, preserves explicit wave/execution inputs, and retains the
  ordinary `YieldedSpace` or `ForcedOut` outcome boundary.
- Existing record identities, replay, branch, objective, and final-debrief
  contracts remain unchanged; four focused Recall tests plus the prior
  forty-four tests pass, for forty-eight Rust tests total.

#### Delivered in the bounded last-known threat-report follow-up

- `ThreatReport` now distinguishes `Unknown` from the bounded
  `LastKnown { region: RiverSide, last_seen_turn }` player-facing report.
- `observe_player` reports only RiverSide as last-known; Absent and hidden
  current InLane threat truth remain Unknown, and source hashes, exact entities,
  execution values, and current-location claims remain outside the projection.
- A RiverSide history record regenerates and replays the same observation while
  preserving the existing transition, lane-state hash, intent set, and replay
  identities.
- Two focused last-known-report tests plus the prior forty-eight tests pass, for
  fifty Rust tests total.

#### Delivered in the bounded gank-response follow-up

- `Withdraw` is a conditional player command intent advertised only through a
  current RiverSide `LastKnown` threat report; the strategic intent set and
  allied two-candidate policy remain unchanged.
- Host validation rejects Withdraw for Unknown, stale, resolved-window,
  wrong-actor, malformed, and unsupported requests before transition evaluation.
- `transition_lane` resolves Withdraw to NearTower for one beat while keeping
  explicit wave/damage/trace inputs authoritative, marking movement as
  intent-attributed, and leaving Contest fallback inactive.
- Withdraw history and terminal-objective review replay with committed intent
  attribution; three focused gank-response tests plus the prior fifty tests
  pass, for fifty-three Rust tests total.

#### Delivered in the bounded variable-duration-window follow-up

- `LaneWindow::TwoBeats` is explicit snapshot state; `LaneSnapshot::new`
  remains the one-beat compatibility constructor and `new_with_window` creates
  the bounded longer window.
- Player and allied observations carry the selected duration, while allied
  candidate selection remains exactly Stabilize/Contest and the TwoBeats
  duration is bound into its visible input digest.
- `transition_lane` advances a TwoBeats state by exactly two turns and closes
  it at the existing resolved commit boundary; one-beat hash bytes remain
  unchanged and TwoBeats hashes are distinct.
- One focused variable-duration test plus the prior fifty-three tests pass, for
  fifty-four Rust tests total.

#### Delivered in the bounded effect-provenance follow-up

- `LaneEffectProvenance` labels each existing effect as `Direct` or `Indirect`
  and `Immediate` or `Delayed`; current emitted effects are all immediate.
- Explicit health, wave, and intent-position effects are direct/immediate,
  while Contest fallback position movement is indirect/immediate; existing
  `LaneEffectCause` and execution trace attribution remain unchanged.
- Delayed timing is vocabulary only in this slice: no delayed queue, future
  event, new state field, or delayed effect is emitted or stored.
- One focused provenance test plus the prior fifty-four tests pass, for
  fifty-five Rust tests total; replay, state hashes, and existing identities
  remain unchanged.

#### Delivered in the bounded mana-resource follow-up

- `LaneMana` is a bounded player-laner resource with a full-resource default;
  player and allied observations expose only the authorized player-laner value.
- Contest execution may spend an explicit resolved mana amount; Stabilize,
  Recall, and Withdraw spending, plus spending above availability, fail before
  transition mutation.
- `ManaSpent` and `ManaChanged` preserve the execution trace and
  direct/immediate provenance, `LaneDebrief` records the spend, and non-full
  mana binds state hashes and the allied visible digest.
- Lane record identities include mana spent; matched-parent branches clear a
  Contest-only spend for non-Contest alternates and record the normalization in
  branch identity/review attribution.
- Four focused mana/branch tests plus the prior fifty-five tests pass, for
  fifty-nine Rust tests total; no-spend defaults retain the prior hash
  representation and history replay verifies spent-resource results.

#### Delivered in the bounded opponent last-known-report follow-up

- A fixed player projection maps hidden opponent `FarSide` to
  `LastKnown { position: FarSide, last_seen_turn }`; Center and NearTower stay
  Unknown.
- Player health/posture remain hidden, allied opponent reports remain Unknown,
  and no state/hash/transition/event/effect/command contract changes.
- One focused FarSide report/replay test plus the prior fifty-nine tests pass,
  for sixty Rust tests total; different hidden health/posture values at the
  same visible FarSide position produce the same player observation.

#### Delivered in the bounded Yield-intent follow-up

- `LaneIntent::Yield` is advertised in `LanerObservation` alongside `Stabilize`, `Contest`, and `Recall`.
- `validate_lane_request` validates host-created `Yield` commands.
- `transition_lane` resolves `Yield` to `NearTower` with zero damage and zero mana spent, producing outcome `YieldedSpace` and emitting intent-attributed position effects.
- Spending mana with `Yield` returns `ManaSpentWithoutContest` execution error.
- `Yield` records replay and verify through `LaneHistory` and `TerminalObjectiveReview`.
- Three focused `Yield` tests plus the prior sixty-two tests pass, for sixty-three Rust tests total.

#### Delivered in the bounded gold-resource follow-up

- `LaneGold` is a bounded player resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_GOLD_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player gold without exposing opponent gold.
- `LaneExecutionInputs` supports gold-earning resolution during execution with direct-immediate `GoldEarned`/`GoldChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Gold overflow exceeding maximum bounds (`MAX_LANE_GOLD`) fails before transition state mutation with `LaneExecutionError::GoldOverflow`.
- Three focused gold-resource tests plus the prior sixty-three tests pass, for sixty-six Rust tests total.

#### Delivered in the bounded experience-resource follow-up

- `LaneExperience` is a bounded player resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_EXPERIENCE_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player experience without exposing opponent experience.
- `LaneExecutionInputs` supports experience-gained resolution during execution with direct-immediate `ExperienceGained`/`ExperienceChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Experience overflow exceeding maximum bounds (`MAX_LANE_EXPERIENCE`) fails before transition state mutation with `LaneExecutionError::ExperienceOverflow`.
- Three focused experience-resource tests plus the prior sixty-six tests pass, for sixty-nine Rust tests total.

#### Delivered in the bounded cooldown-resource follow-up

- `LaneCooldown` is a bounded player resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_COOLDOWN_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player cooldown without exposing opponent cooldown.
- `LaneExecutionInputs` supports explicit `cooldown_set` resolution and automatic turn/window beat ticking during execution with direct-immediate `CooldownSet`/`CooldownTicked`/`CooldownChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Cooldown overflow exceeding maximum bounds (`MAX_LANE_COOLDOWN`) fails before transition state mutation with `LaneExecutionError::CooldownOverflow`.
- Three focused cooldown-resource tests plus the prior sixty-nine tests pass, for seventy-two Rust tests total.

#### Delivered in the bounded bounty-resource follow-up

- `LaneBounty` is a bounded player resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_BOUNTY_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player bounty without exposing opponent bounty.
- `LaneExecutionInputs` supports explicit `bounty_earned` resolution during execution with direct-immediate `BountyEarned`/`BountyChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Bounty overflow exceeding maximum bounds (`MAX_LANE_BOUNTY`) fails before transition state mutation with `LaneExecutionError::BountyOverflow`.
- Three focused bounty-resource tests plus the prior seventy-two tests pass, for seventy-five Rust tests total.

#### Delivered in the bounded level-resource follow-up

- `LaneLevel` is a bounded player resource with an initial default of 1 and non-initial state-hash and allied visible-digest tags (`LANE_LEVEL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player level (`self_level`, `laner_level`) without exposing opponent level.
- `LaneExecutionInputs` supports explicit `level_gained` resolution during execution with direct-immediate `LevelGained`/`LevelChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Level overflow exceeding maximum bounds (`MAX_LANE_LEVEL`) fails before transition state mutation with `LaneExecutionError::LevelOverflow`.
- Three focused level-resource tests plus the prior seventy-five tests pass, for seventy-eight Rust tests total.

#### Delivered in the bounded minion-kills-resource follow-up

- `LaneMinionKills` is a bounded player resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_MINION_KILLS_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player minion kills (`self_minion_kills`, `laner_minion_kills`) without exposing opponent minion kills.
- `LaneExecutionInputs` supports explicit `minion_kills_gained` resolution during execution with direct-immediate `MinionKillsGained`/`MinionKillsChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Minion kills overflow exceeding maximum bounds (`MAX_LANE_MINION_KILLS`) fails before transition state mutation with `LaneExecutionError::MinionKillsOverflow`.
- Three focused minion-kills-resource tests plus the prior seventy-eight tests pass, for eighty-one Rust tests total.

#### Delivered in the bounded target-focus follow-up

- `LaneTargetFocus` is a bounded player intent focus abstraction with default `Minions` and non-default record-identity hash binding (`LANE_TARGET_FOCUS_HASH_TAG`).
- `LanerObservation` advertises available target focus options (`Minions`, `OpposingLaner`, `Tower`).
- Transition evaluation emits direct-immediate `TargetFocusSelected` and `TargetFocusSet` events and effects, records target focus in `LaneDebrief`, and verifies replay through `LaneHistory`.
- Three focused target-focus tests plus the prior eighty-four tests pass, for eighty-seven Rust tests total.

#### Delivered in the bounded commitment follow-up

- `LaneCommitment` is a bounded player intent commitment abstraction with default `Standard` and non-default record-identity hash binding (`LANE_COMMITMENT_HASH_TAG`).
- `LanerObservation` advertises available commitment options (`Standard`, `Cautious`, `Aggressive`).
- Transition evaluation emits direct-immediate `CommitmentSelected` and `CommitmentSet` events and effects, records commitment in `LaneDebrief`, and verifies replay through `LaneHistory`.
- Three focused commitment tests plus the prior eighty-seven tests pass, for ninety Rust tests total.

#### Delivered in the bounded delayed-effect follow-up

- `LaneDelayedEffects` is a bounded player delayed-effect queue abstraction (maximum 4 items) with `LANE_DELAYED_EFFECT_HASH_TAG` state-hash binding.
- `LaneExecutionInputs` supports `delayed_effect` resolution; queued effects tick on each transition beat and resolve when their delay expires.
- Resolving delayed effects (health regen, mana regen, cooldown reduction) emits `DelayedEffectResolved` events/effects with `direct_delayed` provenance, records queuing and resolution counts in `LaneDebrief`, and verifies replay through `LaneScenarioHistory`.
- Overflowing the delayed-effect queue fails before state mutation with `LaneExecutionError::DelayedEffectOverflow`.
- Three focused delayed-effect tests plus the prior ninety tests pass, for ninety-three Rust tests total.

#### Delivered in the bounded shield-resource follow-up

- `LaneShield` is a bounded player defensive shield resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_SHIELD_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player shield (`self_shield`, `laner_shield`) without exposing opponent shield.
- `LaneExecutionInputs` supports `shield_gained` resolution during execution with direct-immediate `ShieldGained`/`ShieldChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Shield overflow exceeding maximum bounds (`MAX_LANE_SHIELD`) fails before transition state mutation with `LaneExecutionError::ShieldOverflow`.
- Three focused shield-resource tests plus the prior ninety-three tests pass, for ninety-six Rust tests total.

#### Delivered in the bounded ward-resource follow-up

- `LaneWard` is a bounded player vision ward resource with a zero default and non-zero state-hash and allied visible-digest tags (`LANE_WARD_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player ward count (`self_ward`, `laner_ward`) without exposing opponent ward count.
- `LaneExecutionInputs` supports `ward_gained` resolution during execution with direct-immediate `WardGained`/`WardChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Ward overflow exceeding maximum bounds (`MAX_LANE_WARD`) fails before transition state mutation with `LaneExecutionError::WardOverflow`.
- Three focused ward-resource tests plus the prior ninety-six tests pass, for ninety-nine Rust tests total.

#### Delivered in the bounded ping-signal follow-up

- `LanePingSignal` is a bounded player intent communication signal abstraction with `None` default and non-default record-identity hash binding (`LANE_PING_SIGNAL_HASH_TAG`).
- `LanerObservation` advertises available ping signals (`None`, `Danger`, `OnMyWay`, `Assist`, `EnemyMissing`).
- Transition evaluation emits direct-immediate `PingSignalSelected` and `PingSignalSet` events and effects, records ping signal in `LaneDebrief`, and verifies replay through `LaneHistory`.
- Three focused ping-signal tests plus the prior ninety-nine tests pass, for 102 Rust tests total.

#### Delivered in the bounded abort-condition follow-up

- `LaneAbortCondition` is a bounded player intent abort condition abstraction with `None` default and non-default record-identity hash binding (`LANE_ABORT_CONDITION_HASH_TAG`).
- `LanerObservation` advertises available abort conditions (`None`, `HealthThreshold`, `ThreatSpotted`, `ResourceDepleted`).
- Transition evaluation emits direct-immediate `AbortConditionSelected`, `AbortConditionSet`, and `AbortConditionTriggered` events and effects, records abort condition in `LaneDebrief`, and verifies replay through `LaneHistory`.
- Three focused abort-condition tests plus the prior 102 tests pass, for 105 Rust tests total.

#### Delivered in the bounded fallback-behavior follow-up

- `LaneFallbackBehavior` is a bounded player intent fallback behavior abstraction with `MaintainPlan` default and non-default record-identity hash binding (`LANE_FALLBACK_BEHAVIOR_HASH_TAG`).
- `LanerObservation` advertises available fallback behaviors (`MaintainPlan`, `RetreatToTower`, `SafeFarm`, `ConserveResources`).
- Transition evaluation emits direct-immediate `FallbackBehaviorSelected`, `FallbackBehaviorSet`, and `FallbackBehaviorTriggered` events and effects, records fallback behavior in `LaneDebrief`, and verifies replay through `LaneHistory`.
- Three focused fallback-behavior tests plus the prior 105 tests pass, for 108 Rust tests total.

#### Delivered in the bounded potion-resource follow-up

- `LanePotion` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_POTION_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player potion count (`self_potion`, `laner_potion`) without exposing opponent potion count.
- `LaneExecutionInputs` supports `potion_gained` and `potion_spent` resolution during execution with direct-immediate `PotionGained`/`PotionSpent`/`PotionChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Potion overflow exceeding maximum bounds (`MAX_LANE_POTION`) or spending without available potions fails before transition state mutation with `LaneExecutionError::PotionOverflow` or `LaneExecutionError::InsufficientPotion`.
- Three focused potion-resource tests plus the prior 108 tests pass, for 111 Rust tests total.

#### Delivered in the bounded elixir-resource follow-up

- `LaneElixir` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_ELIXIR_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player elixir count (`self_elixir`, `laner_elixir`) without exposing opponent elixir count.
- `LaneExecutionInputs` supports `elixir_gained` and `elixir_spent` resolution during execution with direct-immediate `ElixirGained`/`ElixirSpent`/`ElixirChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Elixir overflow exceeding maximum bounds (`MAX_LANE_ELIXIR`) or spending without available elixirs fails before transition state mutation with `LaneExecutionError::ElixirOverflow` or `LaneExecutionError::InsufficientElixir`.
- Three focused elixir-resource tests plus the prior 111 tests pass, for 114 Rust tests total.

#### Delivered in the bounded trinket-resource follow-up

- `LaneTrinket` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_TRINKET_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player trinket count (`self_trinket`, `laner_trinket`) without exposing opponent trinket count.
- `LaneExecutionInputs` supports `trinket_gained` and `trinket_spent` resolution during execution with direct-immediate `TrinketGained`/`TrinketSpent`/`TrinketChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Trinket overflow exceeding maximum bounds (`MAX_LANE_TRINKET`) or spending without available trinkets fails before transition state mutation with `LaneExecutionError::TrinketOverflow` or `LaneExecutionError::InsufficientTrinket`.
- Three focused trinket-resource tests plus the prior 114 tests pass, for 117 Rust tests total.

#### Delivered in the bounded relic-resource follow-up

- `LaneRelic` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_RELIC_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player relic count (`self_relic`, `laner_relic`) without exposing opponent relic count.
- `LaneExecutionInputs` supports `relic_gained` and `relic_spent` resolution during execution with direct-immediate `RelicGained`/`RelicSpent`/`RelicChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Relic overflow exceeding maximum bounds (`MAX_LANE_RELIC`) or spending without available relics fails before transition state mutation with `LaneExecutionError::RelicOverflow` or `LaneExecutionError::InsufficientRelic`.
- Three focused relic-resource tests plus the prior 117 tests pass, for 120 Rust tests total.

#### Delivered in the bounded charm-resource follow-up

- `LaneCharm` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_CHARM_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player charm count (`self_charm`, `laner_charm`) without exposing opponent charm count.
- `LaneExecutionInputs` supports `charm_gained` and `charm_spent` resolution during execution with direct-immediate `CharmGained`/`CharmSpent`/`CharmChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Charm overflow exceeding maximum bounds (`MAX_LANE_CHARM`) or spending without available charms fails before transition state mutation with `LaneExecutionError::CharmOverflow` or `LaneExecutionError::InsufficientCharm`.
- Three focused charm-resource tests plus the prior 120 tests pass, for 123 Rust tests total.

#### Delivered in the bounded scroll-resource follow-up

- `LaneScroll` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_SCROLL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player scroll count (`self_scroll`, `laner_scroll`) without exposing opponent scroll count.
- `LaneExecutionInputs` supports `scroll_gained` and `scroll_spent` resolution during execution with direct-immediate `ScrollGained`/`ScrollSpent`/`ScrollChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Scroll overflow exceeding maximum bounds (`MAX_LANE_SCROLL`) or spending without available scrolls fails before transition state mutation with `LaneExecutionError::ScrollOverflow` or `LaneExecutionError::InsufficientScroll`.
- Three focused scroll-resource tests plus the prior 123 tests pass, for 126 Rust tests total.

#### Delivered in the bounded tome-resource follow-up

- `LaneTome` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_TOME_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player tome count (`self_tome`, `laner_tome`) without exposing opponent tome count.
- `LaneExecutionInputs` supports `tome_gained` and `tome_spent` resolution during execution with direct-immediate `TomeGained`/`TomeSpent`/`TomeChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Tome overflow exceeding maximum bounds (`MAX_LANE_TOME`) or spending without available tomes fails before transition state mutation with `LaneExecutionError::TomeOverflow` or `LaneExecutionError::InsufficientTome`.
- Three focused tome-resource tests plus the prior 126 tests pass, for 129 Rust tests total.

#### Delivered in the bounded rune-resource follow-up

- `LaneRune` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_RUNE_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player rune count (`self_rune`, `laner_rune`) without exposing opponent rune count.
- `LaneExecutionInputs` supports `rune_gained` and `rune_spent` resolution during execution with direct-immediate `RuneGained`/`RuneSpent`/`RuneChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Rune overflow exceeding maximum bounds (`MAX_LANE_RUNE`) or spending without available runes fails before transition state mutation with `LaneExecutionError::RuneOverflow` or `LaneExecutionError::InsufficientRune`.
- Three focused rune-resource tests plus the prior 129 tests pass, for 132 Rust tests total.

#### Delivered in the bounded sigil-resource follow-up

- `LaneSigil` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_SIGIL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player sigil count (`self_sigil`, `laner_sigil`) without exposing opponent sigil count.
- `LaneExecutionInputs` supports `sigil_gained` and `sigil_spent` resolution during execution with direct-immediate `SigilGained`/`SigilSpent`/`SigilChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Sigil overflow exceeding maximum bounds (`MAX_LANE_SIGIL`) or spending without available sigils fails before transition state mutation with `LaneExecutionError::SigilOverflow` or `LaneExecutionError::InsufficientSigil`.
- Three focused sigil-resource tests plus the prior 132 tests pass, for 135 Rust tests total.

#### Delivered in the bounded talisman-resource follow-up

- `LaneTalisman` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_TALISMAN_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player talisman count (`self_talisman`, `laner_talisman`) without exposing opponent talisman count.
- `LaneExecutionInputs` supports `talisman_gained` and `talisman_spent` resolution during execution with direct-immediate `TalismanGained`/`TalismanSpent`/`TalismanChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Talisman overflow exceeding maximum bounds (`MAX_LANE_TALISMAN`) or spending without available talismans fails before transition state mutation with `LaneExecutionError::TalismanOverflow` or `LaneExecutionError::InsufficientTalisman`.
- Three focused talisman-resource tests plus the prior 135 tests pass, for 138 Rust tests total.

#### Delivered in the bounded amulet-resource follow-up

- `LaneAmulet` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_AMULET_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player amulet count (`self_amulet`, `laner_amulet`) without exposing opponent amulet count.
- `LaneExecutionInputs` supports `amulet_gained` and `amulet_spent` resolution during execution with direct-immediate `AmuletGained`/`AmuletSpent`/`AmuletChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Amulet overflow exceeding maximum bounds (`MAX_LANE_AMULET`) or spending without available amulets fails before transition state mutation with `LaneExecutionError::AmuletOverflow` or `LaneExecutionError::InsufficientAmulet`.
- Three focused amulet-resource tests plus the prior 138 tests pass, for 141 Rust tests total.

#### Delivered in the bounded phial-resource follow-up

- `LanePhial` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_PHIAL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player phial count (`self_phial`, `laner_phial`) without exposing opponent phial count.
- `LaneExecutionInputs` supports `phial_gained` and `phial_spent` resolution during execution with direct-immediate `PhialGained`/`PhialSpent`/`PhialChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Phial overflow exceeding maximum bounds (`MAX_LANE_PHIAL`) or spending without available phials fails before transition state mutation with `LaneExecutionError::PhialOverflow` or `LaneExecutionError::InsufficientPhial`.
- Three focused phial-resource tests plus the prior 141 tests pass, for 144 Rust tests total.

#### Delivered in the bounded flask-resource follow-up

- `LaneFlask` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_FLASK_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player flask count (`self_flask`, `laner_flask`) without exposing opponent flask count.
- `LaneExecutionInputs` supports `flask_gained` and `flask_spent` resolution during execution with direct-immediate `FlaskGained`/`FlaskSpent`/`FlaskChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Flask overflow exceeding maximum bounds (`MAX_LANE_FLASK`) or spending without available flasks fails before transition state mutation with `LaneExecutionError::FlaskOverflow` or `LaneExecutionError::InsufficientFlask`.
- Three focused flask-resource tests plus the prior 144 tests pass, for 147 Rust tests total.

#### Delivered in the bounded incense-resource follow-up

- `LaneIncense` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_INCENSE_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player incense count (`self_incense`, `laner_incense`) without exposing opponent incense count.
- `LaneExecutionInputs` supports `incense_gained` and `incense_spent` resolution during execution with direct-immediate `IncenseGained`/`IncenseSpent`/`IncenseChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Incense overflow exceeding maximum bounds (`MAX_LANE_INCENSE`) or spending without available incenses fails before transition state mutation with `LaneExecutionError::IncenseOverflow` or `LaneExecutionError::InsufficientIncense`.
- Three focused incense-resource tests plus the prior 147 tests pass, for 150 Rust tests total.

#### Delivered in the bounded salve-resource follow-up

- `LaneSalve` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_SALVE_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player salve count (`self_salve`, `laner_salve`) without exposing opponent salve count.
- `LaneExecutionInputs` supports `salve_gained` and `salve_spent` resolution during execution with direct-immediate `SalveGained`/`SalveSpent`/`SalveChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Salve overflow exceeding maximum bounds (`MAX_LANE_SALVE`) or spending without available salves fails before transition state mutation with `LaneExecutionError::SalveOverflow` or `LaneExecutionError::InsufficientSalve`.
- Three focused salve-resource tests plus the prior 150 tests pass, for 153 Rust tests total.

#### Delivered in the bounded poultice-resource follow-up

- `LanePoultice` is a bounded player consumable resource with zero default and non-zero state-hash and allied visible-digest tags (`LANE_POULTICE_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` expose player poultice count (`self_poultice`, `laner_poultice`) without exposing opponent poultice count.
- `LaneExecutionInputs` supports `poultice_gained` and `poultice_spent` resolution during execution with direct-immediate `PoulticeGained`/`PoulticeSpent`/`PoulticeChanged` events and effects, debrief recording, and `LaneRecordIdentity` integration.
- Poultice overflow exceeding maximum bounds (`MAX_LANE_POULTICE`) or spending without available poultices fails before transition state mutation with `LaneExecutionError::PoulticeOverflow` or `LaneExecutionError::InsufficientPoultice`.
- Three focused poultice-resource tests plus the prior 153 tests pass, for 156 Rust tests total.

#### Verification

- Identical prior state, validated intent, resolved input, and ruleset yield
  equivalent output and hash.
- Invalid actor, turn, ruleset, stale-hash, and out-of-contract commands fail
  before transition evaluation.
- Actor-visible observation omits latent opponent values and labels unknown or
  last-known information explicitly.
- A legal but unfavorable execution result remains distinct from command
  rejection and is replay-verifiable.

#### Not Yet Done

- The complete lane scenario still needs complete vision/belief updates,
  adaptive pacing, communication, automatic threat/execution timing, causal
  completeness, memory/communication-aware reports, a broader debrief surface,
  and an evidence-backed resource economy. The current delayed effects,
  cooldown, gold, and experience slices are bounded fixtures rather than a
  complete economy; the opponent-report follow-up is one fixed sighting rule
  and does not implement complete vision or beliefs.
- CLI, MCP, full agent ecology, and human-experience evidence remain future
  M3/M4+ work; the M3 grammar foundation is present but this diagnostic slice
  is not playable and makes no enjoyment,
  accessibility, trust, or behavioral-validity claim.

### M3 — CLI grammar foundation — 2026-08-06

**Status:** Bounded grammar, replay-validated host artifacts, injected file
storage, pure terminal text, a thin fixture command loop, and explicit binary
store wiring delivered; scenario selection and complete accessibility evidence
remain open.

- `src/cli.rs` defines stable lowercase command identities and borrowed
  payloads for the planned in-session verbs.
- Parsing returns typed errors for empty input, unknown verbs, missing payloads,
  and unexpected arguments without reading simulation state.
- The parser is an adapter contract only; it does not authorize domain actions,
  persist artifacts, or change the lane transition boundary. `src/terminal.rs`
  renders already-authorized host projections as plain text but performs no
  terminal I/O.
- `observe`, bounded `inspect`, and contextual `help` map to typed read-only
  requests with actor-visible target restrictions and static command metadata.
- `message`, `plan`, `contingency`, `commit`, and `advance` map to distinct
  typed borrowed write requests; the adapter does not map them to `LaneIntent`,
  validate legality, execute a turn, or mutate history.
- `review`, `debrief`, `replay`, and `branch` map to distinct typed borrowed
  process requests (`CliProcessRequest`); the adapter remains host-agnostic,
  while bounded fixture execution, history inspection, and branch rejection
  are covered by the host evidence below.
- `save`, `load`, `undo`, and `quit` map to distinct typed borrowed session
  requests (`CliSessionRequest`); the adapter remains host-agnostic, while
  bounded persistence, uncommitted choice editing, and session lifecycle
  execution are covered by the host/store evidence below.
- Top-level process commands (`play`, `replay`, `branch`, `experiment`, `export`,
  `validate`, `mcp`, `help`, `version`) parse positional and flag options, map
  to typed requests (`CliTopLevelRequest`), enforce interaction modes (`Guided`,
  `Expert`), verbosity policies (`Concise`, `Standard`, `Explanatory`, `Research`),
  and explicit privilege guards (`Unprivileged`, `Privileged`).
- `CliTopLevelHelpCatalog` documents top-level subcommands and their usage
  without adding runtime dependencies or executing simulation state.
- The versioned `m3-cli-information-labels-v1` vocabulary distinguishes
  `observed`, `believed`, `inferred`, `reported`, and `unknown` values for future
  actor-visible CLI projections.
- `CliInformation<T>` preserves the selected label through borrowed
  projections; its explicit `into_option()` extraction intentionally drops
  provenance while returning the payload, and `Unknown` remains payload-free.
  This is adapter metadata only; no inference engine, host flow, or external
  compatibility guarantee exists.
- `CliDraft` stages borrowed message, plan, and contingency payloads with
  last-write-wins edits, clear-all `undo()`, and fail-closed empty/commit/
  advance checks. `commit()` consumes the editable value and returns a
  read-only `CliCommittedDraft`; no host command or committed history is
  created by this adapter marker.
- The CLI rendering boundary is now explicit: the application host solely owns
  true-state lifecycle, legality, ordering, history commit, and adapter
  coordination; the pure kernel and lane modules evaluate validated inputs,
  while `src/cli.rs` remains a request/projection adapter. Current core and CLI
  code has no terminal I/O, rendering loop, or mutable runtime presentation
  state. The versioned `m3-cli-terminal-text-v1` projection consumes
  host-projected actor-valid values at the edge without authorizing commands or
  mutating history; complete client and accessibility evidence remain open.
- `src/command_loop.rs` provides the versioned `m3-cli-command-loop-v1` edge
  adapter. It reads newline-delimited input, continues after bounded errors,
  renders each result through the pure text projection, and stops on `quit` or
  end-of-input. Its bounded process-argument helper recognizes the one
  versioned `--scenario m3-two-window-fixture-v1` ID and `--run-dir` without
  echoing values; `src/main.rs` maps the closed scenario enum to the existing
  fixture and injects the configured artifact store. It does not authorize
  host actions, add prompts/styling, or load external scenario data.
- `CliRunId<'a>` is the versioned `m3-cli-run-id-v1` borrowed identifier for
  save/load/replay/export requests. It accepts bounded human-readable ASCII
  forms and rejects malformed values before host execution; it does not create
  durable storage or guarantee uniqueness, and the host artifact binds it to a
  replay identity.
- CLI tests now exercise a representative grammar transcript and common errors
  across read/write/process/session mappings. Application-edge tests cover the
  bounded process option contract; host-backed scenario, terminal-text,
  fixture-loop, matched-parent branch, and two-process store evidence are
  described below, while full client behavior remains open.
- `src/host.rs` now provides the versioned `m3-cli-host-v1` synchronous host
  fixture. It accepts explicit resolved inputs, maps the grammar to a bounded
  two-window scenario, and returns actor-valid observation/history, outcome,
  replay, and debrief projections while keeping true-state snapshots and hashes
  private. `src/host_artifact.rs` gives save/load a versioned, replay-validated
  text artifact and `src/run_store.rs` provides injected file storage; the
  binary selects the one versioned fixture ID at the process edge and accepts
  an explicit `--run-dir <path>` option while retaining the in-memory fixture
  when that option is absent.
- Host tests cover staged message/plan/contingency text, pre-commit undo,
  commit/advance, artifact save/load and divergent-input rejection, replay
  verification, debrief, quit, malformed plans, matched-parent branch review,
  unsupported branch requests, and
  deterministic repeated runs. A pure text renderer now covers every host
  output/error variant, control character sanitization, and bounded labels. The
  fixture command loop covers stdin/stdout recovery and quit/end-of-input
  behavior. A two-process integration smoke test covers the explicit run
  directory handoff. Broader scenario catalogs, regenerated/graph branching,
  and human keyboard/screen-reader evidence remain unimplemented. Store locking
  and fsync/crash recovery remain open.

## Future

The detailed and canonical order is in `ROADMAP.md`.

- **M2:** complete the one-lane scenario from the active decision-window slice,
  including actor-specific uncertainty, intent, delegated execution, branching,
  and causal debrief.
- **M3:** keyboard-first CLI reference experience.
- **M4:** interpretable non-LLM agent ecology.
- **M5:** thin, versioned, model-agnostic MCP adapter.
- **M6:** automated behavioral experiments and regression evidence.
- **M7:** evidence-limited semantic-to-parametric calibration proof.
- **M8:** trust-sensitive team communication and shot-calling.
- **M9:** bounded multi-lane match prototype.
- **M10:** human usability and accessibility alpha evidence.
- **M11:** optional host-bound GUI if demonstrated needs justify it.
- **M12:** public research-capable alpha with release and claim governance.

## Persistent Product Non-Goals

- Full reproduction of a proprietary game, roster, item catalog, or live
  metagame.
- Real-time mechanical control or reaction-time requirements.
- Networked multiplayer in the initial roadmap.
- Perfect-rationality or global-equilibrium claims.
- Treating AI-agent behavior as human behavior.
- A general-purpose multi-agent simulation framework before a proven vertical
  slice.
- Public, legal, accessibility, entertainment, or scientific claims without the
  evidence appropriate to each claim.
