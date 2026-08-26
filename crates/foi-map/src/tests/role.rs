//! Tests for role-specific observations, actions, and debrief perspectives for M9.

use crate::kernel::ActorId;
use crate::map::composition::MatchRole;
use crate::map::objective::ObjectiveKind;
use crate::map::role_action::{
  BotCarryIntent, JungleIntent, RoleAction, RoleActionError, RoleIntent, SupportIntent, TopIntent,
  validate_role_action,
};
use crate::map::role_catalog::RoleScenarioCatalog;
use crate::map::role_debrief::{
  RoleCausalFactor, RoleDebriefPerspective, RoleKpis, RolePerformanceTier,
};
use crate::map::role_observation::{
  JunglerContext, RoleMatchObservation, RoleSpecificContext, SupportContext, TopLanerContext,
  WaveStateSummary,
};
use crate::map::state::MatchMapState;
use crate::map::topology::{LaneId, LaneSector, MapLocation, RiverSide, TeamSide};
use crate::map::travel::ActorLocation;

#[test]
fn role_observation_creation_and_mismatch_rejection() {
  let actor = ActorId::new(1);
  let state = MatchMapState::new(
    1,
    vec![actor],
    vec![ActorId::new(2)],
    vec![
      (
        actor,
        ActorLocation::Stationary(MapLocation::Lane(LaneId::Top, LaneSector::Center)),
      ),
      (
        ActorId::new(2),
        ActorLocation::Stationary(MapLocation::Lane(LaneId::Bot, LaneSector::Center)),
      ),
    ],
  );

  let map_obs = state.observe(actor).expect("map observation");

  let valid_obs = RoleMatchObservation::new(
    MatchRole::TopLaner,
    actor,
    TeamSide::Allied,
    1,
    map_obs.clone(),
    RoleSpecificContext::TopLaner(TopLanerContext {
      top_wave_state: WaveStateSummary::FrozenAtCenter,
      top_sector: LaneSector::Center,
      top_river_objective_status: None,
      teleport_ready: true,
      side_lane_pressure_bp: 5000,
    }),
  );
  assert!(valid_obs.is_ok());

  // Mismatch: Jungler role with TopLaner context
  let mismatch_obs = RoleMatchObservation::new(
    MatchRole::Jungler,
    actor,
    TeamSide::Allied,
    1,
    map_obs,
    RoleSpecificContext::TopLaner(TopLanerContext {
      top_wave_state: WaveStateSummary::FrozenAtCenter,
      top_sector: LaneSector::Center,
      top_river_objective_status: None,
      teleport_ready: true,
      side_lane_pressure_bp: 5000,
    }),
  );
  assert!(mismatch_obs.is_err());
}

#[test]
fn role_action_validation_for_all_roles_and_error_cases() {
  let actor = ActorId::new(1);
  let state = MatchMapState::new(
    5,
    vec![actor],
    vec![ActorId::new(10)],
    vec![
      (
        actor,
        ActorLocation::Stationary(MapLocation::Lane(LaneId::Top, LaneSector::Center)),
      ),
      (
        ActorId::new(10),
        ActorLocation::Stationary(MapLocation::Lane(LaneId::Top, LaneSector::FarSide)),
      ),
    ],
  );
  let map_obs = state.observe(actor).expect("map observation");

  // 1. TopLaner: Teleport availability check
  let top_obs_no_tp = RoleMatchObservation::new(
    MatchRole::TopLaner,
    actor,
    TeamSide::Allied,
    5,
    map_obs.clone(),
    RoleSpecificContext::TopLaner(TopLanerContext {
      top_wave_state: WaveStateSummary::PushingToAlly,
      top_sector: LaneSector::Center,
      top_river_objective_status: None,
      teleport_ready: false, // on cooldown!
      side_lane_pressure_bp: 4000,
    }),
  )
  .expect("top obs");

  let tp_action = RoleAction::new(
    MatchRole::TopLaner,
    RoleIntent::Top(TopIntent::TeleportFlank {
      target_location: MapLocation::River(RiverSide::BotRiver),
    }),
  );
  assert_eq!(
    validate_role_action(&tp_action, &top_obs_no_tp),
    Err(RoleActionError::TeleportUnavailable)
  );

  // 2. Jungler: Smite availability check
  let jg_obs_no_smite = RoleMatchObservation::new(
    MatchRole::Jungler,
    actor,
    TeamSide::Allied,
    5,
    map_obs.clone(),
    RoleSpecificContext::Jungler(JunglerContext {
      camps_cleared_ratio_bp: 6000,
      smite_ready: false, // smite unavailable
      top_objective_timer: None,
      bot_objective_timer: Some(0),
      gank_opportunities: vec![],
    }),
  )
  .expect("jg obs");

  let smite_action = RoleAction::new(
    MatchRole::Jungler,
    RoleIntent::Jungle(JungleIntent::SecureNeutralObjective {
      kind: ObjectiveKind::BotRiverObjective,
    }),
  );
  assert_eq!(
    validate_role_action(&smite_action, &jg_obs_no_smite),
    Err(RoleActionError::SmiteUnavailable)
  );

  // 3. Support: Wards and sweep availability checks
  let sup_obs_no_wards = RoleMatchObservation::new(
    MatchRole::Support,
    actor,
    TeamSide::Allied,
    5,
    map_obs.clone(),
    RoleSpecificContext::Support(SupportContext {
      wards_available: 0,
      oracle_sweep_ready: false,
      protected_role: MatchRole::BotCarry,
      engage_readiness_bp: 5000,
      contested_river_side: None,
    }),
  )
  .expect("sup obs");

  let ward_action = RoleAction::new(
    MatchRole::Support,
    RoleIntent::Support(SupportIntent::EstablishVisionZone {
      location: MapLocation::River(RiverSide::BotRiver),
    }),
  );
  assert_eq!(
    validate_role_action(&ward_action, &sup_obs_no_wards),
    Err(RoleActionError::WardsUnavailable)
  );

  let sweep_action = RoleAction::new(
    MatchRole::Support,
    RoleIntent::Support(SupportIntent::ClearEnemyVision {
      location: MapLocation::River(RiverSide::BotRiver),
    }),
  );
  assert_eq!(
    validate_role_action(&sweep_action, &sup_obs_no_wards),
    Err(RoleActionError::OracleSweepOnCooldown)
  );

  // 4. Role mismatch rejection
  let bot_action = RoleAction::new(
    MatchRole::BotCarry,
    RoleIntent::Bot(BotCarryIntent::FarmWaveSafely),
  );
  assert_eq!(
    validate_role_action(&bot_action, &sup_obs_no_wards),
    Err(RoleActionError::RoleMismatch)
  );
}

#[test]
fn role_debrief_kpi_composite_and_markdown_generation() {
  let top_kpis = RoleKpis::TopLaner {
    side_lane_pressure_bp: 8000,
    structure_damage_bp: 8000,
    tp_flank_impact_bp: 8000,
    teamfight_presence_bp: 8000,
  };
  assert_eq!(top_kpis.compute_composite_rating_bp(), 8000);

  let debrief = RoleDebriefPerspective::new(
    MatchRole::TopLaner,
    TeamSide::Allied,
    top_kpis,
    vec![
      RoleCausalFactor::DecisiveFlank,
      RoleCausalFactor::SideLaneDemolition,
    ],
    "Flawless split-push and teleport execution.",
  );

  assert_eq!(debrief.composite_rating_bp, 8000);
  assert_eq!(debrief.performance_tier, RolePerformanceTier::Exceptional);

  let md = debrief.to_markdown();
  assert!(md.contains("### Role Debrief: top-laner (Allied)"));
  assert!(md.contains("- **Rating**: 8000 bp (exceptional)"));
  assert!(md.contains("- Side Lane Pressure: 8000 bp"));
  assert!(md.contains("- decisive-flank"));
  assert!(md.contains("- side-lane-demolition"));
}

#[test]
fn execute_all_five_canonical_role_scenarios() {
  let catalog = RoleScenarioCatalog::list_scenarios();
  assert_eq!(catalog.len(), 5);

  for def in catalog {
    let result =
      RoleScenarioCatalog::execute_scenario(def.scenario_id).expect("scenario execution");
    assert_eq!(result.scenario_id, def.scenario_id);
    assert_eq!(result.role, def.role);
    assert_eq!(result.team, def.team);
    assert_eq!(result.final_turn, def.initial_turn + 1);
    assert!(result.debrief.composite_rating_bp >= 5000);
    assert!(!result.debrief.causal_factors.is_empty());
    assert_ne!(
      result.initial_state_hash, result.final_state_hash,
      "State hash must advance after action execution"
    );

    // Verify Markdown generation works cleanly
    let md = result.debrief.to_markdown();
    assert!(md.contains(def.role.as_str()));
  }
}
