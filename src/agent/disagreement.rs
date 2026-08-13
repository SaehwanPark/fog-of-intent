//! Evaluation contract for strategic legitimacy of disagreement.
//!
//! In Fog of Intent, strategic play preserves bounded rationality and actor autonomy:
//! shot-callers and peer leaders express intent and directives, but autonomous teammates
//! evaluate incoming calls against local observation, survival limits, and resource constraints.
//!
//! When an actor dissents from a directive, that decision is not mere noise or disobedience;
//! it may be *strategically legitimate* when compliance would have resulted in catastrophic
//! loss (e.g., a wipe caused by overextending with low health into an enemy ambush).
//!
//! This module provides:
//! 1. `DisagreementLegitimacyClassification`: Categorizes dissent into `LegitimateDissent`,
//!    `ConstructiveAlternative`, and `UnjustifiedInsubordination`.
//! 2. `DisagreementLegitimacyEvaluation`: Quantifies the counterfactual value delta in integer
//!    basis points ($[-10,000..=10,000]$ bp) comparing actual dissenting trajectory against
//!    blind compliance.
//! 3. `TeamDisagreementEvaluator`: Deterministic evaluator analyzing local conditions,
//!    dissent reasons, directive risk, and counterfactual survival outcomes.

use core::fmt;

use crate::agent::communication::TeamDissentReason;
use crate::lane::{LaneIntent, LanerObservation, ThreatReport};

/// Versioned schema for strategic disagreement legitimacy evaluation.
pub const DISAGREEMENT_SCHEMA: &str = "m8-strategic-disagreement-v1";

/// Maximum allowed value delta bound in basis points ($\pm 10,000$ bp).
pub const MAX_DELTA_BP: i32 = 10_000;

/// Discrete classification of actor dissent legitimacy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisagreementLegitimacyClassification {
  /// Dissent was necessary to avert catastrophic loss (e.g., death/wipe under lethal threat).
  LegitimateDissent,
  /// Dissent replaced a suboptimal directive with a superior local objective (e.g., stabilization).
  ConstructiveAlternative,
  /// Dissent had no valid tactical justification and diminished team cohesion.
  UnjustifiedInsubordination,
}

/// Typed errors emitted during disagreement evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamDisagreementError {
  /// Private chain-of-thought is strictly forbidden in disagreement contracts.
  ChainOfThoughtForbidden,
  /// Value delta exceeded the maximum allowable range.
  ValueDeltaOutOfRange {
    /// Provided delta in bp.
    delta_bp: i32,
    /// Maximum allowed delta.
    max: i32,
  },
}

impl fmt::Display for TeamDisagreementError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChainOfThoughtForbidden => {
        write!(
          f,
          "private chain-of-thought is strictly forbidden in disagreement contracts"
        )
      }
      Self::ValueDeltaOutOfRange { delta_bp, max } => {
        write!(
          f,
          "value delta {delta_bp} bp exceeds maximum allowable range +/-{max} bp"
        )
      }
    }
  }
}

/// Evaluated report quantifying the strategic legitimacy of an actor's dissent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisagreementLegitimacyEvaluation {
  /// Versioned schema identifier.
  schema: &'static str,
  /// Categorical legitimacy classification.
  classification: DisagreementLegitimacyClassification,
  /// Causal reason for the dissent.
  dissent_reason: TeamDissentReason,
  /// Evaluated value of the actual dissenting action in basis points.
  actual_value_bp: i32,
  /// Counterfactual value if the actor had blindly complied in basis points.
  counterfactual_compliance_value_bp: i32,
  /// Net counterfactual value delta ($actual - counterfactual$).
  counterfactual_delta_bp: i32,
  /// Whether the dissent is classified as strategically sound.
  is_legitimate: bool,
  /// Explanation summary.
  explanation: &'static str,
  /// Strict verification flag that no private chain-of-thought is present.
  chain_of_thought_present: bool,
}

impl DisagreementLegitimacyEvaluation {
  /// Constructs a validated disagreement legitimacy evaluation.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    classification: DisagreementLegitimacyClassification,
    dissent_reason: TeamDissentReason,
    actual_value_bp: i32,
    counterfactual_compliance_value_bp: i32,
    counterfactual_delta_bp: i32,
    is_legitimate: bool,
    explanation: &'static str,
    chain_of_thought_present: bool,
  ) -> Result<Self, TeamDisagreementError> {
    if chain_of_thought_present {
      return Err(TeamDisagreementError::ChainOfThoughtForbidden);
    }
    if counterfactual_delta_bp.abs() > MAX_DELTA_BP {
      return Err(TeamDisagreementError::ValueDeltaOutOfRange {
        delta_bp: counterfactual_delta_bp,
        max: MAX_DELTA_BP,
      });
    }

    Ok(Self {
      schema: DISAGREEMENT_SCHEMA,
      classification,
      dissent_reason,
      actual_value_bp,
      counterfactual_compliance_value_bp,
      counterfactual_delta_bp,
      is_legitimate,
      explanation,
      chain_of_thought_present: false,
    })
  }

  /// Schema identifier.
  #[must_use]
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Legitimacy classification.
  #[must_use]
  pub const fn classification(&self) -> DisagreementLegitimacyClassification {
    self.classification
  }

  /// Causal dissent reason.
  #[must_use]
  pub const fn dissent_reason(&self) -> TeamDissentReason {
    self.dissent_reason
  }

  /// Actual value in basis points.
  #[must_use]
  pub const fn actual_value_bp(&self) -> i32 {
    self.actual_value_bp
  }

  /// Counterfactual compliance value in basis points.
  #[must_use]
  pub const fn counterfactual_compliance_value_bp(&self) -> i32 {
    self.counterfactual_compliance_value_bp
  }

  /// Counterfactual value delta in basis points ($actual - counterfactual$).
  #[must_use]
  pub const fn counterfactual_delta_bp(&self) -> i32 {
    self.counterfactual_delta_bp
  }

  /// Whether the dissent is legitimate.
  #[must_use]
  pub const fn is_legitimate(&self) -> bool {
    self.is_legitimate
  }

  /// Explanation text.
  #[must_use]
  pub const fn explanation(&self) -> &'static str {
    self.explanation
  }
}

/// Pure deterministic evaluator for disagreement strategic legitimacy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TeamDisagreementEvaluator;

impl TeamDisagreementEvaluator {
  /// Constructs a new disagreement evaluator.
  #[must_use]
  pub const fn new() -> Self {
    Self
  }

  /// Evaluates the strategic legitimacy of an actor's dissent against an incoming directive.
  pub fn evaluate(
    &self,
    observation: &LanerObservation,
    directed_intent: LaneIntent,
    chosen_intent: LaneIntent,
    dissent_reason: TeamDissentReason,
  ) -> Result<DisagreementLegitimacyEvaluation, TeamDisagreementError> {
    let health_val = observation.self_health().value();
    let threat_present = match observation.jungle_threat() {
      ThreatReport::LastKnown { .. } => true,
      ThreatReport::Unknown => false,
    };

    match (directed_intent, chosen_intent) {
      // Directed Contest, but agent dissented to Stabilize, Yield, or Recall due to danger.
      (
        LaneIntent::Contest,
        LaneIntent::Stabilize | LaneIntent::Yield | LaneIntent::Recall | LaneIntent::Withdraw,
      ) => {
        if health_val <= 3 || (threat_present && health_val <= 6) {
          // Severely compromised health or threat ambush: compliance is lethal.
          let actual_value = 3_000;
          let counterfactual_value = -5_000;
          let delta = actual_value - counterfactual_value; // +8,000 bp
          DisagreementLegitimacyEvaluation::new(
            DisagreementLegitimacyClassification::LegitimateDissent,
            dissent_reason,
            actual_value,
            counterfactual_value,
            delta,
            true,
            "Dissent averted lethal elimination under adverse health/threat conditions.",
            false,
          )
        } else if health_val <= 6 {
          // Moderate risk: constructive alternative to farm/stabilize.
          let actual_value = 2_000;
          let counterfactual_value = 500;
          let delta = actual_value - counterfactual_value; // +1,500 bp
          DisagreementLegitimacyEvaluation::new(
            DisagreementLegitimacyClassification::ConstructiveAlternative,
            dissent_reason,
            actual_value,
            counterfactual_value,
            delta,
            true,
            "Dissent selected a safer resource accumulation trajectory under moderate risk.",
            false,
          )
        } else {
          // Full health and no threat: insubordination undermined a viable call.
          let actual_value = 1_000;
          let counterfactual_value = 4_000;
          let delta = actual_value - counterfactual_value; // -3,000 bp
          DisagreementLegitimacyEvaluation::new(
            DisagreementLegitimacyClassification::UnjustifiedInsubordination,
            dissent_reason,
            actual_value,
            counterfactual_value,
            delta,
            false,
            "Dissent under favorable health and vision reduced team coordination payoff.",
            false,
          )
        }
      }
      // Directed Stabilize, Recall, or Yield, but agent pushed aggressive Contest.
      (
        LaneIntent::Stabilize | LaneIntent::Yield | LaneIntent::Recall | LaneIntent::Withdraw,
        LaneIntent::Contest,
      ) => {
        if health_val >= 8 && !threat_present {
          // High health clutch opportunity: constructive alternative.
          let actual_value = 4_000;
          let counterfactual_value = 2_000;
          let delta = actual_value - counterfactual_value; // +2,000 bp
          DisagreementLegitimacyEvaluation::new(
            DisagreementLegitimacyClassification::ConstructiveAlternative,
            dissent_reason,
            actual_value,
            counterfactual_value,
            delta,
            true,
            "Aggressive counter-action seized favorable lane timing despite conservative call.",
            false,
          )
        } else {
          // Low/moderate health overextension: unjustified insubordination.
          let actual_value = -2_000;
          let counterfactual_value = 2_000;
          let delta = actual_value - counterfactual_value; // -4,000 bp
          DisagreementLegitimacyEvaluation::new(
            DisagreementLegitimacyClassification::UnjustifiedInsubordination,
            dissent_reason,
            actual_value,
            counterfactual_value,
            delta,
            false,
            "Aggressive deviation under compromised health risked lane position.",
            false,
          )
        }
      }
      // Default / fallback matching.
      _ => {
        let actual_value = 1_000;
        let counterfactual_value = 1_000;
        let delta = 0;
        DisagreementLegitimacyEvaluation::new(
          DisagreementLegitimacyClassification::ConstructiveAlternative,
          dissent_reason,
          actual_value,
          counterfactual_value,
          delta,
          true,
          "Dissent yielded equivalent strategic value.",
          false,
        )
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lane::{
    JungleThreatTruth, LaneHealth, LaneSnapshot, ObservationId, PlayerLaneState, observe_player,
  };

  fn make_test_observation(health: u8, threat: bool) -> LanerObservation {
    let initial = LaneSnapshot::initial();
    let player = PlayerLaneState::new(
      initial.player().id(),
      LaneHealth::new(health).expect("valid health"),
      initial.player().resources(),
      initial.player().position(),
    );
    let state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      initial.status(),
      player,
      initial.opponent(),
      initial.wave(),
      if threat {
        JungleThreatTruth::RiverSide
      } else {
        JungleThreatTruth::Absent
      },
    );
    observe_player(&state, ObservationId::new(100)).observation()
  }

  #[test]
  fn legitimate_dissent_under_critical_health_and_threat() {
    let obs = make_test_observation(2, true);
    let evaluator = TeamDisagreementEvaluator::new();
    let eval = evaluator
      .evaluate(
        &obs,
        LaneIntent::Contest,
        LaneIntent::Stabilize,
        TeamDissentReason::LowHealth,
      )
      .expect("successful evaluation");

    assert_eq!(eval.schema(), DISAGREEMENT_SCHEMA);
    assert_eq!(
      eval.classification(),
      DisagreementLegitimacyClassification::LegitimateDissent
    );
    assert!(eval.is_legitimate());
    assert!(eval.counterfactual_delta_bp() > 0);
    assert_eq!(eval.counterfactual_delta_bp(), 8_000);
    assert!(eval.explanation().contains("averted lethal elimination"));
  }

  #[test]
  fn constructive_alternative_under_moderate_risk() {
    let obs = make_test_observation(5, false);
    let evaluator = TeamDisagreementEvaluator::new();
    let eval = evaluator
      .evaluate(
        &obs,
        LaneIntent::Contest,
        LaneIntent::Stabilize,
        TeamDissentReason::AlternativeObjectivePriority,
      )
      .expect("successful evaluation");

    assert_eq!(
      eval.classification(),
      DisagreementLegitimacyClassification::ConstructiveAlternative
    );
    assert!(eval.is_legitimate());
    assert_eq!(eval.counterfactual_delta_bp(), 1_500);
  }

  #[test]
  fn unjustified_insubordination_under_optimal_health() {
    let obs = make_test_observation(10, false);
    let evaluator = TeamDisagreementEvaluator::new();
    let eval = evaluator
      .evaluate(
        &obs,
        LaneIntent::Contest,
        LaneIntent::Stabilize,
        TeamDissentReason::PostureIncompatible,
      )
      .expect("successful evaluation");

    assert_eq!(
      eval.classification(),
      DisagreementLegitimacyClassification::UnjustifiedInsubordination
    );
    assert!(!eval.is_legitimate());
    assert!(eval.counterfactual_delta_bp() < 0);
    assert_eq!(eval.counterfactual_delta_bp(), -3_000);
  }

  #[test]
  fn rejects_chain_of_thought() {
    let err = DisagreementLegitimacyEvaluation::new(
      DisagreementLegitimacyClassification::LegitimateDissent,
      TeamDissentReason::LowHealth,
      3_000,
      -5_000,
      8_000,
      true,
      "test",
      true,
    );
    assert_eq!(
      err.unwrap_err(),
      TeamDisagreementError::ChainOfThoughtForbidden
    );
  }
}
