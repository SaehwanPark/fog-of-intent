//! Compatibility policy and cross-version matrix evaluation for Fog of Intent Public Alpha.

use core::fmt;

/// Canonical schema version for the M12 Alpha compatibility contract.
pub const ALPHA_COMPATIBILITY_SCHEMA_VERSION: &str = "m12-alpha-compatibility-v1";

/// Core simulation and artifact domains subject to version compatibility governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityDomain {
  Ruleset,
  Scenario,
  ProtocolDto,
  AgentProfile,
  PromptTemplate,
  ModelCalibration,
  ReplayArtifact,
  GuiPresentation,
}

impl CompatibilityDomain {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Ruleset => "ruleset",
      Self::Scenario => "scenario",
      Self::ProtocolDto => "protocol-dto",
      Self::AgentProfile => "agent-profile",
      Self::PromptTemplate => "prompt-template",
      Self::ModelCalibration => "model-calibration",
      Self::ReplayArtifact => "replay-artifact",
      Self::GuiPresentation => "gui-presentation",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "ruleset" => Some(Self::Ruleset),
      "scenario" => Some(Self::Scenario),
      "protocol-dto" => Some(Self::ProtocolDto),
      "agent-profile" => Some(Self::AgentProfile),
      "prompt-template" => Some(Self::PromptTemplate),
      "model-calibration" => Some(Self::ModelCalibration),
      "replay-artifact" => Some(Self::ReplayArtifact),
      "gui-presentation" => Some(Self::GuiPresentation),
      _ => None,
    }
  }
}

impl fmt::Display for CompatibilityDomain {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Graded levels of compatibility across versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityLevel {
  FullyCompatible,
  BackwardCompatibleOnly,
  BreakingChangeMigrationRequired,
  DeprecatedUnsupported,
}

impl CompatibilityLevel {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FullyCompatible => "fully-compatible",
      Self::BackwardCompatibleOnly => "backward-compatible-only",
      Self::BreakingChangeMigrationRequired => "breaking-migration-required",
      Self::DeprecatedUnsupported => "deprecated-unsupported",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "fully-compatible" => Some(Self::FullyCompatible),
      "backward-compatible-only" => Some(Self::BackwardCompatibleOnly),
      "breaking-migration-required" => Some(Self::BreakingChangeMigrationRequired),
      "deprecated-unsupported" => Some(Self::DeprecatedUnsupported),
      _ => None,
    }
  }

  /// Returns true if this level permits execution in the current engine.
  pub const fn is_executable(self) -> bool {
    matches!(
      self,
      Self::FullyCompatible | Self::BackwardCompatibleOnly | Self::BreakingChangeMigrationRequired
    )
  }
}

impl fmt::Display for CompatibilityLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// An entry in the version compatibility matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionMatrixEntry {
  pub domain: CompatibilityDomain,
  pub source_version: String,
  pub target_version: String,
  pub level: CompatibilityLevel,
  pub migration_contract_id: Option<String>,
  pub notes: String,
}

impl VersionMatrixEntry {
  pub fn new(
    domain: CompatibilityDomain,
    source_version: impl Into<String>,
    target_version: impl Into<String>,
    level: CompatibilityLevel,
    migration_contract_id: Option<String>,
    notes: impl Into<String>,
  ) -> Self {
    Self {
      domain,
      source_version: source_version.into(),
      target_version: target_version.into(),
      level,
      migration_contract_id,
      notes: notes.into(),
    }
  }
}

/// Formal compatibility matrix definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityMatrixDefinition {
  pub matrix_id: String,
  pub matrix_version: String,
  pub entries: Vec<VersionMatrixEntry>,
}

/// Typed fail-closed error for compatibility evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
  EmptyMatrix,
  EmptySourceVersion,
  EmptyTargetVersion,
  DuplicateDomainVersionPair(CompatibilityDomain, String, String),
  MissingMigrationContract(CompatibilityDomain, String, String),
  EmptyNotes,
}

impl fmt::Display for CompatibilityError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyMatrix => write!(f, "Compatibility matrix cannot be empty"),
      Self::EmptySourceVersion => write!(f, "Source version string cannot be empty"),
      Self::EmptyTargetVersion => write!(f, "Target version string cannot be empty"),
      Self::DuplicateDomainVersionPair(domain, src, tgt) => {
        write!(
          f,
          "Duplicate compatibility entry for {domain} {src} -> {tgt}"
        )
      }
      Self::MissingMigrationContract(domain, src, tgt) => {
        write!(
          f,
          "Breaking change for {domain} {src} -> {tgt} requires an explicit migration contract ID"
        )
      }
      Self::EmptyNotes => write!(f, "Compatibility entry notes cannot be empty"),
    }
  }
}

/// Compatibility evaluation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityEvaluationReport {
  pub schema_version: &'static str,
  pub matrix_id: String,
  pub total_entries: usize,
  pub fully_compatible_count: usize,
  pub backward_compatible_count: usize,
  pub breaking_with_migration_count: usize,
  pub deprecated_count: usize,
  pub is_matrix_sound: bool,
}

/// Pure deterministic compatibility evaluation function.
pub fn evaluate_compatibility_matrix(
  matrix: &CompatibilityMatrixDefinition,
) -> Result<CompatibilityEvaluationReport, CompatibilityError> {
  if matrix.entries.is_empty() {
    return Err(CompatibilityError::EmptyMatrix);
  }

  let mut fully_compatible = 0usize;
  let mut backward_compatible = 0usize;
  let mut breaking_with_migration = 0usize;
  let mut deprecated = 0usize;

  // Check duplicates and invariants
  for (i, entry) in matrix.entries.iter().enumerate() {
    if entry.source_version.trim().is_empty() {
      return Err(CompatibilityError::EmptySourceVersion);
    }
    if entry.target_version.trim().is_empty() {
      return Err(CompatibilityError::EmptyTargetVersion);
    }
    if entry.notes.trim().is_empty() {
      return Err(CompatibilityError::EmptyNotes);
    }

    if entry.level == CompatibilityLevel::BreakingChangeMigrationRequired {
      match &entry.migration_contract_id {
        Some(id) if !id.trim().is_empty() => {}
        _ => {
          return Err(CompatibilityError::MissingMigrationContract(
            entry.domain,
            entry.source_version.clone(),
            entry.target_version.clone(),
          ));
        }
      }
    }

    for other in &matrix.entries[i + 1..] {
      if entry.domain == other.domain
        && entry.source_version == other.source_version
        && entry.target_version == other.target_version
      {
        return Err(CompatibilityError::DuplicateDomainVersionPair(
          entry.domain,
          entry.source_version.clone(),
          entry.target_version.clone(),
        ));
      }
    }

    match entry.level {
      CompatibilityLevel::FullyCompatible => {
        fully_compatible = fully_compatible.saturating_add(1);
      }
      CompatibilityLevel::BackwardCompatibleOnly => {
        backward_compatible = backward_compatible.saturating_add(1);
      }
      CompatibilityLevel::BreakingChangeMigrationRequired => {
        breaking_with_migration = breaking_with_migration.saturating_add(1);
      }
      CompatibilityLevel::DeprecatedUnsupported => {
        deprecated = deprecated.saturating_add(1);
      }
    }
  }

  Ok(CompatibilityEvaluationReport {
    schema_version: ALPHA_COMPATIBILITY_SCHEMA_VERSION,
    matrix_id: matrix.matrix_id.clone(),
    total_entries: matrix.entries.len(),
    fully_compatible_count: fully_compatible,
    backward_compatible_count: backward_compatible,
    breaking_with_migration_count: breaking_with_migration,
    deprecated_count: deprecated,
    is_matrix_sound: true,
  })
}

/// Renders a structured Markdown report from a CompatibilityEvaluationReport.
pub fn render_compatibility_report_markdown(report: &CompatibilityEvaluationReport) -> String {
  let mut md = String::with_capacity(512);
  md.push_str("# Public Alpha Compatibility Evaluation Report\n\n");
  md.push_str(&format!(
    "- **Schema Version**: `{}`\n",
    report.schema_version
  ));
  md.push_str(&format!("- **Matrix ID**: `{}`\n", report.matrix_id));
  md.push_str(&format!(
    "- **Total Entries**: `{}`\n",
    report.total_entries
  ));
  md.push_str(&format!(
    "- **Fully Compatible**: `{}`\n",
    report.fully_compatible_count
  ));
  md.push_str(&format!(
    "- **Backward Compatible Only**: `{}`\n",
    report.backward_compatible_count
  ));
  md.push_str(&format!(
    "- **Breaking with Migration**: `{}`\n",
    report.breaking_with_migration_count
  ));
  md.push_str(&format!(
    "- **Deprecated / Unsupported**: `{}`\n",
    report.deprecated_count
  ));
  md.push_str(&format!(
    "- **Matrix Sound**: `{}`\n",
    if report.is_matrix_sound { "yes" } else { "no" }
  ));

  md
}
