//! Actor-visible scripted-agent policy for the M4 baseline.
//!
//! This module re-exports all public surface from the decomposed submodules.
//! External callers should import from `crate::agent::*` as they did before.

pub mod comparison;
pub mod empirical;
pub mod experiment;
pub mod held_out;
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
pub mod semantic;
pub mod tally;
pub mod uncertainty;

#[cfg(test)]
mod tests;

pub use comparison::*;
pub use empirical::*;
pub use experiment::*;
pub use held_out::*;
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
pub use semantic::*;
pub use tally::*;
pub use uncertainty::*;
