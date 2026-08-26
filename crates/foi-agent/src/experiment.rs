//! Scripted agent experiment manifest, version catalog, batch runner, and checkpoints.

use super::operational::{
  MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, ScriptedAgentOperationalEvent, ScriptedAgentOperationalLog,
};
use super::policy::{ScriptedAgent, ScriptedAgentDecision};
use super::profile::{
  RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID, SCRIPTED_AGENT_PROFILE_ID, SCRIPTED_AGENT_SCHEMA,
  SCRIPTED_AGENT_SEEDED_SELECTION_RULE, ScriptedAgentProfile, ScriptedAgentSeedBundle,
  YIELDING_SCRIPTED_AGENT_PROFILE_ID,
};
use crate::kernel::{ActorId, DrawId, StreamId};
use crate::lane::{
  HiddenValue, JungleThreatRegion, LaneAbortCondition, LaneActorRole, LaneCommitment,
  LaneFallbackBehavior, LaneIntent, LanePingSignal, LanePosition, LaneTargetFocus,
  LanerObservation, ObservationId,
};
use std::hash::Hasher;

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

  pub(crate) fn parse_id(value: &str) -> Option<Self> {
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

  pub(crate) fn matches(
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

  pub fn with_completed_count(self, completed_count: usize) -> Self {
    Self {
      completed_count: u8::try_from(completed_count).expect("batch cap fits in u8"),
      ..self
    }
  }
}

/// Bounded failures from producing one complete batch lifecycle trace.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalBatchRunError {
  Batch(ScriptedAgentBatchError),
  LogCapacityExceeded { max: usize },
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

  pub(crate) fn evaluate_range(
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

pub(crate) fn validate_batch(
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
