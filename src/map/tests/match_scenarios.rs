//! Verification tests for M9 canonical benchmark match scenarios.

use crate::map::match_catalog::MatchScenarioCatalog;
use crate::map::topology::TeamSide;

#[test]
fn catalog_lists_all_4_canonical_match_scenarios() {
  let scenarios = MatchScenarioCatalog::list_scenarios();
  assert_eq!(scenarios.len(), 4);

  assert_eq!(scenarios[0].scenario_id, "scenario-early-pick-snowball-v1");
  assert_eq!(scenarios[1].scenario_id, "scenario-split-push-base-race-v1");
  assert_eq!(
    scenarios[2].scenario_id,
    "scenario-late-game-scaling-comeback-v1"
  );
  assert_eq!(
    scenarios[3].scenario_id,
    "scenario-siege-inhibitor-concession-v1"
  );
}

#[test]
fn execute_all_4_canonical_match_scenarios_successfully() {
  for def in MatchScenarioCatalog::list_scenarios() {
    let result = MatchScenarioCatalog::execute_scenario(def.scenario_id)
      .expect("scenario execution should succeed");

    assert_eq!(result.scenario_id, def.scenario_id);
    assert_eq!(result.final_turn, def.expected_final_turn);
    assert!(result.total_events > 0);
    assert!(result.total_effects > 0);
    assert_ne!(result.initial_state_hash, result.final_state_hash);

    assert!(result.match_status.is_concluded());
    assert_eq!(result.match_status.winner(), Some(def.expected_winner));
  }
}

#[test]
fn scenario_early_pick_snowball_verification() {
  let result = MatchScenarioCatalog::execute_scenario("scenario-early-pick-snowball-v1").unwrap();
  assert_eq!(result.final_turn, 18);
  assert!(result.match_status.is_concluded());
  assert_eq!(result.match_status.winner(), Some(TeamSide::Allied));
}

#[test]
fn scenario_split_push_base_race_verification() {
  let result = MatchScenarioCatalog::execute_scenario("scenario-split-push-base-race-v1").unwrap();
  assert_eq!(result.final_turn, 22);
  assert!(result.match_status.is_concluded());
  assert_eq!(result.match_status.winner(), Some(TeamSide::Allied));
}

#[test]
fn scenario_late_game_scaling_comeback_verification() {
  let result =
    MatchScenarioCatalog::execute_scenario("scenario-late-game-scaling-comeback-v1").unwrap();
  assert_eq!(result.final_turn, 28);
  assert!(result.match_status.is_concluded());
  assert_eq!(result.match_status.winner(), Some(TeamSide::Allied));
}

#[test]
fn scenario_siege_inhibitor_concession_verification() {
  let result =
    MatchScenarioCatalog::execute_scenario("scenario-siege-inhibitor-concession-v1").unwrap();
  assert_eq!(result.final_turn, 24);
  assert!(result.match_status.is_concluded());
  assert_eq!(result.match_status.winner(), Some(TeamSide::Allied));
}

#[test]
fn unknown_scenario_id_fails_closed() {
  let res = MatchScenarioCatalog::execute_scenario("scenario-invalid-id-v1");
  assert_eq!(res, Err("unknown match scenario identifier"));
}
