//! The first information-asymmetric M2 lane decision window.
//!
//! This module is a pure lane decision-window extension of the M1 kernel
//! boundary. A host creates an actor-valid observation, validates a request,
//! resolves execution inputs at the edge, and then invokes the deterministic
//! transition. No function here reads I/O, time, randomness, or hidden state
//! through an actor-facing value.

use std::fmt;

use crate::kernel::{
  ActorId, DrawId, InputTrace, RulesetId, StateHash, StreamId, Turn, hash_bytes,
};

mod branch;
mod coordination;
mod encoding;
mod evaluation;
mod history;
mod intent;
mod objective;
mod observation;
mod projection;
mod result;
mod scenario;
mod state;
mod transition;
mod validation;
mod values;

pub(crate) use branch::lane_record_identity;
pub use branch::*;
pub use coordination::*;
pub(crate) use encoding::*;
pub use history::*;
pub use intent::*;
pub(crate) use objective::response_review;
pub use objective::*;
pub use observation::*;
#[cfg(test)]
pub(crate) use scenario::reopen_resolved_snapshot;
pub use scenario::*;
pub use state::*;
pub use transition::*;
pub use validation::*;
pub use values::*;

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod tests;
