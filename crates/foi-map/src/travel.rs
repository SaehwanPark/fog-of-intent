//! Actor spatial states, rotation commands, transit tracking, and travel validation.

use super::graph::{MapGraphError, TravelRoute, compute_shortest_route, is_adjacent};
use super::topology::MapLocation;
use core::fmt;

/// Error types for travel commands and validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TravelError {
  AlreadyAtDestination {
    location: MapLocation,
  },
  CannotContinueWhenStationary,
  CannotInitiateWhenInTransit {
    current_destination: MapLocation,
  },
  UnreachableDestination {
    from: MapLocation,
    to: MapLocation,
  },
  InvalidAbortFallback {
    fallback: MapLocation,
    current_step: MapLocation,
  },
  Graph(MapGraphError),
}

impl From<MapGraphError> for TravelError {
  fn from(err: MapGraphError) -> Self {
    Self::Graph(err)
  }
}

impl fmt::Display for TravelError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::AlreadyAtDestination { location } => {
        write!(f, "actor is already at destination {location}")
      }
      Self::CannotContinueWhenStationary => {
        f.write_str("cannot continue transit when actor is stationary")
      }
      Self::CannotInitiateWhenInTransit {
        current_destination,
      } => {
        write!(
          f,
          "cannot initiate new rotation while in transit towards {current_destination}"
        )
      }
      Self::UnreachableDestination { from, to } => {
        write!(f, "destination {to} is unreachable from {from}")
      }
      Self::InvalidAbortFallback {
        fallback,
        current_step,
      } => {
        write!(
          f,
          "abort fallback {fallback} is not adjacent to current position {current_step}"
        )
      }
      Self::Graph(err) => write!(f, "graph error: {err}"),
    }
  }
}

/// State of an actor currently rotating across the map.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransitState {
  origin: MapLocation,
  destination: MapLocation,
  route: TravelRoute,
  total_beats: u8,
  progress_beats: u8,
  remaining_beats: u8,
}

impl TransitState {
  /// Construct a new transit state from an origin and destination.
  pub fn new(origin: MapLocation, destination: MapLocation) -> Result<Self, TravelError> {
    if origin == destination {
      return Err(TravelError::AlreadyAtDestination { location: origin });
    }
    let route = compute_shortest_route(origin, destination)?;
    let total_beats = route.duration_beats();
    Ok(Self {
      origin,
      destination,
      route,
      total_beats,
      progress_beats: 0,
      remaining_beats: total_beats,
    })
  }

  /// Construct a transit state from an explicit route.
  pub fn from_route(route: TravelRoute) -> Self {
    let total_beats = route.duration_beats();
    Self {
      origin: route.origin(),
      destination: route.destination(),
      route,
      total_beats,
      progress_beats: 0,
      remaining_beats: total_beats,
    }
  }

  pub fn origin(&self) -> MapLocation {
    self.origin
  }

  pub fn destination(&self) -> MapLocation {
    self.destination
  }

  pub fn route(&self) -> &TravelRoute {
    &self.route
  }

  pub fn total_beats(&self) -> u8 {
    self.total_beats
  }

  pub fn progress_beats(&self) -> u8 {
    self.progress_beats
  }

  pub fn remaining_beats(&self) -> u8 {
    self.remaining_beats
  }

  /// The current map location node corresponding to the progress made along the route.
  pub fn current_step_location(&self) -> MapLocation {
    self.route.step_at_progress(self.progress_beats)
  }

  /// Tick the transit state forward by a number of beats.
  ///
  /// Returns `true` if the destination has been reached.
  pub fn advance(&mut self, beats: u8) -> bool {
    let new_progress = self.progress_beats.saturating_add(beats);
    if new_progress >= self.total_beats {
      self.progress_beats = self.total_beats;
      self.remaining_beats = 0;
      true
    } else {
      self.progress_beats = new_progress;
      self.remaining_beats = self.total_beats.saturating_sub(self.progress_beats);
      false
    }
  }

  /// Abort transit and redirect towards a fallback location.
  ///
  /// The fallback location must be adjacent to the current step location or equal to the origin/current step.
  pub fn abort_to(&self, fallback: MapLocation) -> Result<Self, TravelError> {
    let current_pos = self.current_step_location();
    if current_pos == fallback {
      return Err(TravelError::AlreadyAtDestination { location: fallback });
    }
    if !is_adjacent(current_pos, fallback) && fallback != self.origin {
      return Err(TravelError::InvalidAbortFallback {
        fallback,
        current_step: current_pos,
      });
    }
    Self::new(current_pos, fallback)
  }
}

/// The spatial state of an actor on the map: either stationary in a sector/zone or in transit.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActorLocation {
  Stationary(MapLocation),
  InTransit(TransitState),
}

impl ActorLocation {
  pub const fn stationary(location: MapLocation) -> Self {
    Self::Stationary(location)
  }

  pub fn in_transit(transit: TransitState) -> Self {
    Self::InTransit(transit)
  }

  /// Returns the current physical location or current transit node.
  pub fn current_location(&self) -> MapLocation {
    match self {
      Self::Stationary(loc) => *loc,
      Self::InTransit(transit) => transit.current_step_location(),
    }
  }

  pub fn is_in_transit(&self) -> bool {
    matches!(self, Self::InTransit(_))
  }

  pub fn destination(&self) -> Option<MapLocation> {
    match self {
      Self::Stationary(_) => None,
      Self::InTransit(transit) => Some(transit.destination()),
    }
  }

  pub fn remaining_beats(&self) -> u8 {
    match self {
      Self::Stationary(_) => 0,
      Self::InTransit(transit) => transit.remaining_beats(),
    }
  }

  pub fn transit_state(&self) -> Option<&TransitState> {
    match self {
      Self::Stationary(_) => None,
      Self::InTransit(transit) => Some(transit),
    }
  }
}

/// Commands available to actors for controlling map movement and strategic rotations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TravelCommand {
  /// Start rotating from current location towards a destination.
  InitiateRotation { destination: MapLocation },
  /// Continue travelling along the existing in-progress route.
  ContinueTransit,
  /// Abort current rotation and retreat or divert to a fallback location.
  AbortRotation { fallback: MapLocation },
}

impl TravelCommand {
  pub const fn initiate(destination: MapLocation) -> Self {
    Self::InitiateRotation { destination }
  }

  pub const fn continue_transit() -> Self {
    Self::ContinueTransit
  }

  pub const fn abort(fallback: MapLocation) -> Self {
    Self::AbortRotation { fallback }
  }
}
