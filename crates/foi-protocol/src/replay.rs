//! Bounded actor-visible replay status, record, and debrief DTOs.

use super::action::{ActorActionResultOutcome, ActorActionResultWindow};
use super::codec::{ActorProtocolCodecError, parse_fields};
use super::debrief::{ActorDebriefAttributionLimit, ActorDebriefObjective};
use super::intents::ActorProtocolIntent;

/// Versioned actor-visible replay-verification identity.
pub const ACTOR_REPLAY_SCHEMA: &str = "m5-actor-replay-v1";

/// Versioned actor-visible replay-record identity.
pub const ACTOR_REPLAY_RECORD_SCHEMA: &str = "m5-actor-replay-record-v1";

/// Versioned actor-visible replay-linked debrief-record identity.
pub const ACTOR_REPLAY_DEBRIEF_RECORD_SCHEMA: &str = "m5-actor-replay-debrief-record-v1";

/// Closed actor-visible replay-verification result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorReplayVerification {
  Verified,
}

impl ActorReplayVerification {
  pub const fn id(self) -> &'static str {
    "verified"
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "verified" => Ok(Self::Verified),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Bounded actor-visible replay status without records, hashes, or inputs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorReplayDto {
  schema: &'static str,
  records: u8,
  verification: ActorReplayVerification,
}

impl ActorReplayDto {
  pub fn new(records: u8) -> Result<Self, ActorProtocolCodecError> {
    if records > 2 {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_REPLAY_SCHEMA,
      records,
      verification: ActorReplayVerification::Verified,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn records(self) -> u8 {
    self.records
  }

  pub const fn verification(self) -> ActorReplayVerification {
    self.verification
  }

  /// Encode replay status without exposing record contents or provenance.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nrecords={}\nverification={}\n",
      self.schema,
      self.records,
      self.verification.id(),
    )
  }

  /// Decode replay status without replay or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut records = None;
    let mut verification = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "records" => &mut records,
        "verification" => &mut verification,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_REPLAY_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let records = records
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let verification = ActorReplayVerification::parse_id(
      verification.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    let dto = Self::new(records)?;
    if dto.verification != verification {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(dto)
  }
}

/// Bounded actor-visible record from verified current history.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorReplayRecordDto {
  schema: &'static str,
  window: ActorActionResultWindow,
  intent: ActorProtocolIntent,
  outcome: ActorActionResultOutcome,
  verification: ActorReplayVerification,
}

impl ActorReplayRecordDto {
  pub const fn new(
    window: ActorActionResultWindow,
    intent: ActorProtocolIntent,
    outcome: ActorActionResultOutcome,
  ) -> Self {
    Self {
      schema: ACTOR_REPLAY_RECORD_SCHEMA,
      window,
      intent,
      outcome,
      verification: ActorReplayVerification::Verified,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn window(self) -> ActorActionResultWindow {
    self.window
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  pub const fn outcome(self) -> ActorActionResultOutcome {
    self.outcome
  }

  pub const fn verification(self) -> ActorReplayVerification {
    self.verification
  }

  /// Encode one verified categorical replay record without provenance detail.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nwindow={}\nintent={}\noutcome={}\nverification={}\n",
      self.schema,
      self.window.id(),
      self.intent.id(),
      self.outcome.id(),
      self.verification.id(),
    )
  }

  /// Decode one bounded replay record without replay or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 5)?;
    let mut schema = None;
    let mut window = None;
    let mut intent = None;
    let mut outcome = None;
    let mut verification = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "window" => &mut window,
        "intent" => &mut intent,
        "outcome" => &mut outcome,
        "verification" => &mut verification,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_REPLAY_RECORD_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let verification = ActorReplayVerification::parse_id(
      verification.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    if verification != ActorReplayVerification::Verified {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_REPLAY_RECORD_SCHEMA,
      window: ActorActionResultWindow::parse_id(
        window.ok_or(ActorProtocolCodecError::MissingField)?,
      )?,
      intent: ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
      outcome: ActorActionResultOutcome::parse_id(
        outcome.ok_or(ActorProtocolCodecError::MissingField)?,
      )?,
      verification,
    })
  }
}

/// Bounded actor-visible debrief record linked to verified history.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorReplayDebriefRecordDto {
  schema: &'static str,
  window: ActorActionResultWindow,
  intent: ActorProtocolIntent,
  outcome: ActorActionResultOutcome,
  objective: ActorDebriefObjective,
  attribution: ActorDebriefAttributionLimit,
  verification: ActorReplayVerification,
}

impl ActorReplayDebriefRecordDto {
  pub const fn new(
    window: ActorActionResultWindow,
    intent: ActorProtocolIntent,
    outcome: ActorActionResultOutcome,
    objective: ActorDebriefObjective,
  ) -> Self {
    Self {
      schema: ACTOR_REPLAY_DEBRIEF_RECORD_SCHEMA,
      window,
      intent,
      outcome,
      objective,
      attribution: ActorDebriefAttributionLimit::CommittedFactsOnly,
      verification: ActorReplayVerification::Verified,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn window(self) -> ActorActionResultWindow {
    self.window
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  pub const fn outcome(self) -> ActorActionResultOutcome {
    self.outcome
  }

  pub const fn objective(self) -> ActorDebriefObjective {
    self.objective
  }

  pub const fn attribution(self) -> ActorDebriefAttributionLimit {
    self.attribution
  }

  pub const fn verification(self) -> ActorReplayVerification {
    self.verification
  }

  /// Encode one verified categorical debrief record without causal detail.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nwindow={}\nintent={}\noutcome={}\nobjective={}\nattribution={}\nverification={}\n",
      self.schema,
      self.window.id(),
      self.intent.id(),
      self.outcome.id(),
      self.objective.id(),
      self.attribution.id(),
      self.verification.id(),
    )
  }

  /// Decode one bounded replay-linked debrief record without authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 7)?;
    let mut schema = None;
    let mut window = None;
    let mut intent = None;
    let mut outcome = None;
    let mut objective = None;
    let mut attribution = None;
    let mut verification = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "window" => &mut window,
        "intent" => &mut intent,
        "outcome" => &mut outcome,
        "objective" => &mut objective,
        "attribution" => &mut attribution,
        "verification" => &mut verification,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_REPLAY_DEBRIEF_RECORD_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let attribution = ActorDebriefAttributionLimit::parse_id(
      attribution.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    let verification = ActorReplayVerification::parse_id(
      verification.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    if attribution != ActorDebriefAttributionLimit::CommittedFactsOnly
      || verification != ActorReplayVerification::Verified
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self::new(
      ActorActionResultWindow::parse_id(window.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorActionResultOutcome::parse_id(outcome.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDebriefObjective::parse_id(objective.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}
