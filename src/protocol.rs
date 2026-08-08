//! Versioned actor-protocol DTOs at the M5 adapter boundary.
//!
//! The DTOs contain only bounded actor-visible observation and intent data.
//! They do not validate legality, resolve execution, mutate history, or
//! depend on a transport, async runtime, or provider SDK.

use crate::kernel::ActorId;
use crate::lane::{LaneIntent, LaneIntentRequest, LanerObservation, ObservationId};

/// Versioned actor-protocol vocabulary for this bounded slice.
pub const ACTOR_PROTOCOL_SCHEMA: &str = "m5-actor-protocol-v1";

/// Versioned observation DTO identity.
pub const ACTOR_OBSERVATION_SCHEMA: &str = "m5-actor-observation-v1";

/// Versioned intent-action DTO identity.
pub const ACTOR_ACTION_SCHEMA: &str = "m5-actor-action-v1";

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
}
