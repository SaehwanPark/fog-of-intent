//! Bounded parser and serializer helpers for line-oriented protocol DTOs.

use super::error::{ActorProtocolError, ActorProtocolErrorCode, ActorProtocolRepairHint};

/// Versioned line-oriented codec identity for the bounded DTOs.
pub const ACTOR_PROTOCOL_CODEC_SCHEMA: &str = "m5-actor-codec-v1";

/// Maximum encoded DTO size accepted by the bounded parser.
pub const MAX_ACTOR_PROTOCOL_BYTES: usize = 4096;

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

impl ActorProtocolCodecError {
  /// Project a codec failure without retaining input or parser details.
  pub const fn to_actor_error(self) -> ActorProtocolError {
    match self {
      Self::Oversized => ActorProtocolError::new(
        ActorProtocolErrorCode::OversizedInput,
        ActorProtocolRepairHint::RetryWithinSizeBound,
      ),
      Self::UnexpectedLineCount { .. } => ActorProtocolError::new(
        ActorProtocolErrorCode::UnexpectedLineCount,
        ActorProtocolRepairHint::ResendExactPayload,
      ),
      Self::UnknownField => ActorProtocolError::new(
        ActorProtocolErrorCode::UnknownField,
        ActorProtocolRepairHint::ResendExactPayload,
      ),
      Self::DuplicateField => ActorProtocolError::new(
        ActorProtocolErrorCode::DuplicateField,
        ActorProtocolRepairHint::ResendExactPayload,
      ),
      Self::MissingField => ActorProtocolError::new(
        ActorProtocolErrorCode::MissingField,
        ActorProtocolRepairHint::ResendCompletePayload,
      ),
      Self::UnsupportedSchema => ActorProtocolError::new(
        ActorProtocolErrorCode::UnsupportedSchema,
        ActorProtocolRepairHint::UseSupportedSchema,
      ),
      Self::InvalidValue => ActorProtocolError::new(
        ActorProtocolErrorCode::InvalidValue,
        ActorProtocolRepairHint::ResendValidPayload,
      ),
    }
  }
}

pub(crate) fn parse_fields(
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
