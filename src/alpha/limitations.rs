//! Public Alpha known limitations, evidence boundaries, research claim constraints, and citation guidance.

use core::fmt;

/// Canonical schema version for the M12 Alpha limitations contract.
pub const ALPHA_LIMITATIONS_SCHEMA_VERSION: &str = "m12-alpha-limitations-v1";

/// Maximum integer basis points scale (100.00%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Discrete limitation categories defining technical and empirical boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitationCategory {
  SimulationFidelity,
  AccessibilityCoverage,
  AgentGeneralization,
  HumanRealism,
  NetworkMultiplayer,
  HardwareRequirements,
}

impl LimitationCategory {
  /// Returns all canonical limitation categories.
  pub const fn all() -> [Self; 6] {
    [
      Self::SimulationFidelity,
      Self::AccessibilityCoverage,
      Self::AgentGeneralization,
      Self::HumanRealism,
      Self::NetworkMultiplayer,
      Self::HardwareRequirements,
    ]
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SimulationFidelity => "simulation-fidelity",
      Self::AccessibilityCoverage => "accessibility-coverage",
      Self::AgentGeneralization => "agent-generalization",
      Self::HumanRealism => "human-realism",
      Self::NetworkMultiplayer => "network-multiplayer",
      Self::HardwareRequirements => "hardware-requirements",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "simulation-fidelity" => Some(Self::SimulationFidelity),
      "accessibility-coverage" => Some(Self::AccessibilityCoverage),
      "agent-generalization" => Some(Self::AgentGeneralization),
      "human-realism" => Some(Self::HumanRealism),
      "network-multiplayer" => Some(Self::NetworkMultiplayer),
      "hardware-requirements" => Some(Self::HardwareRequirements),
      _ => None,
    }
  }
}

impl fmt::Display for LimitationCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Discrete evidence tiers substantiating research and engineering claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceTier {
  SoftwareInvariants,
  SyntheticAgentPlaytest,
  EmpiricalCalibration,
  LimitedHumanStudy,
  UnverifiedHypothesis,
}

impl EvidenceTier {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SoftwareInvariants => "software-invariants",
      Self::SyntheticAgentPlaytest => "synthetic-agent-playtest",
      Self::EmpiricalCalibration => "empirical-calibration",
      Self::LimitedHumanStudy => "limited-human-study",
      Self::UnverifiedHypothesis => "unverified-hypothesis",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "software-invariants" => Some(Self::SoftwareInvariants),
      "synthetic-agent-playtest" => Some(Self::SyntheticAgentPlaytest),
      "empirical-calibration" => Some(Self::EmpiricalCalibration),
      "limited-human-study" => Some(Self::LimitedHumanStudy),
      "unverified-hypothesis" => Some(Self::UnverifiedHypothesis),
      _ => None,
    }
  }

  /// Returns true if this evidence tier is empirical rather than speculative.
  pub const fn is_empirical(self) -> bool {
    matches!(
      self,
      Self::SoftwareInvariants
        | Self::SyntheticAgentPlaytest
        | Self::EmpiricalCalibration
        | Self::LimitedHumanStudy
    )
  }
}

impl fmt::Display for EvidenceTier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Claim classification enforcing research claim hygiene and overclaim prevention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimClassification {
  PermissibleBoundedClaim,
  ConditionalWithDisclaimer,
  ImpermissibleOverclaim,
}

impl ClaimClassification {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::PermissibleBoundedClaim => "permissible-bounded-claim",
      Self::ConditionalWithDisclaimer => "conditional-with-disclaimer",
      Self::ImpermissibleOverclaim => "impermissible-overclaim",
    }
  }

  pub fn parse(s: &str) -> Option<Self> {
    match s {
      "permissible-bounded-claim" => Some(Self::PermissibleBoundedClaim),
      "conditional-with-disclaimer" => Some(Self::ConditionalWithDisclaimer),
      "impermissible-overclaim" => Some(Self::ImpermissibleOverclaim),
      _ => None,
    }
  }

  /// Returns true if this classification is allowable in release documentation.
  pub const fn is_allowed(self) -> bool {
    matches!(
      self,
      Self::PermissibleBoundedClaim | Self::ConditionalWithDisclaimer
    )
  }
}

impl fmt::Display for ClaimClassification {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// An individual research or capability claim with evidence bounds and disclaimers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchClaim {
  pub claim_id: String,
  pub category: LimitationCategory,
  pub statement: String,
  pub supporting_tier: EvidenceTier,
  pub disclaimed_limitations: Vec<LimitationCategory>,
  pub classification: ClaimClassification,
  pub rationale: String,
}

impl ResearchClaim {
  pub fn new(
    claim_id: impl Into<String>,
    category: LimitationCategory,
    statement: impl Into<String>,
    supporting_tier: EvidenceTier,
    disclaimed_limitations: Vec<LimitationCategory>,
    classification: ClaimClassification,
    rationale: impl Into<String>,
  ) -> Self {
    Self {
      claim_id: claim_id.into(),
      category,
      statement: statement.into(),
      supporting_tier,
      disclaimed_limitations,
      classification,
      rationale: rationale.into(),
    }
  }
}

/// Canonical citation guidance and reproducibility metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationGuidance {
  pub bibtex_entry: String,
  pub doi_or_urn: String,
  pub canonical_title: String,
  pub software_version: String,
  pub repository_url: String,
  pub reproducibility_seed_policy: String,
}

impl CitationGuidance {
  pub fn new(
    bibtex_entry: impl Into<String>,
    doi_or_urn: impl Into<String>,
    canonical_title: impl Into<String>,
    software_version: impl Into<String>,
    repository_url: impl Into<String>,
    reproducibility_seed_policy: impl Into<String>,
  ) -> Self {
    Self {
      bibtex_entry: bibtex_entry.into(),
      doi_or_urn: doi_or_urn.into(),
      canonical_title: canonical_title.into(),
      software_version: software_version.into(),
      repository_url: repository_url.into(),
      reproducibility_seed_policy: reproducibility_seed_policy.into(),
    }
  }
}

/// Public Alpha limitations and evidence boundaries declaration manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaLimitationsDeclaration {
  pub manifest_id: String,
  pub version: String,
  pub claims: Vec<ResearchClaim>,
  pub citation: CitationGuidance,
  pub disclosed_limitations: Vec<LimitationCategory>,
}

/// Typed fail-closed errors for limitations and claim boundary auditing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlphaLimitationsError {
  EmptyManifest,
  EmptyClaimId,
  EmptyStatement,
  EmptyRationale,
  DuplicateClaimId(String),
  ImpermissibleClaimDetected(String),
  MissingRequiredDisclaimer {
    claim_id: String,
    required_limitation: LimitationCategory,
  },
  EmptyBibtex,
  EmptyDoiOrUrn,
  EmptyCanonicalTitle,
  EmptyRepositoryUrl,
  EmptySeedPolicy,
  EmptyDisclosedLimitations,
}

impl fmt::Display for AlphaLimitationsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyManifest => write!(f, "Limitations declaration claims cannot be empty"),
      Self::EmptyClaimId => write!(f, "Claim ID cannot be empty"),
      Self::EmptyStatement => write!(f, "Claim statement cannot be empty"),
      Self::EmptyRationale => write!(f, "Claim rationale cannot be empty"),
      Self::DuplicateClaimId(id) => write!(f, "Duplicate claim ID in declaration: '{id}'"),
      Self::ImpermissibleClaimDetected(id) => {
        write!(f, "Impermissible overclaim detected in claim: '{id}'")
      }
      Self::MissingRequiredDisclaimer {
        claim_id,
        required_limitation,
      } => write!(
        f,
        "Claim '{claim_id}' requires disclaimer for category: {required_limitation}"
      ),
      Self::EmptyBibtex => write!(f, "Citation BibTeX entry cannot be empty"),
      Self::EmptyDoiOrUrn => write!(f, "Citation DOI/URN identifier cannot be empty"),
      Self::EmptyCanonicalTitle => write!(f, "Citation canonical title cannot be empty"),
      Self::EmptyRepositoryUrl => write!(f, "Citation repository URL cannot be empty"),
      Self::EmptySeedPolicy => {
        write!(f, "Citation reproducibility seed policy cannot be empty")
      }
      Self::EmptyDisclosedLimitations => {
        write!(f, "Manifest must disclose at least one limitation category")
      }
    }
  }
}

/// Audit report evaluating limitations, claim boundaries, and citation completeness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimitationsAuditReport {
  pub schema_version: &'static str,
  pub manifest_id: String,
  pub safety_score_bp: u32,
  pub total_claims_count: usize,
  pub permissible_claims_count: usize,
  pub conditional_claims_count: usize,
  pub disclosed_limitations_count: usize,
  pub is_audit_passed: bool,
  pub citation: CitationGuidance,
}

/// Pure deterministic audit evaluating research claims, boundary disclaimers, and citation completeness.
pub fn audit_limitations_and_boundaries(
  manifest: &AlphaLimitationsDeclaration,
) -> Result<LimitationsAuditReport, AlphaLimitationsError> {
  if manifest.claims.is_empty() {
    return Err(AlphaLimitationsError::EmptyManifest);
  }
  if manifest.disclosed_limitations.is_empty() {
    return Err(AlphaLimitationsError::EmptyDisclosedLimitations);
  }
  if manifest.citation.bibtex_entry.trim().is_empty() {
    return Err(AlphaLimitationsError::EmptyBibtex);
  }
  if manifest.citation.doi_or_urn.trim().is_empty() {
    return Err(AlphaLimitationsError::EmptyDoiOrUrn);
  }
  if manifest.citation.canonical_title.trim().is_empty() {
    return Err(AlphaLimitationsError::EmptyCanonicalTitle);
  }
  if manifest.citation.repository_url.trim().is_empty() {
    return Err(AlphaLimitationsError::EmptyRepositoryUrl);
  }
  if manifest
    .citation
    .reproducibility_seed_policy
    .trim()
    .is_empty()
  {
    return Err(AlphaLimitationsError::EmptySeedPolicy);
  }

  let mut seen_ids = Vec::with_capacity(manifest.claims.len());
  let mut permissible_count = 0usize;
  let mut conditional_count = 0usize;

  for claim in &manifest.claims {
    if claim.claim_id.trim().is_empty() {
      return Err(AlphaLimitationsError::EmptyClaimId);
    }
    if claim.statement.trim().is_empty() {
      return Err(AlphaLimitationsError::EmptyStatement);
    }
    if claim.rationale.trim().is_empty() {
      return Err(AlphaLimitationsError::EmptyRationale);
    }
    if seen_ids.contains(&claim.claim_id.as_str()) {
      return Err(AlphaLimitationsError::DuplicateClaimId(
        claim.claim_id.clone(),
      ));
    }

    seen_ids.push(claim.claim_id.as_str());

    match claim.classification {
      ClaimClassification::ImpermissibleOverclaim => {
        return Err(AlphaLimitationsError::ImpermissibleClaimDetected(
          claim.claim_id.clone(),
        ));
      }
      ClaimClassification::PermissibleBoundedClaim => {
        permissible_count = permissible_count.saturating_add(1);
      }
      ClaimClassification::ConditionalWithDisclaimer => {
        if claim.disclaimed_limitations.is_empty() {
          return Err(AlphaLimitationsError::MissingRequiredDisclaimer {
            claim_id: claim.claim_id.clone(),
            required_limitation: claim.category,
          });
        }
        for &disclaimed in &claim.disclaimed_limitations {
          if !manifest.disclosed_limitations.contains(&disclaimed) {
            return Err(AlphaLimitationsError::MissingRequiredDisclaimer {
              claim_id: claim.claim_id.clone(),
              required_limitation: disclaimed,
            });
          }
        }
        conditional_count = conditional_count.saturating_add(1);
      }
    }
  }

  let total_count = manifest.claims.len();
  let perm_u32 = u32::try_from(permissible_count).unwrap_or(0);
  let cond_u32 = u32::try_from(conditional_count).unwrap_or(0);
  let total_u32 = u32::try_from(total_count).unwrap_or(1);

  // Basis point safety score: Permissible claims contribute 10,000 bp weight, Conditional claims contribute 8,000 bp weight
  let weighted_sum = perm_u32
    .saturating_mul(MAX_BASIS_POINTS)
    .saturating_add(cond_u32.saturating_mul(8_000));
  let safety_score_bp = weighted_sum / total_u32;

  let is_audit_passed = safety_score_bp >= 8_000 && !manifest.disclosed_limitations.is_empty();

  Ok(LimitationsAuditReport {
    schema_version: ALPHA_LIMITATIONS_SCHEMA_VERSION,
    manifest_id: manifest.manifest_id.clone(),
    safety_score_bp,
    total_claims_count: total_count,
    permissible_claims_count: permissible_count,
    conditional_claims_count: conditional_count,
    disclosed_limitations_count: manifest.disclosed_limitations.len(),
    is_audit_passed,
    citation: manifest.citation.clone(),
  })
}

/// Renders a structured Markdown report from a LimitationsAuditReport.
pub fn render_limitations_report_markdown(report: &LimitationsAuditReport) -> String {
  let mut md = String::with_capacity(512);
  md.push_str("# Public Alpha Known Limitations and Evidence Boundaries Report\n\n");
  md.push_str(&format!(
    "- **Schema Version**: `{}`\n",
    report.schema_version
  ));
  md.push_str(&format!("- **Manifest ID**: `{}`\n", report.manifest_id));
  md.push_str(&format!(
    "- **Claim Safety Score**: `{}` bp\n",
    report.safety_score_bp
  ));
  md.push_str(&format!(
    "- **Permissible Claims**: `{}/{}`\n",
    report.permissible_claims_count, report.total_claims_count
  ));
  md.push_str(&format!(
    "- **Conditional Claims**: `{}/{}`\n",
    report.conditional_claims_count, report.total_claims_count
  ));
  md.push_str(&format!(
    "- **Disclosed Limitations**: `{}` categories\n",
    report.disclosed_limitations_count
  ));
  md.push_str(&format!(
    "- **Audit Status**: `{}`\n\n",
    if report.is_audit_passed {
      "passed"
    } else {
      "failed"
    }
  ));

  md.push_str("### Canonical Citation\n\n");
  md.push_str(&format!(
    "- **Title**: {}\n",
    report.citation.canonical_title
  ));
  md.push_str(&format!(
    "- **DOI/URN**: `{}`\n",
    report.citation.doi_or_urn
  ));
  md.push_str(&format!(
    "- **Software Version**: `{}`\n",
    report.citation.software_version
  ));
  md.push_str(&format!(
    "- **Repository**: {}\n",
    report.citation.repository_url
  ));
  md.push_str(&format!(
    "- **Seed Policy**: {}\n\n",
    report.citation.reproducibility_seed_policy
  ));

  md.push_str("```bibtex\n");
  md.push_str(&report.citation.bibtex_entry);
  md.push_str("\n```\n");

  md
}
