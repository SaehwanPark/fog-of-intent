//! Benchmark scenarios and canonical test catalogs for Public Alpha governance and compatibility.

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

/// Canonical schema version for the M12 Alpha scenario catalog.
pub const ALPHA_CATALOG_SCHEMA_VERSION: &str = "m12-alpha-catalog-v1";

/// Classification of alpha scenario benchmarks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlphaScenarioKind {
  GovernanceEvaluation,
  CompatibilityEvaluation,
  DataDictionaryAudit,
}

impl AlphaScenarioKind {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::GovernanceEvaluation => "governance-evaluation",
      Self::CompatibilityEvaluation => "compatibility-evaluation",
      Self::DataDictionaryAudit => "data-dictionary-audit",
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

/// Catalog of canonical Public Alpha governance, compatibility, and data dictionary scenarios.
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

  pub const ALL: [AlphaScenarioDefinition; 4] = [
    Self::SCENARIO_GOVERNANCE_COMPLIANT,
    Self::SCENARIO_GOVERNANCE_FALLBACK,
    Self::SCENARIO_COMPATIBILITY_MATRIX,
    Self::SCENARIO_DATA_DICTIONARY,
  ];

  pub fn lookup(scenario_id: &str) -> Option<&'static AlphaScenarioDefinition> {
    Self::ALL.iter().find(|s| s.scenario_id == scenario_id)
  }

  /// Constructs the canonical compliant governance manifest.
  pub fn build_compliant_manifest() -> PublicAlphaGovernanceManifest {
    PublicAlphaGovernanceManifest {
      manifest_id: "manifest-alpha-compliant-v1".to_string(),
      version: "0.1.214".to_string(),
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
      version: "0.1.214".to_string(),
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
