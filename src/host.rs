//! Synchronous application-host orchestration for the bounded M3 transcript.
//!
//! The host owns lifecycle, draft, save/load, and history coordination while
//! delegating validation and transition evaluation to the lane contract. It
//! accepts resolved execution inputs explicitly and returns actor-valid
//! projections; it does not render terminal output or expose true state.

use crate::cli::{
  CliCommand, CliParseError, CliProcessError, CliProcessRequest, CliReadError, CliReadRequest,
  CliSessionError, CliSessionRequest, CliWriteError, CliWriteRequest, parse_command,
  process_request, read_request, session_request, write_request,
};
use crate::host_artifact::CliHostArtifact;
use crate::kernel::{DrawId, InputTrace, StreamId};
use crate::lane::{
  BranchExecutionSelection, LaneDamage, LaneExecutionRelation, LaneHistory, LaneIntent,
  LaneIntentRequest, LaneOutcome, LaneResolvedInputs, LaneScenarioHistory, LaneWaveResult,
  ObservationId, PLAYER_LANER, ScenarioDebriefReport, ScenarioWindow, branch_from_window,
  build_scenario_debrief, observe_player,
};
use crate::protocol::{
  ActorActionDto, ActorActionResultDto, ActorActionResultOutcome, ActorActionResultWindow,
  ActorCommitDto, ActorCommitResultDto, ActorDebriefDto, ActorDraftDto, ActorDraftField,
  ActorDraftReceiptDto, ActorHistoryDto, ActorHistoryStatus, ActorObservationDto,
  ActorProtocolError, ActorProtocolErrorCode, ActorProtocolRepairHint,
};
use crate::run_store::{CliRunStore, CliRunStoreError};

/// Versioned contract for the bounded synchronous host fixture.
pub const CLI_HOST_SCHEMA: &str = "m3-cli-host-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
struct HostDraft {
  message: Option<String>,
  plan: Option<String>,
  contingency: Option<String>,
}

impl HostDraft {
  fn is_empty(&self) -> bool {
    self.message.is_none() && self.plan.is_none() && self.contingency.is_none()
  }
}

#[derive(Clone)]
struct SavedRun {
  run_id: String,
  artifact: String,
}

/// Actor-valid results returned by [`CliScenarioHost::apply_line`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliHostOutput {
  Help,
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
}

/// A bounded host for the existing deterministic two-window lane scenario.
///
/// `execution_inputs` are already resolved at construction. The host never
/// creates random values and never returns a true-state snapshot to callers.
pub struct CliScenarioHost {
  history: LaneScenarioHistory,
  execution_inputs: [LaneResolvedInputs; 2],
  draft: HostDraft,
  committed_intent: Option<LaneIntent>,
  saved: Option<SavedRun>,
  store: Option<CliRunStore>,
  closed: bool,
}

impl CliScenarioHost {
  /// Build a host with explicit inputs for the first and second windows.
  pub fn new(execution_inputs: [LaneResolvedInputs; 2]) -> Self {
    Self {
      history: LaneScenarioHistory::new(crate::lane::LaneSnapshot::initial())
        .expect("initial lane fixture must be valid"),
      execution_inputs,
      draft: HostDraft {
        message: None,
        plan: None,
        contingency: None,
      },
      committed_intent: None,
      saved: None,
      store: None,
      closed: false,
    }
  }

  /// Build the deterministic two-window fixture used by host transcript tests.
  pub fn fixture() -> Self {
    Self::new([
      fixture_inputs(1, LaneWaveResult::Advanced, 1),
      fixture_inputs(0, LaneWaveResult::Held, 2),
    ])
  }

  /// Build a deterministic fixture host backed by an explicit artifact store.
  pub fn fixture_with_store(store: CliRunStore) -> Self {
    Self::with_store(
      [
        fixture_inputs(1, LaneWaveResult::Advanced, 1),
        fixture_inputs(0, LaneWaveResult::Held, 2),
      ],
      store,
    )
  }

  /// Build a host with explicit inputs and an injected artifact store.
  pub fn with_store(execution_inputs: [LaneResolvedInputs; 2], store: CliRunStore) -> Self {
    let mut host = Self::new(execution_inputs);
    host.store = Some(store);
    host
  }

  /// Return the stable host schema identifier.
  pub const fn schema() -> &'static str {
    CLI_HOST_SCHEMA
  }

  /// Return the current actor-visible observation.
  pub fn observation(&self) -> crate::lane::LanerObservation {
    observe_player(&self.history.current_state(), self.next_observation_id()).observation()
  }

  /// Return the active observation through the bounded actor protocol DTO.
  pub fn actor_observation(&self) -> Result<ActorObservationDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    if self.is_complete() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    Ok(ActorObservationDto::from_observation(self.observation()))
  }

  /// Return bounded actor-visible history status without exposing state details.
  pub fn actor_history(&self) -> ActorHistoryDto {
    let status = if self.closed {
      ActorHistoryStatus::Closed
    } else if self.is_complete() {
      ActorHistoryStatus::Complete
    } else {
      ActorHistoryStatus::Open
    };
    ActorHistoryDto::new(self.record_count(), status)
      .expect("host history status stays within the two-window fixture bounds")
  }

  /// Validate one actor action without mutating host state or history.
  pub fn validate_actor_action(&self, action: ActorActionDto) -> Result<(), ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let receipt = observe_player(&self.history.current_state(), self.next_observation_id());
    if action.observer() != receipt.observation().observer().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ));
    }
    if self.is_complete() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    if action.observation_id() != receipt.observation().observation_id().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ));
    }
    crate::lane::validate_lane_request(
      &self.history.current_state(),
      &receipt,
      &action.to_lane_request(),
    )
    .map(|_| ())
    .map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostValidationRejected,
        ActorProtocolRepairHint::ResendAdvertisedAction,
      )
    })
  }

  /// Stage one bounded actor draft field without committing or advancing.
  pub fn stage_actor_draft(
    &mut self,
    draft: ActorDraftDto,
  ) -> Result<CliHostOutput, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let receipt = observe_player(&self.history.current_state(), self.next_observation_id());
    if draft.observer() != receipt.observation().observer().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ));
    }
    if self.is_complete() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    if self.committed_intent.is_some() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ));
    }
    if draft.observation_id() != receipt.observation().observation_id().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ));
    }
    match draft.field() {
      ActorDraftField::Message => self.draft.message = Some(draft.value().to_owned()),
      ActorDraftField::Plan => self.draft.plan = Some(draft.value().to_owned()),
      ActorDraftField::Contingency => self.draft.contingency = Some(draft.value().to_owned()),
    }
    Ok(CliHostOutput::DraftStaged {
      field: draft.field().id(),
    })
  }

  /// Stage one bounded actor draft field and acknowledge its host receipt.
  pub fn stage_actor_draft_receipt(
    &mut self,
    draft: ActorDraftDto,
  ) -> Result<ActorDraftReceiptDto, ActorProtocolError> {
    let observer = draft.observer();
    let observation_id = draft.observation_id();
    let field = draft.field();
    self.stage_actor_draft(draft)?;
    Ok(ActorDraftReceiptDto::new(observer, observation_id, field))
  }

  /// Commit one observation-bound actor intent without advancing the host.
  pub fn commit_actor_draft(
    &mut self,
    commit: ActorCommitDto,
  ) -> Result<ActorCommitResultDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let receipt = observe_player(&self.history.current_state(), self.next_observation_id());
    if commit.observer() != receipt.observation().observer().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ));
    }
    if self.is_complete() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    if self.committed_intent.is_some() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ));
    }
    if commit.observation_id() != receipt.observation().observation_id().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ));
    }
    if let Some(staged_plan) = self.draft.plan.as_deref()
      && parse_plan_intent(staged_plan) != Some(commit.to_lane_intent())
    {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::HostValidationRejected,
        ActorProtocolRepairHint::ResendValidPayload,
      ));
    }
    self.committed_intent = Some(commit.to_lane_intent());
    self.draft = HostDraft {
      message: None,
      plan: None,
      contingency: None,
    };
    Ok(ActorCommitResultDto::new(commit.intent()))
  }

  /// Validate and submit one actor action, then close the host-owned window.
  pub fn submit_actor_action(
    &mut self,
    action: ActorActionDto,
  ) -> Result<CliHostOutput, ActorProtocolError> {
    self.validate_actor_action(action)?;
    let previous_committed_intent = self.committed_intent;
    self.committed_intent = Some(action.to_lane_request().intent());
    match self.advance() {
      Ok(output) => Ok(output),
      Err(error) => {
        self.committed_intent = previous_committed_intent;
        Err(match error {
          CliHostError::ScenarioComplete => ActorProtocolError::new(
            ActorProtocolErrorCode::WindowClosed,
            ActorProtocolRepairHint::StartNewSession,
          ),
          _ => ActorProtocolError::new(
            ActorProtocolErrorCode::HostTransitionRejected,
            ActorProtocolRepairHint::StartNewSession,
          ),
        })
      }
    }
  }

  /// Submit one actor action and return only its bounded actor-safe result.
  pub fn submit_actor_action_result(
    &mut self,
    action: ActorActionDto,
  ) -> Result<ActorActionResultDto, ActorProtocolError> {
    let output = self.submit_actor_action(action)?;
    let CliHostOutput::Advanced { window, outcome } = output else {
      unreachable!("actor action submission succeeds only with an advance result")
    };
    let window = match window {
      ScenarioWindow::First => ActorActionResultWindow::First,
      ScenarioWindow::Second => ActorActionResultWindow::Second,
    };
    let outcome = match outcome {
      LaneOutcome::HeldSpace => ActorActionResultOutcome::HeldSpace,
      LaneOutcome::YieldedSpace => ActorActionResultOutcome::YieldedSpace,
      LaneOutcome::ForcedOut => ActorActionResultOutcome::ForcedOut,
    };
    Ok(ActorActionResultDto::new(window, outcome))
  }

  /// Return a bounded actor-visible debrief summary for a completed host.
  pub fn actor_debrief(&self) -> Result<ActorDebriefDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    if !self.is_complete() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DebriefUnavailable,
        ActorProtocolRepairHint::AwaitCompletion,
      ));
    }
    let report = build_scenario_debrief(&self.history)
      .map_err(|_| {
        ActorProtocolError::new(
          ActorProtocolErrorCode::HostTransitionRejected,
          ActorProtocolRepairHint::StartNewSession,
        )
      })?
      .report();
    Ok(ActorDebriefDto::from_report(report))
  }

  /// Return the number of committed scenario windows.
  pub fn record_count(&self) -> u8 {
    u8::try_from(self.history.records().len()).expect("two-window history fits in u8")
  }

  /// Whether both bounded scenario windows have been committed.
  pub fn is_complete(&self) -> bool {
    self.history.records().len() == 2
  }

  /// Apply one parsed-and-mapped CLI line at the host boundary.
  pub fn apply_line<'a>(&mut self, line: &'a str) -> Result<CliHostOutput, CliHostError<'a>> {
    if self.closed {
      return Err(CliHostError::Closed);
    }
    let command = parse_command(line).map_err(CliHostError::Parse)?;
    match command {
      CliCommand::Help | CliCommand::Observe | CliCommand::Inspect(_) => {
        let request = read_request(command).map_err(CliHostError::Read)?;
        self.apply_read(request)
      }
      CliCommand::Message(_)
      | CliCommand::Plan(_)
      | CliCommand::Contingency(_)
      | CliCommand::Commit
      | CliCommand::Advance => {
        let request = write_request(command).map_err(CliHostError::Write)?;
        self.apply_write(request)
      }
      CliCommand::Review | CliCommand::Debrief | CliCommand::Replay(_) | CliCommand::Branch(_) => {
        let request = process_request(command).map_err(CliHostError::Process)?;
        self.apply_process(request)
      }
      CliCommand::Save(_) | CliCommand::Load(_) | CliCommand::Undo | CliCommand::Quit => {
        let request = session_request(command).map_err(CliHostError::Session)?;
        self.apply_session(request)
      }
    }
  }

  fn apply_read(&self, request: CliReadRequest) -> Result<CliHostOutput, CliHostError<'static>> {
    match request {
      CliReadRequest::Help => Ok(CliHostOutput::Help),
      CliReadRequest::Observe
      | CliReadRequest::Inspect(crate::cli::CliInspectTarget::CurrentObservation) => {
        Ok(CliHostOutput::Observation(self.observation()))
      }
      CliReadRequest::Inspect(crate::cli::CliInspectTarget::VisibleHistoryReport) => {
        Ok(CliHostOutput::History {
          records: self.record_count(),
          complete: self.is_complete(),
        })
      }
    }
  }

  fn apply_write<'a>(
    &mut self,
    request: CliWriteRequest<'a>,
  ) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliWriteRequest::Message { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "message" });
        }
        self.draft.message = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged { field: "message" })
      }
      CliWriteRequest::Plan { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "plan" });
        }
        self.draft.plan = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged { field: "plan" })
      }
      CliWriteRequest::Contingency { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary {
            verb: "contingency",
          });
        }
        self.draft.contingency = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged {
          field: "contingency",
        })
      }
      CliWriteRequest::Commit => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "commit" });
        }
        let text = self
          .draft
          .plan
          .as_deref()
          .ok_or(CliHostError::MissingPlan)?;
        let intent = parse_plan_intent(text).ok_or_else(|| CliHostError::InvalidPlan {
          text: text.to_owned(),
        })?;
        self.committed_intent = Some(intent);
        self.draft = HostDraft {
          message: None,
          plan: None,
          contingency: None,
        };
        Ok(CliHostOutput::Committed { intent })
      }
      CliWriteRequest::Advance => self.advance(),
    }
  }

  fn apply_process<'a>(
    &mut self,
    request: CliProcessRequest<'a>,
  ) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliProcessRequest::Review => Ok(CliHostOutput::Review {
        records: self.record_count(),
        complete: self.is_complete(),
      }),
      CliProcessRequest::Debrief => build_scenario_debrief(&self.history)
        .map(|record| CliHostOutput::Debrief(record.report()))
        .map_err(|_| CliHostError::DebriefUnavailable),
      CliProcessRequest::Replay { run_id } => {
        let (run_id, records) = if let Some(run_id) = run_id {
          let requested = run_id.as_str();
          let artifact = CliHostArtifact::decode(&self.load_artifact(requested)?)
            .map_err(|_| CliHostError::ReplayRejected)?;
          if artifact.run_id() != requested {
            return Err(CliHostError::ReplayRejected);
          }
          let history = self.restore_artifact(&artifact)?;
          (Some(requested.to_owned()), history.records().len())
        } else {
          self
            .history
            .verify_replay()
            .map_err(|_| CliHostError::ReplayRejected)?;
          (None, self.history.records().len())
        };
        Ok(CliHostOutput::ReplayVerified {
          run_id,
          records: u8::try_from(records).expect("two-window history fits in u8"),
        })
      }
      CliProcessRequest::Branch { point_id } => self.branch(point_id),
    }
  }

  fn branch<'a>(&self, point_id: Option<&str>) -> Result<CliHostOutput, CliHostError<'a>> {
    let point_id = point_id.unwrap_or("first");
    if point_id != "first" || self.history.records().len() != 1 {
      return Err(CliHostError::BranchUnavailable);
    }
    let alternate_text = self
      .draft
      .plan
      .as_deref()
      .ok_or(CliHostError::BranchMissingPlan)?;
    let alternate_intent =
      parse_plan_intent(alternate_text).ok_or_else(|| CliHostError::InvalidPlan {
        text: alternate_text.to_owned(),
      })?;
    let scenario_record = self
      .history
      .records()
      .first()
      .ok_or(CliHostError::BranchUnavailable)?;
    let transition = scenario_record.transition().clone();
    let parent_intent = transition.command().intent();
    let mut parent = LaneHistory::new(self.history.initial_state())
      .map_err(|_| CliHostError::BranchUnavailable)?;
    parent.current_state = transition.result().next_state();
    parent.records.push(transition);
    let request = LaneIntentRequest::new(
      PLAYER_LANER,
      scenario_record.transition().command().observation_id(),
      alternate_intent,
    );
    let branch = branch_from_window(
      &parent,
      &request,
      BranchExecutionSelection::matched_parent(),
    )
    .map_err(|_| CliHostError::BranchUnavailable)?;
    let review = branch
      .review(&parent)
      .map_err(|_| CliHostError::BranchUnavailable)?;
    Ok(CliHostOutput::Branched {
      point_id: point_id.to_owned(),
      parent_intent,
      branch_intent: review.branch_intent(),
      parent_outcome: review.parent_outcome(),
      branch_outcome: review.branch_outcome(),
      execution_relation: review.execution_relation(),
    })
  }

  fn apply_session<'a>(
    &mut self,
    request: CliSessionRequest<'a>,
  ) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliSessionRequest::Save { run_id } => {
        let run_id = run_id.as_str().to_owned();
        let artifact = CliHostArtifact::encode(&run_id, &self.history)
          .map_err(|_| CliHostError::ReplayRejected)?;
        if let Some(store) = self.store.as_ref() {
          store
            .save(&run_id, &artifact)
            .map_err(|_| CliHostError::StorageUnavailable)?;
        }
        self.saved = Some(SavedRun {
          run_id: run_id.clone(),
          artifact,
        });
        Ok(CliHostOutput::Saved {
          run_id,
          records: self.record_count(),
        })
      }
      CliSessionRequest::Load { run_id } => {
        let requested = run_id.as_str();
        let artifact = CliHostArtifact::decode(&self.load_artifact(requested)?)
          .map_err(|_| CliHostError::ReplayRejected)?;
        if artifact.run_id() != requested {
          return Err(CliHostError::ReplayRejected);
        }
        self.history = self.restore_artifact(&artifact)?;
        self.draft = HostDraft {
          message: None,
          plan: None,
          contingency: None,
        };
        self.committed_intent = None;
        Ok(CliHostOutput::Loaded {
          run_id: requested.to_owned(),
          records: self.record_count(),
        })
      }
      CliSessionRequest::Undo => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "undo" });
        }
        if self.draft.is_empty() {
          return Err(CliHostError::NothingToUndo);
        }
        self.draft = HostDraft {
          message: None,
          plan: None,
          contingency: None,
        };
        Ok(CliHostOutput::Undone)
      }
      CliSessionRequest::Quit => {
        self.closed = true;
        Ok(CliHostOutput::Quit)
      }
    }
  }

  fn load_artifact(&self, run_id: &str) -> Result<String, CliHostError<'static>> {
    if let Some(store) = self.store.as_ref() {
      return store.load(run_id).map_err(|error| match error {
        CliRunStoreError::Read {
          kind: std::io::ErrorKind::NotFound,
        } => CliHostError::RunNotFound {
          run_id: run_id.to_owned(),
        },
        _ => CliHostError::StorageUnavailable,
      });
    }
    self
      .saved
      .as_ref()
      .filter(|saved| saved.run_id == run_id)
      .map(|saved| saved.artifact.clone())
      .ok_or_else(|| CliHostError::RunNotFound {
        run_id: run_id.to_owned(),
      })
  }

  fn restore_artifact(
    &self,
    artifact: &CliHostArtifact,
  ) -> Result<LaneScenarioHistory, CliHostError<'static>> {
    if artifact.replay_id() != crate::lane::M2_TWO_WINDOW_REPLAY_ID {
      return Err(CliHostError::ReplayRejected);
    }
    let mut history = LaneScenarioHistory::new(crate::lane::LaneSnapshot::initial())
      .map_err(|_| CliHostError::ReplayRejected)?;
    for record in artifact.records() {
      let index = record.index();
      let inputs = self
        .execution_inputs
        .get(index)
        .copied()
        .ok_or(CliHostError::ReplayRejected)?;
      let state = history.current_state();
      if state.hash() != record.prior_hash() {
        return Err(CliHostError::ReplayRejected);
      }
      let receipt = observe_player(&state, self.next_observation_id_for(&history));
      let request = LaneIntentRequest::new(
        PLAYER_LANER,
        receipt.observation().observation_id(),
        record.intent(),
      );
      let result = history
        .append(&receipt, &request, inputs)
        .map_err(|_| CliHostError::ReplayRejected)?;
      let restored_record = history
        .records()
        .last()
        .ok_or(CliHostError::ReplayRejected)?;
      if result.state_hash() != record.state_hash()
        || crate::lane::lane_record_identity(restored_record.transition()) != record.identity_hash()
      {
        return Err(CliHostError::ReplayRejected);
      }
    }
    history
      .verify_replay()
      .map_err(|_| CliHostError::ReplayRejected)?;
    Ok(history)
  }

  fn advance(&mut self) -> Result<CliHostOutput, CliHostError<'static>> {
    let intent = self
      .committed_intent
      .ok_or(CliHostError::MissingCommittedIntent)?;
    let index = self.history.records().len();
    let inputs = self
      .execution_inputs
      .get(index)
      .copied()
      .ok_or(CliHostError::ScenarioComplete)?;
    let state = self.history.current_state();
    let receipt = observe_player(&state, self.next_observation_id());
    let request =
      LaneIntentRequest::new(PLAYER_LANER, receipt.observation().observation_id(), intent);
    let result = self
      .history
      .append(&receipt, &request, inputs)
      .map_err(|_| CliHostError::AdvanceRejected)?;
    self.committed_intent = None;
    self.draft = HostDraft {
      message: None,
      plan: None,
      contingency: None,
    };
    let window = match index {
      0 => ScenarioWindow::First,
      1 => ScenarioWindow::Second,
      _ => return Err(CliHostError::ScenarioComplete),
    };
    Ok(CliHostOutput::Advanced {
      window,
      outcome: result.outcome(),
    })
  }

  fn next_observation_id(&self) -> ObservationId {
    self.next_observation_id_for(&self.history)
  }

  fn next_observation_id_for(&self, history: &LaneScenarioHistory) -> ObservationId {
    ObservationId::new(
      u64::try_from(history.records().len() + 1).expect("two-window observation count fits in u64"),
    )
  }

  #[cfg(test)]
  pub(crate) fn history_for_artifact_test(&self) -> &LaneScenarioHistory {
    &self.history
  }
}

fn parse_plan_intent(text: &str) -> Option<LaneIntent> {
  match text.trim() {
    "stabilize" => Some(LaneIntent::Stabilize),
    "contest" => Some(LaneIntent::Contest),
    "yield" => Some(LaneIntent::Yield),
    "recall" => Some(LaneIntent::Recall),
    "withdraw" => Some(LaneIntent::Withdraw),
    _ => None,
  }
}

fn fixture_inputs(
  opponent_damage: u8,
  wave_result: LaneWaveResult,
  stream: u8,
) -> LaneResolvedInputs {
  LaneResolvedInputs::new(
    InputTrace::new(StreamId::new(stream), DrawId::new(1)),
    InputTrace::new(StreamId::new(stream), DrawId::new(2)),
    InputTrace::new(StreamId::new(stream), DrawId::new(3)),
    InputTrace::new(StreamId::new(stream), DrawId::new(4)),
    crate::lane::LaneExecutionInputs::new(
      InputTrace::new(StreamId::new(stream), DrawId::new(5)),
      LaneDamage::zero(),
      LaneDamage::new(opponent_damage).expect("fixture damage must be bounded"),
      wave_result,
    ),
  )
}

#[cfg(test)]
fn forced_out_inputs(stream: u8) -> LaneResolvedInputs {
  LaneResolvedInputs::new(
    InputTrace::new(StreamId::new(stream), DrawId::new(1)),
    InputTrace::new(StreamId::new(stream), DrawId::new(2)),
    InputTrace::new(StreamId::new(stream), DrawId::new(3)),
    InputTrace::new(StreamId::new(stream), DrawId::new(4)),
    crate::lane::LaneExecutionInputs::new(
      InputTrace::new(StreamId::new(stream), DrawId::new(5)),
      LaneDamage::new(8).expect("forced-out fixture damage is bounded"),
      LaneDamage::zero(),
      LaneWaveResult::Held,
    ),
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::{AtomicU64, Ordering};

  static NEXT_STORE_ROOT: AtomicU64 = AtomicU64::new(0);

  fn temporary_store_root() -> std::path::PathBuf {
    let id = NEXT_STORE_ROOT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
      "fog-of-intent-host-store-{}-{id}",
      std::process::id()
    ))
  }

  #[test]
  fn fixture_transcript_completes_save_load_replay_and_debrief() {
    assert_eq!(CliScenarioHost::schema(), CLI_HOST_SCHEMA);
    let mut host = CliScenarioHost::fixture();
    let transcript = [
      "observe",
      "message ping ally",
      "plan contest",
      "contingency retreat if threat",
      "undo",
      "plan contest",
      "commit",
      "advance",
      "save first-window",
      "plan stabilize",
      "commit",
      "advance",
      "replay first-window",
      "load first-window",
      "plan stabilize",
      "commit",
      "advance",
      "save complete-run",
      "load complete-run",
      "replay complete-run",
      "debrief",
      "quit",
    ];

    let outputs = transcript
      .into_iter()
      .map(|line| host.apply_line(line).expect("fixture transcript command"))
      .collect::<Vec<_>>();

    assert_eq!(host.record_count(), 2);
    assert!(host.is_complete());
    assert!(matches!(outputs[0], CliHostOutput::Observation(_)));
    assert!(outputs.iter().any(|output| {
      matches!(
        output,
        CliHostOutput::ReplayVerified {
          run_id: Some(run_id),
          records: 2,
        } if run_id == "complete-run"
      )
    }));
    assert!(outputs.iter().any(|output| {
      matches!(
        output,
        CliHostOutput::Loaded {
          run_id,
          records: 1,
        } if run_id == "first-window"
      )
    }));
    assert!(outputs.iter().any(|output| {
      matches!(
        output,
        CliHostOutput::ReplayVerified {
          run_id: Some(run_id),
          records: 1,
        } if run_id == "first-window"
      )
    }));
    assert!(outputs.iter().any(|output| {
      matches!(output, CliHostOutput::Debrief(report) if report.windows().len() == 2)
    }));
    assert!(matches!(outputs.last(), Some(CliHostOutput::Quit)));
  }

  #[test]
  fn actor_observation_projection_matches_host_receipt_without_mutation() {
    let mut host = CliScenarioHost::fixture();
    let initial = host
      .actor_observation()
      .expect("active observation projects");
    assert_eq!(
      initial,
      ActorObservationDto::from_observation(host.observation())
    );
    assert_eq!(initial.schema(), "m5-actor-observation-v1");
    assert!(initial.advertises(crate::protocol::ActorProtocolIntent::Contest));
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.actor_observation(), Ok(initial.clone()));
    assert!(!format!("{initial:?}").contains("hash"));

    host.apply_line("plan contest").expect("plan is staged");
    host.apply_line("commit").expect("plan is committed");
    host.apply_line("advance").expect("first window advances");
    let next = host
      .actor_observation()
      .expect("next active observation projects");
    assert_eq!(
      next,
      ActorObservationDto::from_observation(host.observation())
    );
    assert_ne!(next.observation_id(), initial.observation_id());
    assert_eq!(host.record_count(), 1);

    host
      .apply_line("plan stabilize")
      .expect("second plan is staged");
    host.apply_line("commit").expect("second plan is committed");
    host.apply_line("advance").expect("second window advances");
    assert_eq!(
      host.actor_observation(),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );

    let mut closed = CliScenarioHost::fixture();
    closed.apply_line("quit").expect("host closes");
    assert_eq!(
      closed.actor_observation(),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );
  }

  #[test]
  fn actor_history_projection_tracks_bounded_lifecycle_without_hidden_state() {
    let mut host = CliScenarioHost::fixture();
    assert_eq!(
      host.actor_history(),
      ActorHistoryDto::new(0, ActorHistoryStatus::Open).expect("open history is bounded")
    );
    assert_eq!(
      host.apply_line("inspect history"),
      Ok(CliHostOutput::History {
        records: 0,
        complete: false,
      })
    );

    for command in ["plan contest", "commit", "advance"] {
      host.apply_line(command).expect("first window advances");
    }
    assert_eq!(
      host.actor_history(),
      ActorHistoryDto::new(1, ActorHistoryStatus::Open).expect("next history is bounded")
    );
    assert!(!format!("{:?}", host.actor_history()).contains("hash"));

    for command in ["plan stabilize", "commit", "advance"] {
      host.apply_line(command).expect("second window advances");
    }
    assert_eq!(
      host.actor_history(),
      ActorHistoryDto::new(2, ActorHistoryStatus::Complete).expect("complete history is bounded")
    );
    host.apply_line("quit").expect("complete host closes");
    assert_eq!(
      host.actor_history(),
      ActorHistoryDto::new(2, ActorHistoryStatus::Closed)
        .expect("closed complete history is bounded")
    );

    let mut partially_closed = CliScenarioHost::fixture();
    for command in ["plan contest", "commit", "advance", "quit"] {
      partially_closed
        .apply_line(command)
        .expect("partial host command succeeds");
    }
    assert_eq!(
      partially_closed.actor_history(),
      ActorHistoryDto::new(1, ActorHistoryStatus::Closed)
        .expect("closed partial history is bounded")
    );

    let mut closed = CliScenarioHost::fixture();
    closed.apply_line("quit").expect("host closes");
    assert_eq!(
      closed.actor_history(),
      ActorHistoryDto::new(0, ActorHistoryStatus::Closed).expect("closed history is bounded")
    );
  }

  #[test]
  fn actor_action_validation_is_read_only_and_actor_safe() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.observation();
    let valid = ActorActionDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    );

    assert_eq!(host.validate_actor_action(valid), Ok(()));
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), observation);

    let cases = [
      (
        ActorActionDto::new(2, observation.observation_id().value(), valid.intent()),
        "actor_mismatch",
        "use_bound_actor",
      ),
      (
        ActorActionDto::new(1, observation.observation_id().value() + 1, valid.intent()),
        "stale_observation",
        "request_fresh_observation",
      ),
      (
        ActorActionDto::new(
          observation.observer().value(),
          observation.observation_id().value(),
          crate::protocol::ActorProtocolIntent::Withdraw,
        ),
        "host_validation_rejected",
        "resend_advertised_action",
      ),
    ];
    for (action, code, repair) in cases {
      let error = host
        .validate_actor_action(action)
        .expect_err("invalid actor action is rejected");
      assert_eq!(error.schema(), "m5-actor-error-v2");
      assert_eq!(error.code().id(), code);
      assert_eq!(error.repair().id(), repair);
      assert!(!format!("{error:?}").contains("hash"));
      assert_eq!(host.record_count(), 0);
      assert_eq!(host.observation(), observation);
    }

    for line in [
      "plan contest",
      "commit",
      "advance",
      "plan stabilize",
      "commit",
      "advance",
    ] {
      host.apply_line(line).expect("fixture action advances");
    }
    let closed_observation = host.observation();
    let error = host
      .validate_actor_action(ActorActionDto::new(
        closed_observation.observer().value(),
        closed_observation.observation_id().value(),
        valid.intent(),
      ))
      .expect_err("complete host rejects actor action");
    assert_eq!(error.code().id(), "window_closed");
    assert_eq!(error.repair().id(), "start_new_session");
    assert_eq!(host.record_count(), 2);
  }

  #[test]
  fn actor_action_submission_is_host_owned_and_closes_each_window() {
    let mut host = CliScenarioHost::fixture();
    let first = host.observation();
    let first_action = ActorActionDto::new(
      first.observer().value(),
      first.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    );
    assert!(matches!(
      host
        .submit_actor_action(first_action)
        .expect("first actor action submits"),
      CliHostOutput::Advanced {
        window: ScenarioWindow::First,
        ..
      }
    ));
    assert_eq!(host.record_count(), 1);
    assert_eq!(
      host.submit_actor_action(first_action),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ))
    );

    let second = host.observation();
    let second_action = ActorActionDto::new(
      second.observer().value(),
      second.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    );
    assert!(matches!(
      host
        .submit_actor_action(second_action)
        .expect("second actor action submits"),
      CliHostOutput::Advanced {
        window: ScenarioWindow::Second,
        ..
      }
    ));
    assert!(host.is_complete());
    let closed = host.observation();
    assert_eq!(
      host.submit_actor_action(ActorActionDto::new(
        closed.observer().value(),
        closed.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Contest,
      )),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );

    let mut malformed = CliScenarioHost::new([
      fixture_inputs(8, LaneWaveResult::Advanced, 1),
      fixture_inputs(0, LaneWaveResult::Held, 2),
    ]);
    let malformed_observation = malformed.observation();
    let transition_error = malformed
      .submit_actor_action(ActorActionDto::new(
        malformed_observation.observer().value(),
        malformed_observation.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Contest,
      ))
      .expect_err("malformed execution is redacted");
    assert_eq!(transition_error.code().id(), "host_transition_rejected");
    assert_eq!(transition_error.repair().id(), "start_new_session");
    assert_eq!(malformed.record_count(), 0);
    assert_eq!(malformed.observation(), malformed_observation);
    assert_eq!(
      malformed.apply_line("plan stabilize"),
      Ok(CliHostOutput::DraftStaged { field: "plan" })
    );
    assert!(!format!("{transition_error:?}").contains("health"));
  }

  #[test]
  fn actor_action_result_projection_is_bounded_and_host_owned() {
    let mut host = CliScenarioHost::fixture();
    let first = host.observation();
    let first_action = ActorActionDto::new(
      first.observer().value(),
      first.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    );
    let first_result = host
      .submit_actor_action_result(first_action)
      .expect("first action result projects");
    assert_eq!(
      first_result,
      ActorActionResultDto::new(
        ActorActionResultWindow::First,
        ActorActionResultOutcome::HeldSpace,
      )
    );
    assert_eq!(host.record_count(), 1);
    assert_eq!(
      ActorActionResultDto::decode(&first_result.encode()),
      Ok(first_result)
    );
    assert!(!format!("{first_result:?}").contains("hash"));
    assert_eq!(
      host.submit_actor_action_result(first_action),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ))
    );

    let second = host.observation();
    let second_result = host
      .submit_actor_action_result(ActorActionDto::new(
        second.observer().value(),
        second.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Stabilize,
      ))
      .expect("second action result projects");
    assert_eq!(
      second_result,
      ActorActionResultDto::new(
        ActorActionResultWindow::Second,
        ActorActionResultOutcome::YieldedSpace,
      )
    );
    assert_eq!(host.record_count(), 2);

    let mut forced = CliScenarioHost::new([
      forced_out_inputs(1),
      fixture_inputs(0, LaneWaveResult::Held, 2),
    ]);
    let forced_observation = forced.observation();
    let forced_result = forced
      .submit_actor_action_result(ActorActionDto::new(
        forced_observation.observer().value(),
        forced_observation.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Contest,
      ))
      .expect("forced-out result projects");
    assert_eq!(
      forced_result,
      ActorActionResultDto::new(
        ActorActionResultWindow::First,
        ActorActionResultOutcome::ForcedOut,
      )
    );
  }

  #[test]
  fn actor_debrief_projection_is_completion_gated_and_actor_safe() {
    let mut host = CliScenarioHost::fixture();
    let initial_observation = host.observation();
    assert_eq!(
      host.actor_debrief(),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DebriefUnavailable,
        ActorProtocolRepairHint::AwaitCompletion,
      ))
    );
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), initial_observation);

    for command in [
      "plan contest",
      "commit",
      "advance",
      "plan stabilize",
      "commit",
      "advance",
    ] {
      host.apply_line(command).expect("fixture completes");
    }
    let debrief = host.actor_debrief().expect("complete host has debrief");
    assert_eq!(debrief.schema(), "m5-actor-debrief-v1");
    assert_eq!(debrief.first().window(), ActorActionResultWindow::First);
    assert_eq!(debrief.first().intent().id(), "contest");
    assert_eq!(
      debrief.first().outcome(),
      ActorActionResultOutcome::HeldSpace
    );
    assert_eq!(
      debrief.first().objective(),
      crate::protocol::ActorDebriefObjective::GoalAchieved
    );
    assert_eq!(debrief.second().window(), ActorActionResultWindow::Second);
    assert_eq!(debrief.second().intent().id(), "stabilize");
    assert_eq!(
      debrief.second().outcome(),
      ActorActionResultOutcome::YieldedSpace
    );
    assert_eq!(
      debrief.second().objective(),
      crate::protocol::ActorDebriefObjective::GoalMissed
    );
    assert_eq!(
      debrief.final_objective(),
      crate::protocol::ActorDebriefObjective::GoalMissed
    );
    assert_eq!(
      debrief.attribution_limit(),
      crate::protocol::ActorDebriefAttributionLimit::CommittedFactsOnly
    );
    assert_eq!(ActorDebriefDto::decode(&debrief.encode()), Ok(debrief));
    assert_eq!(host.record_count(), 2);
    assert!(!format!("{debrief:?}").contains("StateHash"));
    assert!(!format!("{debrief:?}").contains("health"));
    assert!(!format!("{debrief:?}").contains("trace"));

    host.apply_line("quit").expect("completed host closes");
    assert_eq!(
      host.actor_debrief(),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );

    let mut closed = CliScenarioHost::fixture();
    closed.apply_line("quit").expect("incomplete host closes");
    assert_eq!(
      closed.actor_debrief(),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );
  }

  #[test]
  fn actor_commit_is_observation_bound_and_does_not_advance_history() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.observation();
    let make_draft = |field, value| {
      ActorDraftDto::new(
        observation.observer().value(),
        observation.observation_id().value(),
        field,
        value,
      )
      .expect("draft value is bounded")
    };
    for (field, value) in [
      (ActorDraftField::Message, "ping ally"),
      (ActorDraftField::Plan, "contest"),
      (ActorDraftField::Contingency, "retreat if threat"),
    ] {
      host
        .stage_actor_draft(make_draft(field, value))
        .expect("draft stages before commit");
    }

    let first_commit = ActorCommitDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    );
    let result = host
      .commit_actor_draft(first_commit)
      .expect("matching actor commit succeeds");
    assert_eq!(
      result,
      ActorCommitResultDto::new(crate::protocol::ActorProtocolIntent::Contest)
    );
    assert_eq!(ActorCommitResultDto::decode(&result.encode()), Ok(result));
    assert!(host.draft.is_empty());
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), observation);
    assert_eq!(
      host.stage_actor_draft(make_draft(ActorDraftField::Message, "too late")),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ))
    );
    assert_eq!(
      host.commit_actor_draft(first_commit),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ))
    );
    host
      .apply_line("advance")
      .expect("host advances committed intent");

    let second = host.observation();
    assert_eq!(
      host.commit_actor_draft(ActorCommitDto::new(
        second.observer().value(),
        observation.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Stabilize,
      )),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ))
    );
    let second_commit = ActorCommitDto::new(
      second.observer().value(),
      second.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    );
    host
      .commit_actor_draft(second_commit)
      .expect("explicit second intent commits without metadata");
    assert_eq!(host.record_count(), 1);
    assert_eq!(host.observation(), second);
    host.apply_line("advance").expect("second commit advances");
    assert!(host.is_complete());
    assert_eq!(
      host.commit_actor_draft(second_commit),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );

    let mut mismatch = CliScenarioHost::fixture();
    let mismatch_observation = mismatch.observation();
    let staged = ActorDraftDto::new(
      mismatch_observation.observer().value(),
      mismatch_observation.observation_id().value(),
      ActorDraftField::Plan,
      "contest",
    )
    .expect("staged plan is bounded");
    mismatch
      .stage_actor_draft(staged)
      .expect("plan stages for mismatch test");
    let mismatch_error = mismatch
      .commit_actor_draft(ActorCommitDto::new(
        mismatch_observation.observer().value(),
        mismatch_observation.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Stabilize,
      ))
      .expect_err("staged plan mismatch is rejected");
    assert_eq!(mismatch_error.code().id(), "host_validation_rejected");
    assert_eq!(mismatch_error.repair().id(), "resend_valid_payload");
    assert_eq!(mismatch.record_count(), 0);
    assert_eq!(mismatch.observation(), mismatch_observation);
    assert_eq!(mismatch.draft.plan.as_deref(), Some("contest"));

    let wrong_actor = ActorCommitDto::new(
      mismatch_observation.observer().value().saturating_add(1),
      mismatch_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    );
    assert_eq!(
      mismatch.commit_actor_draft(wrong_actor),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ))
    );
    let mut closed = CliScenarioHost::fixture();
    closed.apply_line("quit").expect("host closes");
    assert_eq!(
      closed.commit_actor_draft(first_commit),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );
  }

  #[test]
  fn actor_draft_staging_is_observation_bound_and_replaces_fields() {
    let mut host = CliScenarioHost::fixture();
    let observation = host.observation();
    let make_draft = |field, value| {
      ActorDraftDto::new(
        observation.observer().value(),
        observation.observation_id().value(),
        field,
        value,
      )
      .expect("draft value is bounded")
    };

    for (field, value) in [
      (ActorDraftField::Message, "ping ally"),
      (ActorDraftField::Plan, "contest"),
      (ActorDraftField::Contingency, "retreat if threat"),
    ] {
      assert_eq!(
        host.stage_actor_draft(make_draft(field, value)),
        Ok(CliHostOutput::DraftStaged { field: field.id() })
      );
    }
    assert_eq!(
      host.stage_actor_draft(make_draft(ActorDraftField::Plan, "stabilize")),
      Ok(CliHostOutput::DraftStaged { field: "plan" })
    );
    let stale_before_commit = ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value() + 1,
      ActorDraftField::Message,
      "stale",
    )
    .expect("draft value is bounded");
    assert_eq!(
      host.stage_actor_draft(stale_before_commit),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ))
    );
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), observation);
    assert_eq!(
      host.apply_line("commit"),
      Ok(CliHostOutput::Committed {
        intent: LaneIntent::Stabilize,
      })
    );
    assert_eq!(
      host.stage_actor_draft(make_draft(ActorDraftField::Message, "too late")),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ))
    );

    let wrong_actor = ActorDraftDto::new(
      2,
      observation.observation_id().value(),
      ActorDraftField::Message,
      "ping",
    )
    .expect("draft value is bounded");
    assert_eq!(
      host.stage_actor_draft(wrong_actor),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ActorMismatch,
        ActorProtocolRepairHint::UseBoundActor,
      ))
    );
    let stale = ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value() + 1,
      ActorDraftField::Message,
      "stale",
    )
    .expect("draft value is bounded");
    assert_eq!(
      host.stage_actor_draft(stale),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ))
    );

    host.apply_line("advance").expect("first window advances");
    let second = host.observation();
    host
      .submit_actor_action(ActorActionDto::new(
        second.observer().value(),
        second.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Stabilize,
      ))
      .expect("second window closes");
    let complete = host.observation();
    let complete_draft = ActorDraftDto::new(
      complete.observer().value(),
      complete.observation_id().value(),
      ActorDraftField::Message,
      "complete",
    )
    .expect("draft value is bounded");
    assert_eq!(
      host.stage_actor_draft(complete_draft),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::WindowClosed,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );

    let mut closed = CliScenarioHost::fixture();
    let closed_observation = closed.observation();
    closed.apply_line("quit").expect("host closes");
    let closed_draft = ActorDraftDto::new(
      closed_observation.observer().value(),
      closed_observation.observation_id().value(),
      ActorDraftField::Message,
      "closed",
    )
    .expect("draft value is bounded");
    assert_eq!(
      closed.stage_actor_draft(closed_draft),
      Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ))
    );
  }

  #[test]
  fn actor_draft_receipt_acknowledges_existing_staging_without_advancing() {
    let mut host = CliScenarioHost::fixture();
    let first = host.observation();
    let draft = ActorDraftDto::new(
      first.observer().value(),
      first.observation_id().value(),
      ActorDraftField::Plan,
      "contest",
    )
    .expect("draft value is bounded");
    let receipt = host
      .stage_actor_draft_receipt(draft)
      .expect("staging receipt succeeds");
    assert_eq!(
      receipt,
      ActorDraftReceiptDto::new(
        first.observer().value(),
        first.observation_id().value(),
        ActorDraftField::Plan,
      )
    );
    assert_eq!(ActorDraftReceiptDto::decode(&receipt.encode()), Ok(receipt));
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), first);

    host.apply_line("commit").expect("staged plan commits");
    host.apply_line("advance").expect("first window advances");
    let second = host.observation();
    let second_draft = ActorDraftDto::new(
      second.observer().value(),
      second.observation_id().value(),
      ActorDraftField::Contingency,
      "retreat if threat",
    )
    .expect("second draft value is bounded");
    let second_receipt = host
      .stage_actor_draft_receipt(second_draft)
      .expect("second-window receipt succeeds");
    assert_eq!(second_receipt.field(), ActorDraftField::Contingency);
    assert_eq!(
      second_receipt.observation_id(),
      second.observation_id().value()
    );
    assert_eq!(host.record_count(), 1);
    assert_eq!(host.observation(), second);
  }

  #[test]
  fn artifact_restore_rejects_divergent_resolved_inputs() {
    let mut source = CliScenarioHost::fixture();
    for command in ["plan contest", "commit", "advance"] {
      source.apply_line(command).expect("source fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", source.history_for_artifact_test())
      .expect("artifact encodes");

    let mut divergent = CliScenarioHost::new([
      fixture_inputs(2, LaneWaveResult::Advanced, 1),
      fixture_inputs(0, LaneWaveResult::Held, 2),
    ]);
    divergent.saved = Some(SavedRun {
      run_id: "first-window".to_owned(),
      artifact,
    });

    assert_eq!(
      divergent.apply_line("load first-window"),
      Err(CliHostError::ReplayRejected)
    );
  }

  #[test]
  fn artifact_restore_rejects_run_id_mismatch() {
    let mut host = CliScenarioHost::fixture();
    host
      .apply_line("save first-window")
      .expect("empty fixture saves");
    let saved = host.saved.as_mut().expect("saved artifact");
    saved.artifact = saved
      .artifact
      .replace("run_id=first-window", "run_id=other");

    assert_eq!(
      host.apply_line("load first-window"),
      Err(CliHostError::ReplayRejected)
    );
  }

  #[test]
  fn artifact_restore_rejects_valid_intent_tampering() {
    let mut source = CliScenarioHost::fixture();
    for command in ["plan stabilize", "commit", "advance"] {
      source.apply_line(command).expect("source fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", source.history_for_artifact_test())
      .expect("artifact encodes")
      .replace("intent=stabilize", "intent=yield");
    let mut tampered = CliScenarioHost::fixture();
    tampered.saved = Some(SavedRun {
      run_id: "first-window".to_owned(),
      artifact,
    });

    assert_eq!(
      tampered.apply_line("load first-window"),
      Err(CliHostError::ReplayRejected)
    );
  }

  #[test]
  fn artifact_restore_rejects_hash_tampering() {
    let mut source = CliScenarioHost::fixture();
    for command in ["plan contest", "commit", "advance"] {
      source.apply_line(command).expect("source fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", source.history_for_artifact_test())
      .expect("artifact encodes");

    for field in ["prior_hash", "state_hash", "identity_hash"] {
      let mut tampered = CliScenarioHost::fixture();
      tampered.saved = Some(SavedRun {
        run_id: "first-window".to_owned(),
        artifact: replace_artifact_field(&artifact, field, "0"),
      });
      assert_eq!(
        tampered.apply_line("load first-window"),
        Err(CliHostError::ReplayRejected),
        "tampered {field} must fail closed"
      );
    }
  }

  #[test]
  fn file_store_round_trip_survives_a_fresh_host() {
    let root = temporary_store_root();
    let store = CliRunStore::new(&root);
    let mut source = CliScenarioHost::fixture_with_store(store.clone());
    for command in ["plan contest", "commit", "advance", "save first-window"] {
      source.apply_line(command).expect("source store command");
    }
    source
      .apply_line("plan stabilize")
      .expect("second-window draft");
    source.apply_line("commit").expect("second-window commit");
    source.apply_line("advance").expect("second-window advance");

    let mut fresh = CliScenarioHost::fixture_with_store(store);
    assert_eq!(
      fresh.apply_line("load first-window"),
      Ok(CliHostOutput::Loaded {
        run_id: "first-window".to_owned(),
        records: 1
      })
    );
    assert_eq!(fresh.record_count(), 1);
    assert_eq!(
      fresh.apply_line("replay first-window"),
      Ok(CliHostOutput::ReplayVerified {
        run_id: Some("first-window".to_owned()),
        records: 1
      })
    );
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn file_store_failure_is_bounded_at_the_host_boundary() {
    let root = temporary_store_root();
    std::fs::write(&root, "not a directory").expect("root fixture");
    let mut host = CliScenarioHost::fixture_with_store(CliRunStore::new(&root));
    assert_eq!(
      host.apply_line("save run"),
      Err(CliHostError::StorageUnavailable)
    );
    let _ = std::fs::remove_file(root);
  }

  #[test]
  fn file_store_tampering_is_rejected_before_history_replacement() {
    let root = temporary_store_root();
    let store = CliRunStore::new(&root);
    let mut source = CliScenarioHost::fixture_with_store(store.clone());
    source.apply_line("save run").expect("save fixture");
    std::fs::write(root.join("run.foi-artifact"), "malformed").expect("tampered artifact");

    let mut fresh = CliScenarioHost::fixture_with_store(store);
    fresh.apply_line("plan contest").expect("local plan");
    fresh.apply_line("commit").expect("local commit");
    fresh.apply_line("advance").expect("local advance");
    let before = fresh.observation();
    assert_eq!(fresh.record_count(), 1);
    assert_eq!(
      fresh.apply_line("load run"),
      Err(CliHostError::ReplayRejected)
    );
    assert_eq!(fresh.record_count(), 1);
    assert_eq!(fresh.observation(), before);
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn host_rejects_invalid_plan_and_pre_host_errors() {
    let mut host = CliScenarioHost::fixture();
    assert_eq!(
      host.apply_line("plan ???"),
      Ok(CliHostOutput::DraftStaged { field: "plan" })
    );
    assert_eq!(
      host.apply_line("commit"),
      Err(CliHostError::InvalidPlan {
        text: "???".to_owned(),
      })
    );
    assert_eq!(
      host.apply_line("advance"),
      Err(CliHostError::MissingCommittedIntent)
    );
    host.apply_line("plan contest").expect("valid plan staging");
    host.apply_line("commit").expect("valid commit");
    for (line, verb) in [
      ("plan stabilize", "plan"),
      ("message late", "message"),
      ("contingency late", "contingency"),
      ("commit", "commit"),
      ("undo", "undo"),
    ] {
      assert_eq!(
        host.apply_line(line),
        Err(CliHostError::CommittedBoundary { verb })
      );
    }
    host.apply_line("advance").expect("first window advances");
    host
      .apply_line("plan stabilize")
      .expect("next-window plan staging");
    host.apply_line("commit").expect("next-window commit");
    host.apply_line("advance").expect("second window advances");
    assert_eq!(
      host.apply_line("load missing"),
      Err(CliHostError::RunNotFound {
        run_id: "missing".to_owned(),
      })
    );
    assert_eq!(
      host.apply_line("branch point-0"),
      Err(CliHostError::BranchUnavailable)
    );
  }

  #[test]
  fn branch_review_is_read_only_and_preserves_parent_artifact() {
    let root = temporary_store_root();
    let mut host = CliScenarioHost::fixture_with_store(CliRunStore::new(&root));
    for command in ["plan contest", "commit", "advance", "save parent"] {
      host.apply_line(command).expect("parent command");
    }
    let before_observation = host.observation();
    let before_artifact = host.saved.as_ref().expect("parent saved").artifact.clone();

    host.apply_line("plan yield").expect("alternate plan");
    assert!(matches!(
      host.apply_line("branch first"),
      Ok(CliHostOutput::Branched {
        point_id,
        parent_intent: LaneIntent::Contest,
        branch_intent: LaneIntent::Yield,
        execution_relation: LaneExecutionRelation::Matched,
        ..
      }) if point_id == "first"
    ));
    assert_eq!(host.record_count(), 1);
    assert_eq!(host.observation(), before_observation);
    assert_eq!(
      host.saved.as_ref().expect("parent saved").artifact,
      before_artifact
    );
    assert_eq!(
      host.apply_line("replay"),
      Ok(CliHostOutput::ReplayVerified {
        run_id: None,
        records: 1
      })
    );

    host
      .apply_line("plan stabilize")
      .expect("second alternate plan");
    assert!(matches!(
      host.apply_line("branch"),
      Ok(CliHostOutput::Branched {
        point_id,
        branch_intent: LaneIntent::Stabilize,
        execution_relation: LaneExecutionRelation::Matched,
        ..
      }) if point_id == "first"
    ));
    assert_eq!(
      host.apply_line("load parent"),
      Ok(CliHostOutput::Loaded {
        run_id: "parent".to_owned(),
        records: 1
      })
    );
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn branch_rejects_missing_invalid_same_and_unsupported_requests() {
    let mut host = CliScenarioHost::fixture();
    assert_eq!(
      host.apply_line("branch first"),
      Err(CliHostError::BranchUnavailable)
    );
    for command in ["plan contest", "commit", "advance"] {
      host.apply_line(command).expect("parent command");
    }
    assert_eq!(
      host.apply_line("branch first"),
      Err(CliHostError::BranchMissingPlan)
    );
    host.apply_line("plan ???").expect("invalid alternate plan");
    assert_eq!(
      host.apply_line("branch first"),
      Err(CliHostError::InvalidPlan {
        text: "???".to_owned()
      })
    );
    host
      .apply_line("plan contest")
      .expect("same alternate plan");
    assert_eq!(
      host.apply_line("branch first"),
      Err(CliHostError::BranchUnavailable)
    );
    host.apply_line("plan yield").expect("valid alternate plan");
    assert_eq!(
      host.apply_line("branch second"),
      Err(CliHostError::BranchUnavailable)
    );
    host.apply_line("plan yield").expect("valid alternate plan");
    host.apply_line("branch first").expect("valid branch");
    host
      .apply_line("plan stabilize")
      .expect("second-window plan");
    host.apply_line("commit").expect("second-window commit");
    host.apply_line("advance").expect("second-window advance");
    assert_eq!(
      host.apply_line("branch first"),
      Err(CliHostError::BranchUnavailable)
    );
  }

  fn replace_artifact_field(artifact: &str, field: &str, value: &str) -> String {
    artifact
      .lines()
      .map(|line| {
        line
          .split_whitespace()
          .map(|word| {
            if word.starts_with(&format!("{field}=")) {
              format!("{field}={value}")
            } else {
              word.to_owned()
            }
          })
          .collect::<Vec<_>>()
          .join(" ")
      })
      .collect::<Vec<_>>()
      .join("\n")
  }

  #[test]
  fn malformed_resolved_inputs_return_redacted_host_errors() {
    let mut host = CliScenarioHost::new([
      fixture_inputs(8, LaneWaveResult::Advanced, 3),
      fixture_inputs(0, LaneWaveResult::Held, 4),
    ]);
    host.apply_line("plan contest").expect("plan staging");
    host.apply_line("commit").expect("commit");
    let error = host
      .apply_line("advance")
      .expect_err("malformed fixture input must fail closed");
    assert_eq!(error, CliHostError::AdvanceRejected);
    let debug = format!("{error:?}");
    assert!(!debug.contains("OpponentDamageExceedsHealth"));
    assert!(!debug.contains("health"));
    assert!(!debug.contains("state_hash"));
  }

  #[test]
  fn identical_fixture_transcripts_have_identical_actor_outputs() {
    let run = |host: &mut CliScenarioHost| {
      [
        "plan contest",
        "commit",
        "advance",
        "plan stabilize",
        "commit",
        "advance",
      ]
      .into_iter()
      .map(|line| host.apply_line(line).expect("deterministic command"))
      .collect::<Vec<_>>()
    };
    assert_eq!(
      run(&mut CliScenarioHost::fixture()),
      run(&mut CliScenarioHost::fixture())
    );
  }
}
