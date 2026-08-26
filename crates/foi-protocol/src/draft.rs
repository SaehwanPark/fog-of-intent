//! Actor draft staging, clearing, and presence receipts.

use super::codec::{ActorProtocolCodecError, parse_fields};
use super::intents::ActorProtocolIntent;

/// Versioned actor message/plan/contingency metadata identity.
pub const ACTOR_DRAFT_SCHEMA: &str = "m5-actor-draft-v1";

/// Versioned actor draft-staging acknowledgement identity.
pub const ACTOR_DRAFT_RECEIPT_SCHEMA: &str = "m5-actor-draft-receipt-v1";

/// Versioned actor draft-status projection identity.
pub const ACTOR_DRAFT_STATUS_SCHEMA: &str = "m5-actor-draft-status-v1";

/// Versioned actor draft-clear command identity.
pub const ACTOR_DRAFT_CLEAR_SCHEMA: &str = "m5-actor-draft-clear-v1";

/// Versioned actor draft-clear acknowledgement identity.
pub const ACTOR_DRAFT_CLEAR_RECEIPT_SCHEMA: &str = "m5-actor-draft-clear-receipt-v1";

/// Versioned actor draft-commit field-presence acknowledgement identity.
pub const ACTOR_DRAFT_COMMIT_RECEIPT_SCHEMA: &str = "m5-actor-draft-commit-receipt-v1";

/// Maximum UTF-8 payload size for one actor draft metadata value.
pub const MAX_ACTOR_DRAFT_VALUE_BYTES: usize = 256;

/// Closed actor-draft field-presence values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorDraftPresence {
  Present,
  Absent,
}

impl ActorDraftPresence {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Present => "present",
      Self::Absent => "absent",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "present" => Ok(Self::Present),
      "absent" => Ok(Self::Absent),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Closed actor-draft metadata fields.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorDraftField {
  Message,
  Plan,
  Contingency,
}

impl ActorDraftField {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Message => "message",
      Self::Plan => "plan",
      Self::Contingency => "contingency",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "message" => Ok(Self::Message),
      "plan" => Ok(Self::Plan),
      "contingency" => Ok(Self::Contingency),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Versioned bounded actor message/plan/contingency metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  field: ActorDraftField,
  value: String,
}

impl ActorDraftDto {
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> u8 {
    self.observer
  }

  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  pub const fn field(&self) -> ActorDraftField {
    self.field
  }

  pub fn value(&self) -> &str {
    &self.value
  }

  /// Build bounded metadata without staging or submitting it to the host.
  pub fn new(
    observer: u8,
    observation_id: u64,
    field: ActorDraftField,
    value: &str,
  ) -> Result<Self, ActorProtocolCodecError> {
    if value.is_empty()
      || value.len() > MAX_ACTOR_DRAFT_VALUE_BYTES
      || value.chars().any(char::is_control)
      || (field == ActorDraftField::Plan && ActorProtocolIntent::parse_id(value).is_err())
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_DRAFT_SCHEMA,
      observer,
      observation_id,
      field,
      value: value.to_owned(),
    })
  }

  /// Encode bounded metadata as stable line-oriented text.
  pub fn encode(&self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nfield={}\nvalue={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.field.id(),
      self.value,
    )
  }

  /// Decode bounded metadata without assigning host or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 5)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut field = None;
    let mut value = None;
    for (key, field_value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "field" => &mut field,
        "value" => &mut value,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(field_value);
    }
    if schema != Some(ACTOR_DRAFT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let observer = observer
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let observation_id = observation_id
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let field = ActorDraftField::parse_id(field.ok_or(ActorProtocolCodecError::MissingField)?)?;
    Self::new(
      observer,
      observation_id,
      field,
      value.ok_or(ActorProtocolCodecError::MissingField)?,
    )
  }
}

/// Bounded actor-safe acknowledgement after host-owned draft staging.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftReceiptDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  field: ActorDraftField,
}

impl ActorDraftReceiptDto {
  pub const fn new(observer: u8, observation_id: u64, field: ActorDraftField) -> Self {
    Self {
      schema: ACTOR_DRAFT_RECEIPT_SCHEMA,
      observer,
      observation_id,
      field,
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

  pub const fn field(self) -> ActorDraftField {
    self.field
  }

  /// Encode the bounded draft receipt as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nfield={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.field.id()
    )
  }

  /// Decode a bounded draft receipt without host or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 4)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut field = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "field" => &mut field,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_DRAFT_RECEIPT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      ActorDraftField::parse_id(field.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}

/// Bounded actor-visible aggregate status for the active host draft.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftStatusDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  message: ActorDraftPresence,
  plan: ActorDraftPresence,
  contingency: ActorDraftPresence,
}

impl ActorDraftStatusDto {
  pub const fn new(
    observer: u8,
    observation_id: u64,
    message: ActorDraftPresence,
    plan: ActorDraftPresence,
    contingency: ActorDraftPresence,
  ) -> Self {
    Self {
      schema: ACTOR_DRAFT_STATUS_SCHEMA,
      observer,
      observation_id,
      message,
      plan,
      contingency,
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

  pub const fn message(self) -> ActorDraftPresence {
    self.message
  }

  pub const fn plan(self) -> ActorDraftPresence {
    self.plan
  }

  pub const fn contingency(self) -> ActorDraftPresence {
    self.contingency
  }

  /// Encode aggregate draft presence without returning any payload values.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nmessage={}\nplan={}\ncontingency={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.message.id(),
      self.plan.id(),
      self.contingency.id(),
    )
  }

  /// Decode aggregate draft presence without host or delivery authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 6)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut message = None;
    let mut plan = None;
    let mut contingency = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "message" => &mut message,
        "plan" => &mut plan,
        "contingency" => &mut contingency,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_DRAFT_STATUS_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      ActorDraftPresence::parse_id(message.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(plan.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(contingency.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}

/// Bounded actor command that clears the active draft without carrying values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftClearDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
}

impl ActorDraftClearDto {
  pub const fn new(observer: u8, observation_id: u64) -> Self {
    Self {
      schema: ACTOR_DRAFT_CLEAR_SCHEMA,
      observer,
      observation_id,
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

  /// Encode the observation-bound clear command as stable text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\n",
      self.schema, self.observer, self.observation_id
    )
  }

  /// Decode the bounded clear command without host authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_DRAFT_CLEAR_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
    ))
  }
}

/// Bounded acknowledgement reporting fields present before a successful clear.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftClearReceiptDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  message: ActorDraftPresence,
  plan: ActorDraftPresence,
  contingency: ActorDraftPresence,
}

impl ActorDraftClearReceiptDto {
  pub const fn new(
    observer: u8,
    observation_id: u64,
    message: ActorDraftPresence,
    plan: ActorDraftPresence,
    contingency: ActorDraftPresence,
  ) -> Self {
    Self {
      schema: ACTOR_DRAFT_CLEAR_RECEIPT_SCHEMA,
      observer,
      observation_id,
      message,
      plan,
      contingency,
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

  pub const fn message(self) -> ActorDraftPresence {
    self.message
  }

  pub const fn plan(self) -> ActorDraftPresence {
    self.plan
  }

  pub const fn contingency(self) -> ActorDraftPresence {
    self.contingency
  }

  /// Encode the payload-free clear acknowledgement.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nmessage={}\nplan={}\ncontingency={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.message.id(),
      self.plan.id(),
      self.contingency.id(),
    )
  }

  /// Decode the bounded clear acknowledgement without host authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 6)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut message = None;
    let mut plan = None;
    let mut contingency = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "message" => &mut message,
        "plan" => &mut plan,
        "contingency" => &mut contingency,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_DRAFT_CLEAR_RECEIPT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      ActorDraftPresence::parse_id(message.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(plan.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(contingency.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}

/// Bounded actor-safe acknowledgement after a host-owned draft commit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftCommitReceiptDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  intent: ActorProtocolIntent,
  message: ActorDraftPresence,
  plan: ActorDraftPresence,
  contingency: ActorDraftPresence,
}

impl ActorDraftCommitReceiptDto {
  pub const fn new(
    observer: u8,
    observation_id: u64,
    intent: ActorProtocolIntent,
    message: ActorDraftPresence,
    plan: ActorDraftPresence,
    contingency: ActorDraftPresence,
  ) -> Self {
    Self {
      schema: ACTOR_DRAFT_COMMIT_RECEIPT_SCHEMA,
      observer,
      observation_id,
      intent,
      message,
      plan,
      contingency,
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

  pub const fn message(self) -> ActorDraftPresence {
    self.message
  }

  pub const fn plan(self) -> ActorDraftPresence {
    self.plan
  }

  pub const fn contingency(self) -> ActorDraftPresence {
    self.contingency
  }

  /// Encode accepted field presence without echoing draft values.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nintent={}\nmessage={}\nplan={}\ncontingency={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.intent.id(),
      self.message.id(),
      self.plan.id(),
      self.contingency.id(),
    )
  }

  /// Decode bounded commit metadata without host or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 7)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut intent = None;
    let mut message = None;
    let mut plan = None;
    let mut contingency = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "intent" => &mut intent,
        "message" => &mut message,
        "plan" => &mut plan,
        "contingency" => &mut contingency,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_DRAFT_COMMIT_RECEIPT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(message.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(plan.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorDraftPresence::parse_id(contingency.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}
