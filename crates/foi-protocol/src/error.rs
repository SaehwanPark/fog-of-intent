//! Closed actor-facing validation error categories and deterministic repair hints.

use super::codec::{ActorProtocolCodecError, parse_fields};

/// Historical actor-facing validation-error identity from the initial closed vocabulary.
pub const ACTOR_PROTOCOL_ERROR_SCHEMA_V1: &str = "m5-actor-error-v1";

/// Current actor-facing validation-error identity after the debrief error pair was added.
pub const ACTOR_PROTOCOL_ERROR_SCHEMA: &str = "m5-actor-error-v2";

/// Closed actor-facing validation-error categories.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolErrorCode {
  OversizedInput,
  UnexpectedLineCount,
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
  ActorMismatch,
  ObservationAlreadyOpen,
  NoObservation,
  StaleObservation,
  DuplicateSubmission,
  ClosedSession,
  WindowClosed,
  HostValidationRejected,
  HostTransitionRejected,
  DraftBoundary,
  DebriefUnavailable,
}

impl ActorProtocolErrorCode {
  pub const fn id(self) -> &'static str {
    match self {
      Self::OversizedInput => "oversized_input",
      Self::UnexpectedLineCount => "unexpected_line_count",
      Self::UnknownField => "unknown_field",
      Self::DuplicateField => "duplicate_field",
      Self::MissingField => "missing_field",
      Self::UnsupportedSchema => "unsupported_schema",
      Self::InvalidValue => "invalid_value",
      Self::ActorMismatch => "actor_mismatch",
      Self::ObservationAlreadyOpen => "observation_already_open",
      Self::NoObservation => "no_observation",
      Self::StaleObservation => "stale_observation",
      Self::DuplicateSubmission => "duplicate_submission",
      Self::ClosedSession => "closed_session",
      Self::WindowClosed => "window_closed",
      Self::HostValidationRejected => "host_validation_rejected",
      Self::HostTransitionRejected => "host_transition_rejected",
      Self::DraftBoundary => "draft_boundary",
      Self::DebriefUnavailable => "debrief_unavailable",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "oversized_input" => Ok(Self::OversizedInput),
      "unexpected_line_count" => Ok(Self::UnexpectedLineCount),
      "unknown_field" => Ok(Self::UnknownField),
      "duplicate_field" => Ok(Self::DuplicateField),
      "missing_field" => Ok(Self::MissingField),
      "unsupported_schema" => Ok(Self::UnsupportedSchema),
      "invalid_value" => Ok(Self::InvalidValue),
      "actor_mismatch" => Ok(Self::ActorMismatch),
      "observation_already_open" => Ok(Self::ObservationAlreadyOpen),
      "no_observation" => Ok(Self::NoObservation),
      "stale_observation" => Ok(Self::StaleObservation),
      "duplicate_submission" => Ok(Self::DuplicateSubmission),
      "closed_session" => Ok(Self::ClosedSession),
      "window_closed" => Ok(Self::WindowClosed),
      "host_validation_rejected" => Ok(Self::HostValidationRejected),
      "host_transition_rejected" => Ok(Self::HostTransitionRejected),
      "draft_boundary" => Ok(Self::DraftBoundary),
      "debrief_unavailable" => Ok(Self::DebriefUnavailable),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Deterministic caller guidance for one actor-facing validation failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolRepairHint {
  RetryWithinSizeBound,
  ResendExactPayload,
  ResendCompletePayload,
  UseSupportedSchema,
  ResendValidPayload,
  UseBoundActor,
  SubmitCurrentAction,
  RequestObservation,
  RequestFreshObservation,
  AwaitNextObservation,
  StartNewSession,
  ResendAdvertisedAction,
  AwaitCompletion,
}

impl ActorProtocolRepairHint {
  pub const fn id(self) -> &'static str {
    match self {
      Self::RetryWithinSizeBound => "retry_within_size_bound",
      Self::ResendExactPayload => "resend_exact_payload",
      Self::ResendCompletePayload => "resend_complete_payload",
      Self::UseSupportedSchema => "use_supported_schema",
      Self::ResendValidPayload => "resend_valid_payload",
      Self::UseBoundActor => "use_bound_actor",
      Self::SubmitCurrentAction => "submit_current_action",
      Self::RequestObservation => "request_observation",
      Self::RequestFreshObservation => "request_fresh_observation",
      Self::AwaitNextObservation => "await_next_observation",
      Self::StartNewSession => "start_new_session",
      Self::ResendAdvertisedAction => "resend_advertised_action",
      Self::AwaitCompletion => "await_completion",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "retry_within_size_bound" => Ok(Self::RetryWithinSizeBound),
      "resend_exact_payload" => Ok(Self::ResendExactPayload),
      "resend_complete_payload" => Ok(Self::ResendCompletePayload),
      "use_supported_schema" => Ok(Self::UseSupportedSchema),
      "resend_valid_payload" => Ok(Self::ResendValidPayload),
      "use_bound_actor" => Ok(Self::UseBoundActor),
      "submit_current_action" => Ok(Self::SubmitCurrentAction),
      "request_observation" => Ok(Self::RequestObservation),
      "request_fresh_observation" => Ok(Self::RequestFreshObservation),
      "await_next_observation" => Ok(Self::AwaitNextObservation),
      "start_new_session" => Ok(Self::StartNewSession),
      "resend_advertised_action" => Ok(Self::ResendAdvertisedAction),
      "await_completion" => Ok(Self::AwaitCompletion),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Bounded actor-facing validation error with a deterministic repair hint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorProtocolError {
  schema: &'static str,
  code: ActorProtocolErrorCode,
  repair: ActorProtocolRepairHint,
}

impl ActorProtocolError {
  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn code(self) -> ActorProtocolErrorCode {
    self.code
  }

  pub const fn repair(self) -> ActorProtocolRepairHint {
    self.repair
  }

  pub const fn new(code: ActorProtocolErrorCode, repair: ActorProtocolRepairHint) -> Self {
    Self {
      schema: ACTOR_PROTOCOL_ERROR_SCHEMA,
      code,
      repair,
    }
  }

  /// Encode the bounded actor-safe error as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\ncode={}\nrepair={}\n",
      self.schema,
      self.code.id(),
      self.repair.id()
    )
  }

  /// Decode a bounded actor-safe error without raw payload or domain detail.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut code = None;
    let mut repair = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "code" => &mut code,
        "repair" => &mut repair,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_PROTOCOL_ERROR_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self {
      schema: ACTOR_PROTOCOL_ERROR_SCHEMA,
      code: ActorProtocolErrorCode::parse_id(code.ok_or(ActorProtocolCodecError::MissingField)?)?,
      repair: ActorProtocolRepairHint::parse_id(
        repair.ok_or(ActorProtocolCodecError::MissingField)?,
      )?,
    })
  }
}
