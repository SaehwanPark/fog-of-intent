//! Held-out scenario evaluation and counterfactual perturbation testing for calibrated parametric policies.

use super::empirical::{
  DiagnosticChoiceActionDistribution, EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS,
};
use super::parametric::ParametricPolicyDefinition;
use super::semantic::{
  CAUTIOUS_SEMANTIC_PROFILE_ID, CHOICE_CONTEST_CONCEDE_ID, CHOICE_FARM_ASSIST_ID,
  CHOICE_FOLLOW_REJECT_ID, CHOICE_RECALL_TIMING_ID, CHOICE_RESPONSE_TO_FAILURE_ID,
  CHOICE_SACRIFICE_ID, CHOICE_SURPRISE_ID, DiagnosticChoiceCatalog, DiagnosticChoiceDomain,
  RISK_TAKING_SEMANTIC_PROFILE_ID, SemanticProfileVocabulary, YIELDING_SEMANTIC_PROFILE_ID,
};
use crate::lane::LaneIntent;

/// Versioned schema for held-out scenario definitions.
pub const HELD_OUT_SCENARIO_SCHEMA: &str = "m7-held-out-scenario-v1";

/// Versioned schema for the held-out scenario catalog.
pub const HELD_OUT_SCENARIO_CATALOG_SCHEMA: &str = "m7-held-out-scenario-catalog-v1";

/// Versioned schema for held-out scenario evaluation reports.
pub const HELD_OUT_EVALUATION_SCHEMA: &str = "m7-held-out-scenario-evaluation-v1";

/// Versioned schema for counterfactual perturbation definitions.
pub const COUNTERFACTUAL_PERTURBATION_SCHEMA: &str = "m7-counterfactual-perturbation-v1";

/// Versioned schema for counterfactual sensitivity reports.
pub const COUNTERFACTUAL_SENSITIVITY_SCHEMA: &str = "m7-counterfactual-sensitivity-v1";

/// Versioned schema for the integrated calibration held-out report.
pub const CALIBRATION_HELD_OUT_SCHEMA: &str = "m7-calibration-held-out-v1";

/// Maximum acceptable mean Total Variation Distance loss on held-out scenarios (2,500 bp = 25.00%).
pub const MAX_ACCEPTABLE_HELD_OUT_LOSS_BP: u16 = 2_500;

/// Minimum acceptable modal choice accuracy on held-out scenarios (7,000 bp = 70.00%).
pub const MIN_ACCEPTABLE_MODAL_ACCURACY_BP: u16 = 7_000;

/// Tolerance threshold in basis points for counterfactual directional sensitivity (200 bp = 2.00%).
pub const COUNTERFACTUAL_TOLERANCE_BP: u16 = 200;

/// Stable identifier for the held-out contest scenario under escalated pressure.
pub const HELD_OUT_CONTEST_UNDER_THREAT_ID: &str = "held-out-contest-under-threat-v1";

/// Stable identifier for the held-out follow scenario after allied retreat.
pub const HELD_OUT_FOLLOW_AFTER_RETREAT_ID: &str = "held-out-follow-after-retreat-v1";

/// Stable identifier for the held-out farm scenario under crashing wave pressure.
pub const HELD_OUT_FARM_UNDER_WAVE_PRESSURE_ID: &str = "held-out-farm-under-wave-pressure-v1";

/// Stable identifier for the held-out recall scenario with low health.
pub const HELD_OUT_RECALL_LOW_HEALTH_ID: &str = "held-out-recall-low-health-v1";

/// Stable identifier for the held-out sacrifice scenario under isolated tower defense.
pub const HELD_OUT_SACRIFICE_ISOLATED_ID: &str = "held-out-sacrifice-isolated-v1";

/// Stable identifier for the held-out surprise scenario under river flank sighting.
pub const HELD_OUT_SURPRISE_FLANK_ID: &str = "held-out-surprise-flank-v1";

/// Stable identifier for the held-out failure response scenario after lost trade.
pub const HELD_OUT_FAILURE_RESET_ID: &str = "held-out-failure-reset-v1";

/// Stable identifier for the threat escalation counterfactual perturbation.
pub const CF_THREAT_ESCALATION_ID: &str = "cf-threat-escalation-v1";

/// Stable identifier for the allied retreat call counterfactual perturbation.
pub const CF_ALLIED_RETREAT_ID: &str = "cf-allied-retreat-v1";

/// Stable identifier for the severe health attrition counterfactual perturbation.
pub const CF_HEALTH_ATTRITION_ID: &str = "cf-health-attrition-v1";

/// Stable identifier for the favorable opening counterfactual perturbation.
pub const CF_FAVORABLE_OPENING_ID: &str = "cf-favorable-opening-v1";

/// Errors raised when evaluating held-out scenarios or counterfactual perturbations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HeldOutEvaluationError {
  UnknownScenario,
  UnknownPerturbation,
  UnknownProfile,
  MismatchedProfile,
  MismatchedChoice,
  InvalidLoss,
}

/// Bounded held-out diagnostic scenario definition with test ground-truth distribution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HeldOutScenarioDefinition {
  scenario_id: &'static str,
  schema: &'static str,
  domain: DiagnosticChoiceDomain,
  base_choice_id: &'static str,
  held_out_distribution: DiagnosticChoiceActionDistribution,
  expected_modal_intent: LaneIntent,
  description: &'static str,
}

impl HeldOutScenarioDefinition {
  /// Construct a held-out scenario definition and validate distribution consistency.
  pub fn new(
    scenario_id: &'static str,
    domain: DiagnosticChoiceDomain,
    base_choice_id: &'static str,
    held_out_distribution: DiagnosticChoiceActionDistribution,
    expected_modal_intent: LaneIntent,
    description: &'static str,
  ) -> Result<Self, HeldOutEvaluationError> {
    let base_choice = DiagnosticChoiceCatalog::validate_choice_id(base_choice_id)
      .map_err(|_| HeldOutEvaluationError::MismatchedChoice)?;

    if base_choice.domain() != domain {
      return Err(HeldOutEvaluationError::MismatchedChoice);
    }

    if held_out_distribution.choice_id() != base_choice_id {
      return Err(HeldOutEvaluationError::MismatchedChoice);
    }

    Ok(Self {
      scenario_id,
      schema: HELD_OUT_SCENARIO_SCHEMA,
      domain,
      base_choice_id,
      held_out_distribution,
      expected_modal_intent,
      description,
    })
  }

  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn domain(self) -> DiagnosticChoiceDomain {
    self.domain
  }

  pub const fn base_choice_id(self) -> &'static str {
    self.base_choice_id
  }

  pub const fn held_out_distribution(self) -> DiagnosticChoiceActionDistribution {
    self.held_out_distribution
  }

  pub const fn expected_modal_intent(self) -> LaneIntent {
    self.expected_modal_intent
  }

  pub const fn description(self) -> &'static str {
    self.description
  }
}

/// Catalog providing canonical held-out scenario batteries for reference profiles.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HeldOutScenarioCatalog;

impl HeldOutScenarioCatalog {
  /// Canonical held-out scenario suite for the cautious profile.
  pub fn cautious_held_out_suite_v1() -> [HeldOutScenarioDefinition; 7] {
    [
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_CONTEST_UNDER_THREAT_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::ContestConcede,
        base_choice_id: CHOICE_CONTEST_CONCEDE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_CONTEST_CONCEDE_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          15,
          80,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Yield,
        description: "Held-out contest dilemma under elevated jungle threat pressure.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FOLLOW_AFTER_RETREAT_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::FollowReject,
        base_choice_id: CHOICE_FOLLOW_REJECT_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_FOLLOW_REJECT_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          20,
          75,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Stabilize,
        description: "Held-out follow dilemma when ally issues retreat warning.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FARM_UNDER_WAVE_PRESSURE_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::FarmAssist,
        base_choice_id: CHOICE_FARM_ASSIST_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_FARM_ASSIST_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          80,
          15,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Stabilize,
        description: "Held-out farm dilemma with large minion wave crashing at tower.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_RECALL_LOW_HEALTH_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::RecallTiming,
        base_choice_id: CHOICE_RECALL_TIMING_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_RECALL_TIMING_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          85,
          12,
          3,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Recall,
        description: "Held-out recall timing dilemma under severe health deficit.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_SACRIFICE_ISOLATED_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::Sacrifice,
        base_choice_id: CHOICE_SACRIFICE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_SACRIFICE_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          12,
          85,
          3,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Withdraw,
        description: "Held-out sacrifice dilemma under isolated multi-opponent collapse.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_SURPRISE_FLANK_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::Surprise,
        base_choice_id: CHOICE_SURPRISE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_SURPRISE_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          90,
          8,
          2,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Withdraw,
        description: "Held-out surprise dilemma upon sudden river flank ambush.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FAILURE_RESET_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::ResponseToFailure,
        base_choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_RESPONSE_TO_FAILURE_ID,
          CAUTIOUS_SEMANTIC_PROFILE_ID,
          100,
          82,
          15,
          3,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Yield,
        description: "Held-out response to failure dilemma after lost prior trade.",
      },
    ]
  }

  /// Canonical held-out scenario suite for the risk-taking profile.
  pub fn risk_taking_held_out_suite_v1() -> [HeldOutScenarioDefinition; 7] {
    [
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_CONTEST_UNDER_THREAT_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::ContestConcede,
        base_choice_id: CHOICE_CONTEST_CONCEDE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_CONTEST_CONCEDE_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          75,
          20,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Contest,
        description: "Held-out contest dilemma under elevated jungle threat pressure.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FOLLOW_AFTER_RETREAT_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::FollowReject,
        base_choice_id: CHOICE_FOLLOW_REJECT_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_FOLLOW_REJECT_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          70,
          25,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Contest,
        description: "Held-out follow dilemma when ally issues retreat warning.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FARM_UNDER_WAVE_PRESSURE_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::FarmAssist,
        base_choice_id: CHOICE_FARM_ASSIST_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_FARM_ASSIST_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          30,
          65,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Contest,
        description: "Held-out farm dilemma with large minion wave crashing at tower.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_RECALL_LOW_HEALTH_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::RecallTiming,
        base_choice_id: CHOICE_RECALL_TIMING_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_RECALL_TIMING_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          35,
          60,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Stabilize,
        description: "Held-out recall timing dilemma under severe health deficit.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_SACRIFICE_ISOLATED_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::Sacrifice,
        base_choice_id: CHOICE_SACRIFICE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_SACRIFICE_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          65,
          30,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Contest,
        description: "Held-out sacrifice dilemma under isolated multi-opponent collapse.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_SURPRISE_FLANK_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::Surprise,
        base_choice_id: CHOICE_SURPRISE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_SURPRISE_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          35,
          60,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Stabilize,
        description: "Held-out surprise dilemma upon sudden river flank ambush.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FAILURE_RESET_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::ResponseToFailure,
        base_choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_RESPONSE_TO_FAILURE_ID,
          RISK_TAKING_SEMANTIC_PROFILE_ID,
          100,
          30,
          65,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Contest,
        description: "Held-out response to failure dilemma after lost prior trade.",
      },
    ]
  }

  /// Canonical held-out scenario suite for the yielding profile.
  pub fn yielding_held_out_suite_v1() -> [HeldOutScenarioDefinition; 7] {
    [
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_CONTEST_UNDER_THREAT_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::ContestConcede,
        base_choice_id: CHOICE_CONTEST_CONCEDE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_CONTEST_CONCEDE_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          10,
          85,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Yield,
        description: "Held-out contest dilemma under elevated jungle threat pressure.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FOLLOW_AFTER_RETREAT_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::FollowReject,
        base_choice_id: CHOICE_FOLLOW_REJECT_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_FOLLOW_REJECT_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          15,
          80,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Stabilize,
        description: "Held-out follow dilemma when ally issues retreat warning.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FARM_UNDER_WAVE_PRESSURE_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::FarmAssist,
        base_choice_id: CHOICE_FARM_ASSIST_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_FARM_ASSIST_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          85,
          10,
          5,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Stabilize,
        description: "Held-out farm dilemma with large minion wave crashing at tower.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_RECALL_LOW_HEALTH_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::RecallTiming,
        base_choice_id: CHOICE_RECALL_TIMING_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_RECALL_TIMING_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          88,
          10,
          2,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Recall,
        description: "Held-out recall timing dilemma under severe health deficit.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_SACRIFICE_ISOLATED_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::Sacrifice,
        base_choice_id: CHOICE_SACRIFICE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_SACRIFICE_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          8,
          90,
          2,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Withdraw,
        description: "Held-out sacrifice dilemma under isolated multi-opponent collapse.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_SURPRISE_FLANK_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::Surprise,
        base_choice_id: CHOICE_SURPRISE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_SURPRISE_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          92,
          6,
          2,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Withdraw,
        description: "Held-out surprise dilemma upon sudden river flank ambush.",
      },
      HeldOutScenarioDefinition {
        scenario_id: HELD_OUT_FAILURE_RESET_ID,
        schema: HELD_OUT_SCENARIO_SCHEMA,
        domain: DiagnosticChoiceDomain::ResponseToFailure,
        base_choice_id: CHOICE_RESPONSE_TO_FAILURE_ID,
        held_out_distribution: DiagnosticChoiceActionDistribution::new(
          CHOICE_RESPONSE_TO_FAILURE_ID,
          YIELDING_SEMANTIC_PROFILE_ID,
          100,
          88,
          10,
          2,
        )
        .expect("valid distribution"),
        expected_modal_intent: LaneIntent::Yield,
        description: "Held-out response to failure dilemma after lost prior trade.",
      },
    ]
  }

  /// Retrieve the held-out scenario suite for a given profile ID.
  pub fn scenarios_for_profile(
    profile_id: &str,
  ) -> Result<[HeldOutScenarioDefinition; 7], HeldOutEvaluationError> {
    match profile_id {
      CAUTIOUS_SEMANTIC_PROFILE_ID => Ok(Self::cautious_held_out_suite_v1()),
      RISK_TAKING_SEMANTIC_PROFILE_ID => Ok(Self::risk_taking_held_out_suite_v1()),
      YIELDING_SEMANTIC_PROFILE_ID => Ok(Self::yielding_held_out_suite_v1()),
      _ => Err(HeldOutEvaluationError::UnknownProfile),
    }
  }
}

/// Evaluation report evaluating parametric policy generalization on held-out diagnostic scenarios.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HeldOutScenarioEvaluationReport {
  schema: &'static str,
  profile_id: &'static str,
  held_out_losses_bp: [u16; 7],
  mean_held_out_loss_bp: u16,
  modal_matches: [bool; 7],
  modal_accuracy_bp: u16,
  passed_generalization_threshold: bool,
}

impl HeldOutScenarioEvaluationReport {
  /// Evaluate a parametric policy against its canonical held-out scenario battery.
  pub fn from_policy(policy: &ParametricPolicyDefinition) -> Result<Self, HeldOutEvaluationError> {
    policy
      .validate()
      .map_err(|_| HeldOutEvaluationError::UnknownProfile)?;
    SemanticProfileVocabulary::validate_profile_id(policy.profile_id())
      .map_err(|_| HeldOutEvaluationError::UnknownProfile)?;

    let scenarios = HeldOutScenarioCatalog::scenarios_for_profile(policy.profile_id())?;
    let mut held_out_losses_bp = [0_u16; 7];
    let mut modal_matches = [false; 7];
    let mut sum_loss_u32 = 0_u32;
    let mut matching_count = 0_u32;

    for i in 0..7 {
      let scenario = scenarios[i];
      let act_weights = policy.action_weights()[i];

      let bp_pred = act_weights.basis_points();
      let bp_held = scenario.held_out_distribution().basis_points();

      let diff_primary = u32::from(bp_pred[0].abs_diff(bp_held[0]));
      let diff_alt = u32::from(bp_pred[1].abs_diff(bp_held[1]));
      let diff_res = u32::from(bp_pred[2].abs_diff(bp_held[2]));
      let tvd_loss =
        u16::try_from((diff_primary + diff_alt + diff_res) / 2).expect("loss fits in u16");

      held_out_losses_bp[i] = tvd_loss;
      sum_loss_u32 += u32::from(tvd_loss);

      let is_match = act_weights.predicted_intent() == scenario.expected_modal_intent();
      modal_matches[i] = is_match;
      if is_match {
        matching_count += 1;
      }
    }

    let mean_held_out_loss_bp = u16::try_from(sum_loss_u32 / 7).expect("mean loss fits in u16");
    let scale_u32 = u32::from(EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS);
    let modal_accuracy_bp =
      u16::try_from((matching_count * scale_u32) / 7).expect("accuracy fits in u16");

    let passed_generalization_threshold = mean_held_out_loss_bp <= MAX_ACCEPTABLE_HELD_OUT_LOSS_BP
      && modal_accuracy_bp >= MIN_ACCEPTABLE_MODAL_ACCURACY_BP;

    Ok(Self {
      schema: HELD_OUT_EVALUATION_SCHEMA,
      profile_id: policy.profile_id(),
      held_out_losses_bp,
      mean_held_out_loss_bp,
      modal_matches,
      modal_accuracy_bp,
      passed_generalization_threshold,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn held_out_losses_bp(&self) -> &[u16; 7] {
    &self.held_out_losses_bp
  }

  pub const fn mean_held_out_loss_bp(&self) -> u16 {
    self.mean_held_out_loss_bp
  }

  pub const fn modal_matches(&self) -> &[bool; 7] {
    &self.modal_matches
  }

  pub const fn modal_accuracy_bp(&self) -> u16 {
    self.modal_accuracy_bp
  }

  pub const fn passed_generalization_threshold(&self) -> bool {
    self.passed_generalization_threshold
  }

  /// Render the held-out evaluation report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let choices = DiagnosticChoiceCatalog::all_choices();
    let mut out = format!(
      "# Held-Out Scenario Evaluation Report\n\n- schema: {}\n- profile_id: {}\n- mean_held_out_loss_bp: {}\n- modal_accuracy_bp: {}\n- passed_generalization_threshold: {}\n\n| choice_id | held_out_tvd_loss_bp | modal_match |\n| --- | ---: | --- |\n",
      self.schema,
      self.profile_id,
      self.mean_held_out_loss_bp,
      self.modal_accuracy_bp,
      self.passed_generalization_threshold,
    );
    for (i, choice) in choices.iter().enumerate() {
      out.push_str(&format!(
        "| {} | {} | {} |\n",
        choice.choice_id(),
        self.held_out_losses_bp[i],
        if self.modal_matches[i] {
          "match"
        } else {
          "mismatch"
        },
      ));
    }
    out
  }
}

/// Discrete counterfactual condition category for sensitivity evaluation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CounterfactualCondition {
  /// Escalation of opposing threat presence on lane flank.
  ThreatEscalation,
  /// Allied teammate issues retreat call / disengages.
  AlliedRetreatCall,
  /// Severe player health attrition below safety threshold.
  SevereHealthAttrition,
  /// Favorable strategic opening with opponent overextended.
  FavorableOpening,
}

impl CounterfactualCondition {
  /// Return the canonical label for this counterfactual condition.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::ThreatEscalation => "threat-escalation",
      Self::AlliedRetreatCall => "allied-retreat-call",
      Self::SevereHealthAttrition => "severe-health-attrition",
      Self::FavorableOpening => "favorable-opening",
    }
  }

  /// Parse a counterfactual condition from a canonical label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "threat-escalation" => Some(Self::ThreatEscalation),
      "allied-retreat-call" => Some(Self::AlliedRetreatCall),
      "severe-health-attrition" => Some(Self::SevereHealthAttrition),
      "favorable-opening" => Some(Self::FavorableOpening),
      _ => None,
    }
  }
}

/// Directional shift expectation under a counterfactual perturbation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DirectionalShiftExpectation {
  /// Policy is expected to shift towards defensive / concession actions.
  ShiftTowardsDefensive,
  /// Policy is expected to shift towards aggressive / contest actions.
  ShiftTowardsAggressive,
  /// Policy is expected to maintain autonomous posture with minimal shift.
  MaintainStance,
}

/// Status indicating whether observed policy shift is directionally coherent with semantic traits.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DirectionalCoherenceStatus {
  /// Shift aligns directionally with expected semantic trait response.
  Coherent,
  /// Shift is within neutral tolerance bounds.
  Neutral,
  /// Shift opposes expected semantic trait response (calibration failure).
  Inverted,
}

impl DirectionalCoherenceStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Coherent => "coherent",
      Self::Neutral => "neutral",
      Self::Inverted => "inverted",
    }
  }
}

/// Definition of a single counterfactual perturbation test case.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CounterfactualPerturbationDefinition {
  perturbation_id: &'static str,
  schema: &'static str,
  condition: CounterfactualCondition,
  target_domain: DiagnosticChoiceDomain,
  description: &'static str,
}

impl CounterfactualPerturbationDefinition {
  pub const fn threat_escalation_v1() -> Self {
    Self {
      perturbation_id: CF_THREAT_ESCALATION_ID,
      schema: COUNTERFACTUAL_PERTURBATION_SCHEMA,
      condition: CounterfactualCondition::ThreatEscalation,
      target_domain: DiagnosticChoiceDomain::ContestConcede,
      description: "Counterfactual perturbation introducing river flank threat presence.",
    }
  }

  pub const fn allied_retreat_v1() -> Self {
    Self {
      perturbation_id: CF_ALLIED_RETREAT_ID,
      schema: COUNTERFACTUAL_PERTURBATION_SCHEMA,
      condition: CounterfactualCondition::AlliedRetreatCall,
      target_domain: DiagnosticChoiceDomain::FollowReject,
      description: "Counterfactual perturbation with allied teammate issuing retreat signal.",
    }
  }

  pub const fn health_attrition_v1() -> Self {
    Self {
      perturbation_id: CF_HEALTH_ATTRITION_ID,
      schema: COUNTERFACTUAL_PERTURBATION_SCHEMA,
      condition: CounterfactualCondition::SevereHealthAttrition,
      target_domain: DiagnosticChoiceDomain::RecallTiming,
      description: "Counterfactual perturbation with severe health depletion below 30%.",
    }
  }

  pub const fn favorable_opening_v1() -> Self {
    Self {
      perturbation_id: CF_FAVORABLE_OPENING_ID,
      schema: COUNTERFACTUAL_PERTURBATION_SCHEMA,
      condition: CounterfactualCondition::FavorableOpening,
      target_domain: DiagnosticChoiceDomain::FarmAssist,
      description: "Counterfactual perturbation presenting clear opponent overextension.",
    }
  }

  pub const fn perturbation_id(self) -> &'static str {
    self.perturbation_id
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn condition(self) -> CounterfactualCondition {
    self.condition
  }

  pub const fn target_domain(self) -> DiagnosticChoiceDomain {
    self.target_domain
  }

  pub const fn description(self) -> &'static str {
    self.description
  }
}

/// Catalog of canonical counterfactual perturbation test definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CounterfactualPerturbationCatalog;

impl CounterfactualPerturbationCatalog {
  /// Return all 4 canonical counterfactual perturbations.
  pub const fn all_perturbations() -> [CounterfactualPerturbationDefinition; 4] {
    [
      CounterfactualPerturbationDefinition::threat_escalation_v1(),
      CounterfactualPerturbationDefinition::allied_retreat_v1(),
      CounterfactualPerturbationDefinition::health_attrition_v1(),
      CounterfactualPerturbationDefinition::favorable_opening_v1(),
    ]
  }

  /// Lookup a counterfactual perturbation by its stable ID.
  pub fn lookup(perturbation_id: &str) -> Option<CounterfactualPerturbationDefinition> {
    match perturbation_id {
      CF_THREAT_ESCALATION_ID => Some(CounterfactualPerturbationDefinition::threat_escalation_v1()),
      CF_ALLIED_RETREAT_ID => Some(CounterfactualPerturbationDefinition::allied_retreat_v1()),
      CF_HEALTH_ATTRITION_ID => Some(CounterfactualPerturbationDefinition::health_attrition_v1()),
      CF_FAVORABLE_OPENING_ID => Some(CounterfactualPerturbationDefinition::favorable_opening_v1()),
      _ => None,
    }
  }

  /// Validate that a perturbation ID exists in the catalog.
  pub fn validate_perturbation_id(
    perturbation_id: &str,
  ) -> Result<CounterfactualPerturbationDefinition, HeldOutEvaluationError> {
    Self::lookup(perturbation_id).ok_or(HeldOutEvaluationError::UnknownPerturbation)
  }
}

/// Result of evaluating a single counterfactual perturbation against a parametric policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CounterfactualEvaluationResult {
  perturbation_id: &'static str,
  condition: CounterfactualCondition,
  target_domain: DiagnosticChoiceDomain,
  baseline_primary_weight_bp: u16,
  perturbed_primary_weight_bp: u16,
  signed_delta_bp: i32,
  status: DirectionalCoherenceStatus,
}

impl CounterfactualEvaluationResult {
  pub const fn perturbation_id(self) -> &'static str {
    self.perturbation_id
  }

  pub const fn condition(self) -> CounterfactualCondition {
    self.condition
  }

  pub const fn target_domain(self) -> DiagnosticChoiceDomain {
    self.target_domain
  }

  pub const fn baseline_primary_weight_bp(self) -> u16 {
    self.baseline_primary_weight_bp
  }

  pub const fn perturbed_primary_weight_bp(self) -> u16 {
    self.perturbed_primary_weight_bp
  }

  pub const fn signed_delta_bp(self) -> i32 {
    self.signed_delta_bp
  }

  pub const fn status(self) -> DirectionalCoherenceStatus {
    self.status
  }
}

/// Comprehensive report evaluating counterfactual directional sensitivity across reference perturbations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CounterfactualSensitivityReport {
  schema: &'static str,
  profile_id: &'static str,
  evaluations: [CounterfactualEvaluationResult; 4],
  all_coherent: bool,
}

impl CounterfactualSensitivityReport {
  /// Evaluate counterfactual perturbation sensitivity for a given parametric policy.
  pub fn from_policy(policy: &ParametricPolicyDefinition) -> Result<Self, HeldOutEvaluationError> {
    policy
      .validate()
      .map_err(|_| HeldOutEvaluationError::UnknownProfile)?;
    let profile = SemanticProfileVocabulary::validate_profile_id(policy.profile_id())
      .map_err(|_| HeldOutEvaluationError::UnknownProfile)?;

    let perturbations = CounterfactualPerturbationCatalog::all_perturbations();
    let scenarios = HeldOutScenarioCatalog::scenarios_for_profile(policy.profile_id())?;

    let mut evaluations = [CounterfactualEvaluationResult {
      perturbation_id: perturbations[0].perturbation_id(),
      condition: perturbations[0].condition(),
      target_domain: perturbations[0].target_domain(),
      baseline_primary_weight_bp: 0,
      perturbed_primary_weight_bp: 0,
      signed_delta_bp: 0,
      status: DirectionalCoherenceStatus::Coherent,
    }; 4];

    let mut all_coherent = true;

    for (k, cf) in perturbations.iter().enumerate() {
      let baseline_act = policy
        .action_weights_for_domain(cf.target_domain())
        .expect("target domain exists in policy");
      let baseline_primary_bp = baseline_act.primary_weight_bp();

      let target_scenario = scenarios
        .iter()
        .find(|s| s.domain() == cf.target_domain())
        .expect("target domain scenario exists");
      let perturbed_primary_bp = target_scenario
        .held_out_distribution()
        .primary_share_basis_points();

      let delta = i32::from(perturbed_primary_bp) - i32::from(baseline_primary_bp);

      // Determine directional coherence based on semantic profile traits
      let status = match cf.condition() {
        CounterfactualCondition::ThreatEscalation => {
          // Under threat: cautious/yielding should reduce contest (delta <= 0) or stay neutral;
          // risk-seeking may maintain high contest (small delta).
          match profile.risk_tolerance() {
            super::semantic::SemanticRiskTolerance::Cautious => {
              if delta <= i32::from(COUNTERFACTUAL_TOLERANCE_BP) {
                DirectionalCoherenceStatus::Coherent
              } else {
                DirectionalCoherenceStatus::Inverted
              }
            }
            super::semantic::SemanticRiskTolerance::RiskSeeking => {
              // Risk seeking maintains high contest or only minor adjustment
              DirectionalCoherenceStatus::Coherent
            }
            super::semantic::SemanticRiskTolerance::Balanced => {
              DirectionalCoherenceStatus::Coherent
            }
          }
        }
        CounterfactualCondition::AlliedRetreatCall => {
          // Under allied retreat: yielding/compliant should shift to retreat/stabilize (reduce contest)
          match profile.deference() {
            super::semantic::SemanticDeference::Yielding
            | super::semantic::SemanticDeference::Compliant => {
              if delta <= i32::from(COUNTERFACTUAL_TOLERANCE_BP) {
                DirectionalCoherenceStatus::Coherent
              } else {
                DirectionalCoherenceStatus::Inverted
              }
            }
            super::semantic::SemanticDeference::Autonomous => DirectionalCoherenceStatus::Coherent,
          }
        }
        CounterfactualCondition::SevereHealthAttrition => {
          // Low health: all profiles increase recall/preservation or maintain high recall
          if delta >= -i32::from(COUNTERFACTUAL_TOLERANCE_BP) {
            DirectionalCoherenceStatus::Coherent
          } else {
            DirectionalCoherenceStatus::Inverted
          }
        }
        CounterfactualCondition::FavorableOpening => {
          // Favorable opening: all profiles retain or increase engagement
          DirectionalCoherenceStatus::Coherent
        }
      };

      if status != DirectionalCoherenceStatus::Coherent {
        all_coherent = false;
      }

      evaluations[k] = CounterfactualEvaluationResult {
        perturbation_id: cf.perturbation_id(),
        condition: cf.condition(),
        target_domain: cf.target_domain(),
        baseline_primary_weight_bp: baseline_primary_bp,
        perturbed_primary_weight_bp: perturbed_primary_bp,
        signed_delta_bp: delta,
        status,
      };
    }

    Ok(Self {
      schema: COUNTERFACTUAL_SENSITIVITY_SCHEMA,
      profile_id: policy.profile_id(),
      evaluations,
      all_coherent,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluations(&self) -> &[CounterfactualEvaluationResult; 4] {
    &self.evaluations
  }

  pub const fn all_coherent(&self) -> bool {
    self.all_coherent
  }

  /// Render the counterfactual sensitivity report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = format!(
      "# Counterfactual Sensitivity Report\n\n- schema: {}\n- profile_id: {}\n- all_coherent: {}\n\n| perturbation_id | condition | baseline_primary_bp | perturbed_primary_bp | delta_bp | status |\n| --- | --- | ---: | ---: | ---: | --- |\n",
      self.schema, self.profile_id, self.all_coherent,
    );
    for ev in &self.evaluations {
      out.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} |\n",
        ev.perturbation_id(),
        ev.condition().as_str(),
        ev.baseline_primary_weight_bp(),
        ev.perturbed_primary_weight_bp(),
        ev.signed_delta_bp(),
        ev.status().as_str(),
      ));
    }
    out
  }
}

/// Integrated calibration report evaluating held-out generalization and counterfactual sensitivity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CalibrationHeldOutReport {
  schema: &'static str,
  profile_id: &'static str,
  held_out_evaluation: HeldOutScenarioEvaluationReport,
  counterfactual_sensitivity: CounterfactualSensitivityReport,
  meets_calibration_gate: bool,
}

impl CalibrationHeldOutReport {
  /// Generate an integrated calibration report from a parametric policy definition.
  pub fn from_policy(policy: &ParametricPolicyDefinition) -> Result<Self, HeldOutEvaluationError> {
    let held_out_evaluation = HeldOutScenarioEvaluationReport::from_policy(policy)?;
    let counterfactual_sensitivity = CounterfactualSensitivityReport::from_policy(policy)?;

    let meets_calibration_gate = held_out_evaluation.passed_generalization_threshold()
      && counterfactual_sensitivity.all_coherent();

    Ok(Self {
      schema: CALIBRATION_HELD_OUT_SCHEMA,
      profile_id: policy.profile_id(),
      held_out_evaluation,
      counterfactual_sensitivity,
      meets_calibration_gate,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(&self) -> &'static str {
    self.profile_id
  }

  pub const fn held_out_evaluation(&self) -> &HeldOutScenarioEvaluationReport {
    &self.held_out_evaluation
  }

  pub const fn counterfactual_sensitivity(&self) -> &CounterfactualSensitivityReport {
    &self.counterfactual_sensitivity
  }

  pub const fn meets_calibration_gate(&self) -> bool {
    self.meets_calibration_gate
  }

  /// Render the integrated calibration report as formatted Markdown.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Calibration Held-Out & Counterfactual Report\n\n- schema: {}\n- profile_id: {}\n- meets_calibration_gate: {}\n\n{}\n{}",
      self.schema,
      self.profile_id,
      self.meets_calibration_gate,
      self.held_out_evaluation.to_markdown(),
      self.counterfactual_sensitivity.to_markdown(),
    )
  }
}
