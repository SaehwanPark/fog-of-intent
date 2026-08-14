//! Common types, output DTOs, and error representations for host orchestration.

use super::scenario_host::CliScenarioHost;
use crate::cli::{CliParseError, CliProcessError, CliReadError, CliSessionError, CliWriteError};
use crate::lane::{
  LaneExecutionRelation, LaneIntent, LaneOutcome, ScenarioDebriefReport, ScenarioWindow,
};
use crate::protocol::{ActorActionDto, ActorProtocolErrorCode};

/// Versioned contract for the bounded synchronous host fixture.
pub const CLI_HOST_SCHEMA: &str = "m3-cli-host-v1";

/// Versioned actor-visible report for repeated invalid-command validation.
pub const ACTOR_ILLEGAL_COMMAND_POPULATION_SCHEMA: &str = "m6-actor-illegal-command-population-v1";

/// Maximum caller-declared invalid-command attempts in one bounded report.
pub const MAX_ACTOR_ILLEGAL_COMMAND_POPULATION: usize = 4;

/// Failures from constructing an illegal-command population report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorIllegalCommandPopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  UnexpectedCode { actual: ActorProtocolErrorCode },
}

/// Bounded actor-visible evidence over repeated host validation rejection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorIllegalCommandPopulationReport {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  rejection_code: ActorProtocolErrorCode,
  attempt_count: u8,
}

impl ActorIllegalCommandPopulationReport {
  /// Validate one caller-declared invalid-command population without mutation.
  ///
  /// The host must be active and the bounded population repeats the same
  /// observation-bound `Withdraw` request. Only the stable validation-error
  /// category is retained; no command payload, lane state, or raw failure is
  /// exposed.
  pub fn from_host(
    host: &CliScenarioHost,
    attempt_count: usize,
  ) -> Result<Self, ActorIllegalCommandPopulationError> {
    if attempt_count == 0 {
      return Err(ActorIllegalCommandPopulationError::EmptyPopulation);
    }
    if attempt_count > MAX_ACTOR_ILLEGAL_COMMAND_POPULATION {
      return Err(ActorIllegalCommandPopulationError::PopulationTooLarge {
        max: MAX_ACTOR_ILLEGAL_COMMAND_POPULATION,
        actual: attempt_count,
      });
    }
    let observation = host.observation();
    let action = ActorActionDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Withdraw,
    );
    let rejection_code = ActorProtocolErrorCode::HostValidationRejected;
    for _ in 0..attempt_count {
      let error = host
        .validate_actor_action(action)
        .expect_err("withdraw is invalid in the active fixture observation");
      if error.code() != rejection_code {
        return Err(ActorIllegalCommandPopulationError::UnexpectedCode {
          actual: error.code(),
        });
      }
    }
    Ok(Self {
      schema: ACTOR_ILLEGAL_COMMAND_POPULATION_SCHEMA,
      observer: observation.observer().value(),
      observation_id: observation.observation_id().value(),
      rejection_code,
      attempt_count: u8::try_from(attempt_count).expect("population cap fits in u8"),
    })
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

  pub const fn rejection_code(self) -> ActorProtocolErrorCode {
    self.rejection_code
  }

  pub const fn attempt_count(self) -> u8 {
    self.attempt_count
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HostDraft {
  pub(crate) message: Option<String>,
  pub(crate) plan: Option<String>,
  pub(crate) contingency: Option<String>,
}

impl HostDraft {
  pub(crate) fn empty() -> Self {
    Self {
      message: None,
      plan: None,
      contingency: None,
    }
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.message.is_none() && self.plan.is_none() && self.contingency.is_none()
  }
}

#[derive(Clone)]
pub(crate) struct SavedRun {
  pub(crate) run_id: String,
  pub(crate) artifact: String,
}

/// Actor-valid results returned by [`CliScenarioHost::apply_line`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliHostOutput {
  Help {
    topic: Option<&'static str>,
  },
  Observation(crate::lane::LanerObservation),
  History {
    records: u8,
    complete: bool,
  },
  DraftStaged {
    field: &'static str,
  },
  Committed {
    intent: LaneIntent,
  },
  Advanced {
    window: ScenarioWindow,
    outcome: LaneOutcome,
  },
  Review {
    records: u8,
    complete: bool,
  },
  Debrief(ScenarioDebriefReport),
  ReplayVerified {
    run_id: Option<String>,
    records: u8,
  },
  Branched {
    point_id: String,
    parent_intent: LaneIntent,
    branch_intent: LaneIntent,
    parent_outcome: LaneOutcome,
    branch_outcome: LaneOutcome,
    execution_relation: LaneExecutionRelation,
  },
  Saved {
    run_id: String,
    records: u8,
  },
  Loaded {
    run_id: String,
    records: u8,
  },
  Undone,
  Quit,
}

/// Errors raised before or while applying a CLI command at the host boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliHostError<'a> {
  Closed,
  Parse(CliParseError<'a>),
  Read(CliReadError<'a>),
  Write(CliWriteError),
  Process(CliProcessError),
  Session(CliSessionError),
  UnsupportedCommand { verb: &'static str },
  InvalidPlan { text: String },
  CommittedBoundary { verb: &'static str },
  MissingPlan,
  BranchMissingPlan,
  MissingCommittedIntent,
  NothingToUndo,
  RunNotFound { run_id: String },
  AdvanceRejected,
  ReplayRejected,
  BranchUnavailable,
  DebriefUnavailable,
  ScenarioComplete,
  StorageUnavailable,
  UnknownHelpTopic { topic: String },
}

/// Actor-safe chrome for the interactive presentation edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliSessionView {
  window: CliSessionWindow,
  records: u8,
  draft_fields: Vec<&'static str>,
  committed_intent: Option<LaneIntent>,
  store_configured: bool,
  suggested_next: Vec<&'static str>,
}

impl CliSessionView {
  pub(crate) fn new(
    window: CliSessionWindow,
    records: u8,
    draft_fields: Vec<&'static str>,
    committed_intent: Option<LaneIntent>,
    store_configured: bool,
    suggested_next: Vec<&'static str>,
  ) -> Self {
    Self {
      window,
      records,
      draft_fields,
      committed_intent,
      store_configured,
      suggested_next,
    }
  }

  pub const fn window(&self) -> CliSessionWindow {
    self.window
  }

  pub const fn records(&self) -> u8 {
    self.records
  }

  pub fn draft_fields(&self) -> &[&'static str] {
    &self.draft_fields
  }

  pub const fn committed_intent(&self) -> Option<LaneIntent> {
    self.committed_intent
  }

  pub const fn store_configured(&self) -> bool {
    self.store_configured
  }

  pub fn suggested_next(&self) -> &[&'static str] {
    &self.suggested_next
  }
}

/// Which bounded window the actor-visible session is in.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CliSessionWindow {
  First,
  Second,
  Complete,
}
