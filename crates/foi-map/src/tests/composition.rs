//! Unit tests for M9 team compositions, archetypes, and power spike matchup evaluations.

use crate::map::composition::{
  CompositionArchetype, CompositionCatalog, CompositionMatchupEvaluation, MatchPhase, MatchRole,
  RecommendedPosture,
};
use crate::map::topology::TeamSide;

#[test]
fn match_roles_and_archetypes_properties() {
  assert_eq!(MatchRole::ALL.len(), 5);
  assert_eq!(MatchRole::TopLaner.as_str(), "top-laner");
  assert_eq!(MatchRole::Jungler.as_str(), "jungler");
  assert_eq!(MatchRole::MidLaner.as_str(), "mid-laner");
  assert_eq!(MatchRole::BotCarry.as_str(), "bot-carry");
  assert_eq!(MatchRole::Support.as_str(), "support");

  assert_eq!(CompositionArchetype::ALL.len(), 4);
  assert_eq!(CompositionArchetype::EarlyPick.as_str(), "early-pick");
  assert_eq!(
    CompositionArchetype::TeamfightScaling.as_str(),
    "teamfight-scaling"
  );
  assert_eq!(CompositionArchetype::SplitPush.as_str(), "split-push");
  assert_eq!(CompositionArchetype::PokeSiege.as_str(), "poke-siege");
}

#[test]
fn match_phase_from_turn_boundaries() {
  assert_eq!(MatchPhase::from_turn(1), MatchPhase::EarlyGame);
  assert_eq!(MatchPhase::from_turn(10), MatchPhase::EarlyGame);
  assert_eq!(MatchPhase::from_turn(11), MatchPhase::MidGame);
  assert_eq!(MatchPhase::from_turn(20), MatchPhase::MidGame);
  assert_eq!(MatchPhase::from_turn(21), MatchPhase::LateGame);
  assert_eq!(MatchPhase::from_turn(50), MatchPhase::LateGame);
}

#[test]
fn power_scaling_curve_and_matchup_evaluation() {
  let early_comp = CompositionCatalog::get_by_archetype(CompositionArchetype::EarlyPick);
  let scaling_comp = CompositionCatalog::get_by_archetype(CompositionArchetype::TeamfightScaling);

  // Turn 5 (Early Game): EarlyPick has 8000 bp vs Scaling 4000 bp (+4000 bp net delta for Allied)
  let eval_early = CompositionMatchupEvaluation::evaluate(5, early_comp, scaling_comp);
  assert_eq!(eval_early.phase, MatchPhase::EarlyGame);
  assert_eq!(eval_early.allied_power_bp, 8000);
  assert_eq!(eval_early.opposing_power_bp, 4000);
  assert_eq!(eval_early.net_power_delta_bp, 4000);
  assert_eq!(eval_early.favored_team, Some(TeamSide::Allied));
  assert_eq!(
    eval_early.recommended_allied_posture,
    RecommendedPosture::ForceEarlyFights
  );

  // Turn 25 (Late Game): EarlyPick has 4000 bp vs Scaling 9000 bp (-5000 bp net delta for Allied)
  let eval_late = CompositionMatchupEvaluation::evaluate(25, early_comp, scaling_comp);
  assert_eq!(eval_late.phase, MatchPhase::LateGame);
  assert_eq!(eval_late.allied_power_bp, 4000);
  assert_eq!(eval_late.opposing_power_bp, 9000);
  assert_eq!(eval_late.net_power_delta_bp, -5000);
  assert_eq!(eval_late.favored_team, Some(TeamSide::Opposing));
}

#[test]
fn composition_catalog_retrieval() {
  assert_eq!(CompositionCatalog::ALL_COMPOSITIONS.len(), 4);
  assert!(CompositionCatalog::get_by_id("composition-early-pick-v1").is_some());
  assert!(CompositionCatalog::get_by_id("composition-scaling-teamfight-v1").is_some());
  assert!(CompositionCatalog::get_by_id("composition-split-push-v1").is_some());
  assert!(CompositionCatalog::get_by_id("composition-poke-siege-v1").is_some());
  assert!(CompositionCatalog::get_by_id("non-existent").is_none());
}
