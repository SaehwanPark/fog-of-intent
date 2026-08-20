//! Public Research-Capable Alpha release governance, compatibility policies, and data dictionary contracts for Fog of Intent.

pub mod catalog;
pub mod compatibility;
pub mod data_dictionary;
pub mod governance;

#[cfg(test)]
pub mod tests;

pub use catalog::{
  ALPHA_CATALOG_SCHEMA_VERSION, AlphaScenarioCatalog, AlphaScenarioDefinition, AlphaScenarioKind,
  render_alpha_scenario_markdown,
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
