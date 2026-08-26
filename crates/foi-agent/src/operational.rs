//! Non-authoritative operational logs, events, sequence reports, and causal trace completeness.

use super::replay::{ScriptedAgentReplayError, ScriptedAgentReplayRecord};
use crate::lane::ObservationId;

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

/// Maximum number of replay records evaluated in one scenario-wide identity check.
pub const MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS: usize = 16;

/// Maximum number of operational events retained in one in-memory log.
pub const MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS: usize = 16;

/// Maximum encoded size of one operational log.
pub const MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES: usize = 4096;

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

  pub(crate) fn parse_id(value: &str) -> Option<Self> {
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

/// Ordered, non-authoritative operational metadata kept outside history.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentOperationalLog {
  schema: &'static str,
  entries: Vec<ScriptedAgentOperationalEventRecord>,
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
