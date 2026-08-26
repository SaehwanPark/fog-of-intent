use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneTransitionRecord {
  pub(crate) replay_id: &'static str,
  pub(crate) observation: LanerObservation,
  pub(crate) command: LaneIntentCommand,
  pub(crate) inputs: LaneResolvedInputs,
  pub(crate) prior_state_hash: StateHash,
  pub(crate) result: LaneTransitionResult,
}

impl LaneTransitionRecord {
  pub fn replay_id(&self) -> &'static str {
    self.replay_id
  }

  pub fn observation(&self) -> LanerObservation {
    self.observation
  }

  pub fn command(&self) -> LaneIntentCommand {
    self.command
  }

  pub fn inputs(&self) -> LaneResolvedInputs {
    self.inputs
  }

  pub fn prior_state_hash(&self) -> StateHash {
    self.prior_state_hash
  }

  pub fn result(&self) -> &LaneTransitionResult {
    &self.result
  }
}

pub struct LaneHistory {
  pub(crate) initial_state: LaneSnapshot,
  pub(crate) current_state: LaneSnapshot,
  pub(crate) records: Vec<LaneTransitionRecord>,
}

impl LaneHistory {
  pub fn new(initial_state: LaneSnapshot) -> Result<Self, LaneHistoryError> {
    if !initial_state.is_valid_lane_state() || initial_state.phase() != LanePhase::Open {
      return Err(LaneHistoryError::InvalidInitialState);
    }
    Ok(Self {
      initial_state,
      current_state: initial_state,
      records: Vec::new(),
    })
  }

  pub fn from_records(
    initial_state: LaneSnapshot,
    records: Vec<LaneTransitionRecord>,
  ) -> Result<Self, LaneHistoryError> {
    if !initial_state.is_valid_lane_state() || initial_state.phase() != LanePhase::Open {
      return Err(LaneHistoryError::InvalidInitialState);
    }
    let mut current_state = initial_state;
    for record in &records {
      current_state = record.result().next_state();
    }
    Ok(Self {
      initial_state,
      current_state,
      records,
    })
  }

  pub fn initial_state(&self) -> LaneSnapshot {
    self.initial_state
  }

  pub fn current_state(&self) -> LaneSnapshot {
    self.current_state
  }

  pub fn records(&self) -> &[LaneTransitionRecord] {
    &self.records
  }

  pub fn append(
    &mut self,
    receipt: &LaneObservationReceipt,
    request: &LaneIntentRequest,
    inputs: LaneResolvedInputs,
  ) -> Result<LaneTransitionResult, LaneHistoryError> {
    let index = self.records.len();
    let validated = validate_lane_request(&self.current_state, receipt, request)
      .map_err(|error| LaneHistoryError::Validation { index, error })?;
    let prior_state_hash = self.current_state.hash();
    let result = transition_lane(&self.current_state, &validated, &inputs)
      .map_err(|error| LaneHistoryError::Transition { index, error })?;
    self.current_state = result.next_state();
    self.records.push(LaneTransitionRecord {
      replay_id: M2_REPLAY_ID,
      observation: receipt.observation,
      command: validated.command,
      inputs,
      prior_state_hash,
      result: result.clone(),
    });
    Ok(result)
  }

  pub fn verify_replay(&self) -> Result<LaneSnapshot, LaneReplayError> {
    let mut state = self.initial_state;
    for (index, record) in self.records.iter().enumerate() {
      if record.replay_id != M2_REPLAY_ID {
        return Err(LaneReplayError::ReplayIdMismatch { index });
      }
      let actual_prior_hash = state.hash();
      if record.prior_state_hash != actual_prior_hash {
        return Err(LaneReplayError::PriorHashMismatch {
          index,
          expected: record.prior_state_hash,
          actual: actual_prior_hash,
        });
      }
      let receipt = observe_player(&state, record.command.observation_id);
      if receipt.observation != record.observation {
        return Err(LaneReplayError::ObservationMismatch { index });
      }
      let validated = validate_lane_command(&state, &receipt, &record.command)
        .map_err(|error| LaneReplayError::Validation { index, error })?;
      let result = transition_lane(&state, &validated, &record.inputs)
        .map_err(|error| LaneReplayError::Transition { index, error })?;
      if result != record.result {
        return Err(LaneReplayError::ResultMismatch { index });
      }
      state = result.next_state();
    }
    if state != self.current_state {
      return Err(LaneReplayError::TerminalStateMismatch {
        expected: self.current_state,
        actual: state,
      });
    }
    Ok(state)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LaneHistoryError {
  InvalidInitialState,
  Validation {
    index: usize,
    error: LaneValidationError,
  },
  Transition {
    index: usize,
    error: LaneTransitionError,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LaneReplayError {
  PriorHashMismatch {
    index: usize,
    expected: StateHash,
    actual: StateHash,
  },
  ReplayIdMismatch {
    index: usize,
  },
  ObservationMismatch {
    index: usize,
  },
  Validation {
    index: usize,
    error: LaneValidationError,
  },
  Transition {
    index: usize,
    error: LaneTransitionError,
  },
  ResultMismatch {
    index: usize,
  },
  TerminalStateMismatch {
    expected: LaneSnapshot,
    actual: LaneSnapshot,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatedLaneRecord {
  pub(crate) replay_id: &'static str,
  pub(crate) base_record_identity: StateHash,
  pub(crate) player_observation: LanerObservation,
  pub(crate) allied_observation: AlliedLaneObservation,
  pub(crate) offer: AlliedProposalOffer,
  pub(crate) request: CoordinatedLaneRequest,
  pub(crate) coordination_inputs: CoordinationResolutionInputs,
  pub(crate) resolution: CoordinationResolution,
  pub(crate) base_record: LaneTransitionRecord,
  pub(crate) result: CoordinatedTransitionResult,
}

impl CoordinatedLaneRecord {
  pub fn replay_id(&self) -> &'static str {
    self.replay_id
  }

  pub fn base_record_identity(&self) -> StateHash {
    self.base_record_identity
  }

  pub fn player_observation(&self) -> LanerObservation {
    self.player_observation
  }

  pub fn allied_observation(&self) -> AlliedLaneObservation {
    self.allied_observation
  }

  pub fn offer(&self) -> AlliedProposalOffer {
    self.offer
  }

  pub fn request(&self) -> CoordinatedLaneRequest {
    self.request
  }

  pub fn coordination_inputs(&self) -> CoordinationResolutionInputs {
    self.coordination_inputs
  }

  pub fn resolution(&self) -> CoordinationResolution {
    self.resolution
  }

  pub fn base_record(&self) -> &LaneTransitionRecord {
    &self.base_record
  }

  pub fn result(&self) -> &CoordinatedTransitionResult {
    &self.result
  }
}

pub struct CoordinatedLaneHistory {
  pub(crate) initial_state: LaneSnapshot,
  pub(crate) current_state: LaneSnapshot,
  pub(crate) records: Vec<CoordinatedLaneRecord>,
}

impl CoordinatedLaneHistory {
  pub fn new(initial_state: LaneSnapshot) -> Result<Self, CoordinationError> {
    if !initial_state.is_valid_lane_state() || initial_state.phase() != LanePhase::Open {
      return Err(CoordinationError::InvalidAlliedObservation);
    }
    Ok(Self {
      initial_state,
      current_state: initial_state,
      records: Vec::new(),
    })
  }

  pub fn initial_state(&self) -> LaneSnapshot {
    self.initial_state
  }

  pub fn current_state(&self) -> LaneSnapshot {
    self.current_state
  }

  pub fn records(&self) -> &[CoordinatedLaneRecord] {
    &self.records
  }

  pub fn append(
    &mut self,
    player_receipt: &LaneObservationReceipt,
    allied_receipt: &AlliedObservationReceipt,
    offer: &AlliedProposalOffer,
    request: &CoordinatedLaneRequest,
    coordination_inputs: CoordinationResolutionInputs,
    lane_inputs: LaneResolvedInputs,
  ) -> Result<CoordinatedTransitionResult, CoordinationError> {
    if !self.records.is_empty() {
      return Err(CoordinationError::HistoryAlreadyHasRecord);
    }
    let validated = validate_coordinated_request(
      &self.current_state,
      player_receipt,
      allied_receipt,
      offer,
      request,
      lane_inputs.policy(),
    )?;
    let prior_state_hash = self.current_state.hash();
    let result = resolve_coordinated_lane(
      &self.current_state,
      player_receipt,
      allied_receipt,
      offer,
      request,
      &coordination_inputs,
      &lane_inputs,
    )?;
    let base_record = LaneTransitionRecord {
      replay_id: M2_REPLAY_ID,
      observation: player_receipt.observation,
      command: validated.intent.command,
      inputs: lane_inputs,
      prior_state_hash,
      result: result.lane.clone(),
    };
    self.current_state = result.next_state();
    self.records.push(CoordinatedLaneRecord {
      replay_id: M2_COORDINATION_REPLAY_ID,
      base_record_identity: lane_record_identity(&base_record),
      player_observation: player_receipt.observation,
      allied_observation: allied_receipt.observation,
      offer: *offer,
      request: *request,
      coordination_inputs,
      resolution: result.coordination,
      base_record,
      result: result.clone(),
    });
    Ok(result)
  }

  pub fn verify_replay(&self) -> Result<LaneSnapshot, CoordinationError> {
    if self.records.len() > 1 {
      return Err(CoordinationError::ReplayMismatch);
    }
    let mut state = self.initial_state;
    for record in &self.records {
      if record.replay_id != M2_COORDINATION_REPLAY_ID {
        return Err(CoordinationError::ReplayMismatch);
      }
      if record.base_record.replay_id != M2_REPLAY_ID {
        return Err(CoordinationError::ReplayMismatch);
      }
      if record.base_record.prior_state_hash != state.hash() {
        return Err(CoordinationError::ReplayMismatch);
      }
      if lane_record_identity(&record.base_record) != record.base_record_identity {
        return Err(CoordinationError::ReplayMismatch);
      }
      let player_receipt = observe_player(&state, record.base_record.command.observation_id);
      let allied_receipt = observe_allied(&state, record.allied_observation.observation_id);
      let proposal = scripted_allied_proposal(
        allied_receipt.observation,
        record.base_record.inputs.policy(),
      )
      .map_err(|_| CoordinationError::ReplayMismatch)?;
      let offer = offer_allied_proposal(proposal).map_err(|_| CoordinationError::ReplayMismatch)?;
      if player_receipt.observation != record.player_observation
        || allied_receipt.observation != record.allied_observation
        || offer != record.offer
      {
        return Err(CoordinationError::ReplayMismatch);
      }
      let result = resolve_coordinated_lane(
        &state,
        &player_receipt,
        &allied_receipt,
        &offer,
        &record.request,
        &record.coordination_inputs,
        &record.base_record.inputs,
      )
      .map_err(|_| CoordinationError::ReplayMismatch)?;
      let validated = validate_coordinated_request(
        &state,
        &player_receipt,
        &allied_receipt,
        &offer,
        &record.request,
        record.base_record.inputs.policy(),
      )
      .map_err(|_| CoordinationError::ReplayMismatch)?;
      let expected_base_record = LaneTransitionRecord {
        replay_id: M2_REPLAY_ID,
        observation: player_receipt.observation,
        command: validated.intent.command,
        inputs: record.base_record.inputs,
        prior_state_hash: state.hash(),
        result: result.lane.clone(),
      };
      if result != record.result
        || result.coordination != record.resolution
        || expected_base_record != record.base_record
      {
        return Err(CoordinationError::ReplayMismatch);
      }
      state = result.next_state();
    }
    if state != self.current_state {
      return Err(CoordinationError::ReplayMismatch);
    }
    Ok(state)
  }
}
