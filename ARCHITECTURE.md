# Architecture

**Last reviewed:** 2026-08-13
**Status:** Partially verified — M1 kernel and fixture codec are implemented;
M2 remains an internal bounded target under construction. The current M2 v3
contract includes the lane decision window, retained-resource aggregate,
typed lifecycle status, delayed effects, branch, one-window allied
proposal/coordination overlay, terminal-objective projection, matched-input
strategy fixtures, bounded two-window wrapper, final debrief projection,
Recall/Withdraw/Yield intents, a fixed four-actor roster, explicit advance
conditions, report-derived belief values, and versioned replay identities. Experimental
M2 v1 resource slices are retired history and are not part of the current
surface.

## Overview

Fog of Intent is currently a single Rust 2024 package. The binary reports
package metadata through standalone `--version`/`-V` and runs the bounded
fixture loop with `--scenario m3-two-window-fixture-v1`, optional `--run-dir`,
and `--color auto|always|never`. One deferred edge crate, `reedline`, is used
only for TTY line editing. Kernel, lane, host, CLI grammar, and labeled
terminal-text modules remain free of that crate. Internal `kernel` and `lane`
modules provide bounded deterministic transitions, in-memory history, replay,
branching, coordination, objective, and debrief fixtures. No playable complete
match, MCP, research, or GUI component exists yet; an injected
persistent file store now exists as a library boundary, and the binary injects
it only when that option is supplied. M1 is complete as an internal fixture; M2
remains a bounded lane contract, and M3 now adds a bounded two-window host
fixture, replay-validated artifacts, injected file storage, pure terminal text,
and an optionally persistent fixture command loop rather than a complete
reference client.

The M3 CLI grammar is now a pure adapter module: it parses stable verbs and
borrows payload text, maps observe/inspect/help to typed read requests, maps
planning verbs to distinct typed write requests, maps review/debrief/replay/branch
to typed process requests, maps save/load/undo/quit to typed session requests,
and maps top-level commands (`play`, `replay`, `branch`, `experiment`, `export`,
`validate`, `mcp`, `help`, `version`) with interaction modes (`Guided`, `Expert`),
verbosity policies (`Concise`, `Standard`, `Explanatory`, `Research`), and explicit
privilege guards (`Unprivileged`, `Privileged`) without rendering, authorizing,
persisting, or invoking the simulation. Its versioned information-label schema
(`m3-cli-information-labels-v1`) distinguishes `observed`, `believed`,
`inferred`, `reported`, and `unknown`; the typed `CliInformation<T>` wrapper
cannot carry a payload for `unknown` and does not change the actor-visible
projection boundary. The `m3-cli-precommit-draft-v1` contract adds local
last-write-wins staging, clear-all undo, and a consuming `CliCommittedDraft`
marker with read-only getters; it does not edit committed history or authorize
domain commands.

Run references use the bounded borrowed `CliRunId<'a>` syntax contract for
save/load/replay/export adapter requests. Validation occurs at the adapter edge;
the application host still owns persistence backends, authorization, run
generation, collision handling, and history/replay identity. The bounded
`CliScenarioHost` fixture accepts explicit resolved inputs and stores a
replay-validated `m3-cli-host-artifact-v1` snapshot either in process or through
an injected `CliRunStore`; binary directory selection remains an outer-edge
concern.

Terminal rendering is intentionally outside the authoritative boundary. The
application host solely owns true-state lifecycle, legality, ordering, history
commit, and adapter coordination; the kernel and lane modules evaluate only
validated inputs within that host-owned boundary. The versioned terminal-text
projection consumes host-projected actor-valid values at the edge and must not
authorize commands, infer hidden state, or mutate history. The kernel, lane,
pure CLI grammar, and terminal projection modules own no terminal I/O or
rendering loop; `src/command_loop.rs` owns the explicit outer stdin/stdout
adapter; `src/presentation.rs` and `src/repl.rs` are TTY-only presentation
edges that cannot authorize commands. The CLI's static modes, verbosity
policies, and help metadata remain adapter contracts.

The target architecture is one authoritative Rust simulation product with thin
human, agent, and research adapters. The strongest boundary is:

```text
prior state + validated commands + resolved inputs + ruleset
  -> events + attributed effects + next state + state hash
```

The transition must remain synchronous and deterministic. Anything that reads
the wall clock, performs I/O, generates randomness, waits for agents, persists
artifacts, renders a UI, or speaks an external protocol belongs outside it.

The first recorded boundary decision is
[`docs/adr/0001-authoritative-transition-boundary.md`](docs/adr/0001-authoritative-transition-boundary.md).
The controlled vocabulary for that boundary is
[`docs/TERMINOLOGY.md`](docs/TERMINOLOGY.md).

## Current Repository Structure

```text
Cargo.toml
src/main.rs
src/lib.rs
src/agent.rs
src/cli.rs
src/agent_batch_store.rs
src/agent_operational_store.rs
src/host.rs
src/host_artifact.rs
src/run_store.rs
src/terminal.rs
src/presentation.rs
src/repl.rs
src/command_loop.rs
src/gui/
src/protocol.rs
src/session.rs
src/study/
src/kernel.rs
src/lane/
src/serialization.rs
tests/fixtures/
README.md
ROADMAP.md
SPEC.md
ARCHITECTURE.md
CHANGELOG.md
docs/
.agents/
_workspace/
```

`src/lib.rs`, `src/cli.rs`, `src/agent_batch_store.rs`, `src/agent_operational_store.rs`, `src/gui/`, `src/host.rs`, `src/host_artifact.rs`, `src/run_store.rs`, `src/terminal.rs`, `src/presentation.rs`, `src/repl.rs`, `src/protocol.rs`, `src/session.rs`, `src/study/`, `src/kernel.rs`,
`src/lane/`, and `src/serialization.rs` are the current internal
kernel/adapter/fixture surface;
`src/main.rs` parses bounded process options and runs the fixture loop, using
reedline only when stdin and stdout are terminals. The lane surface is split into private responsibility-oriented
modules behind the existing `crate::lane::*` facade: `evaluation.rs` owns
authoritative state evaluation, `projection.rs` owns ordered event/effect
projection, `result.rs` owns transition result/debrief assembly, and
`transition.rs` owns the public types and façade. The other paths are
project-state, design-source, and agent-workflow artifacts.

`src/agent.rs` is a pure, versioned policy boundary. Its
`m4-scripted-agent-v1` cautious, risk-taking, and yielding profiles consume
actor-visible lane observations and return requests for host validation; they
do not inspect true state, resolve execution, communicate, or mutate
authoritative history. An opt-in seeded tie path accepts only an explicit
policy seed bundle, and the library-only decision replay record remains
outside transition, host history, and durable persistence authority.
`ScriptedAgentExperimentManifest` records the bounded fixture, profile/rule
identities, and explicit policy seed for M6 reproducibility; it does not run
agents, sample populations, or own experiment execution.
`ScriptedAgentBatchRunner` sequences at most 16 such seeded decisions over one
actor-visible observation in process. `ScriptedAgentBatchCheckpoint` binds a
bounded cursor to the ordered actor-visible inputs, and
`ScriptedAgentBatchRunStore` reuses the injected run-store filesystem boundary
for cursor persistence; neither stores decisions nor owns transition/history
authority. `ScriptedAgentMatchedSample` reuses the runner across exactly two
same-actor, distinct-ID observations and returns only ordered profile/rule/seed
labels with selected intents; it does not generate populations or own outcome,
transition, history, or provider authority.
`ScriptedAgentExperimentVersionCatalog` is fixed metadata for the applicable
ruleset, scenario, policy schema, and profile identities; it marks
prompt/model/tool-schema/extractor versions as not applicable and does not
change manifest, execution, storage, or provider boundaries.
`ScriptedAgentMatchedScenarioSample` composes one to four caller-supplied
matched pairs in stable order, requiring one actor and globally distinct
observation IDs; it generates no scenarios or populations and owns no
distribution, outcome, transition, history, or provider authority.
`ScriptedAgentMatchedScenarioTallyReport` aggregates only the selected intents
from a verified sample set into bounded profile/rule counts; it does not rerun
policies or own population, outcome, persistence, or provider authority.
Its line-oriented codec is bounded and closed-field validated for
machine-readable evidence; decoding is accepted only when it matches an
already verified report, and it is not durable export or an external report
pipeline.
`ScriptedAgentMatchedScenarioTallyComparisonReport` pairs two such verified
reports only when their actor and ordered profile/rule identities match. It
retains bounded baseline/candidate counts and signed intent deltas without
rerunning policy evaluation or claiming build provenance, causality,
population, outcome, persistence, or provider authority.
Its bounded codec parses only the fixed positional metadata and ordered rows,
then compares the private candidate with an already verified comparison before
returning it; it is not durable export or an external report pipeline.
Its `m6-fixed-profile-tally-no-change-v1` gate is a pure equality predicate over
those bounded counts; it adds no threshold, balance, build, causal, outcome, or
strategic authority.
Its `largest_delta_candidate` helper exposes a closed
`m6-scripted-agent-tally-outlier-candidate-v1` metric projection under
`m6-largest-absolute-intent-delta-v1`, preserving stable row/intent tie order
and signed deltas. It does not detect outliers, select replays, inspect true
state, or add causal, population, persistence, provider, or outcome authority.
`ScriptedAgentFixtureScenarioSelection` is a closed metadata catalog for the
safe and RiverSide-threat fixture IDs. It binds caller-supplied observation IDs,
projects deterministic actor-visible pairs, and composes the existing sample
contract; repeated IDs are explicit ordered samples. It does not generate a
population, sample randomly, resolve a transition, or own history,
replay, persistence, provider, or outcome authority.
`ScriptedAgentFixtureScenarioPopulation` composes that catalog into one to four
deterministic alternating entries, deriving checked sequential observation-ID
pairs from a caller-supplied starting ID. Its ordered-composition constructor
also accepts caller-declared closed IDs, preserving skewed fixed-fixture input
without sampling authority. It is a fixed-fixture generator only; broader/random
population sampling, distributional metrics, outcomes, and human-behavior
evidence remain outside the core.
`ScriptedAgentFixtureScenarioFrequencyReport` also exposes a pure 10,000-point
caller-declared distribution projection over its two ordered rows. Integer
remainder handling is deterministic and the projection owns no sampling,
population, transition, history, persistence, provider, or outcome authority.
`ScriptedAgentMatchedScenarioTally` similarly exposes pure ordered intent
shares for `[Stabilize, Contest, Yield, Recall, Withdraw]`; this remains a
selected-intent projection over a verified fixture tally, not a population,
outcome, or strategic metric.
`ScriptedAgentStressPopulationReport` is a closed caller-declared four-case
matrix over existing validation, freshness, message-codec, and deterministic
policy boundaries. It adds no adversarial search, runtime, transition,
history, persistence, provider, outcome, or population authority.
`matched_tally` reuses the existing verified sample/tally path over those
entries and does not rerun policy evaluation or add population-metric,
transition, history, persistence, provider, or outcome authority. Multiple
caller manifests retain their stable cautious, risk-taking, and yielding row
order; this is not profile-population or outcome evidence.
`ScriptedAgentFixtureScenarioFrequencyReport` counts those explicit selection
labels in stable catalog order; it is bounded metadata over validated input,
not population, outcome, strategic, persistence, or provider evidence. Its
line-oriented codec validates fixed fields and returns only values matching an
already verified report; it is not durable export. Its pure Markdown projection
renders the same verified fields without I/O or additional authority.
`ScriptedAgentFixtureScenarioFrequencyComparisonReport` compares two such
verified reports with bounded ordered deltas; it is caller-declared evidence,
not independent build provenance or causal attribution. Its fixed no-change
gate is a pure equality check over those fields and adds no threshold authority.
An optional `ScriptedAgentBuildId` pair retains distinct caller-declared numeric
labels on the comparison; it does not verify source/package identity or infer
causality.
`ScriptedAgentRunDispositionRecord` is a caller-declared, payload-free status
envelope for completed, crashed, timed-out, missing-branch, and inconclusive
runs. Its closed codec preserves only categorical status metadata; it does not
detect process failures, retain diagnostics, attach decisions or results, or
own execution, persistence, provider, or experiment authority.
`ScriptedAgentOperationalLog` is a separate bounded in-memory container for
ordered payload-free batch lifecycle event labels. It is non-authoritative and
does not reconstruct history, emit runtime logs, inspect time/process state, or
durably persist operational data.
The batch runner can append a caller-driven start/chunk/finish trio only after
validation and capacity preflight; this producer preserves batch decision
parity and does not provide checkpoint, failure-detection, or transport
authority.
The injected checkpoint store similarly appends save/resume labels only after
successful bounded storage operations and one-slot preflight; it does not make
filesystem activity a runtime diagnostic or durably persist the operational log.
`ScriptedAgentOperationalLogStore` uses a separate bounded codec and file
suffixes, including caller-declared bounded segments, so operational labels
cannot collide with host artifacts or batch cursor files; automatic rotation
and crash recovery remain outer concerns. Its segment inventory is an
observational directory scan rather than a persistence or scheduling authority.
`ScriptedAgentOperationalLogSequenceReport` is a pure categorical check of the
caller-declared `batch_started` → `chunk_completed` → `batch_finished` label
order under `m6-operational-start-chunk-finish-v1`, with optional checkpoint/
resume labels between chunk and finish. `ScriptedAgentReplaySequenceEvidenceReport`
adds only the deterministic replay identity status of one existing decision
record beside that sequence status. These reports add no causal-trace, runtime,
scenario-wide replay, persistence, recovery, provider, or history authority.
`ScriptedAgentTallyOutlierThresholdReport` is a separate provisional
fixed-fixture threshold over verified signed count deltas; it does not infer
outliers, select replays, or add causal, population, persistence, provider,
or history authority.
`ScriptedAgentTallyReplayReference` selects a first caller-declared verified
record matching candidate profile/rule/intent labels. It is a reference only,
not representative replay proof or a scenario-wide replay authority.
`ScriptedAgentDegeneratePolicyPopulationReport` checks one to four
caller-declared actor-visible observations against the fixed cautious
`Stabilize` selection. It adds no adversarial search, prevalence, outcome,
history, persistence, provider, or human-behavior authority.
`ActorIllegalCommandPopulationReport` remains at the host validation boundary:
it repeats one observation-bound invalid command one to four times and retains
only the stable actor-safe `host_validation_rejected` category. It is read-only
metadata over host validation and adds no lane transition, history, replay,
transport, persistence, provider, exploit-search, communication, prevalence,
or outcome authority.
`ScriptedAgentExploitSeekingPopulationReport` is the corresponding fixed
risk-taking policy boundary: it validates one to four same-actor observations
and records only the `risk-taking-laner-v1`/`contest-first-fixed-score-v1`
selection of `Contest`. It does not search for exploits or add population,
prevalence, outcome, strategy-quality, transition, history, persistence,
provider, or human-evidence authority.
`ActorCommunicationAbusePopulationReport` is the corresponding invalid-message
policy boundary at the protocol edge: it validates one to four repeated invalid
message attempts against `ActorMessageDto::new` and retains only the stable
`InvalidValue` codec error plus sender, recipient, observation ID, and attempt
count. It does not route, deliver, or store message text, search for exploits,
or add transition, history, replay, transport, persistence, provider,
prevalence, or outcome authority.
`ScriptedAgentScenarioReplayIdentityReport` verifies deterministic replay
consistency across one to sixteen caller-supplied decision replay records from
a sampled run under `m6-scenario-replay-identity-v1`, retaining record/verified
counts and observation ID bounds. It adds no causal-trace, runtime automated log
production, durable persistence, provider, or human-gameplay authority.
`ScriptedAgentScenarioCausalTraceCompletenessReport` verifies causal-trace
completeness across one to sixteen caller-supplied decision replay records from
a sampled run under `m6-scenario-causal-trace-completeness-v1`, retaining
record/traced counts and observation ID bounds. It adds no runtime automated log
production, durable persistence, provider, or human-gameplay authority.
`ScriptedAgentCalibratedOutlierReplayReport` calibrates outlier detection from a
verified profile-aware comparison against a fixed threshold magnitude (2) and
traces a qualified outlier to a verified decision replay record under
`m6-calibrated-outlier-representative-replay-v1`. It adds no runtime automated
log production, durable persistence, provider, or human-gameplay authority.
`SemanticProfileDefinition` defines `m7-semantic-profile-vocabulary-v1`,
providing a compact categorical trait schema over risk tolerance, deference,
focus, and communication clarity dimensions for baseline reference profiles
(`cautious-laner-semantic-v1`, `risk-taking-laner-semantic-v1`,
`yielding-laner-semantic-v1`) alongside a fail-closed lookup catalog
(`SemanticProfileVocabulary`). It does not execute prompt generation or claim
human psychological validity.
`DiagnosticChoiceDefinition` defines `m7-diagnostic-choice-catalog-v1`,
providing typed diagnostic choice dilemma definitions across seven domains
(`ContestConcede`, `FollowReject`, `FarmAssist`, `RecallTiming`, `Sacrifice`,
`Surprise`, `ResponseToFailure`) alongside a fail-closed registry catalog
(`DiagnosticChoiceCatalog`). It establishes bounded strategic dilemmas for
calibration and does not claim full match scenario execution or human behavioral
completeness.
`ModelPromptProtocolDefinition` defines `m7-model-prompt-protocol-v1` and
`RepeatedSamplingProtocolDefinition` defines `m7-repeated-sampling-protocol-v1`,
providing typed declarations for model family IDs, prompt templates, system prompt
versions, temperature/top-p bounds, structured output enforcement, forbidden private
chain-of-thought, sample counts, and repair retry budgets alongside fail-closed catalogs
(`ModelPromptProtocolCatalog`, `RepeatedSamplingProtocolCatalog`). They establish
declarative sampling protocols for calibration and do not execute live LLM provider I/O.
`DiagnosticChoiceActionDistribution` defines `m7-empirical-action-distribution-v1` and
`DiagnosticChoiceCommunicationDistribution` defines `m7-empirical-communication-distribution-v1`,
providing typed empirical action and communication ping signal frequency distributions
with exact integer 10,000 basis-point representations alongside aggregated diagnostic
reports (`EmpiricalDistributionEstimateReport` under `m7-empirical-distribution-estimation-v1`).
They establish declarative empirical distribution estimates for calibration and do not
execute live LLM provider I/O or claim human psychological ground truth.
`BehavioralDistanceMeasure` (`m7-behavioral-distance-v1`), `BehavioralEntropyMeasure`
(`m7-behavioral-entropy-v1`), `BehavioralSensitivityMeasure` (`m7-behavioral-sensitivity-v1`),
`BehavioralConsistencyMeasure` (`m7-behavioral-consistency-v1`), and `BehavioralAdaptationMeasure`
(`m7-behavioral-adaptation-v1`) provide discrete integer basis-point calculators for Total
Variation Distance, Gini diversity, dilemma contrast deltas, modal adherence, and defensive shifts
alongside aggregated reports (`BehavioralMeasuresReport` under `m7-behavioral-measures-v1`).
They establish declarative behavioral metrics for calibration without floating-point math or
hidden state.
`ParametricActionWeights`, `ParametricCommunicationWeights`, and `ParametricPolicyDefinition`
(`m7-parametric-policy-v1`) define bounded parametric policy parameter weights across the seven
diagnostic dilemma domains with exact integer basis-point conservation ($\sum w_i = 10,000$ bp)
and regularized parameter fitting via `ParametricPolicyFitter`. Regularization applies bounded
basis-point shrinkage towards neutral uniform priors to prevent extreme weights and resolve
unidentifiable parameters deterministically without floating-point math or provider APIs.
`HeldOutScenarioDefinition` (`m7-held-out-scenario-v1`) and `HeldOutScenarioEvaluationReport`
(`m7-held-out-scenario-evaluation-v1`) evaluate Total Variation Distance loss and modal accuracy
against held-out diagnostic scenario ground-truth distributions. `CounterfactualPerturbationDefinition`
(`m7-counterfactual-perturbation-v1`) and `CounterfactualSensitivityReport` (`m7-counterfactual-sensitivity-v1`)
evaluate directional shift coherence under discrete perturbation conditions (`ThreatEscalation`,
`AlliedRetreatCall`, `SevereHealthAttrition`, `FavorableOpening`). `CalibrationHeldOutReport`
(`m7-calibration-held-out-v1`) unifies generalization loss and counterfactual sensitivity into a
deterministic qualification gate.
`MultiModelComparisonReport` (`m7-multi-model-comparison-v1`) evaluates Total Variation Distance
deltas across action and communication distributions, parametric policy weight shifts, modal choice
agreement (0..=7), and categorical alignment status (`aligned`, `shifted`, `divergent`) between
reference and alternative model/prompting protocols across diagnostic dilemmas.
`ParameterIdentifiabilityReport` (`m7-parameter-identifiability-v1`), `SemanticLabelStabilityReport`
(`m7-semantic-label-stability-v1`), and `CalibrationUncertaintyReport` (`m7-calibration-uncertainty-v1`)
evaluate empirical sensitivity, confounding risk, cross-model stability, and overall uncertainty scoring
with explicit basis-point thresholds and canonical disclaimers stating that AI behavior serves solely
as a reference policy distribution, not human ground truth.
`ReferenceOutputRecord` (`m7-reference-output-v1`), `StructuredRationale`, `ReferenceOutputPreservationReport`
(`m7-reference-output-preservation-v1`), and `ReferenceOutputCatalog` preserve observable decision outputs
(`LaneIntent`, `LaneTargetFocus`, `LaneCommitment`, `LanePingSignal`, bounded `StructuredRationale`) across all
seven diagnostic dilemma domains, strictly failing closed if private chain-of-thought is requested or present
(`chain_of_thought_present == true`).
`RecalibrationTriggerCondition` (`m7-recalibration-trigger-v1`), `RecalibrationPolicy`, and `RecalibrationEvaluationReport`
(`m7-recalibration-evaluation-v1`) evaluate distributional drift and contract breaches across 9 discrete trigger reasons
and 3 urgency levels (`Immediate`, `Scheduled`, `None`), enforcing basis-point threshold checks across model comparisons,
uncertainty reports, generalization loss, and CoT-free reference outputs.
`CalibrationModelCardReport` (`m7-calibration-model-card-v1`) formalizes the canonical M7 calibration proof deliverable,
documenting intended use, evidence boundaries, evaluated profiles, generalization status, uncertainty findings,
recalibration policy rules, and zero private chain-of-thought constraints.

`src/agent/communication.rs` defines the foundational M8 team communication contracts:
`TeamSpeechAct` (`m8-team-speech-act-v1`) covering 8 canonical communicative speech acts
(`Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`, `ConditionalCommitment`,
`Withdrawal`, `FailureReport`), `TeamRecipient` (broadcast vs direct role targeting), `TeamMessageUrgency`
(`Low`, `Standard`, `Critical`), `TeamConfidenceLevel` (`Tentative`, `Confident`, `Definite`),
`TeamMessageCondition` (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`, `ResourceSufficient`),
`TeamMessageVisibility` (`TeamOnly`, `DirectOnly`, `Public`) with actor/team visibility predicate rules,
`TeamMessageEnvelope` (`m8-team-message-envelope-v1`) with fail-closed zero private chain-of-thought enforcement,
`TeamCommunicationCatalog` (`m8-team-communication-v1`) with registered canonical example envelopes across
all speech acts, `TeamDialogueStatus` (`m8-team-dialogue-v1`) covering 8 discrete dialogue states,
`TeamDissentReason` covering 6 discrete causal dissent reasons, `TeamConditionEvaluator` evaluating prerequisite
conditions against actor-visible context, `TeamSpeechActProfile` evaluating proposals across cautious, risk-taking,
and yielding postures, `TeamDialogueSession` managing bounded multi-turn dialogue state transitions (max 4 rounds,
max 8 messages), and `TeamDialogueCatalog` providing 7 canonical complete dialogue session transcripts.

`src/agent/team_plan.rs` defines the structured team-plan and individual-plan contracts:
`TeamStrategicObjective` (6 discrete strategic objectives), `TeamPlanPhase` (4 discrete plan phases),
`RolePlanAssignment`, `TeamPlanDefinition` (`m8-team-plan-v1`), `IndividualPlanDefinition` (`m8-individual-plan-v1`),
`TeamPlanAlignmentType` (`m8-team-plan-relationship-v1`), `AlignmentEvaluation`, `TeamPlanAlignmentReport`,
`TeamPlanEvaluator` managing deterministic alignment evaluation with exact integer basis-point cohesion scoring
($[0..=10,000]$ bp) and Markdown summary export, and `TeamPlanCatalog` registering 6 canonical reference team plans.

`src/agent/trust.rs` defines multi-agent trust dynamics, caller reputation, and communication channel physics:
`TeamTrustLevel` (4 discrete tiers derived from basis points), `CallOutcome`, `CallerReputationRecord`
(`m8-caller-reputation-v1`) with exact basis-point score updates, `TeamTrustMatrix`, `CommunicationClarity`
(4 clarity tiers with basis-point multipliers), `TransmissionDelay` (0..=2 beat delays), `DeliveryStatus`,
`ChannelPacket`, `TeamCommunicationChannel` (`m8-communication-channel-v1`) with bounded FIFO queue (capacity 16)
and turn-tick progression, `TrustComplianceDecision` and `TrustEvaluationReport` (`m8-team-trust-v1`),
`TeamTrustEvaluator` evaluating proposal compliance against caller reputation and local observations, and
`TeamTrustCatalog` registering 4 canonical reference caller reputation records.

`src/agent/leadership.rs` defines designated shot-caller and decentralized coordination baseline policies:
`ConsensusRule` (4 discrete arbitration algorithms: `UnanimousConsensus`, `HighestReputationLead`, `UrgencyFirst`,
`MajoritySupport`), `FallbackLeadershipMode` (3 fallback policies: `FallbackToIndividualPlans`, `FallbackToDefaultHold`,
`FallbackToSecondaryCaller`), `LeadershipStructure` (`m8-leadership-structure-v1`) covering `DesignatedShotCaller`,
`Decentralized`, and `SharedLeadership`, `ShotCallerDirective` and `ShotCallerPolicy` (`m8-shot-caller-policy-v1`)
for observation-conditioned directive generation, `PeerPlanProposal` and `DecentralizedCoordinator`
(`m8-decentralized-coordination-v1`) for proposal arbitration and tie deadlock detection, `LeadershipEvaluationReport`
(`m8-leadership-evaluation-report-v1`), `TeamLeadershipEvaluator` evaluating compliance decisions and dissent reasons
across teammates against trust matrices and local observations, `LeadershipCatalog` defining 6 canonical baseline
configurations, and `TeamLeadershipError`.

`src/agent/simultaneous.rs` defines multi-agent simultaneous decision collection and resolution:
`TeamSimultaneousPhase` (4 discrete window lifecycle states), `TeamCoordinationOutcome` (5 discrete coordination
outcomes: `FullyCoordinated`, `PartiallyCoordinated`, `DivergentIntents`, `ConflictingDirectives`, `CommunicationFailure`),
`TeamSubmissionEnvelope` (`m8-team-simultaneous-submission-v1`) with fail-closed zero private chain-of-thought enforcement,
`TeamSubmissionReceipt` (payload-free receipt), `TeamSimultaneousWindow` (managing up to 4 registered roles with strict
pre-resolution submission redaction), `RoleResolvedIntent`, `TeamSimultaneousResolution` (`m8-team-simultaneous-resolution-v1`)
with Markdown summary reporting, `TeamSimultaneousResolver` evaluating plan alignment, proposal trust compliance, and
leadership consensus into integer basis-point cohesion ($[0..=10,000]$ bp), and `TeamSimultaneousCatalog`
(`m8-team-simultaneous-catalog-v1`) registering 5 canonical reference scenarios.

`src/agent/attribution.rs` defines the strategic coordination versus mechanical execution attribution subsystem:
`AttributionQuadrant` (4 canonical quadrants: `CoordinatedTriumph`, `CoordinatedFailure`, `UncoordinatedBailout`,
`CompoundedFailure`) decoupling coordination effectiveness ($\ge 5,000$ bp) from mechanical execution efficiency ($\ge 5,000$ bp),
`CoordinationRating` and `ExecutionRating` (4 discrete performance tiers each), `CoordinationCausalFactor` (8 discrete drivers)
and `ExecutionCausalFactor` (8 discrete drivers), `CoordinationAssessment` and `ExecutionAssessment`, `AttributionWeights`
(`m8-coordination-execution-attribution-v1`) enforcing exact basis-point sum conservation ($10,000$ bp),
`CoordinationExecutionAttribution` and `CoordinationExecutionAttributionReport` (`m8-coordination-execution-attribution-report-v1`)
with Markdown debrief rendering and fail-closed zero private chain-of-thought rejection, `AttributionEvaluationInput`,
`TeamAttributionEvaluator` synthesizing multi-agent simultaneous resolutions with physical lane outcomes, and
`CoordinationAttributionCatalog` (`m8-coordination-attribution-catalog-v1`) registering 6 canonical benchmark scenarios.

`src/agent/debrief.rs` defines causal post-encounter debrief summaries:
`CommunicationDebriefSummary` (`m8-team-communication-debrief-v1`) tracking packet transmission reliability ($[0..=10,000]$ bp),
dialogue round metrics, and dissent frequency breakdowns, `LeadershipDebriefSummary` (`m8-team-leadership-debrief-v1`)
tracking directive compliance rates, consensus deadlocks, fallback activations, and caller reputation deltas ($[-10,000..=10,000]$ bp),
and `TeamEncounterDebriefReport` (`m8-team-encounter-debrief-v1`) synthesizing simultaneous resolutions, decoupled attribution,
communication debriefs, and leadership debriefs into structured Markdown reports with strict zero private chain-of-thought enforcement.

`src/agent/disagreement.rs` defines strategic disagreement legitimacy evaluation:
`DisagreementLegitimacyClassification` (`LegitimateDissent`, `ConstructiveAlternative`, `UnjustifiedInsubordination`),
`DisagreementLegitimacyEvaluation` (`m8-strategic-disagreement-v1`), and `TeamDisagreementEvaluator` evaluating counterfactual
payoff value deltas ($[-10,000..=10,000]$ bp) to formally verify that dissent under adverse health and threat conditions is
strategically legitimate and value-accretive.

`src/agent/scenarios.rs` defines the canonical benchmark scenario battery:
`TeamScenarioDefinition` (`m8-team-scenarios-v1`), `TeamScenarioExecutionResult`, and `TeamScenarioCatalog` (`m8-team-scenario-catalog-v1`)
registering and executing 5 canonical benchmark scenarios (`scenario-high-trust-gank-v1`, `scenario-low-trust-dissent-v1`,
`scenario-conflicting-calls-arbitration-v1`, `scenario-missing-message-fallback-v1`, `scenario-strategic-dissent-survival-v1`).

`src/map/` defines the foundational spatial topology, graph pathfinding, deterministic travel/rotation model, neutral objective cycles, vision control, cross-map tradeoff mechanics, team compositions, structures hierarchy, match victory conditions, comeback/variance-seeking evaluation, pivotal-decision detection, and decision-density preservation for M9:
- `topology.rs`: `MapLocation` (`m9-map-topology-v1`) covering 15 discrete map locations (2 bases `AlliedBase`, `OpposingBase`, 9 lane sectors across `Top`, `Mid`, `Bot` lanes, 2 river zones `TopRiver`, `BotRiver`, and 2 jungle quadrants `TopJungle`, `BotJungle`).
- `graph.rs`: Adjacency matrix, deterministic BFS shortest-path calculation, integer beat durations ($1\text{ beat} = 1\text{ step}$), and validated `TravelRoute`.
- `travel.rs`: `ActorLocation` (`Stationary` vs `InTransit`), `TransitState` machine, `TravelCommand` (`InitiateRotation`, `ContinueTransit`, `AbortRotation`), and fail-closed validation.
- `transition.rs`: Pure deterministic `transition_travel` function advancing transit progress by integer beats, handling arrivals and aborts, and emitting structured `TravelEvent`s and `TravelEffect`s.
- `state.rs`: `MatchMapState` with turn counter, multi-actor locations, deterministic FNV-1a state hashing, and `MatchMapObservation` projection with strict fog-of-war redaction (unseen rotating opponents are reported as `Unknown`).
- `catalog.rs`: `MapScenarioDefinition` and `MapTravelCatalog` (`m9-map-scenario-catalog-v1`) with 4 canonical benchmark scenarios (`top_to_mid_gank`, `bot_to_river_contest`, `mid_to_base_reset`, `aborted_rotation_threat`).
- `objective.rs`: `ObjectiveKind` (`TopRiverObjective`, `BotRiverObjective`), `ObjectiveStatus` (`Unspawned`, `Active`, `Secured`), and `MatchObjectiveState` (`m9-objective-cycles-v1`) with spawn/respawn turn countdowns and health pools.
- `vision.rs`: `VisionWard`, `VisionCoverage` (`FullVision`, `LastKnown`, `ConcealedInFog`), `MapVisionState`, `VisionCommand` (`PlaceWard`, `ClearWard`), and `MapVisionGrid` (`m9-vision-control-v1`) with fog-of-war resolution and ward expiration.
- `contest.rs`: `ObjectiveIntent` (`Engage`, `SecureBurst`, `ZoneOpponents`, `ConcedeAndTrade`), `CrossMapTradeTarget`, `TradeClassification`, `TradeoffEvaluation` (`m9-objective-contest-v1`), and pure deterministic `transition_objective_contest` emitting `ObjectiveEvent` and `ObjectiveEffect`.
- `objective_catalog.rs`: `ObjectiveScenarioDefinition`, `ObjectiveScenarioExecutionResult`, and `ObjectiveScenarioCatalog` (`m9-objective-catalog-v1`) registering 4 canonical benchmark scenarios (`dragon_contest`, `cross_map_trade`, `vision_setup_and_catch`, `stealth_objective_sneak`) with replay hash verification.
- `composition.rs`: `MatchRole` (5 discrete positions), `CompositionArchetype` (`EarlyPick`, `TeamfightScaling`, `SplitPush`, `PokeSiege`), `PowerScalingCurve` (`EarlyGame`, `MidGame`, `LateGame`), `CompositionMatchupEvaluation` with integer basis-point power deltas ($[-10,000..=10,000]$ bp) and `RecommendedPosture`, and `CompositionCatalog` (`m9-team-composition-v1`).
- `structures.rs`: `StructureTier` (`OuterTurret`, `InnerTurret`, `InhibitorTurret`, `Inhibitor`, `Nexus`), `StructureStatus`, `MatchStructureState` (`m9-match-structures-v1`) with 26-structure defense hierarchy, vulnerability checks, siege resolution (`transition_structure_siege`), inhibitor respawn ticking (`tick_turn`), super minion wave spawning (`has_super_minions`), `StructureEvent`, and `StructureEffect`.
- `victory.rs`: `MatchVictoryCondition` (`NexusDemolished`, `MatchConceded`, `DecisiveAce`), `MatchStatus`, and `MatchTerminalEvaluation` (`m9-match-victory-v1`) evaluating match conclusion milestones with structured Markdown summaries.
- `match_catalog.rs`: `MatchScenarioDefinition`, `MatchScenarioExecutionResult`, and `MatchScenarioCatalog` (`m9-match-scenario-catalog-v1`) registering and executing 4 canonical benchmark match scenarios (`early_pick_snowball`, `split_push_base_race`, `late_game_scaling_comeback`, `siege_inhibitor_concession`) with replay hash verification.
- `role_observation.rs`: `WaveStateSummary`, `RoleSpecificContext` (`TopLanerContext`, `JunglerContext`, `MidLanerContext`, `BotCarryContext`, `SupportContext`), and `RoleMatchObservation` (`m9-role-observation-v1`) projecting role-specialized situational context with strict fog-of-war compliance.
- `role_action.rs`: `TopIntent`, `JungleIntent`, `MidIntent`, `BotCarryIntent`, `SupportIntent`, `RoleIntent`, `RoleAction`, and `validate_role_action` (`m9-role-action-v1`) defining closed role tactical action spaces with fail-closed cooldown and capability validation.
- `role_debrief.rs`: `RolePerformanceTier`, `RoleCausalFactor` (16 discrete causal drivers), `RoleKpis` (integer basis-point metrics in $[0..=10,000]$ bp), and `RoleDebriefPerspective` (`m9-role-debrief-v1`) evaluating role performance without outcome bias.
- `role_catalog.rs`: `RoleScenarioDefinition`, `RoleScenarioExecutionResult`, and `RoleScenarioCatalog` (`m9-role-scenario-catalog-v1`) registering and executing 5 canonical benchmark scenarios (`scenario-top-teleport-flank-v1`, `scenario-jungler-objective-steal-v1`, `scenario-mid-roam-conversion-v1`, `scenario-bot-hypercarry-scaling-v1`, `scenario-support-vision-setup-peel-v1`) with replay hash verification.
- `comeback.rs`: `DeficitLevel` (`Ahead`/`Parity`/`Deficit`/`SevereDeficit`), `VarianceSeekingBehavior`, `ComebackOpportunityInputs`, and the pure `evaluate_comeback_opportunity` (`m9-comeback-mechanics-v1`) classifying explicit structural/objective net deltas in integer basis points into deterministic variance-seeking recommendations.
- `comeback_catalog.rs`: `ComebackScenarioDefinition`, `ComebackScenarioExecutionResult`, and `ComebackCatalog` (`m9-comeback-catalog-v1`) registering and executing 3 canonical benchmark scenarios (`teamfight_comeback`, `desperation_all_in`, `ahead_conservative`).
- `pivotal.rs`: `PivotalDecisionSample`, `PivotalTier` (`Routine`/`Notable`/`Pivotal`/`MatchDefining` at explicit 500/1,500/3,500 bp swing thresholds), `SwingDirection`, `DecisionAlignment`, and the pure fail-closed `detect_pivotal_decisions` (`m9-pivotal-decision-v1`) classifying caller-declared match value trajectories with strict sign-flip lead-change detection.
- `pivotal_catalog.rs`: `PivotalScenarioDefinition`, `PivotalScenarioExecutionResult`, and `PivotalCatalog` (`m9-pivotal-catalog-v1`) registering and executing 3 canonical benchmark scenarios (`base_race_decisive_swing`, `baron_throw_comeback`, `stable_slow_burn`).
- `cli/match_replay.rs`: `MatchReplayTranscript` and `build_match_replay_transcript` (`m9-complete-match-replay-v1` executable scenario) — a pure, I/O-free projection that executes and replay-verifies both canonical complete matches and emits stable labeled plain text with categorical replay-match flags; `write_match_replay_transcript` at the `command_loop.rs` executable boundary writes it and exits, with `--run-dir` rejected for this scenario.
- `decision_density.rs`: `CandidateWindowKind` (5 routine + 5 strategic window categories), `RoutineWindowCandidate`, `EscalationTrigger` (`StrategicKind`/`StakesAboveThreshold` above the 500 bp `ROUTINE_STAKES_CEILING_BP`/`ThreatPresent`/`ObjectiveActive` in fixed priority order), `WindowDisposition` (`AutomaticallyExecuted`/`DecisionRequired`), and the pure fail-closed `evaluate_decision_density` (`m9-decision-density-v1`) classifying caller-declared window streams and evaluating decision share against the explicit `[1,000..=5,000]` bp band and 6-turn decision-gap bound.
- `decision_density_catalog.rs`: `DecisionDensityScenarioDefinition`, `DecisionDensityScenarioExecutionResult`, and `DecisionDensityCatalog` (`m9-decision-density-catalog-v1`) registering and executing 3 canonical benchmark scenarios (`routine_laning_absorption`, `objective_spike_escalation`, `decision_overload`).
- `complete_match.rs`: `CompleteMatchState` (integrated sequencing of the map, objective, vision, and structure state machines), `CompleteMatchAction` (`Rotate`/`PlaceWard`/`ContestObjectives`/`SiegeStructure`/`EvaluateTerminal`), `CompleteMatchPlan::execute` driving each action through its real subsystem transition (`m9-complete-match-v1`), and one combined FNV-1a hash committing every subsystem (including the ward-id sequence and team membership); fail-closed on empty, unterminated, post-conclusion (including post-Nexus), untracked-actor, and subsystem-rejected runs.
- `complete_match_catalog.rs`: `CompleteMatchCatalog` (`m9-complete-match-catalog-v1`) registering 2 canonical complete matches (`complete_allied_snowball` ending `NexusDemolished` at turn 15, `complete_comeback_concession` ending `MatchConceded` at turn 29).
- `population_validation.rs`: `MechanicKind` (closed 8-mechanic M9 catalog), `ReplaySummary` (caller-declared replay summaries), `MechanicExemption`, and the pure fail-closed `measure_validation_population` (`m9-population-validation-v1`) measuring strategy-share diversity, per-role activity (1,000 bp floor), communication usage (2,500 bp floor), and unused-mechanic justification over declared validation populations.
- `population_validation_catalog.rs`: `PopulationScenarioDefinition`, `PopulationScenarioExecutionResult`, and `PopulationValidationCatalog` (`m9-population-validation-catalog-v1`) registering and executing 3 canonical benchmark scenarios (`diverse_engaged_population`, `narrow_passive_population`, `exempted_unused_mechanic`).
- `cost_profile.rs`: `OperationCounts` (exact transition/hash/projection/replay counters), `ScenarioCostProfile`, `ScalingProbe`, and `CostProfileReport` (`m9-cost-profile-v1`) with `profile_travel_scenario`, `profile_scaling_probe`, and `profile_catalog_batch` profiling the canonical travel catalog deterministically by operation counts — wall-clock timing stays at repository edges — including the [1, 8, 64, 512] step scaling ladder with linear transition growth (marginal cost 2 per step) and constant per-pass hash work (2 evaluations). `catalog.rs` gains `execute_with_state` so profiling performs real terminal-state projections without sharing authoritative state.


`src/protocol.rs` owns the bounded actor observation/action/commit/draft/message/draft-receipt/
draft-status/draft-clear/draft-commit-receipt/replay-record/replay-debrief-record/transcript DTO
projection. It
maps primitive actor-visible fields and closed intent IDs without exposing
internal observation/request types as a transport contract; host validation
still owns legality. It also maps codec failures to the versioned,
actor-safe `m5-actor-error-v2` code/repair vocabulary and its bounded codec
without retaining raw input or parser details. `src/host.rs` owns the
actor-observation, actor-commit, draft-readback, draft-status, draft-clear, history-status, replay-status,
replay-record, saved-replay-record, saved-debrief-summary, action-result, and completion-gated debrief
projections plus actor-action validation and submission entry points: it delegates legality to the lane
validator and closes a fixture window only after successful validation and
history append.
`ActorMessageDto` is a bounded recipient-scoped envelope for actor-authored
text; it binds sender, recipient, and observation identity without routing or
delivery authority. `ActorDraftDto` remains a bounded metadata envelope in the
protocol edge, while
`src/host.rs` owns its observation-bound pre-commit staging and its
`ActorDraftReceiptDto` acknowledgement. `CliScenarioHost::actor_draft` reads
the actor-protocol-staged values back to the requesting actor in stable field
order without reinterpreting legacy CLI draft text, delivering them to a
recipient, or changing host state. `ActorDraftStatusDto`
adds only the active binding and aggregate field-presence bits; the status and
receipt DTOs contain no draft value. The receipt contains no draft value;
staging replaces one internal draft field but does not add communication
authority or transition authority. `ActorDraftClearDto` and
`ActorDraftClearReceiptDto` bind an idempotent clear to the active observation
and report only pre-clear field presence; they do not deliver payloads.
`ActorDraftCommitReceiptDto` reports only the
committed intent and `present`/`absent` status of each staged field after a
successful commit; it never echoes values or claims delivery. `ActorDebriefDto`
is a committed-facts summary only; the host derives
it from the existing complete lane report and keeps detailed causal fields,
replay identity, and persistence outside the protocol contract.
`CliScenarioHost::actor_debrief_from_run` applies the same local restore and
completion gate through the injected store and leaves the receiving host
unchanged; it does not add durable or causal replay authority.
`ActorTranscriptDto` is provider-neutral compatibility metadata only; runtime
transport logging, prompt/model details, and durable retention remain outer
adapter concerns.
`ActorToolCapability` is pure ordinary-versus-privileged labeling metadata;
the current catalog contains only ordinary actor tools and grants no runtime
authority. Public protocol compatibility is DTO-only: authoritative lane
observation projection and action-request conversion stay behind crate-private
adapters, keeping domain types out of the provider-facing contract.

`ActorSimultaneousWindow` is a pure two-actor collection boundary. It binds one
shared observation ID, rejects stale/cross-actor/duplicate submissions, and
reveals bounded binding metadata plus readiness; the host still owns ordering,
transition resolution, history, and replay.

The host regression compares the existing CLI observation and
plan/commit/advance path with the actor DTO projection and action-result path on
the same fixture. This proves bounded CLI/protocol parity without introducing
MCP transport or provider authority.

`src/session.rs` owns immutable ordinary-actor session freshness and lifecycle
metadata only. Its `m5-actor-session-v2` boundary maps encoded malformed,
stale, duplicate, timeout, and disconnect events into bounded actor-safe
outcomes; it cannot validate an intent, submit a transition, or mutate history.
`ActorReplayDto` is a read-only host projection of successful current-history
verification; it exposes only a categorical result and bounded record count,
never hashes, resolved inputs, or traces. `ActorReplayRecordDto` is a bounded
categorical window/intent/outcome entry returned only after the same replay
verification and never carries record identity, provenance, or causal detail.
`CliScenarioHost::actor_replay_records_from_run` performs the same verification
after loading an ID-derived artifact through the injected store and leaves the
current host untouched; filesystem hardening and scenario-wide durable replay
remain outer concerns. `CliScenarioHost::actor_replay_debrief_records_from_run`
uses the same local restore boundary, requires a complete two-record history,
and projects only the existing categorical debrief records without replacing
the receiving host.
`ActorReplayDebriefRecordDto` adds only a categorical objective and committed-
facts attribution for each complete verified window; it remains read-only and
omits causal, hash, input, and trace detail.

`src/study/` defines the human usability and accessibility study protocol, participant
session schema, finding taxonomy, deterministic cohort evaluation, and benchmark catalog
for M10:
- `protocol.rs` (`m10-study-protocol-v1`): defines `StudyProtocolDefinition` with explicit
  research questions, target completion and comprehension floors, and strict
  `PrivacyConsentDeclaration` invariants (de-identified IDs, zero PII, zero latent state leakage),
  4 participant cohorts (`StrategyGamer`, `MobaPlayer`, `AccessNeeds`, `NoviceStrategy`), and
  10 canonical evaluation dimensions.
- `finding.rs` (`m10-finding-taxonomy-v1`): classifies findings across 4 orthogonal categories
  (`Usability`, `Accessibility`, `GameplayBalance`, `BehavioralModel`), 4 severity tiers (`Blocker`,
  `MajorBarrier`, `MinorFriction`, `PositiveInsight`), and issue-linked dispositions (`Resolved`,
  `Mitigated`, `Deferred`, `DocumentedLimitation`).
- `session.rs` (`m10-participant-session-v1`): defines `ParticipantSessionRecord` tracking
  anonymous participant sessions, declared access needs (`AccessNeedsDeclaration`), completion
  status (`Completed`, `AbandonedAtTurn`, `Inconclusive`), explanation quality, and debrief
  comprehension in exact integer basis points ($[0..=10,000]$ bp).
- `evaluation.rs` (`m10-study-evaluation-v1`): pure deterministic cohort evaluation (`evaluate_study_cohort`)
  producing `StudyEvaluationReport` with overall and per-cohort metrics, finding breakdowns,
  accessibility qualification gate evaluation, and clean Markdown rendering without private chain-of-thought.
- `catalog.rs` (`m10-study-catalog-v1`): registers 3 canonical benchmark study scenarios
  (`scenario-study-cohort-balanced-alpha-v1`, `scenario-study-cohort-access-friction-v1`,
  `scenario-study-cohort-mixed-novice-v1`) with reproducible execution and verified expectations.
- `dimension.rs` (`m10-dimension-assessment-v1`): defines `CognitiveFrictionIndicator`,
  `ParticipantDimensionAssessment`, `evaluate_dimension_assessments`, and `DimensionEvaluationReport`.
- `interaction.rs` (`m10-interaction-mode-v1`): defines `VerbosityLevel`, `ContrastMode`,
  `InteractionProfile`, and `audit_interaction_transcript`.
- `dimension_catalog.rs` (`m10-dimension-catalog-v1`): registers 3 canonical benchmark scenarios
  for dimension assessment and interaction mode auditing.
- `informal_check.rs` (`m10-informal-check-v1`): defines `InformalCheckPhase`, `InformalCheckMode`,
  `NoteDisposition`, `IssueLinkedNote`, and `InformalCheckSession`.
- `remediation.rs` (`m10-remediation-plan-v1`): defines `RemediationTarget`, `RemediationVerificationStatus`,
  `RemediationAction`, `evaluate_remediation_plan`, and `RemediationEvaluationReport`.
- `remediation_catalog.rs` (`m10-remediation-catalog-v1`): registers 3 canonical benchmark scenarios
  (`scenario-remediation-alpha-baseline-v1`, `scenario-remediation-accessibility-priority-v1`,
  `scenario-remediation-mixed-progress-v1`) with reproducible execution and verified expectations.
- `sampling.rs` (`m10-sampling-limits-v1`): defines `UntestedPopulationCategory`, `UntestedPopulationDisclosure`,
  `SamplingLimitsDeclaration`, `AccessNeedsBreakdown`, `CohortRepresentation`, `evaluate_participant_sampling`,
  and `ParticipantSamplingReport` auditing cohort diversity shares and access needs representation.
- `synthesis.rs` (`m10-alpha-synthesis-v1`): defines `AlphaReadinessGateStatus`, `AlphaDisposition`,
  `EmpiricalFactVsInferredHypothesis`, `synthesize_alpha_evidence`, and `AlphaEvidenceSynthesis` synthesizing
  cohort metrics, dimension assessments, interaction audits, remediation plans, and sampling limits.
- `synthesis_catalog.rs` (`m10-synthesis-catalog-v1`): registers 3 canonical benchmark scenarios
  (`scenario-alpha-synthesis-baseline-v1`, `scenario-alpha-synthesis-accessibility-gated-v1`,
  `scenario-alpha-synthesis-sampling-gap-v1`) with reproducible execution and verified expectations.

`src/gui/` defines the presentation need taxonomy, versioned actor-visible GUI Data Transfer Objects,
reversible client state machine, triple projection parity verification, and canonical benchmark scenarios for M11 (governed by ADR-0003 `docs/adr/0003-shared-boundary-gui.md`):
- `need.rs` (`m11-gui-presentation-need-v1`): defines `ComprehensionDomain` (4 cognitive domains
  `SpatialTopology`, `TemporalTimeline`, `ContingencyBranching`, `CausalDebrief`), `DeficitSeverity`,
  `ComprehensionDeficit`, and pure deterministic `evaluate_presentation_need` calculating basis-point
  impacts ($[0..=10,000]$ bp) and evaluating the GUI justification gate ($\ge 4,000$ bp mean or $\ge 5,000$ bp barrier).
- `dto.rs` (`m11-gui-dto-v1`): defines versioned actor-visible GUI DTO models (`GuiMapViewDto`,
  `GuiTimelineViewDto`, `GuiPlanViewDto`, `GuiDebriefViewDto`, `GuiAccessibilityDto`, `GuiPresentationBundle`),
  top-level navigation enums (`GuiActiveTab`, `GuiViewMode`), and strict invariant validation enforcing zero
  latent opponent leakage, zero true-state hashes, and zero private chain-of-thought.
- `projection.rs`: pure deterministic projection builders (`build_gui_map_view`, `build_gui_timeline_view`,
  `build_gui_plan_view`, `build_gui_debrief_view`, `build_gui_accessibility`, `assemble_gui_presentation_bundle`).
- `catalog.rs` (`m11-gui-scenario-catalog-v1`): registers 3 canonical benchmark scenarios
  (`scenario-gui-map-flank-v1`, `scenario-gui-debrief-quadrant-v1`, `scenario-gui-timeline-siege-v1`)
  with reproducible execution and verified expectations.
- `state.rs` (`m11-gui-client-state-v1`): defines the reversible presentation-only GUI client state machine
  (`GuiClientState`), selection states (`GuiSelectionState`), display options (`GuiDisplayOptions`),
  actions (`GuiPresentationAction`), events (`GuiClientEvent`), and fail-closed validation enforcing that
  selections target only actor-visible entities without simulation authority.
- `parity.rs` (`m11-gui-parity-v1`): implements pure deterministic triple projection parity verification
  (`verify_presentation_parity`) validating that CLI observation (`LanerObservation`), MCP protocol DTO
  (`ActorObservationDto`), and GUI bundle (`GuiPresentationBundle`) share exact turn progression, observer role,
  and legal intent sets with zero hash, latent coordinate, or private CoT leakage.
- `state_catalog.rs` (`m11-gui-state-catalog-v1`): registers 3 benchmark client interaction scenarios
  (`scenario-gui-state-map-inspection-v1`, `scenario-gui-state-debrief-quadrant-filter-v1`,
  `scenario-gui-state-reversible-recovery-v1`) with verified expectations.
- `asset.rs` (`m11-gui-asset-governance-v1`): defines asset classifications (`AssetKind`),
  permissive open-source licensing (`AssetLicense`), non-visual/low-overhead fallback rendering rules
  (`AssetFallbackKind`), content hash verification, and pure deterministic auditing (`audit_asset_governance`)
  with fail-closed validation.
- `asset_catalog.rs` (`m11-gui-asset-catalog-v1`): registers 3 canonical benchmark asset governance manifests
  (`scenario-gui-asset-core-v1`, `scenario-gui-asset-minimal-vector-v1`, `scenario-gui-asset-fallback-audit-v1`)
  with verified expectations and reproducible audit execution.
- `html.rs` (`m11-gui-html-v1`): defines the standalone HTML5/CSS/SVG presentation document generator
  (`render_gui_html_document`) and verification engine (`verify_gui_html_document`) with semantic W3C landmarks,
  WCAG 2.1 AA high contrast and reduced-motion tokens, procedural SVG maps, timeline bars, plan cards,
  debrief quadrant breakdowns, and fail-closed security/privacy rules.
- `html_catalog.rs` (`m11-gui-html-catalog-v1`): registers 3 benchmark HTML presentation scenarios
  (`scenario-gui-html-flank-inspection-v1`, `scenario-gui-html-debrief-quadrant-v1`,
  `scenario-gui-html-high-contrast-accessibility-v1`) with verified expectations.

`src/alpha/` defines the public research-capable alpha release governance manifests, cross-version compatibility matrices, data dictionary redaction auditing, limitations/evidence boundaries, documentation guides DAG verification, sample reproducibility bundle packaging, and release readiness verification check suite for M12:
- `governance.rs` (`m12-alpha-governance-v1`): defines `PolicyComplianceArea`, `LegalPostureStatus`, `PolicyDeclaration`, `PublicAlphaGovernanceManifest`, and pure deterministic `evaluate_alpha_governance` auditing compliance basis points ($[0..=10,000]$ bp) and release eligibility.
- `compatibility.rs` (`m12-alpha-compatibility-v1`): defines `CompatibilityDomain`, `CompatibilityLevel`, `VersionMatrixEntry`, `CompatibilityMatrixDefinition`, and pure deterministic `evaluate_compatibility_matrix` verifying cross-version migration contracts.
- `data_dictionary.rs` (`m12-alpha-data-dictionary-v1`): defines `DataCategory`, `DataSensitivityLevel`, `DataFieldDefinition`, `DataDictionaryDefinition`, and pure deterministic `audit_data_dictionary` enforcing fog-of-war redactions on latent state fields.
- `limitations.rs` (`m12-alpha-limitations-v1`): defines `LimitationCategory`, `EvidenceTier`, `ClaimClassification`, `ResearchClaim`, `CitationGuidance`, `AlphaLimitationsDeclaration`, and pure deterministic `audit_limitations_and_boundaries` evaluating claim safety basis points ($[0..=10,000]$ bp).
- `guides.rs` (`m12-alpha-guides-v1`): defines `GuideAudience`, `GuideSectionKind`, `GuideDocumentDefinition`, `AlphaGuidesManifest`, and pure deterministic `audit_guide_manifests` with DFS prerequisite DAG cycle detection and completeness scoring ($[0..=10,000]$ bp).
- `reproducibility.rs` (`m12-alpha-reproducibility-v1`): defines `SampleArtifactKind`, `ReproducibilityStatus`, `ReproducibilityPackageDefinition`, `ReproducibilityBundleManifest`, and pure deterministic `audit_reproducibility_bundle` verifying 16-hex FNV-1a checksums and bundle release eligibility.
- `checks.rs` (`m12-alpha-release-checks-v1`): defines `ReleaseCheckCategory`, `ReleaseCheckSeverity`, `CheckVerificationStatus`, `ReleaseCheckDefinition`, `AlphaReleaseChecksManifest`, and pure deterministic `audit_release_checks` evaluating release readiness scores ($[0..=10,000]$ bp), category summaries, and `is_release_ready` readiness gates.
- `catalog.rs` (`m12-alpha-catalog-v1`): registers 14 canonical benchmark alpha scenarios covering governance compliance, original fallback triggers, compatibility matrix soundness, data dictionary redactions, limitation claim safety, guide prerequisite DAG resolution, reproducibility bundle checksum integrity, and release readiness verification.

## Target Components

These are ownership boundaries; the bounded kernel, fixture codec, and first
one-window lane observation/transition are implemented, while the host and
adapter rows remain target boundaries.

| Component | Owns | Must not own |
| --- | --- | --- |
| Domain model | Typed identifiers, units, state, beliefs, plans, commands, events, effects, ruleset identities | I/O, transport, rendering, provider SDKs |
| Transition kernel | Pure deterministic evaluation invoked by the host: validation checks, coordination/execution resolution from explicit inputs, next-state result, attributed effects | Simulation authority, random generation, wall time, persistence, async tasks |
| Observation/projection | Actor-valid observations, reported uncertainty, legal-action references, debrief projections | Hidden-state leakage, new domain rules |
| Input resolution | Versioned environment, observation, policy, coordination, and execution draws | Mutation of prior state or replay history |
| History/replay | Host-controlled append-only record operations, snapshots, state hashes, verification, and branching policy | Simulation authority or reconstructing authority from runtime logs |
| Scenario/content | Validated compositions of known mechanics and actors | Executable scripts that become a second engine |
| Agent policies | Scripted, heuristic, parametric, LLM-adapter, and adversarial choices from actor-visible inputs | Legality, state transition, privileged truth in ordinary play |
| Application host | Sole simulation authority: true-state lifecycle, legality, window closure, ordering, transition invocation, history/replay commit, debrief generation, and adapter coordination | Provider-specific rules in the core |
| CLI adapter | Keyboard-first commands and actor-visible text | Duplicated legality, transition, or hidden-state inference |
| MCP adapter | Versioned DTOs and model-agnostic actor/controller tools | Internal domain-type compatibility or simulation resolution |
| Persistence | Portable manifests, snapshots, JSONL history, replay bundles, and later indexes | Exclusive opaque storage of authoritative history |
| Experiment/research | Batch manifests, derived metrics, calibration, analytical exports | Mutation of committed histories or claims beyond evidence |
| Optional GUI | Host-projected presentation and reversible local interaction state | Simulation, legality, committed history, or replay authority |

## Authority and Data Flow

Target decision-window flow:

```text
ruleset + prior snapshot
  -> host derives actor-specific observations and legal actions
  -> human/CLI/MCP/agent policies submit messages, plans, and contingencies
  -> host closes the window and validates the submission set
  -> edge resolver supplies explicit stochastic inputs
  -> host invokes the deterministic kernel, which returns events, effects, next state, and hash
  -> host commits the full transition record through history/replay
  -> actor-visible review and debrief projections are derived
  -> persistence and research adapters consume committed artifacts
```

The host may gather independent actor decisions concurrently at the edge. It
must close the window before resolution so one actor cannot observe another's
private uncommitted action. Async collection never makes the transition itself
asynchronous.

The implemented M2 diagnostic flow is the same boundary without an external
adapter:

```text
LaneSnapshot -> observe_player + observe_allied -> proposal/offer
  -> CoordinatedLaneRequest + host validation
  -> explicit coordination and LaneResolvedInputs
  -> transition_lane -> coordinated history append/replay
  -> terminal-objective evaluation/report
  -> named diagnostic fixture inspection
  -> bounded scenario reopen/second-window replay
  -> final committed-facts debrief/report
```

The observation receipts keep source-state bindings private to the host
boundary; actor-visible observations do not contain the true-state hash or
hidden opponent/threat fields. The allied policy is proposal-only. A
coordination overlay composes typed offer/response/resolution provenance around
one unchanged lane transition and state hash. A terminal-objective review is a
post-commit projection over visible result facts and cannot mutate the lane.
Named strategy fixtures are host-input bundles that reuse these contracts and
cannot become a second simulation engine. The two-window wrapper reopens only
a valid resolved result and records that boundary; it does not alter the
one-window transition. A branch borrows and verifies the parent history, then
owns only a copied one-window record and branch metadata; the old branch API
does not silently discard a future coordination overlay.

The current player-lane state carries one `LaneResources` aggregate containing
bounded mana, gold, experience, and cooldown. Execution uses the corresponding
`LaneResourceInputs` aggregate. Full mana and zero values for the other
resources are defaults; resolved changes are applied by the same transition
authority, while player and allied projections expose only authorized
player-laner values. `LaneStatus` stores either `Open` or
`Resolved(LaneOutcome)`, and `LaneDelay` prevents zero-beat effects.

Together with bounded `LanePosition`, `LaneHealth`, and `WavePressure`, these
types are the minimum state abstractions for the current diagnostic window.
They are host-owned and represented in the snapshot, state hash, and replay.
Actor projections expose only authorized player fields and bounded reports;
explicit inputs carry resolved damage, wave, and resource changes, while
position follows authoritative intent/fallback evaluation, health follows
validated damage/delayed-effect resolution, and terminal outcome is evaluated
from the resulting values. They are not a complete economy or balance model.

The player projection also applies one fixed FarSide opponent sighting rule;
health/posture and allied opponent reports remain hidden, and complete vision
or belief state has not been added. Both player and allied projections carry the
fixed `LaneActorRoster` role/identity metadata, including the abstract opposing
jungle threat identity; this metadata is not mutable lane state and does not
participate in the authoritative state hash.

The bounded intent surface is carried by typed request/command fields for
intent, commitment, target focus, ping signal, abort condition, and fallback.
Observation and validation bind those fields to actor-visible receipts; the ping
signal is communication metadata rather than a free-form message transport.

The current v3 causal path preserves effect relation/timing labels and each
delayed effect's originating execution trace through ordered projection and
bounded delayed resolution, state hashing, and replay identity. `LaneOutcome`
and objective review remain separate read models, and the complete two-window
replay/debrief path is inspected and verified without exposing hidden state.

Team communication contracts in M8 (`m8-team-communication-v1`, `m8-team-dialogue-v1`,
`m8-team-plan-v1`, `m8-team-plan-relationship-v1`, `m8-team-trust-v1`, `m8-caller-reputation-v1`,
`m8-communication-channel-v1`, `m8-leadership-structure-v1`, `m8-shot-caller-policy-v1`,
`m8-decentralized-coordination-v1`, `m8-leadership-evaluation-report-v1`,
`m8-team-simultaneous-submission-v1`, `m8-team-simultaneous-resolution-v1`,
`m8-team-simultaneous-catalog-v1`) define structured speech acts, visibility boundaries,
multi-turn dialogue state machines, structured team plans, deterministic individual/team
alignment evaluation, caller reputation tracking, transmission channel physics, designated
shot-caller directives, decentralized peer consensus arbitration, and simultaneous multi-agent
submission collection and resolution. All message envelopes, dialogues, plans, leadership
proposals, submissions, and resolution reports strictly enforce zero private chain-of-thought,
and evaluations operate purely over actor-safe projections with exact integer basis-point scoring
($[0..=10,000]$ bp), redacted debug views during collection, and pure deterministic resolution.

## Consequential Type Boundaries

Future types and public contracts must preserve these distinctions:

- `TrueState` versus `BeliefState` versus `Observation` versus `Report`;
- proposal versus commitment;
- strategic intent versus coordination versus mechanical execution;
- message versus plan versus authoritative command;
- invalid command versus legal action with an unfavorable outcome;
- environment, observation, policy, communication, coordination, and execution
  uncertainty;
- domain event versus attributed effect;
- committed history versus operational diagnostics;
- ordinary actor authority versus privileged experiment-controller authority;
- internal domain type versus versioned external DTO.

Names may evolve, but future code must not erase the semantic boundaries merely
to reduce type count.

## Determinism and Randomness

- The transition receives resolved values and never constructs an RNG.
- Each stochastic category uses stable stream and draw identities.
- Adding an unrelated draw must not shift later values in another stream.
- Floating-point values that affect authoritative equality or hashes require a
  declared normalization, ordering, or fixed-point representation.
- Collections that affect event ordering or hashes require stable ordering.
- The current fixture hashes ruleset, turn, actor ID, energy, and score with
  64-bit FNV-1a over little-endian integer bytes in that field order. A later
  hash-representation change requires a versioned compatibility decision.
- Replay verifies both transitions and hashes; it does not trust the terminal
  snapshot alone.
- Counterfactual branches record which exogenous inputs are reused, remapped, or
  regenerated.

## Information and Causality

- Actors choose from observations, beliefs, messages, and memory available to
  their represented role.
- CLI projections preserve whether a value is observed, believed, inferred,
  reported, or unknown. `unknown` is a payload-free redaction rather than a
  value that happens to carry an unknown label.
- Terminal presentation remains an outer adapter concern; rendering must not
  become a second transition authority or a source of hidden-state inference.
- Research inspection may expose true state only through a separately authorized
  interface and must not contaminate playable policies or metrics.
- Debriefs evaluate decisions using information available at decision time.
- Current effects expose direct/indirect and immediate/delayed vocabulary while
  retaining their existing cause/trace attribution. A bounded delayed-effect
  queue is implemented; broader causal chains and stochastic provenance remain
  open.
- A good decision may fail and a poor decision may succeed; the model and
  presentation must support that distinction.

## Persistence and Compatibility

The planned early persistence strategy is artifact-first:

```text
runs/<run-id>/
├── manifest.json
├── initial-state.json
├── history.jsonl
├── snapshots/
├── replay-hashes.json
├── metrics.json
└── debrief.md
```

This layout is not implemented. Before it becomes authoritative, M1/M2 must
version the manifest, state, history, ruleset, and scenario schemas and define
fixture compatibility. SQLite may later index runs, and Parquet may store
derived analytical tables, but neither should become the sole authoritative
history format.

## Dependency Direction

The intended dependency direction is inward:

```text
CLI / MCP / optional GUI / research / persistence
                  -> application host
                  -> projections and validated commands
                  -> domain model and deterministic kernel
```

Core domain code must not depend outward on adapter, storage, async-runtime,
terminal, HTTP, model-provider, or analytical concerns. Provider integrations
implement agent-policy boundaries; they do not define them.

## Technology Decisions

Verified today:

- Rust toolchain `1.96.0`, pinned in `rust-toolchain.toml`;
- Rust edition 2024;
- Cargo binary package;
- package license metadata set to MIT;
- no third-party dependencies.
- `scripts/check_repository.py` scans the deterministic core modules for async
  syntax/runtime imports, wall-clock imports, and network transport types; its
  focused tests keep those concerns at the adapter edge.

Proposed but not adopted:

- additional Cargo workspace boundaries; ADR-0002 keeps M1 in one package;
- Serde/JSON and explicit seeded RNG at edges;
- Clap or a small interactive shell;
- Tokio and the official Rust MCP SDK at adapter boundaries;
- artifact-first JSON/JSONL persistence;
- Python/uv, Parquet, and DuckDB for later research;
- Axum plus a web client for an evidence-justified optional GUI.

Adopting one of these choices requires an implementation need, focused tests,
and an architecture update or ADR when it changes a consequential boundary.

## Architectural Constraints

1. Build vertical slices before general frameworks.
2. One host owns simulation authority across every interface.
3. Randomness is explicit data at the deterministic boundary.
4. Actor-visible interfaces fail closed against hidden-state leakage.
5. Committed history is append-only and operational logs are non-authoritative.
6. Replay, schema, ruleset, scenario, prompt, and agent-profile versions are
   recorded when they can affect reproducibility.
7. AI-agent playtests do not establish human experience or behavior.
8. Scenario data composes known mechanics; arbitrary executable content is
   deferred until a concrete need outweighs the second-engine risk.
9. CLI remains a first-class reference interface even if a GUI is added.
10. No future adapter may silently become an alternative simulation engine.

## Known Gaps

- The M1 kernel/codec and M2 v3 lane decision-window, branch, coordination,
  objective, strategy-fixture, two-window, final-debrief, retained-resource,
  intent, and
  observation contracts are implemented internally, but they are not a
  playable scenario, external API, migration framework, or persistence service.
- M3 has typed command contracts, a bounded host fixture, replay-validated
  artifacts, an injected file store, a pure terminal-text projection, and a
  thin line-oriented fixture loop with explicit versioned fixture selection,
  `--run-dir` wiring, a matched-parent host branch projection, and
  machine-checked labeled plain text, process-edge package version reporting,
  and one executable complete-transcript regression; interactive and dynamic scenario selection,
  regenerated/graph branching, and human accessibility evidence remain open.
- M2 still lacks a communication system, full vision geometry, memory decay,
  automatic threat damage, no-choice host scheduling, adaptive pacing, a complete item/resource economy,
  external scenario serialization, a branch tree, and a broader debrief
  surface. The retired v1 bounty, level, minion-kills, shield, ward, and
  consumable slices are historical evidence only.
- Richer external replay bundles and scenario-specific schema fields remain
  open work.
- `.github/workflows/ci.yml` and `scripts/check_repository.py` now define the
  formatting, lint, test, metadata, link, currentness, and dependency-free
  package guard; PR #4's hosted run passed and supports M0 promotion. Future
  changes still require the workflow to pass again.
- No automated advisory/license scanner is configured for a future non-empty
  dependency graph; the current guard blocks dependency additions until the
  approved scanner and its policy are added or a complete machine-readable defer
  record is bound to the exact dependency identity.
- Implementation-backed schema, accessibility, and research governance remain
  incomplete and are tracked in M1 and later roadmap gates. Repository policy
  and the initial authority ADR now exist, but they do not establish legal
  clearance or shipped simulation capability.
