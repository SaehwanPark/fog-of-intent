//! Versioned actor-protocol DTOs at the M5 adapter boundary.
//!
//! The DTOs contain only bounded actor-visible observation and intent data.
//! They do not validate legality, resolve execution, mutate history, or
//! depend on a transport, async runtime, or provider SDK.

use crate::kernel::ActorId;
use crate::lane::{LaneIntent, LaneIntentRequest, LanerObservation, ObservationId};
use std::fmt::Write as _;

/// Versioned actor-protocol vocabulary for this bounded slice.
pub const ACTOR_PROTOCOL_SCHEMA: &str = "m5-actor-protocol-v1";

/// Versioned observation DTO identity.
pub const ACTOR_OBSERVATION_SCHEMA: &str = "m5-actor-observation-v1";

/// Versioned intent-action DTO identity.
pub const ACTOR_ACTION_SCHEMA: &str = "m5-actor-action-v1";

/// Versioned line-oriented codec identity for the bounded DTOs.
pub const ACTOR_PROTOCOL_CODEC_SCHEMA: &str = "m5-actor-codec-v1";

/// Maximum encoded DTO size accepted by the bounded parser.
pub const MAX_ACTOR_PROTOCOL_BYTES: usize = 4096;

/// Closed intent vocabulary exposed by the actor protocol.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolIntent {
  Stabilize,
  Contest,
  Yield,
  Recall,
  Withdraw,
}

impl ActorProtocolIntent {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Stabilize => "stabilize",
      Self::Contest => "contest",
      Self::Yield => "yield",
      Self::Recall => "recall",
      Self::Withdraw => "withdraw",
    }
  }

  const fn from_lane_intent(intent: LaneIntent) -> Self {
    match intent {
      LaneIntent::Stabilize => Self::Stabilize,
      LaneIntent::Contest => Self::Contest,
      LaneIntent::Yield => Self::Yield,
      LaneIntent::Recall => Self::Recall,
      LaneIntent::Withdraw => Self::Withdraw,
    }
  }

  const fn to_lane_intent(self) -> LaneIntent {
    match self {
      Self::Stabilize => LaneIntent::Stabilize,
      Self::Contest => LaneIntent::Contest,
      Self::Yield => LaneIntent::Yield,
      Self::Recall => LaneIntent::Recall,
      Self::Withdraw => LaneIntent::Withdraw,
    }
  }
}

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
  pub fn to_lane_request(self) -> LaneIntentRequest {
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

/// Bounded protocol codec failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
}

impl ActorProtocolIntent {
  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "stabilize" => Ok(Self::Stabilize),
      "contest" => Ok(Self::Contest),
      "yield" => Ok(Self::Yield),
      "recall" => Ok(Self::Recall),
      "withdraw" => Ok(Self::Withdraw),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

fn parse_fields(
  input: &str,
  expected_lines: usize,
) -> Result<Vec<(&str, &str)>, ActorProtocolCodecError> {
  if input.len() > MAX_ACTOR_PROTOCOL_BYTES {
    return Err(ActorProtocolCodecError::Oversized);
  }
  let actual_lines = input.lines().count();
  if actual_lines > expected_lines {
    return Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: expected_lines,
      actual: actual_lines,
    });
  }
  let mut fields = Vec::with_capacity(expected_lines);
  for line in input.lines() {
    let (key, value) = line
      .split_once('=')
      .ok_or(ActorProtocolCodecError::InvalidValue)?;
    if key.is_empty() || value.is_empty() {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    fields.push((key, value));
  }
  if fields.len() < expected_lines {
    return Ok(fields);
  }
  Ok(fields)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lane::{
    JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus, ObservationId, observe_player,
    validate_lane_request,
  };

  #[test]
  fn observation_dto_is_versioned_bounded_and_actor_visible() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(23)).observation();
    let dto = ActorObservationDto::from_observation(observation);

    assert_eq!(ACTOR_PROTOCOL_SCHEMA, "m5-actor-protocol-v1");
    assert_eq!(dto.schema(), "m5-actor-observation-v1");
    assert_eq!(dto.observer(), observation.observer().value());
    assert_eq!(dto.turn(), observation.turn().value());
    assert_eq!(dto.observation_id(), 23);
    assert_eq!(dto.available_actions().len(), 4);
    assert_eq!(
      dto.available_actions(),
      &[
        ActorProtocolIntent::Stabilize,
        ActorProtocolIntent::Contest,
        ActorProtocolIntent::Yield,
        ActorProtocolIntent::Recall,
      ]
    );
    assert!(dto.advertises(ActorProtocolIntent::Contest));
    assert!(!dto.advertises(ActorProtocolIntent::Withdraw));
    assert_eq!(dto.visible_threat_response(), None);
    assert!(!format!("{dto:?}").contains("StateHash"));
    assert!(!format!("{dto:?}").contains("LaneSnapshot"));
  }

  #[test]
  fn visible_threat_is_projected_as_one_additional_action() {
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
    let observation = observe_player(&threat_state, ObservationId::new(24)).observation();
    let dto = ActorObservationDto::from_observation(observation);

    assert_eq!(dto.available_actions().len(), 5);
    assert_eq!(
      dto.visible_threat_response(),
      Some(ActorProtocolIntent::Withdraw)
    );
    assert_eq!(
      dto.available_actions().last(),
      Some(&ActorProtocolIntent::Withdraw)
    );
    assert_eq!(
      ActorObservationDto::decode(&dto.encode()).expect("threat observation decodes"),
      dto
    );
  }

  #[test]
  fn action_dto_round_trips_to_host_validated_intent_request() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(25));
    let dto = ActorActionDto::new(1, 25, ActorProtocolIntent::Contest);
    let request = dto.to_lane_request();

    assert_eq!(dto.schema(), "m5-actor-action-v1");
    assert_eq!(dto.intent().id(), "contest");
    assert_eq!(request.actor(), receipt.observation().observer());
    assert_eq!(
      request.observation_id(),
      receipt.observation().observation_id()
    );
    assert_eq!(request.intent(), LaneIntent::Contest);
    validate_lane_request(&state, &receipt, &request).expect("protocol request is host-valid");
  }

  #[test]
  fn protocol_intent_ids_are_closed_and_stable() {
    assert_eq!(ActorProtocolIntent::Stabilize.id(), "stabilize");
    assert_eq!(ActorProtocolIntent::Contest.id(), "contest");
    assert_eq!(ActorProtocolIntent::Yield.id(), "yield");
    assert_eq!(ActorProtocolIntent::Recall.id(), "recall");
    assert_eq!(ActorProtocolIntent::Withdraw.id(), "withdraw");
  }

  #[test]
  fn protocol_dtos_round_trip_through_bounded_codec() {
    let state = LaneSnapshot::initial();
    let observation = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(32)).observation(),
    );
    let action = ActorActionDto::new(1, 32, ActorProtocolIntent::Contest);

    assert_eq!(
      ActorObservationDto::decode(&observation.encode()).expect("observation decodes"),
      observation
    );
    assert_eq!(
      ActorActionDto::decode(&action.encode()).expect("action decodes"),
      action
    );
    assert_eq!(ACTOR_PROTOCOL_CODEC_SCHEMA, "m5-actor-codec-v1");
  }

  #[test]
  fn protocol_codec_rejects_unknown_duplicate_missing_and_invalid_fields() {
    let observation = "schema=m5-actor-observation-v1\nobserver=1\nturn=0\nobservation_id=33\nactions=stabilize,contest,yield,recall\nthreat=unknown\n";
    assert_eq!(
      ActorObservationDto::decode(&observation.replace("turn=0", "extra=x")),
      Err(ActorProtocolCodecError::UnknownField)
    );
    assert_eq!(
      ActorActionDto::decode("schema=m5-actor-action-v1\nobserver=1\nobserver=1\nintent=contest\n"),
      Err(ActorProtocolCodecError::DuplicateField)
    );
    assert_eq!(
      ActorActionDto::decode("schema=m5-actor-action-v1\nobserver=1\nintent=contest\n"),
      Err(ActorProtocolCodecError::MissingField)
    );
    assert_eq!(
      ActorActionDto::decode(
        "schema=m5-actor-action-v1\nobserver=1\nobservation_id=33\nintent=unknown\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorObservationDto::decode(
        "schema=m5-actor-observation-v1\nobserver=1\nturn=0\nobservation_id=33\nactions=stabilize,contest,yield,recall,withdraw\nthreat=contest\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorObservationDto::decode(
        "schema=m5-actor-observation-v1\nobserver=1\nturn=0\nobservation_id=33\nactions=stabilize,contest,yield,withdraw\nthreat=unknown\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
  }

  #[test]
  fn protocol_codec_rejects_oversized_and_extra_lines_before_projection() {
    let oversized = "x".repeat(MAX_ACTOR_PROTOCOL_BYTES + 1);
    assert_eq!(
      ActorActionDto::decode(&oversized),
      Err(ActorProtocolCodecError::Oversized)
    );
    let extra =
      "schema=m5-actor-action-v1\nobserver=1\nobservation_id=34\nintent=contest\nextra=x\nmore=y\n";
    assert_eq!(
      ActorActionDto::decode(extra),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 4,
        actual: 6
      })
    );
  }

  #[test]
  fn decoded_action_still_requires_host_validation() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(35));
    let encoded = ActorActionDto::new(1, 35, ActorProtocolIntent::Contest).encode();
    let action = ActorActionDto::decode(&encoded).expect("action decodes");

    validate_lane_request(&state, &receipt, &action.to_lane_request())
      .expect("decoded action is accepted by host validator");
  }
}
