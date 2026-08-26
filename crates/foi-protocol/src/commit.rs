//! Actor commit commands and result DTOs.

use super::codec::{ActorProtocolCodecError, parse_fields};
use super::intents::ActorProtocolIntent;
use crate::lane::LaneIntent;

/// Versioned actor commit command identity.
pub const ACTOR_COMMIT_SCHEMA: &str = "m5-actor-commit-v1";

/// Versioned actor commit acknowledgement identity.
pub const ACTOR_COMMIT_RESULT_SCHEMA: &str = "m5-actor-commit-result-v1";

/// Observation-bound actor command that commits one explicit intent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorCommitDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  intent: ActorProtocolIntent,
}

impl ActorCommitDto {
  pub const fn new(observer: u8, observation_id: u64, intent: ActorProtocolIntent) -> Self {
    Self {
      schema: ACTOR_COMMIT_SCHEMA,
      observer,
      observation_id,
      intent,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn observer(self) -> u8 {
    self.observer
  }

  pub const fn observation_id(self) -> u64 {
    self.observation_id
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  pub fn to_lane_intent(self) -> LaneIntent {
    self.intent.to_lane_intent()
  }

  /// Encode the observation-bound commit command as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nintent={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.intent.id()
    )
  }

  /// Decode a bounded commit command without staging or advancing the host.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 4)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut intent = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "intent" => &mut intent,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_COMMIT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}

/// Bounded actor-safe acknowledgement after a host-owned commit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorCommitResultDto {
  schema: &'static str,
  intent: ActorProtocolIntent,
}

impl ActorCommitResultDto {
  pub const fn new(intent: ActorProtocolIntent) -> Self {
    Self {
      schema: ACTOR_COMMIT_RESULT_SCHEMA,
      intent,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  /// Encode the bounded commit acknowledgement as stable line-oriented text.
  pub fn encode(self) -> String {
    format!("schema={}\nintent={}\n", self.schema, self.intent.id())
  }

  /// Decode a bounded commit acknowledgement without transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 2)?;
    let mut schema = None;
    let mut intent = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "intent" => &mut intent,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_COMMIT_RESULT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(ActorProtocolIntent::parse_id(
      intent.ok_or(ActorProtocolCodecError::MissingField)?,
    )?))
  }
}
