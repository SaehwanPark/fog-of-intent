//! Public Alpha release verification and multi-domain check suite: clean-install, reproducibility, security, license, compatibility, and data redaction integrity.

use core::fmt;
use std::collections::{HashMap, HashSet};

use super::reproducibility::is_valid_fnv1a_hash;

/// Canonical schema version for the M12 Alpha release checks contract.
pub const ALPHA_RELEASE_CHECKS_SCHEMA_VERSION: &str = "m12-alpha-release-checks-v1";

/// Maximum integer basis points scale (100.00%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Minimum readiness score basis points (85.00%) required for release readiness.
pub const MIN_RELEASE_READINESS_BP: u32 = 8_500;

/// Discrete check categories required for public alpha release verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReleaseCheckCategory {
  CleanInstall,
  Reproducibility,
  SecurityAdvisory,
  LicenseCompliance,
  CompatibilityMatrix,
  DataRedaction,
}

impl ReleaseCheckCategory {
  /// Returns all canonical release check categories.
  pub const fn all() -> [Self; 6] {
    [
      Self::CleanInstall,
      Self::Reproducibility,
      Self::SecurityAdvisory,
      Self::LicenseCompliance,
      Self::CompatibilityMatrix,
      Self::DataRedaction,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CleanInstall => "clean-install",
      Self::Reproducibility => "reproducibility",
      Self::SecurityAdvisory => "security-advisory",
      Self::LicenseCompliance => "license-compliance",
      Self::CompatibilityMatrix => "compatibility-matrix",
      Self::DataRedaction => "data-redaction",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "clean-install" => Some(Self::CleanInstall),
      "reproducibility" => Some(Self::Reproducibility),
      "security-advisory" => Some(Self::SecurityAdvisory),
      "license-compliance" => Some(Self::LicenseCompliance),
      "compatibility-matrix" => Some(Self::CompatibilityMatrix),
      "data-redaction" => Some(Self::DataRedaction),
      _ => None,
    }
  }
}

impl fmt::Display for ReleaseCheckCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Severity classification of a release verification check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReleaseCheckSeverity {
  CriticalBlocker,
  MajorIssue,
  MinorWarning,
  VerifiedPass,
}

impl ReleaseCheckSeverity {
  /// Returns all canonical severity levels.
  pub const fn all() -> [Self; 4] {
    [
      Self::CriticalBlocker,
      Self::MajorIssue,
      Self::MinorWarning,
      Self::VerifiedPass,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CriticalBlocker => "critical-blocker",
      Self::MajorIssue => "major-issue",
      Self::MinorWarning => "minor-warning",
      Self::VerifiedPass => "verified-pass",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "critical-blocker" => Some(Self::CriticalBlocker),
      "major-issue" => Some(Self::MajorIssue),
      "minor-warning" => Some(Self::MinorWarning),
      "verified-pass" => Some(Self::VerifiedPass),
      _ => None,
    }
  }

  /// Returns true if this severity level represents a blocking issue if failed.
  pub const fn is_blocking(self) -> bool {
    matches!(self, Self::CriticalBlocker | Self::MajorIssue)
  }
}

impl fmt::Display for ReleaseCheckSeverity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Verification status of an individual release check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckVerificationStatus {
  Passed,
  ConditionallyPassed,
  Failed,
  Skipped,
}

impl CheckVerificationStatus {
  /// Returns all canonical verification statuses.
  pub const fn all() -> [Self; 4] {
    [
      Self::Passed,
      Self::ConditionallyPassed,
      Self::Failed,
      Self::Skipped,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Passed => "passed",
      Self::ConditionallyPassed => "conditionally-passed",
      Self::Failed => "failed",
      Self::Skipped => "skipped",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "passed" => Some(Self::Passed),
      "conditionally-passed" => Some(Self::ConditionallyPassed),
      "failed" => Some(Self::Failed),
      "skipped" => Some(Self::Skipped),
      _ => None,
    }
  }

  /// Returns true if the status is considered passing.
  pub const fn is_successful(self) -> bool {
    matches!(self, Self::Passed | Self::ConditionallyPassed)
  }

  /// Returns the basis points contribution weight for this verification status.
  pub const fn score_weight_bp(self) -> u32 {
    match self {
      Self::Passed => 10_000,
      Self::ConditionallyPassed => 7_500,
      Self::Skipped => 5_000,
      Self::Failed => 0,
    }
  }
}

impl fmt::Display for CheckVerificationStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Definition of an individual release verification check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCheckDefinition {
  pub check_id: &'static str,
  pub category: ReleaseCheckCategory,
  pub title: &'static str,
  pub description: &'static str,
  pub severity: ReleaseCheckSeverity,
  pub status: CheckVerificationStatus,
  pub evidence_command: &'static str,
  pub evidence_hash: &'static str,
  pub mitigation_notes: Option<&'static str>,
}

/// Public Alpha release readiness checks manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaReleaseChecksManifest {
  pub schema_version: &'static str,
  pub manifest_id: &'static str,
  pub release_version: &'static str,
  pub target_commit: &'static str,
  pub checks: &'static [ReleaseCheckDefinition],
}

/// Audit record for an individual release check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCheckAuditRecord {
  pub check_id: &'static str,
  pub category: ReleaseCheckCategory,
  pub severity: ReleaseCheckSeverity,
  pub status: CheckVerificationStatus,
  pub score_weight_bp: u32,
  pub hash_valid: bool,
  pub is_passing: bool,
}

/// Category audit summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryAuditSummary {
  pub category: ReleaseCheckCategory,
  pub total_checks: usize,
  pub passed_checks: usize,
  pub has_critical_blocker: bool,
}

/// Audit report summarizing release readiness verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseChecksAuditReport {
  pub schema_version: &'static str,
  pub manifest_id: &'static str,
  pub release_version: &'static str,
  pub target_commit: &'static str,
  pub total_checks: usize,
  pub passed_checks: usize,
  pub conditionally_passed_checks: usize,
  pub failed_checks: usize,
  pub skipped_checks: usize,
  pub critical_blockers_count: usize,
  pub readiness_score_bp: u32,
  pub is_release_ready: bool,
  pub records: Vec<ReleaseCheckAuditRecord>,
  pub category_summaries: Vec<CategoryAuditSummary>,
}

/// Errors encountered during release checks audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaReleaseChecksError {
  EmptyManifest,
  UnsupportedSchemaVersion {
    version: String,
  },
  EmptyManifestId,
  EmptyReleaseVersion,
  EmptyTargetCommit,
  ZeroChecks,
  EmptyCheckId,
  DuplicateCheckId {
    check_id: String,
  },
  EmptyTitle {
    check_id: String,
  },
  EmptyDescription {
    check_id: String,
  },
  EmptyEvidenceCommand {
    check_id: String,
  },
  InvalidEvidenceHash {
    check_id: String,
    hash: String,
  },
  CriticalBlockerDetected {
    check_id: String,
    category: String,
    description: String,
  },
  MissingRequiredCategory {
    category: String,
  },
}

impl fmt::Display for AlphaReleaseChecksError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyManifest => write!(f, "release checks manifest is empty"),
      Self::UnsupportedSchemaVersion { version } => {
        write!(
          f,
          "unsupported release checks schema version '{version}'; expected '{ALPHA_RELEASE_CHECKS_SCHEMA_VERSION}'"
        )
      }
      Self::EmptyManifestId => write!(f, "release checks manifest ID cannot be empty"),
      Self::EmptyReleaseVersion => write!(f, "release version cannot be empty"),
      Self::EmptyTargetCommit => write!(f, "target commit hash cannot be empty"),
      Self::ZeroChecks => write!(f, "manifest must declare at least one release check"),
      Self::EmptyCheckId => write!(f, "check ID cannot be empty"),
      Self::DuplicateCheckId { check_id } => {
        write!(f, "duplicate check ID '{check_id}' detected in manifest")
      }
      Self::EmptyTitle { check_id } => {
        write!(f, "check '{check_id}' has an empty title")
      }
      Self::EmptyDescription { check_id } => {
        write!(f, "check '{check_id}' has an empty description")
      }
      Self::EmptyEvidenceCommand { check_id } => {
        write!(f, "check '{check_id}' has an empty evidence command")
      }
      Self::InvalidEvidenceHash { check_id, hash } => {
        write!(
          f,
          "check '{check_id}' has invalid FNV-1a evidence hash '{hash}'; expected 16-hex character checksum"
        )
      }
      Self::CriticalBlockerDetected {
        check_id,
        category,
        description,
      } => {
        write!(
          f,
          "critical blocker detected in check '{check_id}' (category: '{category}'): {description}"
        )
      }
      Self::MissingRequiredCategory { category } => {
        write!(
          f,
          "manifest is missing required release check category '{category}'"
        )
      }
    }
  }
}

impl std::error::Error for AlphaReleaseChecksError {}

/// Pure deterministic audit evaluating a release checks manifest for public alpha readiness.
pub fn audit_release_checks(
  manifest: &AlphaReleaseChecksManifest,
) -> Result<ReleaseChecksAuditReport, AlphaReleaseChecksError> {
  if manifest.schema_version != ALPHA_RELEASE_CHECKS_SCHEMA_VERSION {
    return Err(AlphaReleaseChecksError::UnsupportedSchemaVersion {
      version: manifest.schema_version.to_string(),
    });
  }

  if manifest.manifest_id.trim().is_empty() {
    return Err(AlphaReleaseChecksError::EmptyManifestId);
  }

  if manifest.release_version.trim().is_empty() {
    return Err(AlphaReleaseChecksError::EmptyReleaseVersion);
  }

  if manifest.target_commit.trim().is_empty() {
    return Err(AlphaReleaseChecksError::EmptyTargetCommit);
  }

  if manifest.checks.is_empty() {
    return Err(AlphaReleaseChecksError::ZeroChecks);
  }

  let mut seen_ids = HashSet::new();
  let mut seen_categories = HashSet::new();
  let mut category_map: HashMap<ReleaseCheckCategory, (usize, usize, bool)> = HashMap::new();

  let mut records = Vec::with_capacity(manifest.checks.len());
  let mut passed_count = 0;
  let mut conditionally_passed_count = 0;
  let mut failed_count = 0;
  let mut skipped_count = 0;
  let mut critical_blockers_count = 0;
  let mut total_score_bp = 0u64;

  for check in manifest.checks {
    if check.check_id.trim().is_empty() {
      return Err(AlphaReleaseChecksError::EmptyCheckId);
    }
    if !seen_ids.insert(check.check_id) {
      return Err(AlphaReleaseChecksError::DuplicateCheckId {
        check_id: check.check_id.to_string(),
      });
    }

    if check.title.trim().is_empty() {
      return Err(AlphaReleaseChecksError::EmptyTitle {
        check_id: check.check_id.to_string(),
      });
    }

    if check.description.trim().is_empty() {
      return Err(AlphaReleaseChecksError::EmptyDescription {
        check_id: check.check_id.to_string(),
      });
    }

    if check.evidence_command.trim().is_empty() {
      return Err(AlphaReleaseChecksError::EmptyEvidenceCommand {
        check_id: check.check_id.to_string(),
      });
    }

    if !is_valid_fnv1a_hash(check.evidence_hash) {
      return Err(AlphaReleaseChecksError::InvalidEvidenceHash {
        check_id: check.check_id.to_string(),
        hash: check.evidence_hash.to_string(),
      });
    }

    seen_categories.insert(check.category);

    let is_critical_blocker = check.severity == ReleaseCheckSeverity::CriticalBlocker
      && check.status == CheckVerificationStatus::Failed;

    if is_critical_blocker {
      return Err(AlphaReleaseChecksError::CriticalBlockerDetected {
        check_id: check.check_id.to_string(),
        category: check.category.as_str().to_string(),
        description: check.description.to_string(),
      });
    }

    let is_passing = check.status.is_successful();
    let weight_bp = check.status.score_weight_bp();
    total_score_bp = total_score_bp.saturating_add(u64::from(weight_bp));

    match check.status {
      CheckVerificationStatus::Passed => passed_count += 1,
      CheckVerificationStatus::ConditionallyPassed => conditionally_passed_count += 1,
      CheckVerificationStatus::Failed => {
        failed_count += 1;
        if check.severity.is_blocking() {
          critical_blockers_count += 1;
        }
      }
      CheckVerificationStatus::Skipped => skipped_count += 1,
    }

    let entry = category_map.entry(check.category).or_insert((0, 0, false));
    entry.0 += 1;
    if is_passing {
      entry.1 += 1;
    }
    if is_critical_blocker {
      entry.2 = true;
    }

    records.push(ReleaseCheckAuditRecord {
      check_id: check.check_id,
      category: check.category,
      severity: check.severity,
      status: check.status,
      score_weight_bp: weight_bp,
      hash_valid: true,
      is_passing,
    });
  }

  // Validate that all required categories are present.
  for req_cat in ReleaseCheckCategory::all() {
    if !seen_categories.contains(&req_cat) {
      return Err(AlphaReleaseChecksError::MissingRequiredCategory {
        category: req_cat.as_str().to_string(),
      });
    }
  }

  let total_checks = manifest.checks.len();
  let readiness_score_bp = if total_checks > 0 {
    let divisor = u64::try_from(total_checks).unwrap_or(1);
    u32::try_from(total_score_bp / divisor).unwrap_or(MAX_BASIS_POINTS)
  } else {
    0
  };

  let is_release_ready = total_checks >= ReleaseCheckCategory::all().len()
    && failed_count == 0
    && critical_blockers_count == 0
    && readiness_score_bp >= MIN_RELEASE_READINESS_BP;

  let mut category_summaries = Vec::with_capacity(ReleaseCheckCategory::all().len());
  for cat in ReleaseCheckCategory::all() {
    if let Some(&(total, passed, blocker)) = category_map.get(&cat) {
      category_summaries.push(CategoryAuditSummary {
        category: cat,
        total_checks: total,
        passed_checks: passed,
        has_critical_blocker: blocker,
      });
    }
  }

  Ok(ReleaseChecksAuditReport {
    schema_version: manifest.schema_version,
    manifest_id: manifest.manifest_id,
    release_version: manifest.release_version,
    target_commit: manifest.target_commit,
    total_checks,
    passed_checks: passed_count,
    conditionally_passed_checks: conditionally_passed_count,
    failed_checks: failed_count,
    skipped_checks: skipped_count,
    critical_blockers_count,
    readiness_score_bp,
    is_release_ready,
    records,
    category_summaries,
  })
}

/// Renders a structured Markdown report of release verification checks without ANSI styling.
pub fn render_release_checks_report_markdown(report: &ReleaseChecksAuditReport) -> String {
  let readiness_status = if report.is_release_ready {
    "READY FOR PUBLIC ALPHA"
  } else {
    "RELEASE BLOCKED / PENDING"
  };

  let mut md = String::with_capacity(2048);
  md.push_str("# Fog of Intent — Public Alpha Release Readiness Audit Report\n\n");
  md.push_str(&format!(
    "- **Schema Version:** `{}`\n",
    report.schema_version
  ));
  md.push_str(&format!("- **Manifest ID:** `{}`\n", report.manifest_id));
  md.push_str(&format!(
    "- **Release Version:** `{}`\n",
    report.release_version
  ));
  md.push_str(&format!(
    "- **Target Commit:** `{}`\n",
    report.target_commit
  ));
  md.push_str(&format!(
    "- **Readiness Status:** **{}**\n",
    readiness_status
  ));
  md.push_str(&format!(
    "- **Readiness Score:** {:.2}% ({} / {} bp)\n",
    f64::from(report.readiness_score_bp) / 100.0,
    report.readiness_score_bp,
    MAX_BASIS_POINTS
  ));
  md.push_str(&format!(
    "- **Checks Breakdown:** {} total ({} passed, {} conditional, {} failed, {} skipped)\n",
    report.total_checks,
    report.passed_checks,
    report.conditionally_passed_checks,
    report.failed_checks,
    report.skipped_checks
  ));
  md.push_str(&format!(
    "- **Critical Blockers:** {}\n\n",
    report.critical_blockers_count
  ));

  md.push_str("## Category Audit Summaries\n\n");
  md.push_str("| Category | Total Checks | Passed Checks | Critical Blocker |\n");
  md.push_str("| --- | --- | --- | --- |\n");
  for cat in &report.category_summaries {
    md.push_str(&format!(
      "| `{}` | {} | {} | {} |\n",
      cat.category.as_str(),
      cat.total_checks,
      cat.passed_checks,
      if cat.has_critical_blocker {
        "YES (FAIL)"
      } else {
        "None"
      }
    ));
  }
  md.push('\n');

  md.push_str("## Detailed Check Verification Records\n\n");
  md.push_str("| Check ID | Category | Severity | Status | Score Weight | Hash Valid |\n");
  md.push_str("| --- | --- | --- | --- | --- | --- |\n");
  for rec in &report.records {
    md.push_str(&format!(
      "| `{}` | `{}` | `{}` | `{}` | {} bp | {} |\n",
      rec.check_id,
      rec.category.as_str(),
      rec.severity.as_str(),
      rec.status.as_str(),
      rec.score_weight_bp,
      if rec.hash_valid { "YES" } else { "NO" }
    ));
  }

  md
}
