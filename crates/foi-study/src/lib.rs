//! M10 Human Usability and Accessibility Alpha Study Protocol and Evaluation Framework.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha

pub mod catalog;
pub mod dimension;
pub mod dimension_catalog;
pub mod empirical_trials;
pub mod empirical_trials_catalog;
pub mod evaluation;
pub mod finding;
pub mod informal_check;
pub mod interaction;
pub mod protocol;
pub mod remediation;
pub mod remediation_catalog;
pub mod sampling;
pub mod session;
pub mod synthesis;
pub mod synthesis_catalog;

pub use crate as study;

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
pub use empirical_trials::{
  CohortTrialSummary, EMPIRICAL_ALPHA_PROTOCOL, EmpiricalCohortError, EmpiricalCohortTrialReport,
  EmpiricalTrialSession, M10_EMPI_COHORT_TRIALS_SCHEMA_V1, evaluate_empirical_trials,
};
pub use empirical_trials_catalog::{
  EmpiricalTrialExecutionResult, EmpiricalTrialScenarioDefinition, EmpiricalTrialsCatalog,
  M10_EMPI_COHORT_CATALOG_SCHEMA_V1,
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
pub use sampling::{
  AccessNeedsBreakdown, CohortRepresentation, DEFAULT_MIN_COHORT_FLOOR_BP,
  M10_SAMPLING_LIMITS_SCHEMA_V1, ParticipantSamplingReport, STANDARD_UNTESTED_DISCLOSURES,
  SamplingEvaluationError, SamplingLimitsDeclaration, UntestedPopulationCategory,
  UntestedPopulationDisclosure, evaluate_participant_sampling,
};
pub use session::{
  AccessNeedsDeclaration, CompletionStatus, M10_PARTICIPANT_SESSION_SCHEMA_V1,
  ParticipantSessionRecord,
};
pub use synthesis::{
  AlphaDisposition, AlphaEvidenceSynthesis, AlphaReadinessGateStatus,
  EmpiricalFactVsInferredHypothesis, M10_ALPHA_SYNTHESIS_SCHEMA_V1, SynthesisEvaluationError,
  synthesize_alpha_evidence,
};
pub use synthesis_catalog::{
  AlphaSynthesisCatalog, AlphaSynthesisExecutionResult, AlphaSynthesisScenarioDefinition,
  M10_SYNTHESIS_CATALOG_SCHEMA_V1,
};
