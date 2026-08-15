//! Dynamic map-level vision control, ward mechanics, and fog-of-war coverage for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use crate::kernel::ActorId;

use super::topology::{MapLocation, TeamSide};
use super::travel::ActorLocation;

pub const DEFAULT_WARD_DURATION_TURNS: u32 = 3;
pub const MAX_WARDS_PER_TEAM: usize = 10;

/// Vision coverage status of a specific map location for a team.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisionCoverage {
  /// Location is actively observed by an allied actor or active allied ward.
  FullVision,
  /// Location was previously observed, but is currently in fog of war.
  LastKnown { last_seen_turn: u32 },
  /// Location is unobserved and in the fog of war.
  ConcealedInFog,
}

impl VisionCoverage {
  pub const fn is_visible(self) -> bool {
    matches!(self, Self::FullVision)
  }
}

/// Vision grid representing visibility coverage across all 15 map locations for one team.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapVisionGrid {
  team: TeamSide,
  coverage: [VisionCoverage; 15],
}

impl MapVisionGrid {
  pub const fn new(team: TeamSide) -> Self {
    Self {
      team,
      coverage: [VisionCoverage::ConcealedInFog; 15],
    }
  }

  pub const fn team(&self) -> TeamSide {
    self.team
  }

  pub fn coverage_at(&self, location: MapLocation) -> VisionCoverage {
    self.coverage[location.index()]
  }

  pub fn is_visible(&self, location: MapLocation) -> bool {
    self.coverage[location.index()].is_visible()
  }

  pub fn set_coverage(&mut self, location: MapLocation, coverage: VisionCoverage) {
    self.coverage[location.index()] = coverage;
  }

  pub fn visible_locations(&self) -> Vec<MapLocation> {
    MapLocation::ALL_LOCATIONS
      .iter()
      .copied()
      .filter(|&loc| self.is_visible(loc))
      .collect()
  }
}

/// Individual vision ward deployed onto a discrete map location.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisionWard {
  pub ward_id: u32,
  pub team: TeamSide,
  pub location: MapLocation,
  pub placed_by: ActorId,
  pub placed_turn: u32,
  pub remaining_turns: u32,
}

/// Dynamic map vision state tracking deployed wards and visibility across the match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapVisionState {
  active_wards: Vec<VisionWard>,
  next_ward_id: u32,
}

impl Default for MapVisionState {
  fn default() -> Self {
    Self::new()
  }
}

impl MapVisionState {
  pub const fn new() -> Self {
    Self {
      active_wards: Vec::new(),
      next_ward_id: 1,
    }
  }

  pub fn active_wards(&self) -> &[VisionWard] {
    &self.active_wards
  }

  pub fn team_wards(&self, team: TeamSide) -> impl Iterator<Item = &VisionWard> {
    self.active_wards.iter().filter(move |w| w.team == team)
  }

  pub fn ward_at(&self, location: MapLocation, team: TeamSide) -> Option<&VisionWard> {
    self
      .active_wards
      .iter()
      .find(|w| w.location == location && w.team == team)
  }

  pub fn has_allied_ward(&self, location: MapLocation, team: TeamSide) -> bool {
    self.ward_at(location, team).is_some()
  }

  /// Place a vision ward at the specified location.
  pub fn place_ward(
    &mut self,
    team: TeamSide,
    placed_by: ActorId,
    location: MapLocation,
    current_turn: u32,
    duration_turns: u32,
  ) -> Result<VisionWard, VisionError> {
    let team_count = self.team_wards(team).count();
    if team_count >= MAX_WARDS_PER_TEAM {
      return Err(VisionError::WardCapacityExceeded);
    }
    if self.has_allied_ward(location, team) {
      return Err(VisionError::LocationAlreadyWardedByTeam);
    }

    let ward = VisionWard {
      ward_id: self.next_ward_id,
      team,
      location,
      placed_by,
      placed_turn: current_turn,
      remaining_turns: duration_turns,
    };
    self.next_ward_id = self.next_ward_id.saturating_add(1);
    self.active_wards.push(ward);
    Ok(ward)
  }

  /// Clear an opposing ward at the specified location (de-warding).
  pub fn clear_ward(
    &mut self,
    location: MapLocation,
    clearing_team: TeamSide,
  ) -> Result<VisionWard, VisionError> {
    let opposing_team = clearing_team.opposing();
    if let Some(pos) = self
      .active_wards
      .iter()
      .position(|w| w.location == location && w.team == opposing_team)
    {
      let cleared = self.active_wards.remove(pos);
      Ok(cleared)
    } else {
      Err(VisionError::NoOpposingWardAtLocation)
    }
  }

  /// Advance turn counter by one tick, decrementing ward durations and removing expired wards.
  /// Returns the list of expired wards.
  pub fn tick_turn(&mut self) -> Vec<VisionWard> {
    let mut expired = Vec::new();
    let mut i = 0;
    while i < self.active_wards.len() {
      if self.active_wards[i].remaining_turns <= 1 {
        expired.push(self.active_wards.remove(i));
      } else {
        self.active_wards[i].remaining_turns =
          self.active_wards[i].remaining_turns.saturating_sub(1);
        i += 1;
      }
    }
    expired
  }

  /// Compute full vision grid for a team given stationed/transit units and active wards.
  pub fn compute_team_vision(
    &self,
    team: TeamSide,
    actor_locations: &[(ActorId, ActorLocation, TeamSide)],
    current_turn: u32,
    prior_grid: Option<&MapVisionGrid>,
  ) -> MapVisionGrid {
    let mut grid = MapVisionGrid::new(team);

    // 1. Mark locations of active allied wards
    for ward in self.team_wards(team) {
      grid.set_coverage(ward.location, VisionCoverage::FullVision);
    }

    // 2. Mark locations of allied units (stationary or in-transit current location)
    for (_, loc, unit_team) in actor_locations {
      if *unit_team == team {
        grid.set_coverage(loc.current_location(), VisionCoverage::FullVision);
      }
    }

    // 3. Retain last-known visibility or conceal in fog for unobserved sectors
    for &loc in &MapLocation::ALL_LOCATIONS {
      if !grid.is_visible(loc) {
        let prev_coverage = prior_grid.map(|g| g.coverage_at(loc));
        match prev_coverage {
          Some(VisionCoverage::FullVision) | Some(VisionCoverage::LastKnown { .. }) => {
            grid.set_coverage(
              loc,
              VisionCoverage::LastKnown {
                last_seen_turn: current_turn.saturating_sub(1),
              },
            );
          }
          _ => {
            grid.set_coverage(loc, VisionCoverage::ConcealedInFog);
          }
        }
      }
    }

    grid
  }
}

/// Typed vision control commands executable by actors on the map.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisionCommand {
  /// Place a vision ward at the actor's current location.
  PlaceWard {
    actor: ActorId,
    location: MapLocation,
  },
  /// Clear an opposing ward at the actor's current location.
  ClearWard {
    actor: ActorId,
    location: MapLocation,
  },
}

/// Errors occurring during vision command validation and execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisionError {
  ActorNotInRange {
    required: MapLocation,
    actual: MapLocation,
  },
  LocationAlreadyWardedByTeam,
  NoOpposingWardAtLocation,
  WardCapacityExceeded,
}

impl fmt::Display for VisionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ActorNotInRange { required, actual } => {
        write!(
          f,
          "actor is at {actual} but vision action requires {required}"
        )
      }
      Self::LocationAlreadyWardedByTeam => {
        f.write_str("location is already warded by the allied team")
      }
      Self::NoOpposingWardAtLocation => {
        f.write_str("no opposing ward exists to clear at this location")
      }
      Self::WardCapacityExceeded => f.write_str("team active ward capacity limit reached"),
    }
  }
}
