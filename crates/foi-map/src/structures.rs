//! Lane structures, turrets, inhibitors, nexus, and siege resolution for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use crate::kernel::{StateHash, hash_bytes};

use super::state::{FNV_OFFSET_BASIS, SectorSight};
use super::topology::{LaneId, LaneSector, MapLocation, TeamSide};

/// Structural tier in lane and base defense hierarchy.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StructureTier {
  /// Outer turret (Tier 1).
  OuterTurret,
  /// Inner turret (Tier 2).
  InnerTurret,
  /// Base inhibitor turret (Tier 3).
  InhibitorTurret,
  /// Lane inhibitor which restrains super minions.
  Inhibitor,
  /// Core Nexus structure whose destruction terminates the match.
  Nexus,
}

impl StructureTier {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::OuterTurret => "outer-turret",
      Self::InnerTurret => "inner-turret",
      Self::InhibitorTurret => "inhibitor-turret",
      Self::Inhibitor => "inhibitor",
      Self::Nexus => "nexus",
    }
  }

  pub const fn default_max_hp(self) -> u32 {
    match self {
      Self::OuterTurret => 3500,
      Self::InnerTurret => 4000,
      Self::InhibitorTurret => 4500,
      Self::Inhibitor => 3000,
      Self::Nexus => 6000,
    }
  }

  pub const fn is_turret(self) -> bool {
    matches!(
      self,
      Self::OuterTurret | Self::InnerTurret | Self::InhibitorTurret
    )
  }

  /// Sector a structure of this tier occupies on the coarse three-lane map.
  ///
  /// The map models nine lane sectors and two bases, while each team fields three
  /// lane tiers plus base structures, so tiers necessarily share sectors:
  ///
  /// - outer turrets of **both** teams stand in the same lane-centre sector, so one
  ///   sight line sees both teams' outer tier;
  /// - inner turrets stand on their own team's side of the lane (`NearTower` for the
  ///   allied team, `FarSide` for the opposing team);
  /// - inhibitor turrets, inhibitors, and the nexus all share their team's base sector.
  ///
  /// A lane tier that carries no lane falls back to its team's base sector rather than
  /// inventing a lane; `MatchStructureState::new_standard_map` always assigns one.
  pub const fn observed_sector(self, team: TeamSide, lane: Option<LaneId>) -> MapLocation {
    match self {
      Self::OuterTurret => match lane {
        Some(LaneId::Top) => MapLocation::TOP_CENTER,
        Some(LaneId::Mid) => MapLocation::MID_CENTER,
        Some(LaneId::Bot) => MapLocation::BOT_CENTER,
        None => MapLocation::Base(team),
      },
      Self::InnerTurret => match lane {
        Some(LaneId::Top) => Self::inner_turret_sector(LaneId::Top, team),
        Some(LaneId::Mid) => Self::inner_turret_sector(LaneId::Mid, team),
        Some(LaneId::Bot) => Self::inner_turret_sector(LaneId::Bot, team),
        None => MapLocation::Base(team),
      },
      Self::InhibitorTurret | Self::Inhibitor | Self::Nexus => MapLocation::Base(team),
    }
  }

  const fn inner_turret_sector(lane: LaneId, team: TeamSide) -> MapLocation {
    let sector = match team {
      TeamSide::Allied => LaneSector::NearTower,
      TeamSide::Opposing => LaneSector::FarSide,
    };
    MapLocation::Lane(lane, sector)
  }
}

impl fmt::Display for StructureTier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Operational status of a defensive structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructureStatus {
  /// Structure is standing and targetable.
  Standing { current_hp: u32, max_hp: u32 },
  /// Structure has been destroyed.
  Destroyed { destroyed_turn: u32 },
  /// Inhibitor is counting down to respawn.
  Respawning {
    destroyed_turn: u32,
    remaining_turns: u32,
  },
}

impl StructureStatus {
  pub const fn is_standing(&self) -> bool {
    matches!(self, Self::Standing { .. })
  }

  pub const fn is_destroyed(&self) -> bool {
    matches!(self, Self::Destroyed { .. })
  }

  pub const fn is_respawning(&self) -> bool {
    matches!(self, Self::Respawning { .. })
  }

  pub const fn current_hp(&self) -> u32 {
    match self {
      Self::Standing { current_hp, .. } => *current_hp,
      Self::Destroyed { .. } | Self::Respawning { .. } => 0,
    }
  }
}

/// Individual structure instance entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StructureEntry {
  pub team: TeamSide,
  pub lane: Option<LaneId>,
  pub tier: StructureTier,
  pub status: StructureStatus,
}

impl StructureEntry {
  pub const fn new(team: TeamSide, lane: Option<LaneId>, tier: StructureTier) -> Self {
    let max_hp = tier.default_max_hp();
    Self {
      team,
      lane,
      tier,
      status: StructureStatus::Standing {
        current_hp: max_hp,
        max_hp,
      },
    }
  }
}

/// Coarse health band of a standing structure that a team can see.
///
/// Fog costs precision, not just facts (`docs/decision_brief_20260830.md` decision D3):
/// a seen structure is reported as one of three bands, never as exact hit points. Exact
/// health stays latent host state, reachable through [`MatchStructureState`] by the host
/// and by research consumers, never through an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructureHealthBand {
  /// More than two thirds of maximum health remains.
  Pristine,
  /// More than one third, and at most two thirds, of maximum health remains.
  Chipped,
  /// One third of maximum health or less remains.
  Failing,
}

/// Inclusive upper bounds, in exact integer basis points of maximum health, for the
/// lower two bands. Fixed integers keep classification independent of tier-specific
/// arithmetic and of floating point entirely.
const FAILING_HEALTH_BP_MAX: u32 = 3_333;
const CHIPPED_HEALTH_BP_MAX: u32 = 6_666;

impl StructureHealthBand {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Pristine => "pristine",
      Self::Chipped => "chipped",
      Self::Failing => "failing",
    }
  }

  /// Classify `current_hp` against `max_hp` in exact integer basis points.
  ///
  /// A structure with no recorded maximum is treated as undamaged: no standard tier
  /// behaves that way, and the fallback keeps the function total.
  pub fn from_hp(current_hp: u32, max_hp: u32) -> Self {
    let health_bp = if max_hp == 0 {
      10_000
    } else {
      u32::try_from(u64::from(current_hp) * 10_000 / u64::from(max_hp)).unwrap_or(10_000)
    };
    if health_bp <= FAILING_HEALTH_BP_MAX {
      Self::Failing
    } else if health_bp <= CHIPPED_HEALTH_BP_MAX {
      Self::Chipped
    } else {
      Self::Pristine
    }
  }
}

/// What one team can observe about one defensive structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservedStructureStatus {
  /// Standing, and inside the team's sight. Health is a band, not a number.
  Standing { band: StructureHealthBand },
  /// Inside the team's sight and not standing. Covers destroyed and respawning
  /// structures alike: the coarse projection reports no respawn countdown.
  Destroyed,
  /// Outside the team's sight. No structure state is projected at all, not even
  /// whether the structure still stands.
  NotVisible,
}

impl ObservedStructureStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Standing { band } => band.as_str(),
      Self::Destroyed => "destroyed",
      Self::NotVisible => "not-visible",
    }
  }
}

/// One entry of a team-scoped defensive-structure observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservedStructure {
  pub side: TeamSide,
  pub tier: StructureTier,
  pub lane: Option<LaneId>,
  /// Sector the structure occupies under [`StructureTier::observed_sector`], so the
  /// player can tell where to look in order to confirm the projection.
  pub sector: MapLocation,
  pub status: ObservedStructureStatus,
}

impl ObservedStructure {
  pub const fn observed_sector(&self) -> MapLocation {
    self.sector
  }
}

/// Default turns required for a destroyed inhibitor to respawn.
pub const INHIBITOR_RESPAWN_TURNS: u32 = 5;

/// Errors that can occur during structure querying or siege resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructureError {
  StructureNotFound,
  StructureInvulnerable,
  StructureAlreadyDestroyed,
  InvalidSiegeTarget,
  ZeroDamage,
}

impl fmt::Display for StructureError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::StructureNotFound => write!(f, "targeted structure does not exist"),
      Self::StructureInvulnerable => write!(f, "targeted structure is currently invulnerable"),
      Self::StructureAlreadyDestroyed => {
        write!(f, "targeted structure has already been destroyed")
      }
      Self::InvalidSiegeTarget => write!(f, "invalid structure target specification"),
      Self::ZeroDamage => write!(f, "siege damage must be greater than zero"),
    }
  }
}

/// Authoritative state of all 26 map structures across both teams.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchStructureState {
  structures: Vec<StructureEntry>,
}

impl Default for MatchStructureState {
  fn default() -> Self {
    Self::new_standard_map()
  }
}

impl MatchStructureState {
  /// Create a fresh standard map with all 26 defensive structures standing at full HP.
  pub fn new_standard_map() -> Self {
    let mut structures = Vec::with_capacity(26);

    for &team in &[TeamSide::Allied, TeamSide::Opposing] {
      for &lane in &[LaneId::Top, LaneId::Mid, LaneId::Bot] {
        structures.push(StructureEntry::new(
          team,
          Some(lane),
          StructureTier::OuterTurret,
        ));
        structures.push(StructureEntry::new(
          team,
          Some(lane),
          StructureTier::InnerTurret,
        ));
        structures.push(StructureEntry::new(
          team,
          Some(lane),
          StructureTier::InhibitorTurret,
        ));
        structures.push(StructureEntry::new(
          team,
          Some(lane),
          StructureTier::Inhibitor,
        ));
      }
      structures.push(StructureEntry::new(team, None, StructureTier::Nexus));
    }

    Self { structures }
  }

  pub fn structures(&self) -> &[StructureEntry] {
    &self.structures
  }

  /// Project structure state for `team` through the sectors that team can see.
  ///
  /// `sight` must come from the single visibility rule,
  /// [`MatchMapState::sector_sight`](super::state::MatchMapState::sector_sight); this
  /// projection never decides on its own whether a sector is seen.
  ///
  /// A team always observes its **own** structures: a team never needs sight to know
  /// the state of its own defenses, and fog is what the opponent must buy. Own
  /// structures are still reported as coarse bands, so exact health never reaches a
  /// player projection either way. Entries keep the canonical structure order.
  pub fn observe_for(&self, team: TeamSide, sight: &SectorSight) -> Vec<ObservedStructure> {
    self
      .structures
      .iter()
      .map(|entry| {
        let sector = entry.tier.observed_sector(entry.team, entry.lane);
        let status = if entry.team == team || sight[sector.index()] {
          match entry.status {
            StructureStatus::Standing { current_hp, max_hp } => ObservedStructureStatus::Standing {
              band: StructureHealthBand::from_hp(current_hp, max_hp),
            },
            StructureStatus::Destroyed { .. } | StructureStatus::Respawning { .. } => {
              ObservedStructureStatus::Destroyed
            }
          }
        } else {
          ObservedStructureStatus::NotVisible
        };
        ObservedStructure {
          side: entry.team,
          tier: entry.tier,
          lane: entry.lane,
          sector,
          status,
        }
      })
      .collect()
  }

  pub fn get_structure(
    &self,
    team: TeamSide,
    lane: Option<LaneId>,
    tier: StructureTier,
  ) -> Option<&StructureEntry> {
    self
      .structures
      .iter()
      .find(|s| s.team == team && s.lane == lane && s.tier == tier)
  }

  pub fn get_structure_mut(
    &mut self,
    team: TeamSide,
    lane: Option<LaneId>,
    tier: StructureTier,
  ) -> Option<&mut StructureEntry> {
    self
      .structures
      .iter_mut()
      .find(|s| s.team == team && s.lane == lane && s.tier == tier)
  }

  /// Check if a structure is currently vulnerable to damage based on defense hierarchy.
  pub fn is_vulnerable(&self, team: TeamSide, lane: Option<LaneId>, tier: StructureTier) -> bool {
    let entry = match self.get_structure(team, lane, tier) {
      Some(e) => e,
      None => return false,
    };

    if !entry.status.is_standing() {
      return false;
    }

    match tier {
      StructureTier::OuterTurret => true,
      StructureTier::InnerTurret => {
        let outer = self.get_structure(team, lane, StructureTier::OuterTurret);
        outer.is_some_and(|o| o.status.is_destroyed())
      }
      StructureTier::InhibitorTurret => {
        let inner = self.get_structure(team, lane, StructureTier::InnerTurret);
        inner.is_some_and(|i| i.status.is_destroyed())
      }
      StructureTier::Inhibitor => {
        let inhib_turret = self.get_structure(team, lane, StructureTier::InhibitorTurret);
        inhib_turret.is_some_and(|it| it.status.is_destroyed())
      }
      StructureTier::Nexus => {
        // Nexus is vulnerable if AT LEAST ONE of the team's inhibitors is destroyed or respawning.
        LaneId::ALL.iter().any(|&l| {
          self
            .get_structure(team, Some(l), StructureTier::Inhibitor)
            .is_some_and(|inh| !inh.status.is_standing())
        })
      }
    }
  }

  /// Check if a team has active super minion wave pressure in a given lane.
  ///
  /// A team has super minions in `lane` if the opposing team's inhibitor in `lane` is destroyed or respawning.
  pub fn has_super_minions(&self, team: TeamSide, lane: LaneId) -> bool {
    let opposing_team = match team {
      TeamSide::Allied => TeamSide::Opposing,
      TeamSide::Opposing => TeamSide::Allied,
    };

    self
      .get_structure(opposing_team, Some(lane), StructureTier::Inhibitor)
      .is_some_and(|inh| !inh.status.is_standing())
  }

  /// Check if any team's Nexus has been destroyed, concluding the match.
  pub fn check_nexus_destroyed(&self) -> Option<TeamSide> {
    [TeamSide::Allied, TeamSide::Opposing]
      .iter()
      .find(|&&team| {
        self
          .get_structure(team, None, StructureTier::Nexus)
          .is_some_and(|nexus| nexus.status.is_destroyed())
      })
      .copied()
  }

  /// Count the total number of destroyed structures for a given team.
  pub fn destroyed_count_for_team(&self, team: TeamSide) -> usize {
    self
      .structures
      .iter()
      .filter(|s| s.team == team && !s.status.is_standing())
      .count()
  }

  /// Advance turn counter and tick inhibitor respawn countdown timers.
  pub fn tick_turn(&mut self) -> Vec<StructureEvent> {
    let mut events = Vec::new();

    for entry in &mut self.structures {
      if entry.tier == StructureTier::Inhibitor {
        match entry.status {
          StructureStatus::Respawning {
            remaining_turns: 0..=1,
            ..
          } => {
            let max_hp = StructureTier::Inhibitor.default_max_hp();
            entry.status = StructureStatus::Standing {
              current_hp: max_hp,
              max_hp,
            };
            events.push(StructureEvent::InhibitorRespawned {
              team: entry.team,
              lane: entry.lane.unwrap_or(LaneId::Mid),
            });
          }
          StructureStatus::Respawning {
            destroyed_turn,
            remaining_turns,
          } => {
            entry.status = StructureStatus::Respawning {
              destroyed_turn,
              remaining_turns: remaining_turns.saturating_sub(1),
            };
          }
          StructureStatus::Standing { .. } | StructureStatus::Destroyed { .. } => {}
        }
      }
    }

    events
  }

  /// Compute deterministic FNV-1a state hash over all 26 structure statuses.
  pub fn compute_hash(&self, turn: u32) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, &turn.to_le_bytes());

    for entry in &self.structures {
      let team_byte = match entry.team {
        TeamSide::Allied => 1u8,
        TeamSide::Opposing => 2u8,
      };
      hash = hash_bytes(hash, &[team_byte]);

      let lane_byte = match entry.lane {
        Some(LaneId::Top) => 1u8,
        Some(LaneId::Mid) => 2u8,
        Some(LaneId::Bot) => 3u8,
        None => 0u8,
      };
      hash = hash_bytes(hash, &[lane_byte]);

      let tier_byte = match entry.tier {
        StructureTier::OuterTurret => 1u8,
        StructureTier::InnerTurret => 2u8,
        StructureTier::InhibitorTurret => 3u8,
        StructureTier::Inhibitor => 4u8,
        StructureTier::Nexus => 5u8,
      };
      hash = hash_bytes(hash, &[tier_byte]);

      match entry.status {
        StructureStatus::Standing { current_hp, .. } => {
          hash = hash_bytes(hash, &[1u8]);
          hash = hash_bytes(hash, &current_hp.to_le_bytes());
        }
        StructureStatus::Destroyed { destroyed_turn } => {
          hash = hash_bytes(hash, &[2u8]);
          hash = hash_bytes(hash, &destroyed_turn.to_le_bytes());
        }
        StructureStatus::Respawning {
          remaining_turns, ..
        } => {
          hash = hash_bytes(hash, &[3u8]);
          hash = hash_bytes(hash, &remaining_turns.to_le_bytes());
        }
      }
    }

    StateHash::from_raw(hash)
  }
}

/// Intent for a siege engagement targeting or defending a structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SiegeIntent {
  /// Attack a target structure with specified siege power.
  AttackStructure {
    tier: StructureTier,
    lane: Option<LaneId>,
    raw_damage: u32,
  },
  /// Defend the lane structure, mitigating incoming damage by basis points.
  DefendStructure {
    lane: Option<LaneId>,
    mitigation_bp: u16,
  },
  /// Concede structure and fall back to next defense line.
  ConcedeStructure { lane: Option<LaneId> },
}

/// Structured causal events emitted during structure siege.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructureEvent {
  StructureDamaged {
    target_team: TeamSide,
    lane: Option<LaneId>,
    tier: StructureTier,
    damage_dealt: u32,
    remaining_hp: u32,
  },
  StructureDestroyed {
    target_team: TeamSide,
    lane: Option<LaneId>,
    tier: StructureTier,
    turn: u32,
  },
  InhibitorRespawned {
    team: TeamSide,
    lane: LaneId,
  },
  SuperMinionsSpawned {
    beneficiary_team: TeamSide,
    lane: LaneId,
  },
  NexusDestroyed {
    losing_team: TeamSide,
    turn: u32,
  },
}

/// Attributed causal effects resulting from structure siege transitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructureEffect {
  StructureHealthReduced {
    team: TeamSide,
    tier: StructureTier,
    hp_lost: u32,
  },
  LaneVulnerabilityExpanded {
    team: TeamSide,
    lane: LaneId,
    next_vulnerable_tier: StructureTier,
  },
  SuperMinionPressureApplied {
    team: TeamSide,
    lane: LaneId,
    pressure_bp: u32,
  },
  MatchConcluded {
    winning_team: TeamSide,
    turn: u32,
  },
}

/// Complete transition outcome of a structure siege action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructureSiegeResult {
  pub turn: u32,
  pub attacker_team: TeamSide,
  pub target_team: TeamSide,
  pub target_tier: StructureTier,
  pub target_lane: Option<LaneId>,
  pub effective_damage: u32,
  pub structure_destroyed: bool,
  pub super_minions_spawned: bool,
  pub match_concluded: bool,
  pub events: Vec<StructureEvent>,
  pub effects: Vec<StructureEffect>,
  pub state_hash: StateHash,
}

/// Pure deterministic transition for resolving structure siege actions.
pub fn transition_structure_siege(
  turn: u32,
  structures: &mut MatchStructureState,
  attacker_team: TeamSide,
  attack_intent: SiegeIntent,
  defense_intent: Option<SiegeIntent>,
) -> Result<StructureSiegeResult, StructureError> {
  let (tier, lane, raw_damage) = match attack_intent {
    SiegeIntent::AttackStructure {
      tier,
      lane,
      raw_damage,
    } => (tier, lane, raw_damage),
    _ => return Err(StructureError::InvalidSiegeTarget),
  };

  if raw_damage == 0 {
    return Err(StructureError::ZeroDamage);
  }

  let target_team = match attacker_team {
    TeamSide::Allied => TeamSide::Opposing,
    TeamSide::Opposing => TeamSide::Allied,
  };

  if !structures.is_vulnerable(target_team, lane, tier) {
    return Err(StructureError::StructureInvulnerable);
  }

  let mitigation_bp = match defense_intent {
    Some(SiegeIntent::DefendStructure {
      lane: def_lane,
      mitigation_bp,
    }) if def_lane == lane => mitigation_bp.min(8000), // Cap defense mitigation at 80%
    _ => 0,
  };

  let damage_multiplier = 10_000u32.saturating_sub(u32::from(mitigation_bp));
  let effective_damage_u64 = u64::from(raw_damage)
    .saturating_mul(u64::from(damage_multiplier))
    .saturating_add(5000)
    / 10_000;
  let effective_damage = u32::try_from(effective_damage_u64)
    .unwrap_or(u32::MAX)
    .max(1);

  let mut events = Vec::new();
  let mut effects = Vec::new();
  let mut structure_destroyed = false;
  let mut super_minions_spawned = false;
  let mut match_concluded = false;

  let entry = structures
    .get_structure_mut(target_team, lane, tier)
    .ok_or(StructureError::StructureNotFound)?;

  match entry.status {
    StructureStatus::Standing { current_hp, .. } => {
      if effective_damage >= current_hp {
        // Structure is destroyed
        structure_destroyed = true;
        if tier == StructureTier::Inhibitor {
          entry.status = StructureStatus::Respawning {
            destroyed_turn: turn,
            remaining_turns: INHIBITOR_RESPAWN_TURNS,
          };
          super_minions_spawned = true;
          let lane_id = lane.unwrap_or(LaneId::Mid);
          events.push(StructureEvent::SuperMinionsSpawned {
            beneficiary_team: attacker_team,
            lane: lane_id,
          });
          effects.push(StructureEffect::SuperMinionPressureApplied {
            team: attacker_team,
            lane: lane_id,
            pressure_bp: 3000,
          });
        } else {
          entry.status = StructureStatus::Destroyed {
            destroyed_turn: turn,
          };
        }

        events.push(StructureEvent::StructureDestroyed {
          target_team,
          lane,
          tier,
          turn,
        });
        effects.push(StructureEffect::StructureHealthReduced {
          team: target_team,
          tier,
          hp_lost: current_hp,
        });

        if tier == StructureTier::Nexus {
          match_concluded = true;
          events.push(StructureEvent::NexusDestroyed {
            losing_team: target_team,
            turn,
          });
          effects.push(StructureEffect::MatchConcluded {
            winning_team: attacker_team,
            turn,
          });
        }
      } else {
        let new_hp = current_hp.saturating_sub(effective_damage);
        entry.status = StructureStatus::Standing {
          current_hp: new_hp,
          max_hp: tier.default_max_hp(),
        };

        events.push(StructureEvent::StructureDamaged {
          target_team,
          lane,
          tier,
          damage_dealt: effective_damage,
          remaining_hp: new_hp,
        });
        effects.push(StructureEffect::StructureHealthReduced {
          team: target_team,
          tier,
          hp_lost: effective_damage,
        });
      }
    }
    _ => return Err(StructureError::StructureAlreadyDestroyed),
  }

  let state_hash = structures.compute_hash(turn);

  Ok(StructureSiegeResult {
    turn,
    attacker_team,
    target_team,
    target_tier: tier,
    target_lane: lane,
    effective_damage,
    structure_destroyed,
    super_minions_spawned,
    match_concluded,
    events,
    effects,
    state_hash,
  })
}
