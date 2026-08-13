//! Actor-visible scripted-agent policy for the M4 baseline.
//!
//! This module re-exports all public surface from the decomposed submodules.
//! External callers should import from `crate::agent::*` as they did before.

pub mod attribution;
pub mod communication;
pub mod comparison;
pub mod debrief;
pub mod disagreement;
pub mod empirical;
pub mod experiment;
pub mod held_out;
pub mod leadership;
pub mod measures;
pub mod multi_model;
pub mod operational;
pub mod parametric;
pub mod policy;
pub mod population;
pub mod profile;
pub mod recalibration;
pub mod reference_output;
pub mod replay;
pub mod scenarios;
pub mod semantic;
pub mod simultaneous;
pub mod tally;
pub mod team_plan;
pub mod trust;
pub mod uncertainty;

#[cfg(test)]
mod tests;

pub use attribution::*;
pub use communication::*;
pub use comparison::*;
pub use debrief::*;
pub use disagreement::*;
pub use empirical::*;
pub use experiment::*;
pub use held_out::*;
pub use leadership::*;
pub use measures::*;
pub use multi_model::*;
pub use operational::*;
pub use parametric::*;
pub use policy::*;
pub use population::*;
pub use profile::*;
pub use recalibration::*;
pub use reference_output::*;
pub use replay::*;
pub use scenarios::*;
pub use semantic::*;
pub use simultaneous::*;
pub use tally::*;
pub use team_plan::*;
pub use trust::*;
pub use uncertainty::*;
