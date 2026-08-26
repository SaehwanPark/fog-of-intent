//! Public Alpha reproducibility packaging, sample scenarios/replays/experiments bundles, and integrity verification.

use core::fmt;
use std::collections::HashMap;

/// Canonical schema version for the M12 Alpha reproducibility contract.
pub const ALPHA_REPRODUCIBILITY_SCHEMA_VERSION: &str = "m12-alpha-reproducibility-v1";

/// Maximum integer basis points scale (100.00%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Discrete sample artifact categories packaged in public alpha bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SampleArtifactKind {
  ScenarioBenchmark,
  ReplayTranscript,
  ExperimentRun,
  ModelCalibrationStudy,
  BehavioralTelemetry,
}

impl SampleArtifactKind {
  /// Returns all canonical sample artifact categories.
  pub const fn all() -> [Self; 5] {
    [
      Self::ScenarioBenchmark,
      Self::ReplayTranscript,
      Self::ExperimentRun,
      Self::ModelCalibrationStudy,
      Self::BehavioralTelemetry,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ScenarioBenchmark => "scenario-benchmark",
      Self::ReplayTranscript => "replay-transcript",
      Self::ExperimentRun => "experiment-run",
      Self::ModelCalibrationStudy => "model-calibration-study",
      Self::BehavioralTelemetry => "behavioral-telemetry",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "scenario-benchmark" => Some(Self::ScenarioBenchmark),
      "replay-transcript" => Some(Self::ReplayTranscript),
      "experiment-run" => Some(Self::ExperimentRun),
      "model-calibration-study" => Some(Self::ModelCalibrationStudy),
      "behavioral-telemetry" => Some(Self::BehavioralTelemetry),
      _ => None,
    }
  }
}

impl fmt::Display for SampleArtifactKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Reproducibility classification of a sample artifact package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReproducibilityStatus {
  FullyReproducible,
  RequiresModelAdapter,
  SyntheticBaselineOnly,
  CorruptedOrMissing,
}

impl ReproducibilityStatus {
  /// Returns all canonical reproducibility statuses.
  pub const fn all() -> [Self; 4] {
    [
      Self::FullyReproducible,
      Self::RequiresModelAdapter,
      Self::SyntheticBaselineOnly,
      Self::CorruptedOrMissing,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FullyReproducible => "fully-reproducible",
      Self::RequiresModelAdapter => "requires-model-adapter",
      Self::SyntheticBaselineOnly => "synthetic-baseline-only",
      Self::CorruptedOrMissing => "corrupted-or-missing",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "fully-reproducible" => Some(Self::FullyReproducible),
      "requires-model-adapter" => Some(Self::RequiresModelAdapter),
      "synthetic-baseline-only" => Some(Self::SyntheticBaselineOnly),
      "corrupted-or-missing" => Some(Self::CorruptedOrMissing),
      _ => None,
    }
  }

  /// Returns true if the status represents a valid, non-corrupted artifact.
  pub const fn is_valid(self) -> bool {
    !matches!(self, Self::CorruptedOrMissing)
  }

  /// Returns the base reproducibility basis-points score for this status.
  pub const fn base_score_bp(self) -> u32 {
    match self {
      Self::FullyReproducible => 10_000,
      Self::SyntheticBaselineOnly => 8_500,
      Self::RequiresModelAdapter => 7_500,
      Self::CorruptedOrMissing => 0,
    }
  }
}

impl fmt::Display for ReproducibilityStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// A packaged reproducibility sample artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityPackageDefinition {
  pub package_id: &'static str,
  pub title: &'static str,
  pub kind: SampleArtifactKind,
  pub artifact_count: usize,
  pub content_hash_fnv1a: &'static str,
  pub verification_command: &'static str,
  pub seed_policy: &'static str,
  pub dependencies: &'static [&'static str],
  pub declared_status: ReproducibilityStatus,
}

/// Public Alpha reproducibility bundle manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityBundleManifest {
  pub schema_version: &'static str,
  pub packages: &'static [ReproducibilityPackageDefinition],
}

/// Audit record for an individual packaged artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageAuditRecord {
  pub package_id: &'static str,
  pub kind: SampleArtifactKind,
  pub status: ReproducibilityStatus,
  pub artifact_count: usize,
  pub hash_valid: bool,
  pub dependencies_resolved: bool,
  pub reproducibility_score_bp: u32,
}

/// Audit report summarizing reproducibility bundle verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityAuditReport {
  pub schema_version: &'static str,
  pub packages_evaluated: usize,
  pub total_artifacts: usize,
  pub fully_reproducible_count: usize,
  pub average_reproducibility_score_bp: u32,
  pub records: Vec<PackageAuditRecord>,
  pub bundle_eligible_for_release: bool,
}

/// Errors encountered during reproducibility bundle audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaReproducibilityError {
  EmptyBundle,
  UnsupportedSchemaVersion {
    version: String,
  },
  EmptyPackageId,
  DuplicatePackageId {
    package_id: String,
  },
  EmptyTitle {
    package_id: String,
  },
  ZeroArtifactCount {
    package_id: String,
  },
  InvalidContentHash {
    package_id: String,
    hash: String,
  },
  EmptyVerificationCommand {
    package_id: String,
  },
  MissingDependency {
    package_id: String,
    dependency: String,
  },
  CorruptedStatus {
    package_id: String,
  },
}

impl fmt::Display for AlphaReproducibilityError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyBundle => write!(f, "reproducibility bundle must not be empty"),
      Self::UnsupportedSchemaVersion { version } => {
        write!(
          f,
          "unsupported reproducibility schema version '{version}'; expected '{ALPHA_REPRODUCIBILITY_SCHEMA_VERSION}'"
        )
      }
      Self::EmptyPackageId => write!(f, "package id must not be empty"),
      Self::DuplicatePackageId { package_id } => {
        write!(f, "duplicate package id: '{package_id}'")
      }
      Self::EmptyTitle { package_id } => {
        write!(f, "package '{package_id}' has an empty title")
      }
      Self::ZeroArtifactCount { package_id } => {
        write!(
          f,
          "package '{package_id}' must define at least one artifact"
        )
      }
      Self::InvalidContentHash { package_id, hash } => {
        write!(
          f,
          "package '{package_id}' has invalid FNV-1a content hash '{hash}'; expected 16-hex character checksum"
        )
      }
      Self::EmptyVerificationCommand { package_id } => {
        write!(
          f,
          "package '{package_id}' has an empty verification command"
        )
      }
      Self::MissingDependency {
        package_id,
        dependency,
      } => {
        write!(
          f,
          "package '{package_id}' references missing dependency '{dependency}'"
        )
      }
      Self::CorruptedStatus { package_id } => {
        write!(
          f,
          "package '{package_id}' declared status is corrupted or missing"
        )
      }
    }
  }
}

impl std::error::Error for AlphaReproducibilityError {}

/// Validates whether a given string is a valid 16-hex character FNV-1a checksum.
pub fn is_valid_fnv1a_hash(hash: &str) -> bool {
  hash.len() == 16 && hash.chars().all(|c| c.is_ascii_hexdigit())
}

/// Evaluates a reproducibility bundle manifest deterministically, checking structural invariants and integrity.
pub fn audit_reproducibility_bundle(
  manifest: &ReproducibilityBundleManifest,
) -> Result<ReproducibilityAuditReport, AlphaReproducibilityError> {
  if manifest.schema_version != ALPHA_REPRODUCIBILITY_SCHEMA_VERSION {
    return Err(AlphaReproducibilityError::UnsupportedSchemaVersion {
      version: manifest.schema_version.to_string(),
    });
  }

  if manifest.packages.is_empty() {
    return Err(AlphaReproducibilityError::EmptyBundle);
  }

  let mut package_map = HashMap::new();
  for pkg in manifest.packages {
    if pkg.package_id.trim().is_empty() {
      return Err(AlphaReproducibilityError::EmptyPackageId);
    }
    if package_map.insert(pkg.package_id, pkg).is_some() {
      return Err(AlphaReproducibilityError::DuplicatePackageId {
        package_id: pkg.package_id.to_string(),
      });
    }
    if pkg.title.trim().is_empty() {
      return Err(AlphaReproducibilityError::EmptyTitle {
        package_id: pkg.package_id.to_string(),
      });
    }
    if pkg.artifact_count == 0 {
      return Err(AlphaReproducibilityError::ZeroArtifactCount {
        package_id: pkg.package_id.to_string(),
      });
    }
    if !is_valid_fnv1a_hash(pkg.content_hash_fnv1a) {
      return Err(AlphaReproducibilityError::InvalidContentHash {
        package_id: pkg.package_id.to_string(),
        hash: pkg.content_hash_fnv1a.to_string(),
      });
    }
    if pkg.verification_command.trim().is_empty() {
      return Err(AlphaReproducibilityError::EmptyVerificationCommand {
        package_id: pkg.package_id.to_string(),
      });
    }
    if pkg.declared_status == ReproducibilityStatus::CorruptedOrMissing {
      return Err(AlphaReproducibilityError::CorruptedStatus {
        package_id: pkg.package_id.to_string(),
      });
    }
  }

  // Verify dependencies
  for pkg in manifest.packages {
    for &dep in pkg.dependencies {
      if !package_map.contains_key(dep) {
        return Err(AlphaReproducibilityError::MissingDependency {
          package_id: pkg.package_id.to_string(),
          dependency: dep.to_string(),
        });
      }
    }
  }

  let mut records = Vec::with_capacity(manifest.packages.len());
  let mut total_artifacts: usize = 0;
  let mut fully_reproducible_count: usize = 0;
  let mut total_score_bp: u64 = 0;

  for pkg in manifest.packages {
    total_artifacts = total_artifacts.saturating_add(pkg.artifact_count);
    if pkg.declared_status == ReproducibilityStatus::FullyReproducible {
      fully_reproducible_count = fully_reproducible_count.saturating_add(1);
    }

    let score_bp = pkg.declared_status.base_score_bp();
    total_score_bp = total_score_bp.saturating_add(u64::from(score_bp));

    records.push(PackageAuditRecord {
      package_id: pkg.package_id,
      kind: pkg.kind,
      status: pkg.declared_status,
      artifact_count: pkg.artifact_count,
      hash_valid: true,
      dependencies_resolved: true,
      reproducibility_score_bp: score_bp,
    });
  }

  let packages_len_u64 = u64::try_from(manifest.packages.len()).unwrap_or(1);
  let average_reproducibility_score_bp = if manifest.packages.is_empty() {
    0
  } else {
    u32::try_from(total_score_bp / packages_len_u64).unwrap_or(0)
  };

  let bundle_eligible_for_release = !manifest.packages.is_empty()
    && records
      .iter()
      .all(|r| r.status.is_valid() && r.hash_valid && r.dependencies_resolved);

  Ok(ReproducibilityAuditReport {
    schema_version: manifest.schema_version,
    packages_evaluated: manifest.packages.len(),
    total_artifacts,
    fully_reproducible_count,
    average_reproducibility_score_bp,
    records,
    bundle_eligible_for_release,
  })
}

/// Renders a Markdown summary of the reproducibility audit report.
pub fn render_reproducibility_report_markdown(report: &ReproducibilityAuditReport) -> String {
  let mut out = String::new();
  out.push_str("# Public Alpha Reproducibility Bundle Audit Report\n\n");
  out.push_str(&format!(
    "- **Schema Version:** `{}`\n",
    report.schema_version
  ));
  out.push_str(&format!(
    "- **Packages Evaluated:** {}\n",
    report.packages_evaluated
  ));
  out.push_str(&format!(
    "- **Total Artifacts:** {}\n",
    report.total_artifacts
  ));
  out.push_str(&format!(
    "- **Fully Reproducible Packages:** {} / {}\n",
    report.fully_reproducible_count, report.packages_evaluated
  ));
  let whole_pct = report.average_reproducibility_score_bp / 100;
  let frac_pct = report.average_reproducibility_score_bp % 100;
  out.push_str(&format!(
    "- **Average Reproducibility Score:** {whole_pct}.{frac_pct:02}% ({} bp)\n",
    report.average_reproducibility_score_bp
  ));
  out.push_str(&format!(
    "- **Eligible for Release:** {}\n\n",
    if report.bundle_eligible_for_release {
      "Yes"
    } else {
      "No"
    }
  ));

  out.push_str("| Package ID | Kind | Status | Artifacts | Hash Valid | Score |\n");
  out.push_str("|---|---|---|---|---|---|\n");
  for r in &report.records {
    let r_whole = r.reproducibility_score_bp / 100;
    let r_frac = r.reproducibility_score_bp % 100;
    out.push_str(&format!(
      "| `{}` | `{}` | `{}` | {} | {} | {r_whole}.{r_frac:02}% ({} bp) |\n",
      r.package_id,
      r.kind.as_str(),
      r.status.as_str(),
      r.artifact_count,
      if r.hash_valid { "Valid" } else { "Invalid" },
      r.reproducibility_score_bp
    ));
  }
  out
}
