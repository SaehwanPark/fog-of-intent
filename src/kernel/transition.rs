//! Transition semantics and outcome records.

use super::command::{Action, ValidatedCommand};
use super::inputs::{InputTrace, ResolvedInputs};
use super::primitives::{ActorId, StateHash, Units};
use super::state::{ActorState, WorldState};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectCause {
  Command,
  Execution(InputTrace),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
  Held {
    actor: ActorId,
  },
  Gathered {
    actor: ActorId,
    requested: Units,
    yielded: Units,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
  EnergySpent {
    actor: ActorId,
    amount: Units,
    cause: EffectCause,
  },
  ScoreAwarded {
    actor: ActorId,
    amount: Units,
    cause: EffectCause,
  },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionError {
  StaleValidation {
    expected: StateHash,
    actual: StateHash,
  },
  YieldExceedsSpend {
    requested: Units,
    yielded: Units,
  },
  InsufficientEnergy {
    available: Units,
    requested: Units,
  },
  ScoreOverflow,
  TurnOverflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionResult {
  next_state: WorldState,
  events: Vec<Event>,
  effects: Vec<Effect>,
  state_hash: StateHash,
}

impl TransitionResult {
  pub fn new(
    next_state: WorldState,
    events: Vec<Event>,
    effects: Vec<Effect>,
    state_hash: StateHash,
  ) -> Self {
    Self {
      next_state,
      events,
      effects,
      state_hash,
    }
  }

  pub fn next_state(&self) -> WorldState {
    self.next_state
  }

  pub fn events(&self) -> &[Event] {
    &self.events
  }

  pub fn effects(&self) -> &[Effect] {
    &self.effects
  }

  pub fn state_hash(&self) -> StateHash {
    self.state_hash
  }
}

pub fn transition(
  state: &WorldState,
  command: &ValidatedCommand,
  inputs: &ResolvedInputs,
) -> Result<TransitionResult, TransitionError> {
  let actual_hash = state.hash();
  if command.validated_state != *state {
    return Err(TransitionError::StaleValidation {
      expected: command.validated_state.hash(),
      actual: actual_hash,
    });
  }

  let actor = state.actor();
  let mut events = Vec::new();
  let mut effects = Vec::new();
  let next_state = match command.command.action() {
    Action::Hold => {
      events.push(Event::Held { actor: actor.id() });
      state.advance_turn()?
    }
    Action::Gather { spend } => {
      let yielded = inputs.execution().yielded();
      if yielded.value() > spend.value() {
        return Err(TransitionError::YieldExceedsSpend {
          requested: spend,
          yielded,
        });
      }
      let energy = actor
        .energy()
        .subtract(spend)
        .ok_or(TransitionError::InsufficientEnergy {
          available: actor.energy(),
          requested: spend,
        })?;
      let score = actor
        .score()
        .checked_add(u16::from(yielded.value()))
        .ok_or(TransitionError::ScoreOverflow)?;
      let execution_cause = EffectCause::Execution(inputs.execution().trace());
      events.push(Event::Gathered {
        actor: actor.id(),
        requested: spend,
        yielded,
      });
      effects.push(Effect::EnergySpent {
        actor: actor.id(),
        amount: spend,
        cause: EffectCause::Command,
      });
      if yielded != Units::zero() {
        effects.push(Effect::ScoreAwarded {
          actor: actor.id(),
          amount: yielded,
          cause: execution_cause,
        });
      }
      state
        .with_actor(ActorState::new(actor.id(), energy, score))
        .advance_turn()?
    }
  };

  let state_hash = next_state.hash();
  Ok(TransitionResult {
    next_state,
    events,
    effects,
    state_hash,
  })
}
