//! Loopback transport contracts and presentation session adapter for M11 GUI.
//!
//! All transport operations operate strictly on actor-visible Data Transfer Objects
//! and presentation actions. Zero simulation authority, zero hidden-state inference,
//! and zero latent state leakage are strictly enforced.

use core::fmt;

use crate::gui::dto::{GUI_DTO_SCHEMA_VERSION, GuiActiveTab, GuiPresentationBundle, GuiViewMode};
use crate::gui::html::{
  GuiHtmlVerificationReport, render_gui_html_document, verify_gui_html_document,
};
use crate::gui::state::{GuiClientError, GuiClientEvent, GuiClientState, GuiPresentationAction};

/// Schema version for GUI loopback transport contracts.
pub const GUI_TRANSPORT_SCHEMA_VERSION: &str = "m11-gui-transport-v1";

/// Maximum allowed character length for transport identifiers.
pub const MAX_IDENTIFIER_LEN: usize = 64;

/// Lifecycle phase of an active GUI presentation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiSessionPhase {
  /// Session is active and accepting presentation interactions.
  Active,
  /// Session is awaiting actor intent submission.
  AwaitingIntent,
  /// Actor intent has been submitted for the current window.
  IntentSubmitted,
  /// Session has been cleanly terminated.
  Closed,
}

impl GuiSessionPhase {
  /// Canonical string identifier for the phase.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Active => "active",
      Self::AwaitingIntent => "awaiting-intent",
      Self::IntentSubmitted => "intent-submitted",
      Self::Closed => "closed",
    }
  }

  /// Parse session phase from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "active" => Some(Self::Active),
      "awaiting-intent" => Some(Self::AwaitingIntent),
      "intent-submitted" => Some(Self::IntentSubmitted),
      "closed" => Some(Self::Closed),
      _ => None,
    }
  }
}

impl fmt::Display for GuiSessionPhase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Reason for closing an active GUI presentation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiSessionCloseReason {
  /// Client explicitly requested session closure.
  ClientRequested,
  /// Session reached idle timeout.
  TimedOut,
  /// Transport connection was severed.
  Disconnected,
  /// Fatal protocol or invariant error occurred.
  FatalError,
}

impl GuiSessionCloseReason {
  /// Canonical string identifier for the close reason.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ClientRequested => "client-requested",
      Self::TimedOut => "timed-out",
      Self::Disconnected => "disconnected",
      Self::FatalError => "fatal-error",
    }
  }

  /// Parse close reason from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "client-requested" => Some(Self::ClientRequested),
      "timed-out" => Some(Self::TimedOut),
      "disconnected" => Some(Self::Disconnected),
      "fatal-error" => Some(Self::FatalError),
      _ => None,
    }
  }
}

impl fmt::Display for GuiSessionCloseReason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Categorical error codes for GUI transport failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiTransportErrorCode {
  ActorMismatch,
  SessionClosed,
  InvalidPayload,
  UnknownEntity,
  InvariantViolation,
  StaleTurn,
  UnsupportedAction,
}

impl GuiTransportErrorCode {
  /// Canonical string identifier for the error code.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ActorMismatch => "actor-mismatch",
      Self::SessionClosed => "session-closed",
      Self::InvalidPayload => "invalid-payload",
      Self::UnknownEntity => "unknown-entity",
      Self::InvariantViolation => "invariant-violation",
      Self::StaleTurn => "stale-turn",
      Self::UnsupportedAction => "unsupported-action",
    }
  }
}

impl fmt::Display for GuiTransportErrorCode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Actionable repair hints for GUI transport failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiTransportRepairHint {
  UseBoundActor,
  StartNewSession,
  InspectActorVisibleEntity,
  RequestFreshBundle,
  SanitizePayload,
  CheckActionParameters,
}

impl GuiTransportRepairHint {
  /// Canonical string identifier for the repair hint.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::UseBoundActor => "use-bound-actor",
      Self::StartNewSession => "start-new-session",
      Self::InspectActorVisibleEntity => "inspect-actor-visible-entity",
      Self::RequestFreshBundle => "request-fresh-bundle",
      Self::SanitizePayload => "sanitize-payload",
      Self::CheckActionParameters => "check-action-parameters",
    }
  }
}

impl fmt::Display for GuiTransportRepairHint {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Fail-closed domain errors for GUI transport operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiTransportError {
  ActorMismatch {
    expected: String,
    actual: String,
  },
  SessionClosed(String),
  InvalidPayload(&'static str),
  UnknownEntity(String),
  InvariantViolation(&'static str),
  StaleTurn {
    current_turn: u32,
    requested_turn: u32,
  },
  ClientStateError(String),
  HtmlGenerationError(String),
}

impl fmt::Display for GuiTransportError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ActorMismatch { expected, actual } => {
        write!(
          f,
          "actor mismatch: expected '{expected}', received '{actual}'"
        )
      }
      Self::SessionClosed(msg) => write!(f, "session closed: {msg}"),
      Self::InvalidPayload(msg) => write!(f, "invalid transport payload: {msg}"),
      Self::UnknownEntity(id) => write!(f, "unknown entity: {id}"),
      Self::InvariantViolation(msg) => write!(f, "transport invariant violation: {msg}"),
      Self::StaleTurn {
        current_turn,
        requested_turn,
      } => write!(
        f,
        "stale turn requested: current turn {current_turn}, requested turn {requested_turn}"
      ),
      Self::ClientStateError(msg) => write!(f, "client state error: {msg}"),
      Self::HtmlGenerationError(msg) => write!(f, "html generation error: {msg}"),
    }
  }
}

impl std::error::Error for GuiTransportError {}

impl GuiTransportError {
  /// Get the categorical error code.
  pub fn error_code(&self) -> GuiTransportErrorCode {
    match self {
      Self::ActorMismatch { .. } => GuiTransportErrorCode::ActorMismatch,
      Self::SessionClosed(_) => GuiTransportErrorCode::SessionClosed,
      Self::InvalidPayload(_) => GuiTransportErrorCode::InvalidPayload,
      Self::UnknownEntity(_) => GuiTransportErrorCode::UnknownEntity,
      Self::InvariantViolation(_) => GuiTransportErrorCode::InvariantViolation,
      Self::StaleTurn { .. } => GuiTransportErrorCode::StaleTurn,
      Self::ClientStateError(_) | Self::HtmlGenerationError(_) => {
        GuiTransportErrorCode::UnsupportedAction
      }
    }
  }

  /// Get the actionable repair hint.
  pub fn repair_hint(&self) -> GuiTransportRepairHint {
    match self {
      Self::ActorMismatch { .. } => GuiTransportRepairHint::UseBoundActor,
      Self::SessionClosed(_) => GuiTransportRepairHint::StartNewSession,
      Self::InvalidPayload(_) => GuiTransportRepairHint::SanitizePayload,
      Self::UnknownEntity(_) => GuiTransportRepairHint::InspectActorVisibleEntity,
      Self::InvariantViolation(_) => GuiTransportRepairHint::SanitizePayload,
      Self::StaleTurn { .. } => GuiTransportRepairHint::RequestFreshBundle,
      Self::ClientStateError(_) | Self::HtmlGenerationError(_) => {
        GuiTransportRepairHint::CheckActionParameters
      }
    }
  }
}

/// Client-to-host request message over loopback transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiClientRequest {
  /// Request actor-visible presentation bundle for current active tab and view mode.
  FetchBundle {
    actor_role: String,
    tab: GuiActiveTab,
    view_mode: GuiViewMode,
  },
  /// Dispatch a presentation inspection action.
  InspectEntity {
    actor_role: String,
    action: GuiPresentationAction,
  },
  /// Submit an actor-selected intent through presentation layer.
  SubmitIntent {
    actor_role: String,
    intent_id: String,
    commitment: String,
    target_focus: String,
  },
  /// Request a standalone verified HTML5 presentation document.
  FetchHtmlDocument {
    actor_role: String,
    tab: GuiActiveTab,
    view_mode: GuiViewMode,
  },
  /// Reset client inspection state to neutral defaults.
  ResetClientState { actor_role: String },
  /// Loopback connection health check ping.
  Ping { nonce: u64 },
  /// Explicitly close presentation session.
  CloseSession {
    actor_role: String,
    reason: GuiSessionCloseReason,
  },
}

impl GuiClientRequest {
  /// Extract target actor role if present.
  pub fn actor_role(&self) -> Option<&str> {
    match self {
      Self::FetchBundle { actor_role, .. }
      | Self::InspectEntity { actor_role, .. }
      | Self::SubmitIntent { actor_role, .. }
      | Self::FetchHtmlDocument { actor_role, .. }
      | Self::ResetClientState { actor_role }
      | Self::CloseSession { actor_role, .. } => Some(actor_role.as_str()),
      Self::Ping { .. } => None,
    }
  }
}

/// Host-to-client response message over loopback transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiHostResponse {
  /// Presentation bundle response containing actor-visible DTOs.
  BundleResponse {
    session_id: String,
    turn: u32,
    actor_role: String,
    bundle: Box<GuiPresentationBundle>,
    client_state: GuiClientState,
  },
  /// Rendered and verified standalone HTML5 presentation document.
  HtmlResponse {
    session_id: String,
    turn: u32,
    actor_role: String,
    html_document: String,
    verification_report: GuiHtmlVerificationReport,
  },
  /// Presentation action acknowledged with updated client state and event.
  ActionAcknowledged {
    session_id: String,
    actor_role: String,
    client_state: GuiClientState,
    event: GuiClientEvent,
  },
  /// Intent submission acknowledged by presentation layer.
  IntentSubmitted {
    session_id: String,
    turn: u32,
    actor_role: String,
    intent_id: String,
    validated: bool,
  },
  /// Client inspection state reset confirmed.
  ClientStateReset {
    session_id: String,
    actor_role: String,
    client_state: GuiClientState,
  },
  /// Loopback health check pong.
  Pong {
    nonce: u64,
    host_timestamp_tick: u64,
  },
  /// Session closure confirmation.
  SessionClosed {
    session_id: String,
    reason: GuiSessionCloseReason,
  },
  /// Fail-closed error response with error code and actionable repair hint.
  ErrorResponse {
    error_code: GuiTransportErrorCode,
    message: String,
    repair_hint: GuiTransportRepairHint,
  },
}

/// Presentation session managing loopback interaction with the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiPresentationSession {
  pub session_id: String,
  pub bound_actor: String,
  pub current_turn: u32,
  pub phase: GuiSessionPhase,
  pub client_state: GuiClientState,
}

impl GuiPresentationSession {
  /// Create a new active presentation session.
  pub fn new(
    session_id: impl Into<String>,
    bound_actor: impl Into<String>,
    current_turn: u32,
  ) -> Result<Self, GuiTransportError> {
    let session_id = session_id.into();
    let bound_actor = bound_actor.into();

    if session_id.trim().is_empty() {
      return Err(GuiTransportError::InvalidPayload(
        "session_id cannot be empty",
      ));
    }
    if session_id.len() > MAX_IDENTIFIER_LEN {
      return Err(GuiTransportError::InvalidPayload(
        "session_id exceeds maximum length",
      ));
    }
    if bound_actor.trim().is_empty() {
      return Err(GuiTransportError::InvalidPayload(
        "bound_actor cannot be empty",
      ));
    }

    let client_state = GuiClientState::new(bound_actor.clone());

    Ok(Self {
      session_id,
      bound_actor,
      current_turn,
      phase: GuiSessionPhase::Active,
      client_state,
    })
  }

  /// Check if the session is currently active.
  pub fn is_active(&self) -> bool {
    self.phase != GuiSessionPhase::Closed
  }

  /// Close the session with a specific reason.
  pub fn close(&mut self, reason: GuiSessionCloseReason) -> GuiHostResponse {
    self.phase = GuiSessionPhase::Closed;
    GuiHostResponse::SessionClosed {
      session_id: self.session_id.clone(),
      reason,
    }
  }

  /// Process an incoming client request against the presentation session.
  pub fn handle_request<F>(
    &mut self,
    request: GuiClientRequest,
    bundle_provider: F,
  ) -> Result<GuiHostResponse, GuiTransportError>
  where
    F: Fn(&str, GuiActiveTab, GuiViewMode) -> Result<GuiPresentationBundle, GuiTransportError>,
  {
    // Handle ping regardless of actor binding
    if let GuiClientRequest::Ping { nonce } = request {
      return Ok(GuiHostResponse::Pong {
        nonce,
        host_timestamp_tick: u64::from(self.current_turn),
      });
    }

    // Fail-closed if session is closed
    if self.phase == GuiSessionPhase::Closed {
      return Err(GuiTransportError::SessionClosed(
        "cannot process requests on a closed presentation session".to_string(),
      ));
    }

    // Validate actor role binding
    if let Some(role) = request.actor_role()
      && role != self.bound_actor
    {
      return Err(GuiTransportError::ActorMismatch {
        expected: self.bound_actor.clone(),
        actual: role.to_string(),
      });
    }

    match request {
      GuiClientRequest::FetchBundle {
        actor_role,
        tab,
        view_mode,
      } => {
        let bundle = bundle_provider(&actor_role, tab, view_mode)?;
        bundle.validate_invariants().map_err(|_| {
          GuiTransportError::InvariantViolation("presentation bundle invariant violation")
        })?;

        self.client_state.active_tab = tab;
        self.client_state.display_options.view_mode = view_mode;

        Ok(GuiHostResponse::BundleResponse {
          session_id: self.session_id.clone(),
          turn: self.current_turn,
          actor_role,
          bundle: Box::new(bundle),
          client_state: self.client_state.clone(),
        })
      }

      GuiClientRequest::InspectEntity { actor_role, action } => {
        let bundle = bundle_provider(
          &actor_role,
          self.client_state.active_tab,
          self.client_state.display_options.view_mode,
        )?;

        let event = self
          .client_state
          .transition(action, &bundle)
          .map_err(|e| match e {
            GuiClientError::UnknownLocationId(id)
            | GuiClientError::UnknownActorRole(id)
            | GuiClientError::UnknownObjectiveKind(id)
            | GuiClientError::UnknownStructureTier(id)
            | GuiClientError::UnknownQuadrant(id) => GuiTransportError::UnknownEntity(id),
            _ => GuiTransportError::ClientStateError(format!("{e}")),
          })?;

        Ok(GuiHostResponse::ActionAcknowledged {
          session_id: self.session_id.clone(),
          actor_role,
          client_state: self.client_state.clone(),
          event,
        })
      }

      GuiClientRequest::SubmitIntent {
        actor_role,
        intent_id,
        commitment,
        target_focus,
      } => {
        if intent_id.trim().is_empty() {
          return Err(GuiTransportError::InvalidPayload(
            "intent_id cannot be empty",
          ));
        }
        if commitment.trim().is_empty() {
          return Err(GuiTransportError::InvalidPayload(
            "commitment cannot be empty",
          ));
        }
        if target_focus.trim().is_empty() {
          return Err(GuiTransportError::InvalidPayload(
            "target_focus cannot be empty",
          ));
        }

        self.phase = GuiSessionPhase::IntentSubmitted;

        Ok(GuiHostResponse::IntentSubmitted {
          session_id: self.session_id.clone(),
          turn: self.current_turn,
          actor_role,
          intent_id,
          validated: true,
        })
      }

      GuiClientRequest::FetchHtmlDocument {
        actor_role,
        tab,
        view_mode,
      } => {
        let bundle = bundle_provider(&actor_role, tab, view_mode)?;
        bundle.validate_invariants().map_err(|_| {
          GuiTransportError::InvariantViolation("presentation bundle invariant violation")
        })?;

        let mut doc_state = self.client_state.clone();
        doc_state.active_tab = tab;
        doc_state.display_options.view_mode = view_mode;

        let html_document = render_gui_html_document(&bundle, &doc_state)
          .map_err(|e| GuiTransportError::HtmlGenerationError(format!("{e}")))?;

        let verification_report = verify_gui_html_document(&html_document, &bundle)
          .map_err(|e| GuiTransportError::HtmlGenerationError(format!("{e}")))?;

        if !verification_report.is_compliant {
          return Err(GuiTransportError::InvariantViolation(
            "rendered HTML document failed compliance verification",
          ));
        }

        Ok(GuiHostResponse::HtmlResponse {
          session_id: self.session_id.clone(),
          turn: self.current_turn,
          actor_role,
          html_document,
          verification_report,
        })
      }

      GuiClientRequest::ResetClientState { actor_role } => {
        self.client_state = GuiClientState::new(&self.bound_actor);
        Ok(GuiHostResponse::ClientStateReset {
          session_id: self.session_id.clone(),
          actor_role,
          client_state: self.client_state.clone(),
        })
      }

      GuiClientRequest::CloseSession {
        actor_role: _,
        reason,
      } => Ok(self.close(reason)),

      GuiClientRequest::Ping { .. } => unreachable!(),
    }
  }
}

/// Verify that a host response strictly preserves presentation invariants.
pub fn verify_transport_invariants(response: &GuiHostResponse) -> Result<(), GuiTransportError> {
  match response {
    GuiHostResponse::BundleResponse { bundle, .. } => {
      if bundle.schema_version != GUI_DTO_SCHEMA_VERSION {
        return Err(GuiTransportError::InvariantViolation(
          "bundle schema version mismatch",
        ));
      }
      bundle
        .validate_invariants()
        .map_err(|_| GuiTransportError::InvariantViolation("bundle invariant validation failed"))?;
    }
    GuiHostResponse::HtmlResponse {
      html_document,
      verification_report,
      ..
    } => {
      if !verification_report.is_compliant {
        return Err(GuiTransportError::InvariantViolation(
          "html verification report indicates non-compliance",
        ));
      }
      if html_document.contains("<thought>") || html_document.contains("chain_of_thought") {
        return Err(GuiTransportError::InvariantViolation(
          "html document contains private chain-of-thought",
        ));
      }
    }
    GuiHostResponse::ErrorResponse { message, .. }
      if message.contains("<thought>") || message.contains("chain_of_thought") =>
    {
      return Err(GuiTransportError::InvariantViolation(
        "error response contains private chain-of-thought",
      ));
    }
    _ => {}
  }
  Ok(())
}
