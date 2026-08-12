//! Actor action request and result DTOs.

use super::codec::{ActorProtocolCodecError, parse_fields};
use super::intents::ActorProtocolIntent;
use crate::kernel::ActorId;
use crate::lane::{LaneIntentRequest, LaneOutcome, ObservationId};

/// Versioned intent-action DTO identity.
pub const ACTOR_ACTION_SCHEMA: &str = "m5-actor-action-v1";

/// Versioned actor-safe action-result identity.
pub const ACTOR_ACTION_RESULT_SCHEMA: &str = "m5-actor-action-result-v1";

/// Bounded actor action DTO carrying only an observer-bound intent request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorActionDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  intent: ActorProtocolIntent,
}

impl ActorActionDto {
  pub const fn new(observer: u8, observation_id: u64, intent: ActorProtocolIntent) -> Self {
    Self {
      schema: ACTOR_ACTION_SCHEMA,
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

  /// Convert to the host-bound request; legality remains a host concern.
  pub(crate) fn to_lane_request(self) -> LaneIntentRequest {
    LaneIntentRequest::new(
      ActorId::new(self.observer),
      ObservationId::new(self.observation_id),
      self.intent.to_lane_intent(),
    )
  }

  /// Encode the bounded action DTO as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nintent={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.intent.id()
    )
  }

  /// Decode a bounded line-oriented action DTO.
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
    if schema != Some(ACTOR_ACTION_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self {
      schema: ACTOR_ACTION_SCHEMA,
      observer: observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id: observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      intent: ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
    })
  }
}

/// Closed fixture window labels in an actor action result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorActionResultWindow {
  First,
  Second,
}

impl ActorActionResultWindow {
  pub const fn id(self) -> &'static str {
    match self {
      Self::First => "first",
      Self::Second => "second",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "first" => Ok(Self::First),
      "second" => Ok(Self::Second),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Closed categorical outcomes in an actor action result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorActionResultOutcome {
  HeldSpace,
  YieldedSpace,
  ForcedOut,
}

impl ActorActionResultOutcome {
  pub const fn id(self) -> &'static str {
    match self {
      Self::HeldSpace => "held_space",
      Self::YieldedSpace => "yielded_space",
      Self::ForcedOut => "forced_out",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "held_space" => Ok(Self::HeldSpace),
      "yielded_space" => Ok(Self::YieldedSpace),
      "forced_out" => Ok(Self::ForcedOut),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }

  pub(crate) const fn from_lane_outcome(outcome: LaneOutcome) -> Self {
    match outcome {
      LaneOutcome::HeldSpace => Self::HeldSpace,
      LaneOutcome::YieldedSpace => Self::YieldedSpace,
      LaneOutcome::ForcedOut => Self::ForcedOut,
    }
  }
}

/// Bounded actor-safe result returned after a successful actor action.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorActionResultDto {
  schema: &'static str,
  window: ActorActionResultWindow,
  outcome: ActorActionResultOutcome,
}

impl ActorActionResultDto {
  pub const fn new(window: ActorActionResultWindow, outcome: ActorActionResultOutcome) -> Self {
    Self {
      schema: ACTOR_ACTION_RESULT_SCHEMA,
      window,
      outcome,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn window(self) -> ActorActionResultWindow {
    self.window
  }

  pub const fn outcome(self) -> ActorActionResultOutcome {
    self.outcome
  }

  /// Encode the bounded action result as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nwindow={}\noutcome={}\n",
      self.schema,
      self.window.id(),
      self.outcome.id()
    )
  }

  /// Decode a bounded action result without transition or history authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut window = None;
    let mut outcome = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "window" => &mut window,
        "outcome" => &mut outcome,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_ACTION_RESULT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      ActorActionResultWindow::parse_id(window.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorActionResultOutcome::parse_id(outcome.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}
