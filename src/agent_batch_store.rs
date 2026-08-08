//! Injected file storage for bounded scripted-agent batch checkpoints.
//!
//! This adapter persists only the versioned cursor from [`crate::agent`]. It
//! reuses the existing bounded run-store filesystem boundary and does not
//! persist decisions, metrics, provider data, or simulation history.

use std::path::{Path, PathBuf};

use crate::agent::{
  MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, ScriptedAgentBatchCheckpoint,
  ScriptedAgentBatchCheckpointError, ScriptedAgentOperationalEvent, ScriptedAgentOperationalLog,
};
use crate::run_store::{
  CLI_RUN_BATCH_CHECKPOINT_SUFFIX, CLI_RUN_BATCH_CHECKPOINT_TEMP_SUFFIX, CliRunStore,
  CliRunStoreError,
};

/// Bounded errors exposed by the batch checkpoint store.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBatchStoreError {
  StorageUnavailable,
  InvalidCheckpoint {
    error: ScriptedAgentBatchCheckpointError,
  },
}

/// Bounded failures from checkpoint storage with caller-owned event production.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBatchStoreOperationalError {
  Store(ScriptedAgentBatchStoreError),
  LogCapacityExceeded { max: usize },
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
      .save_with_suffix(
        run_id,
        &checkpoint.encode(),
        CLI_RUN_BATCH_CHECKPOINT_SUFFIX,
        CLI_RUN_BATCH_CHECKPOINT_TEMP_SUFFIX,
      )
      .map_err(map_store_error)
  }

  /// Save a cursor and append one event only after storage succeeds.
  pub fn save_with_operational_log(
    &self,
    run_id: &str,
    checkpoint: ScriptedAgentBatchCheckpoint,
    log: &mut ScriptedAgentOperationalLog,
  ) -> Result<(), ScriptedAgentBatchStoreOperationalError> {
    ensure_event_capacity(log)?;
    self
      .save(run_id, checkpoint)
      .map_err(ScriptedAgentBatchStoreOperationalError::Store)?;
    log
      .append(ScriptedAgentOperationalEvent::CheckpointSaved)
      .expect("operational log capacity was preflighted");
    Ok(())
  }

  /// Load and decode one bounded checkpoint without executing policy.
  pub fn load(
    &self,
    run_id: &str,
  ) -> Result<ScriptedAgentBatchCheckpoint, ScriptedAgentBatchStoreError> {
    let encoded = self
      .store
      .load_with_suffix(run_id, CLI_RUN_BATCH_CHECKPOINT_SUFFIX)
      .map_err(map_store_error)?;
    ScriptedAgentBatchCheckpoint::decode(&encoded)
      .map_err(|error| ScriptedAgentBatchStoreError::InvalidCheckpoint { error })
  }

  /// Load a cursor and append one event only after storage and decoding succeed.
  pub fn load_with_operational_log(
    &self,
    run_id: &str,
    log: &mut ScriptedAgentOperationalLog,
  ) -> Result<ScriptedAgentBatchCheckpoint, ScriptedAgentBatchStoreOperationalError> {
    ensure_event_capacity(log)?;
    let checkpoint = self
      .load(run_id)
      .map_err(ScriptedAgentBatchStoreOperationalError::Store)?;
    log
      .append(ScriptedAgentOperationalEvent::BatchResumed)
      .expect("operational log capacity was preflighted");
    Ok(checkpoint)
  }
}

fn ensure_event_capacity(
  log: &ScriptedAgentOperationalLog,
) -> Result<(), ScriptedAgentBatchStoreOperationalError> {
  if log.len() >= MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
    return Err(
      ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
        max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
      },
    );
  }
  Ok(())
}

fn map_store_error(_: CliRunStoreError) -> ScriptedAgentBatchStoreError {
  ScriptedAgentBatchStoreError::StorageUnavailable
}
