//! Synchronous scenario host implementation.

use super::types::{CLI_HOST_SCHEMA, CliHostError, CliHostOutput, HostDraft, SavedRun};
use crate::cli::{
  CliCommand, CliProcessRequest, CliReadRequest, CliRunId, CliSessionRequest, CliWriteRequest,
  parse_command, process_request, read_request, session_request, write_request,
};
use crate::host_artifact::CliHostArtifact;
use crate::kernel::{DrawId, InputTrace, StreamId};
use crate::lane::{
  BranchExecutionSelection, LaneDamage, LaneHistory, LaneIntent, LaneIntentRequest, LaneOutcome,
  LaneResolvedInputs, LaneScenarioHistory, LaneWaveResult, ObservationId, PLAYER_LANER,
  ScenarioWindow, StrategyFixtureId, branch_from_window, build_scenario_debrief, observe_player,
  strategy_fixture,
};
use crate::protocol::{
  ActorActionDto, ActorActionResultDto, ActorActionResultOutcome, ActorActionResultWindow,
  ActorCommitDto, ActorCommitResultDto, ActorDebriefDto, ActorDebriefObjective, ActorDraftClearDto,
  ActorDraftClearReceiptDto, ActorDraftCommitReceiptDto, ActorDraftDto, ActorDraftField,
  ActorDraftPresence, ActorDraftReceiptDto, ActorDraftStatusDto, ActorHistoryDto,
  ActorHistoryStatus, ActorObservationDto, ActorProtocolError, ActorProtocolErrorCode,
  ActorProtocolRepairHint, ActorReplayDebriefRecordDto, ActorReplayDto, ActorReplayRecordDto,
};
use crate::run_store::{CliRunStore, CliRunStoreError};

/// A bounded host for the existing deterministic two-window lane scenario.
///
/// `execution_inputs` are already resolved at construction. The host never
/// creates random values and never returns a true-state snapshot to callers.
pub struct CliScenarioHost {
  pub(crate) history: LaneScenarioHistory,
  pub(crate) execution_inputs: [LaneResolvedInputs; 2],
  pub(crate) draft: HostDraft,
  pub(crate) protocol_draft: HostDraft,
  pub(crate) committed_intent: Option<LaneIntent>,
  pub(crate) saved: Option<SavedRun>,
  pub(crate) store: Option<CliRunStore>,
  pub(crate) closed: bool,
}

impl CliScenarioHost {
  /// Build a host with explicit inputs for the first and second windows.
  pub fn new(execution_inputs: [LaneResolvedInputs; 2]) -> Self {
    Self {
      history: LaneScenarioHistory::new(crate::lane::LaneSnapshot::initial())
        .expect("initial lane fixture must be valid"),
      execution_inputs,
      draft: HostDraft::empty(),
      protocol_draft: HostDraft::empty(),
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

  /// Build a host configured for one of the canonical strategy playthroughs.
  pub fn strategy(id: StrategyFixtureId) -> Self {
    let fixture = strategy_fixture(id).expect("canonical strategy fixture must be valid");
    let first_inputs = fixture.lane_inputs();
    let second_inputs = fixture_inputs(0, LaneWaveResult::Held, 2);
    Self::new([first_inputs, second_inputs])
  }

  /// Build a strategy scenario host backed by an explicit artifact store.
  pub fn strategy_with_store(id: StrategyFixtureId, store: CliRunStore) -> Self {
    let mut host = Self::strategy(id);
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

  /// Verify current immutable history and return only bounded actor-safe status.
  pub fn actor_replay(&self) -> Result<ActorReplayDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    self.history.verify_replay().map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    ActorReplayDto::new(self.record_count()).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })
  }

  /// Verify current history and return only bounded categorical replay records.
  pub fn actor_replay_records(&self) -> Result<Vec<ActorReplayRecordDto>, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    self.history.verify_replay().map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    Ok(actor_replay_record_dtos(&self.history))
  }

  /// Load and replay one validated saved run before projecting categorical records.
  pub fn actor_replay_records_from_run(
    &self,
    run_id: CliRunId<'_>,
  ) -> Result<Vec<ActorReplayRecordDto>, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let artifact = CliHostArtifact::decode(&self.load_artifact(run_id.as_str()).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?)
    .map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    if artifact.run_id() != run_id.as_str() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let history = self.restore_artifact(&artifact).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    history.verify_replay().map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    Ok(actor_replay_record_dtos(&history))
  }

  /// Return verified categorical debrief records for a completed host.
  pub fn actor_replay_debrief_records(
    &self,
  ) -> Result<Vec<ActorReplayDebriefRecordDto>, ActorProtocolError> {
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
    actor_replay_debrief_record_dtos(&self.history).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })
  }

  /// Load a validated complete run before projecting categorical debrief records.
  pub fn actor_replay_debrief_records_from_run(
    &self,
    run_id: CliRunId<'_>,
  ) -> Result<Vec<ActorReplayDebriefRecordDto>, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let artifact = CliHostArtifact::decode(&self.load_artifact(run_id.as_str()).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?)
    .map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    if artifact.run_id() != run_id.as_str() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let history = self.restore_artifact(&artifact).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    if history.records().len() != 2 {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DebriefUnavailable,
        ActorProtocolRepairHint::AwaitCompletion,
      ));
    }
    actor_replay_debrief_record_dtos(&history).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })
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
      ActorDraftField::Message => {
        self.draft.message = Some(draft.value().to_owned());
        self.protocol_draft.message = Some(draft.value().to_owned());
      }
      ActorDraftField::Plan => {
        self.draft.plan = Some(draft.value().to_owned());
        self.protocol_draft.plan = Some(draft.value().to_owned());
      }
      ActorDraftField::Contingency => {
        self.draft.contingency = Some(draft.value().to_owned());
        self.protocol_draft.contingency = Some(draft.value().to_owned());
      }
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

  /// Return actor-protocol-staged metadata without mutating host state.
  pub fn actor_draft(&self) -> Result<Vec<ActorDraftDto>, ActorProtocolError> {
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
    if self.committed_intent.is_some() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DraftBoundary,
        ActorProtocolRepairHint::AwaitNextObservation,
      ));
    }
    let receipt = observe_player(&self.history.current_state(), self.next_observation_id());
    let observer = receipt.observation().observer().value();
    let observation_id = receipt.observation().observation_id().value();
    let mut drafts = Vec::new();
    for (field, value) in [
      (
        ActorDraftField::Message,
        self.protocol_draft.message.as_deref(),
      ),
      (ActorDraftField::Plan, self.protocol_draft.plan.as_deref()),
      (
        ActorDraftField::Contingency,
        self.protocol_draft.contingency.as_deref(),
      ),
    ] {
      if let Some(value) = value {
        drafts.push(
          ActorDraftDto::new(observer, observation_id, field, value).map_err(|_| {
            ActorProtocolError::new(
              ActorProtocolErrorCode::HostTransitionRejected,
              ActorProtocolRepairHint::StartNewSession,
            )
          })?,
        );
      }
    }
    Ok(drafts)
  }

  /// Return aggregate presence for the active actor draft without payloads.
  pub fn actor_draft_status(&self) -> Result<ActorDraftStatusDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let receipt = observe_player(&self.history.current_state(), self.next_observation_id());
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
    let presence = |value: &Option<String>| {
      if value.is_some() {
        ActorDraftPresence::Present
      } else {
        ActorDraftPresence::Absent
      }
    };
    Ok(ActorDraftStatusDto::new(
      receipt.observation().observer().value(),
      receipt.observation().observation_id().value(),
      presence(&self.draft.message),
      presence(&self.draft.plan),
      presence(&self.draft.contingency),
    ))
  }

  /// Clear the active actor draft and report only pre-clear field presence.
  pub fn clear_actor_draft(
    &mut self,
    clear: ActorDraftClearDto,
  ) -> Result<ActorDraftClearReceiptDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let receipt = observe_player(&self.history.current_state(), self.next_observation_id());
    if clear.observer() != receipt.observation().observer().value() {
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
    if clear.observation_id() != receipt.observation().observation_id().value() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::StaleObservation,
        ActorProtocolRepairHint::RequestFreshObservation,
      ));
    }
    let presence = |value: &Option<String>| {
      if value.is_some() {
        ActorDraftPresence::Present
      } else {
        ActorDraftPresence::Absent
      }
    };
    let result = ActorDraftClearReceiptDto::new(
      clear.observer(),
      clear.observation_id(),
      presence(&self.draft.message),
      presence(&self.draft.plan),
      presence(&self.draft.contingency),
    );
    self.clear_drafts();
    Ok(result)
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
    self.clear_drafts();
    Ok(ActorCommitResultDto::new(commit.intent()))
  }

  /// Commit one actor intent and report only which draft fields were accepted.
  pub fn commit_actor_draft_receipt(
    &mut self,
    commit: ActorCommitDto,
  ) -> Result<ActorDraftCommitReceiptDto, ActorProtocolError> {
    let message = if self.draft.message.is_some() {
      ActorDraftPresence::Present
    } else {
      ActorDraftPresence::Absent
    };
    let plan = if self.draft.plan.is_some() {
      ActorDraftPresence::Present
    } else {
      ActorDraftPresence::Absent
    };
    let contingency = if self.draft.contingency.is_some() {
      ActorDraftPresence::Present
    } else {
      ActorDraftPresence::Absent
    };
    let observer = commit.observer();
    let observation_id = commit.observation_id();
    let intent = commit.intent();
    self.commit_actor_draft(commit)?;
    Ok(ActorDraftCommitReceiptDto::new(
      observer,
      observation_id,
      intent,
      message,
      plan,
      contingency,
    ))
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

  /// Load a validated complete run before projecting the actor debrief summary.
  pub fn actor_debrief_from_run(
    &self,
    run_id: CliRunId<'_>,
  ) -> Result<ActorDebriefDto, ActorProtocolError> {
    if self.closed {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::ClosedSession,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let artifact = CliHostArtifact::decode(&self.load_artifact(run_id.as_str()).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?)
    .map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    if artifact.run_id() != run_id.as_str() {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      ));
    }
    let history = self.restore_artifact(&artifact).map_err(|_| {
      ActorProtocolError::new(
        ActorProtocolErrorCode::HostTransitionRejected,
        ActorProtocolRepairHint::StartNewSession,
      )
    })?;
    if history.records().len() != 2 {
      return Err(ActorProtocolError::new(
        ActorProtocolErrorCode::DebriefUnavailable,
        ActorProtocolRepairHint::AwaitCompletion,
      ));
    }
    let report = build_scenario_debrief(&history)
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

  /// Return actor-safe session chrome without inspecting true state.
  pub fn session_view(&self) -> crate::host::CliSessionView {
    let window = match self.history.records().len() {
      0 => crate::host::CliSessionWindow::First,
      1 => crate::host::CliSessionWindow::Second,
      _ => crate::host::CliSessionWindow::Complete,
    };
    let mut draft_fields = Vec::new();
    if self.draft.plan.is_some() {
      draft_fields.push("plan");
    }
    if self.draft.message.is_some() {
      draft_fields.push("message");
    }
    if self.draft.contingency.is_some() {
      draft_fields.push("contingency");
    }
    crate::host::CliSessionView::new(
      window,
      self.record_count(),
      draft_fields,
      self.committed_intent,
      self.store.is_some(),
      suggested_next(
        window,
        self.committed_intent.is_some(),
        self.draft.plan.is_some(),
      ),
    )
  }

  /// Return current staged draft field values (message, plan, contingency).
  pub fn staged_draft(&self) -> (Option<&str>, Option<&str>, Option<&str>) {
    (
      self.draft.message.as_deref(),
      self.draft.plan.as_deref(),
      self.draft.contingency.as_deref(),
    )
  }

  /// Apply one parsed-and-mapped CLI line at the host boundary.
  pub fn apply_line<'a>(&mut self, line: &'a str) -> Result<CliHostOutput, CliHostError<'a>> {
    if self.closed {
      return Err(CliHostError::Closed);
    }
    let command = parse_command(line).map_err(CliHostError::Parse)?;
    match command {
      CliCommand::Help(_) | CliCommand::Observe | CliCommand::Inspect(_) => {
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

  fn apply_read<'a>(&self, request: CliReadRequest<'a>) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliReadRequest::Help { topic: None } => Ok(CliHostOutput::Help { topic: None }),
      CliReadRequest::Help { topic: Some(name) } => crate::cli::help_catalog()
        .entry(name)
        .map(|entry| CliHostOutput::Help {
          topic: Some(entry.name),
        })
        .ok_or_else(|| CliHostError::UnknownHelpTopic {
          topic: name.to_owned(),
        }),
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
        self.protocol_draft.message = None;
        self.draft.message = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged { field: "message" })
      }
      CliWriteRequest::Plan { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "plan" });
        }
        self.protocol_draft.plan = None;
        self.draft.plan = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged { field: "plan" })
      }
      CliWriteRequest::Contingency { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary {
            verb: "contingency",
          });
        }
        self.protocol_draft.contingency = None;
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
        self.clear_drafts();
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
    let (target_index, canonical_point_id) = match point_id {
      None => {
        if self.history.records().is_empty() {
          return Err(CliHostError::BranchUnavailable);
        }
        let index = self.history.records().len() - 1;
        let label = if index == 0 { "first" } else { "second" };
        (index, label.to_string())
      }
      Some(id) => {
        let trimmed = id.trim().to_ascii_lowercase();
        match trimmed.as_str() {
          "first" | "1" | "0" | "rec-0" | "window-1" | "w1" => (0, "first".to_string()),
          "second" | "2" | "rec-1" | "window-2" | "w2" => (1, "second".to_string()),
          _ => return Err(CliHostError::BranchUnavailable),
        }
      }
    };
    if target_index >= self.history.records().len() {
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
      .get(target_index)
      .ok_or(CliHostError::BranchUnavailable)?;
    let transition = scenario_record.transition().clone();
    let parent_intent = transition.command().intent();
    let mut parent = LaneHistory::new(scenario_record.start_state())
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
      point_id: canonical_point_id,
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
        self.clear_drafts();
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
        self.clear_drafts();
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
    let state = self.history.current_state();
    let observation_id = self.next_observation_id();
    let receipt = observe_player(&state, observation_id);
    let obs = receipt.observation();
    let legal_intent_count: u8 = if obs.available_threat_response().is_some() {
      5
    } else {
      4
    };

    let condition = state.window().advance_condition();
    let decision = condition.evaluate(self.committed_intent.is_some(), legal_intent_count);
    if decision != crate::lane::LaneAdvanceDecision::AdvanceAutomatically {
      return Err(CliHostError::MissingCommittedIntent);
    }

    let intent = self
      .committed_intent
      .ok_or(CliHostError::MissingCommittedIntent)?;
    let index = self.history.records().len();
    let inputs = self
      .execution_inputs
      .get(index)
      .copied()
      .ok_or(CliHostError::ScenarioComplete)?;
    let request =
      LaneIntentRequest::new(PLAYER_LANER, receipt.observation().observation_id(), intent);
    let result = self
      .history
      .append(&receipt, &request, inputs)
      .map_err(|_| CliHostError::AdvanceRejected)?;
    self.committed_intent = None;
    self.clear_drafts();
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

  fn clear_drafts(&mut self) {
    self.draft = HostDraft::empty();
    self.protocol_draft = HostDraft::empty();
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

fn suggested_next(
  window: crate::host::CliSessionWindow,
  committed: bool,
  plan_staged: bool,
) -> Vec<&'static str> {
  match window {
    crate::host::CliSessionWindow::Complete => {
      vec!["debrief", "review", "replay", "quit"]
    }
    _ if committed => vec!["advance", "observe", "quit"],
    crate::host::CliSessionWindow::Second if plan_staged => {
      vec!["commit", "branch", "undo", "observe"]
    }
    _ if plan_staged => vec!["commit", "undo", "observe"],
    _ => vec!["observe", "plan", "commit"],
  }
}

pub(crate) fn parse_plan_intent(text: &str) -> Option<LaneIntent> {
  match text.trim() {
    "stabilize" => Some(LaneIntent::Stabilize),
    "contest" => Some(LaneIntent::Contest),
    "yield" => Some(LaneIntent::Yield),
    "recall" => Some(LaneIntent::Recall),
    "withdraw" => Some(LaneIntent::Withdraw),
    _ => None,
  }
}

pub(crate) fn actor_replay_record_dtos(history: &LaneScenarioHistory) -> Vec<ActorReplayRecordDto> {
  history
    .records()
    .iter()
    .map(|record| {
      let window = match record.window() {
        ScenarioWindow::First => ActorActionResultWindow::First,
        ScenarioWindow::Second => ActorActionResultWindow::Second,
      };
      let intent = match record.transition().command().intent() {
        LaneIntent::Stabilize => crate::protocol::ActorProtocolIntent::Stabilize,
        LaneIntent::Contest => crate::protocol::ActorProtocolIntent::Contest,
        LaneIntent::Yield => crate::protocol::ActorProtocolIntent::Yield,
        LaneIntent::Recall => crate::protocol::ActorProtocolIntent::Recall,
        LaneIntent::Withdraw => crate::protocol::ActorProtocolIntent::Withdraw,
      };
      let outcome = match record.transition().result().outcome() {
        LaneOutcome::HeldSpace => ActorActionResultOutcome::HeldSpace,
        LaneOutcome::YieldedSpace => ActorActionResultOutcome::YieldedSpace,
        LaneOutcome::ForcedOut => ActorActionResultOutcome::ForcedOut,
      };
      ActorReplayRecordDto::new(window, intent, outcome)
    })
    .collect()
}

pub(crate) fn actor_replay_debrief_record_dtos(
  history: &LaneScenarioHistory,
) -> Result<Vec<ActorReplayDebriefRecordDto>, ()> {
  let report = build_scenario_debrief(history).map_err(|_| ())?.report();
  Ok(
    report
      .windows()
      .into_iter()
      .map(|window| {
        let window_id = match window.window() {
          ScenarioWindow::First => ActorActionResultWindow::First,
          ScenarioWindow::Second => ActorActionResultWindow::Second,
        };
        let intent = match window.intent() {
          LaneIntent::Stabilize => crate::protocol::ActorProtocolIntent::Stabilize,
          LaneIntent::Contest => crate::protocol::ActorProtocolIntent::Contest,
          LaneIntent::Yield => crate::protocol::ActorProtocolIntent::Yield,
          LaneIntent::Recall => crate::protocol::ActorProtocolIntent::Recall,
          LaneIntent::Withdraw => crate::protocol::ActorProtocolIntent::Withdraw,
        };
        let outcome = match window.outcome() {
          LaneOutcome::HeldSpace => ActorActionResultOutcome::HeldSpace,
          LaneOutcome::YieldedSpace => ActorActionResultOutcome::YieldedSpace,
          LaneOutcome::ForcedOut => ActorActionResultOutcome::ForcedOut,
        };
        let objective = match window.objective() {
          crate::lane::ObjectiveDisposition::GoalAchieved => ActorDebriefObjective::GoalAchieved,
          crate::lane::ObjectiveDisposition::GoalPartiallyAchieved => {
            ActorDebriefObjective::GoalPartiallyAchieved
          }
          crate::lane::ObjectiveDisposition::GoalMissed => ActorDebriefObjective::GoalMissed,
        };
        ActorReplayDebriefRecordDto::new(window_id, intent, outcome, objective)
      })
      .collect(),
  )
}

pub(crate) fn fixture_inputs(
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
pub(crate) fn forced_out_inputs(stream: u8) -> LaneResolvedInputs {
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
