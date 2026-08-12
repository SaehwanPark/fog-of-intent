//! Actor and world state representations.

use super::primitives::{
  ActorId, FNV_OFFSET_BASIS, MAX_UNITS, RulesetId, StateHash, Turn, Units, hash_bytes,
};
use super::transition::TransitionError;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorState {
  id: ActorId,
  energy: Units,
  score: u16,
}

impl ActorState {
  pub fn new(id: ActorId, energy: Units, score: u16) -> Self {
    Self { id, energy, score }
  }

  pub fn id(self) -> ActorId {
    self.id
  }

  pub fn energy(self) -> Units {
    self.energy
  }

  pub fn score(self) -> u16 {
    self.score
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorldState {
  ruleset: RulesetId,
  turn: Turn,
  actor: ActorState,
}

impl WorldState {
  pub fn initial(actor: ActorId, ruleset: RulesetId) -> Self {
    Self {
      ruleset,
      turn: Turn::new(0),
      actor: ActorState::new(
        actor,
        Units::new(MAX_UNITS).expect("MAX_UNITS must be a valid Units value"),
        0,
      ),
    }
  }

  pub fn new(ruleset: RulesetId, turn: Turn, actor: ActorState) -> Self {
    Self {
      ruleset,
      turn,
      actor,
    }
  }

  pub fn ruleset(self) -> RulesetId {
    self.ruleset
  }

  pub fn turn(self) -> Turn {
    self.turn
  }

  pub fn actor(self) -> ActorState {
    self.actor
  }

  pub fn hash(self) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, &self.ruleset.value().to_le_bytes());
    hash = hash_bytes(hash, &self.turn.value().to_le_bytes());
    hash = hash_bytes(hash, &[self.actor.id.value(), self.actor.energy.value()]);
    hash = hash_bytes(hash, &self.actor.score.to_le_bytes());
    StateHash::from_raw(hash)
  }

  pub(crate) fn with_actor(self, actor: ActorState) -> Self {
    Self { actor, ..self }
  }

  pub(crate) fn advance_turn(self) -> Result<Self, TransitionError> {
    let next_turn = self
      .turn
      .value()
      .checked_add(1)
      .map(Turn::new)
      .ok_or(TransitionError::TurnOverflow)?;
    Ok(Self {
      turn: next_turn,
      ..self
    })
  }
}
