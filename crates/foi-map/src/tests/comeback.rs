//! Focused tests for M9 comeback mechanics and variance-seeking evaluation.
//!
//! Covers:
//! - Deficit level classification from explicit net deltas
//! - Variance behavior recommendation across all four tiers
//! - Monotonic property: deeper deficit → higher variance seeking
//! - Reproducibility: same inputs yield identical evaluations
//! - Perspective symmetry: Allied and Opposing views are negations
//! - Catalog scenarios produce expected outcomes
//! - No hidden state: all inputs are explicit, no authoritative state consulted
//! - Variance multiplier ordering: DesperationAllIn > HighRisk > Balanced > Conservative

use crate::map::comeback::{
  ComebackOpportunityInputs, DeficitLevel, VarianceSeekingBehavior, evaluate_comeback_opportunity,
};
use crate::map::comeback_catalog::ComebackCatalog;
use crate::map::composition::{CompositionCatalog, MatchPhase};
use crate::map::topology::TeamSide;

// --- DeficitLevel classification ---

#[test]
fn deficit_level_ahead_above_threshold() {
  assert_eq!(DeficitLevel::from_net_delta(501), DeficitLevel::Ahead);
  assert_eq!(DeficitLevel::from_net_delta(5_000), DeficitLevel::Ahead);
  assert_eq!(DeficitLevel::from_net_delta(10_000), DeficitLevel::Ahead);
}

#[test]
fn deficit_level_parity_range() {
  assert_eq!(DeficitLevel::from_net_delta(500), DeficitLevel::Parity);
  assert_eq!(DeficitLevel::from_net_delta(0), DeficitLevel::Parity);
  assert_eq!(DeficitLevel::from_net_delta(-500), DeficitLevel::Parity);
}

#[test]
fn deficit_level_deficit_range() {
  assert_eq!(DeficitLevel::from_net_delta(-501), DeficitLevel::Deficit);
  assert_eq!(DeficitLevel::from_net_delta(-1_500), DeficitLevel::Deficit);
  assert_eq!(DeficitLevel::from_net_delta(-3_000), DeficitLevel::Deficit);
}

#[test]
fn deficit_level_severe_below_threshold() {
  assert_eq!(
    DeficitLevel::from_net_delta(-3_001),
    DeficitLevel::SevereDeficit
  );
  assert_eq!(
    DeficitLevel::from_net_delta(-7_000),
    DeficitLevel::SevereDeficit
  );
  assert_eq!(
    DeficitLevel::from_net_delta(-10_000),
    DeficitLevel::SevereDeficit
  );
}

#[test]
fn deficit_level_is_behind_predicate() {
  assert!(!DeficitLevel::Ahead.is_behind());
  assert!(!DeficitLevel::Parity.is_behind());
  assert!(DeficitLevel::Deficit.is_behind());
  assert!(DeficitLevel::SevereDeficit.is_behind());
}

// --- Variance multiplier ordering ---

#[test]
fn variance_multiplier_ordering_is_monotonic() {
  assert!(
    VarianceSeekingBehavior::DesperationAllIn.variance_multiplier_bp()
      > VarianceSeekingBehavior::HighRiskEngage.variance_multiplier_bp()
  );
  assert!(
    VarianceSeekingBehavior::HighRiskEngage.variance_multiplier_bp()
      > VarianceSeekingBehavior::BalancedApproach.variance_multiplier_bp()
  );
  assert!(
    VarianceSeekingBehavior::BalancedApproach.variance_multiplier_bp()
      > VarianceSeekingBehavior::ConservativePlay.variance_multiplier_bp()
  );
}

// --- Evaluate comeback opportunity ---

fn base_inputs(phase: MatchPhase) -> ComebackOpportunityInputs {
  ComebackOpportunityInputs {
    allied_structures_standing: 13,
    opposing_structures_standing: 13,
    allied_objectives_secured: 0,
    opposing_objectives_secured: 0,
    current_phase: phase,
    allied_power_bp: 7_000,
    opposing_power_bp: 7_000,
    recent_high_value_objective: false,
  }
}

#[test]
fn evaluate_even_match_produces_parity() {
  let inputs = base_inputs(MatchPhase::MidGame);
  let allied = &CompositionCatalog::SPLIT_PUSH;
  let opposing = &CompositionCatalog::SPLIT_PUSH;

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(result.deficit_level, DeficitLevel::Parity);
  assert_eq!(result.perspective, TeamSide::Allied);
  assert!(!result.variance_play_recommended);
}

#[test]
fn evaluate_severe_deficit_recommends_desperation() {
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 2,
    opposing_structures_standing: 13,
    allied_objectives_secured: 0,
    opposing_objectives_secured: 6,
    current_phase: MatchPhase::LateGame,
    allied_power_bp: 4_000,
    opposing_power_bp: 9_000,
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::EARLY_PICK;
  let opposing = &CompositionCatalog::TEAMFIGHT_SCALING;

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(result.deficit_level, DeficitLevel::SevereDeficit);
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::DesperationAllIn
  );
  assert!(result.variance_play_recommended);
}

#[test]
fn evaluate_ahead_recommends_conservative() {
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 13,
    opposing_structures_standing: 6,
    allied_objectives_secured: 5,
    opposing_objectives_secured: 0,
    current_phase: MatchPhase::MidGame,
    allied_power_bp: 7_500,
    opposing_power_bp: 6_500,
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::POKE_SIEGE;
  let opposing = &CompositionCatalog::EARLY_PICK;

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(result.deficit_level, DeficitLevel::Ahead);
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::ConservativePlay
  );
  assert!(!result.variance_play_recommended);
}

#[test]
fn evaluate_deficit_with_recent_objective_recommends_high_risk() {
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 7,
    opposing_structures_standing: 12,
    allied_objectives_secured: 1,
    opposing_objectives_secured: 3,
    current_phase: MatchPhase::LateGame,
    allied_power_bp: 9_000,
    opposing_power_bp: 4_000,
    recent_high_value_objective: true,
  };
  let allied = &CompositionCatalog::TEAMFIGHT_SCALING;
  let opposing = &CompositionCatalog::EARLY_PICK;

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(result.deficit_level, DeficitLevel::Deficit);
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::HighRiskEngage
  );
  assert!(result.variance_play_recommended);
}

// --- Reproducibility: identical inputs produce identical results ---

#[test]
fn evaluate_is_reproducible() {
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 8,
    opposing_structures_standing: 11,
    allied_objectives_secured: 2,
    opposing_objectives_secured: 4,
    current_phase: MatchPhase::LateGame,
    allied_power_bp: 6_500,
    opposing_power_bp: 8_000,
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::TEAMFIGHT_SCALING;
  let opposing = &CompositionCatalog::SPLIT_PUSH;

  let r1 = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  let r2 = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(
    r1, r2,
    "evaluate_comeback_opportunity must be deterministic"
  );
}

// --- Perspective symmetry: Allied and Opposing views negate each other ---

#[test]
fn perspective_negation_symmetry() {
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 10,
    opposing_structures_standing: 5,
    allied_objectives_secured: 3,
    opposing_objectives_secured: 1,
    current_phase: MatchPhase::MidGame,
    allied_power_bp: 7_000,
    opposing_power_bp: 6_000,
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::POKE_SIEGE;
  let opposing = &CompositionCatalog::EARLY_PICK;

  let allied_view = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  let opp_view = evaluate_comeback_opportunity(TeamSide::Opposing, &inputs, allied, opposing);

  // The net delta from Allied perspective must equal the negation of the Opposing perspective.
  assert_eq!(
    allied_view.net_value_delta_bp, -opp_view.net_value_delta_bp,
    "Allied and Opposing net deltas must be symmetric"
  );
  // Allied leads, so Opposing should be behind.
  assert!(allied_view.net_value_delta_bp > 0);
  assert!(opp_view.net_value_delta_bp < 0);
  assert!(opp_view.deficit_level.is_behind());
  assert!(!allied_view.deficit_level.is_behind());
}

// --- Net delta clamp: stays within [-10,000..=10,000] ---

#[test]
fn net_delta_is_clamped_to_bounds() {
  // Extreme lead: 13 vs 0 structures, 10 vs 0 objectives, max power advantage.
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 13,
    opposing_structures_standing: 0,
    allied_objectives_secured: 10,
    opposing_objectives_secured: 0,
    current_phase: MatchPhase::LateGame,
    allied_power_bp: 10_000,
    opposing_power_bp: 0,
    recent_high_value_objective: true,
  };
  let allied = &CompositionCatalog::TEAMFIGHT_SCALING;
  let opposing = &CompositionCatalog::EARLY_PICK;

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert!(
    result.net_value_delta_bp <= 10_000,
    "net delta must not exceed 10,000 bp"
  );
  assert!(
    result.net_value_delta_bp >= -10_000,
    "net delta must not go below -10,000 bp"
  );
}

// --- Catalog scenarios ---

#[test]
fn catalog_teamfight_comeback_scenario_meets_expectations() {
  let result = ComebackCatalog::execute_scenario("scenario-teamfight-comeback-v1")
    .expect("scenario must be registered");
  assert!(
    result.all_expectations_met,
    "teamfight comeback scenario: expected {:?}/{:?}, got {:?}/{:?}",
    ComebackCatalog::SCENARIO_TEAMFIGHT_COMEBACK.expected_deficit_level,
    ComebackCatalog::SCENARIO_TEAMFIGHT_COMEBACK.expected_behavior,
    result.evaluation.deficit_level,
    result.evaluation.recommended_behavior,
  );
}

#[test]
fn catalog_desperation_all_in_scenario_meets_expectations() {
  let result = ComebackCatalog::execute_scenario("scenario-desperation-all-in-v1")
    .expect("scenario must be registered");
  assert!(
    result.all_expectations_met,
    "desperation all-in scenario: expected {:?}/{:?}, got {:?}/{:?}",
    ComebackCatalog::SCENARIO_DESPERATION_ALL_IN.expected_deficit_level,
    ComebackCatalog::SCENARIO_DESPERATION_ALL_IN.expected_behavior,
    result.evaluation.deficit_level,
    result.evaluation.recommended_behavior,
  );
  assert!(result.evaluation.variance_play_recommended);
}

#[test]
fn catalog_ahead_conservative_scenario_meets_expectations() {
  let result = ComebackCatalog::execute_scenario("scenario-ahead-conservative-v1")
    .expect("scenario must be registered");
  assert!(
    result.all_expectations_met,
    "ahead conservative scenario: expected {:?}/{:?}, got {:?}/{:?}",
    ComebackCatalog::SCENARIO_AHEAD_CONSERVATIVE.expected_deficit_level,
    ComebackCatalog::SCENARIO_AHEAD_CONSERVATIVE.expected_behavior,
    result.evaluation.deficit_level,
    result.evaluation.recommended_behavior,
  );
  assert!(!result.evaluation.variance_play_recommended);
}

#[test]
fn catalog_unknown_scenario_returns_error() {
  let err = ComebackCatalog::execute_scenario("scenario-nonexistent-v99");
  assert!(err.is_err(), "unknown scenario ID must return an error");
}

#[test]
fn catalog_all_scenarios_are_reproducible() {
  for def in ComebackCatalog::list_scenarios() {
    let r1 =
      ComebackCatalog::execute_scenario(def.scenario_id).expect("registered scenario must execute");
    let r2 =
      ComebackCatalog::execute_scenario(def.scenario_id).expect("registered scenario must execute");
    assert_eq!(
      r1.evaluation, r2.evaluation,
      "scenario '{}' must produce identical evaluations on repeated runs",
      def.scenario_id
    );
  }
}

#[test]
fn catalog_lists_all_three_scenarios() {
  let scenarios = ComebackCatalog::list_scenarios();
  assert_eq!(scenarios.len(), 3);
  let ids: Vec<&str> = scenarios.iter().map(|s| s.scenario_id).collect();
  assert!(ids.contains(&"scenario-teamfight-comeback-v1"));
  assert!(ids.contains(&"scenario-desperation-all-in-v1"));
  assert!(ids.contains(&"scenario-ahead-conservative-v1"));
}

// --- Markdown rendering smoke test ---

#[test]
fn evaluation_renders_markdown_without_private_data() {
  let inputs = base_inputs(MatchPhase::EarlyGame);
  let allied = &CompositionCatalog::EARLY_PICK;
  let opposing = &CompositionCatalog::SPLIT_PUSH;

  let eval = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  let md = eval.render_markdown();

  // Must include labeled sections; must not include raw hash or trace strings.
  assert!(
    md.contains("Deficit Level"),
    "markdown must include deficit level label"
  );
  assert!(
    md.contains("Recommended Behavior"),
    "markdown must include behavior label"
  );
  assert!(!md.contains("hash"), "markdown must not include hash data");
  assert!(
    !md.contains("trace"),
    "markdown must not include trace data"
  );
}

#[test]
fn render_markdown_variance_multiplier_shows_decimal() {
  // DesperationAllIn = 25,000 bp = 2.5×; must render as "2.5×", not "2×".
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 2,
    opposing_structures_standing: 13,
    allied_objectives_secured: 0,
    opposing_objectives_secured: 6,
    current_phase: MatchPhase::LateGame,
    allied_power_bp: 4_000,
    opposing_power_bp: 9_000,
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::EARLY_PICK;
  let opposing = &CompositionCatalog::TEAMFIGHT_SCALING;

  let eval = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(
    eval.recommended_behavior,
    VarianceSeekingBehavior::DesperationAllIn
  );
  let md = eval.render_markdown();
  assert!(
    md.contains("2.5×"),
    "DesperationAllIn must render as 2.5× in markdown, got: {md}"
  );
}

// --- Opposing perspective behavior correctness ---

#[test]
fn opposing_perspective_uses_opposing_comp_for_behavior() {
  // Setup: Allied is TeamfightScaling (high late power), Opposing is EarlyPick (low late power).
  // Allied is ahead structurally and in objectives.
  // From Opposing's perspective: they are behind → Deficit or SevereDeficit.
  // The behavior should be based on Opposing's composition (EarlyPick), NOT Allied's.
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 11,
    opposing_structures_standing: 5,
    allied_objectives_secured: 4,
    opposing_objectives_secured: 0,
    current_phase: MatchPhase::LateGame,
    allied_power_bp: CompositionCatalog::TEAMFIGHT_SCALING.scaling.late_game_bp,
    opposing_power_bp: CompositionCatalog::EARLY_PICK.scaling.late_game_bp,
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::TEAMFIGHT_SCALING;
  let opposing = &CompositionCatalog::EARLY_PICK;

  let opp_view = evaluate_comeback_opportunity(TeamSide::Opposing, &inputs, allied, opposing);

  // Opposing is in deficit/severe-deficit, so they need a high-variance play.
  assert!(
    opp_view.deficit_level.is_behind(),
    "Opposing should be behind, got {:?}",
    opp_view.deficit_level
  );
  assert!(
    opp_view.variance_play_recommended,
    "Opposing should be recommended a variance play when in deficit"
  );
  // The Opposing team's behavior must NOT be ConservativePlay (that would be
  // Allied's behavior if the perspective bug were present).
  assert_ne!(
    opp_view.recommended_behavior,
    VarianceSeekingBehavior::ConservativePlay,
    "Opposing in deficit must not be recommended ConservativePlay"
  );
}

#[test]
fn opposing_perspective_recent_objective_is_not_credited_to_opponent() {
  // When Allied recently secured a high-value objective (flag = true),
  // the Opposing team should NOT receive the HighRiskEngage boost from it
  // (since the objective was secured by the enemy, not themselves).
  let inputs_with_allied_objective = ComebackOpportunityInputs {
    allied_structures_standing: 7,
    opposing_structures_standing: 12,
    allied_objectives_secured: 2,
    opposing_objectives_secured: 1,
    current_phase: MatchPhase::MidGame, // mid-game: no late-game phase boost
    allied_power_bp: 6_500,
    opposing_power_bp: 7_500,
    recent_high_value_objective: true, // Allied secured it
  };
  let inputs_without_objective = ComebackOpportunityInputs {
    recent_high_value_objective: false,
    ..inputs_with_allied_objective
  };
  let allied = &CompositionCatalog::EARLY_PICK;
  let opposing = &CompositionCatalog::SPLIT_PUSH;

  // From the Opposing perspective with the Allied-secured objective, Opposing
  // must NOT receive the HighRiskEngage boost — they didn't secure it.
  let with_obj = evaluate_comeback_opportunity(
    TeamSide::Opposing,
    &inputs_with_allied_objective,
    allied,
    opposing,
  );
  let without_obj = evaluate_comeback_opportunity(
    TeamSide::Opposing,
    &inputs_without_objective,
    allied,
    opposing,
  );

  // Both evaluations should produce the same recommendation because the flag
  // is Allied-centric and must be ignored when evaluating Opposing's behavior.
  assert_eq!(
    with_obj.recommended_behavior, without_obj.recommended_behavior,
    "Opposing's behavior recommendation must not change based on Allied's recent objective"
  );
}

// --- Parity and Ahead conditional branches ---

#[test]
fn parity_with_phase_power_edge_recommends_high_risk_engage() {
  // Net value delta must be at Parity ([-500..=500]) so that the Parity branch
  // of recommend_variance_behavior is reached.
  //
  // The `inputs.allied_power_bp` / `inputs.opposing_power_bp` feed into the
  // net delta calculation. To keep the net at 0 (Parity), we set them equal.
  // The *compositions* passed to evaluate_comeback_opportunity are separate —
  // they determine which branch fires inside recommend_variance_behavior.
  // PokeSiege mid-game (7500) vs EarlyPick mid-game (6000) → delta 1500 > 1000
  // → recommend_variance_behavior yields HighRiskEngage from the Parity branch.
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 10,
    opposing_structures_standing: 10,
    allied_objectives_secured: 2,
    opposing_objectives_secured: 2,
    current_phase: MatchPhase::MidGame,
    // Equal power_bp so net delta = 0 → Parity.
    allied_power_bp: 7_000,
    opposing_power_bp: 7_000,
    recent_high_value_objective: false,
  };
  // But pass PokeSiege vs EarlyPick as compositions so the phase-power check
  // inside recommend_variance_behavior sees a 1500 bp edge for Allied.
  let allied = &CompositionCatalog::POKE_SIEGE; // mid_game_bp: 7500
  let opposing = &CompositionCatalog::EARLY_PICK; // mid_game_bp: 6000

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(
    result.deficit_level,
    DeficitLevel::Parity,
    "setup must be at parity"
  );
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::HighRiskEngage,
    "parity with >1000 bp composition phase advantage should recommend HighRiskEngage"
  );
}

#[test]
fn parity_without_phase_power_edge_recommends_balanced() {
  // Parity overall; both teams have similar mid-game power (within 1000 bp).
  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 10,
    opposing_structures_standing: 10,
    allied_objectives_secured: 2,
    opposing_objectives_secured: 2,
    current_phase: MatchPhase::MidGame,
    allied_power_bp: CompositionCatalog::SPLIT_PUSH.scaling.mid_game_bp, // 7500
    opposing_power_bp: CompositionCatalog::POKE_SIEGE.scaling.mid_game_bp, // 7500
    recent_high_value_objective: false,
  };
  let allied = &CompositionCatalog::SPLIT_PUSH;
  let opposing = &CompositionCatalog::POKE_SIEGE;

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(
    result.deficit_level,
    DeficitLevel::Parity,
    "setup must be at parity"
  );
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::BalancedApproach,
    "parity without phase advantage should recommend BalancedApproach"
  );
}

#[test]
fn ahead_with_hard_scaling_opponent_recommends_balanced_not_conservative() {
  // Allied is ahead but Opposing is TeamfightScaling with much higher late-game power.
  // When not yet in late game, the Ahead branch should flip to BalancedApproach
  // to pressure before the opponent's power spike arrives.
  let allied = &CompositionCatalog::EARLY_PICK; // late: 4000 bp
  let opposing = &CompositionCatalog::TEAMFIGHT_SCALING; // late: 9000 bp
  // Difference: 9000 - 4000 = 5000 > 2000 threshold → should yield BalancedApproach.

  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 11,
    opposing_structures_standing: 8,
    allied_objectives_secured: 3,
    opposing_objectives_secured: 1,
    current_phase: MatchPhase::MidGame, // not yet late game
    allied_power_bp: allied.scaling.mid_game_bp,
    opposing_power_bp: opposing.scaling.mid_game_bp,
    recent_high_value_objective: false,
  };

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(
    result.deficit_level,
    DeficitLevel::Ahead,
    "setup must be ahead"
  );
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::BalancedApproach,
    "ahead vs hard late-scaling opponent (not yet late game) should recommend BalancedApproach"
  );
}

#[test]
fn ahead_without_hard_scaling_opponent_recommends_conservative() {
  // Allied is ahead with no hard-scaling threat from Opposing — ConservativePlay.
  // SPLIT_PUSH (late: 7000) vs POKE_SIEGE (late: 6500): delta = 500 ≤ 2000 → Conservative.
  let allied = &CompositionCatalog::SPLIT_PUSH; // late: 7000
  let opposing = &CompositionCatalog::POKE_SIEGE; // late: 6500 — delta 500 ≤ 2000

  let inputs = ComebackOpportunityInputs {
    allied_structures_standing: 12,
    opposing_structures_standing: 7,
    allied_objectives_secured: 4,
    opposing_objectives_secured: 1,
    current_phase: MatchPhase::MidGame,
    allied_power_bp: allied.scaling.mid_game_bp,
    opposing_power_bp: opposing.scaling.mid_game_bp,
    recent_high_value_objective: false,
  };

  let result = evaluate_comeback_opportunity(TeamSide::Allied, &inputs, allied, opposing);
  assert_eq!(
    result.deficit_level,
    DeficitLevel::Ahead,
    "setup must be ahead"
  );
  assert_eq!(
    result.recommended_behavior,
    VarianceSeekingBehavior::ConservativePlay,
    "ahead with no hard-scaling threat should recommend ConservativePlay"
  );
}
