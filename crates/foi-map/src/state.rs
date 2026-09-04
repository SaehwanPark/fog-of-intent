//! Match map state, FNV-1a state hashing, and actor-visible observation projections.

use crate::graph::distance_in_beats;
use crate::kernel::{ActorId, StateHash, hash_bytes};

pub(crate) const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
use super::topology::{MapLocation, TeamSide};
use super::travel::ActorLocation;

/// Number of discrete sectors on the three-lane map.
pub const MAP_LOCATION_COUNT: usize = MapLocation::ALL_LOCATIONS.len();

/// Which sectors one team can see, as flags indexed by [`MapLocation::index`].
///
/// Produced only by [`MatchMapState::sector_sight`]; consumers index it with a sector
/// they already hold rather than testing membership some other way.
pub type SectorSight = [bool; MAP_LOCATION_COUNT];

/// Vision status of an opponent actor in an observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpponentSighting {
  /// Opponent is directly observed at a known location.
  Observed {
    location: MapLocation,
    in_transit: bool,
  },
  /// Opponent is in fog of war, but was previously observed.
  LastKnown {
    location: MapLocation,
    last_seen_turn: u32,
  },
  /// Opponent is entirely in fog of war with no recent sighting.
  Unknown,
}

/// Actor-visible observation projection of the multi-lane map.
///
/// Ensures strict fog-of-war compliance: opponents in unobserved sectors are redacted to `Unknown`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchMapObservation {
  pub observer: ActorId,
  pub observer_team: TeamSide,
  pub turn: u32,
  pub self_location: ActorLocation,
  pub allied_locations: Vec<(ActorId, ActorLocation)>,
  pub opposing_sightings: Vec<(ActorId, OpponentSighting)>,
}

/// Authoritative multi-lane match map state owned exclusively by the simulation host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchMapState {
  turn: u32,
  actor_locations: Vec<(ActorId, ActorLocation)>,
  allied_actors: Vec<ActorId>,
  opposing_actors: Vec<ActorId>,
}

impl MatchMapState {
  pub fn new(
    turn: u32,
    allied_actors: Vec<ActorId>,
    opposing_actors: Vec<ActorId>,
    initial_locations: Vec<(ActorId, ActorLocation)>,
  ) -> Self {
    let mut actor_locations = initial_locations;
    actor_locations.sort_by_key(|(id, _)| id.value());
    Self {
      turn,
      actor_locations,
      allied_actors,
      opposing_actors,
    }
  }

  pub fn turn(&self) -> u32 {
    self.turn
  }

  pub fn actor_locations(&self) -> &[(ActorId, ActorLocation)] {
    &self.actor_locations
  }

  pub fn get_actor_location(&self, actor: ActorId) -> Option<&ActorLocation> {
    self
      .actor_locations
      .iter()
      .find(|(id, _)| *id == actor)
      .map(|(_, loc)| loc)
  }

  pub fn set_actor_location(&mut self, actor: ActorId, location: ActorLocation) {
    if let Some((_, loc)) = self.actor_locations.iter_mut().find(|(id, _)| *id == actor) {
      *loc = location;
    } else {
      self.actor_locations.push((actor, location));
      self.actor_locations.sort_by_key(|(id, _)| id.value());
    }
  }

  pub fn is_allied(&self, actor: ActorId) -> bool {
    self.allied_actors.contains(&actor)
  }

  pub fn is_opposing(&self, actor: ActorId) -> bool {
    self.opposing_actors.contains(&actor)
  }

  pub fn advance_turn(&mut self) {
    self.turn = self.turn.saturating_add(1);
  }

  /// Generate an actor-visible observation for an observer without leaking hidden state.
  ///
  /// Equivalent to [`Self::observe_with_wards`] with no ward coverage: only locations
  /// occupied by the observer's own team are seen.
  pub fn observe(&self, observer: ActorId) -> Option<MatchMapObservation> {
    self.observe_with_wards(observer, &[])
  }

  /// Generate an actor-visible observation where the observer's team additionally
  /// wards the locations named in `ward_coverage`.
  ///
  /// Ward state lives in [`super::vision::MapVisionState`], not in this type, so the
  /// caller supplies it. Coverage is paired with the owning `TeamSide` and entries
  /// for the other team are ignored: a caller holding latent enemy ward positions
  /// must not be able to spend them as allied sight. Resolution stays here so that
  /// hosts and renderers never re-derive visibility downstream of the projection.
  pub fn observe_with_wards(
    &self,
    observer: ActorId,
    ward_coverage: &[(TeamSide, MapLocation)],
  ) -> Option<MatchMapObservation> {
    let observer_team = if self.is_allied(observer) {
      TeamSide::Allied
    } else if self.is_opposing(observer) {
      TeamSide::Opposing
    } else {
      return None;
    };

    let self_location = self.get_actor_location(observer)?.clone();

    // Sight is resolved once, here, so no downstream projection re-derives it.
    let team_visible_locations = self.sector_sight(observer_team, ward_coverage);

    let mut allied_locations = Vec::new();
    for (id, loc) in &self.actor_locations {
      let is_ally = if observer_team == TeamSide::Allied {
        self.is_allied(*id)
      } else {
        self.is_opposing(*id)
      };

      if is_ally && *id != observer {
        allied_locations.push((*id, loc.clone()));
      }
    }

    // Evaluate opposing actor visibility
    let mut opposing_sightings = Vec::new();
    for (id, loc) in &self.actor_locations {
      let is_enemy = if observer_team == TeamSide::Allied {
        self.is_opposing(*id)
      } else {
        self.is_allied(*id)
      };

      if is_enemy {
        let enemy_pos = loc.current_location();
        if team_visible_locations[enemy_pos.index()] {
          opposing_sightings.push((
            *id,
            OpponentSighting::Observed {
              location: enemy_pos,
              in_transit: loc.is_in_transit(),
            },
          ));
        } else {
          opposing_sightings.push((*id, OpponentSighting::Unknown));
        }
      }
    }

    Some(MatchMapObservation {
      observer,
      observer_team,
      turn: self.turn,
      self_location,
      allied_locations,
      opposing_sightings,
    })
  }

  /// Sectors `team` can see right now, as flags indexed by [`MapLocation::index`].
  ///
  /// This is the **single visibility rule** for the match: a sector is seen when one
  /// of the team's own actors stands in it, or when that same team has warded it.
  /// Coverage entries owned by the other team are ignored, so a caller holding latent
  /// enemy ward positions cannot spend them as allied sight.
  ///
  /// Every projection that depends on sight — opposing actor sightings through
  /// [`Self::observe_with_wards`] and defensive structure state through
  /// [`super::structures::MatchStructureState::observe_for`] — consumes this array. Hosts
  /// and renderers must not re-derive visibility downstream of it.
  pub fn sector_sight(
    &self,
    team: TeamSide,
    ward_coverage: &[(TeamSide, MapLocation)],
  ) -> SectorSight {
    let mut sight = [false; MAP_LOCATION_COUNT];
    for (id, loc) in &self.actor_locations {
      let is_team_member = match team {
        TeamSide::Allied => self.is_allied(*id),
        TeamSide::Opposing => self.is_opposing(*id),
      };
      if is_team_member {
        sight[loc.current_location().index()] = true;
      }
    }
    for &(ward_team, location) in ward_coverage {
      if ward_team == team {
        sight[location.index()] = true;
      }
    }
    sight
  }

  /// Count `team`'s actors that can apply force at `target` this turn.
  ///
  /// An actor is present when it stands in the target sector or in a sector at most
  /// `reach_beats` of travel away. Presence is *where force reaches*, which is a
  /// different question from [`Self::sector_sight`], which is what a team can learn: an
  /// actor can hit an objective it cannot see, and can see one it is too far away to
  /// support. Actors still in transit count from the sector they currently occupy.
  pub fn presence_within(&self, team: TeamSide, target: MapLocation, reach_beats: u8) -> usize {
    self
      .actor_locations
      .iter()
      .filter(|(id, loc)| {
        let is_team_member = match team {
          TeamSide::Allied => self.is_allied(*id),
          TeamSide::Opposing => self.is_opposing(*id),
        };
        is_team_member && distance_in_beats(loc.current_location(), target) <= reach_beats
      })
      .count()
  }

  /// Compute deterministic FNV-1a state hash over authoritative map state.
  pub fn hash(&self) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, &self.turn.to_le_bytes());

    for (actor_id, location) in &self.actor_locations {
      hash = hash_bytes(hash, &[actor_id.value()]);
      match location {
        ActorLocation::Stationary(loc) => {
          let tag: u8 = 1;
          let idx: u8 = u8::try_from(loc.index()).unwrap_or_default();
          hash = hash_bytes(hash, &[tag, idx]);
        }
        ActorLocation::InTransit(transit) => {
          let tag: u8 = 2;
          let origin_idx: u8 = u8::try_from(transit.origin().index()).unwrap_or_default();
          let dest_idx: u8 = u8::try_from(transit.destination().index()).unwrap_or_default();
          hash = hash_bytes(
            hash,
            &[
              tag,
              origin_idx,
              dest_idx,
              transit.total_beats(),
              transit.progress_beats(),
              transit.remaining_beats(),
            ],
          );
        }
      }
    }

    StateHash::from_raw(hash)
  }
}
