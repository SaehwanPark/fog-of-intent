//! Neutral objective cycles, spawning state machines, and objective health pools for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use super::topology::{MapLocation, TeamSide};

/// Canonical neutral objective kinds on the three-lane map.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObjectiveKind {
  /// Top river objective (Void Herald / Baron tier).
  TopRiverObjective,
  /// Bot river objective (Elemental Drake tier).
  BotRiverObjective,
}

impl ObjectiveKind {
  pub const ALL: [Self; 2] = [Self::TopRiverObjective, Self::BotRiverObjective];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::TopRiverObjective => "top-river-herald",
      Self::BotRiverObjective => "bot-river-drake",
    }
  }

  pub const fn location(self) -> MapLocation {
    match self {
      Self::TopRiverObjective => MapLocation::TOP_RIVER,
      Self::BotRiverObjective => MapLocation::BOT_RIVER,
    }
  }

  pub const fn default_max_health(self) -> u32 {
    match self {
      Self::TopRiverObjective => 5000,
      Self::BotRiverObjective => 3500,
    }
  }

  pub const fn default_initial_spawn_turn(self) -> u32 {
    match self {
      Self::TopRiverObjective => 6,
      Self::BotRiverObjective => 4,
    }
  }

  pub const fn default_respawn_turns(self) -> u32 {
    match self {
      Self::TopRiverObjective => 6,
      Self::BotRiverObjective => 5,
    }
  }

  pub const fn strategic_value_bp(self) -> u32 {
    match self {
      Self::TopRiverObjective => 4500,
      Self::BotRiverObjective => 4000,
    }
  }
}

impl fmt::Display for ObjectiveKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Lifecycle status of an individual map objective.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectiveStatus {
  /// Objective is not yet on the map; counting down to spawn.
  Unspawned { turns_until_spawn: u32 },
  /// Objective is actively spawned and targetable at its river location.
  Active {
    current_health: u32,
    max_health: u32,
    engaged_by: Option<TeamSide>,
  },
  /// Objective has been secured and is on respawn cooldown.
  Secured {
    secured_by: TeamSide,
    secured_turn: u32,
    turns_until_respawn: u32,
  },
}

impl ObjectiveStatus {
  pub const fn is_active(&self) -> bool {
    matches!(self, Self::Active { .. })
  }

  pub const fn is_unspawned(&self) -> bool {
    matches!(self, Self::Unspawned { .. })
  }

  pub const fn is_secured(&self) -> bool {
    matches!(self, Self::Secured { .. })
  }

  pub const fn current_health(&self) -> Option<u32> {
    match self {
      Self::Active { current_health, .. } => Some(*current_health),
      _ => None,
    }
  }
}

/// Tracked state entry for an objective kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectiveEntry {
  pub kind: ObjectiveKind,
  pub status: ObjectiveStatus,
  pub secure_count_allied: u32,
  pub secure_count_opposing: u32,
}

impl ObjectiveEntry {
  pub const fn new_unspawned(kind: ObjectiveKind) -> Self {
    Self {
      kind,
      status: ObjectiveStatus::Unspawned {
        turns_until_spawn: kind.default_initial_spawn_turn(),
      },
      secure_count_allied: 0,
      secure_count_opposing: 0,
    }
  }

  pub const fn new_active(kind: ObjectiveKind, health: u32) -> Self {
    Self {
      kind,
      status: ObjectiveStatus::Active {
        current_health: health,
        max_health: kind.default_max_health(),
        engaged_by: None,
      },
      secure_count_allied: 0,
      secure_count_opposing: 0,
    }
  }
}

/// Errors occurring during objective lifecycle transitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectiveError {
  ObjectiveNotActive,
  InvalidDamageAmount,
}

impl fmt::Display for ObjectiveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ObjectiveNotActive => f.write_str("objective is not currently active"),
      Self::InvalidDamageAmount => f.write_str("damage amount must be greater than zero"),
    }
  }
}

/// Outcome of applying damage to an objective.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DamageOutcome {
  Damaged { remaining_health: u32 },
  Secured { secured_by: TeamSide },
}

/// Complete match objective state tracking all neutral objectives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchObjectiveState {
  entries: [ObjectiveEntry; 2],
}

impl Default for MatchObjectiveState {
  fn default() -> Self {
    Self::new()
  }
}

impl MatchObjectiveState {
  pub const fn new() -> Self {
    Self {
      entries: [
        ObjectiveEntry::new_unspawned(ObjectiveKind::TopRiverObjective),
        ObjectiveEntry::new_unspawned(ObjectiveKind::BotRiverObjective),
      ],
    }
  }

  pub const fn new_with_entries(entries: [ObjectiveEntry; 2]) -> Self {
    Self { entries }
  }

  pub fn get(&self, kind: ObjectiveKind) -> &ObjectiveEntry {
    match kind {
      ObjectiveKind::TopRiverObjective => &self.entries[0],
      ObjectiveKind::BotRiverObjective => &self.entries[1],
    }
  }

  pub fn get_mut(&mut self, kind: ObjectiveKind) -> &mut ObjectiveEntry {
    match kind {
      ObjectiveKind::TopRiverObjective => &mut self.entries[0],
      ObjectiveKind::BotRiverObjective => &mut self.entries[1],
    }
  }

  pub fn entries(&self) -> &[ObjectiveEntry; 2] {
    &self.entries
  }

  /// Advance turn counter by one tick, updating spawning and respawning timers.
  /// Returns a list of objective kinds that successfully spawned this turn.
  pub fn tick_turn(&mut self) -> Vec<ObjectiveKind> {
    let mut spawned = Vec::new();
    for entry in &mut self.entries {
      match entry.status {
        ObjectiveStatus::Unspawned { turns_until_spawn } => {
          if turns_until_spawn <= 1 {
            entry.status = ObjectiveStatus::Active {
              current_health: entry.kind.default_max_health(),
              max_health: entry.kind.default_max_health(),
              engaged_by: None,
            };
            spawned.push(entry.kind);
          } else {
            entry.status = ObjectiveStatus::Unspawned {
              turns_until_spawn: turns_until_spawn.saturating_sub(1),
            };
          }
        }
        ObjectiveStatus::Secured {
          secured_by,
          secured_turn,
          turns_until_respawn,
        } => {
          if turns_until_respawn <= 1 {
            entry.status = ObjectiveStatus::Active {
              current_health: entry.kind.default_max_health(),
              max_health: entry.kind.default_max_health(),
              engaged_by: None,
            };
            spawned.push(entry.kind);
          } else {
            entry.status = ObjectiveStatus::Secured {
              secured_by,
              secured_turn,
              turns_until_respawn: turns_until_respawn.saturating_sub(1),
            };
          }
        }
        ObjectiveStatus::Active { .. } => {}
      }
    }
    spawned
  }

  /// Apply direct damage to an active objective.
  pub fn apply_damage(
    &mut self,
    kind: ObjectiveKind,
    damage: u32,
    by_team: TeamSide,
    current_turn: u32,
  ) -> Result<DamageOutcome, ObjectiveError> {
    if damage == 0 {
      return Err(ObjectiveError::InvalidDamageAmount);
    }
    let entry = self.get_mut(kind);
    match entry.status {
      ObjectiveStatus::Active {
        current_health,
        max_health,
        ..
      } => {
        if damage >= current_health {
          match by_team {
            TeamSide::Allied => {
              entry.secure_count_allied = entry.secure_count_allied.saturating_add(1)
            }
            TeamSide::Opposing => {
              entry.secure_count_opposing = entry.secure_count_opposing.saturating_add(1)
            }
          }
          entry.status = ObjectiveStatus::Secured {
            secured_by: by_team,
            secured_turn: current_turn,
            turns_until_respawn: kind.default_respawn_turns(),
          };
          Ok(DamageOutcome::Secured {
            secured_by: by_team,
          })
        } else {
          let remaining_health = current_health.saturating_sub(damage);
          entry.status = ObjectiveStatus::Active {
            current_health: remaining_health,
            max_health,
            engaged_by: Some(by_team),
          };
          Ok(DamageOutcome::Damaged { remaining_health })
        }
      }
      _ => Err(ObjectiveError::ObjectiveNotActive),
    }
  }
}
