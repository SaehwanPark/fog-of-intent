//! A small deterministic transition boundary.
//!
//! The kernel evaluates owned values only. It does not create random values,
//! read time, perform I/O, or persist history. A host supplies validated
//! commands and already-resolved inputs, then may commit the returned result.

mod command;
mod history;
mod inputs;
mod primitives;
mod state;
mod transition;

#[cfg(test)]
mod tests;

pub use command::{Action, Command, ValidatedCommand, ValidationError, validate_command};
pub use history::{History, HistoryError, ReplayError, TransitionRecord};
pub use inputs::{
  CoordinationInputs, EnvironmentInputs, ExecutionInputs, InputTrace, ObservationInputs,
  PolicyInputs, ResolvedInputs,
};
pub(crate) use primitives::hash_bytes;
pub use primitives::{
  ActorId, BoundsError, CURRENT_RULESET, DrawId, MAX_UNITS, RulesetId, StateHash, StreamId, Turn,
  Units,
};
pub use state::{ActorState, WorldState};
pub use transition::{Effect, EffectCause, Event, TransitionError, TransitionResult, transition};
