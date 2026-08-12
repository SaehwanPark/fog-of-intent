//! Synchronous application-host orchestration for the bounded M3 transcript.
//!
//! The host owns lifecycle, draft, save/load, and history coordination while
//! delegating validation and transition evaluation to the lane contract. It
//! accepts resolved execution inputs explicitly and returns actor-valid
//! projections; it does not render terminal output or expose true state.

mod scenario_host;
mod types;

#[cfg(test)]
mod tests;

pub use scenario_host::CliScenarioHost;
pub use types::{
  ACTOR_ILLEGAL_COMMAND_POPULATION_SCHEMA, ActorIllegalCommandPopulationError,
  ActorIllegalCommandPopulationReport, CLI_HOST_SCHEMA, CliHostError, CliHostOutput,
  MAX_ACTOR_ILLEGAL_COMMAND_POPULATION,
};
