//! Deterministic map adjacency graph, pathfinding, and travel distance calculation.

use super::topology::MapLocation;
use core::fmt;

/// Error types for map graph and pathfinding operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MapGraphError {
  SameLocation { location: MapLocation },
  UnreachableRoute { from: MapLocation, to: MapLocation },
  InvalidRouteStep { from: MapLocation, to: MapLocation },
  EmptyRoute,
}

impl fmt::Display for MapGraphError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::SameLocation { location } => {
        write!(f, "origin and destination are identical: {location}")
      }
      Self::UnreachableRoute { from, to } => {
        write!(f, "no valid route from {from} to {to}")
      }
      Self::InvalidRouteStep { from, to } => {
        write!(f, "step from {from} to {to} is not adjacent")
      }
      Self::EmptyRoute => f.write_str("travel route cannot be empty"),
    }
  }
}

/// A validated sequence of adjacent locations representing a continuous travel path.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TravelRoute {
  steps: Vec<MapLocation>,
}

impl TravelRoute {
  /// Create a route from a sequence of steps. Must have at least 2 locations, and each consecutive pair must be adjacent.
  pub fn from_steps(steps: Vec<MapLocation>) -> Result<Self, MapGraphError> {
    if steps.len() < 2 {
      return Err(MapGraphError::EmptyRoute);
    }
    for window in steps.windows(2) {
      if !is_adjacent(window[0], window[1]) {
        return Err(MapGraphError::InvalidRouteStep {
          from: window[0],
          to: window[1],
        });
      }
    }
    Ok(Self { steps })
  }

  pub fn origin(&self) -> MapLocation {
    self.steps[0]
  }

  pub fn destination(&self) -> MapLocation {
    self.steps[self.steps.len() - 1]
  }

  /// Total duration of the route in beats (number of hops).
  pub fn duration_beats(&self) -> u8 {
    u8::try_from(self.steps.len() - 1).unwrap_or(u8::MAX)
  }

  pub fn steps(&self) -> &[MapLocation] {
    &self.steps
  }

  /// Get the intermediate location at a given progress beat.
  pub fn step_at_progress(&self, progress: u8) -> MapLocation {
    let index = usize::from(progress).min(self.steps.len() - 1);
    self.steps[index]
  }
}

/// Static adjacency matrix for the 15 discrete map locations.
const ADJACENCY_TABLE: [[bool; 15]; 15] = {
  let mut table = [[false; 15]; 15];

  // Helper macro to mark symmetric connection
  macro_rules! connect {
    ($a:expr, $b:expr) => {
      table[$a.index()][$b.index()] = true;
      table[$b.index()][$a.index()] = true;
    };
  }

  // Allied Base (0) connections
  connect!(MapLocation::ALLIED_BASE, MapLocation::TOP_NEAR_TOWER);
  connect!(MapLocation::ALLIED_BASE, MapLocation::MID_NEAR_TOWER);
  connect!(MapLocation::ALLIED_BASE, MapLocation::BOT_NEAR_TOWER);
  connect!(MapLocation::ALLIED_BASE, MapLocation::TOP_JUNGLE);
  connect!(MapLocation::ALLIED_BASE, MapLocation::BOT_JUNGLE);

  // Opposing Base (1) connections
  connect!(MapLocation::OPPOSING_BASE, MapLocation::TOP_FAR_SIDE);
  connect!(MapLocation::OPPOSING_BASE, MapLocation::MID_FAR_SIDE);
  connect!(MapLocation::OPPOSING_BASE, MapLocation::BOT_FAR_SIDE);

  // Top Lane intra-lane and cross-zone connections
  connect!(MapLocation::TOP_NEAR_TOWER, MapLocation::TOP_CENTER);
  connect!(MapLocation::TOP_NEAR_TOWER, MapLocation::TOP_JUNGLE);
  connect!(MapLocation::TOP_CENTER, MapLocation::TOP_FAR_SIDE);
  connect!(MapLocation::TOP_CENTER, MapLocation::TOP_RIVER);
  connect!(MapLocation::TOP_CENTER, MapLocation::TOP_JUNGLE);
  connect!(MapLocation::TOP_FAR_SIDE, MapLocation::TOP_RIVER);

  // Mid Lane intra-lane and cross-zone connections
  connect!(MapLocation::MID_NEAR_TOWER, MapLocation::MID_CENTER);
  connect!(MapLocation::MID_NEAR_TOWER, MapLocation::TOP_JUNGLE);
  connect!(MapLocation::MID_NEAR_TOWER, MapLocation::BOT_JUNGLE);
  connect!(MapLocation::MID_CENTER, MapLocation::MID_FAR_SIDE);
  connect!(MapLocation::MID_CENTER, MapLocation::TOP_RIVER);
  connect!(MapLocation::MID_CENTER, MapLocation::BOT_RIVER);
  connect!(MapLocation::MID_CENTER, MapLocation::TOP_JUNGLE);
  connect!(MapLocation::MID_CENTER, MapLocation::BOT_JUNGLE);
  connect!(MapLocation::MID_FAR_SIDE, MapLocation::TOP_RIVER);
  connect!(MapLocation::MID_FAR_SIDE, MapLocation::BOT_RIVER);

  // Bot Lane intra-lane and cross-zone connections
  connect!(MapLocation::BOT_NEAR_TOWER, MapLocation::BOT_CENTER);
  connect!(MapLocation::BOT_NEAR_TOWER, MapLocation::BOT_JUNGLE);
  connect!(MapLocation::BOT_CENTER, MapLocation::BOT_FAR_SIDE);
  connect!(MapLocation::BOT_CENTER, MapLocation::BOT_RIVER);
  connect!(MapLocation::BOT_CENTER, MapLocation::BOT_JUNGLE);
  connect!(MapLocation::BOT_FAR_SIDE, MapLocation::BOT_RIVER);

  // River and Jungle cross-connections
  connect!(MapLocation::TOP_RIVER, MapLocation::TOP_JUNGLE);
  connect!(MapLocation::BOT_RIVER, MapLocation::BOT_JUNGLE);

  table
};

/// Check whether two map locations are directly adjacent (1 beat travel).
pub const fn is_adjacent(a: MapLocation, b: MapLocation) -> bool {
  ADJACENCY_TABLE[a.index()][b.index()]
}

/// Compute the shortest distance in beats between two locations using BFS.
pub fn distance_in_beats(from: MapLocation, to: MapLocation) -> u8 {
  if from == to {
    return 0;
  }
  if is_adjacent(from, to) {
    return 1;
  }
  match compute_shortest_route(from, to) {
    Ok(route) => route.duration_beats(),
    Err(_) => u8::MAX,
  }
}

/// Find the deterministic shortest path between two locations using Breadth-First Search (BFS).
///
/// Ties are broken deterministically by the canonical index order of MapLocation.
pub fn compute_shortest_route(
  from: MapLocation,
  to: MapLocation,
) -> Result<TravelRoute, MapGraphError> {
  if from == to {
    return Err(MapGraphError::SameLocation { location: from });
  }

  let mut queue = std::collections::VecDeque::new();
  let mut visited = [false; 15];
  let mut parent = [None; 15];

  visited[from.index()] = true;
  queue.push_back(from);

  let mut found = false;
  while let Some(current) = queue.pop_front() {
    if current == to {
      found = true;
      break;
    }
    for &next in &MapLocation::ALL_LOCATIONS {
      if is_adjacent(current, next) && !visited[next.index()] {
        visited[next.index()] = true;
        parent[next.index()] = Some(current);
        queue.push_back(next);
      }
    }
  }

  if !found {
    return Err(MapGraphError::UnreachableRoute { from, to });
  }

  // Reconstruct path
  let mut path = Vec::new();
  let mut curr = Some(to);
  while let Some(node) = curr {
    path.push(node);
    if node == from {
      break;
    }
    curr = parent[node.index()];
  }
  path.reverse();

  TravelRoute::from_steps(path)
}

/// List all adjacent neighbor locations for a given location in canonical index order.
pub fn adjacent_neighbors(location: MapLocation) -> Vec<MapLocation> {
  MapLocation::ALL_LOCATIONS
    .iter()
    .copied()
    .filter(|&neighbor| is_adjacent(location, neighbor))
    .collect()
}
