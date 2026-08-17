//! Canonical usability and accessibility study benchmark scenarios for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Each scenario defines an explicit study population, session records, and
//! findings exercising a distinct evaluation path: a balanced alpha cohort passing
//! all gates and qualifying accessibility claims; an access-needs cohort surfacing
//! an unresolved blocker that disqualifies accessibility claims; and a novice cohort
//! demonstrating high cognitive friction and terminology barriers.

use super::evaluation::{StudyEvaluationError, StudyEvaluationReport, evaluate_study_cohort};
use super::finding::{FindingCategory, FindingDisposition, FindingRecord, FindingSeverity};
use super::protocol::{
  EvaluationDimension, ParticipantCohort, PrivacyConsentDeclaration, StudyProtocolDefinition,
};
use super::session::{AccessNeedsDeclaration, CompletionStatus, ParticipantSessionRecord};

pub const M10_STUDY_CATALOG_SCHEMA_V1: &str = "m10-study-catalog-v1";

/// Standard M10 alpha evaluation protocol definition.
pub const STANDARD_ALPHA_PROTOCOL: StudyProtocolDefinition = StudyProtocolDefinition {
  protocol_id: "protocol-m10-alpha-v1",
  title: "Fog of Intent Human Usability and Accessibility Alpha Protocol",
  research_question: "Can strategy and access-needs participants understand and complete the reference \
     flow, explain their major decisions, and use debriefs to reconstruct outcomes?",
  target_completion_floor_bp: 7_500,
  target_comprehension_floor_bp: 7_000,
  privacy_declaration: PrivacyConsentDeclaration::standard(),
};

/// Specification of a canonical benchmark study scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StudyScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub protocol: StudyProtocolDefinition,
  pub expected_participant_count: usize,
  pub expected_completion_rate_bp: u16,
  pub expected_accessibility_qualified: bool,
  pub expected_completion_target_met: bool,
  pub expected_comprehension_target_met: bool,
}

/// Execution result from running a benchmark study scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StudyScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub report: StudyEvaluationReport,
  pub participant_count_matches: bool,
  pub completion_rate_matches: bool,
  pub accessibility_qualification_matches: bool,
  pub completion_target_matches: bool,
  pub comprehension_target_matches: bool,
  pub all_expectations_met: bool,
}

/// Catalog of registered canonical study scenarios for M10.
pub struct StudyProtocolCatalog;

impl StudyProtocolCatalog {
  /// Scenario 1: Balanced alpha cohort with diverse participants and resolved blockers.
  ///
  /// 8 participants across all 4 cohorts: 7 completed, 1 novice abandoned early.
  /// Blocker resolved in PR #200, major barriers mitigated, positive insights noted.
  /// All gates pass and accessibility claims are qualified.
  pub const SCENARIO_BALANCED_ALPHA: StudyScenarioDefinition = StudyScenarioDefinition {
    scenario_id: "scenario-study-cohort-balanced-alpha-v1",
    name: "Balanced Alpha Cohort",
    description: "Diverse 8-participant sample across strategy, MOBA, access-needs, and novice \
      cohorts. High completion and comprehension; all blockers resolved; accessibility claims qualified.",
    protocol: STANDARD_ALPHA_PROTOCOL,
    expected_participant_count: 8,
    expected_completion_rate_bp: 8_750,
    expected_accessibility_qualified: true,
    expected_completion_target_met: true,
    expected_comprehension_target_met: true,
  };

  /// Scenario 2: Access-needs cohort with an unresolved screen reader blocker.
  ///
  /// 4 participants declaring access needs; 2 completed, 2 abandoned due to screen
  /// reader cursor traps. The blocker is deferred without mitigation, correctly
  /// disqualifying accessibility claims.
  pub const SCENARIO_ACCESS_BARRIERS: StudyScenarioDefinition = StudyScenarioDefinition {
    scenario_id: "scenario-study-cohort-access-friction-v1",
    name: "Access Barriers Cohort",
    description: "4 access-needs participants experiencing screen-reader friction and an \
      unresolved blocker. Correctly disqualifies accessibility claims while preserving honest findings.",
    protocol: STANDARD_ALPHA_PROTOCOL,
    expected_participant_count: 4,
    expected_completion_rate_bp: 5_000,
    expected_accessibility_qualified: false,
    expected_completion_target_met: false,
    expected_comprehension_target_met: true,
  };

  /// Scenario 3: Novice cohort demonstrating terminology and cognitive load friction.
  ///
  /// 4 novice participants; 2 completed with assistance, 2 abandoned. Low comprehension
  /// and completion rates. Findings are documented as limitations, reflecting honest
  /// onboarding barriers.
  pub const SCENARIO_MIXED_NOVICE_FRICTION: StudyScenarioDefinition = StudyScenarioDefinition {
    scenario_id: "scenario-study-cohort-mixed-novice-v1",
    name: "Novice Cohort Friction",
    description: "4 novice strategy participants struggling with domain terminology and pacing \
      cognitive load. Both completion and comprehension targets fail.",
    protocol: STANDARD_ALPHA_PROTOCOL,
    expected_participant_count: 4,
    expected_completion_rate_bp: 5_000,
    expected_accessibility_qualified: false,
    expected_completion_target_met: false,
    expected_comprehension_target_met: false,
  };

  pub const ALL: [StudyScenarioDefinition; 3] = [
    Self::SCENARIO_BALANCED_ALPHA,
    Self::SCENARIO_ACCESS_BARRIERS,
    Self::SCENARIO_MIXED_NOVICE_FRICTION,
  ];

  pub fn find_by_id(scenario_id: &str) -> Option<StudyScenarioDefinition> {
    Self::ALL.into_iter().find(|s| s.scenario_id == scenario_id)
  }

  /// Returns canonical session records and findings for Scenario 1.
  pub fn balanced_alpha_data() -> (Vec<ParticipantSessionRecord>, Vec<FindingRecord>) {
    let sessions = vec![
      ParticipantSessionRecord {
        participant_id: "p-strat-01",
        cohort: ParticipantCohort::StrategyGamer,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 9_000,
        debrief_comprehension_bp: 9_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-strat-02",
        cohort: ParticipantCohort::StrategyGamer,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 8_500,
        debrief_comprehension_bp: 9_000,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-moba-01",
        cohort: ParticipantCohort::MobaPlayer,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 9_500,
        debrief_comprehension_bp: 9_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-moba-02",
        cohort: ParticipantCohort::MobaPlayer,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 9_000,
        debrief_comprehension_bp: 8_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-access-01",
        cohort: ParticipantCohort::AccessNeeds,
        access_needs: AccessNeedsDeclaration {
          screen_reader_user: false,
          color_vision_deficiency: true,
          keyboard_only_user: true,
          reduced_motion_required: false,
        },
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 8_500,
        debrief_comprehension_bp: 8_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-access-02",
        cohort: ParticipantCohort::AccessNeeds,
        access_needs: AccessNeedsDeclaration {
          screen_reader_user: true,
          color_vision_deficiency: false,
          keyboard_only_user: true,
          reduced_motion_required: true,
        },
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 8_000,
        debrief_comprehension_bp: 8_000,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-novice-01",
        cohort: ParticipantCohort::NoviceStrategy,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 7_500,
        debrief_comprehension_bp: 7_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-novice-02",
        cohort: ParticipantCohort::NoviceStrategy,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::AbandonedAtTurn(2),
        explanation_quality_bp: 4_000,
        debrief_comprehension_bp: 4_500,
        turns_completed: 2,
      },
    ];

    let findings = vec![
      FindingRecord {
        finding_id: "f-01",
        participant_id: "p-novice-02",
        dimension: EvaluationDimension::CommandDiscoverability,
        category: FindingCategory::Usability,
        severity: FindingSeverity::Blocker,
        description: "Initial prompt lacked tab-completion hints in non-TTY mode",
        disposition: FindingDisposition::Resolved {
          issue_ref: "PR #200",
        },
      },
      FindingRecord {
        finding_id: "f-02",
        participant_id: "p-access-01",
        dimension: EvaluationDimension::NonColorSemantics,
        category: FindingCategory::Accessibility,
        severity: FindingSeverity::MajorBarrier,
        description: "Status line relied on color without explicit plain-text label",
        disposition: FindingDisposition::Mitigated {
          issue_ref: "PR #200",
        },
      },
      FindingRecord {
        finding_id: "f-03",
        participant_id: "p-access-02",
        dimension: EvaluationDimension::ScreenReaderSuitability,
        category: FindingCategory::Accessibility,
        severity: FindingSeverity::MinorFriction,
        description: "Repeated status headers read on every prompt iteration",
        disposition: FindingDisposition::DocumentedLimitation {
          doc_ref: "docs/ACCESSIBILITY.md#screen-reader-verbosity",
        },
      },
      FindingRecord {
        finding_id: "f-04",
        participant_id: "p-strat-01",
        dimension: EvaluationDimension::DebriefCausalUtility,
        category: FindingCategory::Usability,
        severity: FindingSeverity::PositiveInsight,
        description: "Causal debrief clearly distinguished decision quality from stochastic luck",
        disposition: FindingDisposition::DocumentedLimitation {
          doc_ref: "SPEC.md#m10",
        },
      },
    ];

    (sessions, findings)
  }

  /// Returns canonical session records and findings for Scenario 2.
  pub fn access_barriers_data() -> (Vec<ParticipantSessionRecord>, Vec<FindingRecord>) {
    let sessions = vec![
      ParticipantSessionRecord {
        participant_id: "p-acc-01",
        cohort: ParticipantCohort::AccessNeeds,
        access_needs: AccessNeedsDeclaration {
          screen_reader_user: true,
          color_vision_deficiency: false,
          keyboard_only_user: true,
          reduced_motion_required: false,
        },
        completion_status: CompletionStatus::AbandonedAtTurn(1),
        explanation_quality_bp: 6_000,
        debrief_comprehension_bp: 7_000,
        turns_completed: 1,
      },
      ParticipantSessionRecord {
        participant_id: "p-acc-02",
        cohort: ParticipantCohort::AccessNeeds,
        access_needs: AccessNeedsDeclaration {
          screen_reader_user: true,
          color_vision_deficiency: true,
          keyboard_only_user: true,
          reduced_motion_required: true,
        },
        completion_status: CompletionStatus::AbandonedAtTurn(2),
        explanation_quality_bp: 6_500,
        debrief_comprehension_bp: 7_000,
        turns_completed: 2,
      },
      ParticipantSessionRecord {
        participant_id: "p-acc-03",
        cohort: ParticipantCohort::AccessNeeds,
        access_needs: AccessNeedsDeclaration {
          screen_reader_user: false,
          color_vision_deficiency: true,
          keyboard_only_user: true,
          reduced_motion_required: false,
        },
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 8_500,
        debrief_comprehension_bp: 8_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-acc-04",
        cohort: ParticipantCohort::AccessNeeds,
        access_needs: AccessNeedsDeclaration {
          screen_reader_user: false,
          color_vision_deficiency: false,
          keyboard_only_user: true,
          reduced_motion_required: false,
        },
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 8_000,
        debrief_comprehension_bp: 8_000,
        turns_completed: 14,
      },
    ];

    let findings = vec![FindingRecord {
      finding_id: "f-acc-01",
      participant_id: "p-acc-01",
      dimension: EvaluationDimension::ScreenReaderSuitability,
      category: FindingCategory::Accessibility,
      severity: FindingSeverity::Blocker,
      description: "Screen reader cannot read interactive command loop prompt in raw mode",
      disposition: FindingDisposition::Deferred {
        rationale: "Requires reedline upstream accessibility fix",
      },
    }];

    (sessions, findings)
  }

  /// Returns canonical session records and findings for Scenario 3.
  pub fn mixed_novice_friction_data() -> (Vec<ParticipantSessionRecord>, Vec<FindingRecord>) {
    let sessions = vec![
      ParticipantSessionRecord {
        participant_id: "p-nov-01",
        cohort: ParticipantCohort::NoviceStrategy,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 6_000,
        debrief_comprehension_bp: 6_500,
        turns_completed: 14,
      },
      ParticipantSessionRecord {
        participant_id: "p-nov-02",
        cohort: ParticipantCohort::NoviceStrategy,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::AbandonedAtTurn(1),
        explanation_quality_bp: 4_000,
        debrief_comprehension_bp: 4_500,
        turns_completed: 1,
      },
      ParticipantSessionRecord {
        participant_id: "p-nov-03",
        cohort: ParticipantCohort::NoviceStrategy,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::AbandonedAtTurn(3),
        explanation_quality_bp: 5_000,
        debrief_comprehension_bp: 5_000,
        turns_completed: 3,
      },
      ParticipantSessionRecord {
        participant_id: "p-nov-04",
        cohort: ParticipantCohort::NoviceStrategy,
        access_needs: AccessNeedsDeclaration::none(),
        completion_status: CompletionStatus::Completed,
        explanation_quality_bp: 6_500,
        debrief_comprehension_bp: 6_000,
        turns_completed: 14,
      },
    ];

    let findings = vec![
      FindingRecord {
        finding_id: "f-nov-01",
        participant_id: "p-nov-02",
        dimension: EvaluationDimension::TerminologyClarity,
        category: FindingCategory::Usability,
        severity: FindingSeverity::MajorBarrier,
        description: "Concept of intent vs execution was confusing without an introductory primer",
        disposition: FindingDisposition::DocumentedLimitation {
          doc_ref: "docs/HOW_TO_PLAY.md#intent-vs-execution",
        },
      },
      FindingRecord {
        finding_id: "f-nov-02",
        participant_id: "p-nov-03",
        dimension: EvaluationDimension::PacingLoad,
        category: FindingCategory::GameplayBalance,
        severity: FindingSeverity::MinorFriction,
        description: "Decision window information density felt high for first-time players",
        disposition: FindingDisposition::DocumentedLimitation {
          doc_ref: "docs/HOW_TO_PLAY.md#reading-the-window",
        },
      },
    ];

    (sessions, findings)
  }

  /// Executes a canonical benchmark scenario by ID and verifies expectations.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<StudyScenarioExecutionResult, StudyEvaluationError> {
    let scenario = Self::find_by_id(scenario_id).ok_or(StudyEvaluationError::EmptyPopulation)?;
    let (sessions, findings) = match scenario.scenario_id {
      "scenario-study-cohort-balanced-alpha-v1" => Self::balanced_alpha_data(),
      "scenario-study-cohort-access-friction-v1" => Self::access_barriers_data(),
      "scenario-study-cohort-mixed-novice-v1" => Self::mixed_novice_friction_data(),
      _ => return Err(StudyEvaluationError::EmptyPopulation),
    };

    let report = evaluate_study_cohort(&scenario.protocol, &sessions, &findings)?;

    let participant_count_matches =
      report.total_participants == scenario.expected_participant_count;
    let completion_rate_matches =
      report.overall_completion_rate_bp == scenario.expected_completion_rate_bp;
    let accessibility_qualification_matches =
      report.accessibility_claims_qualified == scenario.expected_accessibility_qualified;
    let completion_target_matches =
      report.completion_target_met == scenario.expected_completion_target_met;
    let comprehension_target_matches =
      report.comprehension_target_met == scenario.expected_comprehension_target_met;

    let all_expectations_met = participant_count_matches
      && completion_rate_matches
      && accessibility_qualification_matches
      && completion_target_matches
      && comprehension_target_matches;

    Ok(StudyScenarioExecutionResult {
      scenario_id: scenario.scenario_id,
      report,
      participant_count_matches,
      completion_rate_matches,
      accessibility_qualification_matches,
      completion_target_matches,
      comprehension_target_matches,
      all_expectations_met,
    })
  }
}
