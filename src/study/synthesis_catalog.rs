//! Canonical benchmark scenarios for M10 human usability and accessibility alpha synthesis.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Registers 3 canonical synthesis scenarios:
//! 1. `scenario-alpha-synthesis-baseline-v1`: Balanced cohort, high completion/comprehension, verified remediations, full accessibility compliance -> `AlphaReady`.
//! 2. `scenario-alpha-synthesis-accessibility-gated-v1`: Access needs cohort surfaces friction, screen reader score below floor -> `BlockedByReadinessGates`.
//! 3. `scenario-alpha-synthesis-sampling-gap-v1`: Unaddressed blocker and pending remediations -> `BlockedByReadinessGates`.

use super::catalog::StudyProtocolCatalog;
use super::dimension_catalog::DimensionAssessmentCatalog;
use super::interaction::{
  ContrastMode, InteractionProfile, VerbosityLevel, audit_interaction_transcript,
};
use super::remediation_catalog::RemediationCatalog;
use super::sampling::{SamplingLimitsDeclaration, evaluate_participant_sampling};
use super::synthesis::{
  AlphaDisposition, AlphaEvidenceSynthesis, SynthesisEvaluationError, synthesize_alpha_evidence,
};

pub const M10_SYNTHESIS_CATALOG_SCHEMA_V1: &str = "m10-synthesis-catalog-v1";

/// Definition of a benchmark synthesis scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AlphaSynthesisScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub study_scenario_id: &'static str,
  pub dimension_scenario_id: &'static str,
  pub remediation_scenario_id: &'static str,
  pub interaction_profile: InteractionProfile,
  pub interaction_transcript_lines: &'static [&'static str],
  pub sampling_declaration: SamplingLimitsDeclaration,
  pub inferred_hypotheses: &'static [&'static str],
  pub expected_disposition: AlphaDisposition,
  pub expected_all_gates_passed: bool,
}

impl AlphaSynthesisScenarioDefinition {
  /// Execute the synthesis scenario and verify all expectations.
  pub fn execute(&self) -> Result<AlphaSynthesisExecutionResult, SynthesisEvaluationError> {
    let study_res = StudyProtocolCatalog::execute_scenario(self.study_scenario_id)
      .map_err(|_| SynthesisEvaluationError::EmptySynthesisId)?;
    let dim_res = DimensionAssessmentCatalog::execute_scenario(self.dimension_scenario_id)
      .map_err(|_| SynthesisEvaluationError::EmptySynthesisId)?;
    let rem_res = RemediationCatalog::execute_scenario(self.remediation_scenario_id)
      .map_err(|_| SynthesisEvaluationError::EmptySynthesisId)?;

    let interaction_report =
      audit_interaction_transcript(&self.interaction_profile, self.interaction_transcript_lines);

    let (sessions, _) = match self.study_scenario_id {
      "scenario-study-cohort-balanced-alpha-v1" => StudyProtocolCatalog::balanced_alpha_data(),
      "scenario-study-cohort-access-friction-v1" => StudyProtocolCatalog::access_barriers_data(),
      "scenario-study-cohort-mixed-novice-v1" => StudyProtocolCatalog::mixed_novice_friction_data(),
      _ => return Err(SynthesisEvaluationError::EmptySynthesisId),
    };

    let sampling_report = evaluate_participant_sampling(&self.sampling_declaration, &sessions)
      .map_err(|_| SynthesisEvaluationError::EmptySynthesisId)?;

    let synthesis = synthesize_alpha_evidence(
      self.scenario_id,
      study_res.report,
      dim_res.report,
      interaction_report,
      rem_res.report,
      sampling_report,
      self.inferred_hypotheses,
    )?;

    let disposition_matches = synthesis.disposition == self.expected_disposition;
    let gates_match = synthesis.gates.all_gates_passed() == self.expected_all_gates_passed;
    let all_expectations_met = disposition_matches && gates_match;

    Ok(AlphaSynthesisExecutionResult {
      scenario_id: self.scenario_id,
      synthesis,
      disposition_matches,
      gates_match,
      all_expectations_met,
    })
  }
}

/// Result of executing an alpha synthesis benchmark scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlphaSynthesisExecutionResult {
  pub scenario_id: &'static str,
  pub synthesis: AlphaEvidenceSynthesis,
  pub disposition_matches: bool,
  pub gates_match: bool,
  pub all_expectations_met: bool,
}

// ---------------------------------------------------------------------------
// Scenario 1: Alpha Baseline Synthesis (AlphaReady)
// ---------------------------------------------------------------------------

static SCENARIO_BASELINE_TRANSCRIPT_LINES: [&str; 6] = [
  "[OK] turn: 1 | actor: laner | status: open",
  "[OK] wave_pressure: 0 | position: NearTower",
  "[OK] available_actions: Stabilize, Contest, Recall, Yield",
  "[OK] action_submitted: Stabilize",
  "[OK] outcome: space-held | damage_dealt: 10",
  "[OK] debrief: objective space held through window",
];

static BASELINE_HYPOTHESES: [&str; 3] = [
  "Clear CLI prompt affordances reduce cognitive load for novice strategy players.",
  "Deterministic debriefs enable rapid causal attribution across both strategy and MOBA cohorts.",
  "Bracketed status markers provide unambiguous state awareness for screen reader users.",
];

pub const SCENARIO_ALPHA_SYNTHESIS_BASELINE: AlphaSynthesisScenarioDefinition =
  AlphaSynthesisScenarioDefinition {
    scenario_id: "scenario-alpha-synthesis-baseline-v1",
    title: "Alpha Baseline Synthesis — All Gates Passed",
    description: "Standard alpha study cohort with high completion, qualified accessibility, verified remediations, and complete sampling quotas.",
    study_scenario_id: "scenario-study-cohort-balanced-alpha-v1",
    dimension_scenario_id: "scenario-dimension-alpha-benchmark-v1",
    remediation_scenario_id: "scenario-remediation-alpha-baseline-v1",
    interaction_profile: InteractionProfile {
      profile_id: "profile-baseline-synthesis-v1",
      verbosity: VerbosityLevel::Standard,
      contrast_mode: ContrastMode::NoColor,
      keyboard_only: true,
      screen_reader_friendly: true,
    },
    interaction_transcript_lines: &SCENARIO_BASELINE_TRANSCRIPT_LINES,
    sampling_declaration: SamplingLimitsDeclaration::standard_alpha(),
    inferred_hypotheses: &BASELINE_HYPOTHESES,
    expected_disposition: AlphaDisposition::AlphaReady,
    expected_all_gates_passed: true,
  };

// ---------------------------------------------------------------------------
// Scenario 2: Accessibility Gated Synthesis (BlockedByReadinessGates)
// ---------------------------------------------------------------------------

static SCENARIO_ACCESS_GATED_TRANSCRIPT_LINES: [&str; 6] = [
  "[WARN] turn: 1 | actor: laner | status: open",
  "[WARN] wave_pressure: 0 | position: NearTower",
  "[WARN] available_actions: Stabilize, Contest, Recall, Yield",
  "[WARN] action_submitted: Contest",
  "[WARN] outcome: contested-trade | damage_dealt: 25",
  "[WARN] debrief: heavy trading occurred near tower",
];

static ACCESS_GATED_HYPOTHESES: [&str; 2] = [
  "Screen reader focus traps during contingency setup require structural command vocabulary simplification.",
  "High cognitive friction in screen reader navigation blocks accessibility qualification until remediation.",
];

pub const SCENARIO_ALPHA_SYNTHESIS_ACCESSIBILITY_GATED: AlphaSynthesisScenarioDefinition =
  AlphaSynthesisScenarioDefinition {
    scenario_id: "scenario-alpha-synthesis-accessibility-gated-v1",
    title: "Accessibility Gated Synthesis — Disqualified by Accessibility Blocker",
    description: "Access-needs cohort surfaces unresolved screen-reader blocker and low accessibility dimension score, failing readiness gates.",
    study_scenario_id: "scenario-study-cohort-access-friction-v1",
    dimension_scenario_id: "scenario-dimension-screen-reader-audit-v1",
    remediation_scenario_id: "scenario-remediation-accessibility-priority-v1",
    interaction_profile: InteractionProfile {
      profile_id: "profile-access-gated-synthesis-v1",
      verbosity: VerbosityLevel::Detailed,
      contrast_mode: ContrastMode::NoColor,
      keyboard_only: true,
      screen_reader_friendly: true,
    },
    interaction_transcript_lines: &SCENARIO_ACCESS_GATED_TRANSCRIPT_LINES,
    sampling_declaration: SamplingLimitsDeclaration::standard_alpha(),
    inferred_hypotheses: &ACCESS_GATED_HYPOTHESES,
    expected_disposition: AlphaDisposition::BlockedByReadinessGates,
    expected_all_gates_passed: false,
  };

// ---------------------------------------------------------------------------
// Scenario 3: Remediation Gap Synthesis (BlockedByReadinessGates)
// ---------------------------------------------------------------------------

static SCENARIO_REMEDIATION_GAP_TRANSCRIPT_LINES: [&str; 6] = [
  "[INFO] turn: 1 | actor: laner | status: open",
  "[INFO] wave_pressure: 2 | position: Center",
  "[INFO] available_actions: Stabilize, Contest, Recall, Yield",
  "[INFO] action_submitted: Recall",
  "[INFO] outcome: yielded-space | damage_dealt: 0",
  "[INFO] debrief: lane space yielded safely",
];

static REMEDIATION_GAP_HYPOTHESES: [&str; 2] = [
  "Novice players struggle with multi-actor fog-of-war without interactive onboarding tutorials.",
  "Pending remediation actions must be verified in regression before alpha readiness gate can pass.",
];

pub const SCENARIO_ALPHA_SYNTHESIS_REMEDIATION_GAP: AlphaSynthesisScenarioDefinition =
  AlphaSynthesisScenarioDefinition {
    scenario_id: "scenario-alpha-synthesis-sampling-gap-v1",
    title: "Remediation Gap Synthesis — Blocked by Incomplete Remediation",
    description: "Mixed novice cohort with unresolved study blockers and pending remediation actions failing the readiness gate.",
    study_scenario_id: "scenario-study-cohort-mixed-novice-v1",
    dimension_scenario_id: "scenario-dimension-novice-friction-v1",
    remediation_scenario_id: "scenario-remediation-mixed-progress-v1",
    interaction_profile: InteractionProfile {
      profile_id: "profile-remediation-gap-synthesis-v1",
      verbosity: VerbosityLevel::Standard,
      contrast_mode: ContrastMode::NoColor,
      keyboard_only: false,
      screen_reader_friendly: false,
    },
    interaction_transcript_lines: &SCENARIO_REMEDIATION_GAP_TRANSCRIPT_LINES,
    sampling_declaration: SamplingLimitsDeclaration::standard_alpha(),
    inferred_hypotheses: &REMEDIATION_GAP_HYPOTHESES,
    expected_disposition: AlphaDisposition::BlockedByReadinessGates,
    expected_all_gates_passed: false,
  };

// ---------------------------------------------------------------------------
// Catalog Interface
// ---------------------------------------------------------------------------

/// Catalog of canonical alpha synthesis scenarios for M10.
pub struct AlphaSynthesisCatalog;

impl AlphaSynthesisCatalog {
  pub const ALL: [AlphaSynthesisScenarioDefinition; 3] = [
    SCENARIO_ALPHA_SYNTHESIS_BASELINE,
    SCENARIO_ALPHA_SYNTHESIS_ACCESSIBILITY_GATED,
    SCENARIO_ALPHA_SYNTHESIS_REMEDIATION_GAP,
  ];

  /// Looks up a scenario by ID.
  pub fn find_by_id(scenario_id: &str) -> Option<AlphaSynthesisScenarioDefinition> {
    Self::ALL.into_iter().find(|s| s.scenario_id == scenario_id)
  }

  /// Executes a single scenario by ID and verifies expectations.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<AlphaSynthesisExecutionResult, SynthesisEvaluationError> {
    let scenario =
      Self::find_by_id(scenario_id).ok_or(SynthesisEvaluationError::EmptySynthesisId)?;
    scenario.execute()
  }

  /// Executes all registered benchmark scenarios and verifies expectations.
  pub fn execute_all() -> Result<Vec<AlphaSynthesisExecutionResult>, SynthesisEvaluationError> {
    let mut results = Vec::with_capacity(Self::ALL.len());
    for scenario in &Self::ALL {
      results.push(scenario.execute()?);
    }
    Ok(results)
  }
}
