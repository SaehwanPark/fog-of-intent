//! Actor observation DTO and line-oriented parser/serializer.

use super::codec::{ActorProtocolCodecError, parse_fields};
use super::intents::ActorProtocolIntent;
use crate::lane::LanerObservation;
use std::fmt::Write as _;

/// Versioned observation DTO identity.
pub const ACTOR_OBSERVATION_SCHEMA: &str = "m5-actor-observation-v1";

/// Actor-visible observation DTO for the bounded intent-only protocol slice.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActorObservationDto {
  schema: &'static str,
  observer: u8,
  turn: u32,
  observation_id: u64,
  available_actions: Vec<ActorProtocolIntent>,
  visible_threat_response: Option<ActorProtocolIntent>,
}

impl ActorObservationDto {
  /// Project an actor-visible lane observation without exposing domain state.
  pub fn from_observation(observation: LanerObservation) -> Self {
    let visible_threat_response = observation
      .available_threat_response()
      .map(ActorProtocolIntent::from_lane_intent);
    let mut available_actions = Vec::with_capacity(5);
    for intent in observation.available_intents() {
      available_actions.push(ActorProtocolIntent::from_lane_intent(intent));
    }
    if let Some(threat_response) = visible_threat_response
      && !available_actions.contains(&threat_response)
    {
      available_actions.push(threat_response);
    }
    Self {
      schema: ACTOR_OBSERVATION_SCHEMA,
      observer: observation.observer().value(),
      turn: observation.turn().value(),
      observation_id: observation.observation_id().value(),
      available_actions,
      visible_threat_response,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> u8 {
    self.observer
  }

  pub const fn turn(&self) -> u32 {
    self.turn
  }

  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  pub fn available_actions(&self) -> &[ActorProtocolIntent] {
    &self.available_actions
  }

  pub const fn visible_threat_response(&self) -> Option<ActorProtocolIntent> {
    self.visible_threat_response
  }

  pub fn advertises(&self, intent: ActorProtocolIntent) -> bool {
    self.available_actions.contains(&intent)
  }

  /// Encode the bounded observation DTO as stable line-oriented text.
  pub fn encode(&self) -> String {
    let mut output = String::new();
    output.push_str("schema=");
    output.push_str(self.schema);
    output.push('\n');
    output.push_str("observer=");
    write!(output, "{}", self.observer).expect("writing to String cannot fail");
    output.push('\n');
    output.push_str("turn=");
    write!(output, "{}", self.turn).expect("writing to String cannot fail");
    output.push('\n');
    output.push_str("observation_id=");
    write!(output, "{}", self.observation_id).expect("writing to String cannot fail");
    output.push('\n');
    output.push_str("actions=");
    for (index, intent) in self.available_actions.iter().enumerate() {
      if index > 0 {
        output.push(',');
      }
      output.push_str(intent.id());
    }
    output.push('\n');
    output.push_str("threat=");
    output.push_str(
      self
        .visible_threat_response
        .map_or("unknown", ActorProtocolIntent::id),
    );
    output.push('\n');
    output
  }

  /// Decode a bounded line-oriented observation DTO.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 6)?;
    let mut schema = None;
    let mut observer = None;
    let mut turn = None;
    let mut observation_id = None;
    let mut actions = None;
    let mut threat = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "turn" => &mut turn,
        "observation_id" => &mut observation_id,
        "actions" => &mut actions,
        "threat" => &mut threat,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_OBSERVATION_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let observer = observer
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let turn = turn
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u32>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let observation_id = observation_id
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let actions = actions.ok_or(ActorProtocolCodecError::MissingField)?;
    let mut available_actions = Vec::with_capacity(5);
    for raw_intent in actions.split(',') {
      let intent = ActorProtocolIntent::parse_id(raw_intent)?;
      if available_actions.contains(&intent) || available_actions.len() == 5 {
        return Err(ActorProtocolCodecError::InvalidValue);
      }
      available_actions.push(intent);
    }
    if !(4..=5).contains(&available_actions.len()) {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    let base_actions = [
      ActorProtocolIntent::Stabilize,
      ActorProtocolIntent::Contest,
      ActorProtocolIntent::Yield,
      ActorProtocolIntent::Recall,
    ];
    if available_actions.get(..4) != Some(base_actions.as_slice()) {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    let visible_threat_response = match threat.ok_or(ActorProtocolCodecError::MissingField)? {
      "unknown" => None,
      value => Some(ActorProtocolIntent::parse_id(value)?),
    };
    if visible_threat_response.is_some_and(|intent| {
      intent != ActorProtocolIntent::Withdraw || !available_actions.contains(&intent)
    }) || (visible_threat_response.is_none() && available_actions.len() == 5)
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_OBSERVATION_SCHEMA,
      observer,
      turn,
      observation_id,
      available_actions,
      visible_threat_response,
    })
  }
}
