# Changelog

All meaningful contributor- and user-visible changes are recorded here. The
project uses the versioning policy in `README.md`; documentation-only changes do
not increment the package version.

## Unreleased

## [0.1.198] - 2026-08-16

### Added

- `m9-decision-density-v1` and `m9-decision-density-catalog-v1` preserving
  meaningful decision density through automatic routine execution for M9:
  - `CandidateWindowKind` — 5 routine window kinds (`WaveClear`,
    `ResourceCollection`, `TransitContinuation`, `WardRefresh`,
    `Regeneration`) delegatable to automatic execution and 5 strategic kinds
    (`ObjectiveContest`, `RotationChoice`, `SiegeCommit`, `ThreatResponse`,
    `TeamCoordination`) that always surface an actor decision.
  - `RoutineWindowCandidate` — explicit caller-declared window snapshot (id,
    strictly increasing turn, kind, value stakes in `[0..=10,000]` bp,
    threat/objective presence flags); no authoritative match state consulted.
  - `EscalationTrigger` (`StrategicKind`, `StakesAboveThreshold` strictly
    above the 500 bp `ROUTINE_STAKES_CEILING_BP` mirroring the pivotal
    `ROUTINE_MAX_SWING_BP` routine tier ceiling, `ThreatPresent`,
    `ObjectiveActive`) evaluated in fixed priority order; untriggered routine
    windows resolve as `AutomaticallyExecuted` without forcing a decision
    window.
  - `evaluate_decision_density` — pure function with fail-closed typed errors
    (`EmptyTrajectory`, `StakesOutOfRange`, `NonMonotonicTurn`) validated
    before classification.
  - `DecisionDensityReport` — window/automatic/decision counts, exact
    complement shares (`routine_absorption_bp` + `decision_share_bp` =
    10,000 bp), decision turns, maximum consecutive decision gap, and
    `meets_density_targets` over the explicit `[1,000..=5,000]` bp
    decision-share band and 6-turn decision-gap bound; renders structured
    Markdown without private chain-of-thought or hidden state.
  - `DecisionDensityCatalog` with 3 canonical benchmark scenarios:
    `scenario-routine-laning-absorption-v1` (7 absorbed, 3,000 bp share,
    targets met), `scenario-objective-spike-escalation-v1` (every escalation
    trigger exercised, density holds at the 5,000 bp ceiling),
    `scenario-decision-overload-v1` (8,333 bp share exceeds the band;
    targets missed as the failure mode automatic execution prevents).
  - 28 focused tests: kind classification, escalation triggers and priority,
    the exact 500 bp ceiling boundary and inclusive stakes bound, share
    arithmetic, band and gap boundaries, fail-closed validation,
    reproducibility, catalog outcomes, and Markdown hygiene.

## [0.1.197] - 2026-08-16

### Added

- `m9-pivotal-decision-v1` and `m9-pivotal-catalog-v1` defining match-level
  pivotal-decision detection for M9:
  - `PivotalDecisionSample` — explicit caller-declared decision measurement
    (decision id, strictly increasing turn, acting side, Allied-perspective net
    match value before/after in `[-10,000..=10,000]` bp); no authoritative
    match state consulted.
  - `PivotalTier` (4 discrete tiers: `Routine`, `Notable`, `Pivotal`,
    `MatchDefining`) classified from absolute swing magnitude with explicit
    500/1,500/3,500 bp thresholds.
  - `SwingDirection` (`AlliedFavorable`/`OpposingFavorable`/`Neutral`) and
    `DecisionAlignment` (`SwingWithActor`/`SwingAgainstActor`/`NeutralSwing`)
    separating outcome direction from acting-side attribution.
  - Strict lead-change detection: only a value-sign flip counts; passing to or
    from exact parity does not.
  - `detect_pivotal_decisions` — pure function with fail-closed typed errors
    (`EmptyTrajectory`, `ValueOutOfRange`, `NonMonotonicTurn`) validated before
    classification.
  - `PivotalDecisionReport` — findings in turn order, `most_pivotal` (largest
    absolute swing, earliest-turn tie-break), `pivotal_count`, ranked
    `pivotal_findings()`, `lead_change_turns`, `final_value_bp`, and saturating
    `total_absolute_swing_bp`; renders structured Markdown without private
    chain-of-thought or hidden state.
  - `PivotalCatalog` with 3 canonical benchmark scenarios:
    `scenario-base-race-decisive-swing-v1` (match-defining swing),
    `scenario-baron-throw-comeback-v1` (against-actor throw + lead change),
    `scenario-stable-slow-burn-v1` (no pivotal decisions).
  - 24 focused tests: tier boundaries, direction/alignment matrices, strict
    lead-change semantics, ranking tie-break, fail-closed validation,
    reproducibility, aggregates, catalog outcomes, and Markdown hygiene.

## [0.1.196] - 2026-08-16

### Added

- `m9-comeback-mechanics-v1` and `m9-comeback-catalog-v1` defining comeback opportunity
  evaluation and variance-seeking behavior recommendations for M9:
  - `DeficitLevel` (4 discrete tiers: `Ahead`, `Parity`, `Deficit`, `SevereDeficit`)
    classified from explicit structural and objective net-delta inputs
    (`[-10,000..=10,000]` bp); no hidden authoritative state consulted.
  - `VarianceSeekingBehavior` (4 discrete strategies: `ConservativePlay`,
    `BalancedApproach`, `HighRiskEngage`, `DesperationAllIn`) recommended
    deterministically from deficit level, match phase, composition power curves,
    and recent high-value objective presence.
  - `ComebackOpportunityInputs` — explicit caller-supplied snapshot of structural
    counts, objective counts, match phase, and composition power ratings.
  - `ComebackEvaluation` — deterministic result with `net_value_delta_bp: i32`,
    `base_opportunity_bp: u32`, `variance_multiplier_bp: u16`, and
    `variance_play_recommended: bool`; renders structured Markdown without
    private chain-of-thought or hidden state.
  - `evaluate_comeback_opportunity` — pure function; no side effects, randomness,
    or authoritative state access.
  - `ComebackCatalog` with 3 canonical benchmark scenarios:
    1. `scenario-teamfight-comeback-v1`: TeamfightScaling with recent Drake in late
       game (`Deficit` → `HighRiskEngage`).
    2. `scenario-desperation-all-in-v1`: EarlyPick in severe late-game deficit
       (`SevereDeficit` → `DesperationAllIn`).
    3. `scenario-ahead-conservative-v1`: SplitPush leading mid-game
       (`Ahead` → `ConservativePlay`).
  - 20 focused library tests covering deficit classification, variance multiplier
    ordering monotonicity, reproducibility, Allied/Opposing perspective symmetry,
    net-delta clamping, catalog scenario outcomes, and Markdown rendering.

### Added

- `m9-role-observation-v1`, `m9-role-action-v1`, `m9-role-debrief-v1`, and
  `m9-role-scenario-catalog-v1` defining role-specific observations, tactical intents,
  debrief perspectives, and benchmark scenarios for all 5 match roles in M9:
  - `WaveStateSummary` and `RoleSpecificContext` (`TopLanerContext`, `JunglerContext`,
    `MidLanerContext`, `BotCarryContext`, `SupportContext`) projecting situational
    context and wave status with strict fog-of-war compliance (`RoleMatchObservation`).
  - `RoleIntent` closed tactical intent spaces (`TopIntent`, `JungleIntent`, `MidIntent`,
    `BotCarryIntent`, `SupportIntent`) and role action validation (`validate_role_action`
    with `RoleActionError`).
  - `RoleKpis` (integer basis-point metrics in $[0..=10,000]$ bp), composite role ratings,
    performance tiers (`RolePerformanceTier`), 16 discrete causal drivers (`RoleCausalFactor`),
    and structured Markdown debrief perspectives with zero private chain-of-thought (`RoleDebriefPerspective`).
  - `RoleScenarioCatalog` registering and executing 5 canonical benchmark scenarios:
    1. `scenario-top-teleport-flank-v1`: TopLaner TP flank at Dragon contest.
    2. `scenario-jungler-objective-steal-v1`: Jungler fog infiltration and Smite secure.
    3. `scenario-mid-roam-conversion-v1`: MidLaner wave shove and 3v2 Bot dive.
    4. `scenario-bot-hypercarry-scaling-v1`: BotCarry late-game kiting and sustained DPS.
    5. `scenario-support-vision-setup-peel-v1`: Support river de-ward and assassin peel.

- `m9-team-composition-v1`, `m9-match-structures-v1`, `m9-match-victory-v1`, and
  `m9-match-scenario-catalog-v1` defining team composition archetypes, match roles,
  power scaling curves, structures defense hierarchy, super minion pressure, and
  match victory terminal conditions for M9:
  - `MatchRole` (5 discrete roles: `TopLaner`, `Jungler`, `MidLaner`, `BotCarry`, `Support`).
  - `CompositionArchetype` (4 discrete archetypes: `EarlyPick`, `TeamfightScaling`, `SplitPush`, `PokeSiege`).
  - `PowerScalingCurve` and `CompositionMatchupEvaluation` with integer basis-point
    power scaling ($[0..=10,000]$ bp) across `EarlyGame`, `MidGame`, and `LateGame`, net power
    deltas ($[-10,000..=10,000]$ bp), and `RecommendedPosture`.
  - `StructureTier` (`OuterTurret`, `InnerTurret`, `InhibitorTurret`, `Inhibitor`, `Nexus`),
    `StructureStatus`, and `MatchStructureState` tracking all 26 defensive structures across
    Allied and Opposing sides with deterministic vulnerability hierarchy enforcement.
  - `transition_structure_siege` resolving attack damage, defense mitigation, structure destruction,
    super minion wave spawning (`has_super_minions`), inhibitor respawn ticking (`tick_turn`),
    `StructureEvent`, and `StructureEffect`.
  - `MatchVictoryCondition` (`NexusDemolished`, `MatchConceded`, `DecisiveAce`), `MatchStatus`,
    and `MatchTerminalEvaluation` evaluating match conclusion milestones with structured Markdown
    summaries and zero private chain-of-thought.
  - `MatchScenarioCatalog` registering and executing 4 canonical benchmark match scenarios:
    1. `scenario-early-pick-snowball-v1`: Early pick comp tears down Mid defenses, demolishing Opposing Nexus at turn 18.
    2. `scenario-split-push-base-race-v1`: Split-push comp trades Baron concession for Bot inhibitor + Nexus demolition in an uncontested base race at turn 22.
    3. `scenario-late-game-scaling-comeback-v1`: Scaling comp holds Tier 3 high ground, scales to late game, wins decisive ace and marches to victory at turn 28.
    4. `scenario-siege-inhibitor-concession-v1`: Poke/siege comp breaks all 3 inhibitors, forcing match concession from overwhelming super minion pressure at turn 24.
- `m9-objective-cycles-v1`, `m9-vision-control-v1`, `m9-objective-contest-v1`, and
  `m9-objective-catalog-v1` defining neutral objective spawning state machines
  (`TopRiverObjective` Herald/Baron, `BotRiverObjective` Drake) with `Unspawned`,
  `Active`, and `Secured` statuses, health pools (3500-5000 HP), deterministic
  turn-tick countdowns, dynamic vision control (`VisionWard`, `VisionCoverage`,
  `MapVisionState`, `VisionCommand` with range/capacity validation), cross-map
  tradeoff evaluations (`TradeoffEvaluation`, `TradeClassification` with exact
  $[-10,000..=10,000]$ bp net deltas), and `ObjectiveScenarioCatalog` with 4
  canonical benchmark scenarios (`dragon_contest`, `cross_map_trade`,
  `vision_setup_and_catch`, `stealth_objective_sneak`).
- TTY `> ` prompt, Tab completion, live verb coloring, optional ANSI, richer
  `help`/`?` topics, and actor-safe session chrome for
  `m3-two-window-fixture-v1`. Piped sessions stay labeled plain text.
- Beginner [How to Play](HOW_TO_PLAY.md) walkthrough of the current
  `m3-two-window-fixture-v1` runner commands.
- Explicit MIT source license, contributor policy, code of conduct, and
  unofficial/noncommercial project notice with an original-setting fallback and
  conservative distribution boundary.
- Concise design principles, authoritative terminology, and ADR-0001 for the
  host-owned deterministic transition boundary.
- Pinned Rust `1.96.0` toolchain and binary package lockfile, with ADR-0002
  keeping M1 in one Cargo package.
- Minimum artifact/replay compatibility and dependency, security, and license
  policy documents for the pre-implementation-to-M1 boundary.
- Canonical evidence-gated project roadmap with milestone dependencies, exit
  evidence, explicit deferrals, and maintenance rules.
- Lightweight specification and architecture state documents that distinguish
  the current placeholder from planned capabilities.
- Repo-wide `AGENTS.md` guidance and a portable Fog of Intent agent harness for
  simulation design, agent-ecology design, synthesis, and domain QA.
- Repo-local `foi-test-player` agent skill for interactive showcase playtesting,
  early-stage feature/functional verification, and late-stage gameplay feel evaluation.
- Deterministic `_workspace/` handoff conventions for substantial work.

### Changed
 
- Package `0.1.194` defines M9 team composition archetypes, match structures hierarchy,
  super minion pressure, and match victory terminal conditions with deterministic FNV-1a state hashing.
- Package `0.1.193` defines M9 neutral objective cycles, vision control, and
  cross-map tradeoff evaluation contracts with deterministic FNV-1a state hashing.
- Package `0.1.192` records one deferred edge crate, `reedline`, for TTY line
  editing only. `--color auto|always|never` selects presentation coloring.
- Condensed `README.md` into a short entry point with a human Quickstart and a
  live fixture transcript; M3–M8 library inventory remains in `SPEC.md`.
- M0 is promoted to complete after the hosted clean-checkout CI run passed; the
  first bounded M1 deterministic-kernel fixture is now the active project-state
  slice.
- M1 is promoted to complete after its replay, codec, determinism, and bounded
  invariant evidence passed; the first bounded M2 lane decision-window slice is
  now active.
- Reconciled the M2 minimum lane/wave/position/health/resource checklist item
  with the existing bounded v2 implementation; no package version increment or
  runtime change was needed.
- Reconciled the M2 bounded intent/commitment/focus/communication/abort/fallback
  definition with existing v2 request, observation, validation, and replay
  evidence; free-form communication remains deferred.
- Reconciled M2 causal/information evidence for effect provenance, non-binary
  outcomes, hidden-state/report coverage, and complete-replay inspection;
  vision/belief remains deferred; the bounded automatic-advance condition
  contract is now explicit while host scheduling remains deferred.
- Reconciled the M3 terminal-rendering boundary with source evidence: the
  application host remains the sole simulation authority, the pure kernel/lane
  modules evaluate validated inputs, and the current CLI adapter owns no
  terminal I/O, rendering loop, or mutable runtime presentation state; a future
  renderer remains an outer adapter concern.
- Added a bounded M5 authorization/redaction regression matrix over wrong-actor
  action, draft, commit, and draft-receipt requests; actor-visible DTOs remain
  free of hidden-state, hash, execution, and raw provenance fields.

## 0.1.64 — 2026-08-08

### Added

- Added the versioned `m3-cli-information-labels-v1` vocabulary for
  `observed`, `believed`, `inferred`, `reported`, and `unknown` actor-visible
  information.
- Added generic `CliInformation<T>` values whose `Unknown` form carries no
  payload, with focused tests for canonical names, redaction, borrowing, and
  explicit extraction.

### Known limits

- The labels are a pure adapter contract; terminal rendering, host execution,
  inference, persistence, and human usability evidence remain deferred.

## 0.1.65 — 2026-08-08

### Added

- Added the versioned `m3-cli-precommit-draft-v1` contract with typed local
  staging for message, plan, and contingency edits.
- Added clear-all `CliDraft::undo()` and a consuming `CliCommittedDraft`
  read-only marker; empty payloads and commit/advance staging fail closed.
- Added focused tests for last-write-wins edits, undo isolation, malformed
  staging, and committed-choice readback.

### Known limits

- Drafts remain adapter-local borrowed values; host command execution,
  persistence, transcript acceptance, and authoritative history are deferred.

## 0.1.66 — 2026-08-08

### Added

- Added the versioned `m3-cli-run-id-v1` borrowed identifier contract with
  bounded human-readable syntax and typed malformed-ID errors.
- Applied validated `CliRunId` values to session save/load, in-session replay,
  and top-level replay/export adapter requests with focused mapping tests.

### Known limits

- Run IDs remain adapter syntax only; generation, persistence, uniqueness,
  resume behavior, and human discoverability remain deferred.

## 0.1.191 — 2026-08-13

### Added

- Added `m9-map-topology-v1`, `m9-travel-model-v1`, `m9-map-observation-v1`, and `m9-map-scenario-catalog-v1`
  in `src/map/`, defining the spatial topology and deterministic travel model for M9:
  - `MapLocation` (`src/map/topology.rs`) covering 15 discrete map locations across 2 team bases (`AlliedBase`, `OpposingBase`),
    9 lane sectors (3 lanes `Top`, `Mid`, `Bot` across 3 sectors `NearTower`, `Center`, `FarSide`), 2 river zones (`TopRiver`, `BotRiver`),
    and 2 jungle quadrants (`TopJungle`, `BotJungle`).
  - `TravelRoute` and `compute_shortest_route` (`src/map/graph.rs`) implementing deterministic BFS pathfinding over a symmetric
    15-node adjacency matrix with integer beat durations ($1\text{ beat} = 1\text{ step}$).
  - `ActorLocation` (`Stationary` vs `InTransit`), `TransitState` machine, and `TravelCommand` (`InitiateRotation`, `ContinueTransit`, `AbortRotation`)
    in `src/map/travel.rs` with fail-closed validation.
  - `transition_travel` (`src/map/transition.rs`) providing pure deterministic transit progression, arrival handling, abort redirection,
    and structured `TravelEvent` and `TravelEffect` emissions.
  - `MatchMapState` (`src/map/state.rs`) managing multi-actor locations, turn ticking, deterministic FNV-1a state hashing, and `MatchMapObservation`
    projections with strict fog-of-war redactions (unseen opponents in fog are reported as `Unknown`).
  - `MapTravelCatalog` (`src/map/catalog.rs`) registering and executing 4 canonical benchmark rotation scenarios:
    1. `scenario-top-to-mid-gank-v1`: Top laner rotates through Top River to Mid Center over 2 beats to execute a gank.
    2. `scenario-bot-to-river-contest-v1`: Bot duo rotates from Near Tower to Bot River over 2 beats for dragon river vision setup.
    3. `scenario-mid-to-base-reset-v1`: Mid laner retreats from enemy tower through mid lane back to base over 3 beats.
    4. `scenario-aborted-rotation-threat-v1`: Laner rotates toward river, spots threat on beat 1, and aborts rotation safely back to tower.

### Known limits

- Objective cycle timers, base destruction victory conditions, and cross-lane combat resolution remain planned for subsequent M9 slices.

## 0.1.190 — 2026-08-13

### Added

- Added `m8-team-communication-debrief-v1`, `m8-team-leadership-debrief-v1`, `m8-team-encounter-debrief-v1`,
  `CommunicationDebriefSummary`, `LeadershipDebriefSummary`, `TeamEncounterDebriefReport`, and `TeamDebriefError`
  in `src/agent/debrief.rs`, delivering post-encounter causal debrief reporting for team communication and leadership:
  - `CommunicationDebriefSummary` tracking packet delivery counts (sent, delivered, delayed, dropped overload, suppressed distrusted),
    basis-point transmission reliability ($[0..=10,000]$ bp), clarity degradation, dialogue rounds, and categorical dissent breakdowns (`TeamDissentReason`).
  - `LeadershipDebriefSummary` tracking directive compliance/dissent counts, compliance rates in basis points, consensus deadlocks,
    fallback activations, and caller reputation updates ($[-10,000..=10,000]$ bp).
  - `TeamEncounterDebriefReport` synthesizing multi-agent simultaneous resolutions, decoupled strategic attribution, communication debriefs,
    leadership debriefs, and strategic takeaways into structured Markdown reports with strict zero private chain-of-thought enforcement (`chain_of_thought_present == false`).
- Added `m8-strategic-disagreement-v1`, `DisagreementLegitimacyClassification`, `DisagreementLegitimacyEvaluation`,
  `TeamDisagreementEvaluator`, and `TeamDisagreementError` in `src/agent/disagreement.rs`, formally proving and evaluating the strategic legitimacy of disagreement:
  - `DisagreementLegitimacyClassification` distinguishing `LegitimateDissent` (dissent prevents disaster), `ConstructiveAlternative`
    (dissent offers better value), and `UnjustifiedInsubordination` (dissent actively harms the team).
  - `TeamDisagreementEvaluator` computing counterfactual value deltas ($[-10,000..=10,000]$ bp) and proving that dissent is value-accretive
    under adverse health and threat conditions.
- Added `m8-team-scenarios-v1`, `m8-team-scenario-catalog-v1`, `TeamScenarioDefinition`, `TeamScenarioExecutionResult`,
  `TeamScenarioCatalog`, and `TeamScenarioError` in `src/agent/scenarios.rs`, registering and executing 5 canonical benchmark scenarios:
  1. `scenario-high-trust-gank-v1`: High-reputation caller, crisp channel, unanimous compliance resulting in `CoordinatedTriumph`.
  2. `scenario-low-trust-dissent-v1`: Distrusted caller, autonomous actor dissents to protect wave position (`UncoordinatedBailout`).
  3. `scenario-conflicting-calls-arbitration-v1`: Competing peer proposals arbitrated deterministically via `HighestReputationLead` consensus rule without deadlocks.
  4. `scenario-missing-message-fallback-v1`: Channel loss drops proposal packet; receiver safely activates fallback routine (`FallbackToDefaultHold`).
  5. `scenario-strategic-dissent-survival-v1`: Caller orders reckless contest under low health; teammate legitimately dissents to yield, preventing lethal wipe (+8,000 bp counterfactual delta).

### Known limits

- This completes Phase 8 (M8); bounded multi-lane match mechanics and cross-lane rotations remain planned for Phase 9 (M9).

## 0.1.189 — 2026-08-13

### Added

- Added `m8-coordination-execution-attribution-v1`, `m8-coordination-execution-attribution-report-v1`,
  `m8-coordination-attribution-catalog-v1`, `AttributionQuadrant`, `CoordinationRating`, `ExecutionRating`,
  `CoordinationCausalFactor`, `ExecutionCausalFactor`, `CoordinationAssessment`, `ExecutionAssessment`,
  `AttributionWeights`, `CoordinationExecutionAttribution`, `CoordinationExecutionAttributionReport`,
  `AttributionEvaluationInput`, `TeamAttributionEvaluator`, `AttributionScenario`, `CoordinationAttributionCatalog`,
  and `TeamAttributionError` in `src/agent/attribution.rs`, decoupling strategic team coordination from mechanical execution outcomes to eliminate outcome bias in causal debriefs for M8:
  - `AttributionQuadrant` classifying team turn outcomes into 4 canonical quadrants (`CoordinatedTriumph`,
    `CoordinatedFailure`, `UncoordinatedBailout`, `CompoundedFailure`) based on orthogonal coordination
    effectiveness ($\ge 5,000$ bp) and mechanical execution efficiency ($\ge 5,000$ bp) thresholds.
  - Discrete performance tiers (`CoordinationRating` and `ExecutionRating`) and 8 discrete causal factor
    taxonomies for each dimension (`CoordinationCausalFactor` and `ExecutionCausalFactor`).
  - `AttributionWeights` enforcing exact integer basis-point sum conservation ($10,000$ bp invariant:
    `coordination + execution + exogenous == 10_000` bp) without floating-point arithmetic.
  - `CoordinationExecutionAttributionReport` providing structured Markdown debrief rendering and fail-closed
    zero private chain-of-thought rejection (`chain_of_thought_present == false`).
  - `TeamAttributionEvaluator` synthesizing `TeamSimultaneousResolution` with physical lane outcomes.
  - `CoordinationAttributionCatalog` registering 6 canonical benchmark scenarios (`attr-coordinated-triumph-gank-v1`,
    `attr-coordinated-failure-overreach-v1`, `attr-uncoordinated-bailout-clutch-v1`,
    `attr-compounded-failure-deadlock-v1`, `attr-legitimate-dissent-avoided-wipe-v1`,
    `attr-trust-breakdown-execution-miss-v1`) with fail-closed lookup and mathematical validation.

### Known limits

- This contract establishes decoupled coordination and execution attribution; high-trust/low-trust/conflicting-call scenario batteries and multi-turn match debriefs remain open.

## 0.1.188 — 2026-08-13

### Added

- Added `m8-team-simultaneous-submission-v1`, `m8-team-simultaneous-resolution-v1`,
  `m8-team-simultaneous-catalog-v1`, `TeamSimultaneousPhase`, `TeamCoordinationOutcome`,
  `TeamSubmissionEnvelope`, `TeamSubmissionReceipt`, `TeamSimultaneousWindow`,
  `RoleResolvedIntent`, `TeamSimultaneousResolution`, `TeamSimultaneousResolver`,
  `TeamSimultaneousCatalog`, `TeamSimultaneousScenario`, and `TeamSimultaneousError`
  in `src/agent/simultaneous.rs`, preserving private multi-agent submissions and enabling
  deterministic simultaneous resolution for M8:
  - `TeamSubmissionEnvelope` encapsulating actor role, observation ID, turn, intent,
    target focus, commitment, ping signal, optional staged message, optional individual plan,
    and strict fail-closed rejection of private chain-of-thought (`chain_of_thought_present == false`).
  - `TeamSubmissionReceipt` providing lightweight, payload-free receipt confirmation without
    echoing submitted choices to peers.
  - `TeamSimultaneousWindow` managing a bounded multi-agent collection window (up to 4 roles)
    with strict privacy protection during the `CollectingSubmissions` phase (`get_submission`
    and `submissions()` fail closed, and `Debug` redacts uncommitted choices).
  - `TeamSimultaneousResolver` evaluating multi-actor plan alignment (`TeamPlanEvaluator`),
    proposal trust compliance (`TeamTrustEvaluator`), and leadership consensus/directives
    (`TeamLeadershipEvaluator`) into integer basis-point cohesion ($[0..=10,000]$ bp) and
    discrete `TeamCoordinationOutcome` classifications (`FullyCoordinated`, `PartiallyCoordinated`,
    `DivergentIntents`, `ConflictingDirectives`, `CommunicationFailure`).
  - `TeamSimultaneousCatalog` defining 5 canonical reference simultaneous resolution scenarios
    (`simultaneous-gank-coordinated-v1`, `simultaneous-defensive-fallback-v1`,
    `simultaneous-dissent-tradeoff-v1`, `simultaneous-conflicting-directives-v1`,
    `simultaneous-communication-failure-v1`) with fail-closed lookup and validation.

### Known limits

- This contract establishes private submission collection and simultaneous multi-agent resolution; causal attribution of coordination success/failure separate from execution and multi-turn match scenarios remain open.

## 0.1.187 — 2026-08-12

### Added

- Added `m8-leadership-structure-v1`, `m8-shot-caller-policy-v1`, `m8-decentralized-coordination-v1`,
  `m8-leadership-evaluation-report-v1`, `ConsensusRule`, `FallbackLeadershipMode`, `LeadershipStructure`,
  `LeadershipResolutionOutcome`, `ShotCallerDirective`, `ShotCallerPolicy`, `PeerPlanProposal`,
  `DecentralizedCoordinator`, `LeadershipEvaluationReport`, `TeamLeadershipEvaluator`,
  `LeadershipCatalog`, and `TeamLeadershipError` in `src/agent/leadership.rs`, establishing designated
  shot-caller and decentralized coordination baseline policies for M8:
  - `ConsensusRule` providing 4 discrete peer proposal arbitration algorithms (`UnanimousConsensus`,
    `HighestReputationLead`, `UrgencyFirst`, `MajoritySupport`).
  - `FallbackLeadershipMode` providing 3 predictable fallback mechanisms (`FallbackToIndividualPlans`,
    `FallbackToDefaultHold`, `FallbackToSecondaryCaller`) when directives or consensus fail.
  - `LeadershipStructure` modeling `DesignatedShotCaller`, `Decentralized`, and `SharedLeadership` team
    authority configurations.
  - `ShotCallerDirective` and `ShotCallerPolicy` enabling designated leaders to evaluate local observations
    and issue structured communicative team plan proposals.
  - `PeerPlanProposal` and `DecentralizedCoordinator` enabling leaderless peer teams to submit bounded plan
    proposals with exact basis-point reputation ratings and zero chain-of-thought enforcement.
  - `TeamLeadershipEvaluator` simulating and evaluating compliance, dissent reasons, and cohesion across
    evaluating teammates against trust matrices and local observations.
  - `LeadershipCatalog` defining and validating 6 canonical reference leadership baseline configurations.

### Known limits

- This contract establishes designated shot-caller heuristics and decentralized consensus arbitration baselines; simultaneous private submission resolution across multi-turn match scenarios remains open.

## 0.1.186 — 2026-08-12

### Added

- Added `m8-team-trust-v1`, `m8-caller-reputation-v1`, `m8-communication-channel-v1`,
  `TeamTrustLevel`, `CallOutcome`, `CallerReputationRecord`, `TeamTrustMatrix`,
  `CommunicationClarity`, `TransmissionDelay`, `DeliveryStatus`, `ChannelPacket`,
  `TeamCommunicationChannel`, `TrustComplianceDecision`, `TrustEvaluationReport`,
  `TeamTrustEvaluator`, `TeamTrustCatalog`, and `TeamTrustError` in `src/agent/trust.rs`,
  establishing multi-agent trust dynamics, caller reputation, and communication channel physics for M8:
  - `TeamTrustLevel` categorizing trust from basis points into 4 discrete tiers (`HighTrust`,
    `StandardTrust`, `LowTrust`, `Distrusted`).
  - `CallerReputationRecord` tracking historical successful, failed, and abandoned calls with exact
    integer basis-point score updates ($[0..=10,000]$ bp) and zero chain-of-thought enforcement.
  - `TeamTrustMatrix` providing pairwise role reputation indexing and average team reputation calculation.
  - `CommunicationClarity` modeling 4 discrete clarity levels (`Crisp`, `Ambiguous`, `Degraded`, `Garbled`)
    with basis-point multipliers ($1,000..=10,000$ bp).
  - `TransmissionDelay` managing simulated beat delay steps (`Immediate`, `OneBeat`, `TwoBeats`).
  - `TeamCommunicationChannel` providing a bounded FIFO queue (capacity 16 packets) with turn-tick delay
    progression, distrusted sender suppression, capacity overload dropping, and visibility filtering.
  - `TeamTrustEvaluator` deterministically evaluating proposal compliance, clarification requests, and
    dissent reasons (`PostureIncompatible`, `ThreatDetected`, `LowHealth`, `ManaDeficit`) based on
    caller reputation, message clarity, and local recipient observations.
  - `TeamTrustCatalog` providing discovery and validation helpers for canonical reference caller profiles.

### Known limits

- This contract establishes structured caller reputation scoring, trust-modulated compliance, transmission delay queues, and channel capacity limits; designated shot-caller heuristics, centralized vs decentralized leadership baselines, and simultaneous private resolution remain open.

## 0.1.185 — 2026-08-12

### Added

- Added `m8-team-plan-v1`, `m8-individual-plan-v1`, `m8-team-plan-relationship-v1`,
  `TeamStrategicObjective`, `TeamPlanPhase`, `RolePlanAssignment`, `TeamPlanDefinition`,
  `IndividualPlanDefinition`, `TeamPlanAlignmentType`, `AlignmentEvaluation`,
  `TeamPlanEvaluator`, `TeamPlanAlignmentReport`, `TeamPlanCatalog`, and `TeamPlanError`
  in `src/agent/team_plan.rs`, establishing team-plan definitions and deterministic alignment evaluation:
  - `TeamStrategicObjective` covering 6 discrete tactical objectives (`GankSetup`, `LaneSiege`,
    `DefensiveHold`, `ResourceFarming`, `ObjectiveContest`, `TacticalReset`).
  - `TeamPlanPhase` covering 4 discrete plan phases (`Preparation`, `Execution`, `Disengagement`, `Contingency`).
  - `RolePlanAssignment` binding actor roles to assigned intents, target focuses, commitments, and fallback behaviors.
  - `TeamPlanDefinition` and `IndividualPlanDefinition` with strict zero private chain-of-thought enforcement (`chain_of_thought_present == false`).
  - `TeamPlanAlignmentType` tracking 5 discrete alignment relationships (`Aligned`, `Divergent`,
    `ConditionalCompliance`, `Independent`, `Conflicted`).
  - `AlignmentEvaluation` assessing intent matches, target focus compatibility, prerequisite condition satisfaction, and causal dissent reasons (`TeamDissentReason`).
  - `TeamPlanEvaluator` deterministically evaluating individual and whole-team alignment with exact integer basis-point cohesion scoring ($[0..=10,000]$ bp) and formatted Markdown reporting.
  - `TeamPlanCatalog` providing discovery and validation helpers for 6 canonical reference team plans.

### Known limits

- This contract establishes structured team plans, role assignments, individual plan bindings, and deterministic alignment evaluation; multi-agent trust dynamics, caller reputation, designated shot-caller heuristics, and leadership arbitration remain open.

## 0.1.184 — 2026-08-12

### Added

- Added `m8-team-dialogue-v1`, `TeamDialogueStatus`, `TeamDissentReason`,
  `TeamConditionEvaluator`, `TeamSpeechActProfile`, `TeamEvaluationOutcome`,
  `TeamDialogueSession`, and `TeamDialogueCatalog` in `src/agent/communication.rs`,
  establishing speech act evaluation and multi-turn dialogue session state machines:
  - `TeamDialogueStatus` tracking 8 discrete dialogue states (`Idle`, `Proposed`, `Clarifying`,
    `Negotiating`, `Agreed`, `Diverged`, `Aborted`, `Failed`).
  - `TeamDissentReason` covering 6 discrete causal dissent reasons (`LowHealth`, `ThreatDetected`,
    `ManaDeficit`, `CooldownActive`, `AlternativeObjectivePriority`, `PostureIncompatible`).
  - `TeamConditionEvaluator` deterministically evaluating tactical prerequisite conditions
    (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`, `ResourceSufficient`)
    against actor-visible observation state.
  - `TeamSpeechActProfile` evaluating incoming proposals across `Cautious`, `RiskTaking`, and
    `Yielding` strategic postures with posture-consistent evaluation outcomes.
  - `TeamDialogueSession` managing bounded multi-turn dialogue transitions (max 4 rounds,
    max 8 messages), participant validation, and Markdown transcript formatting.
  - `TeamDialogueCatalog` registering 7 canonical complete dialogue transcripts covering all 8
    speech acts with fail-closed lookup and validation.

### Known limits

- This contract establishes structured speech act evaluations, prerequisite condition checks, and dialogue state machines; multi-agent trust dynamics, caller reputation, designated shot-caller heuristics, and team-plan negotiation remain open.

## 0.1.183 — 2026-08-12

### Added

- Added `m8-team-communication-v1`, `m8-team-speech-act-v1`, `m8-team-message-envelope-v1`,
  `TeamSpeechAct`, `TeamRecipient`, `TeamMessageUrgency`, `TeamConfidenceLevel`,
  `TeamMessageCondition`, `TeamMessageVisibility`, `TeamCommunicationError`,
  `TeamMessageEnvelope`, and `TeamCommunicationCatalog` in `src/agent/communication.rs`,
  establishing the foundational M8 team communication contracts:
  - `TeamSpeechAct` covering 8 canonical communicative speech acts (`Proposal`, `Clarification`,
    `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`, `Withdrawal`, `FailureReport`).
  - `TeamRecipient` covering broadcast (`Broadcast`) and directed (`Direct(LaneActorRole)`) targeting.
  - `TeamMessageUrgency` (`Low`, `Standard`, `Critical`) and `TeamConfidenceLevel` (`Tentative`, `Confident`, `Definite`).
  - `TeamMessageCondition` (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`, `ResourceSufficient`).
  - `TeamMessageVisibility` (`TeamOnly`, `DirectOnly`, `Public`) with actor/team visibility predicate rules preventing unauthorized information leakage across team boundaries.
  - `TeamMessageEnvelope` with structured metadata, observation and intent binding, Markdown formatting, and strict fail-closed rejection if private chain-of-thought is present (`chain_of_thought_present == true`).
  - `TeamCommunicationCatalog` containing registered canonical example envelopes for all 8 speech acts with fail-closed lookup and validation.

### Known limits

- This contract establishes structured semantic communication schemas, addressing, and visibility rules; multi-agent trust dynamics, caller reputation, designated shot-caller heuristics, and team-plan negotiation remain open.

## 0.1.182 — 2026-08-12

### Added

- Added `m7-recalibration-trigger-v1`, `m7-recalibration-evaluation-v1`, `RecalibrationTriggerReason`,
  `RecalibrationUrgency`, `RecalibrationTriggerCondition`, `RecalibrationPolicy`, and
  `RecalibrationEvaluationReport` in `src/agent/recalibration.rs`, defining deterministic recalibration
  triggers across 9 discrete reasons (`ModelVersionChanged`, `PromptProtocolChanged`,
  `TotalVariationDistanceBreach`, `ModalChoiceDisagreement`, `UnidentifiableParameterDetected`,
  `UnstableSemanticLabel`, `HeldOutLossBreach`, `CounterfactualCoherenceFailure`, `ChainOfThoughtLeakage`)
  with integer basis-point thresholds ($1,500$ bp TVD, max 1 modal disagreement, $2,500$ bp held-out loss limit).
- Added canonical baseline evaluation suites in `RecalibrationEvaluationReport` for `cautious_v1`,
  `risk_taking_v1`, and `yielding_v1`, evaluating model/prompt drift with formatted Markdown reporting
  and explicit calibration disclaimers.
- Added `m7-calibration-model-card-v1` and `CalibrationModelCardReport` in `src/agent/recalibration.rs`,
  formalizing the canonical M7 calibration proof deliverable with intended use, evidence limits,
  evaluated profiles, held-out generalization status, uncertainty findings, recalibration policy summary,
  and zero private chain-of-thought observability rules.

### Known limits

- Live model provider APIs, network adapters, and online telemetry remain explicitly deferred.

## 0.1.181 — 2026-08-12

### Added

- Added `m7-reference-output-v1`, `ReferenceOutputRecord`, `StructuredRationale`,
  `StructuredRationaleCategory`, and `ReferenceOutputError` in `src/agent/reference_output.rs`,
  capturing observable decision outputs (`LaneIntent`, `LaneTargetFocus`, `LaneCommitment`,
  `LanePingSignal`, bounded `StructuredRationale`) with strict fail-closed rejection if
  private chain-of-thought is requested or present (`chain_of_thought_present == true`).
- Added `m7-reference-output-preservation-v1`, `ReferenceOutputPreservationReport`, and
  `ReferenceOutputCatalog` in `src/agent/reference_output.rs`, preserving complete 7-dilemma
  diagnostic reference suites across semantic profiles and model/prompt protocols, asserting
  `chain_of_thought_free: true`, providing formatted Markdown export, and enforcing canonical
  dilemma domain ordering.
- Added canonical baseline reference suites for `cautious_v1`, `risk_taking_v1`, and
  `yielding_v1` under both reference diagnostic and alternative diagnostic prompt protocols.

### Known limits

- Live model provider APIs, online recalibration triggers, and network adapters remain explicitly deferred.

## 0.1.180 — 2026-08-12

### Added

- Added `m7-parameter-identifiability-v1`, `ParameterIdentifiabilityReport`, `TraitIdentifiabilityEntry`,
  `SemanticTraitDimension`, and `ParameterIdentifiabilityStatus` in `src/agent/uncertainty.rs`,
  evaluating empirical sensitivity and confounding risk across four discrete semantic dimensions
  (`RiskTolerance`, `Deference`, `Focus`, `CommunicationClarity`) with basis-point thresholds
  (identifiable $\ge 1,500$ bp, weak $\ge 500$ bp, max confounding risk $3,000$ bp).
- Added `m7-semantic-label-stability-v1`, `SemanticLabelStabilityReport`, `SemanticLabelStabilityEntry`,
  and `SemanticLabelStabilityStatus` in `src/agent/uncertainty.rs`, evaluating cross-model Total
  Variation Distance (TVD) and modal agreement across model/prompt variations with explicit stability
  thresholds (stable $\le 1,000$ bp, sensitive $\le 3,000$ bp).
- Added `m7-calibration-uncertainty-v1` and `CalibrationUncertaintyReport` in `src/agent/uncertainty.rs`,
  integrating parameter identifiability and semantic label stability into a unified qualification report
  with overall uncertainty scoring, unidentifiable parameter / unstable label presence flags, Markdown
  export, and the canonical calibration limit disclaimer stating that AI behavior serves solely as a
  reference policy distribution, not human ground truth.
- Added canonical identifiability, stability, and calibration uncertainty reports for reference profiles
  (`cautious_uncertainty_v1`, `risk_taking_uncertainty_v1`, `yielding_uncertainty_v1`).

### Known limits

- This contract establishes discrete mathematical parameter identifiability and semantic label stability
  reporting for calibration uncertainty; private chain-of-thought preservation, recalibration triggers,
  and live model provider integration remain open.

## 0.1.179 — 2026-08-12

### Added

- Added `m7-multi-model-comparison-v1`, `MultiModelComparisonReport`, `DilemmaModelComparisonEntry`,
  and `ModelFamilyAlignmentStatus` in `src/agent/multi_model.rs`, evaluating Total Variation Distance
  (TVD) deltas across action and communication distributions, parametric policy weight shifts, modal
  choice agreement (0..=7), and categorical alignment status (`aligned`, `shifted`, `divergent`)
  between reference and alternative model/prompting protocols across diagnostic dilemmas.
- Added canonical alternative diagnostic empirical distribution baselines (`cautious_alt_v1`,
  `risk_taking_alt_v1`, `yielding_alt_v1`) in `src/agent/empirical.rs`.
- Added canonical baseline multi-model comparison reports (`cautious_comparison_v1`,
  `risk_taking_comparison_v1`, `yielding_comparison_v1`) and formatted Markdown export.

### Known limits

- This contract establishes discrete mathematical multi-model and prompting family comparison
  for calibration; unidentifiable parameters, private chain-of-thought preservation, recalibration
  triggers, and live model provider integration remain open.

## 0.1.178 — 2026-08-12


### Added

- Added `m7-held-out-scenario-v1`, `HeldOutScenarioDefinition`, and `HeldOutScenarioCatalog`,
  providing canonical held-out scenario test suites for reference semantic profiles
  (`cautious_v1`, `risk_taking_v1`, `yielding_v1`) across all seven diagnostic dilemma domains.
- Added `m7-held-out-scenario-evaluation-v1` and `HeldOutScenarioEvaluationReport`, evaluating Total
  Variation Distance (TVD) loss between predicted parametric policy weights and held-out distributions,
  alongside modal prediction match and accuracy in exact basis points.
- Added `m7-counterfactual-perturbation-v1`, `CounterfactualPerturbationDefinition`, and
  `CounterfactualPerturbationCatalog`, defining canonical perturbation test cases for threat escalation,
  allied retreat calls, severe health attrition, and favorable openings.
- Added `m7-counterfactual-sensitivity-v1` and `CounterfactualSensitivityReport`, assessing directional
  coherence of parametric policy shifts under perturbations.
- Added `m7-calibration-held-out-v1` and `CalibrationHeldOutReport`, integrating held-out scenario
  generalization and counterfactual sensitivity into a deterministic qualification gate with Markdown export.

### Known limits

- This contract establishes bounded mathematical held-out scenario evaluation and counterfactual
  sensitivity testing for calibration; multi-model comparisons, parameter unidentifiability reports,
  and live model provider integration remain open.

## 0.1.177 — 2026-08-12

### Added

- Added `m7-parametric-policy-v1`, `ParametricPolicyDefinition`,
  `ParametricActionWeights`, `ParametricCommunicationWeights`, and
  `ParametricPolicyFitter`, providing bounded parametric policy parameter models
  and regularized closed-form estimation from empirical distribution reports:
  - `ParametricActionWeights` and `ParametricCommunicationWeights` for choice-level
    parameter weights with exact integer basis-point conservation ($\sum w_i = 10,000$ bp)
    and modal intent/signal prediction.
  - `ParametricPolicyFitter` for deterministic parameter fitting with bounded
    regularization penalty $\lambda \in [0..=10,000]$ bp shrinking empirical weights
    towards neutral uniform priors.
  - `ParametricPolicyDefinition` for full profile parameter bundles across all seven
    diagnostic dilemmas with fit loss tracking and formatted Markdown reporting.
  - Canonical baseline fitted policies for `cautious_v1`, `risk_taking_v1`, and
    `yielding_v1`.

### Known limits

- This contract establishes bounded mathematical parametric policy fitting with basis-point
  regularization; held-out scenario evaluation, counterfactual perturbations, and live model
  provider integration remain open.

## 0.1.176 — 2026-08-12

### Added

- Added `m7-behavioral-measures-v1`, `m7-behavioral-distance-v1`,
  `m7-behavioral-entropy-v1`, `m7-behavioral-sensitivity-v1`,
  `m7-behavioral-consistency-v1`, and `m7-behavioral-adaptation-v1`, providing
  pure discrete integer basis-point (10,000 bp scale) metrics:
  - `BehavioralDistanceMeasure` and `BehavioralDistanceReport` for Total Variation
    Distance across action and communication distributions.
  - `BehavioralEntropyMeasure` for Gini diversity index calculation.
  - `BehavioralSensitivityMeasure` for contrasting dilemma primary share shifts.
  - `BehavioralConsistencyMeasure` for modal preference concentration.
  - `BehavioralAdaptationMeasure` for defensive adaptation in adverse dilemmas.
  - `BehavioralMeasuresReport` for unified profile-level behavioral reporting with
    formatted Markdown rendering.

### Known limits

- These are discrete metric calculators over empirical distribution estimates;
  parametric policy fitting, counterfactual perturbations, and live model provider
  integration remain open.

## 0.1.175 — 2026-08-12

### Added

- Added `m7-empirical-distribution-estimation-v1`, `m7-empirical-action-distribution-v1`,
  and `m7-empirical-communication-distribution-v1`, providing typed empirical
  action distributions (`DiagnosticChoiceActionDistribution`), communication ping signal
  distributions (`DiagnosticChoiceCommunicationDistribution`), and aggregated
  diagnostic choice distribution reports (`EmpiricalDistributionEstimateReport`) with
  deterministic integer basis-point representations (10,000 basis points) and canonical
  estimates for baseline semantic profiles (`cautious_v1`, `risk_taking_v1`, `yielding_v1`).

### Known limits

- These are declarative empirical distribution estimates and frequency projections;
  parametric model fitting, distance/entropy metric calculations, and live model provider
  integration remain open.

## 0.1.174 — 2026-08-12

### Added

- Added `m7-model-prompt-protocol-v1`, providing structured model family, prompt
  template, system prompt version, sampling temperature (centipercents), top-p,
  and fail-closed chain-of-thought-free validation (`ModelPromptProtocolDefinition`)
  alongside a registry catalog (`ModelPromptProtocolCatalog`) for canonical protocols
  (`model-prompt-reference-standard-v1`, `model-prompt-reference-diagnostic-v1`,
  `model-prompt-alternative-diagnostic-v1`).
- Added `m7-repeated-sampling-protocol-v1`, providing bounded repeated empirical
  sampling parameters (`RepeatedSamplingProtocolDefinition`), sample count bounds,
  seed offset schedules, retry budgets, and fail-closed validation alongside a
  registry catalog (`RepeatedSamplingProtocolCatalog`) for canonical sampling schedules
  (`sampling-standard-repeat-10-v1`, `sampling-diagnostic-repeat-30-v1`,
  `sampling-quick-check-5-v1`).

### Known limits

- These are declarative protocol definitions and parameter bounds for calibration;
  empirical distribution estimation, action frequency measurement, and parametric
  policy fitting remain open.

## 0.1.173 — 2026-08-12

### Added

- Added `m7-diagnostic-choice-catalog-v1`, providing typed diagnostic choice
  definitions across seven behavioral dilemma domains (`ContestConcede`,
  `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`, `Surprise`, and
  `ResponseToFailure`) with canonical choices (`choice-contest-concede-v1`,
  `choice-follow-reject-v1`, `choice-farm-assist-v1`, `choice-recall-timing-v1`,
  `choice-sacrifice-v1`, `choice-surprise-v1`, `choice-response-to-failure-v1`)
  and a fail-closed registry catalog (`DiagnosticChoiceCatalog`).

### Known limits

- This is a declarative diagnostic choice schema; empirical action/communication
  distribution estimation, prompt protocols, and parametric policy fitting
  remain open.

## 0.1.172 — 2026-08-11

### Added

- Added `m7-semantic-profile-vocabulary-v1`, a compact semantic profile
  vocabulary and schema covering discrete trait dimensions (`SemanticRiskTolerance`,
  `SemanticDeference`, `SemanticFocus`, and `SemanticCommunicationClarity`) and
  canonical descriptors for baseline reference profiles (`cautious-laner-semantic-v1`,
  `risk-taking-laner-semantic-v1`, and `yielding-laner-semantic-v1`) with a
  fail-closed lookup catalog (`SemanticProfileVocabulary`).

### Known limits

- This is a declarative reference vocabulary schema; diagnostic scenario choice
  batteries, empirical action/communication distribution estimation, prompt
  protocols, and parametric model fitting remain open.

## 0.1.171 — 2026-08-11

### Added

- Added `m6-scripted-agent-calibrated-outlier-replay-v1`, a bounded in-process
  evidence report that calibrates outlier detection from a verified
  profile-aware comparison report against an explicit threshold magnitude (2)
  and deterministically traces the qualified outlier to a verified committed
  decision replay record.

### Known limits

- This is in-process calibrated outlier tracing evidence; runtime automated log
  production, durable external persistence, provider integration, and human
  gameplay claims remain open.

## 0.1.170 — 2026-08-10

### Added

- Added `m6-scripted-agent-scenario-causal-trace-completeness-v1`, a bounded
  report over one to sixteen caller-supplied decision replay records from a
  sampled scenario run, verifying causal-trace completeness (`AllComplete` vs
  `IncompleteTrace`).

### Known limits

- This is pure library-side sequence causal-trace completeness evidence; runtime
  automated log production, durable persistence, provider integration, and human
  gameplay claims remain open.

## 0.1.169 — 2026-08-10

### Added

- Added `m6-scripted-agent-scenario-replay-identity-v1`, a bounded report over
  one to sixteen caller-supplied decision replay records from a sampled scenario
  run, verifying deterministic replay consistency (`AllVerified` vs
  `DecisionMismatch`).

### Known limits

- This is pure library-side sequence replay verification; causal-trace
  completeness, runtime automated log production, durable persistence,
  provider integration, and human gameplay claims remain open.

## 0.1.168 — 2026-08-10

### Added

- Added `m6-actor-communication-abuse-population-v1`, a bounded actor-visible
  report over one to four repeated invalid message values validated against
  `ActorMessageDto::new`, retaining only the stable `InvalidValue` codec error.

### Known limits

- This is protocol-level codec boundary evidence only; actual exploit search,
  communication-abuse search, routing, delivery, prevalence, outcomes,
  persistence, providers, and human evidence remain open.

## 0.1.167 — 2026-08-08

### Added

- Added `m6-scripted-agent-exploit-seeking-population-v1`, a bounded
  fixed-fixture report over one to four actor-visible `Contest` selections by
  the risk-taking policy.

### Known limits

- This is selected-intent evidence only; actual exploit search,
  communication-abuse populations, prevalence, outcomes, strategy quality,
  persistence, providers, and human evidence remain open.

## 0.1.166 — 2026-08-08

### Added

- Added `m6-actor-illegal-command-population-v1`, a bounded actor-visible
  report over one to four repeated invalid commands validated through the
  host, retaining only the stable `host_validation_rejected` category.

### Known limits

- This is host-validation boundary evidence only; exploit-seeking,
  communication-abuse, prevalence, outcomes, persistence, providers, and
  human evidence remain open.

## 0.1.165 — 2026-08-08

### Added

- Added `m6-scripted-agent-degenerate-policy-population-v1`, a bounded
  caller-declared fixed population of repeated cautious `Stabilize` decisions
  over actor-visible observations.

### Known limits

- This is fixture-sized degenerate-policy evidence only; illegal-command,
  exploit-seeking, communication-abuse, broad adversarial populations,
  prevalence, outcomes, persistence, providers, and human evidence remain
  open.

## 0.1.164 — 2026-08-08

### Added

- Added `m6-scripted-agent-tally-replay-reference-v1`, which selects the first
  caller-declared replay record whose verified profile, rule, and selected
  intent match a largest-delta candidate.

### Known limits

- The reference is not representative-replay proof or scenario-wide replay;
  calibrated outlier definitions, causality, persistence, providers, and
  human evidence remain open.

## 0.1.163 — 2026-08-08

### Added

- Added `m6-scripted-agent-tally-outlier-threshold-v1`, a pure provisional
  `above_threshold`/`below_threshold`/`no_candidate` signal over verified
  signed intent-count deltas using an inclusive magnitude threshold of 2.

### Known limits

- The threshold is fixed-fixture evidence only; calibrated outlier detection,
  representative replay selection, causal attribution, persistence, providers,
  and human evidence remain open.

## 0.1.162 — 2026-08-08

### Added

- Added `m6-scripted-agent-replay-sequence-evidence-v1`, a pure bounded report
  joining one decision record's deterministic replay identity with the
  caller-declared operational start/chunk/finish sequence status.

### Known limits

- The report does not establish causal-trace completeness, runtime event
  production, scenario-wide replay identity, persistence, providers, or human
  evidence.

## 0.1.161 — 2026-08-08

### Added

- Added `m6-scripted-agent-operational-log-sequence-v1`, a pure categorical
  status over the fixed `m6-operational-start-chunk-finish-v1` lifecycle with
  optional checkpoint/resume labels.

### Known limits

- The status checks payload-free label order only; causal-trace completeness,
  replay identity, runtime production/detection, diagnostics, recovery,
  persistence, providers, and human evidence remain open.

## 0.1.160 — 2026-08-08

### Added

- Added the bounded `m6-scripted-agent-tally-outlier-candidate-v1` projection,
  selecting the first largest absolute signed intent-count delta from a
  verified profile-aware comparison under
  `m6-largest-absolute-intent-delta-v1`.

### Known limits

- The candidate is metric-side fixed-fixture evidence only; actual outlier
  detection, thresholds, representative replay selection, causal attribution,
  broader populations, persistence, providers, and human evidence remain open.

## 0.1.159 — 2026-08-08

### Added

- Added the closed `m6-scripted-agent-stress-population-v1` caller-declared
  four-case matrix with categorical boundary results and one degenerate
  selected-intent count.

### Known limits

- The matrix is deterministic boundary evidence only; actual adversarial or
  degenerate populations, exploit search, prevalence, outcomes, providers,
  persistence, and human evidence remain open.

## 0.1.158 — 2026-08-08

### Added

- Added a pure ordered 10,000-point intent-share projection for each verified
  profile-aware selected-intent tally row, with exact Markdown evidence.

### Known limits

- The projection remains fixed-fixture selected-intent evidence; broader
  population distributions, outcomes, strategic metrics, durable export,
  persistence, providers, calibration, and human evidence remain open.

## 0.1.157 — 2026-08-08

### Added

- Added a pure 10,000-point caller-declared distribution projection to the
  fixed-fixture scenario-frequency report, with stable row order and exact
  Markdown evidence.

### Known limits

- The projection summarizes explicit fixture selections only; random or
  representative sampling, broader scenario generation, population/outcome/
  strategic metrics, durable export, persistence, providers, calibration, and
  human evidence remain open.

## 0.1.156 — 2026-08-08

### Added

- Added a bounded provenance-bound codec for
  `m6-scripted-agent-matched-scenario-tally-compare-v1`, preserving fixed
  metadata and ordered profile-row count deltas while rejecting malformed or
  tampered text.

### Known limits

- The codec remains evidence transport only; durable export, arbitrary report
  pipelines, broader metrics/distributions, outcomes, persistence, providers,
  calibration, and human evidence remain open.

## 0.1.155 — 2026-08-08

### Added

- Added `m6-fixed-profile-tally-no-change-v1`, a provisional equality gate over
  the profile-aware tally comparison that checks top-level counts and every
  ordered row's five intent counts.

### Known limits

- The gate is a fixed regression signal only; broader thresholds, balance,
  build provenance, causality, outcomes, persistence, providers, calibration,
  and human evidence remain open.

## 0.1.154 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-scenario-tally-compare-v1`, a bounded
  comparison of two caller-declared verified profile-aware tally reports with
  shared-observer and ordered profile/rule checks plus signed intent deltas.

### Known limits

- The comparison is declared-baseline selected-intent evidence only; build
  provenance, causal attribution, broader population metrics/distributions,
  outcomes, persistence, providers, calibration, and human evidence remain
  open.

## 0.1.153 — 2026-08-08

### Added

- Added direct codec evidence for the three-profile fixed-fixture population
  tally, including canonical row identities/counts, verified round-trip, and
  tampered-row rejection.

### Known limits

- The codec remains bounded evidence transport; durable export, broader
  population metrics/distributions, outcomes, persistence, providers,
  calibration, and human evidence remain open.

## 0.1.152 — 2026-08-08

### Added

- Added focused profile-aware population tally evidence over the ordered
  cautious, risk-taking, and yielding manifests, binding eight observations to
  stable rows and exact fixed-fixture counts without rerunning policy logic.

### Known limits

- The profile rows remain fixture-sized selected-intent evidence; broader
  profile-population metrics, distributions, outcomes, persistence, providers,
  calibration, and human evidence remain open.

## 0.1.151 — 2026-08-08

### Added

- Added direct population-to-tally composition for the bounded
  `m6-scripted-agent-fixture-population-v1` contract. It reuses verified
  actor-visible selected-intent evidence without rerunning policy evaluation.

### Known limits

- This remains fixture-sized selected-intent evidence; broader population
  metrics, outcomes, random/distributional sampling, persistence, providers,
  and human evidence remain open.

## 0.1.150 — 2026-08-08

### Added

- Added ordered caller-declared composition to
  `m6-scripted-agent-fixture-population-v1`. Closed fixture IDs remain bounded
  to four entries, derive checked sequential observation pairs from one starting
  ID, and feed the existing frequency and matched-sample evidence paths.

### Known limits

- The composition is explicit fixed-fixture input, not random or representative
  population sampling; broader distributions, outcomes, metrics, persistence,
  providers, and human evidence remain open.

## 0.1.149 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-population-v1`, a deterministic fixed-fixture
  population generator capped at four alternating safe and RiverSide-threat
  entries derived from a caller-supplied starting observation ID. It composes
  the existing actor-visible matched-sample validation path.

### Known limits

- Broader/random population generation, distributional sampling, outcome and
  strategic metrics, persistence, providers, and human-behavior evidence remain
  open.

## 0.1.148 — 2026-08-08

### Added

- Added a stable caller-declared segment inventory for the bounded
  operational-log namespace. The directory scan reports recognized indices
  only and does not infer rotation or crash state.

### Known limits

- Race-hard filesystem scanning, automatic rotation, crash recovery, export,
  runtime diagnostics, and durable scenario-wide pipelines remain open.

## 0.1.147 — 2026-08-08

### Added

- Added bounded caller-declared operational-log segments under distinct
  `.foi-operational-log.segment-*` paths. Segment indices are closed and
  storage-only; the existing payload-free codec and base log remain unchanged.

### Known limits

- Automatic rotation, crash recovery, external export, runtime diagnostics,
  and durable scenario-wide event-log pipelines remain open.

## 0.1.146 — 2026-08-08

### Added

- Added the bounded `m6-scripted-agent-operational-log-v1` codec and a
  distinct injected `.foi-operational-log` store namespace. Logs persist only
  ordered payload-free event IDs and remain separate from host artifacts and
  batch checkpoints.

### Known limits

- Crash recovery, rotation, external export, runtime diagnostics, and broader
  operational-log pipelines remain open.

## 0.1.145 — 2026-08-08

### Added

- Added caller-driven `checkpoint_saved` and `batch_resumed` event production
  around injected checkpoint save/load adapters. Events are appended only after
  successful bounded storage operations, with one-slot preflight and no event
  mutation on storage, decode, or capacity failure.

### Known limits

- Automatic runtime failure detection, diagnostics, event-log persistence,
  tracing/transport, scheduling, decision/result attachment, and richer replay
  remain open.

## 0.1.144 — 2026-08-08

### Added

- Added caller-driven lifecycle production around one deterministic in-process
  batch: `batch_started`, `chunk_completed`, and `batch_finished` are appended
  only after batch validation and capacity preflight, preserving decision parity
  and leaving failed calls non-mutating.

### Known limits

- Checkpoint/resume event production, runtime failure detection, diagnostics,
  tracing/transport, persistence, scheduling, and result attachment remain
  open.

## 0.1.143 — 2026-08-08

### Added

- Added `m6-scripted-agent-operational-event-v1`, a bounded in-memory
  non-authoritative event vocabulary and 16-entry log container kept separate
  from committed simulation history and evidence reports.

### Known limits

- Runtime log production, tracing/transport, durations, diagnostics,
  persistence, scheduling, decision/result attachment, and broader experiment
  evidence remain open.

## 0.1.142 — 2026-08-08

### Added

- Added `m6-scripted-agent-build-id-v1` labels to verified fixed-fixture
  comparisons, preserving distinct caller-declared baseline and candidate IDs
  without claiming independent build provenance or causal attribution.

### Known limits

- Source/package verification, causal attribution, durable export, population
  sampling, distributional/outcome/strategic metrics, providers, calibration,
  and human evidence remain open.

## 0.1.141 — 2026-08-08

### Added

- Added `m6-scripted-agent-run-disposition-v1`, a bounded caller-declared
  envelope for `completed`, `crashed`, `timed_out`, `missing_branch`, and
  `inconclusive` run statuses with no process diagnostics or raw failure detail.

### Known limits

- Automatic crash/timeout detection, process diagnostics, decision/result
  attachment, durable export, independent build provenance, causal attribution,
  population sampling, provider execution, outcome metrics, and human evidence
  remain open.

## 0.1.140 — 2026-08-08

### Added

- Added `m6-fixed-frequency-no-change-v1`, a provisional equality gate over
  declared fixed-fixture frequency comparisons with written deterministic
  baseline-mismatch rationale; build provenance and causal attribution remain
  open.

### Known limits

- Broader threshold rationale, independent build provenance, causal attribution,
  durable export, arbitrary report construction, population generation,
  random/distributional sampling, outcomes, strategic metrics, persistence,
  providers, calibration, and human evidence remain open.

## 0.1.139 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-frequency-compare-v1`, a bounded comparison
  of two caller-declared verified frequency reports with stable row order and
  signed candidate-minus-baseline deltas; independent build provenance and
  causal attribution remain open.

### Known limits

- Independent build provenance, causal attribution, durable export, arbitrary
  report construction, population generation, random/distributional sampling,
  outcomes, strategic metrics, persistence, providers, calibration, and human
  evidence remain open.

## 0.1.138 — 2026-08-08

### Added

- Added a concise pure Markdown evidence projection for the verified
  `m6-scripted-agent-fixture-frequency-v1` report, preserving schema, bounded
  selection count, and stable catalog rows without durable export.

### Known limits

- Durable export, arbitrary report construction, population generation,
  random/distributional sampling, outcomes, strategic metrics, persistence,
  providers, calibration, and human evidence remain open.

## 0.1.137 — 2026-08-08

### Added

- Added a 4096-byte closed line codec for the verified
  `m6-scripted-agent-fixture-frequency-v1` report; decoding is accepted only
  when it matches an already verified report and does not create a durable
  export pipeline.

### Known limits

- Durable codec export, arbitrary report construction, population generation,
  random/distributional sampling, outcomes, strategic metrics, persistence,
  providers, calibration, and human evidence remain open.

## 0.1.136 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-frequency-v1`, a bounded stable-order report
  over explicit safe/threat fixture selections. It counts repeated choices from
  validated input without rerunning policies or claiming a generated
  population, general distribution, outcomes, persistence, or providers.

### Known limits

- Population generation, random/distributional sampling, outcome and strategic
  metrics, persistence, providers, calibration, and human evidence remain open.

## 0.1.135 — 2026-08-08

### Added

- Added `m6-scripted-agent-fixture-scenarios-v1`, a closed catalog and
  deterministic selector for the safe and RiverSide-threat fixture variants.
  It binds caller-supplied observation IDs, preserves ordered repeated
  selections, and composes actor-visible matched samples without adding
  population, distributional, transition, history, persistence, or provider
  authority.

### Known limits

- Broad population generation, random/distributional sampling, outcome and
  strategic metrics, persistence, providers, calibration, and human evidence
  remain open.

## 0.1.134 — 2026-08-08

### Added

- Added a bounded line-oriented codec for
  `m6-scripted-agent-matched-scenario-tally-v1`, preserving ordered
  actor-safe rows and rejecting malformed, unknown, duplicate, missing,
  wrong-rule, count-mismatch, extra-line, and oversized input.
- Added canonical round-trip and malformed-input evidence without adding
  durable export, policy execution, population, outcome, or provider paths.

### Known limits

- Durable report export/pipelines, population/distributional sampling, outcome
  and strategic metrics, persistence, providers, and calibration remain open.

## 0.1.133 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-scenario-tally-v1`, a bounded selected-intent
  aggregation over verified caller-supplied sample sets with shared observer,
  pair/observation counts, and ordered profile/rule rows.
- Added exact fixture tally and repeated-equality evidence without rerunning
  policy evaluation or adding population, outcome, persistence, or provider
  authority.

### Known limits

- Population/distributional sampling, outcome and strategic metrics, scenario
  generation, persistence, providers, and calibration remain open.

## 0.1.132 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-scenarios-v1`, a bounded composition of one
  to four caller-supplied matched observation pairs with globally distinct IDs,
  stable pair/observation/manifest order, and no scenario-generation authority.
- Added focused repeated-equality, ordering, mixed-actor, duplicate-ID, empty,
  and capacity-bound evidence while reusing the existing matched-sample and
  seeded batch contracts.

### Known limits

- Scenario generation/selection, population and distributional sampling,
  outcomes, metrics, persistence, providers, and calibration remain open.

## 0.1.131 — 2026-08-08

### Added

- Added `m6-experiment-version-catalog-v1`, a fixed metadata catalog for the
  current ruleset, two-window scenario, scripted policy schema, and three
  profile identities. Prompt, model, tool-schema, and extractor versions are
  explicitly marked `not-applicable` for this in-process deterministic slice.
- Added focused literal-identity and repeated-construction evidence without
  changing manifest, batch, matched-sample, or persistence behavior.

### Known limits

- Provider/model integration, prompt and extractor versioning, population and
  matched-scenario sampling, metrics, persistence, and calibration remain open.

## 0.1.130 — 2026-08-08

### Added

- Added `m6-scripted-agent-matched-sample-v1`, a bounded in-process sample over
  exactly two same-actor, distinct-observation-ID receipts and an ordered list
  of explicit seeded manifests. Rows retain only profile/rule/seed labels and
  the selected intents for each observation.
- Added focused sensitivity, repeatability, ordering, and input-bound tests for
  matched observations while reusing the existing deterministic batch runner.

### Known limits

- Population generation and distribution sampling, outcome/metric reports,
  persistence, providers, and calibration remain open.

## 0.1.129 — 2026-08-08

### Added

- Added `m6-scripted-agent-batch-run-v1`, a bounded checkpoint codec that binds
  an ordered manifest batch and actor-visible observation to a resumable cursor.
- Added `ScriptedAgentBatchRunner::run_next` and the injected
  `ScriptedAgentBatchRunStore` for deterministic chunk resume without storing
  decisions or acquiring simulation authority.
- Added focused codec, mismatch, completion, and save/load cursor evidence.

### Known limits

- Decision/result persistence, crash diagnostics, populations, sampling,
  metrics, report export, providers, and calibration remain open.

## 0.1.128 — 2026-08-08

### Added

- Added `ScriptedAgentBatchRunner` for deterministic in-process evaluation of
  one actor-visible observation against an ordered list of up to 16 explicit
  experiment manifests.
- Added focused order/reproducibility, seed-retention, empty-batch, and
  capacity-bound evidence.

### Known limits

- Resumable run directories, persistence, populations, sampling, metrics,
  report export, providers, and calibration remain open.

## 0.1.127 — 2026-08-08

### Added

- Added `m6-experiment-manifest-v1`, a bounded eight-line reproducibility
  manifest for the versioned two-window fixture, all three scripted profiles,
  exact policy rules, and explicit seed/stream/draw identity.
- Added focused manifest round-trip and malformed-input coverage.

### Known limits

- The manifest is declarative library metadata; batch execution, resumable
  storage, population sampling, metrics, providers, and calibration remain open.

## 0.1.126 — 2026-08-08

### Added

- Added the versioned `m5-actor-message-v1` recipient-scoped envelope with
  bounded actor-authored text, sender/recipient IDs, observation binding, and
  an exact line-oriented codec.
- Added focused protocol coverage for canonical encoding, closed numeric and
  text bounds, malformed fields, and self-delivery rejection.

### Known limits

- The envelope is protocol metadata only; authentication, routing, delivery,
  ordering, retries, trust, transport, and communication-quality evidence
  remain open.

## 0.1.125 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_debrief_from_run`, which loads a validated
  complete injected-store artifact, verifies its replay, and returns the
  existing categorical `m5-actor-debrief-v1` summary without mutating the
  receiving host.
- Added focused fresh-host evidence for complete-run summary retrieval,
  incomplete-run gating, tampered-artifact rejection, and closed-session
  redaction.

### Known limits

- This remains injected in-process file-store evidence; locking, portability,
  crash recovery, scenario-wide durable replay, and detailed causal review
  remain open.

## 0.1.124 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_draft`, an observation-bound readback of the
  requesting actor's actor-protocol-staged message, plan, and contingency
  values using the existing bounded draft DTO; legacy CLI draft text remains
  on its existing path.
- Added focused evidence for stable field ordering, exact binding, unchanged
  host state, and committed/complete/closed lifecycle rejection.

### Known limits

- This is actor-owned in-process metadata readback only; recipient delivery,
  simultaneous drafts, transport, persistence, reconnect, provider behavior,
  and richer plan semantics remain open.

## 0.1.123 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_replay_debrief_records_from_run`, which loads
  a validated complete injected-store artifact, verifies its replay, and
  returns the existing categorical debrief records without mutating the
  receiving host.
- Added focused fresh-host evidence for complete-run debrief retrieval,
  incomplete-run gating, tampered-artifact rejection, and closed-session
  redaction.

### Known limits

- This remains injected in-process file-store evidence; locking, portability,
  crash recovery, scenario-wide durable replay, and detailed causal review
  remain open.

## 0.1.122 — 2026-08-08

### Added

- Added `CliScenarioHost::actor_replay_records_from_run`, which loads a
  validated injected-store artifact, verifies its replay, and returns the
  existing categorical actor replay records without mutating the current host.
- Added focused fresh-host persistence/replay evidence for successful records,
  tampered artifacts, and closed-session redaction.

### Known limits

- This is injected in-process file-store evidence only; locking, portability,
  crash recovery, scenario-wide replay, and durable causal records remain open.

## 0.1.121 — 2026-08-08

### Added

- Added `m5-actor-draft-clear-v1` and
  `m5-actor-draft-clear-receipt-v1`, a bounded observation-bound clear command
  and payload-free acknowledgement reporting pre-clear field presence.
- Added focused codec and host regressions for exact fields, malformed input,
  idempotent empty clears, authorization/freshness gating, payload redaction,
  and unchanged observation/history.

### Known limits

- Clearing does not deliver metadata or define communication, transport,
  persistence, reconnect, simultaneous-draft, or free-form plan semantics.

## 0.1.120 — 2026-08-08

### Added

- Added `m5-actor-draft-status-v1`, a bounded active-draft projection that
  reports only observer/observation binding and aggregate message, plan, and
  contingency presence bits without echoing payloads.
- Added focused codec and host regressions for exact fields, malformed input,
  active-window gating, payload redaction, and unchanged history/observation.

### Known limits

- Draft status does not deliver metadata or define communication, transport,
  persistence, reconnect, simultaneous-draft, or free-form plan semantics.

## 0.1.119 — 2026-08-08

### Added

- Added `m5-actor-replay-debrief-record-v1`, a bounded replay-linked debrief
  record projection for the two complete fixture windows with categorical
  objective labels and committed-facts attribution.
- Added focused codec and host regressions for exact fields, malformed input,
  completion gating, replay verification, tamper/closed errors, and omission
  of causal and provenance detail.

### Known limits

- The projection remains in-process and categorical; detailed causal review,
  durable/scenario replay, transport, persistence, reconnect, and providers
  remain open.

## 0.1.118 — 2026-08-08

### Added

- Added `m5-actor-replay-record-v1`, a bounded actor-safe categorical record
  projection for at most two replay-verified fixture windows.
- Added focused codec and host regressions for exact record fields, malformed
  input rejection, successful empty/partial/complete projections, and replay
  tamper/closed-session redaction.

### Known limits

- Replay records expose only window, intent, outcome, and verified status;
  hashes, resolved inputs, traces, causal detail, persistence, and transport
  remain open.

## 0.1.117 — 2026-08-08

### Added

- Added `m5-actor-draft-commit-receipt-v1`, a bounded actor-safe acknowledgement
  reporting the committed intent and only `present`/`absent` metadata for the
  message, plan, and contingency draft fields.
- Added focused protocol and host regressions proving exact seven-line codec
  behavior, payload-free output, successful field-presence reporting, and
  unchanged draft/observation/history boundaries on failed and successful
  commits.

### Known limits

- The receipt confirms host acceptance metadata only; communication delivery,
  free-form plan semantics, transport, persistence, and simultaneous drafts
  remain open.

## 0.1.116 — 2026-08-08

### Added

- Added `m5-actor-replay-v1`, a bounded actor-visible replay-verification DTO
  and host projection carrying only verified status and record count.
- Added focused codec and host regressions for successful, closed, and tampered
  history paths without exposing records, hashes, resolved inputs, or traces.

### Known limits

- Replay records, durable/scenario replay integration, detailed causal review,
  messages, plans, contingencies, and complete MCP transport remain open.

## 0.1.115 — 2026-08-08

### Added

- Added a repository core-boundary guard that rejects async runtime/syntax,
  wall-clock, and network transport primitives from deterministic core modules.
- Added focused checker coverage for both rejection and clean-core paths.

### Known limits

- The guard verifies source ownership boundaries only; transport framing,
  async orchestration, reconnect, and a complete MCP adapter remain open.

## 0.1.114 — 2026-08-08

### Added

- Versioned the immutable actor session as `m5-actor-session-v2` with explicit
  client-requested, caller-signaled timeout, and disconnect closure reasons.
- Added bounded encoded-action acceptance that maps malformed codec input before
  actor, stale, and duplicate session checks.
- Retained `m5-actor-session-v1` as a historical identity; no v1 migration or
  decoder is provided for the current v2 session contract.

### Known limits

- Timeout is an explicit caller event rather than wall-clock scheduling;
  transport framing, reconnect, persistence, and async orchestration remain
  open.

## 0.1.113 — 2026-08-08

### Added

- Added a host parity regression comparing CLI observation and
  plan/commit/advance behavior with actor-protocol DTO projection and action
  submission on the same deterministic fixture.

### Known limits

- Parity evidence is bounded to the in-process CLI/protocol library paths;
  MCP transport parity, authentication, persistence, and provider integration
  remain open.

## 0.1.112 — 2026-08-08

### Added

- Added `m5-actor-simultaneous-window-v1`, an immutable two-actor collection
  boundary with one shared observation ID, one submission per actor, bounded
  freshness errors, and readiness only after both actions arrive.
- Kept collected intents out of public debug/readiness surfaces; no transition,
  history, replay, transport, or ordering authority is added.

### Known limits

- Host-owned simultaneous ordering/resolution, private transport delivery,
  reconnect, persistence, and broader multi-actor coordination remain open.

## 0.1.111 — 2026-08-08

### Added

- Kept authoritative lane observation/request conversion behind crate-private
  protocol adapters, with two independent compile-fail RustDoc boundaries
  proving public DTO consumers cannot call those domain conversions directly.

### Known limits

- The boundary is library/API visibility only; transport authentication,
  provider compatibility, persistence, and broader MCP integration remain open.

## 0.1.110 — 2026-08-08

### Added

- Added a closed five-entry ordinary-actor capability catalog covering the
  versioned observation, draft, draft-receipt, commit, and action tools.
- Reserved the `privileged_experiment_controller` authority label without
  advertising or implementing privileged tools.

### Known limits

- Capability metadata does not authenticate callers or grant runtime
  authority; privileged tools, transport registration, and experiment control
  remain open.

## 0.1.109 — 2026-08-08

### Added

- Added `m5-actor-transcript-v1`, a provider-neutral six-line record for
  bounded actor tool/schema identity and accepted/rejected outcomes.
- Added exact closed-catalog codec coverage without retaining payloads, raw
  errors, prompts, model metadata, transport details, or simulation state.

### Known limits

- Transcript metadata remains a pure library value; runtime logging,
  persistence, provider compatibility, transport, and replay integration remain
  open.

## 0.1.108 — 2026-08-08

### Added

- Added `m5-actor-draft-receipt-v1`, a bounded acknowledgement containing only
  the bound actor, observation, and staged-field identity after successful
  host-owned draft staging.
- Added exact receipt codec coverage and first/second-window host evidence;
  the receipt does not echo metadata or add communication, transition, or
  history authority.

### Known limits

- Draft receipts remain library-level acknowledgements; transport delivery,
  simultaneous actors, persistence/reconnect, and richer plan/communication
  semantics remain open.

## 0.1.107 — 2026-08-08

### Added

- Added `m5-actor-commit-v1` and `m5-actor-commit-result-v1` for an
  observation-bound explicit intent commit and bounded acknowledgement.
- Added host coverage proving commit clears uncommitted draft metadata without
  advancing the window, changing history, or refreshing the observation;
  staged-plan mismatches and lifecycle boundaries remain actor-safe.

### Known limits

- Commit remains a synchronous host boundary; transport delivery, simultaneous
  ordering, persistence, reconnect, and richer communication/plan semantics
  remain open.

## 0.1.106 — 2026-08-08

### Added

- Added bounded `m5-actor-debrief-v1` output for an active completed fixture,
  exposing only first/second intent, categorical outcome, objective
  dispositions, final objective, and committed-facts attribution.
- Added exact debrief codec coverage and completion/closed host projection
  checks; the current `m5-actor-error-v2` codec carries the dedicated
  `debrief_unavailable`/`await_completion` pair without exposing internal
  report details, while v1 remains the historical pre-debrief vocabulary.

### Known limits

- The debrief remains a synchronous committed-facts summary; detailed causal
  review, replay-linked records, transport, persistence, simultaneous actors,
  and broader MCP compatibility remain open.

## 0.1.105 — 2026-08-08

### Added

- Added bounded `m5-actor-action-result-v1` output for successful host actor
  submissions, exposing only fixture window and categorical outcome.
- Added exact result codec and first/second-window host projection coverage;
  errors and transition authority remain on the existing host boundary.

### Known limits

- Results remain synchronous fixture projections; detailed debrief, transport,
  persistence, simultaneous actors, and broader MCP compatibility remain open.

## 0.1.104 — 2026-08-08

### Added

- Added exact `m5-actor-error-v1` encode/decode for closed error and repair IDs,
  with bounded line count/size and no raw payload or domain detail.
- Added exhaustive closed-ID round-trip and malformed-wire coverage.

### Known limits

- Error codec repair remains advisory-only; automatic repair, transport,
  persistence, and broader MCP compatibility remain open.

## 0.1.103 — 2026-08-08

### Added

- Added the bounded `m5-actor-history-v1` DTO and host projection for record
  count plus open/complete/closed lifecycle status without hashes or snapshots.
- Added exact codec and host lifecycle coverage for open, complete, and closed
  history states.

### Known limits

- History status is a synchronous actor-safe summary; detailed history, replay,
  debrief, transport, persistence, and broader MCP compatibility remain open.

## 0.1.102 — 2026-08-08

### Added

- Added a host-owned `actor_observation` projection that returns the active
  actor-visible receipt through `m5-actor-observation-v1` without exposing
  internal lane types or mutating history; closed and complete hosts return
  actor-safe lifecycle errors.
- Added parity and non-mutation coverage across the initial and next fixture
  observations.

### Known limits

- Observation projection remains a synchronous library boundary; transport,
  simultaneous actors, persistence, and broader MCP compatibility remain open.

## 0.1.101 — 2026-08-08

### Added

- Added observation-bound host staging for bounded actor message, plan, and
  contingency metadata, preserving existing replacement and committed-boundary
  semantics without appending history.
- Added stale, wrong-actor, complete, closed, and committed-draft rejection
  coverage through actor-safe protocol errors.

### Known limits

- Metadata delivery/communication, simultaneous drafts, transport, persistence,
  and free-form plan semantics remain open.

## 0.1.100 — 2026-08-08

### Added

- Added bounded `m5-actor-draft-v1` metadata DTOs for message, plan, and
  contingency values, with observation binding, 256-byte payload caps, and
  closed plan IDs.
- Added round-trip and malformed/control/size-bound coverage without staging
  host drafts or adding communication/transition authority.

### Known limits

- Host draft staging, free-form plan semantics, transport, persistence,
  provider metadata, and broader message/coordination behavior remain open.

## 0.1.99 — 2026-08-08

### Added

- Added host-owned actor action submission for the bounded fixture: validated
  DTOs append through the existing lane/history path and close one window,
  while stale/duplicate/closed actions fail before mutation.
- Added actor-safe transition-rejection mapping for malformed execution input;
  raw transition errors and authoritative values remain private.

### Known limits

- Transport-integrated submission, reconnect, simultaneous decisions,
  privileged tools, and broader scenario/session closure remain open.

## 0.1.98 — 2026-08-08

### Added

- Added a read-only host adapter for validating actor action DTOs against the
  current actor-visible receipt and existing lane validator.
- Added actor-safe mismatch, stale-observation, closed-window, and generic
  host-validation rejection projections without exposing raw lane errors or
  mutating history.

### Known limits

- Actor action submission/window closure, finer host-legality error taxonomy,
  transport integration, retry/reconnect, and privileged tools remain open.

## 0.1.97 — 2026-08-08

### Added

- Added the versioned `m5-actor-error-v1` projection for codec and immutable
  session-freshness failures, with closed actor-safe codes and deterministic
  repair hints.
- Kept repair advisory-only: no payload rewriting, retry loop, host legality,
  transition, history, transport, or provider authority was added.

### Known limits

- Host-legality error projection, automatic repair, transport retry/framing,
  reconnect, and provider-neutral transcripts remain open.

## 0.1.96 — 2026-08-08

### Added

- Added the bounded `m5-actor-codec-v1` line-oriented codec for versioned
  observation and intent-action DTOs.
- Added fail-closed size, exact-field, duplicate/unknown/missing-field,
  closed-intent, and host-validation regressions without adding transport I/O.

### Known limits

- Codec persistence, transport integration, session wire framing, plan/message
  payloads, and provider-neutral transcripts remain open.

## 0.1.95 — 2026-08-08

### Added

- Added the immutable `m5-actor-session-v1` lifecycle for ordinary actor
  binding, current-observation freshness, duplicate-submit rejection, and
  fail-closed close behavior.
- Kept session checks separate from host legality, transition, history, and
  replay authority.

### Known limits

- Session transport, reconnect/disconnect policy, simultaneous submission,
  repair behavior, and provider-neutral transcripts remain open.

## 0.1.94 — 2026-08-08

### Added

- Added the versioned `m5-actor-protocol-v1` observation/action DTO boundary
  with closed intent IDs and bounded actor/turn/observation identity.
- Added host-bound request conversion and hidden-state/authority regressions
  without introducing MCP transport, async orchestration, or provider SDKs.

### Known limits

- Session lifecycle, plan/message DTOs, private submission, transport,
  simultaneous decisions, and provider-neutral transcripts remain open.

## 0.1.93 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-replay-v1` library record for
  re-evaluating actor-visible scripted decisions with optional seed
  provenance.
- Added expected versus declared-anomalous disposition labels and bounded
  decision-mismatch detection without making policy replay part of host
  history or durable persistence.

### Known limits

- Replay records are library-only inspection artifacts; durable persistence,
  degenerate-policy populations, broad sampling, outcomes, and human-behavior
  claims remain open.

## 0.1.92 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-random-v1` seed bundle with explicit
  policy `StreamId`/`DrawId` inputs and an opt-in `choose_with_seed` path.
- Seeded selection uses `max-score-seeded-tie-v1` only for equal top-score
  candidates; the default profile path remains stable-order deterministic.

### Known limits

- Broad random sampling, top-k/nucleus selection, experiment manifests,
  populations, outcomes, and human-behavior claims remain open.

## 0.1.91 — 2026-08-08

### Added

- Added `ScriptedAgentProfile::preferred_intent()` to expose each fixed
  baseline preference separately from the visible-threat override.

### Known limits

- Baseline preference metadata covers the three fixture profiles; richer risk,
  planning, memory, communication, and human-behavior parameters remain open.

## 0.1.90 — 2026-08-08

### Added

- Bumped the action-tally schema to
  `m4-scripted-agent-action-tally-v2` when binding the two-observation tally to
  its actor-visible observation IDs,
  exposing both IDs and rejecting duplicate IDs before policy evaluation.

### Known limits

- Observation-ID binding covers the fixed two-observation fixture only; broader
  replay provenance, scenario sampling, populations, and outcomes remain open.

## 0.1.89 — 2026-08-08

### Added

- Bound the `max-score-stable-order-v1` selection rule with exact rule-ID
  assertions for all three profiles and an equal-score regression proving
  first-advertised tie behavior.

### Known limits

- Selection remains deterministic top-1 fixture behavior; top-k/nucleus
  sampling, randomness, populations, outcomes, and human realism remain open.

## 0.1.88 — 2026-08-08

### Added

- Added candidate-breadth evidence proving the scripted policy exposes four
  safe candidates and five candidates when the actor-visible RiverSide threat
  response is advertised, with unique actor-valid intents and unchanged stable
  selection.

### Known limits

- Candidate breadth is fixture-sized generation evidence, not strategic
  diversity, population variation, randomness, outcomes, or human behavior.

## 0.1.87 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-action-tally-v1` actor-safe report
  over the safe and RiverSide fixture observations, with bounded profile/rule
  IDs and selected-intent counts.
- Rejected mixed-observer tally inputs and added legality checks for all six
  underlying profile/observation requests.

### Known limits

- The tally covers exactly two library observations; population distributions,
  outcomes, strategic quality, and human realism remain deferred.

## 0.1.86 — 2026-08-08

### Added

- Added the versioned `threat-first-pressure-aware-fixed-score-v1` Anchor
  evaluation rule, using only bounded actor-visible wave pressure to adjust
  the `Stabilize` score.
- Added low/high-pressure monotonic score and host-validation evidence while
  preserving candidate generation, stable selection, and the other profiles.

### Known limits

- Pressure sensitivity covers two library fixture observations; memory,
  communication, randomness, populations, outcomes, strategic quality, and
  human realism remain deferred.

## 0.1.85 — 2026-08-08

### Added

- Added transparent `ScriptedAgentRole` metadata with versioned `anchor-v1`,
  `duelist-v1`, and `pacer-v1` IDs bound to the three fixed profiles.
- Added literal role-binding assertions while keeping policy roles distinct from
  the lane scenario roster and human-behavior claims.

### Known limits

- Policy-role labels are metadata over one fixture catalog; scenario role
  behavior, broader populations, outcomes, strategic quality, and human realism
  remain deferred.

## 0.1.84 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-metrics-v1` actor-safe comparison
  report for the three profiles, exposing bounded profile/rule IDs, selected
  intent/score, candidate count, and observation identity.
- Added reproducibility and bounded-row tests without exposing state, hashes,
  execution inputs, or changing host authority.

### Known limits

- The report is a library metric schema over one fixture observation; broad
  action distributions, outcome metrics, population comparisons, strategic
  quality, and human realism remain deferred.

## 0.1.83 — 2026-08-08

### Added

- Added visible-threat profile-sensitivity evidence over safe and RiverSide
  observations, showing cautious response changes while risk-taking and
  yielding fixed preferences remain stable.
- Added host-validation assertions for all six profile/observation requests.

### Known limits

- Sensitivity covers two library fixture observations only; adversarial edge
  matrices, scenario outcomes, strategic quality, and human realism remain
  deferred.

## 0.1.82 — 2026-08-08

### Added

- Added the versioned `yielding-laner-v1` profile with a transparent
  `yield-first-fixed-score-v1` evaluation rule.
- Extended the matched-input catalog regression to three profiles with stable
  candidate sequences, distinct legal intents, profile rule IDs, and repeated
  decisions.

### Known limits

- The catalog remains library-only and fixture-sized; role populations, memory,
  communication, randomness, scenario metrics, strategic quality, and external
  agent adapters remain deferred.

## 0.1.81 — 2026-08-08

### Added

- Added a bounded `ScriptedAgentEvaluationError::UnavailableIntent` result for
  public candidate evaluation outside an actor-visible advertised set.
- Added focused rejection evidence while keeping internal selection limited to
  generated candidates and leaving host legality/transition authority intact.

### Known limits

- Evaluation errors are policy-boundary plumbing only; they do not provide
  scenario outcomes, memory, communication, randomness, population metrics,
  strategic-quality, human-realism, or external-agent evidence.

## 0.1.80 — 2026-08-08

### Added

- Added the versioned `risk-taking-laner-v1` profile beside the cautious
  scripted baseline, sharing actor-visible candidate generation and host
  validation while using a distinct fixed contest-first score rule.
- Added a matched-input regression proving the two profiles choose distinct
  legal intents from the same observation without changing transition or
  history authority.

### Known limits

- The comparison is library-only and covers two profiles on one fixture input;
  role populations, memory, communication, randomness, metrics, strategic
  quality, and external agent adapters remain deferred.

## 0.1.79 — 2026-08-08

### Added

- Added the versioned `m4-scripted-agent-v1` policy boundary with the
  actor-visible `cautious-laner-v1` deterministic baseline.
- Added bounded candidate generation, fixed candidate evaluation, stable
  selection, host-validatable requests, and reproducibility tests without
  introducing agent-owned legality or transition behavior.

### Known limits

- This is one library-only scripted profile; broader agent populations, role
  heuristics, memory, communication, randomness, metrics, and external agent
  adapters remain deferred.

## 0.1.78 — 2026-08-08

### Added

- Added a clean-checkout binary transcript regression that exercises the
  documented two-window commands through replay, debrief, and quit.
- Added actor-safe output/status assertions distinguishing executable evidence
  from library-only host and store tests.

### Known limits

- The transcript covers only the bounded deterministic fixture; complete
  playable behavior, multiple scenarios, branch graphs, and human accessibility
  remain deferred.

## 0.1.77 — 2026-08-08

### Added

- Added standalone `--version` and `-V` process aliases that report the
  package-derived `fog-of-intent <version>` line before host construction.
- Added bounded parser/help and binary regressions for identical aliases,
  exact output, success status, and combined-argument failure.

### Known limits

- Version reporting is process metadata only; schema negotiation, migrations,
  update checks, and version-dependent simulation behavior remain deferred.

## 0.1.76 — 2026-08-08

### Added

- Added machine-checked representative CLI text-structure evidence for stable
  lowercase labels, newline-delimited command-loop lines, and plain text without
  ANSI/control characters.
- Kept control-character sanitization and actor-valid projection boundaries in
  the pure renderer while documenting the remaining human accessibility gap.

### Known limits

- Text-shape checks do not establish keyboard-only usability, focus behavior,
  screen-reader semantics, human accessibility, or complete client behavior.

## 0.1.75 — 2026-08-08

### Added

- Added explicit process-edge selection for the versioned
  `m3-two-window-fixture-v1` executable fixture.
- Added fail-closed missing, empty, option-shaped, duplicate, and unsupported
  scenario-argument handling with bounded path-free errors and process status.
- Added parser and binary regressions for explicit/default selection, option
  composition, help output, and the existing two-process store smoke path.

### Known limits

- Only the existing deterministic two-window fixture is selectable. Scenario
  catalogs, external scenario data, arbitrary configuration, complete playable
  behavior, and accessibility evidence remain deferred.

## 0.1.74 — 2026-08-08

### Added

- Added bounded host execution for the existing `branch` grammar at the
  supported `first` decision point using a staged alternate plan and
  matched-parent execution.
- Added actor-safe branch comparison text and tests proving parent history,
  replay, and saved artifacts remain unchanged.
- Added the M3 host-branch design, QA, handoff, and lesson records.

### Known limits

- Regenerated execution, branch IDs/graphs, branch persistence, multi-window
  branching, scenario selection, and keyboard/screen-reader evidence remain
  open.

## 0.1.73 — 2026-08-08

### Added

- Added bounded executable argument parsing with `--run-dir <path>` and
  `--help`; the no-argument binary remains an in-memory fixture loop.
- Wired the explicit run directory to the injected `CliRunStore` and added a
  two-process save/load smoke test plus path-free argument failure evidence.
- Updated the M3 canonical and workspace documents for the executable boundary.

### Known limits

- The binary still has no default storage directory, scenario selection,
  branch execution, locking, fsync/crash recovery, race-hard symlink
  protection, or keyboard/screen-reader evidence.

## 0.1.72 — 2026-08-08

### Added

- Added the injected dependency-free `CliRunStore` for bounded host artifacts.
  It validates run IDs, bounds reads/writes, and replaces final files through a
  same-directory temporary write plus rename.
- Added fresh-host file round-trip, replacement, missing/malformed/oversized,
  and bounded host-error evidence while retaining an in-memory default fixture.

### Known limits

- The binary does not yet select a run directory; race-hard symlink protection,
  locking, fsync/crash recovery, scenario selection, branch execution, and
  accessibility evidence remain open.

## 0.1.71 — 2026-08-08

### Added

- Added the versioned `m3-cli-host-artifact-v1` pure text artifact for bounded
  host save/load. It records validated run IDs, replay identity, committed
  intents, lane-record identity, and state hashes, then restores only after
  deterministic replay validation with bounded decoding.

### Known limits

- Artifacts remain in-process; durable file storage, scenario selection, branch
  execution, and keyboard/screen-reader evidence remain open.

## 0.1.70 — 2026-08-08

### Added

- Added the versioned `m3-cli-command-loop-v1` line-oriented stdin/stdout edge
  adapter and wired the binary to the deterministic two-window fixture host.
- The loop renders plain text results and bounded errors, continues after
  malformed commands, exits cleanly on `quit` or end-of-input, and propagates
  fatal stdin/stdout errors to a non-success process status.

### Known limits

- The binary remains a deterministic fixture loop without scenario selection,
  persistent storage, branch execution, prompt styling, or human
  keyboard/screen-reader evidence.

## 0.1.69 — 2026-08-08

### Added

- Added the versioned `m3-cli-terminal-text-v1` pure projection for every
  actor-valid host output and bounded host error. It emits stable labeled
  plain text, sanitizes echoed control characters, and performs no terminal
  I/O or hidden-state lookup.

### Known limits

- The projection is library-only; a command loop, terminal integration,
  persistent backend, keyboard/focus inspection, and screen-reader evidence
  remain open.

## 0.1.68 — 2026-08-08

### Added

- Added the dependency-free `m3-cli-host-v1` synchronous host fixture. It
  maps CLI grammar commands to an explicit-input two-window scenario and
  verifies actor-visible observe/history, pre-commit staging and undo,
  in-memory save/load, replay, and debrief projections.

### Known limits

- The host is library-only and deterministic in memory; it does not provide a
  terminal renderer, persistent backend, branch execution, keyboard-only flow,
  or screen-reader evidence.

## 0.1.67 — 2026-08-08

### Added

- Added grammar-level transcript acceptance tests covering a representative
  read/write/process/session sequence and common parser/request errors.

### Known limits

- These tests do not claim a host-backed complete run, save/resume, replay,
  debrief, terminal output, or human keyboard/screen-reader evidence.

## 0.1.63 — 2026-08-08

### Added

- Added repository-wide two-space formatting policy, hard-tab rejection, and
  dependency-free checker tests for Rust, Python, and authored text.
- Added the verified contributor lessons ledger in `LESSONS.md`.

### Changed

- Converted textual lane test inclusions into formatter-visible test modules
  without changing production contracts or test behavior.
- Replaced unchecked numeric casts and data-dependent transition assertions with
  checked bounded operations and typed error paths; Clippy now denies
  `as_conversions`.

### Known limits

- Markdown syntax-sensitive indentation and versioned compatibility fixtures
  remain formatting-policy exceptions; hard tabs remain forbidden.

## 0.1.50 — 2026-08-06

### Changed

- Audited the current M2 implementation and reconciled README, specification,
  architecture, and repository-currentness claims with the verified internal
  kernel and replay fixtures.
- The repository checker now rejects a stale README package version.

### Known limits

- The M2 lane contract remains an internal diagnostic fixture; the complete
  scenario, CLI, MCP, persistence, and human-evidence work remain deferred.

## 0.1.51 — 2026-08-06

### Changed

- Replaced the experimental M2 v1 resource surface with the versioned M2 v2
  contract: retained resources use `LaneResources` and `LaneResourceInputs`,
  lifecycle uses `LaneStatus`, and delayed effects require non-zero `LaneDelay`.
- Retired bounty, level, minion kills, shield, ward, and the sixteen
  experimental consumable slices from state, observations, execution inputs,
  events/effects, debriefs, errors, hashes, and replay identities.
- Versioned current M2 ruleset, observations, replay/profile/strategy fixtures,
  and base transition-record identities. M2 v1 has no migration because it was
  never an external or supported artifact; M1 fixtures and codec remain exact.
- Bound delayed-effect execution inputs into the v2 lane record identity and
  made objective verification reject retired record IDs.
- Updated canonical project-state documents to distinguish current v2 evidence
  from retired v1 history without marking the complete M2 exit criteria done.

## 0.1.62 — 2026-08-07

### Added

- Added typed top-level process commands for `play`, `replay`, `branch`, `experiment`,
  `export`, `validate`, `mcp`, `help`, and `version`.
- Added `CliInteractionMode` (`Guided` default and `Expert`) and `CliVerbosity`
  (`Concise`, `Standard` default, `Explanatory`, `Research`) policies.
- Added `CliPrivilegeLevel` (`Unprivileged` and `Privileged`), enforcing that research
  verbosity and unredacted exports fail closed under standard unprivileged contexts.
- Added pure, dependency-free parsing and validation for top-level arguments and flags.
- Added `CliTopLevelHelpCatalog` and focused top-level command, mode, verbosity, privilege,
  and catalog unit tests.

## 0.1.61 — 2026-08-07

### Added

- Added typed borrowed adapter session requests for `save`, `load`, `undo`, and
  `quit` verbs with run identifier and payload-free boundaries.
- Added focused session-request mapping tests; persistence, save/load execution,
  uncommitted choice editing, and session lifecycle remain outside the adapter.
  Help metadata now identifies these four verbs as session-adapter requests.

## 0.1.60 — 2026-08-07

### Added

- Added typed borrowed adapter process requests for `review`, `debrief`,
  `replay`, and `branch` verbs with optional run and point identifier boundaries.
- Added focused process-request mapping tests; host execution, history inspection,
  and branch derivation remain outside the adapter. Help metadata now identifies
  these four verbs as process-adapter requests.

## 0.1.59 — 2026-08-06

### Added

- Added typed borrowed adapter write requests for `message`, `plan`,
  `contingency`, `commit`, and `advance`, with distinct payload and commitment
  boundaries; empty direct-construction payloads fail closed.
- Added focused write-request mapping tests; domain intent mapping, legality,
  execution, and history mutation remain outside the adapter. Help metadata now
  identifies these five verbs as write-adapter requests.

## 0.1.58 — 2026-08-06

### Added

- Added typed read-only adapter requests for `observe`, bounded `inspect`, and
  contextual `help`, with a static catalog of stable grammar verbs.
- Added actor-visible inspect-target restrictions and read-mapping tests without
  terminal I/O, hidden-state access, or domain mutation.

## 0.1.57 — 2026-08-06

### Added

- Added the dependency-free typed M3 CLI grammar for stable help, observe,
  inspect, planning, review, replay, branch, save/load, undo, and quit verbs.
- Added bounded parse errors and borrowed-payload transcript tests; terminal
  I/O, rendering, and domain authorization remain outside the adapter.

## 0.1.56 — 2026-08-06

### Added

- Added report-derived `LaneBelief<T>` values for unknown, observed, and
  last-known information with an explicit no-decay update rule.
- Added focused opponent/threat report, malformed-pair, and redaction-boundary
  tests without changing observation schemas, authoritative state, or replay
  identities.

## 0.1.55 — 2026-08-06

### Added

- Added typed deterministic `LaneAdvanceCondition` and
  `LaneAdvanceDecision` values for commit-required and no-legal-intent
  evaluation; current one- and two-beat windows remain commit-required.
- Added focused condition-mapping tests without changing authoritative state,
  replay identities, or M1 behavior.

## 0.1.54 — 2026-08-06

### Added

- Retained each delayed lane effect's originating execution trace through
  queueing, ticking, state hashing, branch/history identity, replay,
  resolution event/effect attribution, lane debriefs, and final debrief
  reports.
- Versioned the current internal M2 ruleset, observation, replay, profile,
  strategy, scenario, debrief, and branch identities from v2 to v3; unsupported
  older M2 inputs fail closed while M1 fixtures remain unchanged.
- Added focused origin-trace retention, hash/identity tamper, delayed-resolution
  attribution, and debrief projection tests.

## 0.1.53 — 2026-08-06

### Added

- Added the fixed M2 `LaneActorRoster` and `LaneActorRole` contract for one
  human laner, one opposing laner, one allied autonomous actor, and one
  abstract opposing jungle threat.
- Exposed role identity through player and allied observations while retaining
  hidden opponent/jungle redaction and excluding fixed roster metadata from
  authoritative lane hashes.
- Added focused actor-roster completeness and information-boundary tests.

## 0.1.52 — 2026-08-06

### Changed

- Decomposed the retained M2 transition into private authoritative evaluation
  and ordered event/effect projection modules behind the unchanged `lane`
  facade and v2 contract.
- Added characterization coverage for v2 hashes, replay identity, lifecycle,
  retained resource bounds, delayed effects, observations, branches,
  coordination, scenarios, strategy fixtures, and final debrief replay.

## 0.1.49 — 2026-08-06

### Added

- Added `LanePoultice` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_POULTICE_HASH_TAG` state-hash binding).
- Exposed `self_poultice` in `LanerObservation` and `laner_poultice` in `AlliedLaneObservation`.
- Supported `poultice_gained` and `poultice_spent` resolution during execution with direct-immediate `PoulticeGained`/`PoulticeSpent`/`PoulticeChanged` events and effects, debrief recording, and replay verification.
- Rejection of poultice overflow (`PoulticeOverflow`) or spending without available poultices (`InsufficientPoultice`) before state mutation.

## 0.1.48 — 2026-08-06

### Added

- Added `LaneSalve` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_SALVE_HASH_TAG` state-hash binding).
- Exposed `self_salve` in `LanerObservation` and `laner_salve` in `AlliedLaneObservation`.
- Supported `salve_gained` and `salve_spent` resolution during execution with direct-immediate `SalveGained`/`SalveSpent`/`SalveChanged` events and effects, debrief recording, and replay verification.
- Rejection of salve overflow (`SalveOverflow`) or spending without available salves (`InsufficientSalve`) before state mutation.

## 0.1.47 — 2026-08-06

### Added

- Added `LaneIncense` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_INCENSE_HASH_TAG` state-hash binding).
- Exposed `self_incense` in `LanerObservation` and `laner_incense` in `AlliedLaneObservation`.
- Supported `incense_gained` and `incense_spent` resolution during execution with direct-immediate `IncenseGained`/`IncenseSpent`/`IncenseChanged` events and effects, debrief recording, and replay verification.
- Rejection of incense overflow (`IncenseOverflow`) or spending without available incenses (`InsufficientIncense`) before state mutation.

## 0.1.46 — 2026-08-06

### Added

- Added `LaneFlask` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_FLASK_HASH_TAG` state-hash binding).
- Exposed `self_flask` in `LanerObservation` and `laner_flask` in `AlliedLaneObservation`.
- Supported `flask_gained` and `flask_spent` resolution during execution with direct-immediate `FlaskGained`/`FlaskSpent`/`FlaskChanged` events and effects, debrief recording, and replay verification.
- Rejection of flask overflow (`FlaskOverflow`) or spending without available flasks (`InsufficientFlask`) before state mutation.

## 0.1.45 — 2026-08-06

### Added

- Added `LanePhial` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_PHIAL_HASH_TAG` state-hash binding).
- Exposed `self_phial` in `LanerObservation` and `laner_phial` in `AlliedLaneObservation`.
- Supported `phial_gained` and `phial_spent` resolution during execution with direct-immediate `PhialGained`/`PhialSpent`/`PhialChanged` events and effects, debrief recording, and replay verification.
- Rejection of phial overflow (`PhialOverflow`) or spending without available phials (`InsufficientPhial`) before state mutation.

## 0.1.44 — 2026-08-06

### Added

- Added `LaneAmulet` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_AMULET_HASH_TAG` state-hash binding).
- Exposed `self_amulet` in `LanerObservation` and `laner_amulet` in `AlliedLaneObservation`.
- Supported `amulet_gained` and `amulet_spent` resolution during execution with direct-immediate `AmuletGained`/`AmuletSpent`/`AmuletChanged` events and effects, debrief recording, and replay verification.
- Rejection of amulet overflow (`AmuletOverflow`) or spending without available amulets (`InsufficientAmulet`) before state mutation.

## 0.1.43 — 2026-08-06

### Added

- Added `LaneTalisman` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_TALISMAN_HASH_TAG` state-hash binding).
- Exposed `self_talisman` in `LanerObservation` and `laner_talisman` in `AlliedLaneObservation`.
- Supported `talisman_gained` and `talisman_spent` resolution during execution with direct-immediate `TalismanGained`/`TalismanSpent`/`TalismanChanged` events and effects, debrief recording, and replay verification.
- Rejection of talisman overflow (`TalismanOverflow`) or spending without available talismans (`InsufficientTalisman`) before state mutation.

## 0.1.42 — 2026-08-06

### Added

- Added `LaneSigil` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_SIGIL_HASH_TAG` state-hash binding).
- Exposed `self_sigil` in `LanerObservation` and `laner_sigil` in `AlliedLaneObservation`.
- Supported `sigil_gained` and `sigil_spent` resolution during execution with direct-immediate `SigilGained`/`SigilSpent`/`SigilChanged` events and effects, debrief recording, and replay verification.
- Rejection of sigil overflow (`SigilOverflow`) or spending without available sigils (`InsufficientSigil`) before state mutation.

## 0.1.41 — 2026-08-06

### Added

- Added `LaneRune` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_RUNE_HASH_TAG` state-hash binding).
- Exposed `self_rune` in `LanerObservation` and `laner_rune` in `AlliedLaneObservation`.
- Supported `rune_gained` and `rune_spent` resolution during execution with direct-immediate `RuneGained`/`RuneSpent`/`RuneChanged` events and effects, debrief recording, and replay verification.
- Rejection of rune overflow (`RuneOverflow`) or spending without available runes (`InsufficientRune`) before state mutation.

## 0.1.40 — 2026-08-05

### Added

- Bounded `LaneTome` player consumable resource abstraction (`MAX_LANE_TOME = 5`) with zero default.
- Non-default `LaneTome` state-hash binding (`LANE_TOME_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player tome count (`self_tome`, `laner_tome`).
- `LaneExecutionInputs` support for `tome_gained` and `tome_spent` resolution.
- Direct-immediate `TomeGained`, `TomeSpent`, and `TomeChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::TomeOverflow` and `LaneExecutionError::InsufficientTome` fail-closed error handling.

## 0.1.39 — 2026-08-05

### Added

- Bounded `LaneScroll` player consumable resource abstraction (`MAX_LANE_SCROLL = 5`) with zero default.
- Non-default `LaneScroll` state-hash binding (`LANE_SCROLL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player scroll count (`self_scroll`, `laner_scroll`).
- `LaneExecutionInputs` support for `scroll_gained` and `scroll_spent` resolution.
- Direct-immediate `ScrollGained`, `ScrollSpent`, and `ScrollChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::ScrollOverflow` and `LaneExecutionError::InsufficientScroll` fail-closed error handling.

## 0.1.38 — 2026-08-05

### Added

- Bounded `LaneCharm` player consumable resource abstraction (`MAX_LANE_CHARM = 5`) with zero default.
- Non-default `LaneCharm` state-hash binding (`LANE_CHARM_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player charm count (`self_charm`, `laner_charm`).
- `LaneExecutionInputs` support for `charm_gained` and `charm_spent` resolution.
- Direct-immediate `CharmGained`, `CharmSpent`, and `CharmChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::CharmOverflow` and `LaneExecutionError::InsufficientCharm` fail-closed error handling.

## 0.1.37 — 2026-08-05

### Added

- Bounded `LaneRelic` player consumable resource abstraction (`MAX_LANE_RELIC = 5`) with zero default.
- Non-default `LaneRelic` state-hash binding (`LANE_RELIC_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player relic count (`self_relic`, `laner_relic`).
- `LaneExecutionInputs` support for `relic_gained` and `relic_spent` resolution.
- Direct-immediate `RelicGained`, `RelicSpent`, and `RelicChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::RelicOverflow` and `LaneExecutionError::InsufficientRelic` fail-closed error handling.

## 0.1.36 — 2026-08-05

### Added

- Bounded `LaneTrinket` player consumable resource abstraction (`MAX_LANE_TRINKET = 5`) with zero default.
- Non-default `LaneTrinket` state-hash binding (`LANE_TRINKET_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player trinket count (`self_trinket`, `laner_trinket`).
- `LaneExecutionInputs` support for `trinket_gained` and `trinket_spent` resolution.
- Direct-immediate `TrinketGained`, `TrinketSpent`, and `TrinketChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `TrinketOverflow` and `InsufficientTrinket`.

## 0.1.35 — 2026-08-05

### Added

- Bounded `LaneElixir` player consumable resource abstraction (`MAX_LANE_ELIXIR = 5`) with zero default.
- Non-default `LaneElixir` state-hash binding (`LANE_ELIXIR_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player elixir count (`self_elixir`, `laner_elixir`).
- `LaneExecutionInputs` support for `elixir_gained` and `elixir_spent` resolution.
- Direct-immediate `ElixirGained`, `ElixirSpent`, and `ElixirChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `ElixirOverflow` and `InsufficientElixir`.

## 0.1.34 — 2026-08-05

### Added

- Bounded `LanePotion` player consumable resource abstraction (`MAX_LANE_POTION = 5`) with zero default.
- Non-default `LanePotion` state-hash binding (`LANE_POTION_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player potion count (`self_potion`, `laner_potion`).
- `LaneExecutionInputs` support for `potion_gained` and `potion_spent` resolution.
- Direct-immediate `PotionGained`, `PotionSpent`, and `PotionChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `PotionOverflow` and `InsufficientPotion`.

## 0.1.33 — 2026-08-05

### Added

- Bounded `LaneFallbackBehavior` player intent fallback abstraction (`MaintainPlan`, `RetreatToTower`, `SafeFarm`, `ConserveResources`) with `MaintainPlan` default.
- Non-default `LaneFallbackBehavior` state-hash binding (`LANE_FALLBACK_BEHAVIOR_HASH_TAG`).
- `LanerObservation` advertising of available fallback behaviors.
- Request/command integration with `fallback_behavior` getters and constructors while preserving existing constructors.
- Direct-immediate `FallbackBehaviorSelected`, `FallbackBehaviorSet`, and `FallbackBehaviorTriggered` events and effects during transition evaluation, debrief recording, and replay verification.

## 0.1.32 — 2026-08-05

### Added

- Bounded `LaneAbortCondition` player intent abort condition abstraction (`None`, `HealthThreshold`, `ThreatSpotted`, `ResourceDepleted`) with `None` default.
- Non-default `LaneAbortCondition` state-hash binding (`LANE_ABORT_CONDITION_HASH_TAG`).
- `LanerObservation` advertising of available abort conditions.
- Request/command integration with `abort_condition` getters and constructors while preserving existing constructors.
- Direct-immediate `AbortConditionSelected`, `AbortConditionSet`, and `AbortConditionTriggered` events and effects during transition evaluation, debrief recording, and replay verification.

## 0.1.31 — 2026-08-05

### Added

- Bounded `LanePingSignal` player intent communication signal abstraction (`None`, `Danger`, `OnMyWay`, `Assist`, `EnemyMissing`) with `None` default.
- Non-default `LanePingSignal` state-hash binding (`LANE_PING_SIGNAL_HASH_TAG`).
- `LanerObservation` advertising of available ping signals.
- Request/command integration with `ping_signal` getters and constructors while preserving existing constructors.
- Direct-immediate `PingSignalSelected` and `PingSignalSet` events and effects during transition resolution, debrief recording, and replay verification.

## 0.1.30 — 2026-08-05

### Added

- Bounded `LaneWard` player vision resource abstraction `[0, MAX_LANE_WARD=5]` with zero default.
- Non-zero `LaneWard` state-hash binding (`LANE_WARD_HASH_TAG`).
- Player (`self_ward`) and allied (`laner_ward`) observation projections without exposing opponent ward count.
- Resolution of explicit `ward_gained` execution inputs emitting direct-immediate `WardGained`/`WardChanged` events & effects, debrief recording, and replay verification.

## 0.1.29 — 2026-08-05

### Added

- A bounded `LaneShield` player defensive shield resource with zero default and `LANE_SHIELD_HASH_TAG` state-hash binding.
- `LanerObservation` and `AlliedLaneObservation` exposure for player shield (`self_shield`, `laner_shield`) while hiding opponent shield.
- `LaneExecutionInputs` support for explicit `shield_gained` resolution during execution with direct-immediate `ShieldGained`/`ShieldChanged` events and effects, debrief recording (`shield_gained`), and `LaneRecordIdentity` integration.
- `LaneExecutionError::ShieldOverflow` error when gaining shield beyond `MAX_LANE_SHIELD` (50).

### Changed

- The package version advances to `0.1.29` for the bounded shield-resource slice; complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.28 — 2026-08-05

### Added

- A bounded `LaneDelayedEffects` player delayed-effect queue abstraction (maximum 4 items) with `LANE_DELAYED_EFFECT_HASH_TAG` state-hash binding.
- `LaneExecutionInputs` support for `delayed_effect` resolution; queued effects tick on each transition beat and resolve when delay expires (health regen, mana regen, cooldown reduction).
- Direct/indirect `Delayed` provenance for resolved effects, `DelayedEffectQueued` and `DelayedEffectResolved` events and effects, debrief recording (`delayed_effects_queued`, `delayed_effects_resolved`), and replay verification through `LaneScenarioHistory`.
- `LaneExecutionError::DelayedEffectOverflow` error when queuing past maximum capacity.

### Changed

- The package version advances to `0.1.28` for the bounded delayed-effect slice; complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.27 — 2026-08-05

### Added

- A bounded `LaneCommitment` player intent commitment abstraction with default `Standard`, explicit `Cautious` and `Aggressive` commitment options, observation advertising, request/command integration, state/record identity hash binding for non-default commitment, direct-immediate `CommitmentSelected`/`CommitmentSet` events and effects, debrief recording, and replay verification.

### Changed

- The package version advances to `0.1.27` for the bounded intent-commitment slice; commitment-based stat scaling and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.26 — 2026-08-05

### Added

- A bounded `LaneTargetFocus` player intent focus abstraction with default `Minions`, explicit `OpposingLaner` and `Tower` focus options, observation advertising, request/command integration, state/record identity hash binding for non-default target focus, direct-immediate `TargetFocusSelected`/`TargetFocusSet` events and effects, debrief recording, and replay verification.

### Changed

- The package version advances to `0.1.26` for the bounded intent-focus slice; multi-actor execution resolution and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.25 — 2026-08-05

### Changed

- Split the internal lane implementation and tests into responsibility-oriented
  private modules behind the unchanged `crate::lane::*` facade, and clarified
  resource and transition data flow with private product types without
  changing hashes, events, errors, replay behavior, or the placeholder binary.

## 0.1.24 — 2026-08-05

### Added

- A bounded `LaneMinionKills` player resource abstraction with zero default, player and allied observation projections, state/digest hash binding for non-zero minion kills, execution `minion_kills_gained` resolution, direct-immediate `MinionKillsGained`/`MinionKillsChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.24` for the bounded minion-kills-resource slice; minion wave spawn timing and last-hitting mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.23 — 2026-08-05

### Added

- A bounded `LaneLevel` player resource abstraction with initial default 1, player and allied observation projections, state/digest hash binding for non-initial level, execution `level_gained` resolution, direct-immediate `LevelGained`/`LevelChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.23` for the bounded level-resource slice; ability point trees and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.22 — 2026-08-05

### Added

- A bounded `LaneBounty` player resource abstraction with zero default, player and allied observation projections, state/digest hash binding for non-zero bounty, execution `bounty_earned` resolution, direct-immediate `BountyEarned`/`BountyChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.22` for the bounded bounty-resource slice; item catalog and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.21 — 2026-08-05

### Added

- A bounded `LaneCooldown` player resource abstraction with zero (ready) default, tick reduction by window beats, player and allied observation projections, state/digest hash binding for non-zero cooldowns, execution `cooldown_set` resolution, direct-immediate `CooldownSet`/`CooldownTicked`/`CooldownChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.21` for the bounded cooldown-resource slice; item catalog and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.20 — 2026-08-05

### Added

- A bounded `LaneExperience` player resource with zero default, player and allied observation projections, state/digest hash binding for non-zero experience, execution experience-gaining resolution, direct-immediate `ExperienceGained`/`ExperienceChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.20` for the bounded experience-resource slice; cooldowns, item catalog, and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.19 — 2026-08-05

### Added

- A bounded `LaneGold` player resource with full/zero compatibility defaults, player and allied observation projections, state/digest hash binding for non-zero gold, execution gold-earning resolution, direct-immediate `GoldEarned`/`GoldChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.19` for the bounded gold-resource slice; cooldowns, experience, item catalog, and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.18 — 2026-08-05

### Added

- A bounded player-facing `Yield` intent in `LanerObservation` and `transition_lane`, resolving deterministically to `NearTower` with zero damage and zero mana spent.
- Yield availability, execution validation, mana-spend rejection, replay, and objective-review tests while preserving existing intent tags and state-hash contracts.

### Changed

- The package version advances to `0.1.18` for the bounded Yield-intent slice; the executable remains the documented placeholder.

## 0.1.17 — 2026-08-04

### Added

- A bounded player-only opponent report: hidden `FarSide` truth projects as a
  current-turn `LastKnown` position while Center/NearTower remain Unknown.
- FarSide report, hidden health/posture, allied uncertainty, and history-replay
  coverage without changing lane state, transition inputs, or hashes.

### Changed

- The package version advances to `0.1.17` for the bounded opponent
  last-known-report slice; complete vision and belief updates remain deferred
  and the executable remains the documented placeholder.

## 0.1.16 — 2026-08-04

### Added

- A bounded `LaneMana` player resource with full-resource compatibility
  defaults, player/allied observation projections, and non-full state/digest
  binding.
- Contest-only explicit mana spending with fail-closed validation, ordered
  `ManaSpent`/`ManaChanged` attribution, debrief recording, and replay tests.
- Mana is included in lane record identity; matched-parent branches apply and
  record an intent-aware normalization when a Contest-only spend crosses to a
  non-Contest alternate.

### Changed

- The package version advances to `0.1.16` for the bounded mana-resource
  slice; cooldowns, gold, experience, regeneration, and abilities remain
  deferred and the executable remains the documented placeholder.

## 0.1.15 — 2026-08-04

### Added

- Explicit `LaneEffectProvenance` relationship/timing labels for emitted lane
  effects: direct-immediate for explicit execution/intent changes and
  indirect-immediate for Contest fallback movement.
- Direct/indirect effect provenance and no-delayed-emission tests while
  retaining existing cause/trace attribution and replay behavior.

### Changed

- The package version advances to `0.1.15` for the bounded effect-provenance
  slice; the executable remains the documented placeholder.

## 0.1.14 — 2026-08-04

### Added

- A bounded `LaneWindow::TwoBeats` duration in the authoritative snapshot,
  actor observations, allied proposal input, and transition turn advancement.
- Automatic close-on-commit and distinct two-beat state hashing with replay
  coverage while preserving the one-beat hash/identity behavior.

### Changed

- The package version advances to `0.1.14` for the bounded variable-duration
  window slice; the executable remains the documented placeholder.

## 0.1.13 — 2026-08-04

### Added

- A conditional player `Withdraw` response authorized only by a current
  RiverSide last-known threat report, with deterministic NearTower movement and
  explicit wave/execution preservation.
- Withdraw availability, unknown/stale/resolved rejection, attribution,
  unfavorable execution, replay, and objective tests while preserving the
  allied two-intent policy boundary.

### Changed

- The package version advances to `0.1.13` for the bounded gank-response slice;
  the executable remains the documented placeholder.

## 0.1.12 — 2026-08-04

### Added

- A bounded player-visible `LastKnown` RiverSide threat report with explicit
  observation-turn provenance while Absent and hidden current InLane truth
  remain Unknown.
- Last-known/unknown boundary and RiverSide replay tests while preserving the
  existing transition, intent, state-hash, and replay contracts.

### Changed

- The package version advances to `0.1.12` for the bounded last-known
  threat-report slice; the executable remains the documented placeholder.

## 0.1.11 — 2026-08-04

### Added

- A bounded player-facing `Recall` intent in the existing one-window lane
  command and transition contract, with explicit NearTower movement, wave and
  execution preservation, and ordinary YieldedSpace/ForcedOut outcomes.
- Recall legality, observation-boundary, attribution, and unfavorable
  execution tests while preserving the allied policy's two-intent candidate
  set and existing replay identities.

### Changed

- The package version advances to `0.1.11` for the bounded Recall-intent
  slice; the executable remains the documented placeholder.

## 0.1.10 — 2026-08-04

### Added

- A committed-facts `m2-two-window-final-debrief-v1` projection with per-window
  intent/coordination/execution/objective summaries, final objective
  aggregation, privileged source provenance, and a redacted visible report.
- Final-debrief replay, incomplete-history, tamper, and provenance-redaction
  tests while retaining all existing M2 window, branch, coordination,
  objective, fixture, and two-window tests.

### Changed

- The package version advances to `0.1.10` for the bounded final-debrief
  slice; the executable remains the documented placeholder.

## 0.1.9 — 2026-08-04

### Added

- A bounded `m2-two-window-scenario-v1` history that composes two existing
  one-beat lane transitions, reopens only a valid resolved first window, and
  stores exact sequence/reopen state for replay.
- Two-window append, terminal-state, invalid-reopen, third-window, and replay
  tamper tests while retaining all existing one-window, branch, coordination,
  objective, and strategy-fixture contracts.

### Changed

- The package version advances to `0.1.9` for the bounded two-window scenario
  slice; the executable remains the documented placeholder.

## 0.1.8 — 2026-08-04

### Added

- Named `HappyPath`, `RiskTaking`, and `Conservative` matched-input strategy
  fixtures that run through the existing host validation, coordination,
  execution, history, and terminal-objective contracts.
- Repeated-run, distinct-outcome, legal-unfavorable, replay, and tampered
  expectation tests for the three diagnostic cases.

### Changed

- The package version advances to `0.1.8` for the one-window strategy-fixture
  slice; the executable remains the documented placeholder.

## 0.1.7 — 2026-08-04

### Added

- A bounded `HoldLaneSpaceThroughWindow` scenario goal with deterministic
  `SpaceHeld`/`SurvivedBeat` criteria, achieved/partial/missed dispositions,
  committed-facts attribution, and a redacted visible objective report.
- Versioned objective input/source-record identities plus ordinary and
  coordinated objective review/replay verification with tamper detection.
- Focused objective, coordination-attribution, state-hash, report-redaction,
  and replay tests while retaining the existing M2 window, branch, and
  coordination fixtures.

### Changed

- The package version advances to `0.1.7` for the one-window scenario-goal and
  terminal-objective slice; the executable remains the documented placeholder.

## 0.1.6 — 2026-08-04

### Added

- A deterministic proposal-only allied actor projection with versioned
  profile/input identities, bounded candidate scores, hidden-state-safe
  observations, and stable proposal identity.
- One host-owned support offer, accept/reject/counter response boundary, five
  explicit coordination follow-through outcomes, coordination-attributed
  events/effects/debrief data, and one-record coordinated replay with tamper
  detection.
- Focused policy, information-boundary, coordination, execution-separation,
  state-hash, and coordinated-history tests while retaining the existing lane
  window and counterfactual branch fixtures.

### Changed

- The package version advances to `0.1.6` for the one-window allied
  proposal/coordination slice; the executable remains the documented
  placeholder.

## 0.1.5 — 2026-08-04

### Added

- A bounded one-window counterfactual branch with immutable parent history,
  matched-parent or explicitly regenerated execution inputs, stable branch
  traces, replay identity, and comparison limits that separate decision from
  execution changes.
- Branch validation, replay, tamper, parent-immutability, and causal-review
  tests while preserving the existing M2 lane transition contract.

### Changed

- The package version advances to `0.1.5` for the bounded branch slice; the
  executable remains the documented placeholder.

## 0.1.4 — 2026-08-04

### Added

- Internal M2 lane decision-window contracts for bounded lane state,
  actor-visible observations, `Stabilize`/`Contest` intent validation,
  explicit execution inputs, attributed events/effects, one-window debriefs,
  and append-only replay.
- Focused information-boundary, unfavorable-execution, validation,
  determinism, stream-isolation, and replay tests for the first lane slice.

### Changed

- The package version advances to `0.1.4` for the first M2 code slice; the
  executable remains the documented placeholder.

## 0.1.3 — 2026-08-04

### Added

- Strict dependency-free `1.0.0` snapshot/history text codecs with explicit
  hash-representation versioning, checked-in M1 fixtures, replay-backed
  deserialization, and fail-closed malformed/tampered-input tests.
- Exhaustive bounded spend/yield tests for energy bounds, conservation, and
  score/yield invariants.

## 0.1.2 — 2026-08-04

### Added

- Initial M1 `fog_of_intent::kernel` fixture with typed state, command
  validation, explicit resolved-input categories, deterministic transitions,
  attributed effects, authoritative hashes, append-only in-memory history, and
  replay verification.

### Changed

- The first M1 transition fixture is implemented and verified as an internal
  library surface; serialization, scenario mechanics, and user-facing adapters
  remain deferred.
- README now presents the project thesis, current pre-implementation status,
  initial vertical slice, canonical documents, and contributor workflow.
- The original proposal roadmap is labeled as a design source; `ROADMAP.md` is
  the canonical execution plan.

## 0.1.1 — 2026-08-04

### Added

- Dependency-free repository currentness/link checker, focused parser tests,
  and a pinned GitHub Actions workflow for clean-checkout verification.

## 0.1.0 — 2026-08-04

### Added

- Initial Rust 2024 binary package.
- Comprehensive project proposal for a turn-based, AI-native team-strategy
  simulation.
- Rust-first technology-stack analysis.
