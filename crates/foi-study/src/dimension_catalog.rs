//! Canonical benchmark dimension evaluation scenarios for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Registers benchmark dimension assessment cohorts exercising distinct
//! evaluation trajectories: a balanced multi-cohort alpha benchmark, a screen-reader
//! focused accessibility audit cohort, and a novice strategy friction cohort.

use super::catalog::STANDARD_ALPHA_PROTOCOL;
use super::dimension::{
  CognitiveFrictionIndicator, DimensionEvaluationError, DimensionEvaluationReport, DimensionScore,
  ParticipantDimensionAssessment, evaluate_dimension_assessments,
};
use super::protocol::{EvaluationDimension, ParticipantCohort, StudyProtocolDefinition};

pub const M10_DIMENSION_CATALOG_SCHEMA_V1: &str = "m10-dimension-catalog-v1";

/// Specification of a canonical benchmark dimension scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DimensionScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub protocol: StudyProtocolDefinition,
  pub expected_assessment_count: usize,
  pub expected_overall_mean_bp: u16,
  pub expected_weakest_dimension: EvaluationDimension,
  pub expected_strongest_dimension: EvaluationDimension,
  pub expected_accessibility_qualified: bool,
}

/// Result of executing a benchmark dimension scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub report: DimensionEvaluationReport,
  pub assessment_count_matches: bool,
  pub overall_mean_matches: bool,
  pub weakest_dimension_matches: bool,
  pub strongest_dimension_matches: bool,
  pub accessibility_qualification_matches: bool,
  pub all_expectations_met: bool,
}

/// Catalog of registered canonical dimension assessment scenarios.
pub struct DimensionAssessmentCatalog;

impl DimensionAssessmentCatalog {
  /// Scenario 1: Balanced multi-cohort alpha benchmark across all 10 dimensions.
  pub const SCENARIO_BALANCED_ALPHA: DimensionScenarioDefinition = DimensionScenarioDefinition {
    scenario_id: "scenario-dimension-alpha-benchmark-v1",
    name: "Balanced Alpha 10-Dimension Benchmark",
    description: "4-participant representative sample evaluating all 10 dimensions. High overall \
      ratings with strongest performance on keyboard flow and minor friction on pacing cognitive load.",
    protocol: STANDARD_ALPHA_PROTOCOL,
    expected_assessment_count: 4,
    expected_overall_mean_bp: 8_437,
    expected_weakest_dimension: EvaluationDimension::PacingLoad,
    expected_strongest_dimension: EvaluationDimension::KeyboardFlow,
    expected_accessibility_qualified: true,
  };

  /// Scenario 2: Access-needs cohort highlighting screen reader friction.
  pub const SCENARIO_SCREEN_READER_AUDIT: DimensionScenarioDefinition =
    DimensionScenarioDefinition {
      scenario_id: "scenario-dimension-screen-reader-audit-v1",
      name: "Screen Reader Friction Audit",
      description: "2 access-needs participants evaluating interaction modes. Identifies screen \
        reader verbosity friction and disqualifies accessibility claims until mitigated.",
      protocol: STANDARD_ALPHA_PROTOCOL,
      expected_assessment_count: 2,
      expected_overall_mean_bp: 7_800,
      expected_weakest_dimension: EvaluationDimension::ScreenReaderSuitability,
      expected_strongest_dimension: EvaluationDimension::KeyboardFlow,
      expected_accessibility_qualified: false,
    };

  /// Scenario 3: Novice strategy cohort experiencing terminology and pacing friction.
  pub const SCENARIO_NOVICE_FRICTION: DimensionScenarioDefinition = DimensionScenarioDefinition {
    scenario_id: "scenario-dimension-novice-friction-v1",
    name: "Novice Strategy Cognitive Friction",
    description: "2 novice participants struggling with domain vocabulary and pacing load. \
      Identifies terminology clarity as the weakest dimension.",
    protocol: STANDARD_ALPHA_PROTOCOL,
    expected_assessment_count: 2,
    expected_overall_mean_bp: 6_725,
    expected_weakest_dimension: EvaluationDimension::TerminologyClarity,
    expected_strongest_dimension: EvaluationDimension::KeyboardFlow,
    expected_accessibility_qualified: true,
  };

  pub const ALL: [DimensionScenarioDefinition; 3] = [
    Self::SCENARIO_BALANCED_ALPHA,
    Self::SCENARIO_SCREEN_READER_AUDIT,
    Self::SCENARIO_NOVICE_FRICTION,
  ];

  pub fn find_by_id(scenario_id: &str) -> Option<DimensionScenarioDefinition> {
    Self::ALL.into_iter().find(|s| s.scenario_id == scenario_id)
  }

  /// Returns canonical 10-dimension assessment data for Scenario 1.
  pub fn balanced_alpha_data() -> Vec<ParticipantDimensionAssessment> {
    vec![
      ParticipantDimensionAssessment {
        participant_id: "p-strat-01",
        cohort: ParticipantCohort::StrategyGamer,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Clear manual walkthrough",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Domain terms intuitive",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Tab completion works well",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::PacingOverwhelm,
            notes: "Decision window has dense status lines",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Meaningful tradeoff choices",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Delegation outcomes plausible",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Causal attribution table very helpful",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Fast keyboard command entry",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Bracketed status tags clear",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Linear text format readable",
          },
        ],
      },
      ParticipantDimensionAssessment {
        participant_id: "p-moba-01",
        cohort: ParticipantCohort::MobaPlayer,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Fast onboarding for MOBA players",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Lane and wave concepts instantly understood",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Commands mapped naturally",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Pacing matches turn-based expectations",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "High strategic agency in macro calls",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Execution variance feels fair",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Clear distinction between luck and skill",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Snappy CLI control",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Brackets work well",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Accessible text flow",
          },
        ],
      },
      ParticipantDimensionAssessment {
        participant_id: "p-acc-01",
        cohort: ParticipantCohort::AccessNeeds,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Accessible markdown docs",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Glossary definitions helped",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Help topic discovery was straightforward",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::HighCognitiveLoad,
            notes: "Lots of numbers on screen at once",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Decisions felt impactful",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Understood why orders succeeded or failed",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Debrief gave good closure",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Zero mouse dependency",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 9_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Color deficiency caused no loss of information",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Screen reader navigated prompts cleanly",
          },
        ],
      },
      ParticipantDimensionAssessment {
        participant_id: "p-nov-01",
        cohort: ParticipantCohort::NoviceStrategy,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::AmbiguousTerminology,
            notes: "Needed to re-read intent vs execution rules",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::AmbiguousTerminology,
            notes: "Terminology was unfamiliar initially",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::HiddenActionAffordance,
            notes: "Took time to discover available plan options",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 5_500,
            friction: CognitiveFrictionIndicator::HighCognitiveLoad,
            notes: "Felt overwhelmed by simultaneous actor statuses",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Saw how choices affected outcome",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::UnclearCausalTrace,
            notes: "Was unsure why allied bot yielded on beat 2",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Debrief explained bot yield rationale",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Typing commands was easy",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Text status easy to understand",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Plain text was fine",
          },
        ],
      },
    ]
  }

  /// Returns canonical 10-dimension assessment data for Scenario 2.
  pub fn screen_reader_audit_data() -> Vec<ParticipantDimensionAssessment> {
    vec![
      ParticipantDimensionAssessment {
        participant_id: "p-acc-01",
        cohort: ParticipantCohort::AccessNeeds,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Accessible text",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Clear definitions",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Good help output",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::HighCognitiveLoad,
            notes: "Verbose screen reader output",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Good agency",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Fair simulation",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Clear debrief",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Keyboard flow works",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 8_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "No color dependence",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 6_500,
            friction: CognitiveFrictionIndicator::HighCognitiveLoad,
            notes: "Screen reader repeatedly announces header lines",
          },
        ],
      },
      ParticipantDimensionAssessment {
        participant_id: "p-acc-02",
        cohort: ParticipantCohort::AccessNeeds,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Good onboarding",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Clear terms",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Help works",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::HighCognitiveLoad,
            notes: "Too many spoken lines per turn",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Good agency",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Fair delegation",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Debrief useful",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 9_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Key navigation good",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 8_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Symbols readable",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 6_500,
            friction: CognitiveFrictionIndicator::NavigationDisorientation,
            notes: "Focus jumps when prompt redraws",
          },
        ],
      },
    ]
  }

  /// Returns canonical 10-dimension assessment data for Scenario 3.
  pub fn novice_friction_data() -> Vec<ParticipantDimensionAssessment> {
    vec![
      ParticipantDimensionAssessment {
        participant_id: "p-nov-01",
        cohort: ParticipantCohort::NoviceStrategy,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 6_000,
            friction: CognitiveFrictionIndicator::AmbiguousTerminology,
            notes: "Needed simpler terminology guide",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 5_500,
            friction: CognitiveFrictionIndicator::AmbiguousTerminology,
            notes: "Did not understand why intent is different from execution",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 6_500,
            friction: CognitiveFrictionIndicator::HiddenActionAffordance,
            notes: "Unclear what commands were valid",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 6_000,
            friction: CognitiveFrictionIndicator::PacingOverwhelm,
            notes: "Felt rushed reading options",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Choices mattered",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 6_500,
            friction: CognitiveFrictionIndicator::UnclearCausalTrace,
            notes: "Was surprised by damage result",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Debrief clarified damage math",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Keyboard entry worked",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Text tags fine",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Linear output fine",
          },
        ],
      },
      ParticipantDimensionAssessment {
        participant_id: "p-nov-02",
        cohort: ParticipantCohort::NoviceStrategy,
        scores: [
          DimensionScore {
            dimension: EvaluationDimension::Onboarding,
            score_bp: 6_500,
            friction: CognitiveFrictionIndicator::AmbiguousTerminology,
            notes: "Tutorial could be shorter",
          },
          DimensionScore {
            dimension: EvaluationDimension::TerminologyClarity,
            score_bp: 6_000,
            friction: CognitiveFrictionIndicator::AmbiguousTerminology,
            notes: "Unclear terms: wave pressure, contest fallback",
          },
          DimensionScore {
            dimension: EvaluationDimension::CommandDiscoverability,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::HiddenActionAffordance,
            notes: "Help command was helpful",
          },
          DimensionScore {
            dimension: EvaluationDimension::PacingLoad,
            score_bp: 6_000,
            friction: CognitiveFrictionIndicator::HighCognitiveLoad,
            notes: "Too many variables at once",
          },
          DimensionScore {
            dimension: EvaluationDimension::PerceivedAgency,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Agency felt reasonable",
          },
          DimensionScore {
            dimension: EvaluationDimension::DelegatedFairness,
            score_bp: 6_500,
            friction: CognitiveFrictionIndicator::UnclearCausalTrace,
            notes: "Uncertain about delegate actions",
          },
          DimensionScore {
            dimension: EvaluationDimension::DebriefCausalUtility,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Debrief helped understand outcome",
          },
          DimensionScore {
            dimension: EvaluationDimension::KeyboardFlow,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Easy typing",
          },
          DimensionScore {
            dimension: EvaluationDimension::NonColorSemantics,
            score_bp: 7_500,
            friction: CognitiveFrictionIndicator::None,
            notes: "Text tags fine",
          },
          DimensionScore {
            dimension: EvaluationDimension::ScreenReaderSuitability,
            score_bp: 7_000,
            friction: CognitiveFrictionIndicator::None,
            notes: "Plain text fine",
          },
        ],
      },
    ]
  }

  /// Executes a canonical benchmark dimension scenario by ID.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<DimensionScenarioExecutionResult, DimensionEvaluationError> {
    let scenario =
      Self::find_by_id(scenario_id).ok_or(DimensionEvaluationError::EmptyAssessmentList)?;
    let assessments = match scenario.scenario_id {
      "scenario-dimension-alpha-benchmark-v1" => Self::balanced_alpha_data(),
      "scenario-dimension-screen-reader-audit-v1" => Self::screen_reader_audit_data(),
      "scenario-dimension-novice-friction-v1" => Self::novice_friction_data(),
      _ => return Err(DimensionEvaluationError::EmptyAssessmentList),
    };

    let report = evaluate_dimension_assessments(&scenario.protocol, &assessments)?;

    let assessment_count_matches = report.assessment_count == scenario.expected_assessment_count;
    let overall_mean_matches = report.overall_mean_score_bp == scenario.expected_overall_mean_bp;
    let weakest_dimension_matches = report.weakest_dimension == scenario.expected_weakest_dimension;
    let strongest_dimension_matches =
      report.strongest_dimension == scenario.expected_strongest_dimension;
    let accessibility_qualification_matches =
      report.accessibility_dimensions_qualified == scenario.expected_accessibility_qualified;

    let all_expectations_met = assessment_count_matches
      && overall_mean_matches
      && weakest_dimension_matches
      && strongest_dimension_matches
      && accessibility_qualification_matches;

    Ok(DimensionScenarioExecutionResult {
      scenario_id: scenario.scenario_id,
      report,
      assessment_count_matches,
      overall_mean_matches,
      weakest_dimension_matches,
      strongest_dimension_matches,
      accessibility_qualification_matches,
      all_expectations_met,
    })
  }
}
