//! Strict, dependency-free text codecs for the bounded M1 fixtures.
//!
//! The codec is an edge adapter over the kernel. It owns text syntax and
//! version checks, while the kernel remains responsible for validation,
//! transition semantics, history commitment, and replay verification.

mod error;
mod helpers;
mod history;
mod snapshot;

#[cfg(test)]
mod tests;

pub use error::SerializationError;
pub use helpers::{HASH_REPRESENTATION, HISTORY_SCHEMA_VERSION, SNAPSHOT_SCHEMA_VERSION};
pub use history::{deserialize_history, serialize_history};
pub use snapshot::{deserialize_snapshot, serialize_snapshot};
