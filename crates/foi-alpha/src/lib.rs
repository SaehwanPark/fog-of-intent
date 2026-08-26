//! Public Research-Capable Alpha release governance, compatibility policies, data dictionary, limitations, guides, and reproducibility contracts for Fog of Intent.

pub mod archive;
pub mod catalog;
pub mod checks;
pub mod compatibility;
pub mod data_dictionary;
pub mod governance;
pub mod guides;
pub mod limitations;
pub mod reproducibility;

pub use crate as alpha;

#[cfg(test)]
pub mod tests;

pub use archive::{
  ALPHA_ARCHIVE_SCHEMA_VERSION, AlphaArchiveError, ArchiveCategoryKind, ArchiveItemRecord,
  CategoryArchiveSummary, ReleaseArchiveAuditReport, ReleaseArchiveManifest,
  audit_release_archive_manifest, canonical_alpha_release_archive_manifest,
  render_release_archive_report_markdown,
};

pub use catalog::{
  ALPHA_CATALOG_SCHEMA_VERSION, AlphaScenarioCatalog, AlphaScenarioDefinition, AlphaScenarioKind,
  render_alpha_scenario_markdown,
};
pub use checks::{
  ALPHA_RELEASE_CHECKS_SCHEMA_VERSION, AlphaReleaseChecksError, AlphaReleaseChecksManifest,
  CategoryAuditSummary, CheckVerificationStatus, ReleaseCheckAuditRecord, ReleaseCheckCategory,
  ReleaseCheckDefinition, ReleaseCheckSeverity, ReleaseChecksAuditReport, audit_release_checks,
  render_release_checks_report_markdown,
};
pub use compatibility::{
  ALPHA_COMPATIBILITY_SCHEMA_VERSION, CompatibilityDomain, CompatibilityError,
  CompatibilityEvaluationReport, CompatibilityLevel, CompatibilityMatrixDefinition,
  VersionMatrixEntry, evaluate_compatibility_matrix, render_compatibility_report_markdown,
};
pub use data_dictionary::{
  ALPHA_DATA_DICTIONARY_SCHEMA_VERSION, DataCategory, DataDictionaryAuditReport,
  DataDictionaryDefinition, DataDictionaryError, DataFieldDefinition, DataSensitivityLevel,
  audit_data_dictionary, render_data_dictionary_markdown,
};
pub use governance::{
  ALPHA_GOVERNANCE_SCHEMA_VERSION, AlphaGovernanceError, AlphaGovernanceReport, LegalPostureStatus,
  PolicyComplianceArea, PolicyDeclaration, PublicAlphaGovernanceManifest,
  evaluate_alpha_governance, render_governance_report_markdown,
};
pub use guides::{
  ALPHA_GUIDES_SCHEMA_VERSION, AlphaGuidesError, AlphaGuidesManifest, GuideAudience,
  GuideAuditRecord, GuideDocumentDefinition, GuideSection, GuideSectionKind, GuidesAuditReport,
  audit_guide_manifests, render_guides_report_markdown,
};
pub use limitations::{
  ALPHA_LIMITATIONS_SCHEMA_VERSION, AlphaLimitationsDeclaration, AlphaLimitationsError,
  CitationGuidance, ClaimClassification, EvidenceTier, LimitationCategory, LimitationsAuditReport,
  ResearchClaim, audit_limitations_and_boundaries, render_limitations_report_markdown,
};
pub use reproducibility::{
  ALPHA_REPRODUCIBILITY_SCHEMA_VERSION, AlphaReproducibilityError, PackageAuditRecord,
  ReproducibilityAuditReport, ReproducibilityBundleManifest, ReproducibilityPackageDefinition,
  ReproducibilityStatus, SampleArtifactKind, audit_reproducibility_bundle, is_valid_fnv1a_hash,
  render_reproducibility_report_markdown,
};
