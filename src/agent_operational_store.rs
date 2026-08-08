//! Injected storage for bounded payload-free operational event logs.
//!
//! This adapter persists only the versioned operational-log codec in its own
//! namespace. It does not persist simulation history, decisions, diagnostics,
//! or provider data.

use std::fs;
use std::path::{Path, PathBuf};

use crate::agent::{ScriptedAgentOperationalLog, ScriptedAgentOperationalLogCodecError};
use crate::cli::CliRunId;
use crate::run_store::{
  CLI_RUN_OPERATIONAL_LOG_SEGMENT_SUFFIX, CLI_RUN_OPERATIONAL_LOG_SEGMENT_TEMP_SUFFIX,
  CLI_RUN_OPERATIONAL_LOG_SUFFIX, CLI_RUN_OPERATIONAL_LOG_TEMP_SUFFIX, CliRunStore,
  CliRunStoreError,
};

/// Maximum number of caller-declared operational-log segments.
pub const MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS: u8 = 4;

/// Bounded errors exposed by the operational-log store.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentOperationalLogStoreError {
  InvalidSegment {
    max: u8,
  },
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

  /// Atomically replace one bounded caller-declared log segment.
  ///
  /// Segments are storage labels only; this adapter does not infer rotation
  /// order, recover crashes, or merge segments into a runtime log.
  pub fn save_segment(
    &self,
    run_id: &str,
    segment: u8,
    log: &ScriptedAgentOperationalLog,
  ) -> Result<(), ScriptedAgentOperationalLogStoreError> {
    let (suffix, temporary_suffix) = segment_suffixes(segment)?;
    self
      .store
      .save_with_suffix(run_id, &log.encode(), &suffix, &temporary_suffix)
      .map_err(map_store_error)
  }

  /// Load one bounded caller-declared log segment.
  pub fn load_segment(
    &self,
    run_id: &str,
    segment: u8,
  ) -> Result<ScriptedAgentOperationalLog, ScriptedAgentOperationalLogStoreError> {
    let (suffix, _) = segment_suffixes(segment)?;
    let encoded = self
      .store
      .load_with_suffix(run_id, &suffix)
      .map_err(map_store_error)?;
    ScriptedAgentOperationalLog::decode(&encoded)
      .map_err(|error| ScriptedAgentOperationalLogStoreError::InvalidLog { error })
  }

  /// List recognized caller-declared segment indices in stable order.
  ///
  /// This is an observational directory scan only; it does not infer
  /// rotation order, merge files, or provide race-hard filesystem semantics.
  pub fn list_segments(
    &self,
    run_id: &str,
  ) -> Result<Vec<u8>, ScriptedAgentOperationalLogStoreError> {
    CliRunId::parse(run_id)
      .map_err(|_| ScriptedAgentOperationalLogStoreError::StorageUnavailable)?;
    let prefix = format!("{run_id}{CLI_RUN_OPERATIONAL_LOG_SEGMENT_SUFFIX}");
    let entries = fs::read_dir(self.root())
      .map_err(|_| ScriptedAgentOperationalLogStoreError::StorageUnavailable)?;
    let mut segments = Vec::new();
    for entry in entries {
      let entry = entry.map_err(|_| ScriptedAgentOperationalLogStoreError::StorageUnavailable)?;
      let file_type = entry
        .file_type()
        .map_err(|_| ScriptedAgentOperationalLogStoreError::StorageUnavailable)?;
      if !file_type.is_file() {
        continue;
      }
      let name = entry.file_name();
      let name = name.to_string_lossy();
      let Some(value) = name.strip_prefix(&prefix) else {
        continue;
      };
      let Ok(segment) = value.parse::<u8>() else {
        continue;
      };
      if segment < MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS && value == segment.to_string() {
        segments.push(segment);
      }
    }
    segments.sort_unstable();
    segments.dedup();
    Ok(segments)
  }
}

fn segment_suffixes(
  segment: u8,
) -> Result<(String, String), ScriptedAgentOperationalLogStoreError> {
  if segment >= MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS {
    return Err(ScriptedAgentOperationalLogStoreError::InvalidSegment {
      max: MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS,
    });
  }
  Ok((
    format!("{CLI_RUN_OPERATIONAL_LOG_SEGMENT_SUFFIX}{segment}"),
    format!("{CLI_RUN_OPERATIONAL_LOG_SEGMENT_TEMP_SUFFIX}{segment}"),
  ))
}

fn map_store_error(_: CliRunStoreError) -> ScriptedAgentOperationalLogStoreError {
  ScriptedAgentOperationalLogStoreError::StorageUnavailable
}
