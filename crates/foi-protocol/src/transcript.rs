//! Provider-neutral actor tool and transcript DTOs.

use super::action::ACTOR_ACTION_SCHEMA;
use super::codec::{ActorProtocolCodecError, parse_fields};
use super::commit::ACTOR_COMMIT_SCHEMA;
use super::draft::{ACTOR_DRAFT_RECEIPT_SCHEMA, ACTOR_DRAFT_SCHEMA};
use super::observation::ACTOR_OBSERVATION_SCHEMA;

/// Versioned provider-neutral actor transcript identity.
pub const ACTOR_TRANSCRIPT_SCHEMA: &str = "m5-actor-transcript-v1";

/// Closed provider-neutral actor tool identities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorTranscriptTool {
  Observation,
  Draft,
  DraftReceipt,
  Commit,
  Action,
}

impl ActorTranscriptTool {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Observation => "observation",
      Self::Draft => "draft",
      Self::DraftReceipt => "draft_receipt",
      Self::Commit => "commit",
      Self::Action => "action",
    }
  }

  pub const fn schema_id(self) -> &'static str {
    match self {
      Self::Observation => ACTOR_OBSERVATION_SCHEMA,
      Self::Draft => ACTOR_DRAFT_SCHEMA,
      Self::DraftReceipt => ACTOR_DRAFT_RECEIPT_SCHEMA,
      Self::Commit => ACTOR_COMMIT_SCHEMA,
      Self::Action => ACTOR_ACTION_SCHEMA,
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "observation" => Ok(Self::Observation),
      "draft" => Ok(Self::Draft),
      "draft_receipt" => Ok(Self::DraftReceipt),
      "commit" => Ok(Self::Commit),
      "action" => Ok(Self::Action),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Closed authority labels for actor-facing tool capabilities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorToolAuthority {
  OrdinaryActor,
  PrivilegedExperimentController,
}

impl ActorToolAuthority {
  pub const fn id(self) -> &'static str {
    match self {
      Self::OrdinaryActor => "ordinary_actor",
      Self::PrivilegedExperimentController => "privileged_experiment_controller",
    }
  }
}

/// Pure capability metadata for one closed actor tool.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorToolCapability {
  tool: ActorTranscriptTool,
  authority: ActorToolAuthority,
}

impl ActorToolCapability {
  pub const fn new(tool: ActorTranscriptTool, authority: ActorToolAuthority) -> Self {
    Self { tool, authority }
  }

  pub const fn tool(self) -> ActorTranscriptTool {
    self.tool
  }

  pub const fn authority(self) -> ActorToolAuthority {
    self.authority
  }
}

/// Return the stable ordinary-actor capability catalog.
pub const fn actor_tool_capabilities() -> [ActorToolCapability; 5] {
  [
    ActorToolCapability::new(
      ActorTranscriptTool::Observation,
      ActorToolAuthority::OrdinaryActor,
    ),
    ActorToolCapability::new(
      ActorTranscriptTool::Draft,
      ActorToolAuthority::OrdinaryActor,
    ),
    ActorToolCapability::new(
      ActorTranscriptTool::DraftReceipt,
      ActorToolAuthority::OrdinaryActor,
    ),
    ActorToolCapability::new(
      ActorTranscriptTool::Commit,
      ActorToolAuthority::OrdinaryActor,
    ),
    ActorToolCapability::new(
      ActorTranscriptTool::Action,
      ActorToolAuthority::OrdinaryActor,
    ),
  ]
}

/// Closed provider-neutral actor operation result values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorTranscriptResult {
  Accepted,
  Rejected,
}

impl ActorTranscriptResult {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Accepted => "accepted",
      Self::Rejected => "rejected",
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "accepted" => Ok(Self::Accepted),
      "rejected" => Ok(Self::Rejected),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Bounded provider-neutral actor operation transcript metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorTranscriptDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  tool: ActorTranscriptTool,
  tool_schema: &'static str,
  result: ActorTranscriptResult,
}

impl ActorTranscriptDto {
  pub const fn new(
    observer: u8,
    observation_id: u64,
    tool: ActorTranscriptTool,
    result: ActorTranscriptResult,
  ) -> Self {
    Self {
      schema: ACTOR_TRANSCRIPT_SCHEMA,
      observer,
      observation_id,
      tool,
      tool_schema: tool.schema_id(),
      result,
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

  pub const fn tool(self) -> ActorTranscriptTool {
    self.tool
  }

  pub const fn tool_schema(self) -> &'static str {
    self.tool_schema
  }

  pub const fn result(self) -> ActorTranscriptResult {
    self.result
  }

  /// Encode provider-neutral transcript metadata as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\ntool={}\ntool_schema={}\nresult={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.tool.id(),
      self.tool_schema,
      self.result.id()
    )
  }

  /// Decode bounded transcript metadata without runtime or simulation authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 6)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut tool = None;
    let mut tool_schema = None;
    let mut result = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "tool" => &mut tool,
        "tool_schema" => &mut tool_schema,
        "result" => &mut result,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_TRANSCRIPT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let tool = ActorTranscriptTool::parse_id(tool.ok_or(ActorProtocolCodecError::MissingField)?)?;
    if tool_schema != Some(tool.schema_id()) {
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
      tool,
      ActorTranscriptResult::parse_id(result.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}
