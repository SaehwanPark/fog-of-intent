//! Versioned actor-protocol DTOs at the M5 adapter boundary.
//!
//! The DTOs contain only bounded actor-visible observation, action, metadata,
//! lifecycle, result, and committed-facts review data. They do not validate
//! legality, resolve execution, mutate history, or depend on a transport,
//! async runtime, or provider SDK.

pub mod action;
pub mod codec;
pub mod commit;
pub mod debrief;
pub mod draft;
pub mod error;
pub mod history;
pub mod intents;
pub mod message;
pub mod observation;
pub mod replay;
pub mod transcript;

pub use foi_kernel as kernel;
pub use foi_lane as lane;

pub use crate as protocol;

#[cfg(test)]
mod tests;

pub use action::{
  ACTOR_ACTION_RESULT_SCHEMA, ACTOR_ACTION_SCHEMA, ActorActionDto, ActorActionResultDto,
  ActorActionResultOutcome, ActorActionResultWindow,
};
pub use codec::{ACTOR_PROTOCOL_CODEC_SCHEMA, ActorProtocolCodecError, MAX_ACTOR_PROTOCOL_BYTES};
pub use commit::{
  ACTOR_COMMIT_RESULT_SCHEMA, ACTOR_COMMIT_SCHEMA, ActorCommitDto, ActorCommitResultDto,
};
pub use debrief::{
  ACTOR_DEBRIEF_SCHEMA, ActorDebriefAttributionLimit, ActorDebriefDto, ActorDebriefObjective,
  ActorDebriefWindow,
};
pub use draft::{
  ACTOR_DRAFT_CLEAR_RECEIPT_SCHEMA, ACTOR_DRAFT_CLEAR_SCHEMA, ACTOR_DRAFT_COMMIT_RECEIPT_SCHEMA,
  ACTOR_DRAFT_RECEIPT_SCHEMA, ACTOR_DRAFT_SCHEMA, ACTOR_DRAFT_STATUS_SCHEMA, ActorDraftClearDto,
  ActorDraftClearReceiptDto, ActorDraftCommitReceiptDto, ActorDraftDto, ActorDraftField,
  ActorDraftPresence, ActorDraftReceiptDto, ActorDraftStatusDto, MAX_ACTOR_DRAFT_VALUE_BYTES,
};
pub use error::{
  ACTOR_PROTOCOL_ERROR_SCHEMA, ACTOR_PROTOCOL_ERROR_SCHEMA_V1, ActorProtocolError,
  ActorProtocolErrorCode, ActorProtocolRepairHint,
};
pub use history::{ACTOR_HISTORY_SCHEMA, ActorHistoryDto, ActorHistoryStatus};
pub use intents::{ACTOR_PROTOCOL_SCHEMA, ActorProtocolIntent};
pub use message::{
  ACTOR_COMMUNICATION_ABUSE_POPULATION_SCHEMA, ACTOR_MESSAGE_SCHEMA,
  ActorCommunicationAbusePopulationError, ActorCommunicationAbusePopulationReport, ActorMessageDto,
  MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION,
};
pub use observation::{ACTOR_OBSERVATION_SCHEMA, ActorObservationDto};
pub use replay::{
  ACTOR_REPLAY_DEBRIEF_RECORD_SCHEMA, ACTOR_REPLAY_RECORD_SCHEMA, ACTOR_REPLAY_SCHEMA,
  ActorReplayDebriefRecordDto, ActorReplayDto, ActorReplayRecordDto, ActorReplayVerification,
};
pub use transcript::{
  ACTOR_TRANSCRIPT_SCHEMA, ActorToolAuthority, ActorToolCapability, ActorTranscriptDto,
  ActorTranscriptResult, ActorTranscriptTool, actor_tool_capabilities,
};
