//! Bounded actor-authored message metadata with recipient scope.

use super::codec::{ActorProtocolCodecError, parse_fields};
use super::draft::MAX_ACTOR_DRAFT_VALUE_BYTES;

/// Versioned recipient-scoped actor message envelope identity.
pub const ACTOR_MESSAGE_SCHEMA: &str = "m5-actor-message-v1";

/// Maximum message attempts in one fixed communication-abuse population.
pub const MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION: usize = 4;

/// Versioned identity for bounded communication-abuse evidence.
pub const ACTOR_COMMUNICATION_ABUSE_POPULATION_SCHEMA: &str =
  "m6-actor-communication-abuse-population-v1";

/// Bounded actor-authored message metadata with explicit recipient binding.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActorMessageDto {
  schema: &'static str,
  sender: u8,
  recipient: u8,
  observation_id: u64,
  message: String,
}

impl ActorMessageDto {
  /// Build a message envelope without routing or delivering it.
  pub fn new(
    sender: u8,
    recipient: u8,
    observation_id: u64,
    message: &str,
  ) -> Result<Self, ActorProtocolCodecError> {
    if sender == 0
      || recipient == 0
      || sender == recipient
      || observation_id == 0
      || message.is_empty()
      || message.len() > MAX_ACTOR_DRAFT_VALUE_BYTES
      || message.chars().any(char::is_control)
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_MESSAGE_SCHEMA,
      sender,
      recipient,
      observation_id,
      message: message.to_owned(),
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn sender(&self) -> u8 {
    self.sender
  }

  pub const fn recipient(&self) -> u8 {
    self.recipient
  }

  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  /// Encode recipient-scoped metadata without adding delivery authority.
  pub fn encode(&self) -> String {
    format!(
      "schema={}\nsender={}\nrecipient={}\nobservation_id={}\nmessage={}\n",
      self.schema, self.sender, self.recipient, self.observation_id, self.message,
    )
  }

  /// Decode the exact bounded envelope without host or transport authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 5)?;
    let mut schema = None;
    let mut sender = None;
    let mut recipient = None;
    let mut observation_id = None;
    let mut message = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "sender" => &mut sender,
        "recipient" => &mut recipient,
        "observation_id" => &mut observation_id,
        "message" => &mut message,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_MESSAGE_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Self::new(
      sender
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      recipient
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      message.ok_or(ActorProtocolCodecError::MissingField)?,
    )
  }
}

/// Failures from constructing a communication-abuse population report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorCommunicationAbusePopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  UnexpectedError { actual: ActorProtocolCodecError },
  InvalidTarget,
}

/// Bounded actor-visible evidence over repeated invalid message values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorCommunicationAbusePopulationReport {
  schema: &'static str,
  sender: u8,
  recipient: u8,
  observation_id: u64,
  rejection_error: ActorProtocolCodecError,
  attempt_count: u8,
}

impl ActorCommunicationAbusePopulationReport {
  /// Validate one caller-declared invalid-message population without routing or delivery.
  pub fn from_invalid_payload(
    sender: u8,
    recipient: u8,
    observation_id: u64,
    invalid_payload: &str,
    attempt_count: usize,
  ) -> Result<Self, ActorCommunicationAbusePopulationError> {
    if attempt_count == 0 {
      return Err(ActorCommunicationAbusePopulationError::EmptyPopulation);
    }
    if attempt_count > MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION {
      return Err(ActorCommunicationAbusePopulationError::PopulationTooLarge {
        max: MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION,
        actual: attempt_count,
      });
    }
    if sender == 0 || recipient == 0 || observation_id == 0 {
      return Err(ActorCommunicationAbusePopulationError::InvalidTarget);
    }
    let rejection_error = ActorProtocolCodecError::InvalidValue;
    for _ in 0..attempt_count {
      let error = match ActorMessageDto::new(sender, recipient, observation_id, invalid_payload) {
        Ok(_) => {
          return Err(ActorCommunicationAbusePopulationError::UnexpectedError {
            actual: ActorProtocolCodecError::InvalidValue,
          });
        }
        Err(err) => err,
      };
      if error != rejection_error {
        return Err(ActorCommunicationAbusePopulationError::UnexpectedError { actual: error });
      }
    }
    Ok(Self {
      schema: ACTOR_COMMUNICATION_ABUSE_POPULATION_SCHEMA,
      sender,
      recipient,
      observation_id,
      rejection_error,
      attempt_count: u8::try_from(attempt_count).expect("population cap fits in u8"),
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn sender(self) -> u8 {
    self.sender
  }

  pub const fn recipient(self) -> u8 {
    self.recipient
  }

  pub const fn observation_id(self) -> u64 {
    self.observation_id
  }

  pub const fn rejection_error(self) -> ActorProtocolCodecError {
    self.rejection_error
  }

  pub const fn attempt_count(self) -> u8 {
    self.attempt_count
  }
}
