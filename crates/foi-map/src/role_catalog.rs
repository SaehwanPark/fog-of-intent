//! Canonical benchmark scenarios demonstrating role-specific observations, actions, and debriefs for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use crate::kernel::{ActorId, StateHash};

use super::composition::MatchRole;
use super::objective::{ObjectiveKind, ObjectiveStatus};
use super::role_action::{
  BotCarryIntent, JungleIntent, MidIntent, RoleAction, RoleIntent, SupportIntent, TopIntent,
  validate_role_action,
};
use super::role_debrief::{RoleCausalFactor, RoleDebriefPerspective, RoleKpis};
use super::role_observation::{
  BotCarryContext, JunglerContext, MidLanerContext, RoleMatchObservation, RoleSpecificContext,
  SupportContext, TopLanerContext, WaveStateSummary,
};
use super::state::MatchMapState;
use super::topology::{LaneId, LaneSector, MapLocation, RiverSide, TeamSide};
use super::travel::ActorLocation;
use super::vision::VisionCoverage;

pub const M9_ROLE_SCENARIO_CATALOG_SCHEMA_V1: &str = "m9-role-scenario-catalog-v1";

/// Definition of a benchmark role-specific scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoleScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub role: MatchRole,
  pub team: TeamSide,
  pub initial_turn: u32,
  pub description: &'static str,
}

/// Execution outcome of running a canonical role scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub role: MatchRole,
  pub team: TeamSide,
  pub final_turn: u32,
  pub action_taken: RoleIntent,
  pub debrief: RoleDebriefPerspective,
  pub initial_state_hash: StateHash,
  pub final_state_hash: StateHash,
}

/// Catalog of canonical benchmark scenarios for all 5 match roles in M9.
pub struct RoleScenarioCatalog;

impl RoleScenarioCatalog {
  pub const SCENARIO_TOP_TELEPORT_FLANK: RoleScenarioDefinition = RoleScenarioDefinition {
    scenario_id: "scenario-top-teleport-flank-v1",
    name: "Top Laner Side-Lane TP Flank at Dragon Contest",
    role: MatchRole::TopLaner,
    team: TeamSide::Allied,
    initial_turn: 14,
    description: "Top laner pushes side lane, validates teleport availability, and executes a decisive flank onto Bot River dragon fight.",
  };

  pub const SCENARIO_JUNGLER_OBJECTIVE_STEAL: RoleScenarioDefinition = RoleScenarioDefinition {
    scenario_id: "scenario-jungler-objective-steal-v1",
    name: "Jungler Fog Infiltration & Smite Secure",
    role: MatchRole::Jungler,
    team: TeamSide::Allied,
    initial_turn: 12,
    description: "Jungler maneuvers through fog of war into Bot River, times Smite burst against opponent contest, and secures Drake.",
  };

  pub const SCENARIO_MID_ROAM_CONVERSION: RoleScenarioDefinition = RoleScenarioDefinition {
    scenario_id: "scenario-mid-roam-conversion-v1",
    name: "Mid Laner Wave Push & Bot Roam Dive",
    role: MatchRole::MidLaner,
    team: TeamSide::Allied,
    initial_turn: 8,
    description: "Mid laner establishes mid priority with wave shove, roams bot lane under vision cover, and executes a 3v2 dive.",
  };

  pub const SCENARIO_BOT_HYPERCARRY_SCALING: RoleScenarioDefinition = RoleScenarioDefinition {
    scenario_id: "scenario-bot-hypercarry-scaling-v1",
    name: "Bot Hypercarry Late-Game Kiting & DPS Output",
    role: MatchRole::BotCarry,
    team: TeamSide::Allied,
    initial_turn: 22,
    description: "Bot carry maintains disciplined backline positioning, kites diving frontline, and delivers match-winning sustained DPS.",
  };

  pub const SCENARIO_SUPPORT_VISION_SETUP_PEEL: RoleScenarioDefinition = RoleScenarioDefinition {
    scenario_id: "scenario-support-vision-setup-peel-v1",
    name: "Support River De-Ward & Carry Peel Defense",
    role: MatchRole::Support,
    team: TeamSide::Allied,
    initial_turn: 16,
    description: "Support uses Oracle Lens to clear enemy river vision, sets up deep wards, and peels assassins off BotCarry in teamfight.",
  };

  pub const ALL_SCENARIOS: [RoleScenarioDefinition; 5] = [
    Self::SCENARIO_TOP_TELEPORT_FLANK,
    Self::SCENARIO_JUNGLER_OBJECTIVE_STEAL,
    Self::SCENARIO_MID_ROAM_CONVERSION,
    Self::SCENARIO_BOT_HYPERCARRY_SCALING,
    Self::SCENARIO_SUPPORT_VISION_SETUP_PEEL,
  ];

  pub fn list_scenarios() -> &'static [RoleScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static RoleScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Execute a canonical role benchmark scenario deterministically.
  pub fn execute_scenario(scenario_id: &str) -> Result<RoleScenarioExecutionResult, &'static str> {
    match scenario_id {
      "scenario-top-teleport-flank-v1" => Self::execute_top_teleport_flank(),
      "scenario-jungler-objective-steal-v1" => Self::execute_jungler_objective_steal(),
      "scenario-mid-roam-conversion-v1" => Self::execute_mid_roam_conversion(),
      "scenario-bot-hypercarry-scaling-v1" => Self::execute_bot_hypercarry_scaling(),
      "scenario-support-vision-setup-peel-v1" => Self::execute_support_vision_setup_peel(),
      _ => Err("Unknown role scenario identifier"),
    }
  }

  fn execute_top_teleport_flank() -> Result<RoleScenarioExecutionResult, &'static str> {
    let top_actor = ActorId::new(1);
    let mut state = MatchMapState::new(
      14,
      vec![top_actor, ActorId::new(2)],
      vec![ActorId::new(10)],
      vec![
        (
          top_actor,
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Top, LaneSector::FarSide)),
        ),
        (
          ActorId::new(2),
          ActorLocation::Stationary(MapLocation::River(RiverSide::BotRiver)),
        ),
        (
          ActorId::new(10),
          ActorLocation::Stationary(MapLocation::River(RiverSide::BotRiver)),
        ),
      ],
    );

    let initial_hash = state.hash();
    let map_obs = state.observe(top_actor).ok_or("Failed to observe map")?;

    let role_obs = RoleMatchObservation::new(
      MatchRole::TopLaner,
      top_actor,
      TeamSide::Allied,
      14,
      map_obs,
      RoleSpecificContext::TopLaner(TopLanerContext {
        top_wave_state: WaveStateSummary::CrashingToEnemy,
        top_sector: LaneSector::FarSide,
        top_river_objective_status: Some(ObjectiveStatus::Unspawned {
          turns_until_spawn: 4,
        }),
        teleport_ready: true,
        side_lane_pressure_bp: 8500,
      }),
    )?;

    let intent = TopIntent::TeleportFlank {
      target_location: MapLocation::River(RiverSide::BotRiver),
    };
    let action = RoleAction::new(MatchRole::TopLaner, RoleIntent::Top(intent));
    validate_role_action(&action, &role_obs)
      .map_err(|_| "Validation failed for TopLaner TP action")?;

    // Execute TP move
    state.set_actor_location(
      top_actor,
      ActorLocation::Stationary(MapLocation::River(RiverSide::BotRiver)),
    );
    state.advance_turn();
    let final_hash = state.hash();

    let debrief = RoleDebriefPerspective::new(
      MatchRole::TopLaner,
      TeamSide::Allied,
      RoleKpis::TopLaner {
        side_lane_pressure_bp: 8500,
        structure_damage_bp: 7000,
        tp_flank_impact_bp: 9500,
        teamfight_presence_bp: 8000,
      },
      vec![
        RoleCausalFactor::DecisiveFlank,
        RoleCausalFactor::SideLaneDemolition,
      ],
      "TopLaner drew side-lane pressure, then executed an uncontested TP flank to secure Dragon.",
    );

    Ok(RoleScenarioExecutionResult {
      scenario_id: "scenario-top-teleport-flank-v1",
      role: MatchRole::TopLaner,
      team: TeamSide::Allied,
      final_turn: state.turn(),
      action_taken: RoleIntent::Top(intent),
      debrief,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn execute_jungler_objective_steal() -> Result<RoleScenarioExecutionResult, &'static str> {
    let jg_actor = ActorId::new(2);
    let mut state = MatchMapState::new(
      12,
      vec![jg_actor],
      vec![ActorId::new(12)],
      vec![
        (
          jg_actor,
          ActorLocation::Stationary(MapLocation::River(RiverSide::BotRiver)),
        ),
        (
          ActorId::new(12),
          ActorLocation::Stationary(MapLocation::River(RiverSide::BotRiver)),
        ),
      ],
    );

    let initial_hash = state.hash();
    let map_obs = state.observe(jg_actor).ok_or("Failed to observe map")?;

    let role_obs = RoleMatchObservation::new(
      MatchRole::Jungler,
      jg_actor,
      TeamSide::Allied,
      12,
      map_obs,
      RoleSpecificContext::Jungler(JunglerContext {
        camps_cleared_ratio_bp: 7500,
        smite_ready: true,
        top_objective_timer: None,
        bot_objective_timer: Some(0), // active
        gank_opportunities: vec![(LaneId::Mid, 6000), (LaneId::Bot, 8000)],
      }),
    )?;

    let intent = JungleIntent::SecureNeutralObjective {
      kind: ObjectiveKind::BotRiverObjective,
    };
    let action = RoleAction::new(MatchRole::Jungler, RoleIntent::Jungle(intent));
    validate_role_action(&action, &role_obs)
      .map_err(|_| "Validation failed for Jungler Smite secure")?;

    state.advance_turn();
    let final_hash = state.hash();

    let debrief = RoleDebriefPerspective::new(
      MatchRole::Jungler,
      TeamSide::Allied,
      RoleKpis::Jungler {
        objective_secure_rate_bp: 9000,
        gank_conversion_rate_bp: 7500,
        jungle_efficiency_bp: 8000,
        counter_jungle_bp: 6500,
      },
      vec![
        RoleCausalFactor::ObjectiveSecuredSmite,
        RoleCausalFactor::GankConverted,
      ],
      "Jungler entered contest under pressure, landed exact smite burst, and secured Dragon.",
    );

    Ok(RoleScenarioExecutionResult {
      scenario_id: "scenario-jungler-objective-steal-v1",
      role: MatchRole::Jungler,
      team: TeamSide::Allied,
      final_turn: state.turn(),
      action_taken: RoleIntent::Jungle(intent),
      debrief,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn execute_mid_roam_conversion() -> Result<RoleScenarioExecutionResult, &'static str> {
    let mid_actor = ActorId::new(3);
    let mut state = MatchMapState::new(
      8,
      vec![mid_actor, ActorId::new(4)],
      vec![ActorId::new(14)],
      vec![
        (
          mid_actor,
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Mid, LaneSector::Center)),
        ),
        (
          ActorId::new(4),
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Bot, LaneSector::NearTower)),
        ),
        (
          ActorId::new(14),
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Bot, LaneSector::NearTower)),
        ),
      ],
    );

    let initial_hash = state.hash();
    let map_obs = state.observe(mid_actor).ok_or("Failed to observe map")?;

    let role_obs = RoleMatchObservation::new(
      MatchRole::MidLaner,
      mid_actor,
      TeamSide::Allied,
      8,
      map_obs,
      RoleSpecificContext::MidLaner(MidLanerContext {
        mid_wave_state: WaveStateSummary::CrashingToEnemy,
        roam_threat_level_bp: 2500,
        top_river_vision: VisionCoverage::ConcealedInFog,
        bot_river_vision: VisionCoverage::FullVision,
        lane_priority: true,
      }),
    )?;

    let intent = MidIntent::PushAndRoam {
      target_lane: LaneId::Bot,
    };
    let action = RoleAction::new(MatchRole::MidLaner, RoleIntent::Mid(intent));
    validate_role_action(&action, &role_obs)
      .map_err(|_| "Validation failed for MidLaner PushAndRoam")?;

    state.set_actor_location(
      mid_actor,
      ActorLocation::Stationary(MapLocation::Lane(LaneId::Bot, LaneSector::FarSide)),
    );
    state.advance_turn();
    let final_hash = state.hash();

    let debrief = RoleDebriefPerspective::new(
      MatchRole::MidLaner,
      TeamSide::Allied,
      RoleKpis::MidLaner {
        roam_impact_bp: 9000,
        lane_priority_bp: 8500,
        objective_damage_bp: 6000,
        pick_conversion_bp: 8500,
      },
      vec![
        RoleCausalFactor::RoamAssistedKill,
        RoleCausalFactor::GankConverted,
      ],
      "MidLaner shoved mid wave for priority, executed a clean 3v2 Bot dive, converting two kills.",
    );

    Ok(RoleScenarioExecutionResult {
      scenario_id: "scenario-mid-roam-conversion-v1",
      role: MatchRole::MidLaner,
      team: TeamSide::Allied,
      final_turn: state.turn(),
      action_taken: RoleIntent::Mid(intent),
      debrief,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn execute_bot_hypercarry_scaling() -> Result<RoleScenarioExecutionResult, &'static str> {
    let adc_actor = ActorId::new(4);
    let mut state = MatchMapState::new(
      22,
      vec![adc_actor, ActorId::new(5)],
      vec![ActorId::new(14), ActorId::new(15)],
      vec![
        (
          adc_actor,
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Mid, LaneSector::Center)),
        ),
        (
          ActorId::new(5),
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Mid, LaneSector::Center)),
        ),
        (
          ActorId::new(14),
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Mid, LaneSector::Center)),
        ),
        (
          ActorId::new(15),
          ActorLocation::Stationary(MapLocation::Lane(LaneId::Mid, LaneSector::Center)),
        ),
      ],
    );

    let initial_hash = state.hash();
    let map_obs = state.observe(adc_actor).ok_or("Failed to observe map")?;

    let role_obs = RoleMatchObservation::new(
      MatchRole::BotCarry,
      adc_actor,
      TeamSide::Allied,
      22,
      map_obs,
      RoleSpecificContext::BotCarry(BotCarryContext {
        bot_wave_state: WaveStateSummary::FrozenAtCenter,
        farm_cs_score: 245,
        support_tethered: true,
        dragon_contest_ready: true,
        positioning_safety_bp: 9000,
      }),
    )?;

    let intent = BotCarryIntent::DPSFocusFrontline;
    let action = RoleAction::new(MatchRole::BotCarry, RoleIntent::Bot(intent));
    validate_role_action(&action, &role_obs)
      .map_err(|_| "Validation failed for BotCarry DPS action")?;

    state.advance_turn();
    let final_hash = state.hash();

    let debrief = RoleDebriefPerspective::new(
      MatchRole::BotCarry,
      TeamSide::Allied,
      RoleKpis::BotCarry {
        dps_efficiency_bp: 9200,
        farming_parity_bp: 8800,
        positioning_safety_bp: 9000,
        survivability_bp: 9500,
      },
      vec![RoleCausalFactor::SafeDPSOutput],
      "BotCarry scaled into late game, maintained flawless positioning, and shredded opposing frontline.",
    );

    Ok(RoleScenarioExecutionResult {
      scenario_id: "scenario-bot-hypercarry-scaling-v1",
      role: MatchRole::BotCarry,
      team: TeamSide::Allied,
      final_turn: state.turn(),
      action_taken: RoleIntent::Bot(intent),
      debrief,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn execute_support_vision_setup_peel() -> Result<RoleScenarioExecutionResult, &'static str> {
    let sup_actor = ActorId::new(5);
    let mut state = MatchMapState::new(
      16,
      vec![sup_actor, ActorId::new(4)],
      vec![ActorId::new(15)],
      vec![
        (
          sup_actor,
          ActorLocation::Stationary(MapLocation::River(RiverSide::TopRiver)),
        ),
        (
          ActorId::new(4),
          ActorLocation::Stationary(MapLocation::River(RiverSide::TopRiver)),
        ),
        (
          ActorId::new(15),
          ActorLocation::Stationary(MapLocation::River(RiverSide::TopRiver)),
        ),
      ],
    );

    let initial_hash = state.hash();
    let map_obs = state.observe(sup_actor).ok_or("Failed to observe map")?;

    let role_obs = RoleMatchObservation::new(
      MatchRole::Support,
      sup_actor,
      TeamSide::Allied,
      16,
      map_obs,
      RoleSpecificContext::Support(SupportContext {
        wards_available: 3,
        oracle_sweep_ready: true,
        protected_role: MatchRole::BotCarry,
        engage_readiness_bp: 7500,
        contested_river_side: Some(RiverSide::TopRiver),
      }),
    )?;

    let intent = SupportIntent::ClearEnemyVision {
      location: MapLocation::River(RiverSide::TopRiver),
    };
    let action = RoleAction::new(MatchRole::Support, RoleIntent::Support(intent));
    validate_role_action(&action, &role_obs)
      .map_err(|_| "Validation failed for Support ClearEnemyVision")?;

    state.advance_turn();
    let final_hash = state.hash();

    let debrief = RoleDebriefPerspective::new(
      MatchRole::Support,
      TeamSide::Allied,
      RoleKpis::Support {
        vision_score_bp: 9500,
        peel_effectiveness_bp: 9000,
        engagement_conversion_bp: 7000,
        assist_participation_bp: 8500,
      },
      vec![
        RoleCausalFactor::VisionDominance,
        RoleCausalFactor::PeelSuccess,
      ],
      "Support established complete river vision denial and neutralized diving assassin on BotCarry.",
    );

    Ok(RoleScenarioExecutionResult {
      scenario_id: "scenario-support-vision-setup-peel-v1",
      role: MatchRole::Support,
      team: TeamSide::Allied,
      final_turn: state.turn(),
      action_taken: RoleIntent::Support(intent),
      debrief,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }
}
