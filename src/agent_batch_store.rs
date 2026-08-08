//! Injected file storage for bounded scripted-agent batch checkpoints.
//!
//! This adapter persists only the versioned cursor from [`crate::agent`]. It
//! reuses the existing bounded run-store filesystem boundary and does not
//! persist decisions, metrics, provider data, or simulation history.

use std::path::{Path, PathBuf};

use crate::agent::{ScriptedAgentBatchCheckpoint, ScriptedAgentBatchCheckpointError};
use crate::run_store::{CliRunStore, CliRunStoreError};

/// Bounded errors exposed by the batch checkpoint store.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBatchStoreError {
  StorageUnavailable,
  InvalidCheckpoint {
    error: ScriptedAgentBatchCheckpointError,
  },
}

/// File-backed store for resumable scripted-agent batch cursors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptedAgentBatchRunStore {
  store: CliRunStore,
}

impl ScriptedAgentBatchRunStore {
  /// Configure a store without touching the filesystem.
  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self {
      store: CliRunStore::new(root),
    }
  }

  pub fn root(&self) -> &Path {
    self.store.root()
  }

  /// Atomically replace one bounded checkpoint in the configured run directory.
  pub fn save(
    &self,
    run_id: &str,
    checkpoint: ScriptedAgentBatchCheckpoint,
  ) -> Result<(), ScriptedAgentBatchStoreError> {
    self
      .store
      .save(run_id, &checkpoint.encode())
      .map_err(map_store_error)
  }

  /// Load and decode one bounded checkpoint without executing policy.
  pub fn load(
    &self,
    run_id: &str,
  ) -> Result<ScriptedAgentBatchCheckpoint, ScriptedAgentBatchStoreError> {
    let encoded = self.store.load(run_id).map_err(map_store_error)?;
    ScriptedAgentBatchCheckpoint::decode(&encoded)
      .map_err(|error| ScriptedAgentBatchStoreError::InvalidCheckpoint { error })
  }
}

fn map_store_error(_: CliRunStoreError) -> ScriptedAgentBatchStoreError {
  ScriptedAgentBatchStoreError::StorageUnavailable
}
