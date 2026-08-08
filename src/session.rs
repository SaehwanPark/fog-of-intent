//! Immutable actor-session lifecycle at the M5 protocol edge.
//!
//! This module binds one ordinary actor to one session and one current
//! observation at a time. It checks session freshness and duplicate submission
//! only; host legality, transition, history, and replay remain outside it.

use crate::protocol::{
  ActorActionDto, ActorObservationDto, ActorProtocolError, ActorProtocolErrorCode,
  ActorProtocolRepairHint,
};
use std::fmt;

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

/// Versioned two-actor simultaneous-submission contract.
pub const ACTOR_SIMULTANEOUS_WINDOW_SCHEMA: &str = "m5-actor-simultaneous-window-v1";

/// Read-only lifecycle phase for a simultaneous actor window.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorSimultaneousPhase {
  AwaitingActions,
  Ready,
  Closed,
}

impl ActorSimultaneousPhase {
  pub const fn id(self) -> &'static str {
    match self {
      Self::AwaitingActions => "awaiting_actions",
      Self::Ready => "ready",
      Self::Closed => "closed",
    }
  }
}

/// Construction failures for a simultaneous window.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorSimultaneousConstructionError {
  SameActor,
}

impl ActorSimultaneousConstructionError {
  pub const fn to_actor_error(self) -> ActorProtocolError {
    match self {
      Self::SameActor => ActorProtocolError::new(
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ),
    }
  }
}

/// Bounded errors for private two-actor action collection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorSimultaneousError {
  ActorMismatch,
  StaleObservation,
  DuplicateSubmission,
  Closed,
}

impl ActorSimultaneousError {
  pub const fn to_actor_error(self) -> ActorProtocolError {
    let (code, repair) = match self {
      Self::ActorMismatch => (
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
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

/// Immutable two-actor window that keeps submitted intents private.
///
/// The public surface exposes bounded binding metadata, lifecycle, and
/// readiness only. The collected intents remain internal until a later
/// host-owned resolution contract.
#[derive(Clone, Copy)]
pub struct ActorSimultaneousWindow {
  schema: &'static str,
  first_actor: u8,
  second_actor: u8,
  observation_id: u64,
  first_intent: Option<crate::protocol::ActorProtocolIntent>,
  second_intent: Option<crate::protocol::ActorProtocolIntent>,
  phase: ActorSimultaneousPhase,
}

impl fmt::Debug for ActorSimultaneousWindow {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("ActorSimultaneousWindow")
      .field("schema", &self.schema)
      .field("first_actor", &self.first_actor)
      .field("second_actor", &self.second_actor)
      .field("observation_id", &self.observation_id)
      .field("phase", &self.phase)
      .finish()
  }
}

impl ActorSimultaneousWindow {
  pub fn new(
    first_actor: u8,
    second_actor: u8,
    observation_id: u64,
  ) -> Result<Self, ActorSimultaneousConstructionError> {
    if first_actor == second_actor {
      return Err(ActorSimultaneousConstructionError::SameActor);
    }
    Ok(Self {
      schema: ACTOR_SIMULTANEOUS_WINDOW_SCHEMA,
      first_actor,
      second_actor,
      observation_id,
      first_intent: None,
      second_intent: None,
      phase: ActorSimultaneousPhase::AwaitingActions,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn first_actor(self) -> u8 {
    self.first_actor
  }

  pub const fn second_actor(self) -> u8 {
    self.second_actor
  }

  pub const fn observation_id(self) -> u64 {
    self.observation_id
  }

  pub const fn phase(self) -> ActorSimultaneousPhase {
    self.phase
  }

  pub const fn is_ready(self) -> bool {
    matches!(self.phase, ActorSimultaneousPhase::Ready)
  }

  /// Collect one observer-bound action without exposing either intent.
  pub fn submit(self, action: ActorActionDto) -> Result<Self, ActorSimultaneousError> {
    if self.phase == ActorSimultaneousPhase::Closed {
      return Err(ActorSimultaneousError::Closed);
    }
    if action.observer() != self.first_actor && action.observer() != self.second_actor {
      return Err(ActorSimultaneousError::ActorMismatch);
    }
    if action.observation_id() != self.observation_id {
      return Err(ActorSimultaneousError::StaleObservation);
    }
    let (first_intent, second_intent) = if action.observer() == self.first_actor {
      if self.first_intent.is_some() {
        return Err(ActorSimultaneousError::DuplicateSubmission);
      }
      (Some(action.intent()), self.second_intent)
    } else if action.observer() == self.second_actor {
      if self.second_intent.is_some() {
        return Err(ActorSimultaneousError::DuplicateSubmission);
      }
      (self.first_intent, Some(action.intent()))
    } else {
      return Err(ActorSimultaneousError::ActorMismatch);
    };
    Ok(Self {
      first_intent,
      second_intent,
      phase: if first_intent.is_some() && second_intent.is_some() {
        ActorSimultaneousPhase::Ready
      } else {
        ActorSimultaneousPhase::AwaitingActions
      },
      ..self
    })
  }

  /// Close the window; later submissions fail without exposing collected data.
  pub const fn close(self) -> Self {
    Self {
      phase: ActorSimultaneousPhase::Closed,
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
  fn simultaneous_window_waits_for_both_actions_without_exposing_intents() {
    let window = ActorSimultaneousWindow::new(1, 2, 40).expect("distinct actors");
    assert_eq!(window.schema(), "m5-actor-simultaneous-window-v1");
    assert_eq!(window.phase(), ActorSimultaneousPhase::AwaitingActions);
    assert!(!window.is_ready());

    let first = window
      .submit(ActorActionDto::new(1, 40, ActorProtocolIntent::Contest))
      .expect("first action binds");
    assert_eq!(first.phase(), ActorSimultaneousPhase::AwaitingActions);
    assert!(!first.is_ready());
    let first_debug = format!("{first:?}");
    assert!(!first_debug.contains("Contest") && !first_debug.contains("contest"));

    let ready = first
      .submit(ActorActionDto::new(2, 40, ActorProtocolIntent::Yield))
      .expect("second action binds");
    assert_eq!(ready.phase(), ActorSimultaneousPhase::Ready);
    assert!(ready.is_ready());
    let ready_debug = format!("{ready:?}");
    assert!(!ready_debug.contains("Contest") && !ready_debug.contains("Yield"));
  }

  #[test]
  fn simultaneous_window_rejects_stale_cross_actor_and_duplicate_without_mutation() {
    let window = ActorSimultaneousWindow::new(1, 2, 41).expect("distinct actors");
    assert!(matches!(
      window.submit(ActorActionDto::new(1, 40, ActorProtocolIntent::Contest)),
      Err(ActorSimultaneousError::StaleObservation)
    ));
    assert!(matches!(
      window.submit(ActorActionDto::new(3, 40, ActorProtocolIntent::Contest)),
      Err(ActorSimultaneousError::ActorMismatch)
    ));
    let first = window
      .submit(ActorActionDto::new(1, 41, ActorProtocolIntent::Contest))
      .expect("first action binds");
    assert!(matches!(
      first.submit(ActorActionDto::new(1, 41, ActorProtocolIntent::Yield)),
      Err(ActorSimultaneousError::DuplicateSubmission)
    ));
    assert_eq!(first.phase(), ActorSimultaneousPhase::AwaitingActions);
    assert!(!first.is_ready());
  }

  #[test]
  fn simultaneous_window_rejects_same_actor_and_closes_fail_closed() {
    assert!(matches!(
      ActorSimultaneousWindow::new(4, 4, 42),
      Err(ActorSimultaneousConstructionError::SameActor)
    ));
    let window = ActorSimultaneousWindow::new(4, 5, 42).expect("distinct actors");
    let closed = window.close();
    assert_eq!(closed.phase(), ActorSimultaneousPhase::Closed);
    assert!(matches!(
      closed.submit(ActorActionDto::new(4, 42, ActorProtocolIntent::Contest)),
      Err(ActorSimultaneousError::Closed)
    ));
  }

  #[test]
  fn simultaneous_errors_project_to_bounded_actor_repairs() {
    let cases = [
      (
        ActorSimultaneousConstructionError::SameActor.to_actor_error(),
        "actor_mismatch",
        "use_bound_actor",
      ),
      (
        ActorSimultaneousError::ActorMismatch.to_actor_error(),
        "actor_mismatch",
        "use_bound_actor",
      ),
      (
        ActorSimultaneousError::StaleObservation.to_actor_error(),
        "stale_observation",
        "request_fresh_observation",
      ),
      (
        ActorSimultaneousError::DuplicateSubmission.to_actor_error(),
        "duplicate_submission",
        "await_next_observation",
      ),
      (
        ActorSimultaneousError::Closed.to_actor_error(),
        "closed_session",
        "start_new_session",
      ),
    ];
    for (error, code, repair) in cases {
      assert_eq!(error.schema(), "m5-actor-error-v2");
      assert_eq!(error.code().id(), code);
      assert_eq!(error.repair().id(), repair);
      let debug = format!("{error:?}");
      assert!(!debug.contains("actor=") && !debug.contains("observation_id="));
    }
  }

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
