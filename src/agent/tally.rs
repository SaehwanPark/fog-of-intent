//! Matched sample sets, selected-intent tallies, and outlier reports.

use super::experiment::{
  MAX_SCRIPTED_AGENT_BATCH_MANIFESTS, ScriptedAgentBatchError, ScriptedAgentBatchRunner,
  ScriptedAgentExperimentManifest,
};
use super::population::SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE;
use super::profile::ScriptedAgentProfile;
use super::replay::{ScriptedAgentReplayError, ScriptedAgentReplayRecord};
use crate::kernel::ActorId;
use crate::lane::{LaneIntent, LanerObservation, ObservationId};

/// Versioned identity for the bounded matched-observation sample report.
pub const SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA: &str = "m6-scripted-agent-matched-sample-v1";

/// Versioned identity for the bounded matched-scenario sample set.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLE_SCHEMA: &str =
  "m6-scripted-agent-matched-scenarios-v1";

/// Maximum number of caller-supplied matched pairs in one sample set.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES: usize = 4;

/// Versioned identity for bounded matched-scenario selected-intent tallies.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_SCHEMA: &str =
  "m6-scripted-agent-matched-scenario-tally-v1";

/// Maximum encoded matched-scenario tally size before parsing or allocation.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_BYTES: usize = 4096;

/// Versioned identity for bounded comparisons of verified profile-aware tallies.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA: &str =
  "m6-scripted-agent-matched-scenario-tally-compare-v1";

/// Maximum encoded profile-aware tally comparison size before parsing.
pub const MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_BYTES: usize = 4096;

/// Stable identity for the profile-aware fixed-fixture equality gate.
pub const SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_REGRESSION_RULE: &str =
  "m6-fixed-profile-tally-no-change-v1";

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

/// Bounded failures from matched-observation sampling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentMatchedSampleError {
  MismatchedObserver,
  DuplicateObservationId,
  Batch(ScriptedAgentBatchError),
}

/// One actor-safe profile row across two matched observations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentMatchedSampleEntry {
  profile_id: &'static str,
  evaluation_rule: &'static str,
  seed_bundle: super::profile::ScriptedAgentSeedBundle,
  selected_intents: [LaneIntent; 2],
}

impl ScriptedAgentMatchedSampleEntry {
  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn seed_bundle(self) -> super::profile::ScriptedAgentSeedBundle {
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
  pub(crate) samples: Vec<ScriptedAgentMatchedSample>,
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
  pub(crate) profile_id: &'static str,
  pub(crate) evaluation_rule: &'static str,
  pub(crate) pair_count: u8,
  pub(crate) observation_count: u8,
  pub(crate) stabilize_count: u8,
  pub(crate) contest_count: u8,
  pub(crate) yield_count: u8,
  pub(crate) recall_count: u8,
  pub(crate) withdraw_count: u8,
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
  pub(crate) profile_id: &'static str,
  pub(crate) evaluation_rule: &'static str,
  pub(crate) baseline_stabilize_count: u8,
  pub(crate) baseline_contest_count: u8,
  pub(crate) baseline_yield_count: u8,
  pub(crate) baseline_recall_count: u8,
  pub(crate) baseline_withdraw_count: u8,
  pub(crate) candidate_stabilize_count: u8,
  pub(crate) candidate_contest_count: u8,
  pub(crate) candidate_yield_count: u8,
  pub(crate) candidate_recall_count: u8,
  pub(crate) candidate_withdraw_count: u8,
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
  pub(crate) observer: ActorId,
  pub(crate) pair_count: u8,
  pub(crate) observation_count: u8,
  pub(crate) entries: Vec<ScriptedAgentMatchedScenarioTally>,
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
