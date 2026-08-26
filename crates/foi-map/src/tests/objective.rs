//! Unit and scenario verification tests for objective cycles, vision control, and cross-map tradeoffs.

use crate::kernel::ActorId;
use crate::map::contest::{CrossMapTradeTarget, TradeClassification, TradeoffEvaluation};
use crate::map::objective::{
  DamageOutcome, MatchObjectiveState, ObjectiveEntry, ObjectiveError, ObjectiveKind,
  ObjectiveStatus,
};
use crate::map::objective_catalog::ObjectiveScenarioCatalog;
use crate::map::topology::{LaneId, MapLocation, TeamSide};
use crate::map::travel::ActorLocation;
use crate::map::vision::{
  DEFAULT_WARD_DURATION_TURNS, MAX_WARDS_PER_TEAM, MapVisionState, VisionCoverage, VisionError,
};

#[test]
fn objective_lifecycle_spawning_and_respawn_cycle() {
  let mut state = MatchObjectiveState::new_with_entries([
    ObjectiveEntry {
      kind: ObjectiveKind::TopRiverObjective,
      status: ObjectiveStatus::Unspawned {
        turns_until_spawn: 2,
      },
      secure_count_allied: 0,
      secure_count_opposing: 0,
    },
    ObjectiveEntry {
      kind: ObjectiveKind::BotRiverObjective,
      status: ObjectiveStatus::Unspawned {
        turns_until_spawn: 1,
      },
      secure_count_allied: 0,
      secure_count_opposing: 0,
    },
  ]);

  // Turn 1 tick: Bot river should spawn (turns_until_spawn was 1), Top river countdown reduces to 1
  let spawned = state.tick_turn();
  assert_eq!(spawned, vec![ObjectiveKind::BotRiverObjective]);
  assert!(
    state
      .get(ObjectiveKind::BotRiverObjective)
      .status
      .is_active()
  );
  assert_eq!(
    state
      .get(ObjectiveKind::BotRiverObjective)
      .status
      .current_health(),
    Some(3500)
  );
  assert!(
    state
      .get(ObjectiveKind::TopRiverObjective)
      .status
      .is_unspawned()
  );

  // Turn 2 tick: Top river should now spawn
  let spawned2 = state.tick_turn();
  assert_eq!(spawned2, vec![ObjectiveKind::TopRiverObjective]);
  assert!(
    state
      .get(ObjectiveKind::TopRiverObjective)
      .status
      .is_active()
  );
  assert_eq!(
    state
      .get(ObjectiveKind::TopRiverObjective)
      .status
      .current_health(),
    Some(5000)
  );

  // Secure Bot river objective with 3500 damage
  let outcome = state
    .apply_damage(ObjectiveKind::BotRiverObjective, 3500, TeamSide::Allied, 2)
    .expect("damage should succeed");
  assert_eq!(
    outcome,
    DamageOutcome::Secured {
      secured_by: TeamSide::Allied
    }
  );
  assert!(
    state
      .get(ObjectiveKind::BotRiverObjective)
      .status
      .is_secured()
  );
  assert_eq!(
    state
      .get(ObjectiveKind::BotRiverObjective)
      .secure_count_allied,
    1
  );
}

#[test]
fn objective_damage_validation_and_rejection() {
  let mut state = MatchObjectiveState::new_with_entries([
    ObjectiveEntry::new_unspawned(ObjectiveKind::TopRiverObjective),
    ObjectiveEntry::new_active(ObjectiveKind::BotRiverObjective, 3500),
  ]);

  // Reject zero damage
  let err_zero = state.apply_damage(ObjectiveKind::BotRiverObjective, 0, TeamSide::Allied, 1);
  assert_eq!(err_zero, Err(ObjectiveError::InvalidDamageAmount));

  // Reject damage on unspawned objective
  let err_unspawned =
    state.apply_damage(ObjectiveKind::TopRiverObjective, 1000, TeamSide::Allied, 1);
  assert_eq!(err_unspawned, Err(ObjectiveError::ObjectiveNotActive));

  // Partial damage
  let outcome = state
    .apply_damage(
      ObjectiveKind::BotRiverObjective,
      1500,
      TeamSide::Opposing,
      1,
    )
    .expect("partial damage should succeed");
  assert_eq!(
    outcome,
    DamageOutcome::Damaged {
      remaining_health: 2000
    }
  );
  assert_eq!(
    state
      .get(ObjectiveKind::BotRiverObjective)
      .status
      .current_health(),
    Some(2000)
  );
}

#[test]
fn vision_ward_placement_and_capacity_limits() {
  let mut vision = MapVisionState::new();

  // Place ward at Top River
  let ward = vision
    .place_ward(
      TeamSide::Allied,
      ActorId::new(1),
      MapLocation::TOP_RIVER,
      1,
      DEFAULT_WARD_DURATION_TURNS,
    )
    .expect("ward placement should succeed");
  assert_eq!(ward.location, MapLocation::TOP_RIVER);
  assert_eq!(ward.remaining_turns, 3);
  assert!(vision.has_allied_ward(MapLocation::TOP_RIVER, TeamSide::Allied));

  // Reject duplicate allied ward at same location
  let err_dup = vision.place_ward(
    TeamSide::Allied,
    ActorId::new(2),
    MapLocation::TOP_RIVER,
    1,
    DEFAULT_WARD_DURATION_TURNS,
  );
  assert_eq!(err_dup, Err(VisionError::LocationAlreadyWardedByTeam));

  // Opposing team can ward same location
  assert!(
    vision
      .place_ward(
        TeamSide::Opposing,
        ActorId::new(5),
        MapLocation::TOP_RIVER,
        1,
        DEFAULT_WARD_DURATION_TURNS,
      )
      .is_ok()
  );

  // Fill Allied capacity up to MAX_WARDS_PER_TEAM
  let locations = [
    MapLocation::BOT_RIVER,
    MapLocation::TOP_JUNGLE,
    MapLocation::BOT_JUNGLE,
    MapLocation::TOP_CENTER,
    MapLocation::MID_CENTER,
    MapLocation::BOT_CENTER,
    MapLocation::TOP_FAR_SIDE,
    MapLocation::MID_FAR_SIDE,
    MapLocation::BOT_FAR_SIDE,
  ];
  for loc in locations {
    vision
      .place_ward(
        TeamSide::Allied,
        ActorId::new(1),
        loc,
        1,
        DEFAULT_WARD_DURATION_TURNS,
      )
      .expect("should place up to capacity");
  }
  assert_eq!(
    vision.team_wards(TeamSide::Allied).count(),
    MAX_WARDS_PER_TEAM
  );

  // 11th ward should be rejected
  let err_cap = vision.place_ward(
    TeamSide::Allied,
    ActorId::new(1),
    MapLocation::ALLIED_BASE,
    1,
    DEFAULT_WARD_DURATION_TURNS,
  );
  assert_eq!(err_cap, Err(VisionError::WardCapacityExceeded));
}

#[test]
fn vision_de_warding_and_expiry() {
  let mut vision = MapVisionState::new();

  // Opposing places ward at Bot River
  vision
    .place_ward(
      TeamSide::Opposing,
      ActorId::new(5),
      MapLocation::BOT_RIVER,
      1,
      2,
    )
    .expect("should place ward");

  // Allied clearing non-existent ward at Top River errors
  let err_clear = vision.clear_ward(MapLocation::TOP_RIVER, TeamSide::Allied);
  assert_eq!(err_clear, Err(VisionError::NoOpposingWardAtLocation));

  // Allied clears Bot River opposing ward
  let cleared = vision
    .clear_ward(MapLocation::BOT_RIVER, TeamSide::Allied)
    .expect("de-warding should succeed");
  assert_eq!(cleared.location, MapLocation::BOT_RIVER);
  assert!(!vision.has_allied_ward(MapLocation::BOT_RIVER, TeamSide::Opposing));

  // Place another ward with 1 turn remaining, tick turn -> should expire
  vision
    .place_ward(
      TeamSide::Allied,
      ActorId::new(1),
      MapLocation::MID_CENTER,
      1,
      1,
    )
    .expect("place 1 turn ward");
  let expired = vision.tick_turn();
  assert_eq!(expired.len(), 1);
  assert_eq!(expired[0].location, MapLocation::MID_CENTER);
  assert_eq!(vision.active_wards().len(), 0);
}

#[test]
fn vision_grid_computation_and_fog_of_war_redaction() {
  let mut vision = MapVisionState::new();
  vision
    .place_ward(
      TeamSide::Allied,
      ActorId::new(1),
      MapLocation::TOP_RIVER,
      1,
      DEFAULT_WARD_DURATION_TURNS,
    )
    .expect("place ward");

  let actors = [
    (
      ActorId::new(1),
      ActorLocation::Stationary(MapLocation::TOP_CENTER),
      TeamSide::Allied,
    ),
    (
      ActorId::new(5),
      ActorLocation::Stationary(MapLocation::BOT_JUNGLE),
      TeamSide::Opposing,
    ),
  ];

  let grid = vision.compute_team_vision(TeamSide::Allied, &actors, 1, None);

  // Allied positions and warded location are FullVision
  assert_eq!(
    grid.coverage_at(MapLocation::TOP_CENTER),
    VisionCoverage::FullVision
  );
  assert_eq!(
    grid.coverage_at(MapLocation::TOP_RIVER),
    VisionCoverage::FullVision
  );

  // Opposing unit location in Bot Jungle is unobserved -> ConcealedInFog
  assert_eq!(
    grid.coverage_at(MapLocation::BOT_JUNGLE),
    VisionCoverage::ConcealedInFog
  );
  assert!(!grid.is_visible(MapLocation::BOT_JUNGLE));
}

#[test]
fn cross_map_tradeoff_evaluations_and_basis_point_scaling() {
  // Favorable trade: concede Bot Drake (4000 bp) for Top Herald (4500 bp) at 100% efficiency
  let eval_fav = TradeoffEvaluation::evaluate(
    ObjectiveKind::BotRiverObjective,
    CrossMapTradeTarget::OppositeObjective(ObjectiveKind::TopRiverObjective),
    10_000,
  );
  assert_eq!(eval_fav.conceded_value_bp, 4000);
  assert_eq!(eval_fav.gained_value_bp, 4500);
  assert_eq!(eval_fav.net_value_delta_bp, 500);
  assert_eq!(eval_fav.classification, TradeClassification::FavorableTrade);

  // Even trade: concede Top Herald (4500 bp) for Mid Tower push (4000 bp) at 100% efficiency
  let eval_mid = TradeoffEvaluation::evaluate(
    ObjectiveKind::BotRiverObjective,
    CrossMapTradeTarget::OppositeTowerPush(LaneId::Mid),
    10_000,
  );
  assert_eq!(eval_mid.net_value_delta_bp, 0);
  assert_eq!(eval_mid.classification, TradeClassification::EvenTrade);

  // Desperation sacrifice: concede Top Herald (4500 bp) for Jungle Invade (2000 bp) at 50% efficiency (1000 bp -> net -3500 bp)
  let eval_desp = TradeoffEvaluation::evaluate(
    ObjectiveKind::TopRiverObjective,
    CrossMapTradeTarget::JungleInvadeFarm(crate::map::topology::JungleSide::BotJungle),
    5000,
  );
  assert_eq!(eval_desp.gained_value_bp, 1000);
  assert_eq!(eval_desp.net_value_delta_bp, -3500);
  assert_eq!(
    eval_desp.classification,
    TradeClassification::DesperationSacrifice
  );
}

#[test]
fn state_hash_determinism_and_distinctness() {
  let mut obj1 = MatchObjectiveState::new();
  let mut vis1 = MapVisionState::new();
  let hash1 = ObjectiveScenarioCatalog::compute_hash(1, &obj1, &vis1);

  // Identical duplicate state produces identical hash
  let obj2 = MatchObjectiveState::new();
  let vis2 = MapVisionState::new();
  let hash2 = ObjectiveScenarioCatalog::compute_hash(1, &obj2, &vis2);
  assert_eq!(hash1, hash2);

  // Advancing turn changes hash
  let hash_turn2 = ObjectiveScenarioCatalog::compute_hash(2, &obj1, &vis1);
  assert_ne!(hash1, hash_turn2);

  // Placing ward changes hash
  vis1
    .place_ward(
      TeamSide::Allied,
      ActorId::new(1),
      MapLocation::BOT_RIVER,
      1,
      3,
    )
    .unwrap();
  let hash_ward = ObjectiveScenarioCatalog::compute_hash(1, &obj1, &vis1);
  assert_ne!(hash1, hash_ward);

  // Damaging objective changes hash
  obj1
    .apply_damage(ObjectiveKind::TopRiverObjective, 1000, TeamSide::Allied, 1)
    .unwrap_err(); // not active yet
  obj1.tick_turn(); // spawn
  obj1.tick_turn();
  obj1.tick_turn();
  obj1.tick_turn();
  obj1.tick_turn();
  obj1.tick_turn();
  let _ = obj1.apply_damage(ObjectiveKind::TopRiverObjective, 1000, TeamSide::Allied, 6);
  let hash_dmg = ObjectiveScenarioCatalog::compute_hash(6, &obj1, &vis1);
  assert_ne!(hash1, hash_dmg);
}

#[test]
fn catalog_all_objective_scenarios_execute_and_verify() {
  for scenario in ObjectiveScenarioCatalog::list_scenarios() {
    let result = ObjectiveScenarioCatalog::execute_scenario(scenario.scenario_id)
      .unwrap_or_else(|err| panic!("Scenario {} failed to execute: {err}", scenario.scenario_id));

    assert_eq!(result.scenario_id, scenario.scenario_id);
    assert_eq!(result.final_turn, scenario.expected_final_turn);
    assert!(result.total_events > 0);
    assert_ne!(result.final_state_hash, result.initial_state_hash);

    // Replay reproducibility: second run produces identical hash
    let replay_result = ObjectiveScenarioCatalog::execute_scenario(scenario.scenario_id)
      .expect("replay must succeed");
    assert_eq!(result.final_state_hash, replay_result.final_state_hash);
    assert_eq!(result.total_events, replay_result.total_events);
    assert_eq!(result.total_effects, replay_result.total_effects);
  }
}
