//! M10 Human Usability and Accessibility Alpha Study Protocol and Evaluation Framework.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! This module formalizes:
//! - Study protocol definitions and participant cohort criteria (`protocol.rs`);
//! - Participant session records and completion status (`session.rs`);
//! - Finding taxonomy, severity ranking, and issue-linked disposition tracking (`finding.rs`);
//! - Deterministic cohort evaluation and report generation in basis points (`evaluation.rs`);
//! - Canonical benchmark study scenarios and expectations (`catalog.rs`);
//! - Dimension-level assessments and interaction auditing (`dimension.rs`, `interaction.rs`, `dimension_catalog.rs`);
//! - Informal check protocol and remediation evaluation (`informal_check.rs`, `remediation.rs`, `remediation_catalog.rs`).

pub mod catalog;
pub mod dimension;
pub mod dimension_catalog;
pub mod evaluation;
pub mod finding;
pub mod informal_check;
pub mod interaction;
pub mod protocol;
pub mod remediation;
pub mod remediation_catalog;
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
pub use informal_check::{
  InformalCheckMode, InformalCheckPhase, InformalCheckSession, IssueLinkedNote,
  M10_INFORMAL_CHECK_SCHEMA_V1, NoteDisposition,
};
pub use interaction::{
  ContrastMode, InteractionAuditCheck, InteractionAuditReport, InteractionProfile,
  M10_INTERACTION_MODE_SCHEMA_V1, VerbosityLevel, audit_interaction_transcript,
};
pub use protocol::{
  EvaluationDimension, M10_STUDY_PROTOCOL_SCHEMA_V1, ParticipantCohort, PrivacyConsentDeclaration,
  StudyProtocolDefinition,
};
pub use remediation::{
  BP_SCALE, M10_REMEDIATION_EVALUATION_SCHEMA_V1, M10_REMEDIATION_PLAN_SCHEMA_V1,
  MIN_VERIFIED_SHARE_FOR_READINESS_BP, RemediationAction, RemediationEvaluationError,
  RemediationEvaluationReport, RemediationTarget, RemediationVerificationStatus,
  evaluate_remediation_plan,
};
pub use remediation_catalog::{
  M10_REMEDIATION_CATALOG_SCHEMA_V1, RemediationCatalog, RemediationScenarioDefinition,
  RemediationScenarioExecutionResult,
};
pub use session::{
  AccessNeedsDeclaration, CompletionStatus, M10_PARTICIPANT_SESSION_SCHEMA_V1,
  ParticipantSessionRecord,
};
