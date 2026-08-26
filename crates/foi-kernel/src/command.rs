//! Actions, commands, and legality validation.

use super::primitives::{ActorId, RulesetId, StateHash, Turn, Units};
use super::state::WorldState;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
  Hold,
  Gather { spend: Units },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Command {
  actor: ActorId,
  turn: Turn,
  ruleset: RulesetId,
  expected_state_hash: StateHash,
  action: Action,
}

impl Command {
  pub fn new(
    actor: ActorId,
    turn: Turn,
    ruleset: RulesetId,
    expected_state_hash: StateHash,
    action: Action,
  ) -> Self {
    Self {
      actor,
      turn,
      ruleset,
      expected_state_hash,
      action,
    }
  }

  pub fn hold(
    actor: ActorId,
    turn: Turn,
    ruleset: RulesetId,
    expected_state_hash: StateHash,
  ) -> Self {
    Self::new(actor, turn, ruleset, expected_state_hash, Action::Hold)
  }

  pub fn gather(
    actor: ActorId,
    turn: Turn,
    ruleset: RulesetId,
    expected_state_hash: StateHash,
    spend: Units,
  ) -> Self {
    Self::new(
      actor,
      turn,
      ruleset,
      expected_state_hash,
      Action::Gather { spend },
    )
  }

  pub fn actor(self) -> ActorId {
    self.actor
  }

  pub fn turn(self) -> Turn {
    self.turn
  }

  pub fn ruleset(self) -> RulesetId {
    self.ruleset
  }

  pub fn expected_state_hash(self) -> StateHash {
    self.expected_state_hash
  }

  pub fn action(self) -> Action {
    self.action
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ValidatedCommand {
  pub(crate) command: Command,
  pub(crate) validated_state: WorldState,
}

impl ValidatedCommand {
  pub fn command(self) -> Command {
    self.command
  }

  pub fn validated_against(self) -> StateHash {
    self.validated_state.hash()
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationError {
  WrongActor {
    expected: ActorId,
    actual: ActorId,
  },
  WrongTurn {
    expected: Turn,
    actual: Turn,
  },
  WrongRuleset {
    expected: RulesetId,
    actual: RulesetId,
  },
  StateHashMismatch {
    expected: StateHash,
    actual: StateHash,
  },
  ZeroSpend,
  InsufficientEnergy {
    available: Units,
    requested: Units,
  },
}

pub fn validate_command(
  state: &WorldState,
  command: &Command,
) -> Result<ValidatedCommand, ValidationError> {
  let actor = state.actor();
  if command.actor != actor.id() {
    return Err(ValidationError::WrongActor {
      expected: actor.id(),
      actual: command.actor,
    });
  }
  if command.turn != state.turn() {
    return Err(ValidationError::WrongTurn {
      expected: state.turn(),
      actual: command.turn,
    });
  }
  if command.ruleset != state.ruleset() {
    return Err(ValidationError::WrongRuleset {
      expected: state.ruleset(),
      actual: command.ruleset,
    });
  }
  let actual_hash = state.hash();
  if command.expected_state_hash != actual_hash {
    return Err(ValidationError::StateHashMismatch {
      expected: actual_hash,
      actual: command.expected_state_hash,
    });
  }
  if let Action::Gather { spend } = command.action {
    if spend == Units::zero() {
      return Err(ValidationError::ZeroSpend);
    }
    if spend.value() > actor.energy().value() {
      return Err(ValidationError::InsufficientEnergy {
        available: actor.energy(),
        requested: spend,
      });
    }
  }
  Ok(ValidatedCommand {
    command: *command,
    validated_state: *state,
  })
}
