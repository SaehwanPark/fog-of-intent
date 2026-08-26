//! Public Alpha release governance, policy compliance verification, and legal posture evaluation.

use core::fmt;

/// Canonical schema version for the M12 Alpha governance contract.
pub const ALPHA_GOVERNANCE_SCHEMA_VERSION: &str = "m12-alpha-governance-v1";

/// Maximum integer basis points scale (100.00%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Discrete policy compliance areas required for public alpha release governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyComplianceArea {
  LicenseNotice,
  NonCommercialUse,
  UnofficialDisclaimer,
  OriginalSettingFallback,
  AssetProvenanceAudit,
  ContentIsolation,
}

impl PolicyComplianceArea {
  /// Returns all canonical compliance areas.
  pub const fn all() -> [Self; 6] {
    [
      Self::LicenseNotice,
      Self::NonCommercialUse,
      Self::UnofficialDisclaimer,
      Self::OriginalSettingFallback,
      Self::AssetProvenanceAudit,
      Self::ContentIsolation,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::LicenseNotice => "license-notice",
      Self::NonCommercialUse => "non-commercial-use",
      Self::UnofficialDisclaimer => "unofficial-disclaimer",
      Self::OriginalSettingFallback => "original-setting-fallback",
      Self::AssetProvenanceAudit => "asset-provenance-audit",
      Self::ContentIsolation => "content-isolation",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "license-notice" => Some(Self::LicenseNotice),
      "non-commercial-use" => Some(Self::NonCommercialUse),
      "unofficial-disclaimer" => Some(Self::UnofficialDisclaimer),
      "original-setting-fallback" => Some(Self::OriginalSettingFallback),
      "asset-provenance-audit" => Some(Self::AssetProvenanceAudit),
      "content-isolation" => Some(Self::ContentIsolation),
      _ => None,
    }
  }
}

impl fmt::Display for PolicyComplianceArea {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Categorical legal and distribution posture status for the release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegalPostureStatus {
  CompliantPermissive,
  OriginalFallbackRequired,
  PendingClearance,
  DistributionBlocked,
}

impl LegalPostureStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CompliantPermissive => "compliant-permissive",
      Self::OriginalFallbackRequired => "original-fallback-required",
      Self::PendingClearance => "pending-clearance",
      Self::DistributionBlocked => "distribution-blocked",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "compliant-permissive" => Some(Self::CompliantPermissive),
      "original-fallback-required" => Some(Self::OriginalFallbackRequired),
      "pending-clearance" => Some(Self::PendingClearance),
      "distribution-blocked" => Some(Self::DistributionBlocked),
      _ => None,
    }
  }

  /// Returns true if this posture qualifies for public distribution.
  pub const fn is_distributable(self) -> bool {
    matches!(
      self,
      Self::CompliantPermissive | Self::OriginalFallbackRequired
    )
  }
}

impl fmt::Display for LegalPostureStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// An individual policy compliance declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDeclaration {
  pub area: PolicyComplianceArea,
  pub declaration_id: String,
  pub title: String,
  pub reference_uri: String,
  pub verified: bool,
  pub rationale: String,
}

impl PolicyDeclaration {
  pub fn new(
    area: PolicyComplianceArea,
    declaration_id: impl Into<String>,
    title: impl Into<String>,
    reference_uri: impl Into<String>,
    verified: bool,
    rationale: impl Into<String>,
  ) -> Self {
    Self {
      area,
      declaration_id: declaration_id.into(),
      title: title.into(),
      reference_uri: reference_uri.into(),
      verified,
      rationale: rationale.into(),
    }
  }
}

/// Public Alpha Governance manifest containing all policy declarations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicAlphaGovernanceManifest {
  pub manifest_id: String,
  pub version: String,
  pub declarations: Vec<PolicyDeclaration>,
  pub fallback_universe_name: String,
  pub repository_license: String,
}

/// Typed fail-closed errors for alpha governance evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaGovernanceError {
  EmptyManifest,
  EmptyDeclarationId,
  DuplicateArea(PolicyComplianceArea),
  EmptyTitle,
  EmptyReferenceUri,
  EmptyRationale,
  EmptyFallbackUniverse,
  EmptyLicense,
  InvalidLicense(String),
}

impl fmt::Display for AlphaGovernanceError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyManifest => write!(f, "Governance manifest declarations cannot be empty"),
      Self::EmptyDeclarationId => write!(f, "Declaration ID cannot be empty"),
      Self::DuplicateArea(area) => {
        write!(f, "Duplicate policy compliance area declaration: {area}")
      }
      Self::EmptyTitle => write!(f, "Policy declaration title cannot be empty"),
      Self::EmptyReferenceUri => write!(f, "Policy reference URI cannot be empty"),
      Self::EmptyRationale => write!(f, "Policy verification rationale cannot be empty"),
      Self::EmptyFallbackUniverse => write!(f, "Fallback universe name cannot be empty"),
      Self::EmptyLicense => write!(f, "Repository license cannot be empty"),
      Self::InvalidLicense(lic) => write!(f, "Invalid repository license: '{lic}'"),
    }
  }
}

/// Public Alpha governance evaluation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaGovernanceReport {
  pub schema_version: &'static str,
  pub manifest_id: String,
  pub compliance_score_bp: u32,
  pub verified_areas_count: usize,
  pub total_declared_count: usize,
  pub posture_status: LegalPostureStatus,
  pub is_release_eligible: bool,
  pub missing_areas: Vec<PolicyComplianceArea>,
}

/// Pure deterministic governance evaluation auditing manifest declarations and compliance.
pub fn evaluate_alpha_governance(
  manifest: &PublicAlphaGovernanceManifest,
) -> Result<AlphaGovernanceReport, AlphaGovernanceError> {
  if manifest.declarations.is_empty() {
    return Err(AlphaGovernanceError::EmptyManifest);
  }
  if manifest.fallback_universe_name.trim().is_empty() {
    return Err(AlphaGovernanceError::EmptyFallbackUniverse);
  }
  if manifest.repository_license.trim().is_empty() {
    return Err(AlphaGovernanceError::EmptyLicense);
  }
  if manifest.repository_license != "MIT" && manifest.repository_license != "Apache-2.0" {
    return Err(AlphaGovernanceError::InvalidLicense(
      manifest.repository_license.clone(),
    ));
  }

  let mut seen_areas = [false; 6];
  let mut verified_count = 0usize;
  let mut license_verified = false;
  let mut provenance_verified = false;
  let mut noncommercial_verified = false;
  let mut unofficial_verified = false;
  let mut fallback_verified = false;
  let mut isolation_verified = false;

  for decl in &manifest.declarations {
    if decl.declaration_id.trim().is_empty() {
      return Err(AlphaGovernanceError::EmptyDeclarationId);
    }
    if decl.title.trim().is_empty() {
      return Err(AlphaGovernanceError::EmptyTitle);
    }
    if decl.reference_uri.trim().is_empty() {
      return Err(AlphaGovernanceError::EmptyReferenceUri);
    }
    if decl.rationale.trim().is_empty() {
      return Err(AlphaGovernanceError::EmptyRationale);
    }

    let area_idx = match decl.area {
      PolicyComplianceArea::LicenseNotice => 0,
      PolicyComplianceArea::NonCommercialUse => 1,
      PolicyComplianceArea::UnofficialDisclaimer => 2,
      PolicyComplianceArea::OriginalSettingFallback => 3,
      PolicyComplianceArea::AssetProvenanceAudit => 4,
      PolicyComplianceArea::ContentIsolation => 5,
    };

    if seen_areas[area_idx] {
      return Err(AlphaGovernanceError::DuplicateArea(decl.area));
    }
    seen_areas[area_idx] = true;

    if decl.verified {
      verified_count = verified_count.saturating_add(1);
      match decl.area {
        PolicyComplianceArea::LicenseNotice => license_verified = true,
        PolicyComplianceArea::NonCommercialUse => noncommercial_verified = true,
        PolicyComplianceArea::UnofficialDisclaimer => unofficial_verified = true,
        PolicyComplianceArea::OriginalSettingFallback => fallback_verified = true,
        PolicyComplianceArea::AssetProvenanceAudit => provenance_verified = true,
        PolicyComplianceArea::ContentIsolation => isolation_verified = true,
      }
    }
  }

  let all_canonical = PolicyComplianceArea::all();
  let mut missing_areas = Vec::new();
  for (idx, &seen) in seen_areas.iter().enumerate() {
    if !seen {
      missing_areas.push(all_canonical[idx]);
    }
  }

  let total_canonical = all_canonical.len();
  let verified_u32 = u32::try_from(verified_count).unwrap_or(0);
  let total_u32 = u32::try_from(total_canonical).unwrap_or(1);

  let compliance_score_bp = verified_u32.saturating_mul(MAX_BASIS_POINTS) / total_u32;

  // Determine legal posture:
  // 1. If core license or asset provenance is unverified -> DistributionBlocked
  // 2. If all 6 verified -> CompliantPermissive
  // 3. If unofficial/fallback active and core areas verified -> OriginalFallbackRequired
  // 4. Otherwise -> PendingClearance
  let posture_status = if !license_verified || !provenance_verified {
    LegalPostureStatus::DistributionBlocked
  } else if verified_count == total_canonical {
    LegalPostureStatus::CompliantPermissive
  } else if license_verified
    && provenance_verified
    && fallback_verified
    && isolation_verified
    && (!unofficial_verified || !noncommercial_verified)
  {
    LegalPostureStatus::OriginalFallbackRequired
  } else {
    LegalPostureStatus::PendingClearance
  };

  let is_release_eligible = posture_status.is_distributable() && compliance_score_bp >= 8_000;

  Ok(AlphaGovernanceReport {
    schema_version: ALPHA_GOVERNANCE_SCHEMA_VERSION,
    manifest_id: manifest.manifest_id.clone(),
    compliance_score_bp,
    verified_areas_count: verified_count,
    total_declared_count: manifest.declarations.len(),
    posture_status,
    is_release_eligible,
    missing_areas,
  })
}

/// Renders a structured Markdown report from an AlphaGovernanceReport.
pub fn render_governance_report_markdown(report: &AlphaGovernanceReport) -> String {
  let mut md = String::with_capacity(512);
  md.push_str("# Public Alpha Governance Evaluation Report\n\n");
  md.push_str(&format!(
    "- **Schema Version**: `{}`\n",
    report.schema_version
  ));
  md.push_str(&format!("- **Manifest ID**: `{}`\n", report.manifest_id));
  md.push_str(&format!(
    "- **Compliance Score**: `{}` bp\n",
    report.compliance_score_bp
  ));
  md.push_str(&format!(
    "- **Verified Areas**: `{}/{}`\n",
    report.verified_areas_count, report.total_declared_count
  ));
  md.push_str(&format!(
    "- **Posture Status**: `{}`\n",
    report.posture_status
  ));
  md.push_str(&format!(
    "- **Release Eligible**: `{}`\n",
    if report.is_release_eligible {
      "yes"
    } else {
      "no"
    }
  ));

  if !report.missing_areas.is_empty() {
    md.push_str("\n### Missing Compliance Areas\n\n");
    for area in &report.missing_areas {
      md.push_str(&format!("- `{area}`\n"));
    }
  }

  md
}
