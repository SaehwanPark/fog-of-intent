//! Role-specific actions, tactical intents, and domain validation for M9 match roles.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use super::composition::MatchRole;
use super::objective::ObjectiveKind;
use super::role_observation::{RoleMatchObservation, RoleSpecificContext};
use super::topology::{JungleSide, LaneId, LaneSector, MapLocation, RiverSide};

pub const M9_ROLE_ACTION_SCHEMA_V1: &str = "m9-role-action-v1";

/// Specialized tactical intent for the TopLaner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopIntent {
  /// Split push a specific side-lane sector.
  SplitPushSector { sector: LaneSector },
  /// Hold and freeze the minion wave near tower.
  HoldFreezeWave,
  /// Channel teleport flank to a targeted map location.
  TeleportFlank { target_location: MapLocation },
  /// Engage in a 1v1 duel against opposing laner.
  DuelOpponent,
  /// Group with team to contest a teamfight or objective.
  JoinTeamfight,
}

impl TopIntent {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SplitPushSector { .. } => "split-push-sector",
      Self::HoldFreezeWave => "hold-freeze-wave",
      Self::TeleportFlank { .. } => "teleport-flank",
      Self::DuelOpponent => "duel-opponent",
      Self::JoinTeamfight => "join-teamfight",
    }
  }
}

/// Specialized tactical intent for the Jungler.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JungleIntent {
  /// Gank a targeted lane to secure a kill advantage.
  GankLane { lane: LaneId },
  /// Execute and secure a neutral objective using Smite burst.
  SecureNeutralObjective { kind: ObjectiveKind },
  /// Clear standard jungle camps in an allied quadrant.
  ClearJungleQuadrant { quadrant: JungleSide },
  /// Invade and steal camps in opposing jungle quadrant.
  InvadeEnemyJungle { quadrant: JungleSide },
  /// Shadow a threatened ally to execute a counter-gank.
  CounterGank { lane: LaneId },
}

impl JungleIntent {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::GankLane { .. } => "gank-lane",
      Self::SecureNeutralObjective { .. } => "secure-neutral-objective",
      Self::ClearJungleQuadrant { .. } => "clear-jungle-quadrant",
      Self::InvadeEnemyJungle { .. } => "invade-enemy-jungle",
      Self::CounterGank { .. } => "counter-gank",
    }
  }
}

/// Specialized tactical intent for the MidLaner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MidIntent {
  /// Shove the mid wave and roam toward a side lane.
  PushAndRoam { target_lane: LaneId },
  /// Zone opponents and defend/chip mid turret.
  ZoneMidTurret,
  /// Move to river and contest river vision/crabs.
  ContestRiverControl { river: RiverSide },
  /// Long-range poke and whittle enemy formation.
  PokeEnemyFormation,
  /// Single-target burst assassination focus.
  BurstTargetFocus,
}

impl MidIntent {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::PushAndRoam { .. } => "push-and-roam",
      Self::ZoneMidTurret => "zone-mid-turret",
      Self::ContestRiverControl { .. } => "contest-river-control",
      Self::PokeEnemyFormation => "poke-enemy-formation",
      Self::BurstTargetFocus => "burst-target-focus",
    }
  }
}

/// Specialized tactical intent for the BotCarry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BotCarryIntent {
  /// Safely farm minion waves near support/tower.
  FarmWaveSafely,
  /// Output sustained DPS focusing nearest enemy frontline.
  DPSFocusFrontline,
  /// Burst focus a high-priority enemy target.
  DPSBurstPriorityTarget,
  /// Position at max range to chip structure HP.
  SiegeTowerRange,
  /// Kite back and disengage from diving threats.
  KiteAndDisengage,
}

impl BotCarryIntent {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FarmWaveSafely => "farm-wave-safely",
      Self::DPSFocusFrontline => "dps-focus-frontline",
      Self::DPSBurstPriorityTarget => "dps-burst-priority-target",
      Self::SiegeTowerRange => "siege-tower-range",
      Self::KiteAndDisengage => "kite-and-disengage",
    }
  }
}

/// Specialized tactical intent for the Support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportIntent {
  /// Place vision wards to establish zone vision control.
  EstablishVisionZone { location: MapLocation },
  /// Clear enemy wards using Oracle Lens.
  ClearEnemyVision { location: MapLocation },
  /// Stand by and peel diving assassins off primary carry.
  PeelForCarry,
  /// Initiate hard crowd-control engagement on enemy squad.
  InitiateEngagement,
  /// Roam to assist an allied laner or jungler.
  RoamAssistLane { lane: LaneId },
}

impl SupportIntent {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::EstablishVisionZone { .. } => "establish-vision-zone",
      Self::ClearEnemyVision { .. } => "clear-enemy-vision",
      Self::PeelForCarry => "peel-for-carry",
      Self::InitiateEngagement => "initiate-engagement",
      Self::RoamAssistLane { .. } => "roam-assist-lane",
    }
  }
}

/// Role-specific tactical intent enum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoleIntent {
  Top(TopIntent),
  Jungle(JungleIntent),
  Mid(MidIntent),
  Bot(BotCarryIntent),
  Support(SupportIntent),
}

impl RoleIntent {
  pub const fn role(&self) -> MatchRole {
    match self {
      Self::Top(_) => MatchRole::TopLaner,
      Self::Jungle(_) => MatchRole::Jungler,
      Self::Mid(_) => MatchRole::MidLaner,
      Self::Bot(_) => MatchRole::BotCarry,
      Self::Support(_) => MatchRole::Support,
    }
  }

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Top(i) => i.as_str(),
      Self::Jungle(i) => i.as_str(),
      Self::Mid(i) => i.as_str(),
      Self::Bot(i) => i.as_str(),
      Self::Support(i) => i.as_str(),
    }
  }
}

impl fmt::Display for RoleIntent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// A validated tactical action submitted by an actor with a specific match role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoleAction {
  pub role: MatchRole,
  pub intent: RoleIntent,
}

impl RoleAction {
  pub const fn new(role: MatchRole, intent: RoleIntent) -> Self {
    Self { role, intent }
  }
}

/// Error type when validating a role-specific action against an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoleActionError {
  /// The submitted intent does not belong to the actor's assigned role.
  RoleMismatch,
  /// Teleport ability is currently on cooldown.
  TeleportUnavailable,
  /// Objective is not active/spawned to be contested.
  ObjectiveNotSpawned,
  /// No wards available in inventory to place.
  WardsUnavailable,
  /// Sweep tool is on cooldown.
  OracleSweepOnCooldown,
  /// Smite is unavailable or actor is not in smite range.
  SmiteUnavailable,
}

impl RoleActionError {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::RoleMismatch => "role-mismatch",
      Self::TeleportUnavailable => "teleport-unavailable",
      Self::ObjectiveNotSpawned => "objective-not-spawned",
      Self::WardsUnavailable => "wards-unavailable",
      Self::OracleSweepOnCooldown => "oracle-sweep-on-cooldown",
      Self::SmiteUnavailable => "smite-unavailable",
    }
  }
}

impl fmt::Display for RoleActionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Validate a role action against the actor's current role-specific observation.
pub fn validate_role_action(
  action: &RoleAction,
  observation: &RoleMatchObservation,
) -> Result<(), RoleActionError> {
  if action.role != observation.observer_role || action.intent.role() != action.role {
    return Err(RoleActionError::RoleMismatch);
  }

  match (action.intent, &observation.context) {
    (RoleIntent::Top(TopIntent::TeleportFlank { .. }), RoleSpecificContext::TopLaner(ctx))
      if !ctx.teleport_ready =>
    {
      return Err(RoleActionError::TeleportUnavailable);
    }
    (
      RoleIntent::Jungle(JungleIntent::SecureNeutralObjective { .. }),
      RoleSpecificContext::Jungler(ctx),
    ) if !ctx.smite_ready => {
      return Err(RoleActionError::SmiteUnavailable);
    }
    (
      RoleIntent::Support(SupportIntent::EstablishVisionZone { .. }),
      RoleSpecificContext::Support(ctx),
    ) if ctx.wards_available == 0 => {
      return Err(RoleActionError::WardsUnavailable);
    }
    (
      RoleIntent::Support(SupportIntent::ClearEnemyVision { .. }),
      RoleSpecificContext::Support(ctx),
    ) if !ctx.oracle_sweep_ready => {
      return Err(RoleActionError::OracleSweepOnCooldown);
    }
    _ => {}
  }

  Ok(())
}
