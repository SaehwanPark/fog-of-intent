//! Append-only transition history and replay verification.

use super::command::{Command, ValidationError, validate_command};
use super::inputs::ResolvedInputs;
use super::primitives::StateHash;
use super::state::WorldState;
use super::transition::{TransitionError, TransitionResult, transition};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionRecord {
  command: Command,
  inputs: ResolvedInputs,
  prior_state_hash: StateHash,
  result: TransitionResult,
}

impl TransitionRecord {
  pub fn command(&self) -> Command {
    self.command
  }

  pub fn inputs(&self) -> ResolvedInputs {
    self.inputs
  }

  pub fn prior_state_hash(&self) -> StateHash {
    self.prior_state_hash
  }

  pub fn result(&self) -> &TransitionResult {
    &self.result
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoryError {
  Validation {
    index: usize,
    error: ValidationError,
  },
  Transition {
    index: usize,
    error: TransitionError,
  },
}

pub struct History {
  initial_state: WorldState,
  current_state: WorldState,
  records: Vec<TransitionRecord>,
}

impl History {
  pub fn new(initial_state: WorldState) -> Self {
    Self {
      initial_state,
      current_state: initial_state,
      records: Vec::new(),
    }
  }

  pub fn initial_state(&self) -> WorldState {
    self.initial_state
  }

  pub fn current_state(&self) -> WorldState {
    self.current_state
  }

  pub fn records(&self) -> &[TransitionRecord] {
    &self.records
  }

  pub fn append(
    &mut self,
    command: Command,
    inputs: ResolvedInputs,
  ) -> Result<TransitionResult, HistoryError> {
    let index = self.records.len();
    let validated = validate_command(&self.current_state, &command)
      .map_err(|error| HistoryError::Validation { index, error })?;
    let prior_state_hash = self.current_state.hash();
    let result = transition(&self.current_state, &validated, &inputs)
      .map_err(|error| HistoryError::Transition { index, error })?;
    self.current_state = result.next_state();
    self.records.push(TransitionRecord {
      command,
      inputs,
      prior_state_hash,
      result: result.clone(),
    });
    Ok(result)
  }

  pub fn verify_replay(&self) -> Result<WorldState, ReplayError> {
    let mut state = self.initial_state;
    for (index, record) in self.records.iter().enumerate() {
      let actual_prior_hash = state.hash();
      if record.prior_state_hash != actual_prior_hash {
        return Err(ReplayError::PriorHashMismatch {
          index,
          expected: record.prior_state_hash,
          actual: actual_prior_hash,
        });
      }
      let validated = validate_command(&state, &record.command)
        .map_err(|error| ReplayError::Validation { index, error })?;
      let result = transition(&state, &validated, &record.inputs)
        .map_err(|error| ReplayError::Transition { index, error })?;
      if result != record.result {
        return Err(ReplayError::ResultMismatch { index });
      }
      state = result.next_state();
    }
    if state != self.current_state {
      return Err(ReplayError::TerminalStateMismatch {
        expected: self.current_state,
        actual: state,
      });
    }
    Ok(state)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayError {
  PriorHashMismatch {
    index: usize,
    expected: StateHash,
    actual: StateHash,
  },
  Validation {
    index: usize,
    error: ValidationError,
  },
  Transition {
    index: usize,
    error: TransitionError,
  },
  ResultMismatch {
    index: usize,
  },
  TerminalStateMismatch {
    expected: WorldState,
    actual: WorldState,
  },
}
