//! Benchmark scenarios and canonical test catalogs for Public Alpha governance, compatibility, limitations, guides, and reproducibility.

use crate::alpha::checks::{
  ALPHA_RELEASE_CHECKS_SCHEMA_VERSION, AlphaReleaseChecksError, AlphaReleaseChecksManifest,
  CheckVerificationStatus, ReleaseCheckCategory, ReleaseCheckDefinition, ReleaseCheckSeverity,
  ReleaseChecksAuditReport, audit_release_checks,
};
use crate::alpha::compatibility::{
  CompatibilityDomain, CompatibilityEvaluationReport, CompatibilityLevel,
  CompatibilityMatrixDefinition, VersionMatrixEntry, evaluate_compatibility_matrix,
};
use crate::alpha::data_dictionary::{
  DataCategory, DataDictionaryAuditReport, DataDictionaryDefinition, DataFieldDefinition,
  DataSensitivityLevel, audit_data_dictionary,
};
use crate::alpha::governance::{
  AlphaGovernanceError, AlphaGovernanceReport, PolicyComplianceArea, PolicyDeclaration,
  PublicAlphaGovernanceManifest, evaluate_alpha_governance,
};
use crate::alpha::guides::{
  ALPHA_GUIDES_SCHEMA_VERSION, AlphaGuidesError, AlphaGuidesManifest, GuideAudience,
  GuideDocumentDefinition, GuideSection, GuideSectionKind, GuidesAuditReport,
  audit_guide_manifests,
};
use crate::alpha::limitations::{
  AlphaLimitationsDeclaration, AlphaLimitationsError, CitationGuidance, ClaimClassification,
  EvidenceTier, LimitationCategory, LimitationsAuditReport, ResearchClaim,
  audit_limitations_and_boundaries,
};
use crate::alpha::reproducibility::{
  ALPHA_REPRODUCIBILITY_SCHEMA_VERSION, AlphaReproducibilityError, ReproducibilityAuditReport,
  ReproducibilityBundleManifest, ReproducibilityPackageDefinition, ReproducibilityStatus,
  SampleArtifactKind, audit_reproducibility_bundle,
};

/// Canonical schema version for the M12 Alpha scenario catalog.
pub const ALPHA_CATALOG_SCHEMA_VERSION: &str = "m12-alpha-catalog-v1";

/// Classification of alpha scenario benchmarks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlphaScenarioKind {
  GovernanceEvaluation,
  CompatibilityEvaluation,
  DataDictionaryAudit,
  LimitationsAudit,
  GuidesAudit,
  ReproducibilityAudit,
  ReleaseChecksAudit,
}

impl AlphaScenarioKind {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::GovernanceEvaluation => "governance-evaluation",
      Self::CompatibilityEvaluation => "compatibility-evaluation",
      Self::DataDictionaryAudit => "data-dictionary-audit",
      Self::LimitationsAudit => "limitations-audit",
      Self::GuidesAudit => "guides-audit",
      Self::ReproducibilityAudit => "reproducibility-audit",
      Self::ReleaseChecksAudit => "release-checks-audit",
    }
  }
}

/// A registered benchmark scenario definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub kind: AlphaScenarioKind,
  pub description: &'static str,
  pub expected_eligible: bool,
}

/// Catalog of canonical Public Alpha governance, compatibility, limitations, guides, and reproducibility scenarios.
pub struct AlphaScenarioCatalog;

impl AlphaScenarioCatalog {
  pub const SCENARIO_GOVERNANCE_COMPLIANT: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-governance-compliant-v1",
    title: "Full Compliant Alpha Governance Manifest",
    kind: AlphaScenarioKind::GovernanceEvaluation,
    description: "Evaluates a complete 6-area governance manifest with 100% verified compliance, permissive MIT licensing, and verified fallback universe.",
    expected_eligible: true,
  };

  pub const SCENARIO_GOVERNANCE_FALLBACK: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-governance-fallback-triggered-v1",
    title: "Original Fallback Required Governance Manifest",
    kind: AlphaScenarioKind::GovernanceEvaluation,
    description: "Evaluates a governance manifest where unofficial disclaimer triggers original fallback universe isolation, verifying distributable posture.",
    expected_eligible: true,
  };

  pub const SCENARIO_COMPATIBILITY_MATRIX: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-compatibility-matrix-v1",
    title: "Multi-Domain Version Compatibility Matrix",
    kind: AlphaScenarioKind::CompatibilityEvaluation,
    description: "Evaluates ruleset, scenario, protocol DTO, and GUI presentation version compatibility entries with explicit migration contracts.",
    expected_eligible: true,
  };

  pub const SCENARIO_DATA_DICTIONARY: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-data-dictionary-complete-v1",
    title: "Canonical 12-Field Data Dictionary Audit",
    kind: AlphaScenarioKind::DataDictionaryAudit,
    description: "Audits a comprehensive data dictionary across all 8 categories and 4 sensitivity tiers with verified fog-of-war redactions.",
    expected_eligible: true,
  };

  pub const SCENARIO_LIMITATIONS_COMPLIANT: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-limitations-compliant-v1",
    title: "Compliant Research Claims and Limitations Declaration",
    kind: AlphaScenarioKind::LimitationsAudit,
    description: "Audits bounded research claims across simulation fidelity, accessibility, and agent generalization with explicit limitation disclaimers and valid BibTeX citation.",
    expected_eligible: true,
  };

  pub const SCENARIO_LIMITATIONS_OVERCLAIM_REJECTED: AlphaScenarioDefinition =
    AlphaScenarioDefinition {
      scenario_id: "scenario-alpha-limitations-overclaim-rejected-v1",
      title: "Impermissible Overclaim Rejection",
      kind: AlphaScenarioKind::LimitationsAudit,
      description: "Verifies fail-closed rejection when a manifest asserts unverified commercial parity or human realism claims without supporting evidence.",
      expected_eligible: false,
    };

  pub const SCENARIO_LIMITATIONS_MISSING_DISCLAIMER: AlphaScenarioDefinition =
    AlphaScenarioDefinition {
      scenario_id: "scenario-alpha-limitations-missing-disclaimer-v1",
      title: "Missing Required Disclaimer Rejection",
      kind: AlphaScenarioKind::LimitationsAudit,
      description: "Verifies fail-closed rejection when a conditional research claim omits required limitation category disclosures.",
      expected_eligible: false,
    };

  pub const SCENARIO_GUIDES_COMPLETE: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-guides-complete-v1",
    title: "Complete 6-Guide Public Alpha Documentation Suite",
    kind: AlphaScenarioKind::GuidesAudit,
    description: "Audits a complete 6-guide documentation manifest spanning Player, Contributor, MCP-Agent, Experimenter, Replay-Analyst, and Data-Scientist guides with verified DAG dependencies.",
    expected_eligible: true,
  };

  pub const SCENARIO_GUIDES_CYCLIC_REJECTED: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-guides-cyclic-prereq-rejected-v1",
    title: "Cyclic Prerequisite Guides Rejection",
    kind: AlphaScenarioKind::GuidesAudit,
    description: "Verifies fail-closed rejection when guide prerequisite dependencies contain an invalid circular cycle.",
    expected_eligible: false,
  };

  pub const SCENARIO_REPRODUCIBILITY_BUNDLE: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-reproducibility-bundle-v1",
    title: "Canonical Public Alpha Reproducibility Bundle",
    kind: AlphaScenarioKind::ReproducibilityAudit,
    description: "Audits a comprehensive sample artifact bundle covering scenarios, replays, experiment batches, calibration studies, and telemetries with verified 16-hex FNV-1a checksums.",
    expected_eligible: true,
  };

  pub const SCENARIO_REPRODUCIBILITY_CORRUPT_REJECTED: AlphaScenarioDefinition =
    AlphaScenarioDefinition {
      scenario_id: "scenario-alpha-reproducibility-corrupt-hash-rejected-v1",
      title: "Corrupt or Invalid Content Hash Rejection",
      kind: AlphaScenarioKind::ReproducibilityAudit,
      description: "Verifies fail-closed rejection when a packaged reproducibility sample provides a corrupted or invalid checksum.",
      expected_eligible: false,
    };

  pub const SCENARIO_RELEASE_CHECKS_COMPLIANT: AlphaScenarioDefinition = AlphaScenarioDefinition {
    scenario_id: "scenario-alpha-release-checks-compliant-v1",
    title: "Canonical Public Alpha Release Checks Suite",
    kind: AlphaScenarioKind::ReleaseChecksAudit,
    description: "Audits a complete 6-category release verification suite across clean-install, reproducibility, security, license, compatibility, and data redaction with 100% pass.",
    expected_eligible: true,
  };

  pub const SCENARIO_RELEASE_CHECKS_BLOCKER_REJECTED: AlphaScenarioDefinition =
    AlphaScenarioDefinition {
      scenario_id: "scenario-alpha-release-checks-blocker-rejected-v1",
      title: "Critical Blocker Release Check Rejection",
      kind: AlphaScenarioKind::ReleaseChecksAudit,
      description: "Verifies fail-closed rejection when a release check detects a critical security blocker or latent state disclosure.",
      expected_eligible: false,
    };

  pub const SCENARIO_RELEASE_CHECKS_MISSING_CATEGORY_REJECTED: AlphaScenarioDefinition =
    AlphaScenarioDefinition {
      scenario_id: "scenario-alpha-release-checks-missing-category-rejected-v1",
      title: "Missing Mandatory Check Category Rejection",
      kind: AlphaScenarioKind::ReleaseChecksAudit,
      description: "Verifies fail-closed rejection when a release manifest omits one of the 6 required verification categories.",
      expected_eligible: false,
    };

  pub const ALL: [AlphaScenarioDefinition; 14] = [
    Self::SCENARIO_GOVERNANCE_COMPLIANT,
    Self::SCENARIO_GOVERNANCE_FALLBACK,
    Self::SCENARIO_COMPATIBILITY_MATRIX,
    Self::SCENARIO_DATA_DICTIONARY,
    Self::SCENARIO_LIMITATIONS_COMPLIANT,
    Self::SCENARIO_LIMITATIONS_OVERCLAIM_REJECTED,
    Self::SCENARIO_LIMITATIONS_MISSING_DISCLAIMER,
    Self::SCENARIO_GUIDES_COMPLETE,
    Self::SCENARIO_GUIDES_CYCLIC_REJECTED,
    Self::SCENARIO_REPRODUCIBILITY_BUNDLE,
    Self::SCENARIO_REPRODUCIBILITY_CORRUPT_REJECTED,
    Self::SCENARIO_RELEASE_CHECKS_COMPLIANT,
    Self::SCENARIO_RELEASE_CHECKS_BLOCKER_REJECTED,
    Self::SCENARIO_RELEASE_CHECKS_MISSING_CATEGORY_REJECTED,
  ];

  pub fn lookup(scenario_id: &str) -> Option<&'static AlphaScenarioDefinition> {
    Self::ALL.iter().find(|s| s.scenario_id == scenario_id)
  }

  /// Constructs the canonical compliant governance manifest.
  pub fn build_compliant_manifest() -> PublicAlphaGovernanceManifest {
    PublicAlphaGovernanceManifest {
      manifest_id: "manifest-alpha-compliant-v1".to_string(),
      version: "0.1.215".to_string(),
      declarations: vec![
        PolicyDeclaration::new(
          PolicyComplianceArea::LicenseNotice,
          "DECL-001",
          "MIT Open Source License",
          "LICENSE",
          true,
          "Verified MIT license text present in repository root and Cargo metadata",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::NonCommercialUse,
          "DECL-002",
          "Non-Commercial Research Scope",
          "NOTICE.md",
          true,
          "Verified strict non-commercial research and educational distribution scope",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::UnofficialDisclaimer,
          "DECL-003",
          "Unofficial Fan Project Notice",
          "NOTICE.md",
          true,
          "Verified explicit non-affiliation disclaimer regarding Riot Games",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::OriginalSettingFallback,
          "DECL-004",
          "Original Setting Fallback Universe",
          "SPEC.md",
          true,
          "Verified complete original lore, terminology, and naming fallback universe",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::AssetProvenanceAudit,
          "DECL-005",
          "Asset Provenance and License Audit",
          "src/gui/asset.rs",
          true,
          "Verified 100% permissive open source assets and procedural fallbacks",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::ContentIsolation,
          "DECL-006",
          "Simulation Mechanics Content Isolation",
          "docs/adr/0001-deterministic-transition-authority.md",
          true,
          "Verified simulation kernel is strictly isolated from third-party IP assets",
        ),
      ],
      fallback_universe_name: "Aetheria-Stratagem".to_string(),
      repository_license: "MIT".to_string(),
    }
  }

  /// Constructs the fallback-triggered governance manifest.
  pub fn build_fallback_manifest() -> PublicAlphaGovernanceManifest {
    PublicAlphaGovernanceManifest {
      manifest_id: "manifest-alpha-fallback-v1".to_string(),
      version: "0.1.215".to_string(),
      declarations: vec![
        PolicyDeclaration::new(
          PolicyComplianceArea::LicenseNotice,
          "DECL-001",
          "MIT Open Source License",
          "LICENSE",
          true,
          "Verified MIT license text present in repository root and Cargo metadata",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::NonCommercialUse,
          "DECL-002",
          "Non-Commercial Scope",
          "NOTICE.md",
          false,
          "Commercial clearance pending; fallback activation mandated",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::UnofficialDisclaimer,
          "DECL-003",
          "Unofficial Disclaimer",
          "NOTICE.md",
          false,
          "Disclaimer requires fallback universe activation",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::OriginalSettingFallback,
          "DECL-004",
          "Original Setting Fallback",
          "SPEC.md",
          true,
          "Original fallback universe fully active and configured",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::AssetProvenanceAudit,
          "DECL-005",
          "Asset Provenance Audit",
          "src/gui/asset.rs",
          true,
          "All assets confirmed permissive open source",
        ),
        PolicyDeclaration::new(
          PolicyComplianceArea::ContentIsolation,
          "DECL-006",
          "Content Isolation",
          "docs/adr/0001-deterministic-transition-authority.md",
          true,
          "Pure simulation core isolated from IP",
        ),
      ],
      fallback_universe_name: "Aetheria-Stratagem".to_string(),
      repository_license: "MIT".to_string(),
    }
  }

  /// Constructs the canonical compatibility matrix.
  pub fn build_canonical_compatibility_matrix() -> CompatibilityMatrixDefinition {
    CompatibilityMatrixDefinition {
      matrix_id: "matrix-alpha-canonical-v1".to_string(),
      matrix_version: "1.0.0".to_string(),
      entries: vec![
        VersionMatrixEntry::new(
          CompatibilityDomain::Ruleset,
          "1",
          "4",
          CompatibilityLevel::BreakingChangeMigrationRequired,
          Some("MIGRATION-RULESET-V1-TO-V4".to_string()),
          "Ruleset 1 deterministic kernel migrated to v4 multi-window delayed origin contract",
        ),
        VersionMatrixEntry::new(
          CompatibilityDomain::ProtocolDto,
          "m5-protocol-v1",
          "m5-protocol-v2",
          CompatibilityLevel::FullyCompatible,
          None,
          "External actor DTO envelope backward-compatible extension",
        ),
        VersionMatrixEntry::new(
          CompatibilityDomain::GuiPresentation,
          "m11-gui-dto-v1",
          "m11-gui-dto-v1",
          CompatibilityLevel::FullyCompatible,
          None,
          "Shared-boundary GUI DTO baseline model",
        ),
        VersionMatrixEntry::new(
          CompatibilityDomain::ReplayArtifact,
          "m1-replay-v1",
          "m9-replay-v1",
          CompatibilityLevel::BackwardCompatibleOnly,
          None,
          "M1 append-only replay verifiable in legacy engine path",
        ),
      ],
    }
  }

  /// Constructs the canonical 12-field data dictionary.
  pub fn build_canonical_data_dictionary() -> DataDictionaryDefinition {
    DataDictionaryDefinition {
      dictionary_id: "dict-alpha-canonical-v1".to_string(),
      version: "1.0.0".to_string(),
      fields: vec![
        DataFieldDefinition::new(
          "world_state.turn",
          DataCategory::AuthoritativeState,
          DataSensitivityLevel::PublicActorVisible,
          "u32",
          "[0..=10,000]",
          "Current simulation discrete turn index",
          "public-turn-progression",
        ),
        DataFieldDefinition::new(
          "laner_state.health",
          DataCategory::AuthoritativeState,
          DataSensitivityLevel::PublicActorVisible,
          "u16",
          "[0..=100]",
          "Player laner current health points",
          "direct-actor-vital",
        ),
        DataFieldDefinition::new(
          "laner_state.mana",
          DataCategory::AuthoritativeState,
          DataSensitivityLevel::TeamVisibleShared,
          "u16",
          "[0..=100]",
          "Player laner current mana resource",
          "team-shared-vital",
        ),
        DataFieldDefinition::new(
          "opponent_state.position",
          DataCategory::AuthoritativeState,
          DataSensitivityLevel::LatentHostAuthoritative,
          "MapLocationId",
          "15 canonical map nodes",
          "True hidden coordinates of opposing laner in fog of war",
          "redacted-unless-vision-coverage",
        ),
        DataFieldDefinition::new(
          "jungle_threat.region",
          DataCategory::AuthoritativeState,
          DataSensitivityLevel::LatentHostAuthoritative,
          "ThreatRegion",
          "RiverSide | InLane | Absent",
          "True hidden jungle threat location",
          "redacted-to-last-known-or-unknown",
        ),
        DataFieldDefinition::new(
          "observation.threat_report",
          DataCategory::ObservationProjection,
          DataSensitivityLevel::PublicActorVisible,
          "LaneBelief<ThreatRegion>",
          "Unknown | Observed | LastKnown",
          "Actor-visible filtered threat belief without latent truth",
          "bounded-belief-projection",
        ),
        DataFieldDefinition::new(
          "intent.staged_plan",
          DataCategory::IntentCommand,
          DataSensitivityLevel::PublicActorVisible,
          "LaneIntent",
          "Stabilize | Contest | Yield | Recall",
          "Tactical intent submitted by actor",
          "actor-owned-intent",
        ),
        DataFieldDefinition::new(
          "event.damage_dealt",
          DataCategory::EventLog,
          DataSensitivityLevel::PublicActorVisible,
          "u16",
          "[0..=1,000]",
          "Resolved damage dealt during transition",
          "direct-immediate-event",
        ),
        DataFieldDefinition::new(
          "debrief.attribution_quadrant",
          DataCategory::CausalDebrief,
          DataSensitivityLevel::PublicActorVisible,
          "AttributionQuadrant",
          "CoordinatedTriumph | CoordinatedFailure | UncoordinatedBailout | CompoundedFailure",
          "2D orthogonal post-encounter causal classification",
          "zero-private-cot-attribution",
        ),
        DataFieldDefinition::new(
          "debrief.private_cot",
          DataCategory::CausalDebrief,
          DataSensitivityLevel::ResearchInspectionOnly,
          "String",
          "Bounded diagnostic trace",
          "Actor private chain-of-thought reasoning",
          "redacted-from-actor-visible-reports",
        ),
        DataFieldDefinition::new(
          "replay.state_hash",
          DataCategory::ReplayRecord,
          DataSensitivityLevel::PublicActorVisible,
          "u64",
          "FNV-1a 64-bit hash",
          "Authoritative state hash verifying deterministic transition",
          "deterministic-state-hash",
        ),
        DataFieldDefinition::new(
          "gui.presentation_bundle",
          DataCategory::GuiPresentationBundle,
          DataSensitivityLevel::PublicActorVisible,
          "GuiPresentationBundle",
          "Versioned DTO container",
          "Actor-safe aggregated presentation bundle for GUI",
          "complete-actor-projection-parity",
        ),
      ],
    }
  }

  /// Runs the compliant governance benchmark.
  pub fn execute_governance_compliant() -> Result<AlphaGovernanceReport, AlphaGovernanceError> {
    let manifest = Self::build_compliant_manifest();
    evaluate_alpha_governance(&manifest)
  }

  /// Runs the fallback governance benchmark.
  pub fn execute_governance_fallback() -> Result<AlphaGovernanceReport, AlphaGovernanceError> {
    let manifest = Self::build_fallback_manifest();
    evaluate_alpha_governance(&manifest)
  }

  /// Runs the compatibility matrix benchmark.
  pub fn execute_compatibility()
  -> Result<CompatibilityEvaluationReport, crate::alpha::compatibility::CompatibilityError> {
    let matrix = Self::build_canonical_compatibility_matrix();
    evaluate_compatibility_matrix(&matrix)
  }

  /// Runs the data dictionary benchmark.
  pub fn execute_data_dictionary()
  -> Result<DataDictionaryAuditReport, crate::alpha::data_dictionary::DataDictionaryError> {
    let dict = Self::build_canonical_data_dictionary();
    audit_data_dictionary(&dict)
  }

  /// Constructs the canonical compliant limitations declaration.
  pub fn build_compliant_limitations_declaration() -> AlphaLimitationsDeclaration {
    AlphaLimitationsDeclaration {
      manifest_id: "manifest-limitations-compliant-v1".to_string(),
      version: "0.1.215".to_string(),
      claims: vec![
        ResearchClaim::new(
          "CLAIM-001",
          LimitationCategory::SimulationFidelity,
          "Fog of Intent simulation transitions are fully deterministic and replay-verified across all core subsystems",
          EvidenceTier::SoftwareInvariants,
          vec![],
          ClaimClassification::PermissibleBoundedClaim,
          "Verified by deterministic state hashing, append-only history replay, and zero async/RNG primitives in core",
        ),
        ResearchClaim::new(
          "CLAIM-002",
          LimitationCategory::AccessibilityCoverage,
          "The reference CLI and HTML presentation layers support non-color semantics, high-contrast tokens, and keyboard-driven interaction",
          EvidenceTier::LimitedHumanStudy,
          vec![LimitationCategory::AccessibilityCoverage],
          ClaimClassification::ConditionalWithDisclaimer,
          "Audited across M10 interaction modes and M11 WCAG 2.1 AA tokens, with documented untested assistive hardware limits",
        ),
        ResearchClaim::new(
          "CLAIM-003",
          LimitationCategory::AgentGeneralization,
          "Parametric and heuristic agents exhibit distinct observable strategic trade-offs on diagnostic choice catalogs",
          EvidenceTier::EmpiricalCalibration,
          vec![
            LimitationCategory::AgentGeneralization,
            LimitationCategory::HumanRealism,
          ],
          ClaimClassification::ConditionalWithDisclaimer,
          "Fitted and validated across M7 diagnostic dilemmas with Total Variation Distance metrics, not generalizing to unconstrained human game dynamics",
        ),
      ],
      citation: CitationGuidance::new(
        r#"@software{fog_of_intent_2026,
  author = {Saehwan Park},
  title = {Fog of Intent: Deterministic Turn-Based Strategy Simulation for AI-Native Coordination},
  year = {2026},
  version = {0.1.215},
  url = {https://github.com/SaehwanPark/fog-of-intent}
}"#,
        "10.5281/zenodo.fogofintent.v0.1.215",
        "Fog of Intent: Public Research-Capable Alpha",
        "0.1.215",
        "https://github.com/SaehwanPark/fog-of-intent",
        "Explicit seed bundle injection with zero internal RNG; identical seeds produce identical trajectories",
      ),

      disclosed_limitations: vec![
        LimitationCategory::SimulationFidelity,
        LimitationCategory::AccessibilityCoverage,
        LimitationCategory::AgentGeneralization,
        LimitationCategory::HumanRealism,
        LimitationCategory::NetworkMultiplayer,
        LimitationCategory::HardwareRequirements,
      ],
    }
  }

  /// Constructs an overclaim limitations declaration.
  pub fn build_overclaim_limitations_declaration() -> AlphaLimitationsDeclaration {
    let mut decl = Self::build_compliant_limitations_declaration();
    decl.manifest_id = "manifest-limitations-overclaim-v1".to_string();
    decl.claims.push(ResearchClaim::new(
      "CLAIM-OVERCLAIM",
      LimitationCategory::HumanRealism,
      "Simulated AI agent behaviors faithfully reproduce human psychological decision-making in competitive play",
      EvidenceTier::UnverifiedHypothesis,
      vec![],
      ClaimClassification::ImpermissibleOverclaim,
      "Unsubstantiated claim equating synthetic heuristics to human cognitive ground truth",
    ));
    decl
  }

  /// Constructs a limitations declaration missing required category disclaimers.
  pub fn build_missing_disclaimer_limitations_declaration() -> AlphaLimitationsDeclaration {
    let mut decl = Self::build_compliant_limitations_declaration();
    decl.manifest_id = "manifest-limitations-missing-disclaimer-v1".to_string();
    decl.claims.push(ResearchClaim::new(
      "CLAIM-CONDITIONAL-UNDISCLOSED",
      LimitationCategory::NetworkMultiplayer,
      "Local presentation simulation operates without networked multiplayer synchronizer",
      EvidenceTier::SoftwareInvariants,
      vec![LimitationCategory::NetworkMultiplayer],
      ClaimClassification::ConditionalWithDisclaimer,
      "Requires explicit disclosure in manifest limitations list",
    ));
    decl
      .disclosed_limitations
      .retain(|&cat| cat != LimitationCategory::NetworkMultiplayer);
    decl
  }

  /// Runs the compliant limitations benchmark.
  pub fn execute_limitations_compliant() -> Result<LimitationsAuditReport, AlphaLimitationsError> {
    let decl = Self::build_compliant_limitations_declaration();
    audit_limitations_and_boundaries(&decl)
  }

  /// Runs the overclaim limitations benchmark.
  pub fn execute_limitations_overclaim() -> Result<LimitationsAuditReport, AlphaLimitationsError> {
    let decl = Self::build_overclaim_limitations_declaration();
    audit_limitations_and_boundaries(&decl)
  }

  /// Runs the missing disclaimer limitations benchmark.
  pub fn execute_limitations_missing_disclaimer()
  -> Result<LimitationsAuditReport, AlphaLimitationsError> {
    let decl = Self::build_missing_disclaimer_limitations_declaration();
    audit_limitations_and_boundaries(&decl)
  }

  /// Constructs the canonical compliant documentation guides manifest.
  pub fn build_compliant_guides_manifest() -> AlphaGuidesManifest {
    static SECTIONS_PLAYER: [GuideSection; 4] = [
      GuideSection {
        heading: "Quickstart: First Match",
        kind: GuideSectionKind::Quickstart,
        content_summary: "Launching the reference CLI runner and selecting one-lane scenarios.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Intent Selection Mechanics",
        kind: GuideSectionKind::CoreConcepts,
        content_summary: "Explaining the discrete intent palette and tactical trade-offs.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Turn-by-Turn Walkthrough",
        kind: GuideSectionKind::InteractiveWalkthrough,
        content_summary: "Walking through fog-of-war observations and execution resolution.",
        has_code_example: false,
      },
      GuideSection {
        heading: "Causal Debrief Analysis",
        kind: GuideSectionKind::EvidenceAndLimitations,
        content_summary: "Interpreting 2D orthogonal attribution quadrants and performance factors.",
        has_code_example: true,
      },
    ];

    static SECTIONS_CONTRIB: [GuideSection; 4] = [
      GuideSection {
        heading: "Repository Prerequisites",
        kind: GuideSectionKind::Prerequisites,
        content_summary: "Pinned Rust toolchain 1.96.0, formatting, and linting checks.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Deterministic Kernel Architecture",
        kind: GuideSectionKind::CoreConcepts,
        content_summary: "Authoritative simulation transitions, state hashes, and replay verification.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Protocol Contracts and Codecs",
        kind: GuideSectionKind::ProtocolContracts,
        content_summary: "Bounded serialization contracts and fail-closed validation rules.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Common Pitfalls and Troubleshooting",
        kind: GuideSectionKind::Troubleshooting,
        content_summary: "Diagnosing hash mismatches and async leakage in core modules.",
        has_code_example: false,
      },
    ];

    static SECTIONS_MCP: [GuideSection; 3] = [
      GuideSection {
        heading: "MCP Server Quickstart",
        kind: GuideSectionKind::Quickstart,
        content_summary: "Configuring the standard I/O JSON-RPC loopback host adapter.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Observation and Action Payloads",
        kind: GuideSectionKind::ProtocolContracts,
        content_summary: "Consuming actor-visible DTOs and submitting validated intents.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Rate Limits and Connection Recovery",
        kind: GuideSectionKind::Troubleshooting,
        content_summary: "Handling session disconnection, reconnection, and idempotent receipts.",
        has_code_example: false,
      },
    ];

    static SECTIONS_EXP: [GuideSection; 3] = [
      GuideSection {
        heading: "Batch Runner Configuration",
        kind: GuideSectionKind::Quickstart,
        content_summary: "Defining scripted experiment manifests and seed arrays.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Population Sampling Invariants",
        kind: GuideSectionKind::CoreConcepts,
        content_summary: "Deterministic agent selection, profile grids, and TVD metrics.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Reproducibility Safeguards",
        kind: GuideSectionKind::EvidenceAndLimitations,
        content_summary: "Auditing empirical distributions against synthetic ground truth.",
        has_code_example: false,
      },
    ];

    static SECTIONS_REPLAY: [GuideSection; 3] = [
      GuideSection {
        heading: "Replay Transcript Format",
        kind: GuideSectionKind::CoreConcepts,
        content_summary: "Decoding append-only action logs and transition event summaries.",
        has_code_example: true,
      },
      GuideSection {
        heading: "State Hash Verification",
        kind: GuideSectionKind::ProtocolContracts,
        content_summary: "Verifying 64-bit FNV-1a state hashes across match turns.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Attribution Quadrant Inspection",
        kind: GuideSectionKind::EvidenceAndLimitations,
        content_summary: "Extracting decision vs execution causal attribution factors.",
        has_code_example: false,
      },
    ];

    static SECTIONS_DATA: [GuideSection; 3] = [
      GuideSection {
        heading: "Data Sensitivity Tiers",
        kind: GuideSectionKind::CoreConcepts,
        content_summary: "Understanding Public, TeamVisible, LatentHost, and Research tiers.",
        has_code_example: false,
      },
      GuideSection {
        heading: "Redaction Invariants",
        kind: GuideSectionKind::ProtocolContracts,
        content_summary: "Auditing fog-of-war redactions and chain-of-thought privacy.",
        has_code_example: true,
      },
      GuideSection {
        heading: "Telemetry Export Formats",
        kind: GuideSectionKind::Quickstart,
        content_summary: "Exporting structured CSV and JSONL records for empirical research.",
        has_code_example: true,
      },
    ];

    static GUIDES: [GuideDocumentDefinition; 6] = [
      GuideDocumentDefinition {
        guide_id: "GUIDE-PLAYER-01",
        title: "Player Strategy and Intent Guide",
        audience: GuideAudience::Player,
        summary: "Comprehensive guide to expressing intent, observing bounded game state, and analyzing match debriefs.",
        prerequisite_guide_ids: &[],
        sections: &SECTIONS_PLAYER,
      },
      GuideDocumentDefinition {
        guide_id: "GUIDE-CONTRIB-01",
        title: "Contributor and Simulation Architecture Guide",
        audience: GuideAudience::Contributor,
        summary: "Guide for contributing deterministic simulation mechanics, transitions, and state hashes.",
        prerequisite_guide_ids: &["GUIDE-PLAYER-01"],
        sections: &SECTIONS_CONTRIB,
      },
      GuideDocumentDefinition {
        guide_id: "GUIDE-MCP-01",
        title: "MCP Agent Adapter and Protocol Integration Guide",
        audience: GuideAudience::McpAgent,
        summary: "Guide for integrating external language model agents via thin MCP JSON-RPC protocol.",
        prerequisite_guide_ids: &["GUIDE-PLAYER-01"],
        sections: &SECTIONS_MCP,
      },
      GuideDocumentDefinition {
        guide_id: "GUIDE-EXP-01",
        title: "Agent Ecology and Behavioral Experimentation Guide",
        audience: GuideAudience::Experimenter,
        summary: "Guide for configuring batch experiments, population sampling, and calibration benchmarks.",
        prerequisite_guide_ids: &["GUIDE-CONTRIB-01", "GUIDE-MCP-01"],
        sections: &SECTIONS_EXP,
      },
      GuideDocumentDefinition {
        guide_id: "GUIDE-REPLAY-01",
        title: "Deterministic Replay and Causal Debrief Analysis Guide",
        audience: GuideAudience::ReplayAnalyst,
        summary: "Guide for verifying append-only history logs and 2D orthogonal attribution quadrants.",
        prerequisite_guide_ids: &["GUIDE-PLAYER-01"],
        sections: &SECTIONS_REPLAY,
      },
      GuideDocumentDefinition {
        guide_id: "GUIDE-DATA-01",
        title: "Data Dictionary and Telemetry Extraction Guide",
        audience: GuideAudience::DataScientist,
        summary: "Guide for analyzing simulation variables, sensitivity tiers, and redacted telemetry streams.",
        prerequisite_guide_ids: &["GUIDE-REPLAY-01"],
        sections: &SECTIONS_DATA,
      },
    ];

    AlphaGuidesManifest {
      schema_version: ALPHA_GUIDES_SCHEMA_VERSION,
      guides: &GUIDES,
    }
  }

  /// Constructs a guides manifest with cyclic prerequisite dependencies.
  pub fn build_cyclic_guides_manifest() -> AlphaGuidesManifest {
    static SECTIONS_A: [GuideSection; 1] = [GuideSection {
      heading: "Section A",
      kind: GuideSectionKind::CoreConcepts,
      content_summary: "Summary A",
      has_code_example: false,
    }];
    static SECTIONS_B: [GuideSection; 1] = [GuideSection {
      heading: "Section B",
      kind: GuideSectionKind::CoreConcepts,
      content_summary: "Summary B",
      has_code_example: false,
    }];
    static GUIDES_CYCLIC: [GuideDocumentDefinition; 2] = [
      GuideDocumentDefinition {
        guide_id: "GUIDE-CYCLIC-A",
        title: "Cyclic Guide A",
        audience: GuideAudience::Player,
        summary: "Depends on B",
        prerequisite_guide_ids: &["GUIDE-CYCLIC-B"],
        sections: &SECTIONS_A,
      },
      GuideDocumentDefinition {
        guide_id: "GUIDE-CYCLIC-B",
        title: "Cyclic Guide B",
        audience: GuideAudience::Contributor,
        summary: "Depends on A",
        prerequisite_guide_ids: &["GUIDE-CYCLIC-A"],
        sections: &SECTIONS_B,
      },
    ];

    AlphaGuidesManifest {
      schema_version: ALPHA_GUIDES_SCHEMA_VERSION,
      guides: &GUIDES_CYCLIC,
    }
  }

  /// Runs the compliant guides benchmark.
  pub fn execute_guides_compliant() -> Result<GuidesAuditReport, AlphaGuidesError> {
    let manifest = Self::build_compliant_guides_manifest();
    audit_guide_manifests(&manifest)
  }

  /// Runs the cyclic guides benchmark.
  pub fn execute_guides_cyclic() -> Result<GuidesAuditReport, AlphaGuidesError> {
    let manifest = Self::build_cyclic_guides_manifest();
    audit_guide_manifests(&manifest)
  }

  /// Constructs the canonical compliant reproducibility bundle manifest.
  pub fn build_canonical_reproducibility_bundle() -> ReproducibilityBundleManifest {
    static PACKAGES: [ReproducibilityPackageDefinition; 5] = [
      ReproducibilityPackageDefinition {
        package_id: "PKG-BENCHMARK-01",
        title: "Reference Two-Window and Complete-Match Benchmarks",
        kind: SampleArtifactKind::ScenarioBenchmark,
        artifact_count: 7,
        content_hash_fnv1a: "811c9dc500000001",
        verification_command: "cargo test --locked alpha::",
        seed_policy: "explicit-seed-bundle",
        dependencies: &[],
        declared_status: ReproducibilityStatus::FullyReproducible,
      },
      ReproducibilityPackageDefinition {
        package_id: "PKG-REPLAY-01",
        title: "Append-Only Replay Verification Transcripts",
        kind: SampleArtifactKind::ReplayTranscript,
        artifact_count: 12,
        content_hash_fnv1a: "811c9dc500000002",
        verification_command: "fog-of-intent --scenario m9-complete-match-replay-v1",
        seed_policy: "fixed-prng-seed",
        dependencies: &["PKG-BENCHMARK-01"],
        declared_status: ReproducibilityStatus::FullyReproducible,
      },
      ReproducibilityPackageDefinition {
        package_id: "PKG-EXPERIMENT-01",
        title: "Multi-Agent Decision Density Batch Experiments",
        kind: SampleArtifactKind::ExperimentRun,
        artifact_count: 16,
        content_hash_fnv1a: "811c9dc500000003",
        verification_command: "cargo test --locked agent::experiment::",
        seed_policy: "batch-seed-array",
        dependencies: &["PKG-BENCHMARK-01"],
        declared_status: ReproducibilityStatus::FullyReproducible,
      },
      ReproducibilityPackageDefinition {
        package_id: "PKG-CALIBRATION-01",
        title: "Semantic-to-Parametric Policy Calibration Study",
        kind: SampleArtifactKind::ModelCalibrationStudy,
        artifact_count: 8,
        content_hash_fnv1a: "811c9dc500000004",
        verification_command: "cargo test --locked agent::parametric::",
        seed_policy: "deterministic-grid",
        dependencies: &["PKG-EXPERIMENT-01"],
        declared_status: ReproducibilityStatus::SyntheticBaselineOnly,
      },
      ReproducibilityPackageDefinition {
        package_id: "PKG-TELEMETRY-01",
        title: "Role-Specific Causal Attribution Telemetries",
        kind: SampleArtifactKind::BehavioralTelemetry,
        artifact_count: 10,
        content_hash_fnv1a: "811c9dc500000005",
        verification_command: "cargo test --locked map::role_debrief::",
        seed_policy: "telemetry-fixture",
        dependencies: &["PKG-REPLAY-01"],
        declared_status: ReproducibilityStatus::FullyReproducible,
      },
    ];

    ReproducibilityBundleManifest {
      schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
      packages: &PACKAGES,
    }
  }

  /// Constructs a reproducibility bundle with an invalid content checksum.
  pub fn build_corrupt_reproducibility_bundle() -> ReproducibilityBundleManifest {
    static CORRUPT_PACKAGES: [ReproducibilityPackageDefinition; 1] =
      [ReproducibilityPackageDefinition {
        package_id: "PKG-CORRUPT-01",
        title: "Corrupted Checksum Package",
        kind: SampleArtifactKind::ScenarioBenchmark,
        artifact_count: 1,
        content_hash_fnv1a: "invalid-hash-too-short",
        verification_command: "cargo test",
        seed_policy: "none",
        dependencies: &[],
        declared_status: ReproducibilityStatus::FullyReproducible,
      }];

    ReproducibilityBundleManifest {
      schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
      packages: &CORRUPT_PACKAGES,
    }
  }

  /// Runs the compliant reproducibility benchmark.
  pub fn execute_reproducibility_compliant()
  -> Result<ReproducibilityAuditReport, AlphaReproducibilityError> {
    let bundle = Self::build_canonical_reproducibility_bundle();
    audit_reproducibility_bundle(&bundle)
  }

  /// Runs the corrupt reproducibility benchmark.
  pub fn execute_reproducibility_corrupt()
  -> Result<ReproducibilityAuditReport, AlphaReproducibilityError> {
    let bundle = Self::build_corrupt_reproducibility_bundle();
    audit_reproducibility_bundle(&bundle)
  }

  /// Constructs the canonical compliant release checks manifest.
  pub fn build_canonical_release_checks_manifest() -> AlphaReleaseChecksManifest {
    static CHECKS: [ReleaseCheckDefinition; 6] = [
      ReleaseCheckDefinition {
        check_id: "CHK-CLEAN-INSTALL-01",
        category: ReleaseCheckCategory::CleanInstall,
        title: "Clean Environment Build and Test",
        description: "Fresh checkout builds cleanly with locked toolchain and passes all unit and binary integration tests without dirty files",
        severity: ReleaseCheckSeverity::VerifiedPass,
        status: CheckVerificationStatus::Passed,
        evidence_command: "cargo +1.96.0 test --locked",
        evidence_hash: "811c9dc500000011",
        mitigation_notes: None,
      },
      ReleaseCheckDefinition {
        check_id: "CHK-REPRODUCIBILITY-01",
        category: ReleaseCheckCategory::Reproducibility,
        title: "Deterministic Replay and State Hash Verification",
        description: "Composed complete matches and sample artifacts replay to identical FNV-1a state hashes across independent executions",
        severity: ReleaseCheckSeverity::VerifiedPass,
        status: CheckVerificationStatus::Passed,
        evidence_command: "fog-of-intent --scenario m9-complete-match-replay-v1",
        evidence_hash: "811c9dc500000012",
        mitigation_notes: None,
      },
      ReleaseCheckDefinition {
        check_id: "CHK-SECURITY-01",
        category: ReleaseCheckCategory::SecurityAdvisory,
        title: "Repository Checker and Dependency Audit",
        description: "Zero unauthorized external dependencies, zero async/network primitives in core, and strict memory safety invariants verified",
        severity: ReleaseCheckSeverity::VerifiedPass,
        status: CheckVerificationStatus::Passed,
        evidence_command: "python3 scripts/check_repository.py",
        evidence_hash: "811c9dc500000013",
        mitigation_notes: None,
      },
      ReleaseCheckDefinition {
        check_id: "CHK-LICENSE-01",
        category: ReleaseCheckCategory::LicenseCompliance,
        title: "MIT License Notice and Provenance Audit",
        description: "Canonical MIT license header present in repository root, Cargo metadata, and asset provenance registers",
        severity: ReleaseCheckSeverity::VerifiedPass,
        status: CheckVerificationStatus::Passed,
        evidence_command: "cargo metadata --format-version 1",
        evidence_hash: "811c9dc500000014",
        mitigation_notes: None,
      },
      ReleaseCheckDefinition {
        check_id: "CHK-COMPATIBILITY-01",
        category: ReleaseCheckCategory::CompatibilityMatrix,
        title: "Cross-Version Migration and Compatibility Matrix",
        description: "Ruleset, scenario, protocol DTO, and presentation schema compatibility matrices verified with explicit migration contracts",
        severity: ReleaseCheckSeverity::VerifiedPass,
        status: CheckVerificationStatus::Passed,
        evidence_command: "cargo test --locked alpha::compatibility::",
        evidence_hash: "811c9dc500000015",
        mitigation_notes: None,
      },
      ReleaseCheckDefinition {
        check_id: "CHK-REDACTION-01",
        category: ReleaseCheckCategory::DataRedaction,
        title: "Fog-of-War Redaction and Latent State Secrecy",
        description: "Actor-visible observation DTOs, causal debriefs, and GUI presentation bundles maintain zero latent host state leakage",
        severity: ReleaseCheckSeverity::VerifiedPass,
        status: CheckVerificationStatus::Passed,
        evidence_command: "cargo test --locked alpha::data_dictionary::",
        evidence_hash: "811c9dc500000016",
        mitigation_notes: None,
      },
    ];

    AlphaReleaseChecksManifest {
      schema_version: ALPHA_RELEASE_CHECKS_SCHEMA_VERSION,
      manifest_id: "manifest-alpha-release-checks-compliant-v1",
      release_version: "0.1.217",
      target_commit: "ec340c2a8f01b9e5",
      checks: &CHECKS,
    }
  }

  /// Constructs a release checks manifest with a critical blocker.
  pub fn build_blocker_release_checks_manifest() -> AlphaReleaseChecksManifest {
    static BLOCKER_CHECKS: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
      check_id: "CHK-SECURITY-BLOCKER-01",
      category: ReleaseCheckCategory::SecurityAdvisory,
      title: "Latent State Leak Detected",
      description: "Authoritative opponent coordinates exposed in public observation projection DTO",
      severity: ReleaseCheckSeverity::CriticalBlocker,
      status: CheckVerificationStatus::Failed,
      evidence_command: "cargo test --locked protocol::",
      evidence_hash: "811c9dc500000099",
      mitigation_notes: Some("Requires immediate patch in observation projection adapter"),
    }];

    AlphaReleaseChecksManifest {
      schema_version: ALPHA_RELEASE_CHECKS_SCHEMA_VERSION,
      manifest_id: "manifest-alpha-release-checks-blocker-v1",
      release_version: "0.1.217",
      target_commit: "ec340c2a8f01b9e5",
      checks: &BLOCKER_CHECKS,
    }
  }

  /// Constructs a release checks manifest missing a mandatory category.
  pub fn build_missing_category_release_checks_manifest() -> AlphaReleaseChecksManifest {
    static INCOMPLETE_CHECKS: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
      check_id: "CHK-CLEAN-INSTALL-ONLY-01",
      category: ReleaseCheckCategory::CleanInstall,
      title: "Clean Install Only",
      description: "Fresh checkout builds cleanly",
      severity: ReleaseCheckSeverity::VerifiedPass,
      status: CheckVerificationStatus::Passed,
      evidence_command: "cargo test",
      evidence_hash: "811c9dc500000011",
      mitigation_notes: None,
    }];

    AlphaReleaseChecksManifest {
      schema_version: ALPHA_RELEASE_CHECKS_SCHEMA_VERSION,
      manifest_id: "manifest-alpha-release-checks-missing-cat-v1",
      release_version: "0.1.217",
      target_commit: "ec340c2a8f01b9e5",
      checks: &INCOMPLETE_CHECKS,
    }
  }

  /// Runs the compliant release checks benchmark.
  pub fn execute_release_checks_compliant()
  -> Result<ReleaseChecksAuditReport, AlphaReleaseChecksError> {
    let manifest = Self::build_canonical_release_checks_manifest();
    audit_release_checks(&manifest)
  }

  /// Runs the blocker release checks benchmark.
  pub fn execute_release_checks_blocker()
  -> Result<ReleaseChecksAuditReport, AlphaReleaseChecksError> {
    let manifest = Self::build_blocker_release_checks_manifest();
    audit_release_checks(&manifest)
  }

  /// Runs the missing category release checks benchmark.
  pub fn execute_release_checks_missing_category()
  -> Result<ReleaseChecksAuditReport, AlphaReleaseChecksError> {
    let manifest = Self::build_missing_category_release_checks_manifest();
    audit_release_checks(&manifest)
  }
}

/// Renders a scenario catalog summary in Markdown.
pub fn render_alpha_scenario_markdown(scenario: &AlphaScenarioDefinition) -> String {
  let mut md = String::with_capacity(256);
  md.push_str(&format!("### Scenario: `{}`\n\n", scenario.scenario_id));
  md.push_str(&format!("- **Title**: {}\n", scenario.title));
  md.push_str(&format!("- **Kind**: `{}`\n", scenario.kind.as_str()));
  md.push_str(&format!("- **Description**: {}\n", scenario.description));
  md.push_str(&format!(
    "- **Expected Release Eligible**: `{}`\n",
    if scenario.expected_eligible {
      "yes"
    } else {
      "no"
    }
  ));
  md
}
