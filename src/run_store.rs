//! Explicit file-backed storage for bounded host artifacts.
//!
//! This module owns filesystem effects only. It validates run identifiers,
//! bounds reads and writes, and replaces artifacts through a same-directory
//! temporary file plus rename. Artifact syntax and replay validation remain in
//! [`crate::host_artifact`] and [`crate::host`].

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::cli::{CliRunId, CliRunIdError};
use crate::host_artifact::MAX_CLI_HOST_ARTIFACT_BYTES;

/// Fixed suffix for persisted bounded host artifacts.
pub const CLI_RUN_ARTIFACT_SUFFIX: &str = ".foi-artifact";
/// Fixed suffix for a same-directory replacement temporary file.
pub const CLI_RUN_TEMP_SUFFIX: &str = ".foi-artifact.tmp";

static NEXT_TEMP_FILE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliRunStoreError {
  InvalidRunId { error: CliRunIdError },
  ArtifactTooLarge,
  CreateDirectory { kind: io::ErrorKind },
  Read { kind: io::ErrorKind },
  WriteTemporary { kind: io::ErrorKind },
  Replace { kind: io::ErrorKind },
}

/// File-backed store for explicitly configured host artifacts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliRunStore {
  root: PathBuf,
}

impl CliRunStore {
  /// Configure a store without touching the filesystem.
  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self { root: root.into() }
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  /// Atomically replace one run artifact within the configured root.
  pub fn save(&self, run_id: &str, artifact: &str) -> Result<(), CliRunStoreError> {
    self.validate_run_id(run_id)?;
    if artifact.len() > MAX_CLI_HOST_ARTIFACT_BYTES {
      return Err(CliRunStoreError::ArtifactTooLarge);
    }
    fs::create_dir_all(&self.root)
      .map_err(|error| CliRunStoreError::CreateDirectory { kind: error.kind() })?;
    let temporary = self.temporary_path(run_id);
    let final_path = self.path(run_id, CLI_RUN_ARTIFACT_SUFFIX);
    let mut temporary_file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .open(&temporary)
      .map_err(|error| CliRunStoreError::WriteTemporary { kind: error.kind() })?;
    temporary_file
      .write_all(artifact.as_bytes())
      .map_err(|error| CliRunStoreError::WriteTemporary { kind: error.kind() })?;
    fs::rename(&temporary, &final_path)
      .map_err(|error| CliRunStoreError::Replace { kind: error.kind() })?;
    Ok(())
  }

  /// Read one final artifact, rejecting oversized files before unbounded decode.
  pub fn load(&self, run_id: &str) -> Result<String, CliRunStoreError> {
    self.validate_run_id(run_id)?;
    let final_path = self.path(run_id, CLI_RUN_ARTIFACT_SUFFIX);
    let mut file =
      File::open(final_path).map_err(|error| CliRunStoreError::Read { kind: error.kind() })?;
    let mut artifact = String::new();
    Read::by_ref(&mut file)
      .take(u64::try_from(MAX_CLI_HOST_ARTIFACT_BYTES + 1).expect("bound fits in u64"))
      .read_to_string(&mut artifact)
      .map_err(|error| CliRunStoreError::Read { kind: error.kind() })?;
    if artifact.len() > MAX_CLI_HOST_ARTIFACT_BYTES {
      return Err(CliRunStoreError::ArtifactTooLarge);
    }
    Ok(artifact)
  }

  fn validate_run_id(&self, run_id: &str) -> Result<(), CliRunStoreError> {
    CliRunId::parse(run_id)
      .map(|_| ())
      .map_err(|error| CliRunStoreError::InvalidRunId { error })
  }

  fn path(&self, run_id: &str, suffix: &str) -> PathBuf {
    self.root.join(format!("{run_id}{suffix}"))
  }

  fn temporary_path(&self, run_id: &str) -> PathBuf {
    let sequence = NEXT_TEMP_FILE.fetch_add(1, Ordering::Relaxed);
    self.root.join(format!(
      "{run_id}{CLI_RUN_TEMP_SUFFIX}.{}.{sequence}",
      std::process::id()
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::{AtomicU64, Ordering};

  static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

  fn temporary_root() -> PathBuf {
    let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
      "fog-of-intent-run-store-{}-{id}",
      std::process::id()
    ))
  }

  fn artifact() -> &'static str {
    "artifact schema=m3-cli-host-artifact-v1 replay_id=m2-two-window-scenario-v3 run_id=run records=0"
  }

  fn cleanup(root: &Path) {
    let _ = fs::remove_dir_all(root);
  }

  #[test]
  fn store_round_trips_and_replaces_final_artifact() {
    let root = temporary_root();
    let store = CliRunStore::new(&root);
    store.save("run", artifact()).expect("first save");
    assert_eq!(store.load("run").expect("first load"), artifact());
    assert!(root.join("run.foi-artifact").is_file());
    assert_eq!(
      fs::read_dir(&root).expect("store directory").count(),
      1,
      "successful replacement leaves only the final artifact"
    );

    let replacement = artifact().replace("records=0", "records=0 ");
    store.save("run", &replacement).expect("replacement save");
    assert_eq!(store.load("run").expect("replacement load"), replacement);
    cleanup(&root);
  }

  #[test]
  fn store_rejects_missing_invalid_and_oversized_inputs() {
    let root = temporary_root();
    let store = CliRunStore::new(&root);
    assert!(matches!(
      store.load("missing"),
      Err(CliRunStoreError::Read {
        kind: io::ErrorKind::NotFound
      })
    ));
    assert!(matches!(
      store.save("run/id", artifact()),
      Err(CliRunStoreError::InvalidRunId {
        error: CliRunIdError::InvalidCharacter { character: '/' }
      })
    ));
    let oversized = "x".repeat(MAX_CLI_HOST_ARTIFACT_BYTES + 1);
    assert_eq!(
      store.save("run", &oversized),
      Err(CliRunStoreError::ArtifactTooLarge)
    );
    cleanup(&root);
  }

  #[test]
  fn store_reports_invalid_root_without_writing_outside_it() {
    let root = temporary_root();
    fs::write(&root, "not a directory").expect("root fixture");
    let store = CliRunStore::new(&root);
    assert!(matches!(
      store.save("run", artifact()),
      Err(CliRunStoreError::CreateDirectory { .. })
    ));
    cleanup(&root);
  }

  #[test]
  fn store_rejects_oversized_files_before_returning_contents() {
    let root = temporary_root();
    fs::create_dir_all(&root).expect("root directory");
    fs::write(
      root.join("run.foi-artifact"),
      "x".repeat(MAX_CLI_HOST_ARTIFACT_BYTES + 1),
    )
    .expect("oversized artifact");
    let store = CliRunStore::new(&root);
    assert_eq!(store.load("run"), Err(CliRunStoreError::ArtifactTooLarge));
    cleanup(&root);
  }
}
