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

/// Versioned identity for bounded matched-scenario selected-intent tallies.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_SCHEMA: &str =
  "m6-scripted-agent-matched-scenario-tally-v1";

/// Maximum encoded matched-scenario tally size before parsing or allocation.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_BYTES: usize = 4096;

/// Maximum number of caller-supplied matched pairs in one sample set.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES: usize = 4;

/// Maximum number of selected fixed-fixture scenarios in one request.
pub const MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS: usize = 4;

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
  use crate::kernel::{DrawId, StreamId};
  use crate::lane::{
    ALLIED_AUTONOMOUS_ACTOR, JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus,
    M2_LANE_RULESET, ObservationId, WavePressure, WaveState, observe_player, validate_lane_request,
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
    store.save("resume", checkpoint).expect("checkpoint saves");
    assert_eq!(
      host_store.load("resume").expect("host artifact loads"),
      host_artifact
    );
    let loaded = store.load("resume").expect("checkpoint loads");
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
}
