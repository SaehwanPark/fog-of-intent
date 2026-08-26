//! Role-specific observation projections and contextual awareness for M9 match roles.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use crate::kernel::ActorId;

use super::composition::MatchRole;
use super::objective::ObjectiveStatus;
use super::state::MatchMapObservation;
use super::topology::{LaneId, LaneSector, MapLocation, RiverSide, TeamSide};
use super::vision::VisionCoverage;

pub const M9_ROLE_OBSERVATION_SCHEMA_V1: &str = "m9-role-observation-v1";

/// Summary of wave equilibrium and minion pressure in a lane sector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaveStateSummary {
  /// Wave is pushing toward ally tower.
  PushingToAlly,
  /// Wave is frozen in equilibrium at center.
  FrozenAtCenter,
  /// Wave is crashing into opposing tower.
  CrashingToEnemy,
  /// Super minions are actively pushing.
  SuperMinionPressure,
}

impl WaveStateSummary {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::PushingToAlly => "pushing-to-ally",
      Self::FrozenAtCenter => "frozen-at-center",
      Self::CrashingToEnemy => "crashing-to-enemy",
      Self::SuperMinionPressure => "super-minion-pressure",
    }
  }
}

/// Tactical context specific to the TopLaner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopLanerContext {
  pub top_wave_state: WaveStateSummary,
  pub top_sector: LaneSector,
  pub top_river_objective_status: Option<ObjectiveStatus>,
  pub teleport_ready: bool,
  pub side_lane_pressure_bp: u16,
}

/// Tactical context specific to the Jungler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JunglerContext {
  pub camps_cleared_ratio_bp: u16,
  pub smite_ready: bool,
  pub top_objective_timer: Option<u32>,
  pub bot_objective_timer: Option<u32>,
  pub gank_opportunities: Vec<(LaneId, u16)>,
}

/// Tactical context specific to the MidLaner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MidLanerContext {
  pub mid_wave_state: WaveStateSummary,
  pub roam_threat_level_bp: u16,
  pub top_river_vision: VisionCoverage,
  pub bot_river_vision: VisionCoverage,
  pub lane_priority: bool,
}

/// Tactical context specific to the BotCarry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotCarryContext {
  pub bot_wave_state: WaveStateSummary,
  pub farm_cs_score: u32,
  pub support_tethered: bool,
  pub dragon_contest_ready: bool,
  pub positioning_safety_bp: u16,
}

/// Tactical context specific to the Support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportContext {
  pub wards_available: u8,
  pub oracle_sweep_ready: bool,
  pub protected_role: MatchRole,
  pub engage_readiness_bp: u16,
  pub contested_river_side: Option<RiverSide>,
}

/// Role-specific situational context wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoleSpecificContext {
  TopLaner(TopLanerContext),
  Jungler(JunglerContext),
  MidLaner(MidLanerContext),
  BotCarry(BotCarryContext),
  Support(SupportContext),
}

/// Actor-visible observation projection tailored to a specific team match role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleMatchObservation {
  pub observer_role: MatchRole,
  pub observer: ActorId,
  pub observer_team: TeamSide,
  pub turn: u32,
  pub map_observation: MatchMapObservation,
  pub context: RoleSpecificContext,
}

impl RoleMatchObservation {
  pub fn new(
    observer_role: MatchRole,
    observer: ActorId,
    observer_team: TeamSide,
    turn: u32,
    map_observation: MatchMapObservation,
    context: RoleSpecificContext,
  ) -> Result<Self, &'static str> {
    // Validate role and context match
    match (&observer_role, &context) {
      (MatchRole::TopLaner, RoleSpecificContext::TopLaner(_))
      | (MatchRole::Jungler, RoleSpecificContext::Jungler(_))
      | (MatchRole::MidLaner, RoleSpecificContext::MidLaner(_))
      | (MatchRole::BotCarry, RoleSpecificContext::BotCarry(_))
      | (MatchRole::Support, RoleSpecificContext::Support(_)) => Ok(Self {
        observer_role,
        observer,
        observer_team,
        turn,
        map_observation,
        context,
      }),
      _ => Err("Role mismatch between observer_role and context"),
    }
  }

  pub fn self_location(&self) -> MapLocation {
    self.map_observation.self_location.current_location()
  }

  pub fn is_in_transit(&self) -> bool {
    self.map_observation.self_location.is_in_transit()
  }
}
