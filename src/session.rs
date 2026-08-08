//! Immutable actor-session lifecycle at the M5 protocol edge.
//!
//! This module binds one ordinary actor to one session and one current
//! observation at a time. It checks session freshness and duplicate submission
//! only; host legality, transition, history, and replay remain outside it.

use crate::protocol::{
  ActorActionDto, ActorObservationDto, ActorProtocolError, ActorProtocolErrorCode,
  ActorProtocolRepairHint,
};

/// Versioned actor-session contract identity.
pub const ACTOR_SESSION_SCHEMA: &str = "m5-actor-session-v1";

/// Lifecycle phases for one actor-bound protocol session.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorSessionPhase {
  Open,
  AwaitingAction,
  Submitted,
  Closed,
}

impl ActorSessionPhase {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Open => "open",
      Self::AwaitingAction => "awaiting_action",
      Self::Submitted => "submitted",
      Self::Closed => "closed",
    }
  }
}

/// Bounded actor-session lifecycle failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorSessionError {
  ActorMismatch,
  ObservationAlreadyOpen,
  NoObservation,
  StaleObservation,
  DuplicateSubmission,
  Closed,
}

impl ActorSessionError {
  /// Project a freshness failure without exposing session or actor values.
  pub const fn to_actor_error(self) -> ActorProtocolError {
    let (code, repair) = match self {
      Self::ActorMismatch => (
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ),
      Self::ObservationAlreadyOpen => (
        ActorProtocolErrorCode::ObservationAlreadyOpen,
        ActorProtocolRepairHint::SubmitCurrentAction,
      ),
      Self::NoObservation => (
        ActorProtocolErrorCode::NoObservation,
        ActorProtocolRepairHint::RequestObservation,
      ),
      Self::StaleObservation => (
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ),
      Self::DuplicateSubmission => (
        ActorProtocolErrorCode::DuplicateSubmission,
        ActorProtocolRepairHint::AwaitNextObservation,
      ),
      Self::Closed => (
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ),
    };
    ActorProtocolError::new(code, repair)
  }
}

/// Immutable ordinary-actor session state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorSession {
  schema: &'static str,
  session_id: u64,
  actor: u8,
  phase: ActorSessionPhase,
  observation_id: Option<u64>,
}

impl ActorSession {
  pub const fn new(session_id: u64, actor: u8) -> Self {
    Self {
      schema: ACTOR_SESSION_SCHEMA,
      session_id,
      actor,
      phase: ActorSessionPhase::Open,
      observation_id: None,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn session_id(self) -> u64 {
    self.session_id
  }

  pub const fn actor(self) -> u8 {
    self.actor
  }

  pub const fn phase(self) -> ActorSessionPhase {
    self.phase
  }

  pub const fn observation_id(self) -> Option<u64> {
    self.observation_id
  }

  /// Bind the next actor-visible observation to this session.
  pub fn accept_observation(
    self,
    observation: &ActorObservationDto,
  ) -> Result<Self, ActorSessionError> {
    if self.phase == ActorSessionPhase::Closed {
      return Err(ActorSessionError::Closed);
    }
    if observation.observer() != self.actor {
      return Err(ActorSessionError::ActorMismatch);
    }
    if self.phase == ActorSessionPhase::AwaitingAction {
      return Err(ActorSessionError::ObservationAlreadyOpen);
    }
    if self.phase == ActorSessionPhase::Submitted
      && self.observation_id == Some(observation.observation_id())
    {
      return Err(ActorSessionError::StaleObservation);
    }
    Ok(Self {
      phase: ActorSessionPhase::AwaitingAction,
      observation_id: Some(observation.observation_id()),
      ..self
    })
  }

  /// Accept one observer-bound action without performing host legality checks.
  pub fn accept_action(self, action: ActorActionDto) -> Result<Self, ActorSessionError> {
    if self.phase == ActorSessionPhase::Closed {
      return Err(ActorSessionError::Closed);
    }
    if action.observer() != self.actor {
      return Err(ActorSessionError::ActorMismatch);
    }
    let current_observation = self
      .observation_id
      .ok_or(ActorSessionError::NoObservation)?;
    if action.observation_id() != current_observation {
      return Err(ActorSessionError::StaleObservation);
    }
    if self.phase == ActorSessionPhase::Submitted {
      return Err(ActorSessionError::DuplicateSubmission);
    }
    if self.phase != ActorSessionPhase::AwaitingAction {
      return Err(ActorSessionError::NoObservation);
    }
    Ok(Self {
      phase: ActorSessionPhase::Submitted,
      ..self
    })
  }

  /// Close the session; later observations and actions fail closed.
  pub const fn close(self) -> Self {
    Self {
      phase: ActorSessionPhase::Closed,
      ..self
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lane::{LaneSnapshot, ObservationId, observe_player};
  use crate::protocol::{ActorObservationDto, ActorProtocolIntent};

  #[test]
  fn session_lifecycle_is_immutable_and_allows_next_window_after_submission() {
    let state = LaneSnapshot::initial();
    let first = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(26)).observation(),
    );
    let second = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(27)).observation(),
    );
    let action = ActorActionDto::new(1, 26, ActorProtocolIntent::Stabilize);
    let next_action = ActorActionDto::new(1, 27, ActorProtocolIntent::Contest);
    let session = ActorSession::new(7, 1);

    assert_eq!(session.schema(), "m5-actor-session-v1");
    assert_eq!(session.phase().id(), "open");
    let awaiting = session
      .accept_observation(&first)
      .expect("first observation binds");
    assert_eq!(session.phase(), ActorSessionPhase::Open);
    assert_eq!(awaiting.phase(), ActorSessionPhase::AwaitingAction);
    let submitted = awaiting.accept_action(action).expect("first action binds");
    assert_eq!(submitted.phase(), ActorSessionPhase::Submitted);
    assert_eq!(
      submitted.accept_observation(&first),
      Err(ActorSessionError::StaleObservation)
    );
    let next = submitted
      .accept_observation(&second)
      .expect("next window observation binds");
    let completed = next
      .accept_action(next_action)
      .expect("next window action binds");
    assert_eq!(completed.observation_id(), Some(27));
    assert_eq!(completed.phase(), ActorSessionPhase::Submitted);
  }

  #[test]
  fn session_rejects_cross_actor_stale_and_duplicate_actions() {
    let state = LaneSnapshot::initial();
    let observation = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(28)).observation(),
    );
    let session = ActorSession::new(8, 1);
    let awaiting = session
      .accept_observation(&observation)
      .expect("observation binds");

    assert_eq!(
      awaiting.accept_action(ActorActionDto::new(2, 28, ActorProtocolIntent::Contest)),
      Err(ActorSessionError::ActorMismatch)
    );
    assert_eq!(
      awaiting.accept_action(ActorActionDto::new(1, 27, ActorProtocolIntent::Contest)),
      Err(ActorSessionError::StaleObservation)
    );
    let submitted = awaiting
      .accept_action(ActorActionDto::new(1, 28, ActorProtocolIntent::Contest))
      .expect("first action binds");
    assert_eq!(
      submitted.accept_action(ActorActionDto::new(1, 28, ActorProtocolIntent::Yield)),
      Err(ActorSessionError::DuplicateSubmission)
    );
  }

  #[test]
  fn session_rejects_actions_without_observation_and_after_close() {
    let action = ActorActionDto::new(1, 29, ActorProtocolIntent::Yield);
    let session = ActorSession::new(9, 1);

    assert_eq!(
      session.accept_action(action),
      Err(ActorSessionError::NoObservation)
    );
    let closed = session.close();
    assert_eq!(closed.phase(), ActorSessionPhase::Closed);
    assert_eq!(closed.accept_action(action), Err(ActorSessionError::Closed));
  }

  #[test]
  fn session_rejects_observation_while_waiting() {
    let state = LaneSnapshot::initial();
    let first_observation = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(30)).observation(),
    );
    let waiting_observation = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(31)).observation(),
    );
    let session = ActorSession::new(10, first_observation.observer());
    let awaiting = session
      .accept_observation(&first_observation)
      .expect("first observation binds");
    assert_eq!(
      awaiting.accept_observation(&waiting_observation),
      Err(ActorSessionError::ObservationAlreadyOpen)
    );
  }

  #[test]
  fn session_errors_project_to_bounded_repair_hints() {
    let cases = [
      (
        ActorSessionError::ActorMismatch,
        "actor_mismatch",
        "use_bound_actor",
      ),
      (
        ActorSessionError::ObservationAlreadyOpen,
        "observation_already_open",
        "submit_current_action",
      ),
      (
        ActorSessionError::NoObservation,
        "no_observation",
        "request_observation",
      ),
      (
        ActorSessionError::StaleObservation,
        "stale_observation",
        "request_fresh_observation",
      ),
      (
        ActorSessionError::DuplicateSubmission,
        "duplicate_submission",
        "await_next_observation",
      ),
      (
        ActorSessionError::Closed,
        "closed_session",
        "start_new_session",
      ),
    ];
    for (error, code, repair) in cases {
      let projected = error.to_actor_error();
      assert_eq!(projected.schema(), "m5-actor-error-v2");
      assert_eq!(projected.code().id(), code);
      assert_eq!(projected.repair().id(), repair);
      let debug = format!("{projected:?}");
      assert!(!debug.contains("actor=") && !debug.contains("observation_id="));
    }
  }
}
