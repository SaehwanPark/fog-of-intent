//! Closed intent vocabulary for the actor protocol.

use super::codec::ActorProtocolCodecError;
use crate::lane::LaneIntent;

/// Versioned actor-protocol vocabulary for this bounded slice.
pub const ACTOR_PROTOCOL_SCHEMA: &str = "m5-actor-protocol-v1";

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

  pub(crate) const fn from_lane_intent(intent: LaneIntent) -> Self {
    match intent {
      LaneIntent::Stabilize => Self::Stabilize,
      LaneIntent::Contest => Self::Contest,
      LaneIntent::Yield => Self::Yield,
      LaneIntent::Recall => Self::Recall,
      LaneIntent::Withdraw => Self::Withdraw,
    }
  }

  pub(crate) const fn to_lane_intent(self) -> LaneIntent {
    match self {
      Self::Stabilize => LaneIntent::Stabilize,
      Self::Contest => LaneIntent::Contest,
      Self::Yield => LaneIntent::Yield,
      Self::Recall => LaneIntent::Recall,
      Self::Withdraw => LaneIntent::Withdraw,
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
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
