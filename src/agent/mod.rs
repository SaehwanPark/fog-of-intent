//! Actor-visible scripted-agent policy for the M4 baseline.
//!
//! This module re-exports all public surface from the decomposed submodules.
//! External callers should import from `crate::agent::*` as they did before.

pub mod comparison;
pub mod empirical;
pub mod experiment;
pub mod measures;
pub mod operational;
pub mod parametric;
pub mod policy;
pub mod population;
pub mod profile;
pub mod replay;
pub mod semantic;
pub mod tally;

#[cfg(test)]
mod tests;

pub use comparison::*;
pub use empirical::*;
pub use experiment::*;
pub use measures::*;
pub use operational::*;
pub use parametric::*;
pub use policy::*;
pub use population::*;
pub use profile::*;
pub use replay::*;
pub use semantic::*;
pub use tally::*;
