//! Injected storage for bounded payload-free operational event logs.
//!
//! This adapter persists only the versioned operational-log codec in its own
//! namespace. It does not persist simulation history, decisions, diagnostics,
//! or provider data.

use std::path::{Path, PathBuf};

use crate::agent::{ScriptedAgentOperationalLog, ScriptedAgentOperationalLogCodecError};
use crate::run_store::{
  CLI_RUN_OPERATIONAL_LOG_SUFFIX, CLI_RUN_OPERATIONAL_LOG_TEMP_SUFFIX, CliRunStore,
  CliRunStoreError,
};

/// Bounded errors exposed by the operational-log store.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalLogStoreError {
  StorageUnavailable,
  InvalidLog {
    error: ScriptedAgentOperationalLogCodecError,
  },
}

/// File-backed store for one bounded operational log.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptedAgentOperationalLogStore {
  store: CliRunStore,
}

impl ScriptedAgentOperationalLogStore {
  /// Configure a store without touching the filesystem.
  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self {
      store: CliRunStore::new(root),
    }
  }

  pub fn root(&self) -> &Path {
    self.store.root()
  }

  /// Atomically replace one bounded operational log.
  pub fn save(
    &self,
    run_id: &str,
    log: &ScriptedAgentOperationalLog,
  ) -> Result<(), ScriptedAgentOperationalLogStoreError> {
    self
      .store
      .save_with_suffix(
        run_id,
        &log.encode(),
        CLI_RUN_OPERATIONAL_LOG_SUFFIX,
        CLI_RUN_OPERATIONAL_LOG_TEMP_SUFFIX,
      )
      .map_err(map_store_error)
  }

  /// Load and decode one bounded operational log.
  pub fn load(
    &self,
    run_id: &str,
  ) -> Result<ScriptedAgentOperationalLog, ScriptedAgentOperationalLogStoreError> {
    let encoded = self
      .store
      .load_with_suffix(run_id, CLI_RUN_OPERATIONAL_LOG_SUFFIX)
      .map_err(map_store_error)?;
    ScriptedAgentOperationalLog::decode(&encoded)
      .map_err(|error| ScriptedAgentOperationalLogStoreError::InvalidLog { error })
  }
}

fn map_store_error(_: CliRunStoreError) -> ScriptedAgentOperationalLogStoreError {
  ScriptedAgentOperationalLogStoreError::StorageUnavailable
}
