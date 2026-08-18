//! M10 Human Usability and Accessibility Alpha Study Protocol and Evaluation Framework.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! This module formalizes:
//! - Study protocol definitions and participant cohort criteria (`protocol.rs`);
//! - Participant session records and completion status (`session.rs`);
//! - Finding taxonomy, severity ranking, and issue-linked disposition tracking (`finding.rs`);
//! - Deterministic cohort evaluation and report generation in basis points (`evaluation.rs`);
//! - Canonical benchmark study scenarios and expectations (`catalog.rs`).

pub mod catalog;
pub mod dimension;
pub mod dimension_catalog;
pub mod evaluation;
pub mod finding;
pub mod interaction;
pub mod protocol;
pub mod session;

#[cfg(test)]
mod tests;

pub use catalog::{
  M10_STUDY_CATALOG_SCHEMA_V1, STANDARD_ALPHA_PROTOCOL, StudyProtocolCatalog,
  StudyScenarioDefinition, StudyScenarioExecutionResult,
};
pub use dimension::{
  CognitiveFrictionIndicator, DimensionEvaluationError, DimensionEvaluationReport, DimensionScore,
  DimensionSummary, M10_DIMENSION_ASSESSMENT_SCHEMA_V1, ParticipantDimensionAssessment,
  evaluate_dimension_assessments,
};
pub use dimension_catalog::{
  DimensionAssessmentCatalog, DimensionScenarioDefinition, DimensionScenarioExecutionResult,
  M10_DIMENSION_CATALOG_SCHEMA_V1,
};
pub use evaluation::{
  CohortMetrics, M10_STUDY_EVALUATION_SCHEMA_V1, STANDARD_EVIDENCE_BOUNDARY, StudyEvaluationError,
  StudyEvaluationReport, evaluate_study_cohort,
};
pub use finding::{
  FindingCategory, FindingDisposition, FindingRecord, FindingSeverity,
  M10_FINDING_TAXONOMY_SCHEMA_V1,
};
pub use interaction::{
  ContrastMode, InteractionAuditCheck, InteractionAuditReport, InteractionProfile,
  M10_INTERACTION_MODE_SCHEMA_V1, VerbosityLevel, audit_interaction_transcript,
};
pub use protocol::{
  EvaluationDimension, M10_STUDY_PROTOCOL_SCHEMA_V1, ParticipantCohort, PrivacyConsentDeclaration,
  StudyProtocolDefinition,
};
pub use session::{
  AccessNeedsDeclaration, CompletionStatus, M10_PARTICIPANT_SESSION_SCHEMA_V1,
  ParticipantSessionRecord,
};
