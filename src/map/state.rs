//! Match map state, FNV-1a state hashing, and actor-visible observation projections.

use crate::kernel::{ActorId, StateHash, hash_bytes};

pub(crate) const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
use super::topology::{MapLocation, TeamSide};
use super::travel::ActorLocation;

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
  pub fn observe(&self, observer: ActorId) -> Option<MatchMapObservation> {
    let observer_team = if self.is_allied(observer) {
      TeamSide::Allied
    } else if self.is_opposing(observer) {
      TeamSide::Opposing
    } else {
      return None;
    };

    let self_location = self.get_actor_location(observer)?.clone();

    // Determine team-visible locations (all locations occupied by allies)
    let mut team_visible_locations = [false; 15];
    let mut allied_locations = Vec::new();

    for (id, loc) in &self.actor_locations {
      let is_ally = if observer_team == TeamSide::Allied {
        self.is_allied(*id)
      } else {
        self.is_opposing(*id)
      };

      if is_ally {
        if *id != observer {
          allied_locations.push((*id, loc.clone()));
        }
        team_visible_locations[loc.current_location().index()] = true;
      }
    }
    team_visible_locations[self_location.current_location().index()] = true;

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
