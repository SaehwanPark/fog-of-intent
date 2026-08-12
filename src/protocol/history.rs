//! Bounded actor-visible history lifecycle status and record count.

use super::codec::{ActorProtocolCodecError, parse_fields};

/// Versioned actor-visible bounded history-status identity.
pub const ACTOR_HISTORY_SCHEMA: &str = "m5-actor-history-v1";

/// Closed actor-visible lifecycle status for the bounded fixture history.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorHistoryStatus {
  Open,
  Complete,
  Closed,
}

impl ActorHistoryStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Open => "open",
      Self::Complete => "complete",
      Self::Closed => "closed",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "open" => Ok(Self::Open),
      "complete" => Ok(Self::Complete),
      "closed" => Ok(Self::Closed),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Bounded actor-visible history count and lifecycle status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorHistoryDto {
  schema: &'static str,
  records: u8,
  status: ActorHistoryStatus,
}

impl ActorHistoryDto {
  pub fn new(records: u8, status: ActorHistoryStatus) -> Result<Self, ActorProtocolCodecError> {
    if records > 2
      || (status == ActorHistoryStatus::Open && records == 2)
      || (status == ActorHistoryStatus::Complete && records != 2)
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_HISTORY_SCHEMA,
      records,
      status,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn records(self) -> u8 {
    self.records
  }

  pub const fn status(self) -> ActorHistoryStatus {
    self.status
  }

  /// Encode bounded history status as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nrecords={}\nstatus={}\n",
      self.schema,
      self.records,
      self.status.id()
    )
  }

  /// Decode bounded history status without exposing state hashes or snapshots.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut records = None;
    let mut status = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "records" => &mut records,
        "status" => &mut status,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_HISTORY_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let records = records
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let status =
      ActorHistoryStatus::parse_id(status.ok_or(ActorProtocolCodecError::MissingField)?)?;
    Self::new(records, status)
  }
}
