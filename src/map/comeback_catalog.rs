//! Canonical comeback and variance-seeking benchmark scenarios for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! Each scenario exercises a distinct deficit tier and variance-behavior
//! recommendation path through `evaluate_comeback_opportunity` with explicit
//! caller-supplied inputs. Scenarios are reproducible: same inputs always
//! produce the same evaluation.

use super::comeback::{
  ComebackEvaluation, ComebackOpportunityInputs, DeficitLevel, VarianceSeekingBehavior,
  evaluate_comeback_opportunity,
};
use super::composition::{CompositionCatalog, MatchPhase};
use super::topology::TeamSide;

pub const M9_COMEBACK_CATALOG_SCHEMA_V1: &str = "m9-comeback-catalog-v1";

/// Specification of a canonical comeback benchmark scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComebackScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub expected_deficit_level: DeficitLevel,
  pub expected_behavior: VarianceSeekingBehavior,
  pub expected_variance_recommended: bool,
}

/// Execution result of running a canonical comeback scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComebackScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub evaluation: ComebackEvaluation,
  pub deficit_matches: bool,
  pub behavior_matches: bool,
  pub variance_flag_matches: bool,
  pub all_expectations_met: bool,
}

/// Catalog of registered canonical comeback and variance-seeking scenarios for M9.
pub struct ComebackCatalog;

impl ComebackCatalog {
  /// Scenario 1: Teamfight scaling comp in mid-game deficit with recent Drake secured.
  ///
  /// Allied `TeamfightScaling` is behind in structures but just secured Drake,
  /// creating a momentum window in the late game where their power curve peaks.
  /// Expected: `Deficit` level → `HighRiskEngage` (recent high-value objective + late phase).
  pub const SCENARIO_TEAMFIGHT_COMEBACK: ComebackScenarioDefinition = ComebackScenarioDefinition {
    scenario_id: "scenario-teamfight-comeback-v1",
    name: "Teamfight Scaling Comeback: Dragon Soul Momentum",
    description: "Allied TeamfightScaling comp is down 5 structures but just secured Dragon \
        Soul in late game. Their power spike creates a viable comeback window through \
        a high-risk engage decision.",
    expected_deficit_level: DeficitLevel::Deficit,
    expected_behavior: VarianceSeekingBehavior::HighRiskEngage,
    expected_variance_recommended: true,
  };

  /// Scenario 2: Severe structural deficit with EarlyPick comp in late game.
  ///
  /// Allied `EarlyPick` fell off into late game with all 3 inhibitors down and
  /// heavy objective deficit. No recent objectives secured. Only a desperation
  /// all-in can realistically close a gap of this magnitude.
  /// Expected: `SevereDeficit` → `DesperationAllIn`.
  pub const SCENARIO_DESPERATION_ALL_IN: ComebackScenarioDefinition = ComebackScenarioDefinition {
    scenario_id: "scenario-desperation-all-in-v1",
    name: "EarlyPick Desperation All-In: Late-Game Last Stand",
    description: "Allied EarlyPick comp has fallen massively behind with all inhibitors broken \
        and a severe structural/objective deficit in late game. Desperation all-in is the \
        only rational play — any other strategy guarantees a loss.",
    expected_deficit_level: DeficitLevel::SevereDeficit,
    expected_behavior: VarianceSeekingBehavior::DesperationAllIn,
    expected_variance_recommended: true,
  };

  /// Scenario 3: Leading team plays conservatively to deny comeback windows.
  ///
  /// Allied `SplitPush` is ahead with structural lead and objective advantage.
  /// Opponent is a late-scaling `TeamfightScaling` comp — Allied should minimize
  /// variance to close the game before opponents reach their power spike.
  /// Expected: `Ahead` → `ConservativePlay`.
  pub const SCENARIO_AHEAD_CONSERVATIVE: ComebackScenarioDefinition = ComebackScenarioDefinition {
    scenario_id: "scenario-ahead-conservative-v1",
    name: "SplitPush Leader: Deny Comeback Windows",
    description: "Allied SplitPush comp holds a comfortable structural lead and objective \
        advantage. Opposing TeamfightScaling is not yet at their late-game power spike. \
        Conservative closure minimizes risk of handing opponents a comeback window.",
    expected_deficit_level: DeficitLevel::Ahead,
    expected_behavior: VarianceSeekingBehavior::ConservativePlay,
    expected_variance_recommended: false,
  };

  pub const ALL_SCENARIOS: [ComebackScenarioDefinition; 3] = [
    Self::SCENARIO_TEAMFIGHT_COMEBACK,
    Self::SCENARIO_DESPERATION_ALL_IN,
    Self::SCENARIO_AHEAD_CONSERVATIVE,
  ];

  pub fn list_scenarios() -> &'static [ComebackScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static ComebackScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Execute a named comeback benchmark scenario and return verifiable evaluation.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<ComebackScenarioExecutionResult, &'static str> {
    match scenario_id {
      "scenario-teamfight-comeback-v1" => Ok(Self::run_teamfight_comeback()),
      "scenario-desperation-all-in-v1" => Ok(Self::run_desperation_all_in()),
      "scenario-ahead-conservative-v1" => Ok(Self::run_ahead_conservative()),
      _ => Err("unknown-comeback-scenario"),
    }
  }

  fn run_teamfight_comeback() -> ComebackScenarioExecutionResult {
    let def = Self::SCENARIO_TEAMFIGHT_COMEBACK;
    let allied_comp = &CompositionCatalog::TEAMFIGHT_SCALING;
    let opposing_comp = &CompositionCatalog::EARLY_PICK;

    // Allied has 7 structures standing vs opponent's 12; down 3 objectives to 1.
    // Just secured Drake (recent_high_value_objective = true). Late game phase.
    let inputs = ComebackOpportunityInputs {
      allied_structures_standing: 7,
      opposing_structures_standing: 12,
      allied_objectives_secured: 1,
      opposing_objectives_secured: 3,
      current_phase: MatchPhase::LateGame,
      allied_power_bp: allied_comp.scaling.late_game_bp,
      opposing_power_bp: opposing_comp.scaling.late_game_bp,
      recent_high_value_objective: true,
    };

    let evaluation =
      evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied_comp, opposing_comp);

    let deficit_matches = evaluation.deficit_level == def.expected_deficit_level;
    let behavior_matches = evaluation.recommended_behavior == def.expected_behavior;
    let variance_flag_matches =
      evaluation.variance_play_recommended == def.expected_variance_recommended;
    let all_expectations_met = deficit_matches && behavior_matches && variance_flag_matches;

    ComebackScenarioExecutionResult {
      scenario_id: def.scenario_id,
      evaluation,
      deficit_matches,
      behavior_matches,
      variance_flag_matches,
      all_expectations_met,
    }
  }

  fn run_desperation_all_in() -> ComebackScenarioExecutionResult {
    let def = Self::SCENARIO_DESPERATION_ALL_IN;
    let allied_comp = &CompositionCatalog::EARLY_PICK;
    let opposing_comp = &CompositionCatalog::TEAMFIGHT_SCALING;

    // Allied has only 3 structures standing vs opponent's 13; down 5 objectives to 0.
    // Late game, no recent objective. EarlyPick has fallen off vs late-scaling opponent.
    let inputs = ComebackOpportunityInputs {
      allied_structures_standing: 3,
      opposing_structures_standing: 13,
      allied_objectives_secured: 0,
      opposing_objectives_secured: 5,
      current_phase: MatchPhase::LateGame,
      allied_power_bp: allied_comp.scaling.late_game_bp,
      opposing_power_bp: opposing_comp.scaling.late_game_bp,
      recent_high_value_objective: false,
    };

    let evaluation =
      evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied_comp, opposing_comp);

    let deficit_matches = evaluation.deficit_level == def.expected_deficit_level;
    let behavior_matches = evaluation.recommended_behavior == def.expected_behavior;
    let variance_flag_matches =
      evaluation.variance_play_recommended == def.expected_variance_recommended;
    let all_expectations_met = deficit_matches && behavior_matches && variance_flag_matches;

    ComebackScenarioExecutionResult {
      scenario_id: def.scenario_id,
      evaluation,
      deficit_matches,
      behavior_matches,
      variance_flag_matches,
      all_expectations_met,
    }
  }

  fn run_ahead_conservative() -> ComebackScenarioExecutionResult {
    let def = Self::SCENARIO_AHEAD_CONSERVATIVE;
    let allied_comp = &CompositionCatalog::SPLIT_PUSH;
    let opposing_comp = &CompositionCatalog::TEAMFIGHT_SCALING;

    // Allied leads: 11 structures vs opponent's 7; 4 objectives to opponent's 1.
    // Mid game — opponent's TeamfightScaling has not yet peaked.
    let inputs = ComebackOpportunityInputs {
      allied_structures_standing: 11,
      opposing_structures_standing: 7,
      allied_objectives_secured: 4,
      opposing_objectives_secured: 1,
      current_phase: MatchPhase::MidGame,
      allied_power_bp: allied_comp.scaling.mid_game_bp,
      opposing_power_bp: opposing_comp.scaling.mid_game_bp,
      recent_high_value_objective: false,
    };

    let evaluation =
      evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied_comp, opposing_comp);

    let deficit_matches = evaluation.deficit_level == def.expected_deficit_level;
    let behavior_matches = evaluation.recommended_behavior == def.expected_behavior;
    let variance_flag_matches =
      evaluation.variance_play_recommended == def.expected_variance_recommended;
    let all_expectations_met = deficit_matches && behavior_matches && variance_flag_matches;

    ComebackScenarioExecutionResult {
      scenario_id: def.scenario_id,
      evaluation,
      deficit_matches,
      behavior_matches,
      variance_flag_matches,
      all_expectations_met,
    }
  }
}
