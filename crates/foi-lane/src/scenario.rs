use super::*;

pub const M2_TWO_WINDOW_REPLAY_ID: &str = "m2-two-window-scenario-v3";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScenarioWindow {
  First,
  Second,
}

impl ScenarioWindow {
  fn from_index(index: usize) -> Option<Self> {
    match index {
      0 => Some(Self::First),
      1 => Some(Self::Second),
      _ => None,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneScenarioRecord {
  pub(crate) window: ScenarioWindow,
  pub(crate) start_state: LaneSnapshot,
  pub(crate) transition: LaneTransitionRecord,
  pub(crate) reopened_state: Option<LaneSnapshot>,
}

impl LaneScenarioRecord {
  pub fn window(&self) -> ScenarioWindow {
    self.window
  }

  pub fn start_state(&self) -> LaneSnapshot {
    self.start_state
  }

  pub fn transition(&self) -> &LaneTransitionRecord {
    &self.transition
  }

  pub fn reopened_state(&self) -> Option<LaneSnapshot> {
    self.reopened_state
  }
}

#[derive(Clone)]
pub struct LaneScenarioHistory {
  pub(crate) replay_id: &'static str,
  pub(crate) initial_state: LaneSnapshot,
  pub(crate) current_state: LaneSnapshot,
  pub(crate) records: Vec<LaneScenarioRecord>,
}

impl LaneScenarioHistory {
  pub fn new(initial_state: LaneSnapshot) -> Result<Self, ScenarioError> {
    if !initial_state.is_valid_lane_state() || initial_state.phase() != LanePhase::Open {
      return Err(ScenarioError::InvalidInitialState);
    }
    Ok(Self {
      replay_id: M2_TWO_WINDOW_REPLAY_ID,
      initial_state,
      current_state: initial_state,
      records: Vec::new(),
    })
  }

  pub fn replay_id(&self) -> &'static str {
    self.replay_id
  }

  pub fn initial_state(&self) -> LaneSnapshot {
    self.initial_state
  }

  pub fn current_state(&self) -> LaneSnapshot {
    self.current_state
  }

  pub fn records(&self) -> &[LaneScenarioRecord] {
    &self.records
  }

  pub fn terminal_state(&self) -> Result<LaneSnapshot, ScenarioError> {
    if self.records.len() != 2 {
      return Err(ScenarioError::ScenarioIncomplete);
    }
    Ok(self.current_state)
  }

  pub fn append(
    &mut self,
    receipt: &LaneObservationReceipt,
    request: &LaneIntentRequest,
    inputs: LaneResolvedInputs,
  ) -> Result<LaneTransitionResult, ScenarioError> {
    let index = self.records.len();
    let window = ScenarioWindow::from_index(index).ok_or(ScenarioError::ScenarioComplete)?;
    let start_state = self.current_state;
    if start_state.phase() != LanePhase::Open {
      return Err(ScenarioError::WindowNotOpen);
    }
    let validated = validate_lane_request(&start_state, receipt, request)
      .map_err(|error| ScenarioError::Validation { window, error })?;
    let result = transition_lane(&start_state, &validated, &inputs)
      .map_err(|error| ScenarioError::Transition { window, error })?;
    let reopened_state = if window == ScenarioWindow::First {
      Some(reopen_lane_window(&result)?)
    } else {
      None
    };
    self.current_state = reopened_state.unwrap_or_else(|| result.next_state());
    self.records.push(LaneScenarioRecord {
      window,
      start_state,
      transition: LaneTransitionRecord {
        replay_id: M2_REPLAY_ID,
        observation: receipt.observation,
        command: validated.command,
        inputs,
        prior_state_hash: start_state.hash(),
        result: result.clone(),
      },
      reopened_state,
    });
    Ok(result)
  }

  #[doc(hidden)]
  pub fn tamper_replay_id_for_test(&mut self, replay_id: &'static str) {
    self.replay_id = replay_id;
  }

  pub fn verify_replay(&self) -> Result<LaneSnapshot, ScenarioError> {
    if self.replay_id != M2_TWO_WINDOW_REPLAY_ID || self.records.len() > 2 {
      return Err(ScenarioError::ReplayMismatch);
    }
    let mut state = self.initial_state;
    for (index, record) in self.records.iter().enumerate() {
      let expected_window =
        ScenarioWindow::from_index(index).ok_or(ScenarioError::ReplayMismatch)?;
      if record.window != expected_window || record.start_state != state {
        return Err(ScenarioError::ReplayMismatch);
      }
      let receipt = observe_player(&state, record.transition.command.observation_id);
      if receipt.observation != record.transition.observation {
        return Err(ScenarioError::ReplayMismatch);
      }
      let validated = validate_lane_command(&state, &receipt, &record.transition.command)
        .map_err(|_| ScenarioError::ReplayMismatch)?;
      let result = transition_lane(&state, &validated, &record.transition.inputs)
        .map_err(|_| ScenarioError::ReplayMismatch)?;
      let expected_reopened = if expected_window == ScenarioWindow::First {
        Some(reopen_lane_window(&result)?)
      } else {
        None
      };
      let expected_record = LaneTransitionRecord {
        replay_id: M2_REPLAY_ID,
        observation: receipt.observation,
        command: validated.command,
        inputs: record.transition.inputs,
        prior_state_hash: state.hash(),
        result: result.clone(),
      };
      if record.transition != expected_record || record.reopened_state != expected_reopened {
        return Err(ScenarioError::ReplayMismatch);
      }
      state = expected_reopened.unwrap_or_else(|| result.next_state());
    }
    if state != self.current_state {
      return Err(ScenarioError::ReplayMismatch);
    }
    Ok(state)
  }
}

pub fn reopen_lane_window(result: &LaneTransitionResult) -> Result<LaneSnapshot, ScenarioError> {
  let resolved = result.next_state();
  if result.state_hash() != resolved.hash()
    || result.outcome()
      != resolved
        .terminal_outcome()
        .ok_or(ScenarioError::InvalidReopenState)?
  {
    return Err(ScenarioError::InvalidReopenState);
  }
  reopen_resolved_snapshot(&resolved)
}

pub(crate) fn reopen_resolved_snapshot(
  resolved: &LaneSnapshot,
) -> Result<LaneSnapshot, ScenarioError> {
  if !resolved.is_valid_lane_state()
    || resolved.phase() != LanePhase::Resolved
    || resolved.terminal_outcome().is_none()
  {
    return Err(ScenarioError::InvalidReopenState);
  }
  Ok(LaneSnapshot::new_with_delayed_effects(
    resolved.ruleset,
    resolved.turn,
    resolved.window,
    LaneStatus::Open,
    resolved.player,
    resolved.opponent,
    resolved.wave,
    resolved.jungle_threat,
    resolved.delayed_effects,
  ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScenarioError {
  InvalidInitialState,
  InvalidReopenState,
  WindowNotOpen,
  ScenarioComplete,
  ScenarioIncomplete,
  Validation {
    window: ScenarioWindow,
    error: LaneValidationError,
  },
  Transition {
    window: ScenarioWindow,
    error: LaneTransitionError,
  },
  ReplayMismatch,
}

pub const M2_FINAL_DEBRIEF_REPLAY_ID: &str = "m2-two-window-final-debrief-v3";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FinalDebriefAttributionLimit {
  CommittedHistoryFactsOnly,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WindowDebriefSummary {
  pub(crate) window: ScenarioWindow,
  pub(crate) intent: LaneIntent,
  pub(crate) outcome: LaneOutcome,
  pub(crate) player_health: LaneHealth,
  pub(crate) player_position: LanePosition,
  pub(crate) wave_result: LaneWaveResult,
  pub(crate) coordination: LaneCoordinationReview,
  pub(crate) delayed_effect_origins: LaneDelayedEffectOrigins,
  pub(crate) execution_trace: InputTrace,
  pub(crate) objective: TerminalObjectiveReview,
}

impl WindowDebriefSummary {
  pub fn window(self) -> ScenarioWindow {
    self.window
  }

  pub fn intent(self) -> LaneIntent {
    self.intent
  }

  pub fn outcome(self) -> LaneOutcome {
    self.outcome
  }

  pub fn player_health(self) -> LaneHealth {
    self.player_health
  }

  pub fn player_position(self) -> LanePosition {
    self.player_position
  }

  pub fn wave_result(self) -> LaneWaveResult {
    self.wave_result
  }

  pub fn coordination(self) -> LaneCoordinationReview {
    self.coordination
  }

  pub fn delayed_effect_origins(self) -> LaneDelayedEffectOrigins {
    self.delayed_effect_origins
  }

  pub fn execution_trace(self) -> InputTrace {
    self.execution_trace
  }

  pub fn objective(self) -> TerminalObjectiveReview {
    self.objective
  }

  fn report(self) -> VisibleWindowDebriefSummary {
    VisibleWindowDebriefSummary {
      window: self.window,
      intent: self.intent,
      outcome: self.outcome,
      player_health: self.player_health,
      player_position: self.player_position,
      wave_result: self.wave_result,
      coordination: self.coordination,
      delayed_effect_origins: self.delayed_effect_origins,
      objective: self.objective.disposition(),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VisibleWindowDebriefSummary {
  pub(crate) window: ScenarioWindow,
  pub(crate) intent: LaneIntent,
  pub(crate) outcome: LaneOutcome,
  pub(crate) player_health: LaneHealth,
  pub(crate) player_position: LanePosition,
  pub(crate) wave_result: LaneWaveResult,
  pub(crate) coordination: LaneCoordinationReview,
  pub(crate) delayed_effect_origins: LaneDelayedEffectOrigins,
  pub(crate) objective: ObjectiveDisposition,
}

impl VisibleWindowDebriefSummary {
  pub fn window(self) -> ScenarioWindow {
    self.window
  }

  pub fn intent(self) -> LaneIntent {
    self.intent
  }

  pub fn outcome(self) -> LaneOutcome {
    self.outcome
  }

  pub fn player_health(self) -> LaneHealth {
    self.player_health
  }

  pub fn player_position(self) -> LanePosition {
    self.player_position
  }

  pub fn wave_result(self) -> LaneWaveResult {
    self.wave_result
  }

  pub fn coordination(self) -> LaneCoordinationReview {
    self.coordination
  }

  pub fn delayed_effect_origins(self) -> LaneDelayedEffectOrigins {
    self.delayed_effect_origins
  }

  pub fn objective(self) -> ObjectiveDisposition {
    self.objective
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScenarioDebriefReport {
  pub(crate) schema: &'static str,
  pub(crate) windows: [VisibleWindowDebriefSummary; 2],
  pub(crate) final_objective: ObjectiveDisposition,
  pub(crate) attribution_limit: FinalDebriefAttributionLimit,
}

impl ScenarioDebriefReport {
  pub fn schema(self) -> &'static str {
    self.schema
  }

  pub fn windows(self) -> [VisibleWindowDebriefSummary; 2] {
    self.windows
  }

  pub fn final_objective(self) -> ObjectiveDisposition {
    self.final_objective
  }

  pub fn attribution_limit(self) -> FinalDebriefAttributionLimit {
    self.attribution_limit
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioDebriefRecord {
  pub(crate) replay_id: &'static str,
  pub(crate) source_replay_id: &'static str,
  pub(crate) source_terminal_state_hash: StateHash,
  pub(crate) source_record_identities: [StateHash; 2],
  pub(crate) windows: [WindowDebriefSummary; 2],
  pub(crate) final_objective: ObjectiveDisposition,
  pub(crate) attribution_limit: FinalDebriefAttributionLimit,
  pub(crate) report: ScenarioDebriefReport,
}

impl ScenarioDebriefRecord {
  pub fn replay_id(&self) -> &'static str {
    self.replay_id
  }

  pub fn source_replay_id(&self) -> &'static str {
    self.source_replay_id
  }

  pub fn source_terminal_state_hash(&self) -> StateHash {
    self.source_terminal_state_hash
  }

  pub fn source_record_identities(&self) -> [StateHash; 2] {
    self.source_record_identities
  }

  pub fn windows(&self) -> [WindowDebriefSummary; 2] {
    self.windows
  }

  pub fn final_objective(&self) -> ObjectiveDisposition {
    self.final_objective
  }

  pub fn attribution_limit(&self) -> FinalDebriefAttributionLimit {
    self.attribution_limit
  }

  pub fn report(&self) -> ScenarioDebriefReport {
    self.report
  }

  pub fn verify_replay(&self, history: &LaneScenarioHistory) -> Result<(), ScenarioDebriefError> {
    let expected = build_scenario_debrief(history)?;
    if *self != expected {
      return Err(ScenarioDebriefError::ReplayMismatch);
    }
    Ok(())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScenarioDebriefError {
  History(ScenarioError),
  Objective(ObjectiveError),
  IncompleteHistory,
  ReplayMismatch,
}

pub fn build_scenario_debrief(
  history: &LaneScenarioHistory,
) -> Result<ScenarioDebriefRecord, ScenarioDebriefError> {
  history
    .verify_replay()
    .map_err(ScenarioDebriefError::History)?;
  if history.records().len() != 2 {
    return Err(ScenarioDebriefError::IncompleteHistory);
  }
  let mut summaries = [
    window_debrief_summary(&history.records()[0])?,
    window_debrief_summary(&history.records()[1])?,
  ];
  summaries[0].window = ScenarioWindow::First;
  summaries[1].window = ScenarioWindow::Second;
  let final_objective = if summaries
    .iter()
    .all(|summary| summary.objective.disposition() == ObjectiveDisposition::GoalAchieved)
  {
    ObjectiveDisposition::GoalAchieved
  } else {
    ObjectiveDisposition::GoalMissed
  };
  let attribution_limit = FinalDebriefAttributionLimit::CommittedHistoryFactsOnly;
  Ok(ScenarioDebriefRecord {
    replay_id: M2_FINAL_DEBRIEF_REPLAY_ID,
    source_replay_id: M2_TWO_WINDOW_REPLAY_ID,
    source_terminal_state_hash: history
      .terminal_state()
      .map_err(ScenarioDebriefError::History)?
      .hash(),
    source_record_identities: [
      lane_record_identity(history.records()[0].transition()),
      lane_record_identity(history.records()[1].transition()),
    ],
    windows: summaries,
    final_objective,
    attribution_limit,
    report: ScenarioDebriefReport {
      schema: M2_FINAL_DEBRIEF_REPLAY_ID,
      windows: [summaries[0].report(), summaries[1].report()],
      final_objective,
      attribution_limit,
    },
  })
}

fn window_debrief_summary(
  record: &LaneScenarioRecord,
) -> Result<WindowDebriefSummary, ScenarioDebriefError> {
  let transition = record.transition();
  let objective = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, transition)
    .map_err(ScenarioDebriefError::Objective)?;
  Ok(WindowDebriefSummary {
    window: record.window,
    intent: transition.command.intent,
    outcome: transition.result.outcome,
    player_health: transition.result.next_state.player().health(),
    player_position: transition.result.next_state.player().position(),
    wave_result: transition.inputs.execution.wave_result,
    coordination: LaneCoordinationReview::NotApplicable,
    delayed_effect_origins: transition.result.debrief.delayed_effect_origins,
    execution_trace: transition.inputs.execution.trace,
    objective: objective.review,
  })
}
