//! Canonical empirical multi-cohort trial scenario catalog for Milestone M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Registers 4 benchmark multi-cohort empirical study trial scenarios:
//! 1. `BalancedAlpha`: Full 4-cohort trial (12 participants) meeting all target floors.
//! 2. `AccessFocused`: Accessibility-focused trial testing assistive tool qualification and blocker gating.
//! 3. `NoviceOnboarding`: Novice-focused trial identifying onboarding friction and terminology gaps.
//! 4. `StrategyMobaContrast`: Contrast trial comparing StrategyGamer vs MobaPlayer expectations.

use super::dimension::CognitiveFrictionIndicator;
use super::empirical_trials::{
  EMPIRICAL_ALPHA_PROTOCOL, EmpiricalCohortError, EmpiricalCohortTrialReport,
  EmpiricalTrialSession, evaluate_empirical_trials,
};
use super::finding::{FindingCategory, FindingDisposition, FindingRecord, FindingSeverity};
use super::protocol::{EvaluationDimension, ParticipantCohort};
use super::session::{AccessNeedsDeclaration, CompletionStatus, ParticipantSessionRecord};

/// Versioned catalog schema identifier.
pub const M10_EMPI_COHORT_CATALOG_SCHEMA_V1: &str = "m10-empirical-cohort-catalog-v1";

/// Definition of an empirical cohort trial scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmpiricalTrialScenarioDefinition {
  /// Unique stable identifier.
  pub scenario_id: &'static str,
  /// Human-readable title.
  pub title: &'static str,
  /// Scenario description and research rationale.
  pub description: &'static str,
  /// Expected total participant count.
  pub expected_participant_count: usize,
  /// Expected overall completion rate in basis points ([0..=10,000] bp).
  pub expected_completion_rate_bp: u16,
  /// Expected accessibility qualification status.
  pub expected_accessibility_qualified: bool,
  /// Expected alpha readiness gate disposition.
  pub expected_alpha_ready: bool,
}

/// Execution result for an empirical cohort trial scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmpiricalTrialExecutionResult {
  pub scenario_id: &'static str,
  pub report: EmpiricalCohortTrialReport,
  pub participant_count_matches: bool,
  pub completion_rate_matches: bool,
  pub accessibility_matches: bool,
  pub alpha_ready_matches: bool,
  pub all_expectations_met: bool,
}

/// Canonical catalog containing all M10 empirical multi-cohort benchmark scenarios.
pub struct EmpiricalTrialsCatalog;

impl EmpiricalTrialsCatalog {
  /// 1. Balanced Alpha Benchmark Scenario (12 participants across all 4 cohorts).
  pub const BALANCED_ALPHA: EmpiricalTrialScenarioDefinition = EmpiricalTrialScenarioDefinition {
    scenario_id: "scenario-cohort-trial-balanced-alpha-v1",
    title: "Balanced Alpha Multi-Cohort Usability & Accessibility Trials",
    description: "Evaluates 12 participants across StrategyGamer (3), MobaPlayer (3), AccessNeeds (3), and NoviceStrategy (3). Meets all completion and comprehension floors.",
    expected_participant_count: 12,
    expected_completion_rate_bp: 9_166,
    expected_accessibility_qualified: true,
    expected_alpha_ready: true,
  };

  /// 2. Access-Focused Blocker Gating Scenario (8 participants).
  pub const ACCESS_FOCUSED: EmpiricalTrialScenarioDefinition = EmpiricalTrialScenarioDefinition {
    scenario_id: "scenario-cohort-trial-access-focused-v1",
    title: "Accessibility-Focused Assistive Flow & Blocker Gating Trial",
    description: "Evaluates screen-reader, keyboard-only, and non-color interaction flows with active accessibility blockers, demonstrating fail-closed alpha disqualification.",
    expected_participant_count: 8,
    expected_completion_rate_bp: 8_750,
    expected_accessibility_qualified: false,
    expected_alpha_ready: false,
  };

  /// 3. Novice Onboarding & Cognitive Friction Scenario (8 participants).
  pub const NOVICE_ONBOARDING: EmpiricalTrialScenarioDefinition =
    EmpiricalTrialScenarioDefinition {
      scenario_id: "scenario-cohort-trial-novice-onboarding-v1",
      title: "Novice Onboarding & Cognitive Friction Diagnostic Trial",
      description: "Evaluates novice strategy players encountering domain terminology friction, measuring completion drop-off and remediation impact.",
      expected_participant_count: 8,
      expected_completion_rate_bp: 7_500,
      expected_accessibility_qualified: true,
      expected_alpha_ready: false,
    };

  /// 4. Strategy Gamer vs MOBA Player Contrast Scenario (8 participants).
  pub const STRATEGY_MOBA_CONTRAST: EmpiricalTrialScenarioDefinition =
    EmpiricalTrialScenarioDefinition {
      scenario_id: "scenario-cohort-trial-strategy-moba-contrast-v1",
      title: "Strategy Gamer vs. MOBA Player Mental Model Contrast",
      description: "Contrasts turn-based 4X/tactics expectations against MOBA micro-reflex instincts in delegated execution and causal debrief comprehension.",
      expected_participant_count: 8,
      expected_completion_rate_bp: 10_000,
      expected_accessibility_qualified: true,
      expected_alpha_ready: true,
    };

  /// All registered empirical cohort trial scenarios.
  pub const ALL: [EmpiricalTrialScenarioDefinition; 4] = [
    Self::BALANCED_ALPHA,
    Self::ACCESS_FOCUSED,
    Self::NOVICE_ONBOARDING,
    Self::STRATEGY_MOBA_CONTRAST,
  ];

  /// Find definition by scenario identifier.
  #[must_use]
  pub fn find_by_id(scenario_id: &str) -> Option<EmpiricalTrialScenarioDefinition> {
    Self::ALL
      .iter()
      .copied()
      .find(|s| s.scenario_id == scenario_id)
  }

  /// Execute an empirical cohort trial scenario by ID.
  pub fn execute_by_id(
    scenario_id: &str,
  ) -> Result<EmpiricalTrialExecutionResult, EmpiricalCohortError> {
    let def = Self::find_by_id(scenario_id).ok_or(EmpiricalCohortError::EmptySessionList)?;

    let (sessions, findings) = match scenario_id {
      "scenario-cohort-trial-balanced-alpha-v1" => Self::fixture_balanced_alpha(),
      "scenario-cohort-trial-access-focused-v1" => Self::fixture_access_focused(),
      "scenario-cohort-trial-novice-onboarding-v1" => Self::fixture_novice_onboarding(),
      "scenario-cohort-trial-strategy-moba-contrast-v1" => Self::fixture_strategy_moba_contrast(),
      _ => return Err(EmpiricalCohortError::EmptySessionList),
    };

    let report = evaluate_empirical_trials(&EMPIRICAL_ALPHA_PROTOCOL, &sessions, &findings)?;

    let participant_count_matches = report.total_participants == def.expected_participant_count;
    let completion_rate_matches =
      report.overall_completion_rate_bp == def.expected_completion_rate_bp;
    let accessibility_matches =
      report.accessibility_qualified == def.expected_accessibility_qualified;
    let alpha_ready_matches = report.is_alpha_ready() == def.expected_alpha_ready;

    let all_expectations_met = participant_count_matches
      && completion_rate_matches
      && accessibility_matches
      && alpha_ready_matches;

    Ok(EmpiricalTrialExecutionResult {
      scenario_id: def.scenario_id,
      report,
      participant_count_matches,
      completion_rate_matches,
      accessibility_matches,
      alpha_ready_matches,
      all_expectations_met,
    })
  }

  /// Execute all registered empirical cohort trial benchmark scenarios.
  pub fn execute_all() -> Result<Vec<EmpiricalTrialExecutionResult>, EmpiricalCohortError> {
    Self::ALL
      .iter()
      .map(|def| Self::execute_by_id(def.scenario_id))
      .collect()
  }

  // --- Fixture Generators ---

  fn fixture_balanced_alpha() -> (Vec<EmpiricalTrialSession>, Vec<FindingRecord>) {
    let sessions = vec![
      // StrategyGamer (3)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-STRAT-01",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PerceivedAgency,
        reported_frictions: vec![],
        qualitative_notes: "Strong strategic agency observed.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-STRAT-02",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DebriefCausalUtility,
        reported_frictions: vec![],
        qualitative_notes: "Causal debrief 4-quadrant attribution is immediately clear.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-STRAT-03",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_000,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PacingLoad,
        reported_frictions: vec![CognitiveFrictionIndicator::HighCognitiveLoad],
        qualitative_notes: "Good pacing; minor cognitive density during neutral objective contest.",
      },
      // MobaPlayer (3)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-MOBA-01",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DelegatedFairness,
        reported_frictions: vec![],
        qualitative_notes: "Appreciated macro focus over micro mechanical reflexes.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-MOBA-02",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::TerminologyClarity,
        reported_frictions: vec![],
        qualitative_notes: "Intuitive mapping of lane, ward, and objective terms.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-MOBA-03",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PerceivedAgency,
        reported_frictions: vec![],
        qualitative_notes: "Rotation and siege commands felt tactically impactful.",
      },
      // AccessNeeds (3)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-ACC-01",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: true,
            color_vision_deficiency: false,
            keyboard_only_user: true,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_000,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::ScreenReaderSuitability,
        reported_frictions: vec![],
        qualitative_notes: "Linear plain text and WCAG 2.1 AA HTML presentation worked seamlessly.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-ACC-02",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: false,
            color_vision_deficiency: true,
            keyboard_only_user: false,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::NonColorSemantics,
        reported_frictions: vec![],
        qualitative_notes: "Textual prefixes and symbolic tags eliminated color dependency.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-ACC-03",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: false,
            color_vision_deficiency: false,
            keyboard_only_user: true,
            reduced_motion_required: true,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::KeyboardFlow,
        reported_frictions: vec![],
        qualitative_notes: "Tab completion and single-key shortcuts provided smooth keyboard flow.",
      },
      // NoviceStrategy (3)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-01",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 7_500,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::Onboarding,
        reported_frictions: vec![CognitiveFrictionIndicator::AmbiguousTerminology],
        qualitative_notes: "Initial terms took a turn to grasp, but tutorial hints helped.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-02",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_000,
          debrief_comprehension_bp: 7_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::CommandDiscoverability,
        reported_frictions: vec![],
        qualitative_notes: "Numbered choice menu made command discovery straightforward.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-03",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::AbandonedAtTurn(4),
          explanation_quality_bp: 6_500,
          debrief_comprehension_bp: 6_500,
          turns_completed: 4,
        },
        primary_dimension_focus: EvaluationDimension::Onboarding,
        reported_frictions: vec![CognitiveFrictionIndicator::AmbiguousTerminology],
        qualitative_notes: "Abandoned turn 4 due to terminology confusion.",
      },
    ];

    let findings = vec![
      FindingRecord {
        finding_id: "FINDING-EMPI-01",
        participant_id: "P-NOV-01",
        dimension: EvaluationDimension::Onboarding,
        severity: FindingSeverity::MinorFriction,
        category: FindingCategory::Usability,
        description: "Initial fog terminology benefits from explicit onboarding tooltips",
        disposition: FindingDisposition::Mitigated {
          issue_ref: "docs/guides/onboarding.md",
        },
      },
      FindingRecord {
        finding_id: "FINDING-EMPI-02",
        participant_id: "P-ACC-03",
        dimension: EvaluationDimension::KeyboardFlow,
        severity: FindingSeverity::PositiveInsight,
        category: FindingCategory::Accessibility,
        description: "Tab completion and numbered choices praised by keyboard-only users",
        disposition: FindingDisposition::Resolved {
          issue_ref: "PR #204",
        },
      },
    ];

    (sessions, findings)
  }

  fn fixture_access_focused() -> (Vec<EmpiricalTrialSession>, Vec<FindingRecord>) {
    let sessions = vec![
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-01",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: true,
            color_vision_deficiency: false,
            keyboard_only_user: true,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 7_500,
          debrief_comprehension_bp: 7_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::ScreenReaderSuitability,
        reported_frictions: vec![CognitiveFrictionIndicator::NavigationDisorientation],
        qualitative_notes: "Navigated via screen reader with some disorientation on multi-line tables.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-02",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: true,
            color_vision_deficiency: false,
            keyboard_only_user: true,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::AbandonedAtTurn(5),
          explanation_quality_bp: 5_000,
          debrief_comprehension_bp: 5_000,
          turns_completed: 5,
        },
        primary_dimension_focus: EvaluationDimension::ScreenReaderSuitability,
        reported_frictions: vec![CognitiveFrictionIndicator::NavigationDisorientation],
        qualitative_notes: "Abandoned turn 5 due to unlabelled prompt control loop.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-03",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: false,
            color_vision_deficiency: true,
            keyboard_only_user: false,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_000,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::NonColorSemantics,
        reported_frictions: vec![],
        qualitative_notes: "Non-color tags sufficient.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-04",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: false,
            color_vision_deficiency: false,
            keyboard_only_user: true,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::KeyboardFlow,
        reported_frictions: vec![],
        qualitative_notes: "Keyboard shortcuts operational.",
      },
      // Baseline cohorts
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-05",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PerceivedAgency,
        reported_frictions: vec![],
        qualitative_notes: "Strategy player completed.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-06",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DebriefCausalUtility,
        reported_frictions: vec![],
        qualitative_notes: "Debrief verified.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-07",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DelegatedFairness,
        reported_frictions: vec![],
        qualitative_notes: "Delegation accepted.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-AF-08",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_000,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::CommandDiscoverability,
        reported_frictions: vec![],
        qualitative_notes: "Novice completed.",
      },
    ];

    let findings = vec![FindingRecord {
      finding_id: "FINDING-AF-01",
      participant_id: "P-AF-02",
      dimension: EvaluationDimension::ScreenReaderSuitability,
      severity: FindingSeverity::Blocker,
      category: FindingCategory::Accessibility,
      description: "Screen reader cursor trap on unlabelled secondary prompt control",
      disposition: FindingDisposition::Deferred {
        rationale: "Fix pending in next assistive pass",
      },
    }];

    (sessions, findings)
  }

  fn fixture_novice_onboarding() -> (Vec<EmpiricalTrialSession>, Vec<FindingRecord>) {
    let sessions = vec![
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-01",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 7_000,
          debrief_comprehension_bp: 7_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::Onboarding,
        reported_frictions: vec![CognitiveFrictionIndicator::AmbiguousTerminology],
        qualitative_notes: "Completed with high cognitive effort.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-02",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 7_500,
          debrief_comprehension_bp: 7_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::TerminologyClarity,
        reported_frictions: vec![CognitiveFrictionIndicator::AmbiguousTerminology],
        qualitative_notes: "Unclear on fallback vs abort condition.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-03",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::AbandonedAtTurn(3),
          explanation_quality_bp: 5_500,
          debrief_comprehension_bp: 5_000,
          turns_completed: 3,
        },
        primary_dimension_focus: EvaluationDimension::Onboarding,
        reported_frictions: vec![CognitiveFrictionIndicator::AmbiguousTerminology],
        qualitative_notes: "Abandoned turn 3.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-04",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::AbandonedAtTurn(6),
          explanation_quality_bp: 6_000,
          debrief_comprehension_bp: 5_500,
          turns_completed: 6,
        },
        primary_dimension_focus: EvaluationDimension::PacingLoad,
        reported_frictions: vec![CognitiveFrictionIndicator::PacingOverwhelm],
        qualitative_notes: "Abandoned turn 6 due to pacing overload.",
      },
      // Baseline cohorts
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-05",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PerceivedAgency,
        reported_frictions: vec![],
        qualitative_notes: "Strategy gamer completed.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-06",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DebriefCausalUtility,
        reported_frictions: vec![],
        qualitative_notes: "Debrief understood.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-07",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DelegatedFairness,
        reported_frictions: vec![],
        qualitative_notes: "MOBA player completed.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-NOV-ON-08",
          cohort: ParticipantCohort::AccessNeeds,
          access_needs: AccessNeedsDeclaration {
            screen_reader_user: false,
            color_vision_deficiency: true,
            keyboard_only_user: true,
            reduced_motion_required: false,
          },
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::KeyboardFlow,
        reported_frictions: vec![],
        qualitative_notes: "Access completed.",
      },
    ];

    let findings = vec![FindingRecord {
      finding_id: "FINDING-NOV-01",
      participant_id: "P-NOV-ON-03",
      dimension: EvaluationDimension::Onboarding,
      severity: FindingSeverity::MajorBarrier,
      category: FindingCategory::Usability,
      description: "Contingency and fallback terms create high novice friction without guided tutorial",
      disposition: FindingDisposition::Deferred {
        rationale: "Requires guided onboarding module",
      },
    }];

    (sessions, findings)
  }

  fn fixture_strategy_moba_contrast() -> (Vec<EmpiricalTrialSession>, Vec<FindingRecord>) {
    let sessions = vec![
      // StrategyGamer (3)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-01",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PerceivedAgency,
        reported_frictions: vec![],
        qualitative_notes: "Turn-based structure feels natural and rewarding.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-02",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DebriefCausalUtility,
        reported_frictions: vec![],
        qualitative_notes: "Causal attribution helps identify strategic errors.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-03",
          cohort: ParticipantCohort::StrategyGamer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_500,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PacingLoad,
        reported_frictions: vec![],
        qualitative_notes: "Decision density is well-calibrated.",
      },
      // MobaPlayer (3)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-04",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_500,
          debrief_comprehension_bp: 9_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::DelegatedFairness,
        reported_frictions: vec![],
        qualitative_notes: "Felt like playing shotcaller in competitive MOBA.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-05",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::TerminologyClarity,
        reported_frictions: vec![],
        qualitative_notes: "Dragon and Herald tradeoffs aligned with MOBA intuition.",
      },
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-06",
          cohort: ParticipantCohort::MobaPlayer,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 9_000,
          debrief_comprehension_bp: 9_500,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::PerceivedAgency,
        reported_frictions: vec![],
        qualitative_notes: "Wards and rotations gave great tactical control.",
      },
      // AccessNeeds (1)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-07",
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
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::KeyboardFlow,
        reported_frictions: vec![],
        qualitative_notes: "Keyboard flow smooth.",
      },
      // NoviceStrategy (1)
      EmpiricalTrialSession {
        session: ParticipantSessionRecord {
          participant_id: "P-CONT-08",
          cohort: ParticipantCohort::NoviceStrategy,
          access_needs: AccessNeedsDeclaration::none(),
          completion_status: CompletionStatus::Completed,
          explanation_quality_bp: 8_000,
          debrief_comprehension_bp: 8_000,
          turns_completed: 15,
        },
        primary_dimension_focus: EvaluationDimension::CommandDiscoverability,
        reported_frictions: vec![],
        qualitative_notes: "Novice completed full run.",
      },
    ];

    let findings = vec![FindingRecord {
      finding_id: "FINDING-CONT-01",
      participant_id: "P-CONT-04",
      dimension: EvaluationDimension::PerceivedAgency,
      severity: FindingSeverity::PositiveInsight,
      category: FindingCategory::GameplayBalance,
      description: "MOBA veterans praise delegation mechanic for removing mechanical execution noise and focusing on macro tactics",
      disposition: FindingDisposition::Resolved {
        issue_ref: "PR #205",
      },
    }];

    (sessions, findings)
  }
}
