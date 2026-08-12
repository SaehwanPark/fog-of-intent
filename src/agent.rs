//! Actor-visible scripted-agent policy for the M4 baseline.
//!
//! The policy consumes only a [`crate::lane::LanerObservation`]. It generates
//! legal candidates from that observation, evaluates them with a versioned
//! fixed score table, and returns a request for the host to validate. Its
//! default path is deterministic; an opt-in seeded path resolves equal-score
//! ties from an explicit policy bundle. It never reads true state, resolves
//! execution inputs, or owns a transition.

use std::hash::Hasher;

use crate::kernel::{ActorId, DrawId, InputTrace, StreamId};
use crate::lane::{
  HiddenValue, JungleThreatRegion, JungleThreatTruth, LaneAbortCondition, LaneActorRole,
  LaneCommitment, LaneFallbackBehavior, LaneIntent, LaneIntentRequest, LanePingSignal,
  LanePosition, LaneSnapshot, LaneStatus, LaneTargetFocus, LanerObservation, ObservationId,
  observe_player,
};

/// Versioned identity for the first scripted-agent policy boundary.
pub const SCRIPTED_AGENT_SCHEMA: &str = "m4-scripted-agent-v1";

/// Stable profile identity for the cautious deterministic baseline.
pub const SCRIPTED_AGENT_PROFILE_ID: &str = "cautious-laner-v1";

/// Stable profile identity for the risk-taking deterministic comparison.
pub const RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID: &str = "risk-taking-laner-v1";

/// Stable profile identity for the yielding deterministic comparison.
pub const YIELDING_SCRIPTED_AGENT_PROFILE_ID: &str = "yielding-laner-v1";

/// Versioned actor-safe profile-comparison metric schema.
pub const SCRIPTED_AGENT_METRICS_SCHEMA: &str = "m4-scripted-agent-metrics-v1";

/// Versioned bounded selected-action tally schema.
pub const SCRIPTED_AGENT_ACTION_TALLY_SCHEMA: &str = "m4-scripted-agent-action-tally-v2";

/// Versioned identity for the explicit policy seed bundle contract.
pub const SCRIPTED_AGENT_RANDOMNESS_SCHEMA: &str = "m4-scripted-agent-random-v1";

/// Stable identity for seeded top-1 tie resolution.
pub const SCRIPTED_AGENT_SEEDED_SELECTION_RULE: &str = "max-score-seeded-tie-v1";

/// Versioned identity for actor-visible scripted-decision replay records.
pub const SCRIPTED_AGENT_REPLAY_SCHEMA: &str = "m4-scripted-agent-replay-v1";

/// Versioned experiment-manifest identity for the bounded scripted fixture.
pub const SCRIPTED_AGENT_EXPERIMENT_MANIFEST_SCHEMA: &str = "m6-experiment-manifest-v1";

/// The only scenario currently admitted by the bounded manifest.
pub const SCRIPTED_AGENT_EXPERIMENT_SCENARIO_ID: &str = "m3-two-window-fixture-v1";

/// Versioned identity for the applicable M6 experiment-version catalog.
pub const SCRIPTED_AGENT_EXPERIMENT_VERSION_CATALOG_SCHEMA: &str =
  "m6-experiment-version-catalog-v1";

/// Stable ruleset label recorded by the current scripted fixture catalog.
pub const SCRIPTED_AGENT_EXPERIMENT_RULESET_ID: &str = "m2-lane-ruleset-v4";

/// Explicit marker for experiment integrations not present in this slice.
pub const SCRIPTED_AGENT_VERSION_NOT_APPLICABLE: &str = "not-applicable";

/// Maximum encoded experiment-manifest size.
pub const MAX_SCRIPTED_AGENT_MANIFEST_BYTES: usize = 4096;

/// Maximum number of manifests evaluated by one in-process batch.
pub const MAX_SCRIPTED_AGENT_BATCH_MANIFESTS: usize = 16;

/// Versioned identity for a bounded resumable batch cursor.
pub const SCRIPTED_AGENT_BATCH_RUN_SCHEMA: &str = "m6-scripted-agent-batch-run-v1";

/// Maximum encoded checkpoint size before parsing or allocation.
pub const MAX_SCRIPTED_AGENT_BATCH_RUN_BYTES: usize = 4096;

/// Versioned identity for caller-declared bounded run dispositions.
pub const SCRIPTED_AGENT_RUN_DISPOSITION_SCHEMA: &str = "m6-scripted-agent-run-disposition-v1";

/// Maximum encoded run-disposition size before parsing or allocation.
pub const MAX_SCRIPTED_AGENT_RUN_DISPOSITION_BYTES: usize = 4096;

/// Versioned identity for the bounded matched-observation sample report.
pub const SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA: &str = "m6-scripted-agent-matched-sample-v1";

/// Versioned identity for the bounded matched-scenario sample set.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLE_SCHEMA: &str =
  "m6-scripted-agent-matched-scenarios-v1";

/// Versioned identity for the closed fixture-scenario catalog.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA: &str =
  "m6-scripted-agent-fixture-scenarios-v1";

/// Stable ID for the no-threat fixture variant.
pub const SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID: &str = "safe-fixture-v1";

/// Stable ID for the visible RiverSide-threat fixture variant.
pub const SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID: &str = "river-side-threat-v1";

/// Versioned identity for bounded fixed-fixture scenario-frequency evidence.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA: &str =
  "m6-scripted-agent-fixture-frequency-v1";

/// Maximum encoded scenario-frequency report size before parsing/allocation.
pub const MAX_SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_BYTES: usize = 4096;

/// Integer basis-point scale for the bounded scenario distribution projection.
pub const SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE: u16 = 10_000;

/// Versioned identity for the bounded caller-declared stress-case matrix.
pub const SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA: &str = "m6-scripted-agent-stress-population-v1";

/// Versioned identity for bounded caller-declared degenerate-policy evidence.
pub const SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA: &str =
  "m6-scripted-agent-degenerate-policy-population-v1";

/// Maximum observations in one fixed degenerate-policy population.
pub const MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION: usize = 4;

/// Versioned identity for bounded fixed-fixture exploit-seeking evidence.
pub const SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA: &str =
  "m6-scripted-agent-exploit-seeking-population-v1";

/// Maximum observations in one fixed exploit-seeking policy population.
pub const MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION: usize = 4;

/// Versioned identity for bounded fixed-fixture frequency baseline comparisons.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_COMPARISON_SCHEMA: &str =
  "m6-scripted-agent-fixture-frequency-compare-v1";

/// Stable identity for the fixed-fixture no-change regression gate.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_REGRESSION_RULE: &str =
  "m6-fixed-frequency-no-change-v1";

/// Versioned identity for the bounded largest-delta candidate projection.
pub const SCRIPTED_AGENT_TALLY_OUTLIER_CANDIDATE_SCHEMA: &str =
  "m6-scripted-agent-tally-outlier-candidate-v1";

/// Stable identity for the bounded largest-absolute-delta selection rule.
pub const SCRIPTED_AGENT_TALLY_OUTLIER_CANDIDATE_RULE: &str = "m6-largest-absolute-intent-delta-v1";

/// Versioned identity for a provisional fixed-fixture outlier threshold signal.
pub const SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_SCHEMA: &str =
  "m6-scripted-agent-tally-outlier-threshold-v1";

/// Stable identity for the provisional fixed-fixture threshold rule.
pub const SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_RULE: &str =
  "m6-fixed-intent-delta-outlier-threshold-v1";

/// Provisional inclusive magnitude threshold over signed intent-count deltas.
pub const SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_MAGNITUDE: u16 = 2;

/// Versioned identity for a caller-declared candidate replay reference.
pub const SCRIPTED_AGENT_TALLY_REPLAY_REFERENCE_SCHEMA: &str =
  "m6-scripted-agent-tally-replay-reference-v1";

/// Stable identity for first matching verified replay selection.
pub const SCRIPTED_AGENT_TALLY_REPLAY_REFERENCE_RULE: &str =
  "m6-first-verified-candidate-replay-v1";

/// Versioned identity for a calibrated outlier and representative replay report.
pub const SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA: &str =
  "m6-scripted-agent-calibrated-outlier-replay-v1";

/// Versioned selection rule for calibrated outlier and representative replay tracing.
pub const SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE: &str =
  "m6-calibrated-outlier-representative-replay-v1";

/// Calibrated threshold magnitude for classifying a tally delta as an outlier.
pub const SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE: u16 = 2;

/// Versioned identity for caller-declared build labels on comparisons.
pub const SCRIPTED_AGENT_BUILD_ID_SCHEMA: &str = "m6-scripted-agent-build-id-v1";

/// Versioned identity for the non-authoritative operational event vocabulary.
pub const SCRIPTED_AGENT_OPERATIONAL_EVENT_SCHEMA: &str = "m6-scripted-agent-operational-event-v1";

/// Versioned identity for the bounded operational-log codec.
pub const SCRIPTED_AGENT_OPERATIONAL_LOG_SCHEMA: &str = "m6-scripted-agent-operational-log-v1";

/// Versioned identity for bounded operational-log sequence status.
pub const SCRIPTED_AGENT_OPERATIONAL_LOG_SEQUENCE_SCHEMA: &str =
  "m6-scripted-agent-operational-log-sequence-v1";

/// Stable identity for the required operational lifecycle sequence.
pub const SCRIPTED_AGENT_OPERATIONAL_LOG_SEQUENCE_RULE: &str =
  "m6-operational-start-chunk-finish-v1";

/// Versioned identity for bounded decision-replay and sequence evidence.
pub const SCRIPTED_AGENT_REPLAY_SEQUENCE_EVIDENCE_SCHEMA: &str =
  "m6-scripted-agent-replay-sequence-evidence-v1";

/// Stable identity for the bounded replay/sequence evidence rule.
pub const SCRIPTED_AGENT_REPLAY_SEQUENCE_EVIDENCE_RULE: &str =
  "m6-replay-identity-operational-sequence-v1";

/// Versioned identity for scenario-wide replay identity evidence.
pub const SCRIPTED_AGENT_SCENARIO_REPLAY_IDENTITY_SCHEMA: &str =
  "m6-scripted-agent-scenario-replay-identity-v1";

/// Stable identity for the scenario replay identity rule.
pub const SCRIPTED_AGENT_SCENARIO_REPLAY_IDENTITY_RULE: &str = "m6-scenario-replay-identity-v1";

/// Versioned identity for scenario-wide causal-trace completeness evidence.
pub const SCRIPTED_AGENT_SCENARIO_CAUSAL_TRACE_COMPLETENESS_SCHEMA: &str =
  "m6-scripted-agent-scenario-causal-trace-completeness-v1";

/// Stable identity for the scenario causal-trace completeness rule.
pub const SCRIPTED_AGENT_SCENARIO_CAUSAL_TRACE_COMPLETENESS_RULE: &str =
  "m6-scenario-causal-trace-completeness-v1";

/// Versioned schema for the compact semantic profile vocabulary.
pub const SEMANTIC_PROFILE_VOCABULARY_SCHEMA: &str = "m7-semantic-profile-vocabulary-v1";

/// Stable identifier for the cautious reference semantic profile.
pub const CAUTIOUS_SEMANTIC_PROFILE_ID: &str = "cautious-laner-semantic-v1";

/// Stable identifier for the risk-taking reference semantic profile.
pub const RISK_TAKING_SEMANTIC_PROFILE_ID: &str = "risk-taking-laner-semantic-v1";

/// Stable identifier for the yielding reference semantic profile.
pub const YIELDING_SEMANTIC_PROFILE_ID: &str = "yielding-laner-semantic-v1";

/// Versioned schema for the diagnostic choice catalog.
pub const DIAGNOSTIC_CHOICE_CATALOG_SCHEMA: &str = "m7-diagnostic-choice-catalog-v1";

/// Stable identifier for the contest/concede diagnostic choice.
pub const CHOICE_CONTEST_CONCEDE_ID: &str = "choice-contest-concede-v1";

/// Stable identifier for the follow/reject diagnostic choice.
pub const CHOICE_FOLLOW_REJECT_ID: &str = "choice-follow-reject-v1";

/// Stable identifier for the farm/assist diagnostic choice.
pub const CHOICE_FARM_ASSIST_ID: &str = "choice-farm-assist-v1";

/// Stable identifier for the recall timing diagnostic choice.
pub const CHOICE_RECALL_TIMING_ID: &str = "choice-recall-timing-v1";

/// Stable identifier for the sacrifice diagnostic choice.
pub const CHOICE_SACRIFICE_ID: &str = "choice-sacrifice-v1";

/// Stable identifier for the surprise diagnostic choice.
pub const CHOICE_SURPRISE_ID: &str = "choice-surprise-v1";

/// Stable identifier for the response to failure diagnostic choice.
pub const CHOICE_RESPONSE_TO_FAILURE_ID: &str = "choice-response-to-failure-v1";

/// Versioned schema for the model and prompt configuration protocol.
pub const MODEL_PROMPT_PROTOCOL_SCHEMA: &str = "m7-model-prompt-protocol-v1";

/// Stable identifier for the reference standard model-prompt protocol.
pub const MODEL_PROMPT_REFERENCE_STANDARD_ID: &str = "model-prompt-reference-standard-v1";

/// Stable identifier for the reference diagnostic model-prompt protocol.
pub const MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID: &str = "model-prompt-reference-diagnostic-v1";

/// Stable identifier for the alternative diagnostic model-prompt protocol.
pub const MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID: &str = "model-prompt-alternative-diagnostic-v1";

/// Versioned schema for the repeated sampling protocol.
pub const REPEATED_SAMPLING_PROTOCOL_SCHEMA: &str = "m7-repeated-sampling-protocol-v1";

/// Stable identifier for the standard 10-repeat sampling protocol.
pub const SAMPLING_STANDARD_REPEAT_10_ID: &str = "sampling-standard-repeat-10-v1";

/// Stable identifier for the diagnostic 30-repeat sampling protocol.
pub const SAMPLING_DIAGNOSTIC_REPEAT_30_ID: &str = "sampling-diagnostic-repeat-30-v1";

/// Stable identifier for the quick 5-repeat check sampling protocol.
pub const SAMPLING_QUICK_CHECK_5_ID: &str = "sampling-quick-check-5-v1";

/// Versioned schema for the diagnostic choice empirical distribution estimation report.
pub const EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA: &str =
  "m7-empirical-distribution-estimation-v1";

/// Versioned schema for diagnostic choice action distributions.
pub const EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA: &str = "m7-empirical-action-distribution-v1";

/// Versioned schema for diagnostic choice communication distributions.
pub const EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA: &str =
  "m7-empirical-communication-distribution-v1";

/// Basis-point scale for empirical distribution estimation (10,000 basis points = 100.00%).
pub const EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS: u16 = 10_000;

/// Versioned schema for the diagnostic choice behavioral measures report.
pub const BEHAVIORAL_MEASURES_SCHEMA: &str = "m7-behavioral-measures-v1";

/// Versioned schema for behavioral distance measures (Total Variation Distance).
pub const BEHAVIORAL_DISTANCE_SCHEMA: &str = "m7-behavioral-distance-v1";

/// Versioned schema for behavioral entropy measures (Gini diversity index).
pub const BEHAVIORAL_ENTROPY_SCHEMA: &str = "m7-behavioral-entropy-v1";

/// Versioned schema for behavioral dilemma sensitivity measures.
pub const BEHAVIORAL_SENSITIVITY_SCHEMA: &str = "m7-behavioral-sensitivity-v1";

/// Versioned schema for repeated-sampling consistency measures.
pub const BEHAVIORAL_CONSISTENCY_SCHEMA: &str = "m7-behavioral-consistency-v1";

/// Versioned schema for adverse-condition adaptation measures.
pub const BEHAVIORAL_ADAPTATION_SCHEMA: &str = "m7-behavioral-adaptation-v1";

/// Maximum number of replay records evaluated in one scenario-wide identity check.
pub const MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS: usize = 16;

/// Maximum number of operational events retained in one in-memory log.
pub const MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS: usize = 16;

/// Maximum encoded size of one operational log.
pub const MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES: usize = 4096;

/// Versioned identity for bounded matched-scenario selected-intent tallies.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_SCHEMA: &str =
  "m6-scripted-agent-matched-scenario-tally-v1";

/// Versioned identity for bounded comparisons of verified profile-aware tallies.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA: &str =
  "m6-scripted-agent-matched-scenario-tally-compare-v1";

/// Maximum encoded profile-aware tally comparison size before parsing.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_BYTES: usize = 4096;

/// Stable identity for the profile-aware fixed-fixture equality gate.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_REGRESSION_RULE: &str =
  "m6-fixed-profile-tally-no-change-v1";

/// Maximum encoded matched-scenario tally size before parsing or allocation.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_BYTES: usize = 4096;

/// Maximum number of caller-supplied matched pairs in one sample set.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES: usize = 4;

/// Maximum number of selected fixed-fixture scenarios in one request.
pub const MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS: usize = 4;

/// Versioned identity for a deterministic fixed-fixture population.
pub const SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA: &str =
  "m6-scripted-agent-fixture-population-v1";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScriptedAgentEvaluationRule {
  Threat,
  Contest,
  Yield,
}

/// Transparent policy posture labels, distinct from the scenario actor roster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentRole {
  Anchor,
  Duelist,
  Pacer,
}

impl ScriptedAgentRole {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Anchor => "anchor-v1",
      Self::Duelist => "duelist-v1",
      Self::Pacer => "pacer-v1",
    }
  }
}

/// Versioned profile and policy-rule metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentProfile {
  profile_id: &'static str,
  candidate_rule: &'static str,
  evaluation_rule: &'static str,
  selection_rule: &'static str,
  evaluation: ScriptedAgentEvaluationRule,
  role: ScriptedAgentRole,
}

impl ScriptedAgentProfile {
  /// Return the first cautious baseline profile.
  pub const fn cautious_v1() -> Self {
    Self {
      profile_id: SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "threat-first-pressure-aware-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Threat,
      role: ScriptedAgentRole::Anchor,
    }
  }

  /// Return the risk-taking matched-input comparison profile.
  pub const fn risk_taking_v1() -> Self {
    Self {
      profile_id: RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "contest-first-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Contest,
      role: ScriptedAgentRole::Duelist,
    }
  }

  /// Return the yielding matched-input comparison profile.
  pub const fn yielding_v1() -> Self {
    Self {
      profile_id: YIELDING_SCRIPTED_AGENT_PROFILE_ID,
      candidate_rule: "actor-visible-intents-v1",
      evaluation_rule: "yield-first-fixed-score-v1",
      selection_rule: "max-score-stable-order-v1",
      evaluation: ScriptedAgentEvaluationRule::Yield,
      role: ScriptedAgentRole::Pacer,
    }
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn candidate_rule(self) -> &'static str {
    self.candidate_rule
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn selection_rule(self) -> &'static str {
    self.selection_rule
  }

  pub const fn role(self) -> ScriptedAgentRole {
    self.role
  }

  /// Return the profile's fixed baseline preference before visible-threat
  /// overrides are considered.
  pub const fn preferred_intent(self) -> LaneIntent {
    match self.evaluation {
      ScriptedAgentEvaluationRule::Threat => LaneIntent::Stabilize,
      ScriptedAgentEvaluationRule::Contest => LaneIntent::Contest,
      ScriptedAgentEvaluationRule::Yield => LaneIntent::Yield,
    }
  }

  fn parse_id(value: &str) -> Result<Self, ScriptedAgentManifestError> {
    match value {
      SCRIPTED_AGENT_PROFILE_ID => Ok(Self::cautious_v1()),
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID => Ok(Self::risk_taking_v1()),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID => Ok(Self::yielding_v1()),
      _ => Err(ScriptedAgentManifestError::InvalidValue),
    }
  }
}

/// Explicit policy-only seed and stream/draw identity.
///
/// The caller owns this bundle and must retain it with any seeded decision if
/// the decision is to be reproduced later. It is never derived from true
/// state, a clock, or an implicit global random generator.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentSeedBundle {
  seed: u64,
  policy_trace: InputTrace,
}

impl ScriptedAgentSeedBundle {
  pub fn new(seed: u64, policy_stream: StreamId, policy_draw: DrawId) -> Self {
    Self {
      seed,
      policy_trace: InputTrace::new(policy_stream, policy_draw),
    }
  }

  pub const fn schema(self) -> &'static str {
    SCRIPTED_AGENT_RANDOMNESS_SCHEMA
  }

  pub const fn seed(self) -> u64 {
    self.seed
  }

  pub const fn policy_trace(self) -> InputTrace {
    self.policy_trace
  }

  fn tie_index(self, upper_bound: usize) -> usize {
    assert!(upper_bound > 0, "seeded tie selection requires a candidate");
    let mut value = self.seed
      ^ (u64::from(self.policy_trace.stream().value()) << 32)
      ^ u64::from(self.policy_trace.draw().value());
    value = value.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^= value >> 31;
    let bound = u64::try_from(upper_bound).expect("candidate count fits in u64");
    usize::try_from(value % bound).expect("tie index fits in usize")
  }
}

/// Bounded failures for the versioned experiment-manifest codec.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentManifestError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
}

/// Reproducibility metadata for one bounded scripted-agent fixture run.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentExperimentManifest {
  schema: &'static str,
  scenario_id: &'static str,
  profile: ScriptedAgentProfile,
  seed_bundle: ScriptedAgentSeedBundle,
}

impl ScriptedAgentExperimentManifest {
  /// Construct a manifest for the current versioned two-window fixture.
  pub const fn new(profile: ScriptedAgentProfile, seed_bundle: ScriptedAgentSeedBundle) -> Self {
    Self {
      schema: SCRIPTED_AGENT_EXPERIMENT_MANIFEST_SCHEMA,
      scenario_id: SCRIPTED_AGENT_EXPERIMENT_SCENARIO_ID,
      profile,
      seed_bundle,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn profile(self) -> ScriptedAgentProfile {
    self.profile
  }

  pub const fn seed_bundle(self) -> ScriptedAgentSeedBundle {
    self.seed_bundle
  }

  pub const fn selection_rule(self) -> &'static str {
    SCRIPTED_AGENT_SEEDED_SELECTION_RULE
  }

  /// Encode reproducibility metadata without observations or experiment output.
  pub fn encode(self) -> String {
    let trace = self.seed_bundle.policy_trace();
    format!(
      "schema={}\nscenario={}\nprofile={}\nevaluation_rule={}\nselection_rule={}\nseed={}\npolicy_stream={}\npolicy_draw={}\n",
      self.schema,
      self.scenario_id,
      self.profile.profile_id(),
      self.profile.evaluation_rule(),
      self.selection_rule(),
      self.seed_bundle.seed(),
      trace.stream().value(),
      trace.draw().value(),
    )
  }

  /// Decode a manifest without constructing an agent or running an experiment.
  pub fn decode(input: &str) -> Result<Self, ScriptedAgentManifestError> {
    if input.len() > MAX_SCRIPTED_AGENT_MANIFEST_BYTES {
      return Err(ScriptedAgentManifestError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() > 8 {
      return Err(ScriptedAgentManifestError::UnexpectedLineCount {
        expected: 8,
        actual: lines.len(),
      });
    }
    let mut fields = Vec::with_capacity(8);
    for line in lines {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentManifestError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentManifestError::InvalidValue);
      }
      fields.push((key, value));
    }
    let mut schema = None;
    let mut scenario = None;
    let mut profile = None;
    let mut evaluation_rule = None;
    let mut selection_rule = None;
    let mut seed = None;
    let mut policy_stream = None;
    let mut policy_draw = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "scenario" => &mut scenario,
        "profile" => &mut profile,
        "evaluation_rule" => &mut evaluation_rule,
        "selection_rule" => &mut selection_rule,
        "seed" => &mut seed,
        "policy_stream" => &mut policy_stream,
        "policy_draw" => &mut policy_draw,
        _ => return Err(ScriptedAgentManifestError::UnknownField),
      };
      if slot.is_some() {
        return Err(ScriptedAgentManifestError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(SCRIPTED_AGENT_EXPERIMENT_MANIFEST_SCHEMA) {
      return Err(ScriptedAgentManifestError::UnsupportedSchema);
    }
    if scenario != Some(SCRIPTED_AGENT_EXPERIMENT_SCENARIO_ID) {
      return Err(ScriptedAgentManifestError::InvalidValue);
    }
    let profile =
      ScriptedAgentProfile::parse_id(profile.ok_or(ScriptedAgentManifestError::MissingField)?)?;
    if evaluation_rule != Some(profile.evaluation_rule())
      || selection_rule != Some(SCRIPTED_AGENT_SEEDED_SELECTION_RULE)
    {
      return Err(ScriptedAgentManifestError::InvalidValue);
    }
    let seed = seed
      .ok_or(ScriptedAgentManifestError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ScriptedAgentManifestError::InvalidValue)?;
    let policy_stream = policy_stream
      .ok_or(ScriptedAgentManifestError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentManifestError::InvalidValue)?;
    let policy_draw = policy_draw
      .ok_or(ScriptedAgentManifestError::MissingField)?
      .parse::<u16>()
      .map_err(|_| ScriptedAgentManifestError::InvalidValue)?;
    Ok(Self::new(
      profile,
      ScriptedAgentSeedBundle::new(seed, StreamId::new(policy_stream), DrawId::new(policy_draw)),
    ))
  }
}

/// Fixed version identities applicable to the deterministic M6 fixture.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentExperimentVersionCatalog {
  schema: &'static str,
  ruleset_id: &'static str,
  scenario_id: &'static str,
  policy_schema: &'static str,
  profile_ids: [&'static str; 3],
  prompt_version: &'static str,
  tool_schema_version: &'static str,
  model_version: &'static str,
  extractor_version: &'static str,
}

impl ScriptedAgentExperimentVersionCatalog {
  /// Return the fixed catalog for the current deterministic fixture.
  pub const fn current() -> Self {
    Self {
      schema: SCRIPTED_AGENT_EXPERIMENT_VERSION_CATALOG_SCHEMA,
      ruleset_id: SCRIPTED_AGENT_EXPERIMENT_RULESET_ID,
      scenario_id: SCRIPTED_AGENT_EXPERIMENT_SCENARIO_ID,
      policy_schema: SCRIPTED_AGENT_SCHEMA,
      profile_ids: [
        SCRIPTED_AGENT_PROFILE_ID,
        RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
        YIELDING_SCRIPTED_AGENT_PROFILE_ID,
      ],
      prompt_version: SCRIPTED_AGENT_VERSION_NOT_APPLICABLE,
      tool_schema_version: SCRIPTED_AGENT_VERSION_NOT_APPLICABLE,
      model_version: SCRIPTED_AGENT_VERSION_NOT_APPLICABLE,
      extractor_version: SCRIPTED_AGENT_VERSION_NOT_APPLICABLE,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn ruleset_id(self) -> &'static str {
    self.ruleset_id
  }

  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn policy_schema(self) -> &'static str {
    self.policy_schema
  }

  pub const fn profile_ids(self) -> [&'static str; 3] {
    self.profile_ids
  }

  pub const fn prompt_version(self) -> &'static str {
    self.prompt_version
  }

  pub const fn tool_schema_version(self) -> &'static str {
    self.tool_schema_version
  }

  pub const fn model_version(self) -> &'static str {
    self.model_version
  }

  pub const fn extractor_version(self) -> &'static str {
    self.extractor_version
  }
}

/// Bounded failures from the deterministic local batch runner.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBatchError {
  EmptyBatch,
  BatchTooLarge { max: usize, actual: usize },
}

/// Bounded failures from checkpoint encoding and decoding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBatchCheckpointError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
}

/// Bounded failures from chunked batch execution and checkpoint matching.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBatchRunError {
  Batch(ScriptedAgentBatchError),
  InputMismatch,
  ChunkSizeZero,
}

/// Closed caller-declared outcomes for one bounded experiment run.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentRunDisposition {
  Completed,
  Crashed,
  TimedOut,
  MissingBranch,
  Inconclusive,
}

impl ScriptedAgentRunDisposition {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Completed => "completed",
      Self::Crashed => "crashed",
      Self::TimedOut => "timed_out",
      Self::MissingBranch => "missing_branch",
      Self::Inconclusive => "inconclusive",
    }
  }

  fn parse_id(value: &str) -> Option<Self> {
    match value {
      "completed" => Some(Self::Completed),
      "crashed" => Some(Self::Crashed),
      "timed_out" => Some(Self::TimedOut),
      "missing_branch" => Some(Self::MissingBranch),
      "inconclusive" => Some(Self::Inconclusive),
      _ => None,
    }
  }
}

/// Bounded failures from the caller-declared run-disposition codec.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentRunDispositionCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
}

/// A payload-free, caller-declared status envelope for one bounded run.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentRunDispositionRecord {
  schema: &'static str,
  disposition: ScriptedAgentRunDisposition,
}

impl ScriptedAgentRunDispositionRecord {
  pub const fn new(disposition: ScriptedAgentRunDisposition) -> Self {
    Self {
      schema: SCRIPTED_AGENT_RUN_DISPOSITION_SCHEMA,
      disposition,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn disposition(self) -> ScriptedAgentRunDisposition {
    self.disposition
  }

  /// Encode the caller-declared disposition as bounded line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\ndisposition={}\n",
      self.schema,
      self.disposition.id(),
    )
  }

  /// Decode a disposition without inspecting execution or process details.
  pub fn decode(input: &str) -> Result<Self, ScriptedAgentRunDispositionCodecError> {
    if input.len() > MAX_SCRIPTED_AGENT_RUN_DISPOSITION_BYTES {
      return Err(ScriptedAgentRunDispositionCodecError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() > 2 {
      return Err(ScriptedAgentRunDispositionCodecError::UnexpectedLineCount {
        expected: 2,
        actual: lines.len(),
      });
    }
    let mut schema = None;
    let mut disposition = None;
    for line in lines {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentRunDispositionCodecError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentRunDispositionCodecError::InvalidValue);
      }
      match key {
        "schema" => {
          if schema.is_some() {
            return Err(ScriptedAgentRunDispositionCodecError::DuplicateField);
          }
          schema = Some(value);
        }
        "disposition" => {
          if disposition.is_some() {
            return Err(ScriptedAgentRunDispositionCodecError::DuplicateField);
          }
          disposition = Some(value);
        }
        _ => return Err(ScriptedAgentRunDispositionCodecError::UnknownField),
      }
    }
    if schema != Some(SCRIPTED_AGENT_RUN_DISPOSITION_SCHEMA) {
      return Err(ScriptedAgentRunDispositionCodecError::UnsupportedSchema);
    }
    let disposition = ScriptedAgentRunDisposition::parse_id(
      disposition.ok_or(ScriptedAgentRunDispositionCodecError::MissingField)?,
    )
    .ok_or(ScriptedAgentRunDispositionCodecError::InvalidValue)?;
    Ok(Self::new(disposition))
  }
}

/// Bounded failures from matched-observation sampling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentMatchedSampleError {
  MismatchedObserver,
  DuplicateObservationId,
  Batch(ScriptedAgentBatchError),
}

/// Bounded failures from fixed-fixture scenario selection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixtureScenarioSelectionError {
  EmptySelection,
  SelectionTooLarge { max: usize, actual: usize },
  UnknownScenario,
  MismatchedObservationPairCount { expected: usize, actual: usize },
  DuplicateObservationId,
}

/// Bounded failures from deterministic fixed-fixture population generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixturePopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  ObservationIdOverflow,
  InvalidSelection(ScriptedAgentFixtureScenarioSelectionError),
}

/// Closed fixture variants available to the bounded M6 scenario selector.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixtureScenario {
  Safe,
  RiverSideThreat,
}

impl ScriptedAgentFixtureScenario {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Safe => SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      Self::RiverSideThreat => SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    }
  }

  fn parse_id(value: &str) -> Result<Self, ScriptedAgentFixtureScenarioSelectionError> {
    match value {
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID => Ok(Self::Safe),
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID => Ok(Self::RiverSideThreat),
      _ => Err(ScriptedAgentFixtureScenarioSelectionError::UnknownScenario),
    }
  }

  fn observations(self, observation_ids: [ObservationId; 2]) -> [LanerObservation; 2] {
    let initial = LaneSnapshot::initial();
    let second = match self {
      Self::Safe => initial,
      Self::RiverSideThreat => LaneSnapshot::new(
        initial.ruleset(),
        initial.turn(),
        LaneStatus::Open,
        initial.player(),
        initial.opponent(),
        initial.wave(),
        JungleThreatTruth::RiverSide,
      ),
    };
    [
      observe_player(&initial, observation_ids[0]).observation(),
      observe_player(&second, observation_ids[1]).observation(),
    ]
  }
}

/// Ordered selection of caller-ID-bound fixed fixture scenarios.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioSelection {
  schema: &'static str,
  scenarios: Vec<ScriptedAgentFixtureScenario>,
  observation_ids: Vec<[ObservationId; 2]>,
}

impl ScriptedAgentFixtureScenarioSelection {
  /// Select closed fixture IDs and bind each to two distinct caller IDs.
  pub fn from_ids(
    scenario_ids: &[&str],
    observation_ids: &[[ObservationId; 2]],
  ) -> Result<Self, ScriptedAgentFixtureScenarioSelectionError> {
    if scenario_ids.is_empty() {
      return Err(ScriptedAgentFixtureScenarioSelectionError::EmptySelection);
    }
    if scenario_ids.len() > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS {
      return Err(
        ScriptedAgentFixtureScenarioSelectionError::SelectionTooLarge {
          max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
          actual: scenario_ids.len(),
        },
      );
    }
    if scenario_ids.len() != observation_ids.len() {
      return Err(
        ScriptedAgentFixtureScenarioSelectionError::MismatchedObservationPairCount {
          expected: scenario_ids.len(),
          actual: observation_ids.len(),
        },
      );
    }
    let mut scenarios = Vec::with_capacity(scenario_ids.len());
    for scenario_id in scenario_ids {
      let scenario = ScriptedAgentFixtureScenario::parse_id(scenario_id)?;
      scenarios.push(scenario);
    }
    let mut seen_ids = Vec::with_capacity(observation_ids.len() * 2);
    for pair in observation_ids {
      for observation_id in pair {
        if seen_ids.contains(observation_id) {
          return Err(ScriptedAgentFixtureScenarioSelectionError::DuplicateObservationId);
        }
        seen_ids.push(*observation_id);
      }
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA,
      scenarios,
      observation_ids: observation_ids.to_vec(),
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub fn scenarios(&self) -> &[ScriptedAgentFixtureScenario] {
    &self.scenarios
  }

  pub fn observation_ids(&self) -> &[[ObservationId; 2]] {
    &self.observation_ids
  }

  /// Build the actor-visible pairs in the selected order.
  pub fn observations(&self) -> Vec<[LanerObservation; 2]> {
    self
      .scenarios
      .iter()
      .copied()
      .zip(self.observation_ids.iter().copied())
      .map(|(scenario, observation_ids)| scenario.observations(observation_ids))
      .collect()
  }

  /// Compose the generated pairs through the existing verified sample path.
  pub fn matched_sample(
    &self,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<ScriptedAgentMatchedScenarioSample, ScriptedAgentMatchedScenarioSampleError> {
    let observations = self.observations();
    ScriptedAgentMatchedScenarioSample::from_observations(&observations, manifests)
  }
}

/// A deterministic population over the closed fixture catalog, bound to a
/// caller-supplied starting observation ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioPopulation {
  schema: &'static str,
  selection: ScriptedAgentFixtureScenarioSelection,
}

/// Closed stress cases used by the bounded M6 population matrix.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentStressCase {
  IllegalCommand,
  ExploitSeeking,
  CommunicationAbuse,
  DegeneratePolicy,
}

impl ScriptedAgentStressCase {
  pub const fn id(self) -> &'static str {
    match self {
      Self::IllegalCommand => "illegal-command-v1",
      Self::ExploitSeeking => "exploit-seeking-v1",
      Self::CommunicationAbuse => "communication-abuse-v1",
      Self::DegeneratePolicy => "degenerate-policy-v1",
    }
  }

  pub const fn ordered() -> [Self; 4] {
    [
      Self::IllegalCommand,
      Self::ExploitSeeking,
      Self::CommunicationAbuse,
      Self::DegeneratePolicy,
    ]
  }
}

/// Closed categorical result IDs for the stress-case matrix.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentStressResult {
  HostValidationRejected,
  StaleObservation,
  MessageInvalidValue,
  RepeatedStabilize,
}

impl ScriptedAgentStressResult {
  pub const fn id(self) -> &'static str {
    match self {
      Self::HostValidationRejected => "host_validation_rejected",
      Self::StaleObservation => "stale_observation",
      Self::MessageInvalidValue => "message_invalid_value",
      Self::RepeatedStabilize => "repeated_stabilize",
    }
  }
}

/// One bounded caller-declared stress-case result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentStressPopulationEntry {
  case: ScriptedAgentStressCase,
  result: ScriptedAgentStressResult,
}

impl ScriptedAgentStressPopulationEntry {
  pub const fn case(self) -> ScriptedAgentStressCase {
    self.case
  }

  pub const fn result(self) -> ScriptedAgentStressResult {
    self.result
  }
}

/// Bounded caller-declared stress-case evidence over existing boundaries.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentStressPopulationReport {
  schema: &'static str,
  degenerate_stabilize_count: u8,
  entries: [ScriptedAgentStressPopulationEntry; 4],
}

/// Failures from constructing the closed stress-case report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentStressPopulationError {
  UnexpectedResult,
  InvalidDegenerateCount,
}

impl ScriptedAgentStressPopulationReport {
  /// Bind the four expected categorical results and one degenerate count.
  pub fn from_results(
    results: [ScriptedAgentStressResult; 4],
    degenerate_stabilize_count: u8,
  ) -> Result<Self, ScriptedAgentStressPopulationError> {
    let expected = [
      ScriptedAgentStressResult::HostValidationRejected,
      ScriptedAgentStressResult::StaleObservation,
      ScriptedAgentStressResult::MessageInvalidValue,
      ScriptedAgentStressResult::RepeatedStabilize,
    ];
    if results != expected {
      return Err(ScriptedAgentStressPopulationError::UnexpectedResult);
    }
    if !(1..=MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS)
      .contains(&usize::from(degenerate_stabilize_count))
    {
      return Err(ScriptedAgentStressPopulationError::InvalidDegenerateCount);
    }
    let cases = ScriptedAgentStressCase::ordered();
    Ok(Self {
      schema: SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA,
      degenerate_stabilize_count,
      entries: [
        ScriptedAgentStressPopulationEntry {
          case: cases[0],
          result: results[0],
        },
        ScriptedAgentStressPopulationEntry {
          case: cases[1],
          result: results[1],
        },
        ScriptedAgentStressPopulationEntry {
          case: cases[2],
          result: results[2],
        },
        ScriptedAgentStressPopulationEntry {
          case: cases[3],
          result: results[3],
        },
      ],
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn degenerate_stabilize_count(&self) -> u8 {
    self.degenerate_stabilize_count
  }

  pub const fn entries(&self) -> &[ScriptedAgentStressPopulationEntry; 4] {
    &self.entries
  }

  /// Render the bounded matrix without performing I/O.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Scripted Agent Stress Population\n\n- schema: {}\n- degenerate_stabilize_count: {}\n\n| case_id | result_id |\n| --- | --- |\n| {} | {} |\n| {} | {} |\n| {} | {} |\n| {} | {} |\n",
      self.schema,
      self.degenerate_stabilize_count,
      self.entries[0].case.id(),
      self.entries[0].result.id(),
      self.entries[1].case.id(),
      self.entries[1].result.id(),
      self.entries[2].case.id(),
      self.entries[2].result.id(),
      self.entries[3].case.id(),
      self.entries[3].result.id(),
    )
  }
}

/// Bounded failures from fixed degenerate-policy population construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentDegeneratePolicyPopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  MismatchedObserver,
  DuplicateObservationId,
  UnexpectedIntent,
}

/// Verified caller-declared population whose cautious policy repeats one intent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentDegeneratePolicyPopulationReport {
  schema: &'static str,
  profile_id: &'static str,
  evaluation_rule: &'static str,
  observer: ActorId,
  observation_count: u8,
  selected_intent: LaneIntent,
}

impl ScriptedAgentDegeneratePolicyPopulationReport {
  /// Build fixed degenerate evidence from actor-visible observations only.
  pub fn from_observations(
    observations: &[LanerObservation],
  ) -> Result<Self, ScriptedAgentDegeneratePolicyPopulationError> {
    if observations.is_empty() {
      return Err(ScriptedAgentDegeneratePolicyPopulationError::EmptyPopulation);
    }
    if observations.len() > MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION {
      return Err(
        ScriptedAgentDegeneratePolicyPopulationError::PopulationTooLarge {
          max: MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION,
          actual: observations.len(),
        },
      );
    }
    let observer = observations[0].observer();
    let mut seen_ids = Vec::with_capacity(observations.len());
    for observation in observations {
      if observation.observer() != observer {
        return Err(ScriptedAgentDegeneratePolicyPopulationError::MismatchedObserver);
      }
      if seen_ids.contains(&observation.observation_id()) {
        return Err(ScriptedAgentDegeneratePolicyPopulationError::DuplicateObservationId);
      }
      seen_ids.push(observation.observation_id());
      if ScriptedAgent::cautious_v1()
        .choose(*observation)
        .selected_intent()
        != LaneIntent::Stabilize
      {
        return Err(ScriptedAgentDegeneratePolicyPopulationError::UnexpectedIntent);
      }
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA,
      profile_id: ScriptedAgentProfile::cautious_v1().profile_id(),
      evaluation_rule: ScriptedAgentProfile::cautious_v1().evaluation_rule(),
      observer,
      observation_count: u8::try_from(observations.len()).expect("population cap fits in u8"),
      selected_intent: LaneIntent::Stabilize,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }
}

/// Bounded failures from fixed exploit-seeking policy population construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentExploitSeekingPopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  MismatchedObserver,
  DuplicateObservationId,
  UnexpectedIntent,
}

/// Verified caller-declared population whose risk-taking policy selects Contest.
///
/// This is fixed-fixture selected-intent evidence only; it does not search for
/// exploits or establish adversarial prevalence, outcomes, or strategy quality.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentExploitSeekingPopulationReport {
  schema: &'static str,
  profile_id: &'static str,
  evaluation_rule: &'static str,
  observer: ActorId,
  observation_count: u8,
  selected_intent: LaneIntent,
}

impl ScriptedAgentExploitSeekingPopulationReport {
  /// Build fixed risk-taking evidence from actor-visible observations only.
  pub fn from_observations(
    observations: &[LanerObservation],
  ) -> Result<Self, ScriptedAgentExploitSeekingPopulationError> {
    if observations.is_empty() {
      return Err(ScriptedAgentExploitSeekingPopulationError::EmptyPopulation);
    }
    if observations.len() > MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION {
      return Err(
        ScriptedAgentExploitSeekingPopulationError::PopulationTooLarge {
          max: MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION,
          actual: observations.len(),
        },
      );
    }
    let observer = observations[0].observer();
    let mut seen_ids = Vec::with_capacity(observations.len());
    for observation in observations {
      if observation.observer() != observer {
        return Err(ScriptedAgentExploitSeekingPopulationError::MismatchedObserver);
      }
      if seen_ids.contains(&observation.observation_id()) {
        return Err(ScriptedAgentExploitSeekingPopulationError::DuplicateObservationId);
      }
      seen_ids.push(observation.observation_id());
      if ScriptedAgent::risk_taking_v1()
        .choose(*observation)
        .selected_intent()
        != LaneIntent::Contest
      {
        return Err(ScriptedAgentExploitSeekingPopulationError::UnexpectedIntent);
      }
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA,
      profile_id: ScriptedAgentProfile::risk_taking_v1().profile_id(),
      evaluation_rule: ScriptedAgentProfile::risk_taking_v1().evaluation_rule(),
      observer,
      observation_count: u8::try_from(observations.len()).expect("population cap fits in u8"),
      selected_intent: LaneIntent::Contest,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }
}

impl ScriptedAgentFixtureScenarioPopulation {
  /// Generate an alternating safe/threat population from a starting ID.
  pub fn generate(
    count: usize,
    first_observation_id: u64,
  ) -> Result<Self, ScriptedAgentFixturePopulationError> {
    if count == 0 {
      return Err(ScriptedAgentFixturePopulationError::EmptyPopulation);
    }
    if count > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS {
      return Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: count,
      });
    }
    let mut scenario_ids = Vec::with_capacity(count);
    for index in 0..count {
      scenario_ids.push(if index % 2 == 0 {
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID
      } else {
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID
      });
    }
    Self::generate_from_scenario_ids(&scenario_ids, first_observation_id)
  }

  /// Generate a caller-declared ordered composition from the closed catalog.
  pub fn generate_from_scenario_ids(
    scenario_ids: &[&str],
    first_observation_id: u64,
  ) -> Result<Self, ScriptedAgentFixturePopulationError> {
    if scenario_ids.is_empty() {
      return Err(ScriptedAgentFixturePopulationError::EmptyPopulation);
    }
    if scenario_ids.len() > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS {
      return Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: scenario_ids.len(),
      });
    }
    for scenario_id in scenario_ids {
      ScriptedAgentFixtureScenario::parse_id(scenario_id)
        .map_err(ScriptedAgentFixturePopulationError::InvalidSelection)?;
    }
    let mut observation_ids = Vec::with_capacity(scenario_ids.len());
    for index in 0..scenario_ids.len() {
      let offset = index
        .checked_mul(2)
        .and_then(|value| u64::try_from(value).ok())
        .ok_or(ScriptedAgentFixturePopulationError::ObservationIdOverflow)?;
      let first = first_observation_id
        .checked_add(offset)
        .ok_or(ScriptedAgentFixturePopulationError::ObservationIdOverflow)?;
      let second = first
        .checked_add(1)
        .ok_or(ScriptedAgentFixturePopulationError::ObservationIdOverflow)?;
      observation_ids.push([ObservationId::new(first), ObservationId::new(second)]);
    }
    let selection = ScriptedAgentFixtureScenarioSelection::from_ids(scenario_ids, &observation_ids)
      .map_err(ScriptedAgentFixturePopulationError::InvalidSelection)?;
    Ok(Self {
      schema: SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA,
      selection,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub fn selection(&self) -> &ScriptedAgentFixtureScenarioSelection {
    &self.selection
  }

  pub fn scenarios(&self) -> &[ScriptedAgentFixtureScenario] {
    self.selection.scenarios()
  }

  pub fn observation_ids(&self) -> &[[ObservationId; 2]] {
    self.selection.observation_ids()
  }

  pub fn observations(&self) -> Vec<[LanerObservation; 2]> {
    self.selection.observations()
  }

  pub fn matched_sample(
    &self,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<ScriptedAgentMatchedScenarioSample, ScriptedAgentMatchedScenarioSampleError> {
    self.selection.matched_sample(manifests)
  }

  /// Aggregate the population's verified sample without rerunning policy evaluation.
  pub fn matched_tally(
    &self,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<ScriptedAgentMatchedScenarioTallyReport, ScriptedAgentMatchedScenarioSampleError> {
    let sample = self.matched_sample(manifests)?;
    Ok(ScriptedAgentMatchedScenarioTallyReport::from_sample(
      &sample,
    ))
  }
}

/// One closed fixture-scenario frequency row.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyEntry {
  scenario_id: &'static str,
  count: u8,
}

impl ScriptedAgentFixtureScenarioFrequencyEntry {
  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn count(self) -> u8 {
    self.count
  }
}

/// A caller-declared numeric label for one comparison build.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentBuildId(u32);

impl ScriptedAgentBuildId {
  pub const fn new(value: u32) -> Self {
    Self(value)
  }

  pub const fn schema(self) -> &'static str {
    SCRIPTED_AGENT_BUILD_ID_SCHEMA
  }

  pub const fn value(self) -> u32 {
    self.0
  }
}

/// Bounded failures from a labeled frequency comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBuildComparisonError {
  MatchingBuildIds,
}

/// Closed operational events kept separate from committed simulation history.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalEvent {
  BatchStarted,
  ChunkCompleted,
  CheckpointSaved,
  BatchResumed,
  BatchFinished,
}

impl ScriptedAgentOperationalEvent {
  pub const fn id(self) -> &'static str {
    match self {
      Self::BatchStarted => "batch_started",
      Self::ChunkCompleted => "chunk_completed",
      Self::CheckpointSaved => "checkpoint_saved",
      Self::BatchResumed => "batch_resumed",
      Self::BatchFinished => "batch_finished",
    }
  }

  fn parse_id(value: &str) -> Option<Self> {
    match value {
      "batch_started" => Some(Self::BatchStarted),
      "chunk_completed" => Some(Self::ChunkCompleted),
      "checkpoint_saved" => Some(Self::CheckpointSaved),
      "batch_resumed" => Some(Self::BatchResumed),
      "batch_finished" => Some(Self::BatchFinished),
      _ => None,
    }
  }
}

/// One payload-free event in a non-authoritative operational log.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentOperationalEventRecord {
  schema: &'static str,
  event: ScriptedAgentOperationalEvent,
}

impl ScriptedAgentOperationalEventRecord {
  pub const fn new(event: ScriptedAgentOperationalEvent) -> Self {
    Self {
      schema: SCRIPTED_AGENT_OPERATIONAL_EVENT_SCHEMA,
      event,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn event(self) -> ScriptedAgentOperationalEvent {
    self.event
  }
}

/// Bounded failures from the in-memory operational log container.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalLogError {
  CapacityExceeded { max: usize },
}

/// Bounded failures from operational-log encoding and decoding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalLogCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
}

/// Bounded failures from producing one complete batch lifecycle trace.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalBatchRunError {
  Batch(ScriptedAgentBatchError),
  LogCapacityExceeded { max: usize },
}

/// Ordered, non-authoritative operational metadata kept outside history.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentOperationalLog {
  schema: &'static str,
  entries: Vec<ScriptedAgentOperationalEventRecord>,
}

/// Closed statuses for the required operational lifecycle sequence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalLogSequenceStatus {
  Complete,
  MissingStart,
  MissingChunk,
  MissingFinish,
  InvalidOrder,
}

impl ScriptedAgentOperationalLogSequenceStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Complete => "complete",
      Self::MissingStart => "missing_start",
      Self::MissingChunk => "missing_chunk",
      Self::MissingFinish => "missing_finish",
      Self::InvalidOrder => "invalid_order",
    }
  }
}

/// Pure sequence status over one caller-declared operational log.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentOperationalLogSequenceReport {
  schema: &'static str,
  rule: &'static str,
  status: ScriptedAgentOperationalLogSequenceStatus,
}

impl ScriptedAgentOperationalLogSequenceReport {
  /// Classify the fixed lifecycle without mutating or producing events.
  pub fn from_log(log: &ScriptedAgentOperationalLog) -> Self {
    let mut phase = 0_u8;
    for event in log.entries().iter().map(|entry| entry.event()) {
      match phase {
        0 if event == ScriptedAgentOperationalEvent::BatchStarted => phase = 1,
        0 => {
          phase = 4;
          break;
        }
        1 if event == ScriptedAgentOperationalEvent::ChunkCompleted => phase = 2,
        1 => {
          phase = 4;
          break;
        }
        2 if event == ScriptedAgentOperationalEvent::BatchFinished => phase = 3,
        2 if matches!(
          event,
          ScriptedAgentOperationalEvent::CheckpointSaved
            | ScriptedAgentOperationalEvent::BatchResumed
        ) => {}
        2 => {
          phase = 4;
          break;
        }
        3 => {
          phase = 4;
          break;
        }
        _ => unreachable!("sequence phases are closed"),
      }
    }
    let status = match phase {
      0 => ScriptedAgentOperationalLogSequenceStatus::MissingStart,
      1 => ScriptedAgentOperationalLogSequenceStatus::MissingChunk,
      2 => ScriptedAgentOperationalLogSequenceStatus::MissingFinish,
      3 => ScriptedAgentOperationalLogSequenceStatus::Complete,
      4 => ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
      _ => unreachable!("sequence phases are bounded"),
    };
    Self {
      schema: SCRIPTED_AGENT_OPERATIONAL_LOG_SEQUENCE_SCHEMA,
      rule: SCRIPTED_AGENT_OPERATIONAL_LOG_SEQUENCE_RULE,
      status,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn rule(self) -> &'static str {
    self.rule
  }

  pub const fn status(self) -> ScriptedAgentOperationalLogSequenceStatus {
    self.status
  }
}

/// Whether a recorded scripted-agent decision reproduced exactly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReplayIdentityStatus {
  Verified,
  DecisionMismatch,
}

impl ScriptedAgentReplayIdentityStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Verified => "verified",
      Self::DecisionMismatch => "decision_mismatch",
    }
  }
}

/// Bounded evidence joining decision replay identity with operational sequence status.
///
/// This report checks one actor-visible decision record against its deterministic
/// replay and one caller-declared operational log against the fixed lifecycle
/// sequence. It does not establish causal-trace completeness, runtime event
/// production, or scenario-wide replay identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentReplaySequenceEvidenceReport {
  schema: &'static str,
  rule: &'static str,
  replay_identity: ScriptedAgentReplayIdentityStatus,
  sequence_status: ScriptedAgentOperationalLogSequenceStatus,
}

impl ScriptedAgentReplaySequenceEvidenceReport {
  /// Build pure evidence from one replay record and one operational label log.
  pub fn from_record_and_log(
    record: &ScriptedAgentReplayRecord,
    log: &ScriptedAgentOperationalLog,
  ) -> Self {
    let replay_identity = match record.replay() {
      Ok(_) => ScriptedAgentReplayIdentityStatus::Verified,
      Err(ScriptedAgentReplayError::DecisionMismatch) => {
        ScriptedAgentReplayIdentityStatus::DecisionMismatch
      }
    };
    Self {
      schema: SCRIPTED_AGENT_REPLAY_SEQUENCE_EVIDENCE_SCHEMA,
      rule: SCRIPTED_AGENT_REPLAY_SEQUENCE_EVIDENCE_RULE,
      replay_identity,
      sequence_status: ScriptedAgentOperationalLogSequenceReport::from_log(log).status(),
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn rule(self) -> &'static str {
    self.rule
  }

  pub const fn replay_identity(self) -> ScriptedAgentReplayIdentityStatus {
    self.replay_identity
  }

  pub const fn sequence_status(self) -> ScriptedAgentOperationalLogSequenceStatus {
    self.sequence_status
  }
}

/// Closed outcome status for scenario-wide replay identity evaluation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentScenarioReplayIdentityStatus {
  AllVerified,
  DecisionMismatch,
}

impl ScriptedAgentScenarioReplayIdentityStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::AllVerified => "all_verified",
      Self::DecisionMismatch => "decision_mismatch",
    }
  }
}

/// Bounded failure modes when building scenario replay identity evidence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentScenarioReplayIdentityError {
  Empty,
  Oversized,
  DuplicateObservationId,
}

/// Bounded evidence verifying deterministic replay across a sequence of decision records.
///
/// This report checks one to sixteen caller-supplied replay records from a sampled
/// scenario run against deterministic replay. It does not claim causal-trace
/// completeness, runtime event production, or scenario-wide persistence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentScenarioReplayIdentityReport {
  schema: &'static str,
  rule: &'static str,
  record_count: u8,
  verified_count: u8,
  status: ScriptedAgentScenarioReplayIdentityStatus,
  start_observation_id: ObservationId,
  end_observation_id: ObservationId,
}

impl ScriptedAgentScenarioReplayIdentityReport {
  /// Evaluate deterministic replay across an ordered slice of decision records.
  pub fn from_records(
    records: &[ScriptedAgentReplayRecord],
  ) -> Result<Self, ScriptedAgentScenarioReplayIdentityError> {
    if records.is_empty() {
      return Err(ScriptedAgentScenarioReplayIdentityError::Empty);
    }
    if records.len() > MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS {
      return Err(ScriptedAgentScenarioReplayIdentityError::Oversized);
    }
    let record_count = match u8::try_from(records.len()) {
      Ok(count) => count,
      Err(_) => return Err(ScriptedAgentScenarioReplayIdentityError::Oversized),
    };

    for (i, record) in records.iter().enumerate() {
      for other in &records[i.saturating_add(1)..] {
        if record.observation_id() == other.observation_id() {
          return Err(ScriptedAgentScenarioReplayIdentityError::DuplicateObservationId);
        }
      }
    }

    let start_observation_id = match records.first() {
      Some(record) => record.observation_id(),
      None => return Err(ScriptedAgentScenarioReplayIdentityError::Empty),
    };
    let end_observation_id = match records.last() {
      Some(record) => record.observation_id(),
      None => return Err(ScriptedAgentScenarioReplayIdentityError::Empty),
    };

    let mut verified_count: u8 = 0;
    let mut all_verified = true;

    for record in records {
      match record.replay() {
        Ok(_) => {
          verified_count = verified_count.saturating_add(1);
        }
        Err(ScriptedAgentReplayError::DecisionMismatch) => {
          all_verified = false;
        }
      }
    }

    let status = if all_verified {
      ScriptedAgentScenarioReplayIdentityStatus::AllVerified
    } else {
      ScriptedAgentScenarioReplayIdentityStatus::DecisionMismatch
    };

    Ok(Self {
      schema: SCRIPTED_AGENT_SCENARIO_REPLAY_IDENTITY_SCHEMA,
      rule: SCRIPTED_AGENT_SCENARIO_REPLAY_IDENTITY_RULE,
      record_count,
      verified_count,
      status,
      start_observation_id,
      end_observation_id,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn rule(self) -> &'static str {
    self.rule
  }

  pub const fn record_count(self) -> u8 {
    self.record_count
  }

  pub const fn verified_count(self) -> u8 {
    self.verified_count
  }

  pub const fn status(self) -> ScriptedAgentScenarioReplayIdentityStatus {
    self.status
  }

  pub const fn start_observation_id(self) -> ObservationId {
    self.start_observation_id
  }

  pub const fn end_observation_id(self) -> ObservationId {
    self.end_observation_id
  }
}

/// Closed outcome status for scenario-wide causal-trace completeness evaluation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentScenarioCausalTraceCompletenessStatus {
  AllComplete,
  IncompleteTrace,
}

impl ScriptedAgentScenarioCausalTraceCompletenessStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::AllComplete => "all_complete",
      Self::IncompleteTrace => "incomplete_trace",
    }
  }
}

/// Bounded failure modes when building scenario causal-trace completeness evidence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentScenarioCausalTraceCompletenessError {
  Empty,
  Oversized,
  DuplicateObservationId,
}

/// Bounded evidence verifying causal-trace completeness across a sequence of decision records.
///
/// This report checks one to sixteen caller-supplied replay records from a sampled
/// scenario run for complete causal policy trace provenance and deterministic replay.
/// It does not claim runtime automated log production, durable persistence, or
/// human gameplay evidence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentScenarioCausalTraceCompletenessReport {
  schema: &'static str,
  rule: &'static str,
  record_count: u8,
  traced_count: u8,
  status: ScriptedAgentScenarioCausalTraceCompletenessStatus,
  start_observation_id: ObservationId,
  end_observation_id: ObservationId,
}

impl ScriptedAgentScenarioCausalTraceCompletenessReport {
  /// Evaluate causal-trace completeness across an ordered slice of decision records.
  pub fn from_records(
    records: &[ScriptedAgentReplayRecord],
  ) -> Result<Self, ScriptedAgentScenarioCausalTraceCompletenessError> {
    if records.is_empty() {
      return Err(ScriptedAgentScenarioCausalTraceCompletenessError::Empty);
    }
    if records.len() > MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS {
      return Err(ScriptedAgentScenarioCausalTraceCompletenessError::Oversized);
    }
    let record_count = match u8::try_from(records.len()) {
      Ok(count) => count,
      Err(_) => return Err(ScriptedAgentScenarioCausalTraceCompletenessError::Oversized),
    };

    for (i, record) in records.iter().enumerate() {
      for other in &records[i.saturating_add(1)..] {
        if record.observation_id() == other.observation_id() {
          return Err(ScriptedAgentScenarioCausalTraceCompletenessError::DuplicateObservationId);
        }
      }
    }

    let start_observation_id = match records.first() {
      Some(record) => record.observation_id(),
      None => return Err(ScriptedAgentScenarioCausalTraceCompletenessError::Empty),
    };
    let end_observation_id = match records.last() {
      Some(record) => record.observation_id(),
      None => return Err(ScriptedAgentScenarioCausalTraceCompletenessError::Empty),
    };

    let mut traced_count: u8 = 0;
    let mut all_complete = true;

    for record in records {
      let is_complete = match record.replay() {
        Ok(decision) => {
          let selected = decision.selected_intent();
          let candidate_matches = decision.candidates().iter().any(|c| c.intent() == selected);
          let rule_valid = !decision.selection_rule().is_empty();
          candidate_matches && rule_valid
        }
        Err(_) => false,
      };

      if is_complete {
        traced_count = traced_count.saturating_add(1);
      } else {
        all_complete = false;
      }
    }

    let status = if all_complete {
      ScriptedAgentScenarioCausalTraceCompletenessStatus::AllComplete
    } else {
      ScriptedAgentScenarioCausalTraceCompletenessStatus::IncompleteTrace
    };

    Ok(Self {
      schema: SCRIPTED_AGENT_SCENARIO_CAUSAL_TRACE_COMPLETENESS_SCHEMA,
      rule: SCRIPTED_AGENT_SCENARIO_CAUSAL_TRACE_COMPLETENESS_RULE,
      record_count,
      traced_count,
      status,
      start_observation_id,
      end_observation_id,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn rule(self) -> &'static str {
    self.rule
  }

  pub const fn record_count(self) -> u8 {
    self.record_count
  }

  pub const fn traced_count(self) -> u8 {
    self.traced_count
  }

  pub const fn status(self) -> ScriptedAgentScenarioCausalTraceCompletenessStatus {
    self.status
  }

  pub const fn start_observation_id(self) -> ObservationId {
    self.start_observation_id
  }

  pub const fn end_observation_id(self) -> ObservationId {
    self.end_observation_id
  }
}

impl ScriptedAgentOperationalLog {
  pub fn new() -> Self {
    Self {
      schema: SCRIPTED_AGENT_OPERATIONAL_EVENT_SCHEMA,
      entries: Vec::with_capacity(MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS),
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub fn append(
    &mut self,
    event: ScriptedAgentOperationalEvent,
  ) -> Result<(), ScriptedAgentOperationalLogError> {
    if self.entries.len() == MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
      return Err(ScriptedAgentOperationalLogError::CapacityExceeded {
        max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
      });
    }
    self
      .entries
      .push(ScriptedAgentOperationalEventRecord::new(event));
    Ok(())
  }

  pub const fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  pub const fn len(&self) -> usize {
    self.entries.len()
  }

  pub fn entries(&self) -> &[ScriptedAgentOperationalEventRecord] {
    &self.entries
  }

  /// Encode the bounded payload-free event log as canonical text.
  pub fn encode(&self) -> String {
    let mut encoded = format!(
      "schema={}\nentries={}\n",
      SCRIPTED_AGENT_OPERATIONAL_LOG_SCHEMA,
      self.entries.len(),
    );
    for entry in &self.entries {
      encoded.push_str(&format!("event={}\n", entry.event().id()));
    }
    encoded
  }

  /// Decode and validate one bounded payload-free event log.
  pub fn decode(input: &str) -> Result<Self, ScriptedAgentOperationalLogCodecError> {
    if input.len() > MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES {
      return Err(ScriptedAgentOperationalLogCodecError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() > MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS + 2 {
      return Err(ScriptedAgentOperationalLogCodecError::UnexpectedLineCount {
        expected: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS + 2,
        actual: lines.len(),
      });
    }
    let mut schema = None;
    let mut entries_count = None;
    let mut event_ids = Vec::new();
    for line in lines.iter() {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentOperationalLogCodecError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentOperationalLogCodecError::InvalidValue);
      }
      match key {
        "schema" => {
          if schema.is_some() {
            return Err(ScriptedAgentOperationalLogCodecError::DuplicateField);
          }
          schema = Some(value);
        }
        "entries" => {
          if entries_count.is_some() {
            return Err(ScriptedAgentOperationalLogCodecError::DuplicateField);
          }
          entries_count = Some(value);
        }
        "event" => event_ids.push(value),
        _ => return Err(ScriptedAgentOperationalLogCodecError::UnknownField),
      }
    }
    if schema != Some(SCRIPTED_AGENT_OPERATIONAL_LOG_SCHEMA) {
      return Err(ScriptedAgentOperationalLogCodecError::UnsupportedSchema);
    }
    let entries_count = entries_count
      .ok_or(ScriptedAgentOperationalLogCodecError::MissingField)?
      .parse::<usize>()
      .map_err(|_| ScriptedAgentOperationalLogCodecError::InvalidValue)?;
    if entries_count > MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
      return Err(ScriptedAgentOperationalLogCodecError::InvalidValue);
    }
    let expected_lines = 2 + entries_count;
    if lines.len() != expected_lines || event_ids.len() != entries_count {
      return Err(ScriptedAgentOperationalLogCodecError::UnexpectedLineCount {
        expected: expected_lines,
        actual: lines.len(),
      });
    }
    let first_key = lines[0].split_once('=').map(|(key, _)| key);
    let second_key = lines[1].split_once('=').map(|(key, _)| key);
    if first_key != Some("schema")
      || second_key != Some("entries")
      || lines
        .iter()
        .skip(2)
        .any(|line| line.split_once('=').map(|(key, _)| key) != Some("event"))
    {
      return Err(ScriptedAgentOperationalLogCodecError::InvalidValue);
    }
    let entries = event_ids
      .into_iter()
      .map(|id| {
        ScriptedAgentOperationalEvent::parse_id(id)
          .map(ScriptedAgentOperationalEventRecord::new)
          .ok_or(ScriptedAgentOperationalLogCodecError::InvalidValue)
      })
      .collect::<Result<Vec<_>, _>>()?;
    Ok(Self {
      schema: SCRIPTED_AGENT_OPERATIONAL_EVENT_SCHEMA,
      entries,
    })
  }
}

impl Default for ScriptedAgentOperationalLog {
  fn default() -> Self {
    Self::new()
  }
}

/// Bounded frequency evidence over one validated fixed-fixture selection.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyReport {
  schema: &'static str,
  selection_count: u8,
  entries: [ScriptedAgentFixtureScenarioFrequencyEntry; 2],
}

/// Bounded failures from the scenario-frequency codec.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixtureScenarioFrequencyCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
  InputMismatch,
}

/// One actor-safe row in a declared baseline comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
  scenario_id: &'static str,
  baseline_count: u8,
  candidate_count: u8,
}

impl ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn baseline_count(self) -> u8 {
    self.baseline_count
  }

  pub const fn candidate_count(self) -> u8 {
    self.candidate_count
  }

  pub fn delta(self) -> i16 {
    i16::from(self.candidate_count) - i16::from(self.baseline_count)
  }
}

/// Bounded row deltas between two caller-declared verified frequency reports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyComparisonReport {
  schema: &'static str,
  baseline_build_id: Option<ScriptedAgentBuildId>,
  candidate_build_id: Option<ScriptedAgentBuildId>,
  baseline_selection_count: u8,
  candidate_selection_count: u8,
  entries: [ScriptedAgentFixtureScenarioFrequencyComparisonEntry; 2],
}

impl ScriptedAgentFixtureScenarioFrequencyComparisonReport {
  /// Compare two verified reports without rerunning selection or policy code.
  pub fn from_reports(
    baseline: &ScriptedAgentFixtureScenarioFrequencyReport,
    candidate: &ScriptedAgentFixtureScenarioFrequencyReport,
  ) -> Self {
    Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_COMPARISON_SCHEMA,
      baseline_build_id: None,
      candidate_build_id: None,
      baseline_selection_count: baseline.selection_count,
      candidate_selection_count: candidate.selection_count,
      entries: [
        ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
          scenario_id: baseline.entries[0].scenario_id,
          baseline_count: baseline.entries[0].count,
          candidate_count: candidate.entries[0].count,
        },
        ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
          scenario_id: baseline.entries[1].scenario_id,
          baseline_count: baseline.entries[1].count,
          candidate_count: candidate.entries[1].count,
        },
      ],
    }
  }

  /// Compare verified reports while retaining distinct caller-declared labels.
  pub fn from_reports_with_build_ids(
    baseline: &ScriptedAgentFixtureScenarioFrequencyReport,
    candidate: &ScriptedAgentFixtureScenarioFrequencyReport,
    baseline_build_id: ScriptedAgentBuildId,
    candidate_build_id: ScriptedAgentBuildId,
  ) -> Result<Self, ScriptedAgentBuildComparisonError> {
    if baseline_build_id == candidate_build_id {
      return Err(ScriptedAgentBuildComparisonError::MatchingBuildIds);
    }
    let mut comparison = Self::from_reports(baseline, candidate);
    comparison.baseline_build_id = Some(baseline_build_id);
    comparison.candidate_build_id = Some(candidate_build_id);
    Ok(comparison)
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn baseline_build_id(&self) -> Option<ScriptedAgentBuildId> {
    self.baseline_build_id
  }

  pub const fn candidate_build_id(&self) -> Option<ScriptedAgentBuildId> {
    self.candidate_build_id
  }

  pub const fn baseline_selection_count(&self) -> u8 {
    self.baseline_selection_count
  }

  pub const fn candidate_selection_count(&self) -> u8 {
    self.candidate_selection_count
  }

  pub const fn regression_rule(&self) -> &'static str {
    SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_REGRESSION_RULE
  }

  pub const fn passes_no_change_gate(&self) -> bool {
    self.baseline_selection_count == self.candidate_selection_count
      && self.entries[0].baseline_count == self.entries[0].candidate_count
      && self.entries[1].baseline_count == self.entries[1].candidate_count
  }

  pub const fn entries(&self) -> &[ScriptedAgentFixtureScenarioFrequencyComparisonEntry; 2] {
    &self.entries
  }
}

impl ScriptedAgentFixtureScenarioFrequencyReport {
  /// Count explicit scenario selections without rerunning policy evaluation.
  pub fn from_selection(selection: &ScriptedAgentFixtureScenarioSelection) -> Self {
    let mut safe_count = 0_u8;
    let mut river_side_count = 0_u8;
    for scenario in selection.scenarios() {
      match scenario {
        ScriptedAgentFixtureScenario::Safe => safe_count += 1,
        ScriptedAgentFixtureScenario::RiverSideThreat => river_side_count += 1,
      }
    }
    Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA,
      selection_count: safe_count + river_side_count,
      entries: [
        ScriptedAgentFixtureScenarioFrequencyEntry {
          scenario_id: SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
          count: safe_count,
        },
        ScriptedAgentFixtureScenarioFrequencyEntry {
          scenario_id: SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
          count: river_side_count,
        },
      ],
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn selection_count(&self) -> u8 {
    self.selection_count
  }

  pub const fn entries(&self) -> &[ScriptedAgentFixtureScenarioFrequencyEntry; 2] {
    &self.entries
  }

  /// Encode the verified frequency report as bounded line-oriented text.
  pub fn encode(&self) -> String {
    format!(
      "schema={}\nselection_count={}\nentries=2\nrow={}|{}\nrow={}|{}\n",
      self.schema,
      self.selection_count,
      self.entries[0].scenario_id,
      self.entries[0].count,
      self.entries[1].scenario_id,
      self.entries[1].count,
    )
  }

  /// Render the verified report as a concise, non-persistent Markdown summary.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Scenario Frequency\n\n- schema: {}\n- selection_count: {}\n\n| scenario_id | count |\n| --- | ---: |\n| {} | {} |\n| {} | {} |\n",
      self.schema,
      self.selection_count,
      self.entries[0].scenario_id,
      self.entries[0].count,
      self.entries[1].scenario_id,
      self.entries[1].count,
    )
  }

  /// Return ordered integer basis-point shares at a 10,000-point scale.
  ///
  /// The first row uses floor division and the second row receives the
  /// remainder, so the two shares always sum to exactly 10,000.
  pub fn distribution_basis_points(&self) -> [u16; 2] {
    let selection_count = u16::from(self.selection_count);
    let first = u16::from(self.entries[0].count) * SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE
      / selection_count;
    [first, SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE - first]
  }

  /// Render the bounded distribution projection without performing I/O.
  pub fn to_distribution_markdown(&self) -> String {
    let shares = self.distribution_basis_points();
    format!(
      "# Scenario Distribution\n\n- schema: {}\n- selection_count: {}\n- share_scale_basis_points: {}\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| {} | {} | {} |\n| {} | {} | {} |\n",
      self.schema,
      self.selection_count,
      SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE,
      self.entries[0].scenario_id,
      self.entries[0].count,
      shares[0],
      self.entries[1].scenario_id,
      self.entries[1].count,
      shares[1],
    )
  }

  /// Decode and validate a report against an already verified value.
  pub fn decode(
    input: &str,
    expected: &Self,
  ) -> Result<Self, ScriptedAgentFixtureScenarioFrequencyCodecError> {
    let decoded = Self::decode_unverified(input)?;
    if decoded != *expected {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InputMismatch);
    }
    Ok(decoded)
  }

  fn decode_unverified(
    input: &str,
  ) -> Result<Self, ScriptedAgentFixtureScenarioFrequencyCodecError> {
    if input.len() > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_BYTES {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    let mut schema = None;
    let mut selection_count = None;
    let mut entries_count = None;
    let mut rows = Vec::new();
    for line in lines.iter() {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
      }
      match key {
        "schema" => {
          if schema.is_some() {
            return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField);
          }
          schema = Some(value);
        }
        "selection_count" => {
          if selection_count.is_some() {
            return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField);
          }
          selection_count = Some(value);
        }
        "entries" => {
          if entries_count.is_some() {
            return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField);
          }
          entries_count = Some(value);
        }
        "row" => rows.push(value),
        _ => return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::UnknownField),
      }
    }
    if schema != Some(SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA) {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::UnsupportedSchema);
    }
    let selection_count = selection_count
      .ok_or(ScriptedAgentFixtureScenarioFrequencyCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
    if !(1..=MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS).contains(&usize::from(selection_count)) {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
    }
    let entries_count = entries_count
      .ok_or(ScriptedAgentFixtureScenarioFrequencyCodecError::MissingField)?
      .parse::<usize>()
      .map_err(|_| ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
    if entries_count != 2 {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
    }
    if lines.len() != 5 || rows.len() != 2 {
      return Err(
        ScriptedAgentFixtureScenarioFrequencyCodecError::UnexpectedLineCount {
          expected: 5,
          actual: lines.len(),
        },
      );
    }
    let mut entries = [
      ScriptedAgentFixtureScenarioFrequencyEntry {
        scenario_id: SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        count: 0,
      },
      ScriptedAgentFixtureScenarioFrequencyEntry {
        scenario_id: SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        count: 0,
      },
    ];
    for (index, row) in rows.into_iter().enumerate() {
      let fields = row.split('|').collect::<Vec<_>>();
      if fields.len() != 2 {
        return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
      }
      if fields[0] != entries[index].scenario_id {
        return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
      }
      entries[index].count = fields[1]
        .parse::<u8>()
        .map_err(|_| ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
    }
    if u16::from(entries[0].count) + u16::from(entries[1].count) != u16::from(selection_count) {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA,
      selection_count,
      entries,
    })
  }
}

/// One actor-safe profile row across two matched observations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedSampleEntry {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  seed_bundle: ScriptedAgentSeedBundle,
  selected_intents: [LaneIntent; 2],
}

impl ScriptedAgentMatchedSampleEntry {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn seed_bundle(self) -> ScriptedAgentSeedBundle {
    self.seed_bundle
  }

  pub const fn selected_intents(self) -> [LaneIntent; 2] {
    self.selected_intents
  }
}

/// Bounded actor-safe selected-intent rows over one matched observation pair.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedSample {
  schema: &'static str,
  observer: ActorId,
  observation_ids: [ObservationId; 2],
  entries: Vec<ScriptedAgentMatchedSampleEntry>,
}

impl ScriptedAgentMatchedSample {
  /// Build a stable sample from two same-actor observations and ordered manifests.
  pub fn from_observations(
    observations: [LanerObservation; 2],
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<Self, ScriptedAgentMatchedSampleError> {
    if observations[0].observer() != observations[1].observer() {
      return Err(ScriptedAgentMatchedSampleError::MismatchedObserver);
    }
    if observations[0].observation_id() == observations[1].observation_id() {
      return Err(ScriptedAgentMatchedSampleError::DuplicateObservationId);
    }
    let first = ScriptedAgentBatchRunner::run(observations[0], manifests)
      .map_err(ScriptedAgentMatchedSampleError::Batch)?;
    let second = ScriptedAgentBatchRunner::run(observations[1], manifests)
      .map_err(ScriptedAgentMatchedSampleError::Batch)?;
    let entries = manifests
      .iter()
      .zip(first.iter())
      .zip(second.iter())
      .map(
        |((manifest, first), second)| ScriptedAgentMatchedSampleEntry {
          profile_id: manifest.profile().profile_id(),
          evaluation_rule: manifest.profile().evaluation_rule(),
          seed_bundle: manifest.seed_bundle(),
          selected_intents: [first.selected_intent(), second.selected_intent()],
        },
      )
      .collect();
    Ok(Self {
      schema: SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA,
      observer: observations[0].observer(),
      observation_ids: [
        observations[0].observation_id(),
        observations[1].observation_id(),
      ],
      entries,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> ActorId {
    self.observer
  }

  pub const fn observation_ids(&self) -> &[ObservationId; 2] {
    &self.observation_ids
  }

  pub fn entries(&self) -> &[ScriptedAgentMatchedSampleEntry] {
    &self.entries
  }
}

/// Bounded failures from matched-scenario sample-set composition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentMatchedScenarioSampleError {
  EmptySample,
  SampleTooLarge { max: usize, actual: usize },
  MismatchedObserver,
  DuplicateObservationId,
  Matched(ScriptedAgentMatchedSampleError),
}

/// Ordered actor-safe matched samples over caller-supplied observation pairs.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedScenarioSample {
  schema: &'static str,
  observer: ActorId,
  samples: Vec<ScriptedAgentMatchedSample>,
}

impl ScriptedAgentMatchedScenarioSample {
  /// Build a bounded sample set without generating scenarios or populations.
  pub fn from_observations(
    observations: &[[LanerObservation; 2]],
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<Self, ScriptedAgentMatchedScenarioSampleError> {
    if observations.is_empty() {
      return Err(ScriptedAgentMatchedScenarioSampleError::EmptySample);
    }
    if observations.len() > MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES {
      return Err(ScriptedAgentMatchedScenarioSampleError::SampleTooLarge {
        max: MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES,
        actual: observations.len(),
      });
    }
    let observer = observations[0][0].observer();
    let mut observation_ids = Vec::with_capacity(observations.len() * 2);
    for pair in observations {
      if pair[0].observer() != pair[1].observer() || pair[0].observer() != observer {
        return Err(ScriptedAgentMatchedScenarioSampleError::MismatchedObserver);
      }
      for observation in pair {
        let observation_id = observation.observation_id();
        if observation_ids.contains(&observation_id) {
          return Err(ScriptedAgentMatchedScenarioSampleError::DuplicateObservationId);
        }
        observation_ids.push(observation_id);
      }
    }
    let samples = observations
      .iter()
      .copied()
      .map(|pair| ScriptedAgentMatchedSample::from_observations(pair, manifests))
      .collect::<Result<Vec<_>, _>>()
      .map_err(ScriptedAgentMatchedScenarioSampleError::Matched)?;
    Ok(Self {
      schema: SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLE_SCHEMA,
      observer,
      samples,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> ActorId {
    self.observer
  }

  pub fn samples(&self) -> &[ScriptedAgentMatchedSample] {
    &self.samples
  }
}

/// One actor-safe selected-intent tally for a sampled profile.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedScenarioTally {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  pair_count: u8,
  observation_count: u8,
  stabilize_count: u8,
  contest_count: u8,
  yield_count: u8,
  recall_count: u8,
  withdraw_count: u8,
}

impl ScriptedAgentMatchedScenarioTally {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn pair_count(self) -> u8 {
    self.pair_count
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn stabilize_count(self) -> u8 {
    self.stabilize_count
  }

  pub const fn contest_count(self) -> u8 {
    self.contest_count
  }

  pub const fn yield_count(self) -> u8 {
    self.yield_count
  }

  pub const fn recall_count(self) -> u8 {
    self.recall_count
  }

  pub const fn withdraw_count(self) -> u8 {
    self.withdraw_count
  }

  /// Return `[Stabilize, Contest, Yield, Recall, Withdraw]` shares at the
  /// shared 10,000-point scale.
  ///
  /// The first four intents use floor division and Withdraw receives the
  /// integer remainder, so the five shares always sum to exactly 10,000.
  pub fn intent_distribution_basis_points(self) -> [u16; 5] {
    let counts = [
      self.stabilize_count,
      self.contest_count,
      self.yield_count,
      self.recall_count,
      self.withdraw_count,
    ];
    let denominator = u16::from(self.observation_count);
    let mut shares = [0_u16; 5];
    let mut assigned = 0_u16;
    for (index, count) in counts.iter().take(4).enumerate() {
      shares[index] = u16::try_from(
        u32::from(*count) * u32::from(SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE)
          / u32::from(denominator),
      )
      .expect("basis-point share fits");
      assigned += shares[index];
    }
    shares[4] = SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE - assigned;
    shares
  }
}

/// Bounded failures from a profile-aware tally comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentMatchedScenarioTallyComparisonError {
  MismatchedObserver,
  MismatchedRows,
}

/// One actor-safe baseline/candidate row in a verified tally comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedScenarioTallyComparisonEntry {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  baseline_stabilize_count: u8,
  baseline_contest_count: u8,
  baseline_yield_count: u8,
  baseline_recall_count: u8,
  baseline_withdraw_count: u8,
  candidate_stabilize_count: u8,
  candidate_contest_count: u8,
  candidate_yield_count: u8,
  candidate_recall_count: u8,
  candidate_withdraw_count: u8,
}

impl ScriptedAgentMatchedScenarioTallyComparisonEntry {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  /// Return counts in `[Stabilize, Contest, Yield, Recall, Withdraw]` order.
  pub const fn baseline_counts(self) -> [u8; 5] {
    [
      self.baseline_stabilize_count,
      self.baseline_contest_count,
      self.baseline_yield_count,
      self.baseline_recall_count,
      self.baseline_withdraw_count,
    ]
  }

  /// Return counts in `[Stabilize, Contest, Yield, Recall, Withdraw]` order.
  pub const fn candidate_counts(self) -> [u8; 5] {
    [
      self.candidate_stabilize_count,
      self.candidate_contest_count,
      self.candidate_yield_count,
      self.candidate_recall_count,
      self.candidate_withdraw_count,
    ]
  }

  /// Return candidate-minus-baseline deltas in
  /// `[Stabilize, Contest, Yield, Recall, Withdraw]` order.
  pub fn deltas(self) -> [i16; 5] {
    let baseline = self.baseline_counts();
    let candidate = self.candidate_counts();
    [
      i16::from(candidate[0]) - i16::from(baseline[0]),
      i16::from(candidate[1]) - i16::from(baseline[1]),
      i16::from(candidate[2]) - i16::from(baseline[2]),
      i16::from(candidate[3]) - i16::from(baseline[3]),
      i16::from(candidate[4]) - i16::from(baseline[4]),
    ]
  }
}

/// One deterministic metric-side candidate from a verified tally comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedScenarioTallyOutlierCandidate {
  schema: &'static str,
  selection_rule: &'static str,
  row_index: u8,
  profile_id: &'static str,
  evaluation_rule: &'static str,
  intent: LaneIntent,
  delta: i16,
  magnitude: u16,
}

impl ScriptedAgentMatchedScenarioTallyOutlierCandidate {
  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn selection_rule(self) -> &'static str {
    self.selection_rule
  }

  pub const fn row_index(self) -> u8 {
    self.row_index
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn intent(self) -> LaneIntent {
    self.intent
  }

  pub const fn delta(self) -> i16 {
    self.delta
  }

  pub const fn magnitude(self) -> u16 {
    self.magnitude
  }
}

/// Closed result of the provisional fixed-fixture outlier threshold signal.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentTallyOutlierThresholdStatus {
  AboveThreshold,
  BelowThreshold,
  NoCandidate,
}

impl ScriptedAgentTallyOutlierThresholdStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::AboveThreshold => "above_threshold",
      Self::BelowThreshold => "below_threshold",
      Self::NoCandidate => "no_candidate",
    }
  }
}

/// Bounded provisional threshold evidence over a verified tally comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentTallyOutlierThresholdReport {
  schema: &'static str,
  rule: &'static str,
  threshold: u16,
  status: ScriptedAgentTallyOutlierThresholdStatus,
}

impl ScriptedAgentTallyOutlierThresholdReport {
  /// Classify the existing largest-delta candidate without rerunning policy.
  pub fn from_comparison(comparison: &ScriptedAgentMatchedScenarioTallyComparisonReport) -> Self {
    let status = match comparison.largest_delta_candidate() {
      Some(candidate)
        if candidate.magnitude() >= SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_MAGNITUDE =>
      {
        ScriptedAgentTallyOutlierThresholdStatus::AboveThreshold
      }
      Some(_) => ScriptedAgentTallyOutlierThresholdStatus::BelowThreshold,
      None => ScriptedAgentTallyOutlierThresholdStatus::NoCandidate,
    };
    Self {
      schema: SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_SCHEMA,
      rule: SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_RULE,
      threshold: SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_MAGNITUDE,
      status,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn rule(self) -> &'static str {
    self.rule
  }

  pub const fn threshold(self) -> u16 {
    self.threshold
  }

  pub const fn status(self) -> ScriptedAgentTallyOutlierThresholdStatus {
    self.status
  }
}

/// Bounded actor-safe count deltas between two verified tally reports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedScenarioTallyComparisonReport {
  schema: &'static str,
  observer: ActorId,
  baseline_pair_count: u8,
  baseline_observation_count: u8,
  candidate_pair_count: u8,
  candidate_observation_count: u8,
  entries: Vec<ScriptedAgentMatchedScenarioTallyComparisonEntry>,
}

/// Bounded failures from the profile-aware tally comparison codec.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentMatchedScenarioTallyComparisonCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
  InputMismatch,
}

impl ScriptedAgentMatchedScenarioTallyComparisonReport {
  /// Compare two verified reports without rerunning policy evaluation.
  pub fn from_reports(
    baseline: &ScriptedAgentMatchedScenarioTallyReport,
    candidate: &ScriptedAgentMatchedScenarioTallyReport,
  ) -> Result<Self, ScriptedAgentMatchedScenarioTallyComparisonError> {
    if baseline.observer != candidate.observer {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonError::MismatchedObserver);
    }
    if baseline.entries.len() != candidate.entries.len()
      || baseline
        .entries
        .iter()
        .zip(&candidate.entries)
        .any(|(baseline, candidate)| {
          baseline.profile_id != candidate.profile_id
            || baseline.evaluation_rule != candidate.evaluation_rule
        })
    {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonError::MismatchedRows);
    }
    let entries = baseline
      .entries
      .iter()
      .zip(&candidate.entries)
      .map(
        |(baseline, candidate)| ScriptedAgentMatchedScenarioTallyComparisonEntry {
          profile_id: baseline.profile_id,
          evaluation_rule: baseline.evaluation_rule,
          baseline_stabilize_count: baseline.stabilize_count,
          baseline_contest_count: baseline.contest_count,
          baseline_yield_count: baseline.yield_count,
          baseline_recall_count: baseline.recall_count,
          baseline_withdraw_count: baseline.withdraw_count,
          candidate_stabilize_count: candidate.stabilize_count,
          candidate_contest_count: candidate.contest_count,
          candidate_yield_count: candidate.yield_count,
          candidate_recall_count: candidate.recall_count,
          candidate_withdraw_count: candidate.withdraw_count,
        },
      )
      .collect();
    Ok(Self {
      schema: SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA,
      observer: baseline.observer,
      baseline_pair_count: baseline.pair_count,
      baseline_observation_count: baseline.observation_count,
      candidate_pair_count: candidate.pair_count,
      candidate_observation_count: candidate.observation_count,
      entries,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> ActorId {
    self.observer
  }

  pub const fn baseline_pair_count(&self) -> u8 {
    self.baseline_pair_count
  }

  pub const fn baseline_observation_count(&self) -> u8 {
    self.baseline_observation_count
  }

  pub const fn candidate_pair_count(&self) -> u8 {
    self.candidate_pair_count
  }

  pub const fn candidate_observation_count(&self) -> u8 {
    self.candidate_observation_count
  }

  pub fn entries(&self) -> &[ScriptedAgentMatchedScenarioTallyComparisonEntry] {
    &self.entries
  }

  /// Select the first largest absolute signed intent-count delta.
  ///
  /// Rows retain their declared order and intents use
  /// `[Stabilize, Contest, Yield, Recall, Withdraw]` order for ties. A
  /// comparison with no changed intent counts returns `None`; this is a
  /// metric-side candidate projection, not an outlier or replay judgment.
  pub fn largest_delta_candidate(
    &self,
  ) -> Option<ScriptedAgentMatchedScenarioTallyOutlierCandidate> {
    let mut best: Option<ScriptedAgentMatchedScenarioTallyOutlierCandidate> = None;
    for (row_index, entry) in self.entries.iter().enumerate() {
      let deltas = entry.deltas();
      for (intent_index, delta) in deltas.into_iter().enumerate() {
        let magnitude = delta.unsigned_abs();
        if magnitude == 0 {
          continue;
        }
        if best.is_some_and(|candidate| magnitude <= candidate.magnitude()) {
          continue;
        }
        best = Some(ScriptedAgentMatchedScenarioTallyOutlierCandidate {
          schema: SCRIPTED_AGENT_TALLY_OUTLIER_CANDIDATE_SCHEMA,
          selection_rule: SCRIPTED_AGENT_TALLY_OUTLIER_CANDIDATE_RULE,
          row_index: u8::try_from(row_index).expect("comparison rows fit in u8"),
          profile_id: entry.profile_id(),
          evaluation_rule: entry.evaluation_rule(),
          intent: match intent_index {
            0 => LaneIntent::Stabilize,
            1 => LaneIntent::Contest,
            2 => LaneIntent::Yield,
            3 => LaneIntent::Recall,
            4 => LaneIntent::Withdraw,
            _ => unreachable!("fixed intent count has five entries"),
          },
          delta,
          magnitude,
        });
      }
    }
    best
  }

  /// Encode the verified comparison as bounded positional line-oriented text.
  pub fn encode(&self) -> String {
    let mut encoded = format!(
      "schema={}\nobserver={}\nbaseline_pair_count={}\nbaseline_observation_count={}\ncandidate_pair_count={}\ncandidate_observation_count={}\nentries={}\n",
      self.schema,
      self.observer.value(),
      self.baseline_pair_count,
      self.baseline_observation_count,
      self.candidate_pair_count,
      self.candidate_observation_count,
      self.entries.len(),
    );
    for entry in &self.entries {
      let baseline = entry.baseline_counts();
      let candidate = entry.candidate_counts();
      encoded.push_str(&format!(
        "row={}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        entry.profile_id,
        entry.evaluation_rule,
        baseline[0],
        baseline[1],
        baseline[2],
        baseline[3],
        baseline[4],
        candidate[0],
        candidate[1],
        candidate[2],
        candidate[3],
        candidate[4],
      ));
    }
    encoded
  }

  /// Decode and validate a comparison against an already verified report.
  pub fn decode(
    input: &str,
    expected: &Self,
  ) -> Result<Self, ScriptedAgentMatchedScenarioTallyComparisonCodecError> {
    let decoded = Self::decode_unverified(input)?;
    if decoded != *expected {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InputMismatch);
    }
    Ok(decoded)
  }

  fn decode_unverified(
    input: &str,
  ) -> Result<Self, ScriptedAgentMatchedScenarioTallyComparisonCodecError> {
    if input.len() > MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_BYTES {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() < 7 {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::MissingField);
    }
    let expected_keys = [
      "schema",
      "observer",
      "baseline_pair_count",
      "baseline_observation_count",
      "candidate_pair_count",
      "candidate_observation_count",
      "entries",
    ];
    let mut values = [None; 7];
    for (index, line) in lines.iter().take(7).enumerate() {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue);
      }
      if key != expected_keys[index] {
        if expected_keys.contains(&key) {
          return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::DuplicateField);
        }
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnknownField);
      }
      values[index] = Some(value);
    }
    let schema = values[0].expect("schema header is collected");
    if schema != SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnsupportedSchema);
    }
    let parse_u8 = |value: &str| {
      value
        .parse::<u8>()
        .map_err(|_| ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue)
    };
    let observer = parse_u8(values[1].expect("observer header is collected"))?;
    let baseline_pair_count = parse_u8(values[2].expect("baseline pair header is collected"))?;
    let baseline_observation_count =
      parse_u8(values[3].expect("baseline observation header is collected"))?;
    let candidate_pair_count = parse_u8(values[4].expect("candidate pair header is collected"))?;
    let candidate_observation_count =
      parse_u8(values[5].expect("candidate observation header is collected"))?;
    for (pair_count, observation_count) in [
      (baseline_pair_count, baseline_observation_count),
      (candidate_pair_count, candidate_observation_count),
    ] {
      if !(1..=MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES).contains(&usize::from(pair_count))
        || observation_count != pair_count * 2
      {
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue);
      }
    }
    let entries_count = values[6]
      .expect("entries header is collected")
      .parse::<usize>()
      .map_err(|_| ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue)?;
    if !(1..=MAX_SCRIPTED_AGENT_BATCH_MANIFESTS).contains(&entries_count) {
      return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue);
    }
    let expected_line_count = 7 + entries_count;
    if lines.len() != expected_line_count {
      return Err(
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnexpectedLineCount {
          expected: expected_line_count,
          actual: lines.len(),
        },
      );
    }
    let mut entries = Vec::with_capacity(entries_count);
    for line in lines.iter().skip(7) {
      let (key, row) = line
        .split_once('=')
        .ok_or(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue)?;
      if key != "row" {
        if expected_keys.contains(&key) {
          return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::DuplicateField);
        }
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnknownField);
      }
      let fields = row.split('|').collect::<Vec<_>>();
      if fields.len() != 12 || fields.iter().any(|value| value.is_empty()) {
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue);
      }
      let profile = ScriptedAgentProfile::parse_id(fields[0])
        .map_err(|_| ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue)?;
      if profile.evaluation_rule() != fields[1] {
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue);
      }
      let mut counts = [0_u8; 10];
      for (index, value) in fields.iter().skip(2).enumerate() {
        counts[index] = parse_u8(value)?;
      }
      let baseline_total = counts[..5]
        .iter()
        .map(|value| u16::from(*value))
        .sum::<u16>();
      let candidate_total = counts[5..]
        .iter()
        .map(|value| u16::from(*value))
        .sum::<u16>();
      if baseline_total != u16::from(baseline_observation_count)
        || candidate_total != u16::from(candidate_observation_count)
      {
        return Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue);
      }
      entries.push(ScriptedAgentMatchedScenarioTallyComparisonEntry {
        profile_id: profile.profile_id(),
        evaluation_rule: profile.evaluation_rule(),
        baseline_stabilize_count: counts[0],
        baseline_contest_count: counts[1],
        baseline_yield_count: counts[2],
        baseline_recall_count: counts[3],
        baseline_withdraw_count: counts[4],
        candidate_stabilize_count: counts[5],
        candidate_contest_count: counts[6],
        candidate_yield_count: counts[7],
        candidate_recall_count: counts[8],
        candidate_withdraw_count: counts[9],
      });
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA,
      observer: ActorId::new(observer),
      baseline_pair_count,
      baseline_observation_count,
      candidate_pair_count,
      candidate_observation_count,
      entries,
    })
  }

  pub const fn regression_rule(&self) -> &'static str {
    SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_REGRESSION_RULE
  }

  /// Return true only when top-level counts and every ordered row are equal.
  pub fn passes_no_change_gate(&self) -> bool {
    self.baseline_pair_count == self.candidate_pair_count
      && self.baseline_observation_count == self.candidate_observation_count
      && self
        .entries
        .iter()
        .all(|entry| entry.baseline_counts() == entry.candidate_counts())
  }
}

/// Bounded actor-safe selected-intent counts over one verified sample set.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedScenarioTallyReport {
  schema: &'static str,
  observer: ActorId,
  pair_count: u8,
  observation_count: u8,
  entries: Vec<ScriptedAgentMatchedScenarioTally>,
}

/// Bounded failures from the selected-intent tally codec.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentMatchedScenarioTallyCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
  InputMismatch,
}

impl ScriptedAgentMatchedScenarioTallyReport {
  /// Aggregate a validated sample set without rerunning policy evaluation.
  pub fn from_sample(sample: &ScriptedAgentMatchedScenarioSample) -> Self {
    let pair_count = u8::try_from(sample.samples.len()).expect("sample cap fits in u8");
    let observation_count = pair_count * 2;
    let manifest_count = sample.samples[0].entries().len();
    let entries = (0..manifest_count)
      .map(|index| {
        let first = sample.samples[0].entries()[index];
        let mut tally = ScriptedAgentMatchedScenarioTally {
          profile_id: first.profile_id(),
          evaluation_rule: first.evaluation_rule(),
          pair_count,
          observation_count,
          stabilize_count: 0,
          contest_count: 0,
          yield_count: 0,
          recall_count: 0,
          withdraw_count: 0,
        };
        for matched in &sample.samples {
          let entry = matched.entries()[index];
          for intent in entry.selected_intents() {
            match intent {
              LaneIntent::Stabilize => tally.stabilize_count += 1,
              LaneIntent::Contest => tally.contest_count += 1,
              LaneIntent::Yield => tally.yield_count += 1,
              LaneIntent::Recall => tally.recall_count += 1,
              LaneIntent::Withdraw => tally.withdraw_count += 1,
            }
          }
        }
        tally
      })
      .collect();
    Self {
      schema: SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_SCHEMA,
      observer: sample.observer,
      pair_count,
      observation_count,
      entries,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> ActorId {
    self.observer
  }

  pub const fn pair_count(&self) -> u8 {
    self.pair_count
  }

  pub const fn observation_count(&self) -> u8 {
    self.observation_count
  }

  pub fn entries(&self) -> &[ScriptedAgentMatchedScenarioTally] {
    &self.entries
  }

  /// Render ordered profile/rule rows and intent shares without performing I/O.
  pub fn to_intent_distribution_markdown(&self) -> String {
    let mut rendered = format!(
      "# Profile Intent Distribution\n\n- schema: {}\n- observer: {}\n\n| profile_id | evaluation_rule | observation_count | stabilize_bp | contest_bp | yield_bp | recall_bp | withdraw_bp |\n| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n",
      self.schema,
      self.observer.value(),
    );
    for entry in &self.entries {
      let shares = (*entry).intent_distribution_basis_points();
      rendered.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
        entry.profile_id,
        entry.evaluation_rule,
        entry.observation_count,
        shares[0],
        shares[1],
        shares[2],
        shares[3],
        shares[4],
      ));
    }
    rendered
  }

  /// Encode the verified selected-intent tally as bounded line-oriented text.
  pub fn encode(&self) -> String {
    let mut encoded = format!(
      "schema={}\nobserver={}\npair_count={}\nobservation_count={}\nentries={}\n",
      self.schema,
      self.observer.value(),
      self.pair_count,
      self.observation_count,
      self.entries.len(),
    );
    for entry in &self.entries {
      encoded.push_str(&format!(
        "row={}|{}|{}|{}|{}|{}|{}\n",
        entry.profile_id,
        entry.evaluation_rule,
        entry.stabilize_count,
        entry.contest_count,
        entry.yield_count,
        entry.recall_count,
        entry.withdraw_count,
      ));
    }
    encoded
  }

  /// Decode and validate a tally against an already verified report.
  pub fn decode(
    input: &str,
    expected: &Self,
  ) -> Result<Self, ScriptedAgentMatchedScenarioTallyCodecError> {
    let decoded = Self::decode_unverified(input)?;
    if decoded != *expected {
      return Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch);
    }
    Ok(decoded)
  }

  fn decode_unverified(input: &str) -> Result<Self, ScriptedAgentMatchedScenarioTallyCodecError> {
    if input.len() > MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_BYTES {
      return Err(ScriptedAgentMatchedScenarioTallyCodecError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    let mut schema = None;
    let mut observer = None;
    let mut pair_count = None;
    let mut observation_count = None;
    let mut entries_count = None;
    let mut rows = Vec::new();
    for line in lines.iter() {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
      }
      match key {
        "schema" => {
          if schema.is_some() {
            return Err(ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField);
          }
          schema = Some(value);
        }
        "observer" => {
          if observer.is_some() {
            return Err(ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField);
          }
          observer = Some(value);
        }
        "pair_count" => {
          if pair_count.is_some() {
            return Err(ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField);
          }
          pair_count = Some(value);
        }
        "observation_count" => {
          if observation_count.is_some() {
            return Err(ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField);
          }
          observation_count = Some(value);
        }
        "entries" => {
          if entries_count.is_some() {
            return Err(ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField);
          }
          entries_count = Some(value);
        }
        "row" => rows.push(value),
        _ => return Err(ScriptedAgentMatchedScenarioTallyCodecError::UnknownField),
      }
    }
    if schema != Some(SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_SCHEMA) {
      return Err(ScriptedAgentMatchedScenarioTallyCodecError::UnsupportedSchema);
    }
    let observer = observer
      .ok_or(ScriptedAgentMatchedScenarioTallyCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)?;
    let pair_count = pair_count
      .ok_or(ScriptedAgentMatchedScenarioTallyCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)?;
    if !(1..=MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES).contains(&usize::from(pair_count)) {
      return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
    }
    let observation_count = observation_count
      .ok_or(ScriptedAgentMatchedScenarioTallyCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)?;
    if observation_count != pair_count * 2 {
      return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
    }
    let entries_count = entries_count
      .ok_or(ScriptedAgentMatchedScenarioTallyCodecError::MissingField)?
      .parse::<usize>()
      .map_err(|_| ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)?;
    if !(1..=MAX_SCRIPTED_AGENT_BATCH_MANIFESTS).contains(&entries_count) {
      return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
    }
    let expected = 5 + entries_count;
    if lines.len() != expected || rows.len() != entries_count {
      return Err(
        ScriptedAgentMatchedScenarioTallyCodecError::UnexpectedLineCount {
          expected,
          actual: lines.len(),
        },
      );
    }
    let entries = rows
      .into_iter()
      .map(|row| {
        let fields = row.split('|').collect::<Vec<_>>();
        if fields.len() != 7 {
          return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
        }
        let profile = ScriptedAgentProfile::parse_id(fields[0])
          .map_err(|_| ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)?;
        if profile.evaluation_rule() != fields[1] {
          return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
        }
        let parse_count = |value: &str| {
          value
            .parse::<u8>()
            .map_err(|_| ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue)
        };
        let counts = [
          parse_count(fields[2])?,
          parse_count(fields[3])?,
          parse_count(fields[4])?,
          parse_count(fields[5])?,
          parse_count(fields[6])?,
        ];
        if counts.iter().map(|value| u16::from(*value)).sum::<u16>() != u16::from(observation_count)
        {
          return Err(ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue);
        }
        Ok(ScriptedAgentMatchedScenarioTally {
          profile_id: profile.profile_id(),
          evaluation_rule: profile.evaluation_rule(),
          pair_count,
          observation_count,
          stabilize_count: counts[0],
          contest_count: counts[1],
          yield_count: counts[2],
          recall_count: counts[3],
          withdraw_count: counts[4],
        })
      })
      .collect::<Result<Vec<_>, _>>()?;
    Ok(Self {
      schema: SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_SCHEMA,
      observer: ActorId::new(observer),
      pair_count,
      observation_count,
      entries,
    })
  }
}

/// A versioned cursor for resuming one bounded manifest batch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentBatchCheckpoint {
  schema: &'static str,
  observer: ActorId,
  observation_id: ObservationId,
  manifest_count: u8,
  completed_count: u8,
  input_fingerprint: u64,
}

impl ScriptedAgentBatchCheckpoint {
  /// Start a cursor for one actor-visible observation and ordered manifest list.
  pub fn new(
    observation: LanerObservation,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<Self, ScriptedAgentBatchError> {
    validate_batch(manifests)?;
    Ok(Self {
      schema: SCRIPTED_AGENT_BATCH_RUN_SCHEMA,
      observer: observation.observer(),
      observation_id: observation.observation_id(),
      manifest_count: u8::try_from(manifests.len()).expect("batch cap fits in u8"),
      completed_count: 0,
      input_fingerprint: batch_input_fingerprint(observation, manifests),
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn observation_id(self) -> ObservationId {
    self.observation_id
  }

  pub const fn manifest_count(self) -> u8 {
    self.manifest_count
  }

  pub const fn completed_count(self) -> u8 {
    self.completed_count
  }

  pub const fn input_fingerprint(self) -> u64 {
    self.input_fingerprint
  }

  pub const fn is_complete(self) -> bool {
    self.completed_count == self.manifest_count
  }

  /// Encode only bounded cursor and input-binding metadata.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nmanifest_count={}\ncompleted_count={}\ninput_fingerprint={}\n",
      self.schema,
      self.observer.value(),
      self.observation_id.value(),
      self.manifest_count,
      self.completed_count,
      self.input_fingerprint,
    )
  }

  /// Decode a cursor without executing policy or touching the filesystem.
  pub fn decode(input: &str) -> Result<Self, ScriptedAgentBatchCheckpointError> {
    if input.len() > MAX_SCRIPTED_AGENT_BATCH_RUN_BYTES {
      return Err(ScriptedAgentBatchCheckpointError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() > 6 {
      return Err(ScriptedAgentBatchCheckpointError::UnexpectedLineCount {
        expected: 6,
        actual: lines.len(),
      });
    }
    let mut fields = Vec::with_capacity(6);
    for line in lines {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentBatchCheckpointError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentBatchCheckpointError::InvalidValue);
      }
      fields.push((key, value));
    }
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut manifest_count = None;
    let mut completed_count = None;
    let mut input_fingerprint = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "manifest_count" => &mut manifest_count,
        "completed_count" => &mut completed_count,
        "input_fingerprint" => &mut input_fingerprint,
        _ => return Err(ScriptedAgentBatchCheckpointError::UnknownField),
      };
      if slot.is_some() {
        return Err(ScriptedAgentBatchCheckpointError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(SCRIPTED_AGENT_BATCH_RUN_SCHEMA) {
      return Err(ScriptedAgentBatchCheckpointError::UnsupportedSchema);
    }
    let observer = observer
      .ok_or(ScriptedAgentBatchCheckpointError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentBatchCheckpointError::InvalidValue)?;
    let observation_id = observation_id
      .ok_or(ScriptedAgentBatchCheckpointError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ScriptedAgentBatchCheckpointError::InvalidValue)?;
    let manifest_count = manifest_count
      .ok_or(ScriptedAgentBatchCheckpointError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentBatchCheckpointError::InvalidValue)?;
    let completed_count = completed_count
      .ok_or(ScriptedAgentBatchCheckpointError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentBatchCheckpointError::InvalidValue)?;
    let input_fingerprint = input_fingerprint
      .ok_or(ScriptedAgentBatchCheckpointError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ScriptedAgentBatchCheckpointError::InvalidValue)?;
    if !(1..=MAX_SCRIPTED_AGENT_BATCH_MANIFESTS).contains(&usize::from(manifest_count))
      || completed_count > manifest_count
    {
      return Err(ScriptedAgentBatchCheckpointError::InvalidValue);
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_BATCH_RUN_SCHEMA,
      observer: ActorId::new(observer),
      observation_id: ObservationId::new(observation_id),
      manifest_count,
      completed_count,
      input_fingerprint,
    })
  }

  fn matches(
    self,
    observation: LanerObservation,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> bool {
    self.schema == SCRIPTED_AGENT_BATCH_RUN_SCHEMA
      && self.observer == observation.observer()
      && self.observation_id == observation.observation_id()
      && usize::from(self.manifest_count) == manifests.len()
      && self.input_fingerprint == batch_input_fingerprint(observation, manifests)
  }

  fn with_completed_count(self, completed_count: usize) -> Self {
    Self {
      completed_count: u8::try_from(completed_count).expect("batch cap fits in u8"),
      ..self
    }
  }
}

/// Deterministic in-process evaluation of declared scripted-agent manifests.
pub struct ScriptedAgentBatchRunner;

impl ScriptedAgentBatchRunner {
  /// Evaluate manifests in order using only one actor-visible observation.
  pub fn run(
    observation: LanerObservation,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<Vec<ScriptedAgentDecision>, ScriptedAgentBatchError> {
    validate_batch(manifests)?;
    Ok(Self::evaluate_range(observation, manifests))
  }

  /// Evaluate one complete batch while appending a caller-owned lifecycle trace.
  pub fn run_with_operational_log(
    observation: LanerObservation,
    manifests: &[ScriptedAgentExperimentManifest],
    log: &mut ScriptedAgentOperationalLog,
  ) -> Result<Vec<ScriptedAgentDecision>, ScriptedAgentOperationalBatchRunError> {
    validate_batch(manifests).map_err(ScriptedAgentOperationalBatchRunError::Batch)?;
    const EVENTS_PER_COMPLETE_BATCH: usize = 3;
    if log.len().saturating_add(EVENTS_PER_COMPLETE_BATCH) > MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
      return Err(ScriptedAgentOperationalBatchRunError::LogCapacityExceeded {
        max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
      });
    }
    log
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("operational log capacity was preflighted");
    let decisions = Self::evaluate_range(observation, manifests);
    log
      .append(ScriptedAgentOperationalEvent::ChunkCompleted)
      .expect("operational log capacity was preflighted");
    log
      .append(ScriptedAgentOperationalEvent::BatchFinished)
      .expect("operational log capacity was preflighted");
    Ok(decisions)
  }

  /// Evaluate one bounded remaining chunk and return its advanced cursor.
  pub fn run_next(
    observation: LanerObservation,
    manifests: &[ScriptedAgentExperimentManifest],
    checkpoint: ScriptedAgentBatchCheckpoint,
    chunk_size: usize,
  ) -> Result<(Vec<ScriptedAgentDecision>, ScriptedAgentBatchCheckpoint), ScriptedAgentBatchRunError>
  {
    if chunk_size == 0 {
      return Err(ScriptedAgentBatchRunError::ChunkSizeZero);
    }
    validate_batch(manifests).map_err(ScriptedAgentBatchRunError::Batch)?;
    if !checkpoint.matches(observation, manifests) {
      return Err(ScriptedAgentBatchRunError::InputMismatch);
    }
    let start = usize::from(checkpoint.completed_count);
    let end = start.saturating_add(chunk_size).min(manifests.len());
    let decisions = Self::evaluate_range(observation, &manifests[start..end]);
    Ok((decisions, checkpoint.with_completed_count(end)))
  }

  fn evaluate_range(
    observation: LanerObservation,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Vec<ScriptedAgentDecision> {
    let mut decisions = Vec::with_capacity(manifests.len());
    for manifest in manifests {
      let agent = ScriptedAgent {
        profile: manifest.profile(),
      };
      decisions.push(agent.choose_with_seed(observation, manifest.seed_bundle()));
    }
    decisions
  }
}

fn validate_batch(
  manifests: &[ScriptedAgentExperimentManifest],
) -> Result<(), ScriptedAgentBatchError> {
  if manifests.is_empty() {
    return Err(ScriptedAgentBatchError::EmptyBatch);
  }
  if manifests.len() > MAX_SCRIPTED_AGENT_BATCH_MANIFESTS {
    return Err(ScriptedAgentBatchError::BatchTooLarge {
      max: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS,
      actual: manifests.len(),
    });
  }
  Ok(())
}

fn batch_input_fingerprint(
  observation: LanerObservation,
  manifests: &[ScriptedAgentExperimentManifest],
) -> u64 {
  let mut hasher = FnvHasher::default();
  write_str(&mut hasher, SCRIPTED_AGENT_BATCH_RUN_SCHEMA);
  write_observation(&mut hasher, observation);
  write_u8(
    &mut hasher,
    u8::try_from(manifests.len()).expect("batch cap fits in u8"),
  );
  for manifest in manifests {
    write_str(&mut hasher, manifest.schema());
    write_str(&mut hasher, manifest.scenario_id());
    write_str(&mut hasher, manifest.profile().profile_id());
    write_str(&mut hasher, manifest.profile().evaluation_rule());
    write_str(&mut hasher, manifest.selection_rule());
    write_u64(&mut hasher, manifest.seed_bundle().seed());
    write_u8(
      &mut hasher,
      manifest.seed_bundle().policy_trace().stream().value(),
    );
    write_u16(
      &mut hasher,
      manifest.seed_bundle().policy_trace().draw().value(),
    );
  }
  hasher.finish()
}

fn write_observation(hasher: &mut FnvHasher, observation: LanerObservation) {
  write_str(hasher, observation.schema());
  write_u8(hasher, observation.observer().value());
  for role in LaneActorRole::roster() {
    write_u8(hasher, observation.actors().actor(role).value());
  }
  write_u32(hasher, observation.turn().value());
  write_u64(hasher, observation.observation_id().value());
  write_u8(hasher, observation.self_health().value());
  write_u8(hasher, observation.self_mana().value());
  write_u8(hasher, observation.self_gold().value());
  write_u8(hasher, observation.self_experience().value());
  write_u8(hasher, observation.self_cooldown().value());
  write_position(hasher, observation.self_position());
  write_u8(hasher, observation.wave_pressure().value());

  let opponent = observation.opponent();
  write_optional_position(hasher, opponent.last_known_position());
  write_optional_u32(hasher, opponent.last_seen_turn().map(|turn| turn.value()));
  write_hidden_value(hasher, opponent.health());
  write_hidden_value(hasher, opponent.posture());

  let threat = observation.jungle_threat();
  match threat.last_known_region() {
    None => write_u8(hasher, 0),
    Some(region) => {
      write_u8(hasher, 1);
      write_threat_region(hasher, region);
      write_u32(
        hasher,
        threat
          .last_seen_turn()
          .expect("known threat has a last-seen turn")
          .value(),
      );
    }
  }
  for intent in observation.available_intents() {
    write_intent(hasher, intent);
  }
  write_optional_intent(hasher, observation.available_threat_response());
  for focus in observation.available_target_focuses() {
    write_target_focus(hasher, focus);
  }
  for commitment in observation.available_commitments() {
    write_commitment(hasher, commitment);
  }
  for signal in observation.available_ping_signals() {
    write_ping_signal(hasher, signal);
  }
  for condition in observation.available_abort_conditions() {
    write_abort_condition(hasher, condition);
  }
  for behavior in observation.available_fallback_behaviors() {
    write_fallback_behavior(hasher, behavior);
  }
  write_u32(hasher, observation.window().beats());
}

fn write_str(hasher: &mut FnvHasher, value: &str) {
  write_u64(
    hasher,
    u64::try_from(value.len()).expect("string length fits in u64"),
  );
  hasher.write(value.as_bytes());
}

fn write_u8(hasher: &mut FnvHasher, value: u8) {
  hasher.write(&[value]);
}

fn write_u16(hasher: &mut FnvHasher, value: u16) {
  hasher.write(&value.to_le_bytes());
}

fn write_u32(hasher: &mut FnvHasher, value: u32) {
  hasher.write(&value.to_le_bytes());
}

fn write_u64(hasher: &mut FnvHasher, value: u64) {
  hasher.write(&value.to_le_bytes());
}

fn write_optional_u32(hasher: &mut FnvHasher, value: Option<u32>) {
  match value {
    Some(value) => {
      write_u8(hasher, 1);
      write_u32(hasher, value);
    }
    None => write_u8(hasher, 0),
  }
}

fn write_hidden_value(hasher: &mut FnvHasher, value: HiddenValue) {
  match value {
    HiddenValue::Unknown => write_u8(hasher, 0),
  }
}

fn write_position(hasher: &mut FnvHasher, value: LanePosition) {
  write_u8(
    hasher,
    match value {
      LanePosition::NearTower => 0,
      LanePosition::Center => 1,
      LanePosition::FarSide => 2,
    },
  );
}

fn write_optional_position(hasher: &mut FnvHasher, value: Option<LanePosition>) {
  match value {
    Some(value) => {
      write_u8(hasher, 1);
      write_position(hasher, value);
    }
    None => write_u8(hasher, 0),
  }
}

fn write_threat_region(hasher: &mut FnvHasher, value: JungleThreatRegion) {
  match value {
    JungleThreatRegion::RiverSide => write_u8(hasher, 0),
  }
}

fn write_intent(hasher: &mut FnvHasher, value: LaneIntent) {
  write_u8(
    hasher,
    match value {
      LaneIntent::Stabilize => 0,
      LaneIntent::Contest => 1,
      LaneIntent::Yield => 2,
      LaneIntent::Recall => 3,
      LaneIntent::Withdraw => 4,
    },
  );
}

fn write_optional_intent(hasher: &mut FnvHasher, value: Option<LaneIntent>) {
  match value {
    Some(value) => {
      write_u8(hasher, 1);
      write_intent(hasher, value);
    }
    None => write_u8(hasher, 0),
  }
}

fn write_target_focus(hasher: &mut FnvHasher, value: LaneTargetFocus) {
  write_u8(
    hasher,
    match value {
      LaneTargetFocus::Minions => 0,
      LaneTargetFocus::OpposingLaner => 1,
      LaneTargetFocus::Tower => 2,
    },
  );
}

fn write_commitment(hasher: &mut FnvHasher, value: LaneCommitment) {
  write_u8(
    hasher,
    match value {
      LaneCommitment::Standard => 0,
      LaneCommitment::Cautious => 1,
      LaneCommitment::Aggressive => 2,
    },
  );
}

fn write_ping_signal(hasher: &mut FnvHasher, value: LanePingSignal) {
  write_u8(
    hasher,
    match value {
      LanePingSignal::None => 0,
      LanePingSignal::Danger => 1,
      LanePingSignal::OnMyWay => 2,
      LanePingSignal::Assist => 3,
      LanePingSignal::EnemyMissing => 4,
    },
  );
}

fn write_abort_condition(hasher: &mut FnvHasher, value: LaneAbortCondition) {
  write_u8(
    hasher,
    match value {
      LaneAbortCondition::None => 0,
      LaneAbortCondition::HealthThreshold => 1,
      LaneAbortCondition::ThreatSpotted => 2,
      LaneAbortCondition::ResourceDepleted => 3,
    },
  );
}

fn write_fallback_behavior(hasher: &mut FnvHasher, value: LaneFallbackBehavior) {
  write_u8(
    hasher,
    match value {
      LaneFallbackBehavior::MaintainPlan => 0,
      LaneFallbackBehavior::RetreatToTower => 1,
      LaneFallbackBehavior::SafeFarm => 2,
      LaneFallbackBehavior::ConserveResources => 3,
    },
  );
}

struct FnvHasher(u64);

impl Default for FnvHasher {
  fn default() -> Self {
    Self(0xcbf29ce484222325)
  }
}

impl Hasher for FnvHasher {
  fn finish(&self) -> u64 {
    self.0
  }

  fn write(&mut self, bytes: &[u8]) {
    for byte in bytes {
      self.0 ^= u64::from(*byte);
      self.0 = self.0.wrapping_mul(0x100000001b3);
    }
  }
}

/// Why the policy assigned a candidate its score.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReason {
  ThreatResponse,
  RiskPreference,
  YieldPreference,
  StableDefault,
  AvailableAlternative,
}

/// Bounded error returned when a caller evaluates an intent outside the
/// observation's actor-visible candidate set.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentEvaluationError {
  UnavailableIntent,
}

/// One candidate produced from actor-visible information and its fixed score.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentCandidate {
  intent: LaneIntent,
  score: i16,
  reason: ScriptedAgentReason,
}

impl ScriptedAgentCandidate {
  pub const fn intent(self) -> LaneIntent {
    self.intent
  }

  pub const fn score(self) -> i16 {
    self.score
  }

  pub const fn reason(self) -> ScriptedAgentReason {
    self.reason
  }
}

/// A reproducible policy result ready for host-side validation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentDecision {
  profile: ScriptedAgentProfile,
  observer: crate::kernel::ActorId,
  observation_id: crate::lane::ObservationId,
  candidates: Vec<ScriptedAgentCandidate>,
  selected_intent: LaneIntent,
  request: LaneIntentRequest,
  selection_rule: &'static str,
  seed_bundle: Option<ScriptedAgentSeedBundle>,
}

impl ScriptedAgentDecision {
  pub const fn profile(&self) -> ScriptedAgentProfile {
    self.profile
  }

  pub const fn observer(&self) -> crate::kernel::ActorId {
    self.observer
  }

  pub const fn observation_id(&self) -> crate::lane::ObservationId {
    self.observation_id
  }

  pub fn candidates(&self) -> &[ScriptedAgentCandidate] {
    &self.candidates
  }

  pub const fn selected_intent(&self) -> LaneIntent {
    self.selected_intent
  }

  pub const fn request(&self) -> LaneIntentRequest {
    self.request
  }

  pub const fn selection_rule(&self) -> &'static str {
    self.selection_rule
  }

  pub const fn seed_bundle(&self) -> Option<ScriptedAgentSeedBundle> {
    self.seed_bundle
  }
}

/// Whether a replayed policy decision matched its declared expectation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReplayDisposition {
  Expected,
  Anomalous,
}

/// Bounded failure when a recorded policy decision no longer replays exactly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentReplayError {
  DecisionMismatch,
}

/// Actor-visible scripted-policy decision record for deterministic replay.
///
/// The record stores no true state, state hash, execution input, or host
/// history. It is a library inspection artifact; durable persistence remains an
/// outer adapter concern.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentReplayRecord {
  schema: &'static str,
  observation: LanerObservation,
  profile: ScriptedAgentProfile,
  seed_bundle: Option<ScriptedAgentSeedBundle>,
  decision: ScriptedAgentDecision,
  expected_intent: LaneIntent,
  disposition: ScriptedAgentReplayDisposition,
}

impl ScriptedAgentReplayRecord {
  /// Capture a decision and classify it against a declared expected intent.
  pub fn capture(
    agent: ScriptedAgent,
    observation: LanerObservation,
    expected_intent: LaneIntent,
    seed_bundle: Option<ScriptedAgentSeedBundle>,
  ) -> Self {
    let decision = match seed_bundle {
      Some(seed_bundle) => agent.choose_with_seed(observation, seed_bundle),
      None => agent.choose(observation),
    };
    let disposition = if decision.selected_intent() == expected_intent {
      ScriptedAgentReplayDisposition::Expected
    } else {
      ScriptedAgentReplayDisposition::Anomalous
    };
    Self {
      schema: SCRIPTED_AGENT_REPLAY_SCHEMA,
      observation,
      profile: agent.profile(),
      seed_bundle,
      decision,
      expected_intent,
      disposition,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile(&self) -> ScriptedAgentProfile {
    self.profile
  }

  pub const fn observation_id(&self) -> crate::lane::ObservationId {
    self.observation.observation_id()
  }

  pub const fn expected_intent(&self) -> LaneIntent {
    self.expected_intent
  }

  pub const fn selected_intent(&self) -> LaneIntent {
    self.decision.selected_intent()
  }

  pub const fn disposition(&self) -> ScriptedAgentReplayDisposition {
    self.disposition
  }

  pub const fn seed_bundle(&self) -> Option<ScriptedAgentSeedBundle> {
    self.seed_bundle
  }

  pub fn decision(&self) -> &ScriptedAgentDecision {
    &self.decision
  }

  /// Re-evaluate the actor-visible policy input and verify the recorded result.
  pub fn replay(&self) -> Result<ScriptedAgentDecision, ScriptedAgentReplayError> {
    let agent = ScriptedAgent {
      profile: self.profile,
    };
    let decision = match self.seed_bundle() {
      Some(seed_bundle) => agent.choose_with_seed(self.observation, seed_bundle),
      None => agent.choose(self.observation),
    };
    if decision == self.decision {
      Ok(decision)
    } else {
      Err(ScriptedAgentReplayError::DecisionMismatch)
    }
  }
}

/// Bounded failures from candidate-to-replay reference selection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentTallyReplayReferenceError {
  NoMatchingReplay,
  DecisionMismatch,
}

/// Caller-declared reference to the first verified replay matching one tally candidate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentTallyReplayReference {
  schema: &'static str,
  selection_rule: &'static str,
  row_index: u8,
  profile_id: &'static str,
  evaluation_rule: &'static str,
  intent: LaneIntent,
  delta: i16,
  magnitude: u16,
  observation_id: ObservationId,
}

impl ScriptedAgentTallyReplayReference {
  /// Select the first caller-declared replay whose verified decision matches the candidate.
  pub fn from_candidate_and_records(
    candidate: ScriptedAgentMatchedScenarioTallyOutlierCandidate,
    records: &[ScriptedAgentReplayRecord],
  ) -> Result<Self, ScriptedAgentTallyReplayReferenceError> {
    let mut matching_mismatch = false;
    for record in records {
      if record.profile().profile_id() != candidate.profile_id()
        || record.profile().evaluation_rule() != candidate.evaluation_rule()
        || record.selected_intent() != candidate.intent()
      {
        continue;
      }
      match record.replay() {
        Ok(_) => {
          return Ok(Self {
            schema: SCRIPTED_AGENT_TALLY_REPLAY_REFERENCE_SCHEMA,
            selection_rule: SCRIPTED_AGENT_TALLY_REPLAY_REFERENCE_RULE,
            row_index: candidate.row_index(),
            profile_id: candidate.profile_id(),
            evaluation_rule: candidate.evaluation_rule(),
            intent: candidate.intent(),
            delta: candidate.delta(),
            magnitude: candidate.magnitude(),
            observation_id: record.observation_id(),
          });
        }
        Err(ScriptedAgentReplayError::DecisionMismatch) => matching_mismatch = true,
      }
    }
    Err(if matching_mismatch {
      ScriptedAgentTallyReplayReferenceError::DecisionMismatch
    } else {
      ScriptedAgentTallyReplayReferenceError::NoMatchingReplay
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn selection_rule(self) -> &'static str {
    self.selection_rule
  }

  pub const fn row_index(self) -> u8 {
    self.row_index
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn intent(self) -> LaneIntent {
    self.intent
  }

  pub const fn delta(self) -> i16 {
    self.delta
  }

  pub const fn magnitude(self) -> u16 {
    self.magnitude
  }

  pub const fn observation_id(self) -> ObservationId {
    self.observation_id
  }
}

/// Closed result of calibrated outlier detection and representative replay tracing.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentCalibratedOutlierReplayStatus {
  Qualified,
  BelowThreshold,
  NoCandidate,
  NoMatchingReplay,
  DecisionMismatch,
}

impl ScriptedAgentCalibratedOutlierReplayStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Qualified => "qualified",
      Self::BelowThreshold => "below_threshold",
      Self::NoCandidate => "no_candidate",
      Self::NoMatchingReplay => "no_matching_replay",
      Self::DecisionMismatch => "decision_mismatch",
    }
  }
}

/// Bounded evidence report tracing a calibrated outlier to a committed representative replay.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentCalibratedOutlierReplayReport {
  schema: &'static str,
  rule: &'static str,
  threshold: u16,
  status: ScriptedAgentCalibratedOutlierReplayStatus,
  candidate: Option<ScriptedAgentMatchedScenarioTallyOutlierCandidate>,
  observation_id: Option<ObservationId>,
}

impl ScriptedAgentCalibratedOutlierReplayReport {
  /// Trace a calibrated outlier candidate from comparison to a verified representative replay.
  pub fn from_comparison_and_records(
    comparison: &ScriptedAgentMatchedScenarioTallyComparisonReport,
    records: &[ScriptedAgentReplayRecord],
  ) -> Self {
    let candidate = match comparison.largest_delta_candidate() {
      Some(c) => c,
      None => {
        return Self {
          schema: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
          rule: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
          threshold: SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE,
          status: ScriptedAgentCalibratedOutlierReplayStatus::NoCandidate,
          candidate: None,
          observation_id: None,
        };
      }
    };

    if candidate.magnitude() < SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE {
      return Self {
        schema: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
        rule: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
        threshold: SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE,
        status: ScriptedAgentCalibratedOutlierReplayStatus::BelowThreshold,
        candidate: Some(candidate),
        observation_id: None,
      };
    }

    let mut matching_mismatch: Option<ObservationId> = None;
    for record in records {
      if record.profile().profile_id() != candidate.profile_id()
        || record.profile().evaluation_rule() != candidate.evaluation_rule()
        || record.selected_intent() != candidate.intent()
      {
        continue;
      }
      match record.replay() {
        Ok(_) => {
          return Self {
            schema: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
            rule: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
            threshold: SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE,
            status: ScriptedAgentCalibratedOutlierReplayStatus::Qualified,
            candidate: Some(candidate),
            observation_id: Some(record.observation_id()),
          };
        }
        Err(ScriptedAgentReplayError::DecisionMismatch) => {
          if matching_mismatch.is_none() {
            matching_mismatch = Some(record.observation_id());
          }
        }
      }
    }

    if let Some(obs_id) = matching_mismatch {
      Self {
        schema: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
        rule: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
        threshold: SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE,
        status: ScriptedAgentCalibratedOutlierReplayStatus::DecisionMismatch,
        candidate: Some(candidate),
        observation_id: Some(obs_id),
      }
    } else {
      Self {
        schema: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
        rule: SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
        threshold: SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE,
        status: ScriptedAgentCalibratedOutlierReplayStatus::NoMatchingReplay,
        candidate: Some(candidate),
        observation_id: None,
      }
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn rule(self) -> &'static str {
    self.rule
  }

  pub const fn threshold(self) -> u16 {
    self.threshold
  }

  pub const fn status(self) -> ScriptedAgentCalibratedOutlierReplayStatus {
    self.status
  }

  pub const fn candidate(self) -> Option<ScriptedAgentMatchedScenarioTallyOutlierCandidate> {
    self.candidate
  }

  pub const fn observation_id(self) -> Option<ObservationId> {
    self.observation_id
  }
}

/// Compact semantic risk tolerance level.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticRiskTolerance {
  /// Prioritizes damage avoidance and retreat under ambiguity or threat.
  Cautious,
  /// Balances resource gain and safety.
  Balanced,
  /// Prioritizes contested objectives and forward pressure despite risk.
  RiskSeeking,
}

impl SemanticRiskTolerance {
  /// Return the canonical label for this risk tolerance level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Cautious => "cautious",
      Self::Balanced => "balanced",
      Self::RiskSeeking => "risk-seeking",
    }
  }

  /// Parse a risk tolerance level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "cautious" => Some(Self::Cautious),
      "balanced" => Some(Self::Balanced),
      "risk-seeking" => Some(Self::RiskSeeking),
      _ => None,
    }
  }
}

/// Compact semantic deference level for authority and coordination.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticDeference {
  /// Acts primarily on own local evaluation.
  Autonomous,
  /// Aligns with external calls or leader direction.
  Compliant,
  /// Readily yields contest priority to ally or neutral presence.
  Yielding,
}

impl SemanticDeference {
  /// Return the canonical label for this deference level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Autonomous => "autonomous",
      Self::Compliant => "compliant",
      Self::Yielding => "yielding",
    }
  }

  /// Parse a deference level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "autonomous" => Some(Self::Autonomous),
      "compliant" => Some(Self::Compliant),
      "yielding" => Some(Self::Yielding),
      _ => None,
    }
  }
}

/// Compact semantic focus level for decision posture.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticFocus {
  /// Waits for wave stabilization and defensive positioning.
  Patience,
  /// Exploits openings and immediate favorable conditions.
  Opportunity,
  /// Prioritizes rapid escalation or immediate objective contest.
  Urgency,
}

impl SemanticFocus {
  /// Return the canonical label for this focus level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Patience => "patience",
      Self::Opportunity => "opportunity",
      Self::Urgency => "urgency",
    }
  }

  /// Parse a focus level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "patience" => Some(Self::Patience),
      "opportunity" => Some(Self::Opportunity),
      "urgency" => Some(Self::Urgency),
      _ => None,
    }
  }
}

/// Compact semantic communication clarity level.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticCommunicationClarity {
  /// Minimal signals, essential threat and status only.
  Terse,
  /// Balanced communicative frequency.
  Standard,
  /// High communicative frequency and explicit intents.
  Verbose,
}

impl SemanticCommunicationClarity {
  /// Return the canonical label for this communication clarity level.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Terse => "terse",
      Self::Standard => "standard",
      Self::Verbose => "verbose",
    }
  }

  /// Parse a communication clarity level from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "terse" => Some(Self::Terse),
      "standard" => Some(Self::Standard),
      "verbose" => Some(Self::Verbose),
      _ => None,
    }
  }
}

/// Compact semantic profile definition schema and trait bundle.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticProfileDefinition {
  profile_id: &'static str,
  schema: &'static str,
  risk_tolerance: SemanticRiskTolerance,
  deference: SemanticDeference,
  focus: SemanticFocus,
  communication_clarity: SemanticCommunicationClarity,
  description: &'static str,
}

impl SemanticProfileDefinition {
  /// Construct the cautious baseline semantic profile definition.
  pub const fn cautious_v1() -> Self {
    Self {
      profile_id: CAUTIOUS_SEMANTIC_PROFILE_ID,
      schema: SEMANTIC_PROFILE_VOCABULARY_SCHEMA,
      risk_tolerance: SemanticRiskTolerance::Cautious,
      deference: SemanticDeference::Autonomous,
      focus: SemanticFocus::Patience,
      communication_clarity: SemanticCommunicationClarity::Terse,
      description: "Cautious autonomous laner prioritizing lane stabilization and threat retreat.",
    }
  }

  /// Construct the risk-taking baseline semantic profile definition.
  pub const fn risk_taking_v1() -> Self {
    Self {
      profile_id: RISK_TAKING_SEMANTIC_PROFILE_ID,
      schema: SEMANTIC_PROFILE_VOCABULARY_SCHEMA,
      risk_tolerance: SemanticRiskTolerance::RiskSeeking,
      deference: SemanticDeference::Autonomous,
      focus: SemanticFocus::Opportunity,
      communication_clarity: SemanticCommunicationClarity::Standard,
      description: "Risk-seeking autonomous laner prioritizing contest opportunities.",
    }
  }

  /// Construct the yielding baseline semantic profile definition.
  pub const fn yielding_v1() -> Self {
    Self {
      profile_id: YIELDING_SEMANTIC_PROFILE_ID,
      schema: SEMANTIC_PROFILE_VOCABULARY_SCHEMA,
      risk_tolerance: SemanticRiskTolerance::Cautious,
      deference: SemanticDeference::Yielding,
      focus: SemanticFocus::Patience,
      communication_clarity: SemanticCommunicationClarity::Terse,
      description: "Yielding laner deferring contest to avoid confrontation.",
    }
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn risk_tolerance(self) -> SemanticRiskTolerance {
    self.risk_tolerance
  }

  pub const fn deference(self) -> SemanticDeference {
    self.deference
  }

  pub const fn focus(self) -> SemanticFocus {
    self.focus
  }

  pub const fn communication_clarity(self) -> SemanticCommunicationClarity {
    self.communication_clarity
  }

  pub const fn description(self) -> &'static str {
    self.description
  }
}

/// Errors raised when parsing or validating semantic profile vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticProfileVocabularyError {
  UnknownProfile,
}

/// Canonical catalog of semantic profile vocabulary entries.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticProfileVocabulary;

impl SemanticProfileVocabulary {
  /// Return all registered canonical semantic profile definitions.
  pub const fn all_profiles() -> [SemanticProfileDefinition; 3] {
    [
      SemanticProfileDefinition::cautious_v1(),
      SemanticProfileDefinition::risk_taking_v1(),
      SemanticProfileDefinition::yielding_v1(),
    ]
  }

  /// Lookup a semantic profile definition by its stable ID.
  pub fn lookup(profile_id: &str) -> Option<SemanticProfileDefinition> {
    match profile_id {
      CAUTIOUS_SEMANTIC_PROFILE_ID => Some(SemanticProfileDefinition::cautious_v1()),
      RISK_TAKING_SEMANTIC_PROFILE_ID => Some(SemanticProfileDefinition::risk_taking_v1()),
      YIELDING_SEMANTIC_PROFILE_ID => Some(SemanticProfileDefinition::yielding_v1()),
      _ => None,
    }
  }

  /// Validate that a profile ID exists in the vocabulary.
  pub fn validate_profile_id(
    profile_id: &str,
  ) -> Result<SemanticProfileDefinition, SemanticProfileVocabularyError> {
    Self::lookup(profile_id).ok_or(SemanticProfileVocabularyError::UnknownProfile)
  }
}

/// Compact diagnostic choice domain covering core lane decision dilemmas.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiagnosticChoiceDomain {
  /// Contesting space vs yielding position.
  ContestConcede,
  /// Following allied calls vs acting autonomously.
  FollowReject,
  /// Solo resource farming vs rotating to assist.
  FarmAssist,
  /// Greedy lane stay vs timely recall reset.
  RecallTiming,
  /// Holding ground under threat vs self-preservation.
  Sacrifice,
  /// Adapting to sudden threat vs holding prior posture.
  Surprise,
  /// Resetting after unfavorable outcome vs doubling down.
  ResponseToFailure,
}

impl DiagnosticChoiceDomain {
  /// Return the canonical label for this choice domain.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ContestConcede => "contest-concede",
      Self::FollowReject => "follow-reject",
      Self::FarmAssist => "farm-assist",
      Self::RecallTiming => "recall-timing",
      Self::Sacrifice => "sacrifice",
      Self::Surprise => "surprise",
      Self::ResponseToFailure => "response-to-failure",
    }
  }

  /// Parse a choice domain from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "contest-concede" => Some(Self::ContestConcede),
      "follow-reject" => Some(Self::FollowReject),
      "farm-assist" => Some(Self::FarmAssist),
      "recall-timing" => Some(Self::RecallTiming),
      "sacrifice" => Some(Self::Sacrifice),
      "surprise" => Some(Self::Surprise),
      "response-to-failure" => Some(Self::ResponseToFailure),
      _ => None,
    }
  }
}

/// Compact diagnostic choice definition schema and contrast metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceDefinition {
  choice_id: &'static str,
  schema: &'static str,
  domain: DiagnosticChoiceDomain,
  primary_intent: LaneIntent,
  alternative_intent: LaneIntent,
  intended_contrast: &'static str,
  description: &'static str,
}

impl DiagnosticChoiceDefinition {
  /// Construct the canonical contest/concede diagnostic choice definition.
  pub const fn contest_concede_v1() -> Self {
    Self {
      choice_id: CHOICE_CONTEST_CONCEDE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::ContestConcede,
      primary_intent: LaneIntent::Contest,
      alternative_intent: LaneIntent::Yield,
      intended_contrast: "Contesting space vs yielding position to preserve survivability.",
      description: "Diagnostic choice contrasting contested wave/objective assertiveness against tactical concession.",
    }
  }

  /// Construct the canonical follow/reject diagnostic choice definition.
  pub const fn follow_reject_v1() -> Self {
    Self {
      choice_id: CHOICE_FOLLOW_REJECT_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::FollowReject,
      primary_intent: LaneIntent::Contest,
      alternative_intent: LaneIntent::Stabilize,
      intended_contrast: "Accepting allied coordinated contest vs autonomous lane stabilization.",
      description: "Diagnostic choice contrasting adherence to allied coordination proposals against autonomous action.",
    }
  }

  /// Construct the canonical farm/assist diagnostic choice definition.
  pub const fn farm_assist_v1() -> Self {
    Self {
      choice_id: CHOICE_FARM_ASSIST_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::FarmAssist,
      primary_intent: LaneIntent::Stabilize,
      alternative_intent: LaneIntent::Contest,
      intended_contrast: "Farming wave resources in lane vs committing to assist forward contest.",
      description: "Diagnostic choice contrasting solo resource farming against assisting allied engagements.",
    }
  }

  /// Construct the canonical recall timing diagnostic choice definition.
  pub const fn recall_timing_v1() -> Self {
    Self {
      choice_id: CHOICE_RECALL_TIMING_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::RecallTiming,
      primary_intent: LaneIntent::Recall,
      alternative_intent: LaneIntent::Stabilize,
      intended_contrast: "Executing timely recall to reset vs greedily remaining in lane to stabilize wave.",
      description: "Diagnostic choice contrasting proactive recall resets against high-risk wave stabilization.",
    }
  }

  /// Construct the canonical sacrifice diagnostic choice definition.
  pub const fn sacrifice_v1() -> Self {
    Self {
      choice_id: CHOICE_SACRIFICE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::Sacrifice,
      primary_intent: LaneIntent::Contest,
      alternative_intent: LaneIntent::Withdraw,
      intended_contrast: "Holding ground despite attrition danger vs withdrawing to preserve health.",
      description: "Diagnostic choice contrasting objective defense at personal cost against self-preservation.",
    }
  }

  /// Construct the canonical surprise diagnostic choice definition.
  pub const fn surprise_v1() -> Self {
    Self {
      choice_id: CHOICE_SURPRISE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::Surprise,
      primary_intent: LaneIntent::Withdraw,
      alternative_intent: LaneIntent::Stabilize,
      intended_contrast: "Immediate threat withdrawal vs standing ground under unexpected pressure.",
      description: "Diagnostic choice contrasting reactive threat retreat against holding standard posture when surprised.",
    }
  }

  /// Construct the canonical response to failure diagnostic choice definition.
  pub const fn response_to_failure_v1() -> Self {
    Self {
      choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
      schema: DIAGNOSTIC_CHOICE_CATALOG_SCHEMA,
      domain: DiagnosticChoiceDomain::ResponseToFailure,
      primary_intent: LaneIntent::Yield,
      alternative_intent: LaneIntent::Contest,
      intended_contrast: "Yielding space after an unfavorable exchange vs doubling down on contest.",
      description: "Diagnostic choice contrasting risk reduction and tactical reset against persistent escalation after failure.",
    }
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn domain(self) -> DiagnosticChoiceDomain {
    self.domain
  }

  pub const fn primary_intent(self) -> LaneIntent {
    self.primary_intent
  }

  pub const fn alternative_intent(self) -> LaneIntent {
    self.alternative_intent
  }

  pub const fn intended_contrast(self) -> &'static str {
    self.intended_contrast
  }

  pub const fn description(self) -> &'static str {
    self.description
  }
}

/// Errors raised when validating diagnostic choice catalog lookups.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DiagnosticChoiceCatalogError {
  UnknownChoice,
}

/// Canonical catalog of diagnostic choice definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceCatalog;

impl DiagnosticChoiceCatalog {
  /// Return all registered canonical diagnostic choice definitions.
  pub const fn all_choices() -> [DiagnosticChoiceDefinition; 7] {
    [
      DiagnosticChoiceDefinition::contest_concede_v1(),
      DiagnosticChoiceDefinition::follow_reject_v1(),
      DiagnosticChoiceDefinition::farm_assist_v1(),
      DiagnosticChoiceDefinition::recall_timing_v1(),
      DiagnosticChoiceDefinition::sacrifice_v1(),
      DiagnosticChoiceDefinition::surprise_v1(),
      DiagnosticChoiceDefinition::response_to_failure_v1(),
    ]
  }

  /// Lookup a diagnostic choice definition by its stable ID.
  pub fn lookup(choice_id: &str) -> Option<DiagnosticChoiceDefinition> {
    match choice_id {
      CHOICE_CONTEST_CONCEDE_ID => Some(DiagnosticChoiceDefinition::contest_concede_v1()),
      CHOICE_FOLLOW_REJECT_ID => Some(DiagnosticChoiceDefinition::follow_reject_v1()),
      CHOICE_FARM_ASSIST_ID => Some(DiagnosticChoiceDefinition::farm_assist_v1()),
      CHOICE_RECALL_TIMING_ID => Some(DiagnosticChoiceDefinition::recall_timing_v1()),
      CHOICE_SACRIFICE_ID => Some(DiagnosticChoiceDefinition::sacrifice_v1()),
      CHOICE_SURPRISE_ID => Some(DiagnosticChoiceDefinition::surprise_v1()),
      CHOICE_RESPONSE_TO_FAILURE_ID => Some(DiagnosticChoiceDefinition::response_to_failure_v1()),
      _ => None,
    }
  }

  /// Validate that a choice ID exists in the catalog.
  pub fn validate_choice_id(
    choice_id: &str,
  ) -> Result<DiagnosticChoiceDefinition, DiagnosticChoiceCatalogError> {
    Self::lookup(choice_id).ok_or(DiagnosticChoiceCatalogError::UnknownChoice)
  }

  /// Return the canonical choice definition for a given domain.
  pub const fn choice_for_domain(domain: DiagnosticChoiceDomain) -> DiagnosticChoiceDefinition {
    match domain {
      DiagnosticChoiceDomain::ContestConcede => DiagnosticChoiceDefinition::contest_concede_v1(),
      DiagnosticChoiceDomain::FollowReject => DiagnosticChoiceDefinition::follow_reject_v1(),
      DiagnosticChoiceDomain::FarmAssist => DiagnosticChoiceDefinition::farm_assist_v1(),
      DiagnosticChoiceDomain::RecallTiming => DiagnosticChoiceDefinition::recall_timing_v1(),
      DiagnosticChoiceDomain::Sacrifice => DiagnosticChoiceDefinition::sacrifice_v1(),
      DiagnosticChoiceDomain::Surprise => DiagnosticChoiceDefinition::surprise_v1(),
      DiagnosticChoiceDomain::ResponseToFailure => {
        DiagnosticChoiceDefinition::response_to_failure_v1()
      }
    }
  }
}

/// Errors raised when validating model and prompt version protocol definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ModelPromptProtocolError {
  UnknownProtocol,
  InvalidTemperature,
  InvalidTopP,
  PrivateChainOfThoughtForbidden,
}

/// Structured protocol for model family, prompt template, and sampling parameter bounds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModelPromptProtocolDefinition {
  protocol_id: &'static str,
  schema: &'static str,
  model_family_id: &'static str,
  prompt_template_id: &'static str,
  system_prompt_version: &'static str,
  temperature_centiperc: u16,
  top_p_centiperc: u16,
  requires_structured_output: bool,
  chain_of_thought_required: bool,
}

impl ModelPromptProtocolDefinition {
  /// Reference model with standard decision prompt protocol.
  pub const fn reference_standard_v1() -> Self {
    Self {
      protocol_id: MODEL_PROMPT_REFERENCE_STANDARD_ID,
      schema: MODEL_PROMPT_PROTOCOL_SCHEMA,
      model_family_id: "model-family-reference-v1",
      prompt_template_id: "prompt-template-lane-standard-v1",
      system_prompt_version: "sysprompt-actor-contract-v1",
      temperature_centiperc: 70,
      top_p_centiperc: 95,
      requires_structured_output: true,
      chain_of_thought_required: false,
    }
  }

  /// Reference model with diagnostic dilemma prompt protocol.
  pub const fn reference_diagnostic_v1() -> Self {
    Self {
      protocol_id: MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      schema: MODEL_PROMPT_PROTOCOL_SCHEMA,
      model_family_id: "model-family-reference-v1",
      prompt_template_id: "prompt-template-lane-diagnostic-v1",
      system_prompt_version: "sysprompt-actor-contract-v1",
      temperature_centiperc: 50,
      top_p_centiperc: 90,
      requires_structured_output: true,
      chain_of_thought_required: false,
    }
  }

  /// Alternative model family with diagnostic dilemma prompt protocol.
  pub const fn alternative_diagnostic_v1() -> Self {
    Self {
      protocol_id: MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID,
      schema: MODEL_PROMPT_PROTOCOL_SCHEMA,
      model_family_id: "model-family-alternative-v1",
      prompt_template_id: "prompt-template-lane-diagnostic-v1",
      system_prompt_version: "sysprompt-actor-contract-v1",
      temperature_centiperc: 50,
      top_p_centiperc: 90,
      requires_structured_output: true,
      chain_of_thought_required: false,
    }
  }

  pub const fn protocol_id(self) -> &'static str {
    self.protocol_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn model_family_id(self) -> &'static str {
    self.model_family_id
  }

  pub const fn prompt_template_id(self) -> &'static str {
    self.prompt_template_id
  }

  pub const fn system_prompt_version(self) -> &'static str {
    self.system_prompt_version
  }

  pub const fn temperature_centiperc(self) -> u16 {
    self.temperature_centiperc
  }

  pub const fn top_p_centiperc(self) -> u16 {
    self.top_p_centiperc
  }

  pub const fn requires_structured_output(self) -> bool {
    self.requires_structured_output
  }

  pub const fn chain_of_thought_required(self) -> bool {
    self.chain_of_thought_required
  }

  /// Validate protocol bounds.
  pub fn validate(self) -> Result<(), ModelPromptProtocolError> {
    if self.temperature_centiperc > 200 {
      return Err(ModelPromptProtocolError::InvalidTemperature);
    }
    if self.top_p_centiperc > 100 {
      return Err(ModelPromptProtocolError::InvalidTopP);
    }
    if self.chain_of_thought_required {
      return Err(ModelPromptProtocolError::PrivateChainOfThoughtForbidden);
    }
    Ok(())
  }
}

/// Catalog of canonical model and prompt protocols.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ModelPromptProtocolCatalog;

impl ModelPromptProtocolCatalog {
  /// Return all registered canonical model and prompt protocols.
  pub const fn all_protocols() -> [ModelPromptProtocolDefinition; 3] {
    [
      ModelPromptProtocolDefinition::reference_standard_v1(),
      ModelPromptProtocolDefinition::reference_diagnostic_v1(),
      ModelPromptProtocolDefinition::alternative_diagnostic_v1(),
    ]
  }

  /// Lookup a model/prompt protocol by its stable ID.
  pub fn lookup(protocol_id: &str) -> Option<ModelPromptProtocolDefinition> {
    match protocol_id {
      MODEL_PROMPT_REFERENCE_STANDARD_ID => {
        Some(ModelPromptProtocolDefinition::reference_standard_v1())
      }
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID => {
        Some(ModelPromptProtocolDefinition::reference_diagnostic_v1())
      }
      MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID => {
        Some(ModelPromptProtocolDefinition::alternative_diagnostic_v1())
      }
      _ => None,
    }
  }

  /// Validate that a protocol ID exists in the catalog and meets bounds.
  pub fn validate_protocol_id(
    protocol_id: &str,
  ) -> Result<ModelPromptProtocolDefinition, ModelPromptProtocolError> {
    let def = Self::lookup(protocol_id).ok_or(ModelPromptProtocolError::UnknownProtocol)?;
    def.validate()?;
    Ok(def)
  }
}

/// Errors raised when validating repeated-sampling protocol definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RepeatedSamplingProtocolError {
  UnknownProtocol,
  InvalidSampleCount,
  InvalidMaxRetries,
  InvalidSeedOffsetStep,
}

/// Structured protocol for repeated empirical sampling schedules and retry budgets.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RepeatedSamplingProtocolDefinition {
  protocol_id: &'static str,
  schema: &'static str,
  sample_count: u16,
  seed_offset_step: u32,
  max_repair_retries: u8,
  fail_closed_on_unrepaired: bool,
}

impl RepeatedSamplingProtocolDefinition {
  /// Standard 10-repeat sampling protocol.
  pub const fn standard_repeat_10_v1() -> Self {
    Self {
      protocol_id: SAMPLING_STANDARD_REPEAT_10_ID,
      schema: REPEATED_SAMPLING_PROTOCOL_SCHEMA,
      sample_count: 10,
      seed_offset_step: 1,
      max_repair_retries: 3,
      fail_closed_on_unrepaired: true,
    }
  }

  /// Comprehensive diagnostic 30-repeat sampling protocol.
  pub const fn diagnostic_repeat_30_v1() -> Self {
    Self {
      protocol_id: SAMPLING_DIAGNOSTIC_REPEAT_30_ID,
      schema: REPEATED_SAMPLING_PROTOCOL_SCHEMA,
      sample_count: 30,
      seed_offset_step: 1,
      max_repair_retries: 3,
      fail_closed_on_unrepaired: true,
    }
  }

  /// Quick check 5-repeat sampling protocol.
  pub const fn quick_check_5_v1() -> Self {
    Self {
      protocol_id: SAMPLING_QUICK_CHECK_5_ID,
      schema: REPEATED_SAMPLING_PROTOCOL_SCHEMA,
      sample_count: 5,
      seed_offset_step: 1,
      max_repair_retries: 2,
      fail_closed_on_unrepaired: true,
    }
  }

  pub const fn protocol_id(self) -> &'static str {
    self.protocol_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn sample_count(self) -> u16 {
    self.sample_count
  }

  pub const fn seed_offset_step(self) -> u32 {
    self.seed_offset_step
  }

  pub const fn max_repair_retries(self) -> u8 {
    self.max_repair_retries
  }

  pub const fn fail_closed_on_unrepaired(self) -> bool {
    self.fail_closed_on_unrepaired
  }

  /// Validate protocol bounds.
  pub fn validate(self) -> Result<(), RepeatedSamplingProtocolError> {
    if self.sample_count == 0 || self.sample_count > 100 {
      return Err(RepeatedSamplingProtocolError::InvalidSampleCount);
    }
    if self.seed_offset_step == 0 {
      return Err(RepeatedSamplingProtocolError::InvalidSeedOffsetStep);
    }
    if self.max_repair_retries > 10 {
      return Err(RepeatedSamplingProtocolError::InvalidMaxRetries);
    }
    Ok(())
  }
}

/// Catalog of canonical repeated sampling protocols.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RepeatedSamplingProtocolCatalog;

impl RepeatedSamplingProtocolCatalog {
  /// Return all registered canonical repeated sampling protocols.
  pub const fn all_protocols() -> [RepeatedSamplingProtocolDefinition; 3] {
    [
      RepeatedSamplingProtocolDefinition::standard_repeat_10_v1(),
      RepeatedSamplingProtocolDefinition::diagnostic_repeat_30_v1(),
      RepeatedSamplingProtocolDefinition::quick_check_5_v1(),
    ]
  }

  /// Lookup a repeated sampling protocol by its stable ID.
  pub fn lookup(protocol_id: &str) -> Option<RepeatedSamplingProtocolDefinition> {
    match protocol_id {
      SAMPLING_STANDARD_REPEAT_10_ID => {
        Some(RepeatedSamplingProtocolDefinition::standard_repeat_10_v1())
      }
      SAMPLING_DIAGNOSTIC_REPEAT_30_ID => {
        Some(RepeatedSamplingProtocolDefinition::diagnostic_repeat_30_v1())
      }
      SAMPLING_QUICK_CHECK_5_ID => Some(RepeatedSamplingProtocolDefinition::quick_check_5_v1()),
      _ => None,
    }
  }

  /// Validate that a protocol ID exists in the catalog and meets bounds.
  pub fn validate_protocol_id(
    protocol_id: &str,
  ) -> Result<RepeatedSamplingProtocolDefinition, RepeatedSamplingProtocolError> {
    let def = Self::lookup(protocol_id).ok_or(RepeatedSamplingProtocolError::UnknownProtocol)?;
    def.validate()?;
    Ok(def)
  }
}

/// Errors raised when validating empirical distribution estimates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EmpiricalDistributionEstimationError {
  UnknownProfile,
  UnknownChoice,
  UnknownSamplingProtocol,
  UnknownModelPromptProtocol,
  InvalidSampleCount,
  CountSumMismatch,
  MismatchedChoice,
  MismatchedProfile,
}

/// Bounded empirical action distribution over repeated samples for a diagnostic choice dilemma.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceActionDistribution {
  schema: &'static str,
  choice_id: &'static str,
  profile_id: &'static str,
  primary_intent: LaneIntent,
  alternative_intent: LaneIntent,
  sample_count: u16,
  primary_count: u16,
  alternative_count: u16,
  other_count: u16,
}

impl DiagnosticChoiceActionDistribution {
  /// Create and validate a new diagnostic choice action distribution.
  pub fn new(
    choice_id: &'static str,
    profile_id: &'static str,
    sample_count: u16,
    primary_count: u16,
    alternative_count: u16,
    other_count: u16,
  ) -> Result<Self, EmpiricalDistributionEstimationError> {
    SemanticProfileVocabulary::validate_profile_id(profile_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownProfile)?;
    let choice = DiagnosticChoiceCatalog::validate_choice_id(choice_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownChoice)?;

    if sample_count == 0 || sample_count > 100 {
      return Err(EmpiricalDistributionEstimationError::InvalidSampleCount);
    }
    if primary_count + alternative_count + other_count != sample_count {
      return Err(EmpiricalDistributionEstimationError::CountSumMismatch);
    }

    Ok(Self {
      schema: EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA,
      choice_id,
      profile_id,
      primary_intent: choice.primary_intent(),
      alternative_intent: choice.alternative_intent(),
      sample_count,
      primary_count,
      alternative_count,
      other_count,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn primary_intent(self) -> LaneIntent {
    self.primary_intent
  }

  pub const fn alternative_intent(self) -> LaneIntent {
    self.alternative_intent
  }

  pub const fn sample_count(self) -> u16 {
    self.sample_count
  }

  pub const fn primary_count(self) -> u16 {
    self.primary_count
  }

  pub const fn alternative_count(self) -> u16 {
    self.alternative_count
  }

  pub const fn other_count(self) -> u16 {
    self.other_count
  }

  /// Return `[primary, alternative, other]` basis-point shares scaled to 10,000.
  ///
  /// The primary and alternative shares use integer floor division; the
  /// other share receives the remainder so the three shares always sum to 10,000.
  pub fn basis_points(self) -> [u16; 3] {
    let scale = u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let total = u32::from(self.sample_count);
    let primary_bp = u16::try_from(u32::from(self.primary_count) * scale / total)
      .expect("primary basis points fit in u16");
    let alt_bp = u16::try_from(u32::from(self.alternative_count) * scale / total)
      .expect("alternative basis points fit in u16");
    let other_bp = EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS - (primary_bp + alt_bp);
    [primary_bp, alt_bp, other_bp]
  }

  pub fn primary_share_basis_points(self) -> u16 {
    self.basis_points()[0]
  }

  pub fn alternative_share_basis_points(self) -> u16 {
    self.basis_points()[1]
  }

  pub fn other_share_basis_points(self) -> u16 {
    self.basis_points()[2]
  }

  /// Render the distribution as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let bp = self.basis_points();
    format!(
      "| {} | {} | {} | {} | {} | {} |\n",
      self.choice_id, self.sample_count, self.primary_count, bp[0], self.alternative_count, bp[1],
    )
  }
}

/// Bounded empirical communication ping signal distribution over repeated samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DiagnosticChoiceCommunicationDistribution {
  schema: &'static str,
  choice_id: &'static str,
  profile_id: &'static str,
  sample_count: u16,
  signal_counts: [u16; 5],
}

impl DiagnosticChoiceCommunicationDistribution {
  /// Create and validate a new diagnostic choice communication distribution.
  pub fn new(
    choice_id: &'static str,
    profile_id: &'static str,
    sample_count: u16,
    signal_counts: [u16; 5],
  ) -> Result<Self, EmpiricalDistributionEstimationError> {
    SemanticProfileVocabulary::validate_profile_id(profile_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownProfile)?;
    DiagnosticChoiceCatalog::validate_choice_id(choice_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownChoice)?;

    if sample_count == 0 || sample_count > 100 {
      return Err(EmpiricalDistributionEstimationError::InvalidSampleCount);
    }
    let total_signals =
      signal_counts[0] + signal_counts[1] + signal_counts[2] + signal_counts[3] + signal_counts[4];
    if total_signals != sample_count {
      return Err(EmpiricalDistributionEstimationError::CountSumMismatch);
    }

    Ok(Self {
      schema: EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA,
      choice_id,
      profile_id,
      sample_count,
      signal_counts,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn choice_id(self) -> &'static str {
    self.choice_id
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn sample_count(self) -> u16 {
    self.sample_count
  }

  pub const fn signal_counts(self) -> [u16; 5] {
    self.signal_counts
  }

  /// Return `[None, Danger, OnMyWay, Assist, EnemyMissing]` basis-point shares scaled to 10,000.
  pub fn basis_points(self) -> [u16; 5] {
    let scale = u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let total = u32::from(self.sample_count);
    let mut shares = [0_u16; 5];
    let mut assigned = 0_u16;
    for (idx, count) in self.signal_counts.iter().take(4).enumerate() {
      shares[idx] =
        u16::try_from(u32::from(*count) * scale / total).expect("signal basis points fit in u16");
      assigned += shares[idx];
    }
    shares[4] = EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS - assigned;
    shares
  }

  pub fn signal_share_basis_points(self, signal: LanePingSignal) -> u16 {
    let idx = match signal {
      LanePingSignal::None => 0,
      LanePingSignal::Danger => 1,
      LanePingSignal::OnMyWay => 2,
      LanePingSignal::Assist => 3,
      LanePingSignal::EnemyMissing => 4,
    };
    self.basis_points()[idx]
  }

  /// Render the communication distribution as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let bp = self.basis_points();
    format!(
      "| {} | {} | {} | {} | {} | {} | {} |\n",
      self.choice_id, self.sample_count, bp[0], bp[1], bp[2], bp[3], bp[4],
    )
  }
}

/// Comprehensive empirical action and communication distribution estimate report over all 7 diagnostic dilemmas.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmpiricalDistributionEstimateReport {
  schema: &'static str,
  profile_id: &'static str,
  sampling_protocol_id: &'static str,
  model_prompt_protocol_id: &'static str,
  action_distributions: [DiagnosticChoiceActionDistribution; 7],
  communication_distributions: [DiagnosticChoiceCommunicationDistribution; 7],
}

impl EmpiricalDistributionEstimateReport {
  /// Create and validate a complete empirical distribution estimate report.
  pub fn new(
    profile_id: &'static str,
    sampling_protocol_id: &'static str,
    model_prompt_protocol_id: &'static str,
    action_distributions: [DiagnosticChoiceActionDistribution; 7],
    communication_distributions: [DiagnosticChoiceCommunicationDistribution; 7],
  ) -> Result<Self, EmpiricalDistributionEstimationError> {
    let report = Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id,
      model_prompt_protocol_id,
      action_distributions,
      communication_distributions,
    };
    report.validate()?;
    Ok(report)
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn sampling_protocol_id(&self) -> &'static str {
    self.sampling_protocol_id
  }

  pub const fn model_prompt_protocol_id(&self) -> &'static str {
    self.model_prompt_protocol_id
  }

  pub const fn action_distributions(&self) -> &[DiagnosticChoiceActionDistribution; 7] {
    &self.action_distributions
  }

  pub const fn communication_distributions(
    &self,
  ) -> &[DiagnosticChoiceCommunicationDistribution; 7] {
    &self.communication_distributions
  }

  /// Validate report consistency against registered catalogs and ordering.
  pub fn validate(&self) -> Result<(), EmpiricalDistributionEstimationError> {
    SemanticProfileVocabulary::validate_profile_id(self.profile_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownProfile)?;
    RepeatedSamplingProtocolCatalog::validate_protocol_id(self.sampling_protocol_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownSamplingProtocol)?;
    ModelPromptProtocolCatalog::validate_protocol_id(self.model_prompt_protocol_id)
      .map_err(|_| EmpiricalDistributionEstimationError::UnknownModelPromptProtocol)?;

    let canonical_choices = DiagnosticChoiceCatalog::all_choices();
    for (i, action_dist) in self.action_distributions.iter().enumerate() {
      if action_dist.profile_id() != self.profile_id {
        return Err(EmpiricalDistributionEstimationError::MismatchedProfile);
      }
      if action_dist.choice_id() != canonical_choices[i].choice_id() {
        return Err(EmpiricalDistributionEstimationError::MismatchedChoice);
      }
    }
    for (i, comm_dist) in self.communication_distributions.iter().enumerate() {
      if comm_dist.profile_id() != self.profile_id {
        return Err(EmpiricalDistributionEstimationError::MismatchedProfile);
      }
      if comm_dist.choice_id() != canonical_choices[i].choice_id() {
        return Err(EmpiricalDistributionEstimationError::MismatchedChoice);
      }
    }
    Ok(())
  }

  /// Canonical empirical distribution estimate for the cautious semantic profile.
  pub fn cautious_v1() -> Self {
    let profile_id = CAUTIOUS_SEMANTIC_PROFILE_ID;
    let sampling_id = SAMPLING_STANDARD_REPEAT_10_ID;
    let prompt_id = MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID;

    let action_dists = [
      DiagnosticChoiceActionDistribution::new(CHOICE_CONTEST_CONCEDE_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FOLLOW_REJECT_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FARM_ASSIST_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_RECALL_TIMING_ID, profile_id, 10, 8, 2, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SACRIFICE_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SURPRISE_ID, profile_id, 10, 10, 0, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        9,
        1,
        0,
      )
      .expect("valid"),
    ];

    let comm_dists = [
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_CONTEST_CONCEDE_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FOLLOW_REJECT_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FARM_ASSIST_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RECALL_TIMING_ID,
        profile_id,
        10,
        [9, 0, 1, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SACRIFICE_ID,
        profile_id,
        10,
        [7, 3, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SURPRISE_ID,
        profile_id,
        10,
        [5, 5, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        [8, 2, 0, 0, 0],
      )
      .expect("valid"),
    ];

    Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id: sampling_id,
      model_prompt_protocol_id: prompt_id,
      action_distributions: action_dists,
      communication_distributions: comm_dists,
    }
  }

  /// Canonical empirical distribution estimate for the risk-taking semantic profile.
  pub fn risk_taking_v1() -> Self {
    let profile_id = RISK_TAKING_SEMANTIC_PROFILE_ID;
    let sampling_id = SAMPLING_STANDARD_REPEAT_10_ID;
    let prompt_id = MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID;

    let action_dists = [
      DiagnosticChoiceActionDistribution::new(CHOICE_CONTEST_CONCEDE_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FOLLOW_REJECT_ID, profile_id, 10, 8, 2, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FARM_ASSIST_ID, profile_id, 10, 3, 7, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_RECALL_TIMING_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SACRIFICE_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SURPRISE_ID, profile_id, 10, 2, 8, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        1,
        9,
        0,
      )
      .expect("valid"),
    ];

    let comm_dists = [
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_CONTEST_CONCEDE_ID,
        profile_id,
        10,
        [8, 0, 0, 2, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FOLLOW_REJECT_ID,
        profile_id,
        10,
        [8, 0, 2, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FARM_ASSIST_ID,
        profile_id,
        10,
        [7, 0, 3, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RECALL_TIMING_ID,
        profile_id,
        10,
        [9, 0, 1, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SACRIFICE_ID,
        profile_id,
        10,
        [6, 0, 0, 4, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SURPRISE_ID,
        profile_id,
        10,
        [8, 2, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        [7, 0, 0, 3, 0],
      )
      .expect("valid"),
    ];

    Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id: sampling_id,
      model_prompt_protocol_id: prompt_id,
      action_distributions: action_dists,
      communication_distributions: comm_dists,
    }
  }

  /// Canonical empirical distribution estimate for the yielding semantic profile.
  pub fn yielding_v1() -> Self {
    let profile_id = YIELDING_SEMANTIC_PROFILE_ID;
    let sampling_id = SAMPLING_STANDARD_REPEAT_10_ID;
    let prompt_id = MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID;

    let action_dists = [
      DiagnosticChoiceActionDistribution::new(CHOICE_CONTEST_CONCEDE_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FOLLOW_REJECT_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_FARM_ASSIST_ID, profile_id, 10, 8, 2, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_RECALL_TIMING_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SACRIFICE_ID, profile_id, 10, 1, 9, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(CHOICE_SURPRISE_ID, profile_id, 10, 9, 1, 0)
        .expect("valid"),
      DiagnosticChoiceActionDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        10,
        0,
        0,
      )
      .expect("valid"),
    ];

    let comm_dists = [
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_CONTEST_CONCEDE_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FOLLOW_REJECT_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_FARM_ASSIST_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RECALL_TIMING_ID,
        profile_id,
        10,
        [10, 0, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SACRIFICE_ID,
        profile_id,
        10,
        [8, 2, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_SURPRISE_ID,
        profile_id,
        10,
        [6, 4, 0, 0, 0],
      )
      .expect("valid"),
      DiagnosticChoiceCommunicationDistribution::new(
        CHOICE_RESPONSE_TO_FAILURE_ID,
        profile_id,
        10,
        [9, 1, 0, 0, 0],
      )
      .expect("valid"),
    ];

    Self {
      schema: EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA,
      profile_id,
      sampling_protocol_id: sampling_id,
      model_prompt_protocol_id: prompt_id,
      action_distributions: action_dists,
      communication_distributions: comm_dists,
    }
  }

  /// Render the empirical distribution estimate report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Empirical Distribution Estimate Report\n\n- schema: {}\n- profile_id: {}\n- sampling_protocol_id: {}\n- model_prompt_protocol_id: {}\n- scale_basis_points: {}\n\n## Action Distributions\n\n| choice_id | sample_count | primary_count | primary_bp | alternative_count | alternative_bp |\n| --- | ---: | ---: | ---: | ---: | ---: |\n",
      self.schema,
      self.profile_id,
      self.sampling_protocol_id,
      self.model_prompt_protocol_id,
      EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS,
    );
    for dist in &self.action_distributions {
      out.push_str(&dist.to_markdown());
    }
    out.push_str("\n## Communication Distributions\n\n| choice_id | sample_count | none_bp | danger_bp | on_my_way_bp | assist_bp | enemy_missing_bp |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for dist in &self.communication_distributions {
      out.push_str(&dist.to_markdown());
    }
    out
  }
}

/// Errors raised when evaluating or comparing behavioral measures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BehavioralMeasuresError {
  MismatchedChoice,
  MismatchedProfile,
}

/// Bounded behavioral distance calculator (Total Variation Distance in integer basis points [0..=10,000]).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralDistanceMeasure;

impl BehavioralDistanceMeasure {
  /// Calculate Total Variation Distance (TVD) between two action distributions over 3 categories [primary, alternative, other].
  ///
  /// TVD(P, Q) = 1/2 * sum(|P_i - Q_i|) in basis points.
  /// Result is in 0..=10,000 basis points.
  pub fn action_tvd(
    a: DiagnosticChoiceActionDistribution,
    b: DiagnosticChoiceActionDistribution,
  ) -> u16 {
    let bp_a = a.basis_points();
    let bp_b = b.basis_points();
    let diff_primary = u32::from(bp_a[0].abs_diff(bp_b[0]));
    let diff_alt = u32::from(bp_a[1].abs_diff(bp_b[1]));
    let diff_other = u32::from(bp_a[2].abs_diff(bp_b[2]));
    let sum_diff = diff_primary + diff_alt + diff_other;
    u16::try_from(sum_diff / 2).expect("tvd fits in u16")
  }

  /// Calculate Total Variation Distance (TVD) between two communication distributions over 5 signal categories.
  ///
  /// TVD(P, Q) = 1/2 * sum(|P_i - Q_i|) in basis points.
  pub fn communication_tvd(
    a: DiagnosticChoiceCommunicationDistribution,
    b: DiagnosticChoiceCommunicationDistribution,
  ) -> u16 {
    let bp_a = a.basis_points();
    let bp_b = b.basis_points();
    let mut sum_diff = 0_u32;
    for i in 0..5 {
      sum_diff += u32::from(bp_a[i].abs_diff(bp_b[i]));
    }
    u16::try_from(sum_diff / 2).expect("tvd fits in u16")
  }

  /// Calculate the mean action TVD across all 7 diagnostic choices.
  pub fn mean_action_distance(
    rep_a: &EmpiricalDistributionEstimateReport,
    rep_b: &EmpiricalDistributionEstimateReport,
  ) -> u16 {
    let mut sum_tvd = 0_u32;
    for i in 0..7 {
      sum_tvd += u32::from(Self::action_tvd(
        rep_a.action_distributions()[i],
        rep_b.action_distributions()[i],
      ));
    }
    u16::try_from(sum_tvd / 7).expect("mean tvd fits in u16")
  }

  /// Calculate the mean communication TVD across all 7 diagnostic choices.
  pub fn mean_communication_distance(
    rep_a: &EmpiricalDistributionEstimateReport,
    rep_b: &EmpiricalDistributionEstimateReport,
  ) -> u16 {
    let mut sum_tvd = 0_u32;
    for i in 0..7 {
      sum_tvd += u32::from(Self::communication_tvd(
        rep_a.communication_distributions()[i],
        rep_b.communication_distributions()[i],
      ));
    }
    u16::try_from(sum_tvd / 7).expect("mean comm tvd fits in u16")
  }
}

/// Comprehensive behavioral distance report comparing two empirical distribution estimate reports across all 7 diagnostic dilemmas.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralDistanceReport {
  schema: &'static str,
  baseline_profile_id: &'static str,
  candidate_profile_id: &'static str,
  action_choice_distances: [u16; 7],
  communication_choice_distances: [u16; 7],
  mean_action_distance_bp: u16,
  mean_communication_distance_bp: u16,
}

impl BehavioralDistanceReport {
  /// Compare two empirical distribution estimate reports across all 7 diagnostic choices.
  pub fn from_reports(
    baseline: &EmpiricalDistributionEstimateReport,
    candidate: &EmpiricalDistributionEstimateReport,
  ) -> Self {
    let mut action_choice_distances = [0_u16; 7];
    let mut communication_choice_distances = [0_u16; 7];

    for i in 0..7 {
      action_choice_distances[i] = BehavioralDistanceMeasure::action_tvd(
        baseline.action_distributions()[i],
        candidate.action_distributions()[i],
      );
      communication_choice_distances[i] = BehavioralDistanceMeasure::communication_tvd(
        baseline.communication_distributions()[i],
        candidate.communication_distributions()[i],
      );
    }

    let mean_action_distance_bp =
      BehavioralDistanceMeasure::mean_action_distance(baseline, candidate);
    let mean_communication_distance_bp =
      BehavioralDistanceMeasure::mean_communication_distance(baseline, candidate);

    Self {
      schema: BEHAVIORAL_DISTANCE_SCHEMA,
      baseline_profile_id: baseline.profile_id(),
      candidate_profile_id: candidate.profile_id(),
      action_choice_distances,
      communication_choice_distances,
      mean_action_distance_bp,
      mean_communication_distance_bp,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn baseline_profile_id(&self) -> &'static str {
    self.baseline_profile_id
  }

  pub const fn candidate_profile_id(&self) -> &'static str {
    self.candidate_profile_id
  }

  pub const fn action_choice_distances(&self) -> &[u16; 7] {
    &self.action_choice_distances
  }

  pub const fn communication_choice_distances(&self) -> &[u16; 7] {
    &self.communication_choice_distances
  }

  pub const fn mean_action_distance_bp(&self) -> u16 {
    self.mean_action_distance_bp
  }

  pub const fn mean_communication_distance_bp(&self) -> u16 {
    self.mean_communication_distance_bp
  }

  /// Render the distance report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let choices = DiagnosticChoiceCatalog::all_choices();
    let mut out = format!(
      "# Behavioral Distance Report\n\n- schema: {}\n- baseline_profile_id: {}\n- candidate_profile_id: {}\n- mean_action_distance_bp: {}\n- mean_communication_distance_bp: {}\n\n| choice_id | action_tvd_bp | communication_tvd_bp |\n| --- | ---: |\n",
      self.schema,
      self.baseline_profile_id,
      self.candidate_profile_id,
      self.mean_action_distance_bp,
      self.mean_communication_distance_bp,
    );
    for (i, choice) in choices.iter().enumerate() {
      out.push_str(&format!(
        "| {} | {} | {} |\n",
        choice.choice_id(),
        self.action_choice_distances[i],
        self.communication_choice_distances[i],
      ));
    }
    out
  }
}

/// Bounded behavioral entropy and diversity calculator (Gini diversity index in integer basis points [0..=10,000]).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralEntropyMeasure;

impl BehavioralEntropyMeasure {
  /// Calculate Gini diversity index for an action distribution: 10,000 - sum(p_i^2)/10,000.
  pub fn action_entropy(dist: DiagnosticChoiceActionDistribution) -> u16 {
    let bp = dist.basis_points();
    let sum_sq = u64::from(bp[0]) * u64::from(bp[0])
      + u64::from(bp[1]) * u64::from(bp[1])
      + u64::from(bp[2]) * u64::from(bp[2]);
    let scale = u64::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let conc = u16::try_from(sum_sq / scale).expect("concentration fits in u16");
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS.saturating_sub(conc)
  }

  /// Calculate Gini diversity index for a communication distribution.
  pub fn communication_entropy(dist: DiagnosticChoiceCommunicationDistribution) -> u16 {
    let bp = dist.basis_points();
    let mut sum_sq = 0_u64;
    for p in bp {
      sum_sq += u64::from(p) * u64::from(p);
    }
    let scale = u64::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let conc = u16::try_from(sum_sq / scale).expect("concentration fits in u16");
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS.saturating_sub(conc)
  }

  /// Calculate mean action entropy across all 7 diagnostic choices in a report.
  pub fn mean_action_entropy(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_entropy = 0_u32;
    for dist in report.action_distributions() {
      sum_entropy += u32::from(Self::action_entropy(*dist));
    }
    u16::try_from(sum_entropy / 7).expect("mean action entropy fits in u16")
  }

  /// Calculate mean communication entropy across all 7 diagnostic choices in a report.
  pub fn mean_communication_entropy(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_entropy = 0_u32;
    for dist in report.communication_distributions() {
      sum_entropy += u32::from(Self::communication_entropy(*dist));
    }
    u16::try_from(sum_entropy / 7).expect("mean comm entropy fits in u16")
  }
}

/// Bounded behavioral sensitivity calculator measuring shifts across contrasting dilemma pairs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralSensitivityMeasure;

impl BehavioralSensitivityMeasure {
  /// Calculate primary intent sensitivity between ContestConcede (idx 0) and Surprise (idx 5).
  pub fn surprise_sensitivity(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let contest_bp = report.action_distributions()[0].primary_share_basis_points();
    let surprise_bp = report.action_distributions()[5].primary_share_basis_points();
    contest_bp.abs_diff(surprise_bp)
  }

  /// Calculate primary intent sensitivity between ContestConcede (idx 0) and Sacrifice (idx 4).
  pub fn sacrifice_sensitivity(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let contest_bp = report.action_distributions()[0].primary_share_basis_points();
    let sacrifice_bp = report.action_distributions()[4].primary_share_basis_points();
    contest_bp.abs_diff(sacrifice_bp)
  }

  /// Calculate primary intent sensitivity between ContestConcede (idx 0) and ResponseToFailure (idx 6).
  pub fn failure_sensitivity(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let contest_bp = report.action_distributions()[0].primary_share_basis_points();
    let failure_bp = report.action_distributions()[6].primary_share_basis_points();
    contest_bp.abs_diff(failure_bp)
  }
}

/// Bounded behavioral consistency calculator measuring modal adherence across repeated samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralConsistencyMeasure;

impl BehavioralConsistencyMeasure {
  /// Calculate modal action consistency: max(p_i) in basis points.
  pub fn action_consistency(dist: DiagnosticChoiceActionDistribution) -> u16 {
    let bp = dist.basis_points();
    bp[0].max(bp[1]).max(bp[2])
  }

  /// Calculate modal communication consistency: max(p_i) in basis points.
  pub fn communication_consistency(dist: DiagnosticChoiceCommunicationDistribution) -> u16 {
    let bp = dist.basis_points();
    let mut max_p = 0_u16;
    for p in bp {
      if p > max_p {
        max_p = p;
      }
    }
    max_p
  }

  /// Calculate mean action consistency across all 7 diagnostic choices in a report.
  pub fn mean_action_consistency(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_cons = 0_u32;
    for dist in report.action_distributions() {
      sum_cons += u32::from(Self::action_consistency(*dist));
    }
    u16::try_from(sum_cons / 7).expect("mean action consistency fits in u16")
  }

  /// Calculate mean communication consistency across all 7 diagnostic choices in a report.
  pub fn mean_communication_consistency(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let mut sum_cons = 0_u32;
    for dist in report.communication_distributions() {
      sum_cons += u32::from(Self::communication_consistency(*dist));
    }
    u16::try_from(sum_cons / 7).expect("mean comm consistency fits in u16")
  }
}

/// Bounded behavioral adaptation calculator measuring defensive adjustment under adverse dilemmas.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralAdaptationMeasure;

impl BehavioralAdaptationMeasure {
  /// Defensive adaptation in Surprise (idx 5): primary withdrawal intent basis points.
  pub fn surprise_adaptation_bp(report: &EmpiricalDistributionEstimateReport) -> u16 {
    report.action_distributions()[5].primary_share_basis_points()
  }

  /// Defensive adaptation in ResponseToFailure (idx 6): primary yield intent basis points.
  pub fn failure_adaptation_bp(report: &EmpiricalDistributionEstimateReport) -> u16 {
    report.action_distributions()[6].primary_share_basis_points()
  }

  /// Composite adaptation score: mean defensive shift across adverse conditions.
  pub fn composite_adaptation_bp(report: &EmpiricalDistributionEstimateReport) -> u16 {
    let s = u32::from(Self::surprise_adaptation_bp(report));
    let f = u32::from(Self::failure_adaptation_bp(report));
    u16::try_from((s + f) / 2).expect("composite adaptation fits in u16")
  }
}

/// Unified behavioral measures summary report aggregating distance, entropy, sensitivity, consistency, and adaptation profiles.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BehavioralMeasuresReport {
  schema: &'static str,
  profile_id: &'static str,
  mean_action_entropy_bp: u16,
  mean_communication_entropy_bp: u16,
  mean_action_consistency_bp: u16,
  mean_communication_consistency_bp: u16,
  surprise_sensitivity_bp: u16,
  sacrifice_sensitivity_bp: u16,
  failure_sensitivity_bp: u16,
  composite_adaptation_bp: u16,
}

impl BehavioralMeasuresReport {
  /// Generate a unified behavioral measures report from an empirical distribution estimate report.
  pub fn from_report(report: &EmpiricalDistributionEstimateReport) -> Self {
    Self {
      schema: BEHAVIORAL_MEASURES_SCHEMA,
      profile_id: report.profile_id(),
      mean_action_entropy_bp: BehavioralEntropyMeasure::mean_action_entropy(report),
      mean_communication_entropy_bp: BehavioralEntropyMeasure::mean_communication_entropy(report),
      mean_action_consistency_bp: BehavioralConsistencyMeasure::mean_action_consistency(report),
      mean_communication_consistency_bp:
        BehavioralConsistencyMeasure::mean_communication_consistency(report),
      surprise_sensitivity_bp: BehavioralSensitivityMeasure::surprise_sensitivity(report),
      sacrifice_sensitivity_bp: BehavioralSensitivityMeasure::sacrifice_sensitivity(report),
      failure_sensitivity_bp: BehavioralSensitivityMeasure::failure_sensitivity(report),
      composite_adaptation_bp: BehavioralAdaptationMeasure::composite_adaptation_bp(report),
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn mean_action_entropy_bp(&self) -> u16 {
    self.mean_action_entropy_bp
  }

  pub const fn mean_communication_entropy_bp(&self) -> u16 {
    self.mean_communication_entropy_bp
  }

  pub const fn mean_action_consistency_bp(&self) -> u16 {
    self.mean_action_consistency_bp
  }

  pub const fn mean_communication_consistency_bp(&self) -> u16 {
    self.mean_communication_consistency_bp
  }

  pub const fn surprise_sensitivity_bp(&self) -> u16 {
    self.surprise_sensitivity_bp
  }

  pub const fn sacrifice_sensitivity_bp(&self) -> u16 {
    self.sacrifice_sensitivity_bp
  }

  pub const fn failure_sensitivity_bp(&self) -> u16 {
    self.failure_sensitivity_bp
  }

  pub const fn composite_adaptation_bp(&self) -> u16 {
    self.composite_adaptation_bp
  }

  /// Render the unified behavioral measures report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Behavioral Measures Report\n\n- schema: {}\n- profile_id: {}\n- mean_action_entropy_bp: {}\n- mean_communication_entropy_bp: {}\n- mean_action_consistency_bp: {}\n- mean_communication_consistency_bp: {}\n- surprise_sensitivity_bp: {}\n- sacrifice_sensitivity_bp: {}\n- failure_sensitivity_bp: {}\n- composite_adaptation_bp: {}\n",
      self.schema,
      self.profile_id,
      self.mean_action_entropy_bp,
      self.mean_communication_entropy_bp,
      self.mean_action_consistency_bp,
      self.mean_communication_consistency_bp,
      self.surprise_sensitivity_bp,
      self.sacrifice_sensitivity_bp,
      self.failure_sensitivity_bp,
      self.composite_adaptation_bp,
    )
  }
}

/// One actor-safe row in a scripted-agent comparison report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentComparisonEntry {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  selected_intent: LaneIntent,
  selected_score: i16,
  candidate_count: u8,
}

impl ScriptedAgentComparisonEntry {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }

  pub const fn selected_score(self) -> i16 {
    self.selected_score
  }

  pub const fn candidate_count(self) -> u8 {
    self.candidate_count
  }
}

/// Bounded actor-safe comparison report for the three catalog profiles.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentComparisonReport {
  schema: &'static str,
  observer: crate::kernel::ActorId,
  observation_id: crate::lane::ObservationId,
  entries: [ScriptedAgentComparisonEntry; 3],
}

impl ScriptedAgentComparisonReport {
  /// Build a report from one actor-visible observation.
  pub fn from_observation(observation: LanerObservation) -> Self {
    let decisions = [
      ScriptedAgent::cautious_v1().choose(observation),
      ScriptedAgent::risk_taking_v1().choose(observation),
      ScriptedAgent::yielding_v1().choose(observation),
    ];
    Self {
      schema: SCRIPTED_AGENT_METRICS_SCHEMA,
      observer: observation.observer(),
      observation_id: observation.observation_id(),
      entries: [
        Self::entry(&decisions[0]),
        Self::entry(&decisions[1]),
        Self::entry(&decisions[2]),
      ],
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> crate::kernel::ActorId {
    self.observer
  }

  pub const fn observation_id(&self) -> crate::lane::ObservationId {
    self.observation_id
  }

  pub const fn entries(&self) -> &[ScriptedAgentComparisonEntry; 3] {
    &self.entries
  }

  fn entry(decision: &ScriptedAgentDecision) -> ScriptedAgentComparisonEntry {
    let selected = decision
      .candidates()
      .iter()
      .find(|candidate| candidate.intent() == decision.selected_intent())
      .expect("selected intent must be in the candidate report");
    ScriptedAgentComparisonEntry {
      profile_id: decision.profile().profile_id(),
      evaluation_rule: decision.profile().evaluation_rule(),
      selected_intent: decision.selected_intent(),
      selected_score: selected.score(),
      candidate_count: u8::try_from(decision.candidates().len())
        .expect("bounded candidate count fits in u8"),
    }
  }
}

/// Bounded error returned when a tally mixes observers or repeats an
/// observation ID.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentActionTallyError {
  MismatchedObserver,
  DuplicateObservationId,
}

/// One actor-safe selected-action tally for a catalog profile.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentActionTally {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  observation_count: u8,
  stabilize_count: u8,
  contest_count: u8,
  yield_count: u8,
  recall_count: u8,
  withdraw_count: u8,
}

impl ScriptedAgentActionTally {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn stabilize_count(self) -> u8 {
    self.stabilize_count
  }

  pub const fn contest_count(self) -> u8 {
    self.contest_count
  }

  pub const fn yield_count(self) -> u8 {
    self.yield_count
  }

  pub const fn recall_count(self) -> u8 {
    self.recall_count
  }

  pub const fn withdraw_count(self) -> u8 {
    self.withdraw_count
  }
}

/// Bounded actor-safe selected-action counts over exactly two observations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentActionTallyReport {
  schema: &'static str,
  observer: crate::kernel::ActorId,
  observation_ids: [crate::lane::ObservationId; 2],
  entries: [ScriptedAgentActionTally; 3],
}

impl ScriptedAgentActionTallyReport {
  /// Build a tally from two observations belonging to the same actor with
  /// distinct observation IDs.
  pub fn from_observations(
    observations: [LanerObservation; 2],
  ) -> Result<Self, ScriptedAgentActionTallyError> {
    if observations[0].observer() != observations[1].observer() {
      return Err(ScriptedAgentActionTallyError::MismatchedObserver);
    }
    if observations[0].observation_id() == observations[1].observation_id() {
      return Err(ScriptedAgentActionTallyError::DuplicateObservationId);
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_ACTION_TALLY_SCHEMA,
      observer: observations[0].observer(),
      observation_ids: [
        observations[0].observation_id(),
        observations[1].observation_id(),
      ],
      entries: [
        Self::entry(ScriptedAgent::cautious_v1(), observations),
        Self::entry(ScriptedAgent::risk_taking_v1(), observations),
        Self::entry(ScriptedAgent::yielding_v1(), observations),
      ],
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> crate::kernel::ActorId {
    self.observer
  }

  pub const fn observation_ids(&self) -> &[crate::lane::ObservationId; 2] {
    &self.observation_ids
  }

  pub const fn entries(&self) -> &[ScriptedAgentActionTally; 3] {
    &self.entries
  }

  fn entry(agent: ScriptedAgent, observations: [LanerObservation; 2]) -> ScriptedAgentActionTally {
    let mut tally = ScriptedAgentActionTally {
      profile_id: agent.profile().profile_id(),
      evaluation_rule: agent.profile().evaluation_rule(),
      observation_count: 2,
      stabilize_count: 0,
      contest_count: 0,
      yield_count: 0,
      recall_count: 0,
      withdraw_count: 0,
    };
    for observation in observations {
      match agent.choose(observation).selected_intent() {
        LaneIntent::Stabilize => tally.stabilize_count += 1,
        LaneIntent::Contest => tally.contest_count += 1,
        LaneIntent::Yield => tally.yield_count += 1,
        LaneIntent::Recall => tally.recall_count += 1,
        LaneIntent::Withdraw => tally.withdraw_count += 1,
      }
    }
    tally
  }
}

/// Scripted-agent policy with no implicit random stream or hidden input.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ScriptedAgent {
  profile: ScriptedAgentProfile,
}

impl Default for ScriptedAgentProfile {
  fn default() -> Self {
    Self::cautious_v1()
  }
}

impl ScriptedAgent {
  /// Construct the first cautious baseline.
  pub const fn cautious_v1() -> Self {
    Self {
      profile: ScriptedAgentProfile::cautious_v1(),
    }
  }

  /// Construct the risk-taking matched-input comparison profile.
  pub const fn risk_taking_v1() -> Self {
    Self {
      profile: ScriptedAgentProfile::risk_taking_v1(),
    }
  }

  /// Construct the yielding matched-input comparison profile.
  pub const fn yielding_v1() -> Self {
    Self {
      profile: ScriptedAgentProfile::yielding_v1(),
    }
  }

  pub const fn profile(self) -> ScriptedAgentProfile {
    self.profile
  }

  /// Generate bounded candidates from the actor-visible legal intent set.
  pub fn generate_candidates(self, observation: LanerObservation) -> Vec<LaneIntent> {
    let mut candidates = Vec::with_capacity(5);
    for intent in observation.available_intents() {
      candidates.push(intent);
    }
    if let Some(threat_response) = observation.available_threat_response()
      && !candidates.contains(&threat_response)
    {
      candidates.push(threat_response);
    }
    candidates
  }

  /// Evaluate one generated candidate without reading anything beyond the observation.
  pub fn evaluate_candidate(
    self,
    observation: LanerObservation,
    intent: LaneIntent,
  ) -> Result<ScriptedAgentCandidate, ScriptedAgentEvaluationError> {
    if !self.candidate_is_advertised(observation, intent) {
      return Err(ScriptedAgentEvaluationError::UnavailableIntent);
    }
    Ok(self.score_candidate(observation, intent))
  }

  fn candidate_is_advertised(self, observation: LanerObservation, intent: LaneIntent) -> bool {
    observation.available_intents().contains(&intent)
      || observation.available_threat_response() == Some(intent)
  }

  fn score_candidate(
    self,
    observation: LanerObservation,
    intent: LaneIntent,
  ) -> ScriptedAgentCandidate {
    let threat_response = observation.available_threat_response() == Some(intent);
    let reason = if self.profile.evaluation == ScriptedAgentEvaluationRule::Contest
      && intent == LaneIntent::Contest
    {
      ScriptedAgentReason::RiskPreference
    } else if self.profile.evaluation == ScriptedAgentEvaluationRule::Yield
      && intent == LaneIntent::Yield
    {
      ScriptedAgentReason::YieldPreference
    } else if threat_response {
      ScriptedAgentReason::ThreatResponse
    } else if intent == LaneIntent::Stabilize {
      ScriptedAgentReason::StableDefault
    } else {
      ScriptedAgentReason::AvailableAlternative
    };
    let score = match (self.profile.evaluation, reason) {
      (_, ScriptedAgentReason::RiskPreference) => 100,
      (_, ScriptedAgentReason::YieldPreference) => 100,
      (ScriptedAgentEvaluationRule::Threat, ScriptedAgentReason::ThreatResponse) => 100,
      (ScriptedAgentEvaluationRule::Contest, ScriptedAgentReason::ThreatResponse) => 90,
      (ScriptedAgentEvaluationRule::Yield, ScriptedAgentReason::ThreatResponse) => 90,
      (_, ScriptedAgentReason::StableDefault) => {
        80 + if self.profile.evaluation == ScriptedAgentEvaluationRule::Threat {
          i16::from(observation.wave_pressure().value())
        } else {
          0
        }
      }
      (_, ScriptedAgentReason::AvailableAlternative) => match intent {
        LaneIntent::Contest => 60,
        LaneIntent::Yield => 40,
        LaneIntent::Recall => 20,
        LaneIntent::Withdraw => 10,
        LaneIntent::Stabilize => 80,
      },
    };
    ScriptedAgentCandidate {
      intent,
      score,
      reason,
    }
  }

  fn select_candidate(candidates: &[ScriptedAgentCandidate]) -> ScriptedAgentCandidate {
    candidates
      .iter()
      .copied()
      .reduce(|best, candidate| {
        if candidate.score > best.score {
          candidate
        } else {
          best
        }
      })
      .expect("actor observation must advertise an intent")
  }

  fn select_candidate_with_seed(
    candidates: &[ScriptedAgentCandidate],
    seed_bundle: ScriptedAgentSeedBundle,
  ) -> ScriptedAgentCandidate {
    let max_score = candidates
      .iter()
      .map(|candidate| candidate.score())
      .max()
      .expect("actor observation must advertise an intent");
    let tied_count = candidates
      .iter()
      .filter(|candidate| candidate.score() == max_score)
      .count();
    let selected_tie = seed_bundle.tie_index(tied_count);
    candidates
      .iter()
      .copied()
      .filter(|candidate| candidate.score() == max_score)
      .nth(selected_tie)
      .expect("seeded tie index must select a candidate")
  }

  fn decision(
    self,
    observation: LanerObservation,
    candidates: Vec<ScriptedAgentCandidate>,
    selected: ScriptedAgentCandidate,
    selection_rule: &'static str,
    seed_bundle: Option<ScriptedAgentSeedBundle>,
  ) -> ScriptedAgentDecision {
    let request = LaneIntentRequest::new(
      observation.observer(),
      observation.observation_id(),
      selected.intent,
    );
    ScriptedAgentDecision {
      profile: self.profile,
      observer: observation.observer(),
      observation_id: observation.observation_id(),
      candidates,
      selected_intent: selected.intent,
      request,
      selection_rule,
      seed_bundle,
    }
  }

  /// Generate, evaluate, and select one deterministic actor-visible request.
  pub fn choose(self, observation: LanerObservation) -> ScriptedAgentDecision {
    let candidates = self
      .generate_candidates(observation)
      .into_iter()
      .map(|intent| self.score_candidate(observation, intent))
      .collect::<Vec<_>>();
    let selected = Self::select_candidate(&candidates).intent;
    let selected = candidates
      .iter()
      .copied()
      .find(|candidate| candidate.intent() == selected)
      .expect("selected intent must be in candidates");
    self.decision(
      observation,
      candidates,
      selected,
      self.profile.selection_rule(),
      None,
    )
  }

  /// Generate and select one request using an explicit reproducible policy
  /// stream. The seed affects only ties among equal top-scoring candidates.
  pub fn choose_with_seed(
    self,
    observation: LanerObservation,
    seed_bundle: ScriptedAgentSeedBundle,
  ) -> ScriptedAgentDecision {
    let candidates = self
      .generate_candidates(observation)
      .into_iter()
      .map(|intent| self.score_candidate(observation, intent))
      .collect::<Vec<_>>();
    let selected = Self::select_candidate_with_seed(&candidates, seed_bundle);
    self.decision(
      observation,
      candidates,
      selected,
      SCRIPTED_AGENT_SEEDED_SELECTION_RULE,
      Some(seed_bundle),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::host::CliScenarioHost;
  use crate::kernel::{DrawId, StreamId};
  use crate::lane::{
    ALLIED_AUTONOMOUS_ACTOR, JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus,
    M2_LANE_RULESET, ObservationId, WavePressure, WaveState, observe_player, validate_lane_request,
  };
  use crate::protocol::{
    ActorActionDto, ActorMessageDto, ActorProtocolCodecError, ActorProtocolIntent,
    MAX_ACTOR_DRAFT_VALUE_BYTES,
  };

  #[test]
  fn cautious_agent_uses_initial_actor_visible_candidates_and_legal_request() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(9));
    let agent = ScriptedAgent::cautious_v1();
    let decision = agent.choose(receipt.observation());

    assert_eq!(SCRIPTED_AGENT_SCHEMA, "m4-scripted-agent-v1");
    assert_eq!(decision.profile().profile_id(), SCRIPTED_AGENT_PROFILE_ID);
    assert_eq!(decision.observer(), receipt.observation().observer());
    assert_eq!(
      decision.observation_id(),
      receipt.observation().observation_id()
    );
    assert_eq!(decision.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(decision.candidates().len(), 4);
    assert_eq!(decision.request().intent(), LaneIntent::Stabilize);
    assert!(decision.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Contest
        && candidate.reason() == ScriptedAgentReason::AvailableAlternative
    }));
    assert_eq!(
      agent
        .evaluate_candidate(receipt.observation(), LaneIntent::Contest)
        .expect("advertised intent evaluates"),
      ScriptedAgentCandidate {
        intent: LaneIntent::Contest,
        score: 60,
        reason: ScriptedAgentReason::AvailableAlternative,
      }
    );
    validate_lane_request(&state, &receipt, &decision.request()).expect("policy request is legal");
  }

  #[test]
  fn experiment_manifest_codec_binds_profiles_rules_and_seed() {
    let seed = ScriptedAgentSeedBundle::new(42, StreamId::new(7), DrawId::new(9));
    let profiles = [
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentProfile::risk_taking_v1(),
      ScriptedAgentProfile::yielding_v1(),
    ];
    for profile in profiles {
      let manifest = ScriptedAgentExperimentManifest::new(profile, seed);
      assert_eq!(manifest.schema(), "m6-experiment-manifest-v1");
      assert_eq!(manifest.scenario_id(), "m3-two-window-fixture-v1");
      assert_eq!(manifest.profile().profile_id(), profile.profile_id());
      assert_eq!(
        manifest.profile().evaluation_rule(),
        profile.evaluation_rule()
      );
      assert_eq!(manifest.selection_rule(), "max-score-seeded-tie-v1");
      assert_eq!(manifest.seed_bundle(), seed);
      assert_eq!(
        ScriptedAgentExperimentManifest::decode(&manifest.encode()),
        Ok(manifest)
      );
    }
    assert_eq!(
      ScriptedAgentExperimentManifest::new(profiles[0], seed).encode(),
      "schema=m6-experiment-manifest-v1\nscenario=m3-two-window-fixture-v1\nprofile=cautious-laner-v1\nevaluation_rule=threat-first-pressure-aware-fixed-score-v1\nselection_rule=max-score-seeded-tie-v1\nseed=42\npolicy_stream=7\npolicy_draw=9\n"
    );

    let valid = ScriptedAgentExperimentManifest::new(profiles[0], seed).encode();
    for malformed in [
      (
        valid.replacen("schema=m6-experiment-manifest-v1", "schema=other", 1),
        ScriptedAgentManifestError::UnsupportedSchema,
      ),
      (
        valid.replacen("profile=cautious-laner-v1", "profile=unknown", 1),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen(
          "evaluation_rule=threat-first-pressure-aware-fixed-score-v1",
          "evaluation_rule=wrong",
          1,
        ),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen(
          "selection_rule=max-score-seeded-tie-v1",
          "selection_rule=wrong",
          1,
        ),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen("policy_stream=7", "policy_stream=nope", 1),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen("policy_draw=9", "policy_draw=nope", 1),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen("seed=42", "seed=nope", 1),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen("scenario=m3-two-window-fixture-v1", "scenario=other", 1),
        ScriptedAgentManifestError::InvalidValue,
      ),
      (
        valid.replacen("profile=cautious-laner-v1", "unknown=profile", 1),
        ScriptedAgentManifestError::UnknownField,
      ),
      (
        valid.replacen("profile=cautious-laner-v1\n", "", 1),
        ScriptedAgentManifestError::MissingField,
      ),
      (
        valid.replacen(
          "profile=cautious-laner-v1",
          "schema=m6-experiment-manifest-v1",
          1,
        ),
        ScriptedAgentManifestError::DuplicateField,
      ),
      (
        format!("{valid}extra=value\n"),
        ScriptedAgentManifestError::UnexpectedLineCount {
          expected: 8,
          actual: 9,
        },
      ),
    ] {
      assert_eq!(
        ScriptedAgentExperimentManifest::decode(&malformed.0),
        Err(malformed.1)
      );
    }
    assert_eq!(
      ScriptedAgentExperimentManifest::decode(&"x".repeat(MAX_SCRIPTED_AGENT_MANIFEST_BYTES + 1)),
      Err(ScriptedAgentManifestError::Oversized)
    );
  }

  #[test]
  fn experiment_version_catalog_is_literal_and_deterministic() {
    let catalog = ScriptedAgentExperimentVersionCatalog::current();
    assert_eq!(catalog.schema(), "m6-experiment-version-catalog-v1");
    assert_eq!(catalog.ruleset_id(), "m2-lane-ruleset-v4");
    assert_eq!(M2_LANE_RULESET.value(), 4);
    assert_eq!(catalog.scenario_id(), "m3-two-window-fixture-v1");
    assert_eq!(catalog.policy_schema(), "m4-scripted-agent-v1");
    assert_eq!(
      catalog.profile_ids(),
      [
        "cautious-laner-v1",
        "risk-taking-laner-v1",
        "yielding-laner-v1",
      ]
    );
    assert_eq!(catalog.prompt_version(), "not-applicable");
    assert_eq!(catalog.tool_schema_version(), "not-applicable");
    assert_eq!(catalog.model_version(), "not-applicable");
    assert_eq!(catalog.extractor_version(), "not-applicable");
    assert_eq!(catalog, ScriptedAgentExperimentVersionCatalog::current());
  }

  #[test]
  fn bounded_batch_runner_preserves_order_and_reproducibility() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(44)).observation();
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(1, StreamId::new(2), DrawId::new(3)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(4, StreamId::new(5), DrawId::new(6)),
      ),
    ];
    let first = ScriptedAgentBatchRunner::run(observation, &manifests).expect("batch runs");
    let second = ScriptedAgentBatchRunner::run(observation, &manifests).expect("batch repeats");
    assert_eq!(first, second);
    assert_eq!(first.len(), 2);
    assert_eq!(first[0].profile(), manifests[0].profile());
    assert_eq!(first[1].profile(), manifests[1].profile());
    assert_eq!(first[0].seed_bundle(), Some(manifests[0].seed_bundle()));
    assert_eq!(first[1].seed_bundle(), Some(manifests[1].seed_bundle()));
    assert_eq!(
      ScriptedAgentBatchRunner::run(observation, &[]),
      Err(ScriptedAgentBatchError::EmptyBatch)
    );
    let at_capacity = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS];
    let capacity_decisions =
      ScriptedAgentBatchRunner::run(observation, &at_capacity).expect("inclusive cap runs");
    assert_eq!(capacity_decisions.len(), MAX_SCRIPTED_AGENT_BATCH_MANIFESTS);
    assert!(
      capacity_decisions
        .iter()
        .all(|decision| decision.seed_bundle() == Some(manifests[0].seed_bundle()))
    );
    let too_many = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1];
    assert_eq!(
      ScriptedAgentBatchRunner::run(observation, &too_many),
      Err(ScriptedAgentBatchError::BatchTooLarge {
        max: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS,
        actual: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1,
      })
    );
  }

  #[test]
  fn run_disposition_codec_preserves_all_closed_statuses_and_rejects_malformed_text() {
    let dispositions = [
      (ScriptedAgentRunDisposition::Completed, "completed"),
      (ScriptedAgentRunDisposition::Crashed, "crashed"),
      (ScriptedAgentRunDisposition::TimedOut, "timed_out"),
      (ScriptedAgentRunDisposition::MissingBranch, "missing_branch"),
      (ScriptedAgentRunDisposition::Inconclusive, "inconclusive"),
    ];
    for (disposition, expected_id) in dispositions {
      let record = ScriptedAgentRunDispositionRecord::new(disposition);
      assert_eq!(record.schema(), SCRIPTED_AGENT_RUN_DISPOSITION_SCHEMA);
      assert_eq!(record.disposition(), disposition);
      assert_eq!(disposition.id(), expected_id);
      assert_eq!(
        record.encode(),
        format!("schema=m6-scripted-agent-run-disposition-v1\ndisposition={expected_id}\n")
      );
      assert_eq!(
        ScriptedAgentRunDispositionRecord::decode(&record.encode()),
        Ok(record)
      );
    }
    let valid =
      ScriptedAgentRunDispositionRecord::new(ScriptedAgentRunDisposition::Completed).encode();
    assert_eq!(
      valid,
      "schema=m6-scripted-agent-run-disposition-v1\ndisposition=completed\n"
    );
    for (malformed, expected) in [
      (
        valid.replacen(
          "schema=m6-scripted-agent-run-disposition-v1",
          "schema=other",
          1,
        ),
        ScriptedAgentRunDispositionCodecError::UnsupportedSchema,
      ),
      (
        valid.replacen("disposition=completed", "unknown=completed", 1),
        ScriptedAgentRunDispositionCodecError::UnknownField,
      ),
      (
        valid.replacen("schema=", "disposition=", 1),
        ScriptedAgentRunDispositionCodecError::DuplicateField,
      ),
      (
        valid.replacen("disposition=completed\n", "", 1),
        ScriptedAgentRunDispositionCodecError::MissingField,
      ),
      (
        valid.replacen("disposition=completed", "disposition=unknown", 1),
        ScriptedAgentRunDispositionCodecError::InvalidValue,
      ),
      (
        format!("{valid}extra=value\n"),
        ScriptedAgentRunDispositionCodecError::UnexpectedLineCount {
          expected: 2,
          actual: 3,
        },
      ),
    ] {
      assert_eq!(
        ScriptedAgentRunDispositionRecord::decode(&malformed),
        Err(expected)
      );
    }
    assert_eq!(MAX_SCRIPTED_AGENT_RUN_DISPOSITION_BYTES, 4096);
    assert_eq!(
      ScriptedAgentRunDispositionRecord::decode(&"x".repeat(4096)),
      Err(ScriptedAgentRunDispositionCodecError::InvalidValue)
    );
    assert_eq!(
      ScriptedAgentRunDispositionRecord::decode(&"x".repeat(4097)),
      Err(ScriptedAgentRunDispositionCodecError::Oversized)
    );
  }

  #[test]
  fn operational_log_preserves_closed_ordered_events_without_history_payloads() {
    let events = [
      (ScriptedAgentOperationalEvent::BatchStarted, "batch_started"),
      (
        ScriptedAgentOperationalEvent::ChunkCompleted,
        "chunk_completed",
      ),
      (
        ScriptedAgentOperationalEvent::CheckpointSaved,
        "checkpoint_saved",
      ),
      (ScriptedAgentOperationalEvent::BatchResumed, "batch_resumed"),
      (
        ScriptedAgentOperationalEvent::BatchFinished,
        "batch_finished",
      ),
    ];
    let mut log = ScriptedAgentOperationalLog::new();
    assert_eq!(log.schema(), "m6-scripted-agent-operational-event-v1");
    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
    for (event, expected_id) in events {
      assert_eq!(event.id(), expected_id);
      log.append(event).expect("event fits in operational log");
    }
    assert_eq!(log.len(), events.len());
    assert!(!log.is_empty());
    assert_eq!(log.entries()[0].schema(), log.schema());
    assert_eq!(
      log.entries()[0].event(),
      ScriptedAgentOperationalEvent::BatchStarted
    );
    assert_eq!(
      log.entries()[1].event(),
      ScriptedAgentOperationalEvent::ChunkCompleted
    );
    assert_eq!(
      log.entries()[2].event(),
      ScriptedAgentOperationalEvent::CheckpointSaved
    );
    assert_eq!(
      log.entries()[3].event(),
      ScriptedAgentOperationalEvent::BatchResumed
    );
    assert_eq!(
      log.entries()[4].event(),
      ScriptedAgentOperationalEvent::BatchFinished
    );
    assert_eq!(
      log.encode(),
      "schema=m6-scripted-agent-operational-log-v1\nentries=5\nevent=batch_started\nevent=chunk_completed\nevent=checkpoint_saved\nevent=batch_resumed\nevent=batch_finished\n"
    );
    assert_eq!(
      ScriptedAgentOperationalLog::decode(&log.encode()),
      Ok(log.clone())
    );
    let encoded = log.encode();
    for (malformed, expected) in [
      (
        encoded.replacen(
          "schema=m6-scripted-agent-operational-log-v1",
          "schema=other",
          1,
        ),
        ScriptedAgentOperationalLogCodecError::UnsupportedSchema,
      ),
      (
        encoded.replacen("entries=5", "unknown=5", 1),
        ScriptedAgentOperationalLogCodecError::UnknownField,
      ),
      (
        encoded.replacen(
          "entries=5\n",
          "schema=m6-scripted-agent-operational-log-v1\nentries=5\n",
          1,
        ),
        ScriptedAgentOperationalLogCodecError::DuplicateField,
      ),
      (
        encoded.replacen("entries=5\n", "", 1),
        ScriptedAgentOperationalLogCodecError::MissingField,
      ),
      (
        encoded.replacen("event=batch_finished", "event=unknown", 1),
        ScriptedAgentOperationalLogCodecError::InvalidValue,
      ),
      (
        "not-a-field".to_owned(),
        ScriptedAgentOperationalLogCodecError::InvalidValue,
      ),
      (
        "schema=\nentries=0\n".to_owned(),
        ScriptedAgentOperationalLogCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=5", "entries=not-a-number", 1),
        ScriptedAgentOperationalLogCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=5", "entries=17", 1),
        ScriptedAgentOperationalLogCodecError::InvalidValue,
      ),
      (
        format!("{encoded}event=batch_started\n"),
        ScriptedAgentOperationalLogCodecError::UnexpectedLineCount {
          expected: 7,
          actual: 8,
        },
      ),
      (
        format!(
          "schema=m6-scripted-agent-operational-log-v1\nentries=16\n{}",
          "event=batch_started\n".repeat(17)
        ),
        ScriptedAgentOperationalLogCodecError::UnexpectedLineCount {
          expected: 18,
          actual: 19,
        },
      ),
    ] {
      assert_eq!(
        ScriptedAgentOperationalLog::decode(&malformed),
        Err(expected)
      );
    }
    let swapped_headers = "entries=5\nschema=m6-scripted-agent-operational-log-v1\nevent=batch_started\nevent=chunk_completed\nevent=checkpoint_saved\nevent=batch_resumed\nevent=batch_finished\n";
    assert_eq!(
      ScriptedAgentOperationalLog::decode(swapped_headers),
      Err(ScriptedAgentOperationalLogCodecError::InvalidValue)
    );
    let event_before_headers = "event=batch_started\nentries=5\nschema=m6-scripted-agent-operational-log-v1\nevent=chunk_completed\nevent=checkpoint_saved\nevent=batch_resumed\nevent=batch_finished\n";
    assert_eq!(
      ScriptedAgentOperationalLog::decode(event_before_headers),
      Err(ScriptedAgentOperationalLogCodecError::InvalidValue)
    );
    assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES, 4096);
    assert_eq!(
      SCRIPTED_AGENT_OPERATIONAL_LOG_SCHEMA,
      "m6-scripted-agent-operational-log-v1"
    );
    let inclusive_size_input = format!("{}\n", "x".repeat(4095));
    assert_eq!(
      inclusive_size_input.len(),
      MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES
    );
    assert_eq!(
      ScriptedAgentOperationalLog::decode(&inclusive_size_input),
      Err(ScriptedAgentOperationalLogCodecError::InvalidValue)
    );
    assert_eq!(
      ScriptedAgentOperationalLog::decode(&"x".repeat(4097)),
      Err(ScriptedAgentOperationalLogCodecError::Oversized)
    );
    assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, 16);
    for _ in events.len()..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
      log
        .append(ScriptedAgentOperationalEvent::BatchStarted)
        .expect("event fits at inclusive cap");
    }
    assert_eq!(log.len(), MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS);
    let entries_before_overflow = log.entries().to_vec();
    assert_eq!(
      log.append(ScriptedAgentOperationalEvent::BatchFinished),
      Err(ScriptedAgentOperationalLogError::CapacityExceeded {
        max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
      })
    );
    assert_eq!(log.len(), MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS);
    assert_eq!(log.entries(), entries_before_overflow.as_slice());
    assert_eq!(
      ScriptedAgentOperationalLog::decode(&log.encode()),
      Ok(log.clone())
    );
  }

  #[test]
  fn operational_log_sequence_status_is_closed_ordered_and_read_only() {
    let build_log = |events: &[ScriptedAgentOperationalEvent]| {
      let mut log = ScriptedAgentOperationalLog::new();
      for event in events {
        log.append(*event).expect("sequence fixture fits");
      }
      log
    };
    let complete = build_log(&[
      ScriptedAgentOperationalEvent::BatchStarted,
      ScriptedAgentOperationalEvent::ChunkCompleted,
      ScriptedAgentOperationalEvent::BatchFinished,
    ]);
    let before = complete.clone();
    let report = ScriptedAgentOperationalLogSequenceReport::from_log(&complete);
    assert_eq!(
      report.schema(),
      "m6-scripted-agent-operational-log-sequence-v1"
    );
    assert_eq!(report.rule(), "m6-operational-start-chunk-finish-v1");
    assert_eq!(
      report.status(),
      ScriptedAgentOperationalLogSequenceStatus::Complete
    );
    assert_eq!(report.status().id(), "complete");
    assert_eq!(
      ScriptedAgentOperationalLogSequenceReport::from_log(&complete),
      report,
      "repeated sequence classification is deterministic"
    );
    assert_eq!(
      complete, before,
      "status inspection does not mutate the log"
    );

    let optional = build_log(&[
      ScriptedAgentOperationalEvent::BatchStarted,
      ScriptedAgentOperationalEvent::ChunkCompleted,
      ScriptedAgentOperationalEvent::CheckpointSaved,
      ScriptedAgentOperationalEvent::BatchResumed,
      ScriptedAgentOperationalEvent::BatchFinished,
    ]);
    assert_eq!(
      ScriptedAgentOperationalLogSequenceReport::from_log(&optional).status(),
      ScriptedAgentOperationalLogSequenceStatus::Complete
    );

    for (events, expected) in [
      (
        &[][..],
        ScriptedAgentOperationalLogSequenceStatus::MissingStart,
      ),
      (
        &[ScriptedAgentOperationalEvent::BatchStarted][..],
        ScriptedAgentOperationalLogSequenceStatus::MissingChunk,
      ),
      (
        &[
          ScriptedAgentOperationalEvent::BatchStarted,
          ScriptedAgentOperationalEvent::ChunkCompleted,
        ][..],
        ScriptedAgentOperationalLogSequenceStatus::MissingFinish,
      ),
      (
        &[
          ScriptedAgentOperationalEvent::ChunkCompleted,
          ScriptedAgentOperationalEvent::BatchFinished,
        ][..],
        ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
      ),
      (
        &[
          ScriptedAgentOperationalEvent::BatchStarted,
          ScriptedAgentOperationalEvent::CheckpointSaved,
          ScriptedAgentOperationalEvent::ChunkCompleted,
          ScriptedAgentOperationalEvent::BatchFinished,
        ][..],
        ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
      ),
      (
        &[
          ScriptedAgentOperationalEvent::BatchStarted,
          ScriptedAgentOperationalEvent::ChunkCompleted,
          ScriptedAgentOperationalEvent::BatchFinished,
          ScriptedAgentOperationalEvent::BatchStarted,
        ][..],
        ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
      ),
    ] {
      assert_eq!(
        ScriptedAgentOperationalLogSequenceReport::from_log(&build_log(events)).status(),
        expected
      );
    }
    assert_eq!(
      [
        ScriptedAgentOperationalLogSequenceStatus::Complete,
        ScriptedAgentOperationalLogSequenceStatus::MissingStart,
        ScriptedAgentOperationalLogSequenceStatus::MissingChunk,
        ScriptedAgentOperationalLogSequenceStatus::MissingFinish,
        ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
      ]
      .into_iter()
      .map(ScriptedAgentOperationalLogSequenceStatus::id)
      .collect::<Vec<_>>(),
      vec![
        "complete",
        "missing_start",
        "missing_chunk",
        "missing_finish",
        "invalid_order"
      ]
    );
  }

  #[test]
  fn batch_runner_operational_log_producer_is_ordered_and_preflights_capacity() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(47)).observation();
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(1, StreamId::new(2), DrawId::new(3)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(4, StreamId::new(5), DrawId::new(6)),
      ),
    ];
    let expected = ScriptedAgentBatchRunner::run(observation, &manifests)
      .expect("the direct batch remains the parity reference");
    let mut log = ScriptedAgentOperationalLog::new();
    let produced =
      ScriptedAgentBatchRunner::run_with_operational_log(observation, &manifests, &mut log)
        .expect("the complete batch fits in the operational log");
    assert_eq!(produced, expected);
    assert_eq!(
      log
        .entries()
        .iter()
        .map(|entry| entry.event().id())
        .collect::<Vec<_>>(),
      ["batch_started", "chunk_completed", "batch_finished"]
    );

    let mut invalid_log = ScriptedAgentOperationalLog::new();
    invalid_log
      .append(ScriptedAgentOperationalEvent::CheckpointSaved)
      .expect("one event fits");
    let invalid_before = invalid_log.entries().to_vec();
    assert_eq!(
      ScriptedAgentBatchRunner::run_with_operational_log(observation, &[], &mut invalid_log),
      Err(ScriptedAgentOperationalBatchRunError::Batch(
        ScriptedAgentBatchError::EmptyBatch,
      ))
    );
    assert_eq!(invalid_log.entries(), invalid_before.as_slice());

    assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, 16);
    let mut at_capacity_log = ScriptedAgentOperationalLog::new();
    for _ in 0..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS - 3 {
      at_capacity_log
        .append(ScriptedAgentOperationalEvent::CheckpointSaved)
        .expect("inclusive-capacity fixture fits");
    }
    let at_capacity_decisions = ScriptedAgentBatchRunner::run_with_operational_log(
      observation,
      &manifests,
      &mut at_capacity_log,
    )
    .expect("exactly three lifecycle events fit at the inclusive cap");
    assert_eq!(at_capacity_decisions, expected);
    assert_eq!(at_capacity_log.len(), MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS);
    assert_eq!(
      at_capacity_log
        .entries()
        .iter()
        .rev()
        .take(3)
        .map(|entry| entry.event().id())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>(),
      ["batch_started", "chunk_completed", "batch_finished"]
    );

    let mut full_log = ScriptedAgentOperationalLog::new();
    for _ in 0..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS - 2 {
      full_log
        .append(ScriptedAgentOperationalEvent::BatchStarted)
        .expect("preflight fixture fits");
    }
    let full_before = full_log.entries().to_vec();
    assert_eq!(
      ScriptedAgentBatchRunner::run_with_operational_log(observation, &manifests, &mut full_log),
      Err(ScriptedAgentOperationalBatchRunError::LogCapacityExceeded { max: 16 })
    );
    assert_eq!(full_log.entries(), full_before.as_slice());
  }

  #[test]
  fn batch_checkpoint_codec_and_store_resume_one_chunk() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(45)).observation();
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(7, StreamId::new(8), DrawId::new(9)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(10, StreamId::new(11), DrawId::new(12)),
      ),
    ];
    let checkpoint =
      ScriptedAgentBatchCheckpoint::new(observation, &manifests).expect("checkpoint starts");
    assert_eq!(checkpoint.schema(), SCRIPTED_AGENT_BATCH_RUN_SCHEMA);
    assert_eq!(
      SCRIPTED_AGENT_BATCH_RUN_SCHEMA,
      "m6-scripted-agent-batch-run-v1"
    );
    let encoded = checkpoint.encode();
    assert_eq!(
      encoded,
      format!(
        "schema=m6-scripted-agent-batch-run-v1\nobserver={}\nobservation_id=45\nmanifest_count=2\ncompleted_count=0\ninput_fingerprint={}\n",
        observation.observer().value(),
        12216804097755993549u64,
      )
    );
    assert_eq!(
      ScriptedAgentBatchCheckpoint::decode(&encoded),
      Ok(checkpoint)
    );
    let valid = encoded;
    for (malformed, expected) in [
      (
        valid.replacen("schema=m6-scripted-agent-batch-run-v1", "schema=other", 1),
        ScriptedAgentBatchCheckpointError::UnsupportedSchema,
      ),
      (
        valid.replacen("observer=", "unknown=", 1),
        ScriptedAgentBatchCheckpointError::UnknownField,
      ),
      (
        valid.replacen("observer=", "schema=", 1),
        ScriptedAgentBatchCheckpointError::DuplicateField,
      ),
      (
        valid.replacen("completed_count=0\n", "", 1),
        ScriptedAgentBatchCheckpointError::MissingField,
      ),
      (
        format!("{valid}extra=value\n"),
        ScriptedAgentBatchCheckpointError::UnexpectedLineCount {
          expected: 6,
          actual: 7,
        },
      ),
      (
        valid.replacen("completed_count=0", "completed_count=3", 1),
        ScriptedAgentBatchCheckpointError::InvalidValue,
      ),
    ] {
      assert_eq!(
        ScriptedAgentBatchCheckpoint::decode(&malformed),
        Err(expected)
      );
    }
    assert_eq!(
      ScriptedAgentBatchCheckpoint::decode(&"x".repeat(MAX_SCRIPTED_AGENT_BATCH_RUN_BYTES + 1)),
      Err(ScriptedAgentBatchCheckpointError::Oversized)
    );

    let root =
      std::env::temp_dir().join(format!("fog-of-intent-agent-batch-{}", std::process::id()));
    let store = crate::agent_batch_store::ScriptedAgentBatchRunStore::new(&root);
    let host_store = crate::run_store::CliRunStore::new(&root);
    let host_artifact = "artifact schema=m3-cli-host-artifact-v1 replay_id=m2-two-window-scenario-v3 run_id=resume records=0";
    host_store
      .save("resume", host_artifact)
      .expect("host artifact saves");
    let mut operational_log = ScriptedAgentOperationalLog::new();
    store
      .save_with_operational_log("resume", checkpoint, &mut operational_log)
      .expect("checkpoint saves with an event");
    assert_eq!(
      operational_log.entries()[0].event(),
      ScriptedAgentOperationalEvent::CheckpointSaved
    );
    assert_eq!(
      host_store.load("resume").expect("host artifact loads"),
      host_artifact
    );
    let loaded = store
      .load_with_operational_log("resume", &mut operational_log)
      .expect("checkpoint loads with an event");
    assert_eq!(
      operational_log.entries()[1].event(),
      ScriptedAgentOperationalEvent::BatchResumed
    );
    let operational_store =
      crate::agent_operational_store::ScriptedAgentOperationalLogStore::new(&root);
    operational_store
      .save("resume", &operational_log)
      .expect("operational log saves beside checkpoint");
    assert_eq!(
      host_store
        .load("resume")
        .expect("host artifact survives log save"),
      host_artifact
    );
    assert_eq!(
      store.load("resume").expect("checkpoint survives log save"),
      checkpoint
    );
    assert!(root.join("resume.foi-operational-log").is_file());
    assert_eq!(
      operational_store
        .load("resume")
        .expect("operational log loads"),
      operational_log
    );
    assert_eq!(
      crate::agent_operational_store::MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS,
      4
    );
    let mut first_segment = ScriptedAgentOperationalLog::new();
    first_segment
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("first segment fits");
    let mut second_segment = ScriptedAgentOperationalLog::new();
    second_segment
      .append(ScriptedAgentOperationalEvent::BatchFinished)
      .expect("second segment fits");
    operational_store
      .save_segment("resume", 0, &first_segment)
      .expect("first segment saves");
    operational_store
      .save_segment("resume", 1, &second_segment)
      .expect("second segment saves");
    operational_store
      .save_segment("resume", 3, &second_segment)
      .expect("highest segment saves");
    std::fs::write(root.join("resume.foi-operational-log.segment-01"), "bad")
      .expect("leading-zero fixture writes");
    std::fs::write(root.join("resume.foi-operational-log.segment-4"), "bad")
      .expect("out-of-range fixture writes");
    std::fs::write(root.join("resume.foi-operational-log.segment-.tmp0"), "bad")
      .expect("temporary-name fixture writes");
    std::fs::create_dir(root.join("resume.foi-operational-log.segment-2"))
      .expect("non-file fixture creates");
    assert_eq!(
      operational_store
        .load_segment("resume", 0)
        .expect("first segment loads"),
      first_segment
    );
    assert_eq!(
      operational_store
        .load_segment("resume", 1)
        .expect("second segment loads"),
      second_segment
    );
    assert_eq!(
      operational_store
        .load_segment("resume", 3)
        .expect("highest segment loads"),
      second_segment
    );
    assert!(root.join("resume.foi-operational-log.segment-0").is_file());
    assert!(root.join("resume.foi-operational-log.segment-1").is_file());
    assert!(root.join("resume.foi-operational-log.segment-3").is_file());
    assert_eq!(
      operational_store
        .list_segments("resume")
        .expect("segments list"),
      vec![0, 1, 3]
    );
    assert_eq!(
      operational_store
        .load("resume")
        .expect("base log survives segments"),
      operational_log
    );
    let invalid_segment_root = root.join("invalid-segment");
    let invalid_segment_store =
      crate::agent_operational_store::ScriptedAgentOperationalLogStore::new(&invalid_segment_root);
    assert_eq!(
      invalid_segment_store.save_segment("resume", 4, &first_segment),
      Err(
        crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::InvalidSegment {
          max: 4,
        }
      )
    );
    assert!(!invalid_segment_root.exists());
    assert_eq!(
      invalid_segment_store.list_segments("resume"),
      Err(
        crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::StorageUnavailable
      )
    );
    assert_eq!(
      operational_store.list_segments("bad/id"),
      Err(
        crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::StorageUnavailable
      )
    );
    assert_eq!(
      invalid_segment_store.load_segment("resume", 4),
      Err(
        crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::InvalidSegment {
          max: 4,
        }
      )
    );
    assert!(!invalid_segment_root.exists());
    assert_eq!(
      host_store
        .load("resume")
        .expect("host artifact survives segments"),
      host_artifact
    );
    assert_eq!(
      store.load("resume").expect("checkpoint survives segments"),
      checkpoint
    );
    std::fs::write(root.join("broken.foi-batch-run"), "bad")
      .expect("malformed checkpoint fixture writes");
    let log_before_decode_error = operational_log.entries().to_vec();
    assert_eq!(
      store.load_with_operational_log("broken", &mut operational_log),
      Err(
        crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::Store(
          crate::agent_batch_store::ScriptedAgentBatchStoreError::InvalidCheckpoint {
            error: ScriptedAgentBatchCheckpointError::InvalidValue,
          },
        )
      )
    );
    assert_eq!(
      operational_log.entries(),
      log_before_decode_error.as_slice()
    );
    let invalid_root = root.join("not-a-directory");
    std::fs::write(&invalid_root, "file").expect("invalid storage root fixture writes");
    let invalid_store = crate::agent_batch_store::ScriptedAgentBatchRunStore::new(&invalid_root);
    let mut storage_error_log = ScriptedAgentOperationalLog::new();
    storage_error_log
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("one event fits");
    let storage_error_before = storage_error_log.entries().to_vec();
    assert_eq!(
      invalid_store.save_with_operational_log("resume", checkpoint, &mut storage_error_log),
      Err(
        crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::Store(
          crate::agent_batch_store::ScriptedAgentBatchStoreError::StorageUnavailable,
        )
      )
    );
    assert_eq!(storage_error_log.entries(), storage_error_before.as_slice());
    assert_eq!(
      invalid_store.load_with_operational_log("resume", &mut storage_error_log),
      Err(
        crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::Store(
          crate::agent_batch_store::ScriptedAgentBatchStoreError::StorageUnavailable,
        )
      )
    );
    assert_eq!(storage_error_log.entries(), storage_error_before.as_slice());
    std::fs::write(root.join("broken.foi-operational-log"), "bad")
      .expect("malformed operational log fixture writes");
    assert_eq!(
      operational_store.load("broken"),
      Err(
        crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::InvalidLog {
          error: ScriptedAgentOperationalLogCodecError::InvalidValue,
        }
      )
    );
    for _ in operational_log.len()..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
      operational_log
        .append(ScriptedAgentOperationalEvent::BatchStarted)
        .expect("event log reaches its cap");
    }
    let log_before_capacity_error = operational_log.entries().to_vec();
    assert_eq!(
      store.load_with_operational_log("resume", &mut operational_log),
      Err(
        crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
          max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
        }
      )
    );
    assert_eq!(
      operational_log.entries(),
      log_before_capacity_error.as_slice()
    );
    let mut save_capacity_log = ScriptedAgentOperationalLog::new();
    for _ in 0..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
      save_capacity_log
        .append(ScriptedAgentOperationalEvent::BatchStarted)
        .expect("save capacity fixture reaches its cap");
    }
    let save_capacity_before = save_capacity_log.entries().to_vec();
    assert_eq!(
      store.save_with_operational_log(
        "resume",
        checkpoint.with_completed_count(1),
        &mut save_capacity_log,
      ),
      Err(
        crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
          max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
        }
      )
    );
    assert_eq!(save_capacity_log.entries(), save_capacity_before.as_slice());
    assert_eq!(
      store.load("resume").expect("prior checkpoint remains"),
      checkpoint
    );
    let (first, advanced) = ScriptedAgentBatchRunner::run_next(observation, &manifests, loaded, 1)
      .expect("first chunk runs");
    assert_eq!(first.len(), 1);
    assert_eq!(advanced.completed_count(), 1);
    store
      .save("resume", advanced)
      .expect("advanced checkpoint saves");
    let (remaining, complete) = ScriptedAgentBatchRunner::run_next(
      observation,
      &manifests,
      store.load("resume").expect("advanced checkpoint loads"),
      16,
    )
    .expect("remaining chunk runs");
    let full = ScriptedAgentBatchRunner::run(observation, &manifests).expect("full batch runs");
    assert_eq!(remaining, full[1..]);
    assert!(complete.is_complete());
    assert_eq!(complete.completed_count(), 2);
    assert_eq!(
      ScriptedAgentBatchRunner::run_next(observation, &manifests, complete, 1,)
        .expect("completed run is idempotent")
        .0,
      Vec::<ScriptedAgentDecision>::new()
    );
    let mismatched_observation = observe_player(&state, ObservationId::new(46)).observation();
    assert_eq!(
      ScriptedAgentBatchRunner::run_next(mismatched_observation, &manifests, complete, 1),
      Err(ScriptedAgentBatchRunError::InputMismatch)
    );
    let reordered = [manifests[1], manifests[0]];
    assert_eq!(
      ScriptedAgentBatchRunner::run_next(observation, &reordered, complete, 1),
      Err(ScriptedAgentBatchRunError::InputMismatch)
    );
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn matched_observation_sample_is_stable_and_bounded() {
    let initial = LaneSnapshot::initial();
    let threat = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let observations = [
      observe_player(&initial, ObservationId::new(60)).observation(),
      observe_player(&threat, ObservationId::new(61)).observation(),
    ];
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(13, StreamId::new(14), DrawId::new(15)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(16, StreamId::new(17), DrawId::new(18)),
      ),
    ];
    let sample = ScriptedAgentMatchedSample::from_observations(observations, &manifests)
      .expect("matched sample builds");
    assert_eq!(
      SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA,
      "m6-scripted-agent-matched-sample-v1"
    );
    assert_eq!(sample.schema(), SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA);
    assert_eq!(sample.observer(), observations[0].observer());
    assert_eq!(
      sample.observation_ids(),
      &[ObservationId::new(60), ObservationId::new(61)]
    );
    assert_eq!(sample.entries().len(), 2);
    assert_eq!(sample.entries()[0].profile_id(), SCRIPTED_AGENT_PROFILE_ID);
    assert_eq!(
      sample.entries()[0].evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(
      sample.entries()[0].seed_bundle(),
      manifests[0].seed_bundle()
    );
    assert_eq!(
      sample.entries()[0].selected_intents(),
      [LaneIntent::Stabilize, LaneIntent::Withdraw]
    );
    assert_eq!(
      sample.entries()[1].profile_id(),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      sample.entries()[1].evaluation_rule(),
      "yield-first-fixed-score-v1"
    );
    assert_eq!(
      sample.entries()[1].seed_bundle(),
      manifests[1].seed_bundle()
    );
    assert_eq!(
      sample.entries()[1].selected_intents(),
      [LaneIntent::Yield, LaneIntent::Yield]
    );
    assert_eq!(
      sample,
      ScriptedAgentMatchedSample::from_observations(observations, &manifests)
        .expect("matched sample repeats")
    );

    let mut mixed_observation = observations[1];
    mixed_observation.observer = ALLIED_AUTONOMOUS_ACTOR;
    let mixed_actor = [observations[0], mixed_observation];
    assert_eq!(
      ScriptedAgentMatchedSample::from_observations(mixed_actor, &manifests),
      Err(ScriptedAgentMatchedSampleError::MismatchedObserver)
    );
    let duplicate_id = [
      observations[0],
      observe_player(&threat, ObservationId::new(60)).observation(),
    ];
    assert_eq!(
      ScriptedAgentMatchedSample::from_observations(duplicate_id, &manifests),
      Err(ScriptedAgentMatchedSampleError::DuplicateObservationId)
    );
    assert_eq!(
      ScriptedAgentMatchedSample::from_observations(observations, &[]),
      Err(ScriptedAgentMatchedSampleError::Batch(
        ScriptedAgentBatchError::EmptyBatch
      ))
    );
    let too_many = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1];
    assert_eq!(
      ScriptedAgentMatchedSample::from_observations(observations, &too_many),
      Err(ScriptedAgentMatchedSampleError::Batch(
        ScriptedAgentBatchError::BatchTooLarge {
          max: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS,
          actual: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1,
        }
      ))
    );
  }

  #[test]
  fn matched_scenario_sample_set_preserves_order_and_bounds() {
    let initial = LaneSnapshot::initial();
    let threat = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(19, StreamId::new(20), DrawId::new(21)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(22, StreamId::new(23), DrawId::new(24)),
      ),
    ];
    let pairs = [
      [
        observe_player(&initial, ObservationId::new(70)).observation(),
        observe_player(&threat, ObservationId::new(71)).observation(),
      ],
      [
        observe_player(&initial, ObservationId::new(72)).observation(),
        observe_player(&threat, ObservationId::new(73)).observation(),
      ],
    ];
    let sample = ScriptedAgentMatchedScenarioSample::from_observations(&pairs, &manifests)
      .expect("matched scenario sample builds");
    assert_eq!(sample.schema(), "m6-scripted-agent-matched-scenarios-v1");
    assert_eq!(sample.observer(), pairs[0][0].observer());
    assert_eq!(sample.samples().len(), 2);
    assert_eq!(sample.samples()[0].entries().len(), 2);
    assert_eq!(
      sample.samples()[0].entries()[0].profile_id(),
      SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      sample.samples()[0].entries()[1].profile_id(),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      sample.samples()[0].entries()[0].seed_bundle(),
      manifests[0].seed_bundle()
    );
    assert_eq!(
      sample.samples()[0].entries()[1].seed_bundle(),
      manifests[1].seed_bundle()
    );
    assert_eq!(
      sample.samples()[0].observation_ids(),
      &[ObservationId::new(70), ObservationId::new(71)]
    );
    assert_eq!(
      sample.samples()[1].observation_ids(),
      &[ObservationId::new(72), ObservationId::new(73)]
    );
    assert_eq!(
      sample,
      ScriptedAgentMatchedScenarioSample::from_observations(&pairs, &manifests)
        .expect("matched scenario sample repeats")
    );
    let tally = ScriptedAgentMatchedScenarioTallyReport::from_sample(&sample);
    assert_eq!(
      tally.schema(),
      "m6-scripted-agent-matched-scenario-tally-v1"
    );
    assert_eq!(tally.observer(), sample.observer());
    assert_eq!(tally.pair_count(), 2);
    assert_eq!(tally.observation_count(), 4);
    assert_eq!(tally.entries().len(), 2);
    assert_eq!(tally.entries()[0].profile_id(), SCRIPTED_AGENT_PROFILE_ID);
    assert_eq!(
      tally.entries()[0].evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(tally.entries()[0].stabilize_count(), 2);
    assert_eq!(tally.entries()[0].contest_count(), 0);
    assert_eq!(tally.entries()[0].withdraw_count(), 2);
    assert_eq!(
      tally.entries()[0].stabilize_count()
        + tally.entries()[0].contest_count()
        + tally.entries()[0].yield_count()
        + tally.entries()[0].recall_count()
        + tally.entries()[0].withdraw_count(),
      tally.entries()[0].observation_count()
    );
    assert_eq!(
      tally.entries()[1].profile_id(),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      tally.entries()[1].evaluation_rule(),
      "yield-first-fixed-score-v1"
    );
    assert_eq!(tally.entries()[1].yield_count(), 4);
    assert_eq!(
      tally.entries()[1].stabilize_count()
        + tally.entries()[1].contest_count()
        + tally.entries()[1].yield_count()
        + tally.entries()[1].recall_count()
        + tally.entries()[1].withdraw_count(),
      tally.entries()[1].observation_count()
    );
    assert_eq!(
      tally,
      ScriptedAgentMatchedScenarioTallyReport::from_sample(&sample)
    );
    let encoded = tally.encode();
    assert_eq!(
      encoded,
      "schema=m6-scripted-agent-matched-scenario-tally-v1\nobserver=1\npair_count=2\nobservation_count=4\nentries=2\nrow=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2\nrow=yielding-laner-v1|yield-first-fixed-score-v1|0|0|4|0|0\n"
    );
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(&encoded, &tally),
      Ok(tally.clone())
    );
    for malformed in [
      (
        encoded.replacen(
          "schema=m6-scripted-agent-matched-scenario-tally-v1",
          "schema=other",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::UnsupportedSchema,
      ),
      (
        encoded.replacen("entries=2", "unknown=2", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::UnknownField,
      ),
      (
        encoded.replacen("cautious-laner-v1", "unknown-profile", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen(
          "entries=2",
          "schema=m6-scripted-agent-matched-scenario-tally-v1",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField,
      ),
      (
        encoded.replacen("entries=2\n", "", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::MissingField,
      ),
      (
        format!(
          "{encoded}row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2\n"
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::UnexpectedLineCount {
          expected: 7,
          actual: 8,
        },
      ),
      (
        encoded.replacen(
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2",
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|oops|0|0|0|2",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen(
          "yielding-laner-v1|yield-first-fixed-score-v1",
          "yielding-laner-v1|contest-first-fixed-score-v1",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen(
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2",
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|1|0|0|0|2",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("pair_count=2", "pair_count=0", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("pair_count=2", "pair_count=5", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("observation_count=4", "observation_count=3", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=2", "entries=0", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=2", "entries=17", 1),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
      (
        encoded.replacen(
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2",
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
      ),
    ] {
      assert_eq!(
        ScriptedAgentMatchedScenarioTallyReport::decode(&malformed.0, &tally),
        Err(malformed.1)
      );
    }
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(
        &"x".repeat(MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_BYTES + 1),
        &tally,
      ),
      Err(ScriptedAgentMatchedScenarioTallyCodecError::Oversized)
    );
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(
        &encoded.replacen("observer=1", "observer=255", 1),
        &tally,
      ),
      Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch)
    );
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(
        &encoded.replacen(
          "row=yielding-laner-v1|yield-first-fixed-score-v1|0|0|4|0|0",
          "row=yielding-laner-v1|yield-first-fixed-score-v1|0|0|2|2|0",
          1,
        ),
        &tally,
      ),
      Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch)
    );

    let at_capacity = [
      [
        observe_player(&initial, ObservationId::new(80)).observation(),
        observe_player(&threat, ObservationId::new(81)).observation(),
      ],
      [
        observe_player(&initial, ObservationId::new(82)).observation(),
        observe_player(&threat, ObservationId::new(83)).observation(),
      ],
      [
        observe_player(&initial, ObservationId::new(84)).observation(),
        observe_player(&threat, ObservationId::new(85)).observation(),
      ],
      [
        observe_player(&initial, ObservationId::new(86)).observation(),
        observe_player(&threat, ObservationId::new(87)).observation(),
      ],
    ];
    let capacity_sample =
      ScriptedAgentMatchedScenarioSample::from_observations(&at_capacity, &manifests)
        .expect("inclusive sample cap runs");
    assert_eq!(
      capacity_sample.samples().len(),
      MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES
    );
    assert_eq!(
      capacity_sample.samples()[3].observation_ids(),
      &[ObservationId::new(86), ObservationId::new(87)]
    );
    let capacity_tally = ScriptedAgentMatchedScenarioTallyReport::from_sample(&capacity_sample);
    assert_eq!(capacity_tally.pair_count(), 4);
    assert_eq!(capacity_tally.observation_count(), 8);
    assert_eq!(capacity_tally.entries().len(), 2);
    assert_eq!(capacity_tally.entries()[0].stabilize_count(), 4);
    assert_eq!(capacity_tally.entries()[0].withdraw_count(), 4);
    assert_eq!(capacity_tally.entries()[1].yield_count(), 8);
    for entry in capacity_tally.entries() {
      assert_eq!(
        entry.stabilize_count()
          + entry.contest_count()
          + entry.yield_count()
          + entry.recall_count()
          + entry.withdraw_count(),
        8
      );
    }
    let capacity_encoded = capacity_tally.encode();
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(&capacity_encoded, &capacity_tally),
      Ok(capacity_tally.clone())
    );

    let max_manifest_batch = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS];
    let max_entry_sample =
      ScriptedAgentMatchedScenarioSample::from_observations(&[pairs[0]], &max_manifest_batch)
        .expect("inclusive entry cap runs");
    let max_entry_tally = ScriptedAgentMatchedScenarioTallyReport::from_sample(&max_entry_sample);
    assert_eq!(max_entry_tally.pair_count(), 1);
    assert_eq!(max_entry_tally.observation_count(), 2);
    assert_eq!(
      max_entry_tally.entries().len(),
      MAX_SCRIPTED_AGENT_BATCH_MANIFESTS
    );
    let max_entry_encoded = max_entry_tally.encode();
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(&max_entry_encoded, &max_entry_tally),
      Ok(max_entry_tally)
    );

    assert_eq!(
      ScriptedAgentMatchedScenarioSample::from_observations(&[], &manifests),
      Err(ScriptedAgentMatchedScenarioSampleError::EmptySample)
    );
    let too_many = [pairs[0]; MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES + 1];
    assert_eq!(
      ScriptedAgentMatchedScenarioSample::from_observations(&too_many, &manifests),
      Err(ScriptedAgentMatchedScenarioSampleError::SampleTooLarge {
        max: MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES,
        actual: MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES + 1,
      })
    );
    let mut mixed = pairs;
    mixed[1][1].observer = ALLIED_AUTONOMOUS_ACTOR;
    assert_eq!(
      ScriptedAgentMatchedScenarioSample::from_observations(&mixed, &manifests),
      Err(ScriptedAgentMatchedScenarioSampleError::MismatchedObserver)
    );
    let duplicate = [
      pairs[0],
      [
        pairs[1][0],
        observe_player(&threat, ObservationId::new(70)).observation(),
      ],
    ];
    assert_eq!(
      ScriptedAgentMatchedScenarioSample::from_observations(&duplicate, &manifests),
      Err(ScriptedAgentMatchedScenarioSampleError::DuplicateObservationId)
    );
  }

  #[test]
  fn fixture_scenario_selection_is_closed_ordered_and_bounded() {
    let scenario_ids = [
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ];
    let observation_ids = [
      [ObservationId::new(100), ObservationId::new(101)],
      [ObservationId::new(102), ObservationId::new(103)],
      [ObservationId::new(104), ObservationId::new(105)],
      [ObservationId::new(106), ObservationId::new(107)],
    ];
    let selection =
      ScriptedAgentFixtureScenarioSelection::from_ids(&scenario_ids, &observation_ids)
        .expect("closed fixture selection builds");
    assert_eq!(
      [
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      ["safe-fixture-v1", "river-side-threat-v1"]
    );
    assert_eq!(
      SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA,
      "m6-scripted-agent-fixture-scenarios-v1"
    );
    assert_eq!(
      selection.schema(),
      SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA
    );
    assert_eq!(
      selection
        .scenarios()
        .iter()
        .map(|scenario| scenario.id())
        .collect::<Vec<_>>(),
      scenario_ids
    );
    assert_eq!(selection.observation_ids(), &observation_ids);
    assert_eq!(selection.observations(), selection.observations());
    let observations = selection.observations();
    assert_eq!(observations.len(), 4);
    assert_eq!(observations[0][0].observation_id(), ObservationId::new(100));
    assert_eq!(observations[1][1].observation_id(), ObservationId::new(103));
    assert_eq!(observations[0][1].available_threat_response(), None);
    assert_eq!(
      observations[1][1].available_threat_response(),
      Some(LaneIntent::Withdraw)
    );

    let manifests = [ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(31, StreamId::new(32), DrawId::new(33)),
    )];
    let sample = selection
      .matched_sample(&manifests)
      .expect("selected fixture samples compose");
    assert_eq!(sample.samples().len(), 4);
    assert_eq!(
      sample,
      ScriptedAgentFixtureScenarioSelection::from_ids(&scenario_ids, &observation_ids)
        .expect("selection repeats")
        .matched_sample(&manifests)
        .expect("repeated samples compose")
    );

    assert_eq!(MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS, 4);
    let population = ScriptedAgentFixtureScenarioPopulation::generate(4, 200)
      .expect("maximum fixed-fixture population builds");
    assert_eq!(
      SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA,
      "m6-scripted-agent-fixture-population-v1"
    );
    assert_eq!(
      population.schema(),
      SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA
    );
    assert_eq!(population.scenarios(), selection.scenarios());
    assert_eq!(
      population.observation_ids(),
      &[
        [ObservationId::new(200), ObservationId::new(201)],
        [ObservationId::new(202), ObservationId::new(203)],
        [ObservationId::new(204), ObservationId::new(205)],
        [ObservationId::new(206), ObservationId::new(207)],
      ]
    );
    assert_eq!(
      population,
      ScriptedAgentFixtureScenarioPopulation::generate(4, 200).expect("repeated population builds")
    );
    assert_eq!(
      population.matched_sample(&manifests),
      population.selection().matched_sample(&manifests)
    );
    let boundary_population = ScriptedAgentFixtureScenarioPopulation::generate(4, u64::MAX - 7)
      .expect("maximum observation IDs fit the population");
    assert_eq!(
      boundary_population.observation_ids().last(),
      Some(&[
        ObservationId::new(u64::MAX - 1),
        ObservationId::new(u64::MAX),
      ])
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate(0, 200),
      Err(ScriptedAgentFixturePopulationError::EmptyPopulation)
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate(
        MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
        200,
      ),
      Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
      })
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate(1, u64::MAX),
      Err(ScriptedAgentFixturePopulationError::ObservationIdOverflow)
    );

    assert_eq!(
      ScriptedAgentFixtureScenarioSelection::from_ids(&[], &[]),
      Err(ScriptedAgentFixtureScenarioSelectionError::EmptySelection)
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioSelection::from_ids(
        &["unknown-fixture-v1"],
        &[[ObservationId::new(108), ObservationId::new(109)]],
      ),
      Err(ScriptedAgentFixtureScenarioSelectionError::UnknownScenario)
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioSelection::from_ids(
        &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
        &[],
      ),
      Err(
        ScriptedAgentFixtureScenarioSelectionError::MismatchedObservationPairCount {
          expected: 1,
          actual: 0,
        }
      )
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioSelection::from_ids(
        &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
        &[[ObservationId::new(110), ObservationId::new(110)]],
      ),
      Err(ScriptedAgentFixtureScenarioSelectionError::DuplicateObservationId)
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioSelection::from_ids(
        &[
          SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
          SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        ],
        &[
          [ObservationId::new(112), ObservationId::new(113)],
          [ObservationId::new(114), ObservationId::new(112)],
        ],
      ),
      Err(ScriptedAgentFixtureScenarioSelectionError::DuplicateObservationId)
    );
    let too_many_scenarios =
      [SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID; MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1];
    let too_many_ids = (0..=MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS)
      .map(|index| {
        let offset = u64::try_from(index).expect("fixture index fits in u64") * 2;
        [
          ObservationId::new(120 + offset),
          ObservationId::new(121 + offset),
        ]
      })
      .collect::<Vec<_>>();
    assert_eq!(
      ScriptedAgentFixtureScenarioSelection::from_ids(&too_many_scenarios, &too_many_ids,),
      Err(
        ScriptedAgentFixtureScenarioSelectionError::SelectionTooLarge {
          max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
          actual: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
        }
      )
    );
  }

  #[test]
  fn caller_declared_population_composition_preserves_order_and_frequency() {
    let population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      220,
    )
    .expect("caller-declared composition builds");
    assert_eq!(
      population.scenarios(),
      &[
        ScriptedAgentFixtureScenario::Safe,
        ScriptedAgentFixtureScenario::Safe,
        ScriptedAgentFixtureScenario::Safe,
        ScriptedAgentFixtureScenario::RiverSideThreat,
      ]
    );
    assert_eq!(
      population.observation_ids(),
      &[
        [ObservationId::new(220), ObservationId::new(221)],
        [ObservationId::new(222), ObservationId::new(223)],
        [ObservationId::new(224), ObservationId::new(225)],
        [ObservationId::new(226), ObservationId::new(227)],
      ]
    );
    let frequency =
      ScriptedAgentFixtureScenarioFrequencyReport::from_selection(population.selection());
    assert_eq!(frequency.entries()[0].count(), 3);
    assert_eq!(frequency.entries()[1].count(), 1);
    let manifests = [ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(41, StreamId::new(42), DrawId::new(43)),
    )];
    assert_eq!(
      population.matched_sample(&manifests),
      population.selection().matched_sample(&manifests)
    );
    let tally = population
      .matched_tally(&manifests)
      .expect("caller-declared population tallies");
    assert_eq!(tally.pair_count(), 4);
    assert_eq!(tally.observation_count(), 8);
    assert_eq!(tally.entries().len(), 1);
    assert_eq!(tally.entries()[0].stabilize_count(), 7);
    assert_eq!(tally.entries()[0].withdraw_count(), 1);
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(&[], u64::MAX),
      Err(ScriptedAgentFixturePopulationError::EmptyPopulation)
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
        &["unknown-fixture-v1"; MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1],
        u64::MAX,
      ),
      Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
      })
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
        &["unknown-fixture-v1"],
        u64::MAX,
      ),
      Err(ScriptedAgentFixturePopulationError::InvalidSelection(
        ScriptedAgentFixtureScenarioSelectionError::UnknownScenario,
      ))
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
        &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
        u64::MAX,
      ),
      Err(ScriptedAgentFixturePopulationError::ObservationIdOverflow)
    );
  }

  #[test]
  fn profile_aware_population_tally_preserves_rows_and_counts() {
    let population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      230,
    )
    .expect("profile-aware population builds");
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(51, StreamId::new(52), DrawId::new(53)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::risk_taking_v1(),
        ScriptedAgentSeedBundle::new(54, StreamId::new(55), DrawId::new(56)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(57, StreamId::new(58), DrawId::new(59)),
      ),
    ];
    let tally = population
      .matched_tally(&manifests)
      .expect("profile-aware population tallies");
    assert_eq!(tally.pair_count(), 4);
    assert_eq!(tally.observation_count(), 8);
    assert_eq!(
      tally
        .entries()
        .iter()
        .map(|entry| entry.profile_id())
        .collect::<Vec<_>>(),
      vec![
        SCRIPTED_AGENT_PROFILE_ID,
        RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
        YIELDING_SCRIPTED_AGENT_PROFILE_ID,
      ]
    );
    assert_eq!(tally.entries()[0].stabilize_count(), 7);
    assert_eq!(tally.entries()[0].withdraw_count(), 1);
    assert_eq!(tally.entries()[1].contest_count(), 8);
    assert_eq!(tally.entries()[2].yield_count(), 8);
    assert_eq!(
      tally.entries()[0].intent_distribution_basis_points(),
      [8_750, 0, 0, 0, 1_250]
    );
    assert_eq!(
      tally.entries()[1].intent_distribution_basis_points(),
      [0, 10_000, 0, 0, 0]
    );
    assert_eq!(
      tally.entries()[2].intent_distribution_basis_points(),
      [0, 0, 10_000, 0, 0]
    );
    assert_eq!(
      tally
        .entries()
        .iter()
        .map(|entry| entry.intent_distribution_basis_points().iter().sum::<u16>())
        .collect::<Vec<_>>(),
      vec![10_000, 10_000, 10_000]
    );
    assert_eq!(
      tally.to_intent_distribution_markdown(),
      "# Profile Intent Distribution\n\n- schema: m6-scripted-agent-matched-scenario-tally-v1\n- observer: 1\n\n| profile_id | evaluation_rule | observation_count | stabilize_bp | contest_bp | yield_bp | recall_bp | withdraw_bp |\n| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n| cautious-laner-v1 | threat-first-pressure-aware-fixed-score-v1 | 8 | 8750 | 0 | 0 | 0 | 1250 |\n| risk-taking-laner-v1 | contest-first-fixed-score-v1 | 8 | 0 | 10000 | 0 | 0 | 0 |\n| yielding-laner-v1 | yield-first-fixed-score-v1 | 8 | 0 | 0 | 10000 | 0 | 0 |\n"
    );
    let remainder_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      238,
    )
    .expect("remainder population builds");
    let remainder_manifest = [ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(60, StreamId::new(61), DrawId::new(62)),
    )];
    let remainder_tally = remainder_population
      .matched_tally(&remainder_manifest)
      .expect("remainder tally builds");
    assert_eq!(remainder_tally.observation_count(), 6);
    assert_eq!(
      remainder_tally.entries()[0].intent_distribution_basis_points(),
      [8_333, 0, 0, 0, 1_667]
    );
    assert_eq!(
      remainder_tally.entries()[0]
        .intent_distribution_basis_points()
        .iter()
        .sum::<u16>(),
      SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE
    );
    assert_eq!(
      tally
        .entries()
        .iter()
        .map(|entry| {
          u16::from(entry.stabilize_count())
            + u16::from(entry.contest_count())
            + u16::from(entry.yield_count())
            + u16::from(entry.recall_count())
            + u16::from(entry.withdraw_count())
        })
        .collect::<Vec<_>>(),
      vec![8, 8, 8]
    );
  }

  #[test]
  fn scripted_agent_stress_population_catalog_is_closed_and_reproducible() {
    let state = LaneSnapshot::initial();
    let first_receipt = observe_player(&state, ObservationId::new(410));
    let first_observation = first_receipt.observation();
    let host = CliScenarioHost::fixture();
    let host_observation = host.observation();
    let illegal_error = host
      .validate_actor_action(ActorActionDto::new(
        host_observation.observer().value(),
        host_observation.observation_id().value(),
        ActorProtocolIntent::Withdraw,
      ))
      .expect_err("illegal actor command is rejected by host validation");
    assert_eq!(illegal_error.code().id(), "host_validation_rejected");
    let stale_error = host
      .validate_actor_action(ActorActionDto::new(
        host_observation.observer().value(),
        host_observation.observation_id().value() + 1,
        ActorProtocolIntent::Stabilize,
      ))
      .expect_err("stale actor command is rejected by host freshness");
    assert_eq!(stale_error.code().id(), "stale_observation");

    assert_eq!(
      ActorMessageDto::new(
        first_observation.observer().value(),
        ALLIED_AUTONOMOUS_ACTOR.value(),
        first_observation.observation_id().value(),
        &"x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES + 1),
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );

    let second_receipt = observe_player(&state, ObservationId::new(411));
    let choices = [
      ScriptedAgent::cautious_v1().choose(first_observation),
      ScriptedAgent::cautious_v1().choose(second_receipt.observation()),
    ];
    let degenerate_stabilize_count = u8::try_from(
      choices
        .iter()
        .filter(|choice| choice.selected_intent() == LaneIntent::Stabilize)
        .count(),
    )
    .expect("bounded degenerate count fits in u8");
    assert_eq!(degenerate_stabilize_count, 2);

    let results = [
      ScriptedAgentStressResult::HostValidationRejected,
      ScriptedAgentStressResult::StaleObservation,
      ScriptedAgentStressResult::MessageInvalidValue,
      ScriptedAgentStressResult::RepeatedStabilize,
    ];
    let report =
      ScriptedAgentStressPopulationReport::from_results(results, degenerate_stabilize_count)
        .expect("stress report binds expected results");
    assert_eq!(
      SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA,
      "m6-scripted-agent-stress-population-v1"
    );
    assert_eq!(report.schema(), SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA);
    assert_eq!(report.degenerate_stabilize_count(), 2);
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| (entry.case().id(), entry.result().id()))
        .collect::<Vec<_>>(),
      vec![
        ("illegal-command-v1", "host_validation_rejected"),
        ("exploit-seeking-v1", "stale_observation"),
        ("communication-abuse-v1", "message_invalid_value"),
        ("degenerate-policy-v1", "repeated_stabilize"),
      ]
    );
    assert_eq!(
      report.to_markdown(),
      "# Scripted Agent Stress Population\n\n- schema: m6-scripted-agent-stress-population-v1\n- degenerate_stabilize_count: 2\n\n| case_id | result_id |\n| --- | --- |\n| illegal-command-v1 | host_validation_rejected |\n| exploit-seeking-v1 | stale_observation |\n| communication-abuse-v1 | message_invalid_value |\n| degenerate-policy-v1 | repeated_stabilize |\n"
    );
    assert_eq!(
      ScriptedAgentStressPopulationReport::from_results(results, 2),
      Ok(report.clone())
    );
    assert_eq!(
      ScriptedAgentStressPopulationReport::from_results(
        [
          ScriptedAgentStressResult::RepeatedStabilize,
          ScriptedAgentStressResult::StaleObservation,
          ScriptedAgentStressResult::MessageInvalidValue,
          ScriptedAgentStressResult::RepeatedStabilize,
        ],
        2,
      ),
      Err(ScriptedAgentStressPopulationError::UnexpectedResult)
    );
    assert_eq!(
      ScriptedAgentStressPopulationReport::from_results(results, 0),
      Err(ScriptedAgentStressPopulationError::InvalidDegenerateCount)
    );
    assert!(ScriptedAgentStressPopulationReport::from_results(results, 4).is_ok());
    assert_eq!(
      ScriptedAgentStressPopulationReport::from_results(results, 5),
      Err(ScriptedAgentStressPopulationError::InvalidDegenerateCount)
    );
  }

  #[test]
  fn degenerate_policy_population_is_bounded_and_actor_visible() {
    let state = LaneSnapshot::initial();
    let observations = (0..MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION)
      .map(|offset| {
        observe_player(
          &state,
          ObservationId::new(700 + u64::try_from(offset).expect("offset fits")),
        )
        .observation()
      })
      .collect::<Vec<_>>();
    let report = ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&observations)
      .expect("fixed cautious observations repeat Stabilize");
    assert_eq!(
      SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA,
      "m6-scripted-agent-degenerate-policy-population-v1"
    );
    assert_eq!(
      report.schema(),
      SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA
    );
    assert_eq!(report.profile_id(), "cautious-laner-v1");
    assert_eq!(
      report.evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(report.observer(), observations[0].observer());
    assert_eq!(report.observation_count(), 4);
    assert_eq!(report.selected_intent(), LaneIntent::Stabilize);
    let singleton =
      ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&observations[..1])
        .expect("inclusive one-member population fits");
    assert_eq!(singleton.observation_count(), 1);
    assert_eq!(singleton.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(
      ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&observations),
      Ok(report)
    );
    assert_eq!(
      ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&[]),
      Err(ScriptedAgentDegeneratePolicyPopulationError::EmptyPopulation)
    );
    assert_eq!(
      ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&[
        observations[0],
        observations[0],
      ]),
      Err(ScriptedAgentDegeneratePolicyPopulationError::DuplicateObservationId)
    );
    let river_observation = ScriptedAgentFixtureScenario::RiverSideThreat
      .observations([ObservationId::new(900), ObservationId::new(901)])[1];
    assert_eq!(
      ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&[river_observation]),
      Err(ScriptedAgentDegeneratePolicyPopulationError::UnexpectedIntent)
    );
    let too_many = (0..=MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION)
      .map(|offset| {
        observe_player(
          &state,
          ObservationId::new(800 + u64::try_from(offset).expect("offset fits")),
        )
        .observation()
      })
      .collect::<Vec<_>>();
    assert_eq!(
      ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&too_many),
      Err(
        ScriptedAgentDegeneratePolicyPopulationError::PopulationTooLarge {
          max: MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION,
          actual: MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION + 1,
        }
      )
    );
  }

  #[test]
  fn exploit_seeking_population_is_bounded_and_fixed_fixture_only() {
    let state = LaneSnapshot::initial();
    let observations = (0..MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION)
      .map(|offset| {
        observe_player(
          &state,
          ObservationId::new(1_000 + u64::try_from(offset).expect("offset fits")),
        )
        .observation()
      })
      .collect::<Vec<_>>();
    let report = ScriptedAgentExploitSeekingPopulationReport::from_observations(&observations)
      .expect("risk-taking policy selects Contest in the safe fixture");
    assert_eq!(
      SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA,
      "m6-scripted-agent-exploit-seeking-population-v1"
    );
    assert_eq!(
      report.schema(),
      SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA
    );
    assert_eq!(report.profile_id(), "risk-taking-laner-v1");
    assert_eq!(report.evaluation_rule(), "contest-first-fixed-score-v1");
    assert_eq!(report.observer(), observations[0].observer());
    assert_eq!(report.observation_count(), 4);
    assert_eq!(report.selected_intent(), LaneIntent::Contest);
    let singleton =
      ScriptedAgentExploitSeekingPopulationReport::from_observations(&observations[..1])
        .expect("inclusive one-member population fits");
    assert_eq!(singleton.observation_count(), 1);
    assert_eq!(singleton.selected_intent(), LaneIntent::Contest);
    assert_eq!(
      ScriptedAgentExploitSeekingPopulationReport::from_observations(&observations),
      Ok(report)
    );
    assert_eq!(
      ScriptedAgentExploitSeekingPopulationReport::from_observations(&[]),
      Err(ScriptedAgentExploitSeekingPopulationError::EmptyPopulation)
    );
    assert_eq!(
      ScriptedAgentExploitSeekingPopulationReport::from_observations(&[
        observations[0],
        observations[0],
      ]),
      Err(ScriptedAgentExploitSeekingPopulationError::DuplicateObservationId)
    );
    let allied_observation = LanerObservation {
      observer: ALLIED_AUTONOMOUS_ACTOR,
      ..observations[0]
    };
    assert_eq!(
      ScriptedAgentExploitSeekingPopulationReport::from_observations(&[
        observations[0],
        allied_observation,
      ]),
      Err(ScriptedAgentExploitSeekingPopulationError::MismatchedObserver)
    );
    let too_many = (0..=MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION)
      .map(|offset| {
        observe_player(
          &state,
          ObservationId::new(1_100 + u64::try_from(offset).expect("offset fits")),
        )
        .observation()
      })
      .collect::<Vec<_>>();
    assert_eq!(
      ScriptedAgentExploitSeekingPopulationReport::from_observations(&too_many),
      Err(
        ScriptedAgentExploitSeekingPopulationError::PopulationTooLarge {
          max: MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION,
          actual: MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION + 1,
        }
      )
    );
  }

  #[test]
  fn profile_aware_population_tally_codec_round_trips_verified_rows() {
    let population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      240,
    )
    .expect("codec population builds");
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(61, StreamId::new(62), DrawId::new(63)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::risk_taking_v1(),
        ScriptedAgentSeedBundle::new(64, StreamId::new(65), DrawId::new(66)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(67, StreamId::new(68), DrawId::new(69)),
      ),
    ];
    let tally = population
      .matched_tally(&manifests)
      .expect("codec tally builds");
    let encoded = tally.encode();
    assert!(encoded.starts_with("schema=m6-scripted-agent-matched-scenario-tally-v1\n"));
    assert!(encoded.contains("entries=3\n"));
    assert!(
      encoded
        .contains("row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1\n")
    );
    assert!(encoded.contains("row=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0\n"));
    assert!(encoded.contains("row=yielding-laner-v1|yield-first-fixed-score-v1|0|0|8|0|0\n"));
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(&encoded, &tally),
      Ok(tally.clone())
    );
    let tampered = encoded.replace("|7|0|0|0|1\n", "|6|0|0|0|2\n");
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(&tampered, &tally),
      Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch)
    );
  }

  #[test]
  fn profile_aware_tally_comparison_preserves_rows_and_signed_deltas() {
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(71, StreamId::new(72), DrawId::new(73)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::risk_taking_v1(),
        ScriptedAgentSeedBundle::new(74, StreamId::new(75), DrawId::new(76)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::yielding_v1(),
        ScriptedAgentSeedBundle::new(77, StreamId::new(78), DrawId::new(79)),
      ),
    ];
    let baseline_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      280,
    )
    .expect("baseline population builds");
    let candidate_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      288,
    )
    .expect("candidate population builds");
    let baseline = baseline_population
      .matched_tally(&manifests)
      .expect("baseline tally builds");
    let candidate = candidate_population
      .matched_tally(&manifests)
      .expect("candidate tally builds");
    let comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
        .expect("matching verified tallies compare");
    assert_eq!(
      SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA,
      "m6-scripted-agent-matched-scenario-tally-compare-v1"
    );
    assert_eq!(
      comparison.schema(),
      "m6-scripted-agent-matched-scenario-tally-compare-v1"
    );
    let encoded = comparison.encode();
    assert_eq!(
      encoded,
      "schema=m6-scripted-agent-matched-scenario-tally-compare-v1\nobserver=1\nbaseline_pair_count=4\nbaseline_observation_count=8\ncandidate_pair_count=4\ncandidate_observation_count=8\nentries=3\nrow=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1|5|0|0|0|3\nrow=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0|0|8|0|0|0\nrow=yielding-laner-v1|yield-first-fixed-score-v1|0|0|8|0|0|0|0|8|0|0\n"
    );
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyComparisonReport::decode(&encoded, &comparison),
      Ok(comparison.clone())
    );
    for (malformed, error) in [
      (
        encoded.replacen(
          "schema=m6-scripted-agent-matched-scenario-tally-compare-v1",
          "schema=wrong-v1",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnsupportedSchema,
      ),
      (
        encoded.replacen("entries=3", "unknown=3", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnknownField,
      ),
      (
        encoded.replacen("observer=1", "schema=wrong-v1", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::DuplicateField,
      ),
      (
        encoded.replacen(
          "observer=1\nbaseline_pair_count=4",
          "baseline_pair_count=4\nobserver=1",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::DuplicateField,
      ),
      (
        encoded.replacen("cautious-laner-v1", "unknown-profile-v1", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        encoded.replacen(
          "threat-first-pressure-aware-fixed-score-v1",
          "contest-first-fixed-score-v1",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        encoded.replacen("baseline_pair_count=4", "baseline_pair_count=x", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        encoded.replacen("|7|0|0|0|1|5|0|0|0|3\n", "|x|0|0|0|1|5|0|0|0|3\n", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=3", "entries=0", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=3", "entries=17", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        encoded.replacen(
          "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1|5|0|0|0|3\nrow=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0|0|8|0|0|0\n",
          "row=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0|0|8|0|0|0\nrow=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1|5|0|0|0|3\n",
          1,
        ),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InputMismatch,
      ),
      (
        encoded.lines().take(6).collect::<Vec<_>>().join("\n"),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::MissingField,
      ),
      (
        encoded.replacen("|7|0|0|0|1|5|0|0|0|3\n", "|6|0|0|0|1|5|0|0|0|3\n", 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
      ),
      (
        format!("{encoded}extra=x\n"),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnexpectedLineCount {
          expected: 10,
          actual: 11,
        },
      ),
      (
        "x".repeat(MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_BYTES + 1),
        ScriptedAgentMatchedScenarioTallyComparisonCodecError::Oversized,
      ),
    ] {
      assert_eq!(
        ScriptedAgentMatchedScenarioTallyComparisonReport::decode(&malformed, &comparison),
        Err(error)
      );
    }
    let tampered = encoded.replacen("|7|0|0|0|1|5|0|0|0|3\n", "|6|0|0|0|2|5|0|0|0|3\n", 1);
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyComparisonReport::decode(&tampered, &comparison),
      Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InputMismatch)
    );
    assert_eq!(comparison.observer(), baseline.observer());
    assert_eq!(comparison.baseline_pair_count(), 4);
    assert_eq!(comparison.baseline_observation_count(), 8);
    assert_eq!(comparison.candidate_pair_count(), 4);
    assert_eq!(comparison.candidate_observation_count(), 8);
    assert_eq!(comparison.entries().len(), 3);
    assert_eq!(comparison.entries()[0].profile_id(), "cautious-laner-v1");
    assert_eq!(
      comparison.entries()[0].evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(comparison.entries()[0].baseline_counts(), [7, 0, 0, 0, 1]);
    assert_eq!(comparison.entries()[0].candidate_counts(), [5, 0, 0, 0, 3]);
    assert_eq!(comparison.entries()[0].deltas(), [-2, 0, 0, 0, 2]);
    assert_eq!(comparison.entries()[1].profile_id(), "risk-taking-laner-v1");
    assert_eq!(
      comparison.entries()[1].evaluation_rule(),
      "contest-first-fixed-score-v1"
    );
    assert_eq!(comparison.entries()[1].baseline_counts(), [0, 8, 0, 0, 0]);
    assert_eq!(comparison.entries()[1].candidate_counts(), [0, 8, 0, 0, 0]);
    assert_eq!(comparison.entries()[1].deltas(), [0, 0, 0, 0, 0]);
    assert_eq!(comparison.entries()[2].profile_id(), "yielding-laner-v1");
    assert_eq!(
      comparison.entries()[2].evaluation_rule(),
      "yield-first-fixed-score-v1"
    );
    assert_eq!(comparison.entries()[2].baseline_counts(), [0, 0, 8, 0, 0]);
    assert_eq!(comparison.entries()[2].candidate_counts(), [0, 0, 8, 0, 0]);
    assert_eq!(comparison.entries()[2].deltas(), [0, 0, 0, 0, 0]);
    assert_eq!(
      comparison.regression_rule(),
      "m6-fixed-profile-tally-no-change-v1"
    );
    assert!(!comparison.passes_no_change_gate());
    let unchanged =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
        .expect("unchanged verified tallies compare");
    assert!(unchanged.passes_no_change_gate());
    assert_eq!(
      comparison,
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
        .expect("repeated comparison is stable")
    );
    let reversed =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&candidate, &baseline)
        .expect("reversed verified tallies compare");
    assert_eq!(reversed.entries()[0].deltas(), [2, 0, 0, 0, -2]);

    let smaller_candidate_population =
      ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
        &[
          SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
          SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
          SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        ],
        296,
      )
      .expect("smaller candidate population builds");
    let smaller_candidate = smaller_candidate_population
      .matched_tally(&manifests)
      .expect("smaller candidate tally builds");
    let changed_total = ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(
      &baseline,
      &smaller_candidate,
    )
    .expect("changed-total verified tallies compare");
    assert_eq!(changed_total.baseline_pair_count(), 4);
    assert_eq!(changed_total.candidate_pair_count(), 3);
    assert_eq!(changed_total.baseline_observation_count(), 8);
    assert_eq!(changed_total.candidate_observation_count(), 6);
    assert!(!changed_total.passes_no_change_gate());

    let redistributed_population =
      ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
        &[
          SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
          SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
          SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
          SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        ],
        302,
      )
      .expect("redistributed population builds");
    let redistributed = redistributed_population
      .matched_tally(&manifests)
      .expect("redistributed tally builds");
    let same_total_redistribution =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &redistributed)
        .expect("same-total verified tallies compare");
    assert_eq!(same_total_redistribution.baseline_pair_count(), 4);
    assert_eq!(same_total_redistribution.candidate_pair_count(), 4);
    assert_eq!(same_total_redistribution.baseline_observation_count(), 8);
    assert_eq!(same_total_redistribution.candidate_observation_count(), 8);
    assert_eq!(
      same_total_redistribution.entries()[0].baseline_counts(),
      [7, 0, 0, 0, 1]
    );
    assert_eq!(
      same_total_redistribution.entries()[0].candidate_counts(),
      [6, 0, 0, 0, 2]
    );
    assert!(!same_total_redistribution.passes_no_change_gate());

    let reordered_candidate = candidate_population
      .matched_tally(&[manifests[1], manifests[0], manifests[2]])
      .expect("reordered candidate tally builds");
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(
        &baseline,
        &reordered_candidate,
      ),
      Err(ScriptedAgentMatchedScenarioTallyComparisonError::MismatchedRows)
    );

    let mut alternate_observations = candidate_population.observations();
    for pair in &mut alternate_observations {
      pair[0].observer = ALLIED_AUTONOMOUS_ACTOR;
      pair[1].observer = ALLIED_AUTONOMOUS_ACTOR;
    }
    let alternate_sample =
      ScriptedAgentMatchedScenarioSample::from_observations(&alternate_observations, &manifests)
        .expect("alternate observer sample builds");
    let alternate = ScriptedAgentMatchedScenarioTallyReport::from_sample(&alternate_sample);
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &alternate),
      Err(ScriptedAgentMatchedScenarioTallyComparisonError::MismatchedObserver)
    );
  }

  #[test]
  fn profile_aware_tally_largest_delta_candidate_is_stable_and_bounded() {
    let manifests = [
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(81, StreamId::new(82), DrawId::new(83)),
      ),
      ScriptedAgentExperimentManifest::new(
        ScriptedAgentProfile::cautious_v1(),
        ScriptedAgentSeedBundle::new(84, StreamId::new(85), DrawId::new(86)),
      ),
    ];
    let baseline_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      340,
    )
    .expect("baseline candidate builds");
    let candidate_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      348,
    )
    .expect("candidate population builds");
    let baseline = baseline_population
      .matched_tally(&manifests)
      .expect("baseline tally builds");
    let candidate = candidate_population
      .matched_tally(&manifests)
      .expect("candidate tally builds");
    let comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
        .expect("verified tallies compare");
    let selected = comparison
      .largest_delta_candidate()
      .expect("changed comparison has a candidate");
    assert_eq!(
      selected.schema(),
      "m6-scripted-agent-tally-outlier-candidate-v1"
    );
    assert_eq!(
      selected.selection_rule(),
      "m6-largest-absolute-intent-delta-v1"
    );
    assert_eq!(selected.row_index(), 0);
    assert_eq!(selected.profile_id(), "cautious-laner-v1");
    assert_eq!(
      selected.evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(selected.intent(), LaneIntent::Stabilize);
    assert_eq!(selected.delta(), -2);
    assert_eq!(selected.magnitude(), 2);
    assert_eq!(
      selected.magnitude(),
      selected.delta().unsigned_abs(),
      "magnitude retains the bounded absolute signed delta"
    );
    assert_eq!(
      comparison.largest_delta_candidate(),
      Some(selected),
      "repeated ranking is deterministic"
    );

    let reversed =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&candidate, &baseline)
        .expect("reversed verified tallies compare");
    let reversed_selected = reversed
      .largest_delta_candidate()
      .expect("reversed changed comparison has a candidate");
    assert_eq!(reversed_selected.intent(), LaneIntent::Stabilize);
    assert_eq!(reversed_selected.delta(), 2);
    assert_eq!(reversed_selected.magnitude(), 2);

    let unchanged =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
        .expect("unchanged verified tallies compare");
    assert_eq!(unchanged.largest_delta_candidate(), None);
  }

  #[test]
  fn profile_aware_tally_outlier_threshold_is_provisional_and_bounded() {
    let manifest = [ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(91, StreamId::new(92), DrawId::new(93)),
    )];
    let baseline_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      500,
    )
    .expect("baseline population builds");
    let candidate_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      506,
    )
    .expect("candidate population builds");
    let baseline = baseline_population
      .matched_tally(&manifest)
      .expect("baseline tally builds");
    let candidate = candidate_population
      .matched_tally(&manifest)
      .expect("candidate tally builds");
    let below =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
        .expect("verified tallies compare");
    assert_eq!(
      below
        .largest_delta_candidate()
        .expect("magnitude-one candidate exists")
        .magnitude(),
      1
    );
    let below_report = ScriptedAgentTallyOutlierThresholdReport::from_comparison(&below);
    assert_eq!(
      below_report.schema(),
      "m6-scripted-agent-tally-outlier-threshold-v1"
    );
    assert_eq!(
      below_report.rule(),
      "m6-fixed-intent-delta-outlier-threshold-v1"
    );
    assert_eq!(below_report.threshold(), 2);
    assert_eq!(
      below_report.status(),
      ScriptedAgentTallyOutlierThresholdStatus::BelowThreshold
    );
    assert_eq!(below_report.status().id(), "below_threshold");

    let above = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      512,
    )
    .expect("above-threshold population builds")
    .matched_tally(&manifest)
    .expect("above-threshold tally builds");
    let baseline_four = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      520,
    )
    .expect("four-pair baseline population builds")
    .matched_tally(&manifest)
    .expect("four-pair baseline tally builds");
    let above_comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline_four, &above)
        .expect("above-threshold tallies compare");
    assert_eq!(
      above_comparison
        .largest_delta_candidate()
        .expect("magnitude-two candidate exists")
        .magnitude(),
      SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_MAGNITUDE
    );
    let above_report = ScriptedAgentTallyOutlierThresholdReport::from_comparison(&above_comparison);
    assert_eq!(
      above_report.status(),
      ScriptedAgentTallyOutlierThresholdStatus::AboveThreshold
    );
    assert_eq!(above_report.status().id(), "above_threshold");

    let unchanged =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
        .expect("unchanged tallies compare");
    let unchanged_report = ScriptedAgentTallyOutlierThresholdReport::from_comparison(&unchanged);
    assert_eq!(
      unchanged_report.status(),
      ScriptedAgentTallyOutlierThresholdStatus::NoCandidate
    );
    assert_eq!(unchanged_report.status().id(), "no_candidate");
  }

  #[test]
  fn tally_candidate_replay_reference_selects_first_verified_match() {
    let manifest = [ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(101, StreamId::new(102), DrawId::new(103)),
    )];
    let baseline = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      540,
    )
    .expect("baseline population builds")
    .matched_tally(&manifest)
    .expect("baseline tally builds");
    let candidate = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      548,
    )
    .expect("candidate population builds")
    .matched_tally(&manifest)
    .expect("candidate tally builds");
    let comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
        .expect("verified tallies compare");
    let candidate = comparison
      .largest_delta_candidate()
      .expect("largest candidate exists");
    let state = LaneSnapshot::initial();
    let first_observation = observe_player(&state, ObservationId::new(600)).observation();
    let selected_observation = observe_player(&state, ObservationId::new(601)).observation();
    let later_observation = observe_player(&state, ObservationId::new(602)).observation();
    let noise = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::risk_taking_v1(),
      first_observation,
      LaneIntent::Contest,
      None,
    );
    let first_match = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      selected_observation,
      LaneIntent::Stabilize,
      None,
    );
    let later_match = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      later_observation,
      LaneIntent::Stabilize,
      None,
    );
    let reference = ScriptedAgentTallyReplayReference::from_candidate_and_records(
      candidate,
      &[noise, first_match.clone(), later_match],
    )
    .expect("first verified matching replay is selected");
    assert_eq!(
      reference.schema(),
      "m6-scripted-agent-tally-replay-reference-v1"
    );
    assert_eq!(
      reference.selection_rule(),
      "m6-first-verified-candidate-replay-v1"
    );
    assert_eq!(reference.row_index(), candidate.row_index());
    assert_eq!(reference.profile_id(), candidate.profile_id());
    assert_eq!(reference.evaluation_rule(), candidate.evaluation_rule());
    assert_eq!(reference.intent(), candidate.intent());
    assert_eq!(reference.delta(), candidate.delta());
    assert_eq!(reference.magnitude(), candidate.magnitude());
    assert_eq!(reference.observation_id(), ObservationId::new(601));

    let mut mismatched = first_match.clone();
    mismatched
      .decision
      .candidates
      .iter_mut()
      .find(|candidate| candidate.intent() == LaneIntent::Stabilize)
      .expect("selected candidate exists")
      .score += 1;
    let later_reference = ScriptedAgentTallyReplayReference::from_candidate_and_records(
      candidate,
      &[mismatched.clone(), first_match.clone()],
    )
    .expect("later verified matching replay is selected");
    assert_eq!(later_reference.observation_id(), ObservationId::new(601));
    assert_eq!(
      ScriptedAgentTallyReplayReference::from_candidate_and_records(candidate, &[mismatched]),
      Err(ScriptedAgentTallyReplayReferenceError::DecisionMismatch)
    );
    let no_match = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::yielding_v1(),
      first_observation,
      LaneIntent::Yield,
      None,
    );
    assert_eq!(
      ScriptedAgentTallyReplayReference::from_candidate_and_records(candidate, &[no_match]),
      Err(ScriptedAgentTallyReplayReferenceError::NoMatchingReplay)
    );
  }

  #[test]
  fn calibrated_outlier_detection_and_representative_replay_is_deterministic() {
    assert_eq!(
      SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
      "m6-scripted-agent-calibrated-outlier-replay-v1"
    );
    assert_eq!(
      SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
      "m6-calibrated-outlier-representative-replay-v1"
    );
    assert_eq!(SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE, 2);
    assert_eq!(
      ScriptedAgentCalibratedOutlierReplayStatus::Qualified.id(),
      "qualified"
    );
    assert_eq!(
      ScriptedAgentCalibratedOutlierReplayStatus::BelowThreshold.id(),
      "below_threshold"
    );
    assert_eq!(
      ScriptedAgentCalibratedOutlierReplayStatus::NoCandidate.id(),
      "no_candidate"
    );
    assert_eq!(
      ScriptedAgentCalibratedOutlierReplayStatus::NoMatchingReplay.id(),
      "no_matching_replay"
    );
    assert_eq!(
      ScriptedAgentCalibratedOutlierReplayStatus::DecisionMismatch.id(),
      "decision_mismatch"
    );

    let manifest = [ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(101, StreamId::new(102), DrawId::new(103)),
    )];

    // 1. Qualified Outlier: delta >= 2 with matching verified replay record
    let baseline = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      540,
    )
    .expect("baseline population builds")
    .matched_tally(&manifest)
    .expect("baseline tally builds");
    let candidate_tally = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      548,
    )
    .expect("candidate population builds")
    .matched_tally(&manifest)
    .expect("candidate tally builds");
    let comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate_tally)
        .expect("verified tallies compare");
    let candidate = comparison
      .largest_delta_candidate()
      .expect("largest candidate exists");
    assert!(candidate.magnitude() >= SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE);

    let state = LaneSnapshot::initial();
    let noise_observation = observe_player(&state, ObservationId::new(700)).observation();
    let match_observation = observe_player(&state, ObservationId::new(701)).observation();
    let noise_record = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::risk_taking_v1(),
      noise_observation,
      LaneIntent::Contest,
      None,
    );
    let match_record = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      match_observation,
      LaneIntent::Stabilize,
      None,
    );

    let qualified_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
      &comparison,
      &[noise_record.clone(), match_record.clone()],
    );
    assert_eq!(
      qualified_report.schema(),
      SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA
    );
    assert_eq!(
      qualified_report.rule(),
      SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE
    );
    assert_eq!(
      qualified_report.threshold(),
      SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE
    );
    assert_eq!(
      qualified_report.status(),
      ScriptedAgentCalibratedOutlierReplayStatus::Qualified
    );
    assert_eq!(qualified_report.candidate(), Some(candidate));
    assert_eq!(
      qualified_report.observation_id(),
      Some(ObservationId::new(701))
    );

    // 2. Below Threshold: delta = 1
    let below_candidate = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      548,
    )
    .expect("below candidate population builds")
    .matched_tally(&manifest)
    .expect("below candidate tally builds");
    let below_comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &below_candidate)
        .expect("below tallies compare");
    let below_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
      &below_comparison,
      std::slice::from_ref(&match_record),
    );
    assert_eq!(
      below_report.status(),
      ScriptedAgentCalibratedOutlierReplayStatus::BelowThreshold
    );
    assert!(below_report.candidate().is_some());
    assert_eq!(below_report.candidate().unwrap().magnitude(), 1);
    assert_eq!(below_report.observation_id(), None);

    // 3. No Candidate: unchanged baseline vs baseline
    let unchanged_comparison =
      ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
        .expect("unchanged tallies compare");
    let no_cand_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
      &unchanged_comparison,
      std::slice::from_ref(&match_record),
    );
    assert_eq!(
      no_cand_report.status(),
      ScriptedAgentCalibratedOutlierReplayStatus::NoCandidate
    );
    assert_eq!(no_cand_report.candidate(), None);
    assert_eq!(no_cand_report.observation_id(), None);

    // 4. No Matching Replay: delta >= 2 but no matching record
    let no_matching_report =
      ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
        &comparison,
        std::slice::from_ref(&noise_record),
      );
    assert_eq!(
      no_matching_report.status(),
      ScriptedAgentCalibratedOutlierReplayStatus::NoMatchingReplay
    );
    assert_eq!(no_matching_report.candidate(), Some(candidate));
    assert_eq!(no_matching_report.observation_id(), None);

    // 5. Decision Mismatch: delta >= 2 with corrupted replay record
    let mut corrupted = match_record;
    corrupted
      .decision
      .candidates
      .iter_mut()
      .find(|c| c.intent() == LaneIntent::Stabilize)
      .expect("candidate exists")
      .score += 1;
    let mismatch_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
      &comparison,
      std::slice::from_ref(&corrupted),
    );
    assert_eq!(
      mismatch_report.status(),
      ScriptedAgentCalibratedOutlierReplayStatus::DecisionMismatch
    );
    assert_eq!(mismatch_report.candidate(), Some(candidate));
    assert_eq!(
      mismatch_report.observation_id(),
      Some(ObservationId::new(701))
    );
  }

  #[test]
  fn fixture_scenario_frequency_report_counts_ordered_selection() {
    let selection = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(130), ObservationId::new(131)],
        [ObservationId::new(132), ObservationId::new(133)],
        [ObservationId::new(134), ObservationId::new(135)],
        [ObservationId::new(136), ObservationId::new(137)],
      ],
    )
    .expect("frequency selection builds");
    let report = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&selection);
    assert_eq!(
      SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA,
      "m6-scripted-agent-fixture-frequency-v1"
    );
    assert_eq!(
      report.schema(),
      SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA
    );
    assert_eq!(report.selection_count(), 4);
    assert_eq!(report.entries()[0].scenario_id(), "safe-fixture-v1");
    assert_eq!(report.entries()[0].count(), 2);
    assert_eq!(report.entries()[1].scenario_id(), "river-side-threat-v1");
    assert_eq!(report.entries()[1].count(), 2);
    assert_eq!(
      report.entries()[0].count() + report.entries()[1].count(),
      report.selection_count()
    );
    assert_eq!(SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE, 10_000);
    assert_eq!(report.distribution_basis_points(), [5_000, 5_000]);
    assert_eq!(
      report.distribution_basis_points().iter().sum::<u16>(),
      10_000
    );
    assert_eq!(
      report,
      ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&selection)
    );
    let encoded = report.encode();
    assert_eq!(
      encoded,
      "schema=m6-scripted-agent-fixture-frequency-v1\nselection_count=4\nentries=2\nrow=safe-fixture-v1|2\nrow=river-side-threat-v1|2\n"
    );
    assert_eq!(
      report.to_markdown(),
      "# Scenario Frequency\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n\n| scenario_id | count |\n| --- | ---: |\n| safe-fixture-v1 | 2 |\n| river-side-threat-v1 | 2 |\n"
    );
    assert_eq!(
      report.to_distribution_markdown(),
      "# Scenario Distribution\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n- share_scale_basis_points: 10000\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| safe-fixture-v1 | 2 | 5000 |\n| river-side-threat-v1 | 2 | 5000 |\n"
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioFrequencyReport::decode(&encoded, &report),
      Ok(report.clone())
    );

    let singleton = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
      &[[ObservationId::new(140), ObservationId::new(141)]],
    )
    .expect("singleton selection builds");
    let singleton_report = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&singleton);
    assert_eq!(singleton_report.selection_count(), 1);
    assert_eq!(
      singleton_report.entries()[0].scenario_id(),
      "safe-fixture-v1"
    );
    assert_eq!(singleton_report.entries()[0].count(), 1);
    assert_eq!(
      singleton_report.entries()[1].scenario_id(),
      "river-side-threat-v1"
    );
    assert_eq!(singleton_report.entries()[1].count(), 0);
    assert_eq!(
      singleton_report.entries()[0].count() + singleton_report.entries()[1].count(),
      singleton_report.selection_count()
    );
    assert_eq!(singleton_report.distribution_basis_points(), [10_000, 0]);
    let singleton_encoded = singleton_report.encode();
    assert_eq!(
      singleton_report.to_markdown(),
      "# Scenario Frequency\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 1\n\n| scenario_id | count |\n| --- | ---: |\n| safe-fixture-v1 | 1 |\n| river-side-threat-v1 | 0 |\n"
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioFrequencyReport::decode(&singleton_encoded, &singleton_report,),
      Ok(singleton_report.clone())
    );

    let skewed_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(142), ObservationId::new(143)],
        [ObservationId::new(144), ObservationId::new(145)],
        [ObservationId::new(146), ObservationId::new(147)],
        [ObservationId::new(148), ObservationId::new(149)],
      ],
    )
    .expect("skewed selection builds");
    let skewed_report =
      ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&skewed_selection);
    assert_eq!(skewed_report.selection_count(), 4);
    assert_eq!(
      skewed_report
        .entries()
        .iter()
        .map(|entry| entry.count())
        .collect::<Vec<_>>(),
      vec![1, 3]
    );
    assert_eq!(skewed_report.distribution_basis_points(), [2_500, 7_500]);
    assert_eq!(
      skewed_report
        .distribution_basis_points()
        .iter()
        .sum::<u16>(),
      SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE
    );
    assert_eq!(
      skewed_report.to_distribution_markdown(),
      "# Scenario Distribution\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n- share_scale_basis_points: 10000\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| safe-fixture-v1 | 1 | 2500 |\n| river-side-threat-v1 | 3 | 7500 |\n"
    );

    let all_safe_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(150), ObservationId::new(151)],
        [ObservationId::new(152), ObservationId::new(153)],
        [ObservationId::new(154), ObservationId::new(155)],
        [ObservationId::new(156), ObservationId::new(157)],
      ],
    )
    .expect("all-safe selection builds");
    let all_safe_report =
      ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&all_safe_selection);
    assert_eq!(all_safe_report.distribution_basis_points(), [10_000, 0]);
    assert_eq!(
      all_safe_report.to_distribution_markdown(),
      "# Scenario Distribution\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n- share_scale_basis_points: 10000\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| safe-fixture-v1 | 4 | 10000 |\n| river-side-threat-v1 | 0 | 0 |\n"
    );
    for malformed in [
      (
        encoded.replacen(
          "schema=m6-scripted-agent-fixture-frequency-v1",
          "schema=other",
          1,
        ),
        ScriptedAgentFixtureScenarioFrequencyCodecError::UnsupportedSchema,
      ),
      (
        encoded.replacen("entries=2", "unknown=2", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::UnknownField,
      ),
      (
        encoded.replacen(
          "entries=2",
          "schema=m6-scripted-agent-fixture-frequency-v1",
          1,
        ),
        ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField,
      ),
      (
        encoded.replacen("entries=2\n", "", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::MissingField,
      ),
      (
        format!("{encoded}row=safe-fixture-v1|2\n"),
        ScriptedAgentFixtureScenarioFrequencyCodecError::UnexpectedLineCount {
          expected: 5,
          actual: 6,
        },
      ),
      (
        encoded.replacen("row=safe-fixture-v1|2", "row=unknown-fixture-v1|2", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("row=safe-fixture-v1|2", "row=safe-fixture-v1|oops", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("row=safe-fixture-v1|2", "row=safe-fixture-v1|1", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("row=safe-fixture-v1|2", "row=safe-fixture-v1|255", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
      ),
      (
        encoded.replacen("entries=2", "entries=3", 1),
        ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
      ),
    ] {
      assert_eq!(
        ScriptedAgentFixtureScenarioFrequencyReport::decode(&malformed.0, &report),
        Err(malformed.1)
      );
    }
    assert_eq!(
      ScriptedAgentFixtureScenarioFrequencyReport::decode(
        &encoded.replacen(
          "row=safe-fixture-v1|2\nrow=river-side-threat-v1|2",
          "row=safe-fixture-v1|1\nrow=river-side-threat-v1|3",
          1,
        ),
        &report,
      ),
      Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InputMismatch)
    );
    assert_eq!(
      ScriptedAgentFixtureScenarioFrequencyReport::decode(
        &"x".repeat(MAX_SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_BYTES + 1),
        &report,
      ),
      Err(ScriptedAgentFixtureScenarioFrequencyCodecError::Oversized)
    );
  }

  #[test]
  fn fixture_frequency_report_comparison_preserves_declared_order_and_deltas() {
    let baseline_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(150), ObservationId::new(151)],
        [ObservationId::new(152), ObservationId::new(153)],
      ],
    )
    .expect("baseline selection builds");
    let candidate_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(154), ObservationId::new(155)],
        [ObservationId::new(156), ObservationId::new(157)],
        [ObservationId::new(158), ObservationId::new(159)],
        [ObservationId::new(160), ObservationId::new(161)],
      ],
    )
    .expect("candidate selection builds");
    let baseline = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&baseline_selection);
    let candidate =
      ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&candidate_selection);
    let comparison =
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &candidate);
    assert_eq!(
      SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_COMPARISON_SCHEMA,
      "m6-scripted-agent-fixture-frequency-compare-v1"
    );
    assert_eq!(
      comparison.schema(),
      "m6-scripted-agent-fixture-frequency-compare-v1"
    );
    assert_eq!(comparison.baseline_build_id(), None);
    assert_eq!(comparison.candidate_build_id(), None);
    assert_eq!(comparison.baseline_selection_count(), 2);
    assert_eq!(comparison.candidate_selection_count(), 4);
    assert_eq!(comparison.entries()[0].scenario_id(), "safe-fixture-v1");
    assert_eq!(comparison.entries()[0].baseline_count(), 1);
    assert_eq!(comparison.entries()[0].candidate_count(), 2);
    assert_eq!(comparison.entries()[0].delta(), 1);
    assert_eq!(
      comparison.entries()[1].scenario_id(),
      "river-side-threat-v1"
    );
    assert_eq!(comparison.entries()[1].baseline_count(), 1);
    assert_eq!(comparison.entries()[1].candidate_count(), 2);
    assert_eq!(comparison.entries()[1].delta(), 1);
    assert_eq!(
      comparison.regression_rule(),
      "m6-fixed-frequency-no-change-v1"
    );
    assert!(!comparison.passes_no_change_gate());
    assert_eq!(
      comparison,
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &candidate)
    );
    let reversed =
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&candidate, &baseline);
    assert_eq!(reversed.entries()[0].delta(), -1);
    assert_eq!(reversed.entries()[1].delta(), -1);
    let unchanged =
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &baseline);
    assert!(unchanged.passes_no_change_gate());
    let baseline_build = ScriptedAgentBuildId::new(140);
    let candidate_build = ScriptedAgentBuildId::new(141);
    assert_eq!(baseline_build.schema(), "m6-scripted-agent-build-id-v1");
    assert_eq!(baseline_build.value(), 140);
    let labeled =
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
        &baseline,
        &candidate,
        baseline_build,
        candidate_build,
      )
      .expect("distinct build labels compare");
    assert_eq!(labeled.baseline_build_id(), Some(baseline_build));
    assert_eq!(labeled.candidate_build_id(), Some(candidate_build));
    assert_eq!(labeled.entries(), comparison.entries());
    assert_eq!(
      labeled.baseline_selection_count(),
      comparison.baseline_selection_count()
    );
    assert_eq!(
      labeled.candidate_selection_count(),
      comparison.candidate_selection_count()
    );
    assert_eq!(
      labeled.passes_no_change_gate(),
      comparison.passes_no_change_gate()
    );
    assert_eq!(
      labeled,
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
        &baseline,
        &candidate,
        baseline_build,
        candidate_build,
      )
      .expect("repeated labeled comparison is stable")
    );
    let labeled_unchanged =
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
        &baseline,
        &baseline,
        baseline_build,
        candidate_build,
      )
      .expect("distinct labels retain unchanged comparison");
    assert_eq!(labeled_unchanged.baseline_selection_count(), 2);
    assert_eq!(labeled_unchanged.candidate_selection_count(), 2);
    assert!(labeled_unchanged.passes_no_change_gate());
    assert_eq!(
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
        &baseline,
        &candidate,
        baseline_build,
        baseline_build,
      ),
      Err(ScriptedAgentBuildComparisonError::MatchingBuildIds)
    );
    let redistributed_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(162), ObservationId::new(163)],
        [ObservationId::new(164), ObservationId::new(165)],
      ],
    )
    .expect("redistributed selection builds");
    let redistributed =
      ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&redistributed_selection);
    let same_total_redistribution =
      ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(
        &baseline,
        &redistributed,
      );
    assert_eq!(same_total_redistribution.baseline_selection_count(), 2);
    assert_eq!(same_total_redistribution.candidate_selection_count(), 2);
    assert_eq!(same_total_redistribution.entries()[0].candidate_count(), 2);
    assert_eq!(same_total_redistribution.entries()[1].candidate_count(), 0);
    assert!(!same_total_redistribution.passes_no_change_gate());
  }

  #[test]
  fn evaluation_rejects_intents_outside_the_actor_visible_candidate_set() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(13)).observation();

    assert_eq!(
      ScriptedAgent::cautious_v1().evaluate_candidate(observation, LaneIntent::Withdraw),
      Err(ScriptedAgentEvaluationError::UnavailableIntent)
    );
  }

  #[test]
  fn cautious_agent_prioritizes_visible_threat_response_without_hidden_state() {
    let initial = LaneSnapshot::initial();
    let state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let observation = observe_player(&state, ObservationId::new(10)).observation();
    let decision = ScriptedAgent::cautious_v1().choose(observation);

    assert_eq!(decision.selected_intent(), LaneIntent::Withdraw);
    assert!(decision.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
        && candidate.score() == 100
    }));
    assert_eq!(
      ScriptedAgent::cautious_v1()
        .evaluate_candidate(observation, LaneIntent::Withdraw)
        .expect("visible threat response evaluates"),
      ScriptedAgentCandidate {
        intent: LaneIntent::Withdraw,
        score: 100,
        reason: ScriptedAgentReason::ThreatResponse,
      }
    );
  }

  #[test]
  fn cautious_agent_decision_is_reproducible_for_identical_observation() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(11)).observation();
    let agent = ScriptedAgent::cautious_v1();

    assert_eq!(agent.choose(observation), agent.choose(observation));
  }

  #[test]
  fn seeded_decision_records_bundle_and_repeats_for_identical_inputs() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(19)).observation();
    let seed = ScriptedAgentSeedBundle::new(42, StreamId::new(21), DrawId::new(3));
    let decision = ScriptedAgent::cautious_v1().choose_with_seed(observation, seed);

    assert_eq!(seed.schema(), "m4-scripted-agent-random-v1");
    assert_eq!(seed.seed(), 42);
    assert_eq!(seed.policy_trace().stream().value(), 21);
    assert_eq!(seed.policy_trace().draw().value(), 3);
    assert_eq!(decision.seed_bundle(), Some(seed));
    assert_eq!(decision.selection_rule(), "max-score-seeded-tie-v1");
    assert_eq!(decision.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(
      decision,
      ScriptedAgent::cautious_v1().choose_with_seed(observation, seed)
    );
    validate_lane_request(
      &state,
      &observe_player(&state, ObservationId::new(19)),
      &decision.request(),
    )
    .expect("seeded policy request is legal");
  }

  #[test]
  fn seeded_tie_selection_is_reproducible_and_stream_scoped() {
    let candidates = [
      ScriptedAgentCandidate {
        intent: LaneIntent::Contest,
        score: 70,
        reason: ScriptedAgentReason::AvailableAlternative,
      },
      ScriptedAgentCandidate {
        intent: LaneIntent::Stabilize,
        score: 70,
        reason: ScriptedAgentReason::StableDefault,
      },
    ];
    let first_seed = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(3));
    let same_seed = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(3));
    let next_draw = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(4));

    let first = ScriptedAgent::select_candidate_with_seed(&candidates, first_seed);
    assert_eq!(
      first,
      ScriptedAgent::select_candidate_with_seed(&candidates, same_seed)
    );
    assert_ne!(
      first,
      ScriptedAgent::select_candidate_with_seed(&candidates, next_draw)
    );
  }

  #[test]
  fn decision_replay_classifies_expected_and_declared_anomalous_cases() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(20)).observation();
    let expected = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      None,
    );
    let anomalous = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Contest,
      None,
    );
    let seeded = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      Some(ScriptedAgentSeedBundle::new(
        42,
        StreamId::new(21),
        DrawId::new(3),
      )),
    );

    assert_eq!(expected.schema(), "m4-scripted-agent-replay-v1");
    assert_eq!(expected.profile().profile_id(), SCRIPTED_AGENT_PROFILE_ID);
    assert_eq!(
      expected.disposition(),
      ScriptedAgentReplayDisposition::Expected
    );
    assert_eq!(expected.expected_intent(), LaneIntent::Stabilize);
    assert_eq!(expected.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(expected.replay(), Ok(expected.decision().clone()));
    assert_eq!(
      anomalous.disposition(),
      ScriptedAgentReplayDisposition::Anomalous
    );
    assert_eq!(anomalous.expected_intent(), LaneIntent::Contest);
    assert_eq!(anomalous.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(anomalous.replay(), Ok(anomalous.decision().clone()));
    assert_eq!(
      seeded.disposition(),
      ScriptedAgentReplayDisposition::Expected
    );
    assert!(seeded.seed_bundle().is_some());
    assert_eq!(seeded.replay(), Ok(seeded.decision().clone()));
  }

  #[test]
  fn decision_replay_rejects_tampered_recorded_decision() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(22)).observation();
    let mut record = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      None,
    );
    record.decision.selected_intent = LaneIntent::Contest;

    assert_eq!(
      record.replay(),
      Err(ScriptedAgentReplayError::DecisionMismatch)
    );

    let mut seeded_record = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      Some(ScriptedAgentSeedBundle::new(
        42,
        StreamId::new(21),
        DrawId::new(3),
      )),
    );
    seeded_record.decision.seed_bundle = Some(ScriptedAgentSeedBundle::new(
      99,
      StreamId::new(22),
      DrawId::new(4),
    ));

    assert_eq!(
      seeded_record.replay(),
      Err(ScriptedAgentReplayError::DecisionMismatch)
    );
  }

  #[test]
  fn replay_sequence_evidence_binds_decision_identity_and_log_status() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(23)).observation();
    let expected = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      observation,
      LaneIntent::Stabilize,
      None,
    );
    let mut complete = ScriptedAgentOperationalLog::new();
    for event in [
      ScriptedAgentOperationalEvent::BatchStarted,
      ScriptedAgentOperationalEvent::ChunkCompleted,
      ScriptedAgentOperationalEvent::BatchFinished,
    ] {
      complete.append(event).expect("sequence fixture fits");
    }
    let evidence =
      ScriptedAgentReplaySequenceEvidenceReport::from_record_and_log(&expected, &complete);
    assert_eq!(
      evidence.schema(),
      "m6-scripted-agent-replay-sequence-evidence-v1"
    );
    assert_eq!(
      evidence.rule(),
      "m6-replay-identity-operational-sequence-v1"
    );
    assert_eq!(
      evidence.replay_identity(),
      ScriptedAgentReplayIdentityStatus::Verified
    );
    assert_eq!(evidence.replay_identity().id(), "verified");
    assert_eq!(
      evidence.sequence_status(),
      ScriptedAgentOperationalLogSequenceStatus::Complete
    );

    let mut incomplete = ScriptedAgentOperationalLog::new();
    incomplete
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("sequence fixture fits");
    assert_eq!(
      ScriptedAgentReplaySequenceEvidenceReport::from_record_and_log(&expected, &incomplete,)
        .sequence_status(),
      ScriptedAgentOperationalLogSequenceStatus::MissingChunk
    );

    let mut tampered = expected.clone();
    tampered.decision.selected_intent = LaneIntent::Contest;
    let mismatch =
      ScriptedAgentReplaySequenceEvidenceReport::from_record_and_log(&tampered, &complete);
    assert_eq!(
      mismatch.replay_identity(),
      ScriptedAgentReplayIdentityStatus::DecisionMismatch
    );
    assert_eq!(mismatch.replay_identity().id(), "decision_mismatch");
    assert_eq!(
      mismatch.sequence_status(),
      ScriptedAgentOperationalLogSequenceStatus::Complete
    );
  }

  #[test]
  fn scenario_replay_identity_verifies_sequence_and_rejects_malformed_input() {
    let state = LaneSnapshot::initial();
    let obs1 = observe_player(&state, ObservationId::new(101)).observation();
    let obs2 = observe_player(&state, ObservationId::new(102)).observation();
    let obs3 = observe_player(&state, ObservationId::new(103)).observation();

    let rec1 = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      obs1,
      LaneIntent::Stabilize,
      None,
    );
    let rec2 = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::risk_taking_v1(),
      obs2,
      LaneIntent::Contest,
      None,
    );
    let rec3 = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::yielding_v1(),
      obs3,
      LaneIntent::Yield,
      None,
    );

    let report = ScriptedAgentScenarioReplayIdentityReport::from_records(&[
      rec1.clone(),
      rec2.clone(),
      rec3.clone(),
    ])
    .expect("valid sequence verifies");

    assert_eq!(
      report.schema(),
      "m6-scripted-agent-scenario-replay-identity-v1"
    );
    assert_eq!(report.rule(), "m6-scenario-replay-identity-v1");
    assert_eq!(report.record_count(), 3);
    assert_eq!(report.verified_count(), 3);
    assert_eq!(
      report.status(),
      ScriptedAgentScenarioReplayIdentityStatus::AllVerified
    );
    assert_eq!(report.status().id(), "all_verified");
    assert_eq!(report.start_observation_id(), ObservationId::new(101));
    assert_eq!(report.end_observation_id(), ObservationId::new(103));

    // Decision mismatch in one record
    let mut tampered = rec2.clone();
    tampered.decision.selected_intent = LaneIntent::Stabilize;
    let mismatch_report = ScriptedAgentScenarioReplayIdentityReport::from_records(&[
      rec1.clone(),
      tampered,
      rec3.clone(),
    ])
    .expect("evaluates with mismatch");
    assert_eq!(mismatch_report.record_count(), 3);
    assert_eq!(mismatch_report.verified_count(), 2);
    assert_eq!(
      mismatch_report.status(),
      ScriptedAgentScenarioReplayIdentityStatus::DecisionMismatch
    );
    assert_eq!(mismatch_report.status().id(), "decision_mismatch");

    // Empty input fails closed
    assert_eq!(
      ScriptedAgentScenarioReplayIdentityReport::from_records(&[]),
      Err(ScriptedAgentScenarioReplayIdentityError::Empty)
    );

    // Duplicate observation ID fails closed
    let duplicate_obs = observe_player(&state, ObservationId::new(101)).observation();
    let duplicate_rec = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::yielding_v1(),
      duplicate_obs,
      LaneIntent::Yield,
      None,
    );
    assert_eq!(
      ScriptedAgentScenarioReplayIdentityReport::from_records(&[rec1.clone(), duplicate_rec]),
      Err(ScriptedAgentScenarioReplayIdentityError::DuplicateObservationId)
    );

    // Oversized input fails closed
    let mut oversized = Vec::new();
    for i in 0..=MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS {
      let obs_id = u64::try_from(i.saturating_add(200)).expect("fits in u64");
      let obs = observe_player(&state, ObservationId::new(obs_id)).observation();
      oversized.push(ScriptedAgentReplayRecord::capture(
        ScriptedAgent::cautious_v1(),
        obs,
        LaneIntent::Stabilize,
        None,
      ));
    }
    assert_eq!(
      ScriptedAgentScenarioReplayIdentityReport::from_records(&oversized),
      Err(ScriptedAgentScenarioReplayIdentityError::Oversized)
    );
  }

  #[test]
  fn scenario_causal_trace_completeness_verifies_sequence_and_rejects_malformed_input() {
    let state = LaneSnapshot::initial();
    let obs1 = observe_player(&state, ObservationId::new(201)).observation();
    let obs2 = observe_player(&state, ObservationId::new(202)).observation();
    let obs3 = observe_player(&state, ObservationId::new(203)).observation();

    let rec1 = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      obs1,
      LaneIntent::Stabilize,
      None,
    );
    let rec2 = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::risk_taking_v1(),
      obs2,
      LaneIntent::Contest,
      None,
    );
    let rec3 = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::yielding_v1(),
      obs3,
      LaneIntent::Yield,
      None,
    );

    let report = ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[
      rec1.clone(),
      rec2.clone(),
      rec3.clone(),
    ])
    .expect("valid sequence verifies causal completeness");

    assert_eq!(
      report.schema(),
      "m6-scripted-agent-scenario-causal-trace-completeness-v1"
    );
    assert_eq!(report.rule(), "m6-scenario-causal-trace-completeness-v1");
    assert_eq!(report.record_count(), 3);
    assert_eq!(report.traced_count(), 3);
    assert_eq!(
      report.status(),
      ScriptedAgentScenarioCausalTraceCompletenessStatus::AllComplete
    );
    assert_eq!(report.status().id(), "all_complete");
    assert_eq!(report.start_observation_id(), ObservationId::new(201));
    assert_eq!(report.end_observation_id(), ObservationId::new(203));

    // Decision mismatch in one record makes it incomplete
    let mut tampered = rec2.clone();
    tampered.decision.selected_intent = LaneIntent::Stabilize;
    let incomplete_report = ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[
      rec1.clone(),
      tampered,
      rec3.clone(),
    ])
    .expect("evaluates with incomplete trace");
    assert_eq!(incomplete_report.record_count(), 3);
    assert_eq!(incomplete_report.traced_count(), 2);
    assert_eq!(
      incomplete_report.status(),
      ScriptedAgentScenarioCausalTraceCompletenessStatus::IncompleteTrace
    );
    assert_eq!(incomplete_report.status().id(), "incomplete_trace");

    // Empty input fails closed
    assert_eq!(
      ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[]),
      Err(ScriptedAgentScenarioCausalTraceCompletenessError::Empty)
    );

    // Duplicate observation ID fails closed
    let duplicate_obs = observe_player(&state, ObservationId::new(201)).observation();
    let duplicate_rec = ScriptedAgentReplayRecord::capture(
      ScriptedAgent::yielding_v1(),
      duplicate_obs,
      LaneIntent::Yield,
      None,
    );
    assert_eq!(
      ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[
        rec1.clone(),
        duplicate_rec
      ]),
      Err(ScriptedAgentScenarioCausalTraceCompletenessError::DuplicateObservationId)
    );

    // Oversized input fails closed
    let mut oversized = Vec::new();
    for i in 0..=MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS {
      let obs_id = u64::try_from(i.saturating_add(300)).expect("fits in u64");
      let obs = observe_player(&state, ObservationId::new(obs_id)).observation();
      oversized.push(ScriptedAgentReplayRecord::capture(
        ScriptedAgent::cautious_v1(),
        obs,
        LaneIntent::Stabilize,
        None,
      ));
    }
    assert_eq!(
      ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&oversized),
      Err(ScriptedAgentScenarioCausalTraceCompletenessError::Oversized)
    );
  }

  #[test]
  fn cautious_agent_stabilize_score_rises_with_observed_wave_pressure() {
    let initial = LaneSnapshot::initial();
    let low_pressure = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      WaveState::new(WavePressure::new(0).expect("bounded pressure")),
      initial.jungle_threat(),
    );
    let high_pressure = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      WaveState::new(WavePressure::new(3).expect("bounded pressure")),
      initial.jungle_threat(),
    );
    let low_receipt = observe_player(&low_pressure, ObservationId::new(17));
    let high_receipt = observe_player(&high_pressure, ObservationId::new(17));
    let agent = ScriptedAgent::cautious_v1();
    let low = agent
      .evaluate_candidate(low_receipt.observation(), LaneIntent::Stabilize)
      .expect("stabilize is advertised at low pressure");
    let high = agent
      .evaluate_candidate(high_receipt.observation(), LaneIntent::Stabilize)
      .expect("stabilize is advertised at high pressure");

    assert_eq!(low.score(), 80);
    assert_eq!(high.score(), 83);
    assert!(high.score() > low.score());
    assert_eq!(
      agent.choose(low_receipt.observation()).selected_intent(),
      LaneIntent::Stabilize
    );
    assert_eq!(
      agent.choose(high_receipt.observation()).selected_intent(),
      LaneIntent::Stabilize
    );
    validate_lane_request(
      &low_pressure,
      &low_receipt,
      &agent.choose(low_receipt.observation()).request(),
    )
    .expect("low-pressure request is legal");
    validate_lane_request(
      &high_pressure,
      &high_receipt,
      &agent.choose(high_receipt.observation()).request(),
    )
    .expect("high-pressure request is legal");
  }

  #[test]
  fn candidate_breadth_tracks_only_actor_visible_advertisements() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let safe = observe_player(&initial, ObservationId::new(18)).observation();
    let threat = observe_player(&threat_state, ObservationId::new(18)).observation();
    let agent = ScriptedAgent::cautious_v1();
    let safe_candidates = agent.generate_candidates(safe);
    let threat_candidates = agent.generate_candidates(threat);

    assert_eq!(safe_candidates.len(), 4);
    assert_eq!(threat_candidates.len(), 5);
    assert_eq!(safe_candidates, safe.available_intents().to_vec());
    assert!(safe_candidates.iter().all(|intent| {
      safe.available_intents().contains(intent) || safe.available_threat_response() == Some(*intent)
    }));
    assert!(threat_candidates.iter().all(|intent| {
      threat.available_intents().contains(intent)
        || threat.available_threat_response() == Some(*intent)
    }));
    assert_eq!(
      threat_candidates
        .iter()
        .filter(|intent| **intent == LaneIntent::Withdraw)
        .count(),
      1
    );
    for candidates in [safe_candidates, threat_candidates] {
      for (index, candidate) in candidates.iter().enumerate() {
        assert!(!candidates[index + 1..].contains(candidate));
      }
    }
    assert_eq!(agent.choose(safe).selected_intent(), LaneIntent::Stabilize);
    assert_eq!(agent.choose(threat).selected_intent(), LaneIntent::Withdraw);
  }

  #[test]
  fn stable_selection_keeps_the_first_advertised_maximum() {
    let candidates = [
      ScriptedAgentCandidate {
        intent: LaneIntent::Contest,
        score: 70,
        reason: ScriptedAgentReason::AvailableAlternative,
      },
      ScriptedAgentCandidate {
        intent: LaneIntent::Stabilize,
        score: 70,
        reason: ScriptedAgentReason::StableDefault,
      },
      ScriptedAgentCandidate {
        intent: LaneIntent::Yield,
        score: 60,
        reason: ScriptedAgentReason::AvailableAlternative,
      },
    ];

    assert_eq!(
      ScriptedAgent::select_candidate(&candidates).intent(),
      LaneIntent::Contest
    );
  }

  #[test]
  fn matched_observation_distinguishes_three_profiles() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(12));
    let cautious = ScriptedAgent::cautious_v1().choose(receipt.observation());
    let risk_taking = ScriptedAgent::risk_taking_v1().choose(receipt.observation());
    let yielding = ScriptedAgent::yielding_v1().choose(receipt.observation());

    assert_eq!(cautious.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(risk_taking.selected_intent(), LaneIntent::Contest);
    assert_eq!(yielding.selected_intent(), LaneIntent::Yield);
    assert_eq!(cautious.profile().role(), ScriptedAgentRole::Anchor);
    assert_eq!(risk_taking.profile().role(), ScriptedAgentRole::Duelist);
    assert_eq!(yielding.profile().role(), ScriptedAgentRole::Pacer);
    assert_eq!(cautious.profile().role().id(), "anchor-v1");
    assert_eq!(risk_taking.profile().role().id(), "duelist-v1");
    assert_eq!(yielding.profile().role().id(), "pacer-v1");
    assert_eq!(
      cautious.profile().selection_rule(),
      "max-score-stable-order-v1"
    );
    assert_eq!(
      risk_taking.profile().selection_rule(),
      "max-score-stable-order-v1"
    );
    assert_eq!(
      yielding.profile().selection_rule(),
      "max-score-stable-order-v1"
    );
    assert_eq!(cautious.profile().preferred_intent(), LaneIntent::Stabilize);
    assert_eq!(
      risk_taking.profile().preferred_intent(),
      LaneIntent::Contest
    );
    assert_eq!(yielding.profile().preferred_intent(), LaneIntent::Yield);
    assert_eq!(
      cautious.profile().evaluation_rule(),
      "threat-first-pressure-aware-fixed-score-v1"
    );
    assert_eq!(
      risk_taking.profile().profile_id(),
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      risk_taking.profile().evaluation_rule(),
      "contest-first-fixed-score-v1"
    );
    assert_eq!(
      yielding.profile().profile_id(),
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    );
    assert_eq!(
      yielding.profile().evaluation_rule(),
      "yield-first-fixed-score-v1"
    );
    assert_eq!(
      cautious
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>(),
      risk_taking
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>()
    );
    assert_eq!(
      cautious
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>(),
      yielding
        .candidates()
        .iter()
        .map(|candidate| candidate.intent())
        .collect::<Vec<_>>()
    );
    assert!(risk_taking.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Contest
        && candidate.reason() == ScriptedAgentReason::RiskPreference
        && candidate.score() == 100
    }));
    assert!(yielding.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Yield
        && candidate.reason() == ScriptedAgentReason::YieldPreference
        && candidate.score() == 100
    }));
    assert_eq!(
      risk_taking,
      ScriptedAgent::risk_taking_v1().choose(receipt.observation())
    );
    assert_eq!(
      yielding,
      ScriptedAgent::yielding_v1().choose(receipt.observation())
    );
    validate_lane_request(&state, &receipt, &cautious.request()).expect("cautious is legal");
    validate_lane_request(&state, &receipt, &risk_taking.request()).expect("risk-taking is legal");
    validate_lane_request(&state, &receipt, &yielding.request()).expect("yielding is legal");
  }

  #[test]
  fn comparison_report_is_versioned_bounded_and_reproducible() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(16)).observation();
    let report = ScriptedAgentComparisonReport::from_observation(observation);

    assert_eq!(
      SCRIPTED_AGENT_METRICS_SCHEMA,
      "m4-scripted-agent-metrics-v1"
    );
    assert_eq!(report.schema(), SCRIPTED_AGENT_METRICS_SCHEMA);
    assert_eq!(report.observer(), observation.observer());
    assert_eq!(report.observation_id(), observation.observation_id());
    assert_eq!(report.entries().len(), 3);
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.profile_id())
        .collect::<Vec<_>>(),
      vec![
        SCRIPTED_AGENT_PROFILE_ID,
        RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
        YIELDING_SCRIPTED_AGENT_PROFILE_ID
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.evaluation_rule())
        .collect::<Vec<_>>(),
      vec![
        "threat-first-pressure-aware-fixed-score-v1",
        "contest-first-fixed-score-v1",
        "yield-first-fixed-score-v1"
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.selected_intent())
        .collect::<Vec<_>>(),
      vec![
        LaneIntent::Stabilize,
        LaneIntent::Contest,
        LaneIntent::Yield
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.selected_score())
        .collect::<Vec<_>>(),
      vec![81, 100, 100]
    );
    assert!(
      report
        .entries()
        .iter()
        .all(|entry| entry.candidate_count() == 4)
    );
    assert_eq!(
      report,
      ScriptedAgentComparisonReport::from_observation(observation)
    );
  }

  #[test]
  fn action_tally_reports_bounded_profile_counts_and_rejects_mixed_observers() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let safe_receipt = observe_player(&initial, ObservationId::new(14));
    let threat_receipt = observe_player(&threat_state, ObservationId::new(15));
    let report = ScriptedAgentActionTallyReport::from_observations([
      safe_receipt.observation(),
      threat_receipt.observation(),
    ])
    .expect("matched player observations tally");
    assert_eq!(
      report,
      ScriptedAgentActionTallyReport::from_observations([
        safe_receipt.observation(),
        threat_receipt.observation(),
      ])
      .expect("repeated matched observations tally")
    );

    assert_eq!(
      SCRIPTED_AGENT_ACTION_TALLY_SCHEMA,
      "m4-scripted-agent-action-tally-v2"
    );
    assert_eq!(report.schema(), SCRIPTED_AGENT_ACTION_TALLY_SCHEMA);
    assert_eq!(report.observer(), safe_receipt.observation().observer());
    assert_eq!(
      report.observation_ids(),
      &[ObservationId::new(14), ObservationId::new(15)]
    );
    assert_eq!(report.entries().len(), 3);
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.profile_id())
        .collect::<Vec<_>>(),
      vec![
        SCRIPTED_AGENT_PROFILE_ID,
        RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
        YIELDING_SCRIPTED_AGENT_PROFILE_ID
      ]
    );
    assert_eq!(
      report
        .entries()
        .iter()
        .map(|entry| entry.evaluation_rule())
        .collect::<Vec<_>>(),
      vec![
        "threat-first-pressure-aware-fixed-score-v1",
        "contest-first-fixed-score-v1",
        "yield-first-fixed-score-v1"
      ]
    );
    let cautious = report.entries()[0];
    assert_eq!(cautious.observation_count(), 2);
    assert_eq!(cautious.stabilize_count(), 1);
    assert_eq!(cautious.withdraw_count(), 1);
    assert_eq!(cautious.contest_count(), 0);
    assert_eq!(cautious.yield_count(), 0);
    assert_eq!(cautious.recall_count(), 0);
    let risk_taking = report.entries()[1];
    assert_eq!(risk_taking.contest_count(), 2);
    assert_eq!(risk_taking.stabilize_count(), 0);
    assert_eq!(risk_taking.withdraw_count(), 0);
    let yielding = report.entries()[2];
    assert_eq!(yielding.yield_count(), 2);
    assert_eq!(yielding.stabilize_count(), 0);
    assert_eq!(yielding.withdraw_count(), 0);

    for agent in [
      ScriptedAgent::cautious_v1(),
      ScriptedAgent::risk_taking_v1(),
      ScriptedAgent::yielding_v1(),
    ] {
      validate_lane_request(
        &initial,
        &safe_receipt,
        &agent.choose(safe_receipt.observation()).request(),
      )
      .expect("safe tally request is legal");
      validate_lane_request(
        &threat_state,
        &threat_receipt,
        &agent.choose(threat_receipt.observation()).request(),
      )
      .expect("threat tally request is legal");
    }

    let mixed_observer = LanerObservation {
      observer: ALLIED_AUTONOMOUS_ACTOR,
      ..safe_receipt.observation()
    };
    assert_eq!(
      ScriptedAgentActionTallyReport::from_observations([
        safe_receipt.observation(),
        mixed_observer,
      ]),
      Err(ScriptedAgentActionTallyError::MismatchedObserver)
    );
    assert_eq!(
      ScriptedAgentActionTallyReport::from_observations([
        safe_receipt.observation(),
        observe_player(&threat_state, ObservationId::new(14)).observation(),
      ]),
      Err(ScriptedAgentActionTallyError::DuplicateObservationId)
    );
  }

  #[test]
  fn visible_threat_changes_only_the_cautious_profile_selection() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let safe_receipt = observe_player(&initial, ObservationId::new(14));
    let threat_receipt = observe_player(&threat_state, ObservationId::new(14));

    let cautious = ScriptedAgent::cautious_v1();
    let risk_taking = ScriptedAgent::risk_taking_v1();
    let yielding = ScriptedAgent::yielding_v1();
    let cautious_safe = cautious.choose(safe_receipt.observation());
    let cautious_threat = cautious.choose(threat_receipt.observation());
    let risk_safe = risk_taking.choose(safe_receipt.observation());
    let risk_threat = risk_taking.choose(threat_receipt.observation());
    let yielding_safe = yielding.choose(safe_receipt.observation());
    let yielding_threat = yielding.choose(threat_receipt.observation());

    assert_eq!(cautious_safe.selected_intent(), LaneIntent::Stabilize);
    assert_eq!(cautious_threat.selected_intent(), LaneIntent::Withdraw);
    assert_eq!(risk_safe.selected_intent(), LaneIntent::Contest);
    assert_eq!(risk_threat.selected_intent(), LaneIntent::Contest);
    assert_eq!(yielding_safe.selected_intent(), LaneIntent::Yield);
    assert_eq!(yielding_threat.selected_intent(), LaneIntent::Yield);
    assert!(cautious_threat.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
    }));
    assert!(risk_threat.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
    }));
    assert!(yielding_threat.candidates().iter().any(|candidate| {
      candidate.intent() == LaneIntent::Withdraw
        && candidate.reason() == ScriptedAgentReason::ThreatResponse
    }));
    validate_lane_request(&initial, &safe_receipt, &cautious_safe.request())
      .expect("cautious safe request is legal");
    validate_lane_request(&threat_state, &threat_receipt, &cautious_threat.request())
      .expect("cautious threat request is legal");
    validate_lane_request(&initial, &safe_receipt, &risk_safe.request())
      .expect("risk safe request is legal");
    validate_lane_request(&threat_state, &threat_receipt, &risk_threat.request())
      .expect("risk threat request is legal");
    validate_lane_request(&initial, &safe_receipt, &yielding_safe.request())
      .expect("yielding safe request is legal");
    validate_lane_request(&threat_state, &threat_receipt, &yielding_threat.request())
      .expect("yielding threat request is legal");
  }

  #[test]
  fn semantic_profile_dimensions_round_trip_and_reject_invalid() {
    for (val, label) in [
      (SemanticRiskTolerance::Cautious, "cautious"),
      (SemanticRiskTolerance::Balanced, "balanced"),
      (SemanticRiskTolerance::RiskSeeking, "risk-seeking"),
    ] {
      assert_eq!(val.as_str(), label);
      assert_eq!(SemanticRiskTolerance::parse(label), Some(val));
    }
    assert_eq!(SemanticRiskTolerance::parse("unknown"), None);

    for (val, label) in [
      (SemanticDeference::Autonomous, "autonomous"),
      (SemanticDeference::Compliant, "compliant"),
      (SemanticDeference::Yielding, "yielding"),
    ] {
      assert_eq!(val.as_str(), label);
      assert_eq!(SemanticDeference::parse(label), Some(val));
    }
    assert_eq!(SemanticDeference::parse("unknown"), None);

    for (val, label) in [
      (SemanticFocus::Patience, "patience"),
      (SemanticFocus::Opportunity, "opportunity"),
      (SemanticFocus::Urgency, "urgency"),
    ] {
      assert_eq!(val.as_str(), label);
      assert_eq!(SemanticFocus::parse(label), Some(val));
    }
    assert_eq!(SemanticFocus::parse("unknown"), None);

    for (val, label) in [
      (SemanticCommunicationClarity::Terse, "terse"),
      (SemanticCommunicationClarity::Standard, "standard"),
      (SemanticCommunicationClarity::Verbose, "verbose"),
    ] {
      assert_eq!(val.as_str(), label);
      assert_eq!(SemanticCommunicationClarity::parse(label), Some(val));
    }
    assert_eq!(SemanticCommunicationClarity::parse("unknown"), None);
  }

  #[test]
  fn semantic_profile_definitions_and_vocabulary_lookup_are_canonical() {
    let cautious = SemanticProfileDefinition::cautious_v1();
    let risk_taking = SemanticProfileDefinition::risk_taking_v1();
    let yielding = SemanticProfileDefinition::yielding_v1();

    assert_eq!(cautious.schema(), SEMANTIC_PROFILE_VOCABULARY_SCHEMA);
    assert_eq!(risk_taking.schema(), SEMANTIC_PROFILE_VOCABULARY_SCHEMA);
    assert_eq!(yielding.schema(), SEMANTIC_PROFILE_VOCABULARY_SCHEMA);

    assert_eq!(cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
    assert_eq!(risk_taking.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
    assert_eq!(yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);

    assert_eq!(cautious.risk_tolerance(), SemanticRiskTolerance::Cautious);
    assert_eq!(cautious.deference(), SemanticDeference::Autonomous);
    assert_eq!(cautious.focus(), SemanticFocus::Patience);
    assert_eq!(
      cautious.communication_clarity(),
      SemanticCommunicationClarity::Terse
    );
    assert!(!cautious.description().is_empty());

    assert_eq!(
      risk_taking.risk_tolerance(),
      SemanticRiskTolerance::RiskSeeking
    );
    assert_eq!(risk_taking.deference(), SemanticDeference::Autonomous);
    assert_eq!(risk_taking.focus(), SemanticFocus::Opportunity);
    assert_eq!(
      risk_taking.communication_clarity(),
      SemanticCommunicationClarity::Standard
    );
    assert!(!risk_taking.description().is_empty());

    assert_eq!(yielding.risk_tolerance(), SemanticRiskTolerance::Cautious);
    assert_eq!(yielding.deference(), SemanticDeference::Yielding);
    assert_eq!(yielding.focus(), SemanticFocus::Patience);
    assert_eq!(
      yielding.communication_clarity(),
      SemanticCommunicationClarity::Terse
    );
    assert!(!yielding.description().is_empty());

    let all = SemanticProfileVocabulary::all_profiles();
    assert_eq!(all.len(), 3);
    assert_eq!(all[0], cautious);
    assert_eq!(all[1], risk_taking);
    assert_eq!(all[2], yielding);

    assert_eq!(
      SemanticProfileVocabulary::lookup(CAUTIOUS_SEMANTIC_PROFILE_ID),
      Some(cautious)
    );
    assert_eq!(
      SemanticProfileVocabulary::lookup(RISK_TAKING_SEMANTIC_PROFILE_ID),
      Some(risk_taking)
    );
    assert_eq!(
      SemanticProfileVocabulary::lookup(YIELDING_SEMANTIC_PROFILE_ID),
      Some(yielding)
    );
    assert_eq!(SemanticProfileVocabulary::lookup("unknown-profile"), None);

    assert_eq!(
      SemanticProfileVocabulary::validate_profile_id(CAUTIOUS_SEMANTIC_PROFILE_ID),
      Ok(cautious)
    );
    assert_eq!(
      SemanticProfileVocabulary::validate_profile_id(RISK_TAKING_SEMANTIC_PROFILE_ID),
      Ok(risk_taking)
    );
    assert_eq!(
      SemanticProfileVocabulary::validate_profile_id(YIELDING_SEMANTIC_PROFILE_ID),
      Ok(yielding)
    );
    assert_eq!(
      SemanticProfileVocabulary::validate_profile_id("unknown-profile"),
      Err(SemanticProfileVocabularyError::UnknownProfile)
    );
  }

  #[test]
  fn diagnostic_choice_domains_and_catalog_are_canonical() {
    for (domain, label) in [
      (DiagnosticChoiceDomain::ContestConcede, "contest-concede"),
      (DiagnosticChoiceDomain::FollowReject, "follow-reject"),
      (DiagnosticChoiceDomain::FarmAssist, "farm-assist"),
      (DiagnosticChoiceDomain::RecallTiming, "recall-timing"),
      (DiagnosticChoiceDomain::Sacrifice, "sacrifice"),
      (DiagnosticChoiceDomain::Surprise, "surprise"),
      (
        DiagnosticChoiceDomain::ResponseToFailure,
        "response-to-failure",
      ),
    ] {
      assert_eq!(domain.as_str(), label);
      assert_eq!(DiagnosticChoiceDomain::parse(label), Some(domain));
    }
    assert_eq!(DiagnosticChoiceDomain::parse("unknown"), None);

    let cc = DiagnosticChoiceDefinition::contest_concede_v1();
    let fr = DiagnosticChoiceDefinition::follow_reject_v1();
    let fa = DiagnosticChoiceDefinition::farm_assist_v1();
    let rt = DiagnosticChoiceDefinition::recall_timing_v1();
    let sc = DiagnosticChoiceDefinition::sacrifice_v1();
    let sp = DiagnosticChoiceDefinition::surprise_v1();
    let rf = DiagnosticChoiceDefinition::response_to_failure_v1();

    for choice in [cc, fr, fa, rt, sc, sp, rf] {
      assert_eq!(choice.schema(), DIAGNOSTIC_CHOICE_CATALOG_SCHEMA);
      assert!(!choice.choice_id().is_empty());
      assert!(!choice.intended_contrast().is_empty());
      assert!(!choice.description().is_empty());
      assert_ne!(choice.primary_intent(), choice.alternative_intent());
    }

    assert_eq!(cc.choice_id(), CHOICE_CONTEST_CONCEDE_ID);
    assert_eq!(cc.domain(), DiagnosticChoiceDomain::ContestConcede);
    assert_eq!(cc.primary_intent(), LaneIntent::Contest);
    assert_eq!(cc.alternative_intent(), LaneIntent::Yield);

    assert_eq!(fr.choice_id(), CHOICE_FOLLOW_REJECT_ID);
    assert_eq!(fr.domain(), DiagnosticChoiceDomain::FollowReject);
    assert_eq!(fr.primary_intent(), LaneIntent::Contest);
    assert_eq!(fr.alternative_intent(), LaneIntent::Stabilize);

    assert_eq!(fa.choice_id(), CHOICE_FARM_ASSIST_ID);
    assert_eq!(fa.domain(), DiagnosticChoiceDomain::FarmAssist);
    assert_eq!(fa.primary_intent(), LaneIntent::Stabilize);
    assert_eq!(fa.alternative_intent(), LaneIntent::Contest);

    assert_eq!(rt.choice_id(), CHOICE_RECALL_TIMING_ID);
    assert_eq!(rt.domain(), DiagnosticChoiceDomain::RecallTiming);
    assert_eq!(rt.primary_intent(), LaneIntent::Recall);
    assert_eq!(rt.alternative_intent(), LaneIntent::Stabilize);

    assert_eq!(sc.choice_id(), CHOICE_SACRIFICE_ID);
    assert_eq!(sc.domain(), DiagnosticChoiceDomain::Sacrifice);
    assert_eq!(sc.primary_intent(), LaneIntent::Contest);
    assert_eq!(sc.alternative_intent(), LaneIntent::Withdraw);

    assert_eq!(sp.choice_id(), CHOICE_SURPRISE_ID);
    assert_eq!(sp.domain(), DiagnosticChoiceDomain::Surprise);
    assert_eq!(sp.primary_intent(), LaneIntent::Withdraw);
    assert_eq!(sp.alternative_intent(), LaneIntent::Stabilize);

    assert_eq!(rf.choice_id(), CHOICE_RESPONSE_TO_FAILURE_ID);
    assert_eq!(rf.domain(), DiagnosticChoiceDomain::ResponseToFailure);
    assert_eq!(rf.primary_intent(), LaneIntent::Yield);
    assert_eq!(rf.alternative_intent(), LaneIntent::Contest);

    let all = DiagnosticChoiceCatalog::all_choices();
    assert_eq!(all.len(), 7);
    assert_eq!(all[0], cc);
    assert_eq!(all[1], fr);
    assert_eq!(all[2], fa);
    assert_eq!(all[3], rt);
    assert_eq!(all[4], sc);
    assert_eq!(all[5], sp);
    assert_eq!(all[6], rf);

    for choice in [cc, fr, fa, rt, sc, sp, rf] {
      assert_eq!(
        DiagnosticChoiceCatalog::lookup(choice.choice_id()),
        Some(choice)
      );
      assert_eq!(
        DiagnosticChoiceCatalog::validate_choice_id(choice.choice_id()),
        Ok(choice)
      );
      assert_eq!(
        DiagnosticChoiceCatalog::choice_for_domain(choice.domain()),
        choice
      );
    }

    assert_eq!(DiagnosticChoiceCatalog::lookup("unknown-choice"), None);
    assert_eq!(
      DiagnosticChoiceCatalog::validate_choice_id("unknown-choice"),
      Err(DiagnosticChoiceCatalogError::UnknownChoice)
    );
  }

  #[test]
  fn m7_model_prompt_and_repeated_sampling_protocols_are_bounded_and_fail_closed() {
    let std_prompt = ModelPromptProtocolDefinition::reference_standard_v1();
    let diag_prompt = ModelPromptProtocolDefinition::reference_diagnostic_v1();
    let alt_prompt = ModelPromptProtocolDefinition::alternative_diagnostic_v1();

    assert_eq!(std_prompt.schema(), MODEL_PROMPT_PROTOCOL_SCHEMA);
    assert_eq!(std_prompt.protocol_id(), MODEL_PROMPT_REFERENCE_STANDARD_ID);
    assert_eq!(std_prompt.model_family_id(), "model-family-reference-v1");
    assert_eq!(
      std_prompt.prompt_template_id(),
      "prompt-template-lane-standard-v1"
    );
    assert_eq!(
      std_prompt.system_prompt_version(),
      "sysprompt-actor-contract-v1"
    );
    assert_eq!(std_prompt.temperature_centiperc(), 70);
    assert_eq!(std_prompt.top_p_centiperc(), 95);
    assert!(std_prompt.requires_structured_output());
    assert!(!std_prompt.chain_of_thought_required());
    assert_eq!(std_prompt.validate(), Ok(()));

    assert_eq!(diag_prompt.schema(), MODEL_PROMPT_PROTOCOL_SCHEMA);
    assert_eq!(
      diag_prompt.protocol_id(),
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
    );
    assert_eq!(diag_prompt.model_family_id(), "model-family-reference-v1");
    assert_eq!(
      diag_prompt.prompt_template_id(),
      "prompt-template-lane-diagnostic-v1"
    );
    assert_eq!(diag_prompt.temperature_centiperc(), 50);
    assert_eq!(diag_prompt.top_p_centiperc(), 90);
    assert!(diag_prompt.requires_structured_output());
    assert!(!diag_prompt.chain_of_thought_required());
    assert_eq!(diag_prompt.validate(), Ok(()));

    assert_eq!(alt_prompt.schema(), MODEL_PROMPT_PROTOCOL_SCHEMA);
    assert_eq!(
      alt_prompt.protocol_id(),
      MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
    );
    assert_eq!(alt_prompt.model_family_id(), "model-family-alternative-v1");
    assert_eq!(
      alt_prompt.prompt_template_id(),
      "prompt-template-lane-diagnostic-v1"
    );
    assert_eq!(alt_prompt.temperature_centiperc(), 50);
    assert_eq!(alt_prompt.top_p_centiperc(), 90);
    assert!(alt_prompt.requires_structured_output());
    assert!(!alt_prompt.chain_of_thought_required());
    assert_eq!(alt_prompt.validate(), Ok(()));

    let all_prompts = ModelPromptProtocolCatalog::all_protocols();
    assert_eq!(all_prompts.len(), 3);
    assert_eq!(all_prompts[0], std_prompt);
    assert_eq!(all_prompts[1], diag_prompt);
    assert_eq!(all_prompts[2], alt_prompt);

    for p in [std_prompt, diag_prompt, alt_prompt] {
      assert_eq!(ModelPromptProtocolCatalog::lookup(p.protocol_id()), Some(p));
      assert_eq!(
        ModelPromptProtocolCatalog::validate_protocol_id(p.protocol_id()),
        Ok(p)
      );
    }
    assert_eq!(
      ModelPromptProtocolCatalog::lookup("unknown-model-prompt"),
      None
    );
    assert_eq!(
      ModelPromptProtocolCatalog::validate_protocol_id("unknown-model-prompt"),
      Err(ModelPromptProtocolError::UnknownProtocol)
    );

    let mut invalid_temp = std_prompt;
    invalid_temp.temperature_centiperc = 201;
    assert_eq!(
      invalid_temp.validate(),
      Err(ModelPromptProtocolError::InvalidTemperature)
    );

    let mut invalid_top_p = std_prompt;
    invalid_top_p.top_p_centiperc = 101;
    assert_eq!(
      invalid_top_p.validate(),
      Err(ModelPromptProtocolError::InvalidTopP)
    );

    let mut invalid_cot = std_prompt;
    invalid_cot.chain_of_thought_required = true;
    assert_eq!(
      invalid_cot.validate(),
      Err(ModelPromptProtocolError::PrivateChainOfThoughtForbidden)
    );

    let std_samp = RepeatedSamplingProtocolDefinition::standard_repeat_10_v1();
    let diag_samp = RepeatedSamplingProtocolDefinition::diagnostic_repeat_30_v1();
    let quick_samp = RepeatedSamplingProtocolDefinition::quick_check_5_v1();

    assert_eq!(std_samp.schema(), REPEATED_SAMPLING_PROTOCOL_SCHEMA);
    assert_eq!(std_samp.protocol_id(), SAMPLING_STANDARD_REPEAT_10_ID);
    assert_eq!(std_samp.sample_count(), 10);
    assert_eq!(std_samp.seed_offset_step(), 1);
    assert_eq!(std_samp.max_repair_retries(), 3);
    assert!(std_samp.fail_closed_on_unrepaired());
    assert_eq!(std_samp.validate(), Ok(()));

    assert_eq!(diag_samp.schema(), REPEATED_SAMPLING_PROTOCOL_SCHEMA);
    assert_eq!(diag_samp.protocol_id(), SAMPLING_DIAGNOSTIC_REPEAT_30_ID);
    assert_eq!(diag_samp.sample_count(), 30);
    assert_eq!(diag_samp.seed_offset_step(), 1);
    assert_eq!(diag_samp.max_repair_retries(), 3);
    assert!(diag_samp.fail_closed_on_unrepaired());
    assert_eq!(diag_samp.validate(), Ok(()));

    assert_eq!(quick_samp.schema(), REPEATED_SAMPLING_PROTOCOL_SCHEMA);
    assert_eq!(quick_samp.protocol_id(), SAMPLING_QUICK_CHECK_5_ID);
    assert_eq!(quick_samp.sample_count(), 5);
    assert_eq!(quick_samp.seed_offset_step(), 1);
    assert_eq!(quick_samp.max_repair_retries(), 2);
    assert!(quick_samp.fail_closed_on_unrepaired());
    assert_eq!(quick_samp.validate(), Ok(()));

    let all_samps = RepeatedSamplingProtocolCatalog::all_protocols();
    assert_eq!(all_samps.len(), 3);
    assert_eq!(all_samps[0], std_samp);
    assert_eq!(all_samps[1], diag_samp);
    assert_eq!(all_samps[2], quick_samp);

    for s in [std_samp, diag_samp, quick_samp] {
      assert_eq!(
        RepeatedSamplingProtocolCatalog::lookup(s.protocol_id()),
        Some(s)
      );
      assert_eq!(
        RepeatedSamplingProtocolCatalog::validate_protocol_id(s.protocol_id()),
        Ok(s)
      );
    }
    assert_eq!(
      RepeatedSamplingProtocolCatalog::lookup("unknown-sampling-protocol"),
      None
    );
    assert_eq!(
      RepeatedSamplingProtocolCatalog::validate_protocol_id("unknown-sampling-protocol"),
      Err(RepeatedSamplingProtocolError::UnknownProtocol)
    );

    let mut zero_samples = std_samp;
    zero_samples.sample_count = 0;
    assert_eq!(
      zero_samples.validate(),
      Err(RepeatedSamplingProtocolError::InvalidSampleCount)
    );

    let mut excessive_samples = std_samp;
    excessive_samples.sample_count = 101;
    assert_eq!(
      excessive_samples.validate(),
      Err(RepeatedSamplingProtocolError::InvalidSampleCount)
    );

    let mut zero_step = std_samp;
    zero_step.seed_offset_step = 0;
    assert_eq!(
      zero_step.validate(),
      Err(RepeatedSamplingProtocolError::InvalidSeedOffsetStep)
    );

    let mut excessive_retries = std_samp;
    excessive_retries.max_repair_retries = 11;
    assert_eq!(
      excessive_retries.validate(),
      Err(RepeatedSamplingProtocolError::InvalidMaxRetries)
    );
  }

  #[test]
  fn m7_empirical_action_and_communication_distribution_estimates_are_bounded_and_exact() {
    let cautious_profile = CAUTIOUS_SEMANTIC_PROFILE_ID;
    let cc_choice = CHOICE_CONTEST_CONCEDE_ID;

    let valid_action =
      DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 10, 2, 8, 0)
        .expect("valid action distribution");

    assert_eq!(valid_action.schema(), EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA);
    assert_eq!(valid_action.choice_id(), cc_choice);
    assert_eq!(valid_action.profile_id(), cautious_profile);
    assert_eq!(valid_action.primary_intent(), LaneIntent::Contest);
    assert_eq!(valid_action.alternative_intent(), LaneIntent::Yield);
    assert_eq!(valid_action.sample_count(), 10);
    assert_eq!(valid_action.primary_count(), 2);
    assert_eq!(valid_action.alternative_count(), 8);
    assert_eq!(valid_action.other_count(), 0);
    assert_eq!(valid_action.basis_points(), [2_000, 8_000, 0]);
    assert_eq!(valid_action.primary_share_basis_points(), 2_000);
    assert_eq!(valid_action.alternative_share_basis_points(), 8_000);
    assert_eq!(valid_action.other_share_basis_points(), 0);
    assert_eq!(
      valid_action.basis_points().iter().sum::<u16>(),
      EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
    );
    assert!(valid_action.to_markdown().contains(cc_choice));

    // Remainder handling in basis points
    let odd_action =
      DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 7, 2, 4, 1)
        .expect("valid odd sample distribution");
    let bp = odd_action.basis_points();
    assert_eq!(bp[0], 2857); // 2 * 10000 / 7
    assert_eq!(bp[1], 5714); // 4 * 10000 / 7
    assert_eq!(bp[2], 1429); // 10000 - (2857 + 5714) = 1429
    assert_eq!(
      bp.iter().sum::<u16>(),
      EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
    );

    // Validation errors
    assert_eq!(
      DiagnosticChoiceActionDistribution::new("unknown-choice", cautious_profile, 10, 2, 8, 0,),
      Err(EmpiricalDistributionEstimationError::UnknownChoice)
    );
    assert_eq!(
      DiagnosticChoiceActionDistribution::new(cc_choice, "unknown-profile", 10, 2, 8, 0,),
      Err(EmpiricalDistributionEstimationError::UnknownProfile)
    );
    assert_eq!(
      DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 0, 0, 0, 0,),
      Err(EmpiricalDistributionEstimationError::InvalidSampleCount)
    );
    assert_eq!(
      DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 101, 50, 50, 1,),
      Err(EmpiricalDistributionEstimationError::InvalidSampleCount)
    );
    assert_eq!(
      DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 10, 2, 7, 0,),
      Err(EmpiricalDistributionEstimationError::CountSumMismatch)
    );

    // Communication distribution
    let valid_comm = DiagnosticChoiceCommunicationDistribution::new(
      cc_choice,
      cautious_profile,
      10,
      [8, 1, 1, 0, 0],
    )
    .expect("valid comm distribution");
    assert_eq!(
      valid_comm.schema(),
      EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA
    );
    assert_eq!(valid_comm.choice_id(), cc_choice);
    assert_eq!(valid_comm.profile_id(), cautious_profile);
    assert_eq!(valid_comm.sample_count(), 10);
    assert_eq!(valid_comm.signal_counts(), [8, 1, 1, 0, 0]);
    assert_eq!(valid_comm.basis_points(), [8_000, 1_000, 1_000, 0, 0]);
    assert_eq!(
      valid_comm.signal_share_basis_points(LanePingSignal::None),
      8_000
    );
    assert_eq!(
      valid_comm.signal_share_basis_points(LanePingSignal::Danger),
      1_000
    );
    assert_eq!(
      valid_comm.signal_share_basis_points(LanePingSignal::OnMyWay),
      1_000
    );
    assert_eq!(
      valid_comm.signal_share_basis_points(LanePingSignal::Assist),
      0
    );
    assert_eq!(
      valid_comm.signal_share_basis_points(LanePingSignal::EnemyMissing),
      0
    );
    assert_eq!(
      valid_comm.basis_points().iter().sum::<u16>(),
      EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
    );
    assert!(valid_comm.to_markdown().contains(cc_choice));

    // Communication validation errors
    assert_eq!(
      DiagnosticChoiceCommunicationDistribution::new(
        "unknown-choice",
        cautious_profile,
        10,
        [10, 0, 0, 0, 0],
      ),
      Err(EmpiricalDistributionEstimationError::UnknownChoice)
    );
    assert_eq!(
      DiagnosticChoiceCommunicationDistribution::new(
        cc_choice,
        "unknown-profile",
        10,
        [10, 0, 0, 0, 0],
      ),
      Err(EmpiricalDistributionEstimationError::UnknownProfile)
    );
    assert_eq!(
      DiagnosticChoiceCommunicationDistribution::new(
        cc_choice,
        cautious_profile,
        0,
        [0, 0, 0, 0, 0],
      ),
      Err(EmpiricalDistributionEstimationError::InvalidSampleCount)
    );
    assert_eq!(
      DiagnosticChoiceCommunicationDistribution::new(
        cc_choice,
        cautious_profile,
        10,
        [9, 0, 0, 0, 0],
      ),
      Err(EmpiricalDistributionEstimationError::CountSumMismatch)
    );

    // Canonical baseline reports
    let cautious_rep = EmpiricalDistributionEstimateReport::cautious_v1();
    let risk_rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
    let yielding_rep = EmpiricalDistributionEstimateReport::yielding_v1();

    for rep in [&cautious_rep, &risk_rep, &yielding_rep] {
      assert_eq!(rep.schema(), EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA);
      assert_eq!(rep.validate(), Ok(()));
      assert_eq!(rep.action_distributions().len(), 7);
      assert_eq!(rep.communication_distributions().len(), 7);

      for action_dist in rep.action_distributions() {
        assert_eq!(
          action_dist.basis_points().iter().sum::<u16>(),
          EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
        );
      }
      for comm_dist in rep.communication_distributions() {
        assert_eq!(
          comm_dist.basis_points().iter().sum::<u16>(),
          EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
        );
      }
      let md = rep.to_markdown();
      assert!(md.contains("# Empirical Distribution Estimate Report"));
      assert!(md.contains("## Action Distributions"));
      assert!(md.contains("## Communication Distributions"));
    }

    assert_eq!(cautious_rep.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
    assert_eq!(risk_rep.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
    assert_eq!(yielding_rep.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);

    // Verify report validation failure on mismatched profile inside action dist
    let mut bad_rep = cautious_rep.clone();
    bad_rep.action_distributions[0] = DiagnosticChoiceActionDistribution::new(
      CHOICE_CONTEST_CONCEDE_ID,
      RISK_TAKING_SEMANTIC_PROFILE_ID,
      10,
      9,
      1,
      0,
    )
    .expect("valid dist");
    assert_eq!(
      bad_rep.validate(),
      Err(EmpiricalDistributionEstimationError::MismatchedProfile)
    );

    // Verify report validation failure on mismatched choice order
    let mut unordered_rep = cautious_rep.clone();
    unordered_rep.action_distributions[0] = cautious_rep.action_distributions[1];
    assert_eq!(
      unordered_rep.validate(),
      Err(EmpiricalDistributionEstimationError::MismatchedChoice)
    );
  }

  #[test]
  fn behavioral_measures_evaluate_distance_entropy_sensitivity_consistency_and_adaptation() {
    let cautious_rep = EmpiricalDistributionEstimateReport::cautious_v1();
    let risk_rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
    let yielding_rep = EmpiricalDistributionEstimateReport::yielding_v1();

    // 1. Behavioral Distance (TVD)
    let dist_cautious_cautious = BehavioralDistanceMeasure::action_tvd(
      cautious_rep.action_distributions()[0],
      cautious_rep.action_distributions()[0],
    );
    assert_eq!(dist_cautious_cautious, 0);

    let dist_cautious_risk = BehavioralDistanceMeasure::action_tvd(
      cautious_rep.action_distributions()[0],
      risk_rep.action_distributions()[0],
    );
    let dist_risk_cautious = BehavioralDistanceMeasure::action_tvd(
      risk_rep.action_distributions()[0],
      cautious_rep.action_distributions()[0],
    );
    assert_eq!(dist_cautious_risk, dist_risk_cautious);
    // Cautious contest-concede is [2000, 8000, 0]; Risk is [9000, 1000, 0].
    // diff = |2000-9000| + |8000-1000| + |0-0| = 7000 + 7000 = 14000. TVD = 7000 bp.
    assert_eq!(dist_cautious_risk, 7000);

    // Triangle inequality on contest-concede: TVD(A, C) <= TVD(A, B) + TVD(B, C)
    let dist_cautious_yielding = BehavioralDistanceMeasure::action_tvd(
      cautious_rep.action_distributions()[0],
      yielding_rep.action_distributions()[0],
    );
    let dist_risk_yielding = BehavioralDistanceMeasure::action_tvd(
      risk_rep.action_distributions()[0],
      yielding_rep.action_distributions()[0],
    );
    assert!(dist_cautious_risk <= dist_cautious_yielding + dist_risk_yielding);

    // Distance Report
    let dist_rep = BehavioralDistanceReport::from_reports(&cautious_rep, &risk_rep);
    assert_eq!(dist_rep.schema(), BEHAVIORAL_DISTANCE_SCHEMA);
    assert_eq!(dist_rep.baseline_profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
    assert_eq!(
      dist_rep.candidate_profile_id(),
      RISK_TAKING_SEMANTIC_PROFILE_ID
    );
    assert_eq!(dist_rep.action_choice_distances().len(), 7);
    assert_eq!(dist_rep.communication_choice_distances().len(), 7);
    assert!(dist_rep.mean_action_distance_bp() > 0);
    let dist_md = dist_rep.to_markdown();
    assert!(dist_md.contains("# Behavioral Distance Report"));
    assert!(dist_md.contains("contest-concede"));

    // 2. Behavioral Entropy (Gini diversity index)
    // Deterministic distribution ([10, 0, 0]) in Surprise for Cautious -> entropy == 0
    let surprise_cautious = cautious_rep.action_distributions()[5];
    assert_eq!(
      BehavioralEntropyMeasure::action_entropy(surprise_cautious),
      0
    );

    // Mixed distribution in ContestConcede for Cautious ([2, 8, 0] -> [2000, 8000, 0])
    // sum_sq = 2000^2 + 8000^2 = 4_000_000 + 64_000_000 = 68_000_000. conc = 6800.
    // entropy = 10000 - 6800 = 3200 bp.
    let contest_cautious = cautious_rep.action_distributions()[0];
    assert_eq!(
      BehavioralEntropyMeasure::action_entropy(contest_cautious),
      3200
    );

    let mean_action_entropy = BehavioralEntropyMeasure::mean_action_entropy(&cautious_rep);
    assert!(mean_action_entropy > 0);
    assert!(mean_action_entropy < 5000);

    // 3. Behavioral Sensitivity
    // Cautious: ContestConcede primary_bp = 2000; Surprise primary_bp (Withdraw) = 10000.
    // |2000 - 10000| = 8000 bp sensitivity.
    assert_eq!(
      BehavioralSensitivityMeasure::surprise_sensitivity(&cautious_rep),
      8000
    );

    // 4. Behavioral Consistency
    // In Surprise, Cautious is [10, 0, 0] -> consistency is 10,000 bp (100% modal adherence).
    assert_eq!(
      BehavioralConsistencyMeasure::action_consistency(surprise_cautious),
      10000
    );
    let mean_consistency = BehavioralConsistencyMeasure::mean_action_consistency(&cautious_rep);
    assert!(mean_consistency >= 8000); // High modal adherence for baseline

    // 5. Behavioral Adaptation
    assert_eq!(
      BehavioralAdaptationMeasure::surprise_adaptation_bp(&cautious_rep),
      10000
    );
    assert_eq!(
      BehavioralAdaptationMeasure::failure_adaptation_bp(&cautious_rep),
      9000
    );
    assert_eq!(
      BehavioralAdaptationMeasure::composite_adaptation_bp(&cautious_rep),
      9500
    );

    // RiskTaking should have low defensive adaptation
    assert_eq!(
      BehavioralAdaptationMeasure::surprise_adaptation_bp(&risk_rep),
      2000
    );
    assert_eq!(
      BehavioralAdaptationMeasure::failure_adaptation_bp(&risk_rep),
      1000
    );
    assert_eq!(
      BehavioralAdaptationMeasure::composite_adaptation_bp(&risk_rep),
      1500
    );

    // 6. Unified Behavioral Measures Report
    let measures_cautious = BehavioralMeasuresReport::from_report(&cautious_rep);
    assert_eq!(measures_cautious.schema(), BEHAVIORAL_MEASURES_SCHEMA);
    assert_eq!(measures_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
    assert_eq!(measures_cautious.surprise_sensitivity_bp(), 8000);
    assert_eq!(measures_cautious.composite_adaptation_bp(), 9500);

    let md = measures_cautious.to_markdown();
    assert!(md.contains("# Behavioral Measures Report"));
    assert!(md.contains("composite_adaptation_bp: 9500"));
  }
}
