//! Unit and invariant tests for the M9 map topology, graph pathfinding, travel model, and scenarios.

use super::catalog::MapTravelCatalog;
use super::graph::{
  MapGraphError, TravelRoute, adjacent_neighbors, compute_shortest_route, distance_in_beats,
  is_adjacent,
};
use super::state::{MatchMapState, OpponentSighting};
use super::topology::{LaneId, LaneSector, MapLocation, TeamSide};
use super::transition::{TravelEffect, TravelEvent, transition_travel};
use super::travel::{ActorLocation, TransitState, TravelCommand, TravelError};
use crate::kernel::ActorId;

#[test]
fn topology_locations_index_and_reverse_lookup_round_trip() {
  for (idx, &location) in MapLocation::ALL_LOCATIONS.iter().enumerate() {
    assert_eq!(location.index(), idx);
    assert_eq!(MapLocation::from_index(idx), Some(location));
    assert!(!location.as_str().is_empty());
    assert_eq!(format!("{location}"), location.as_str());
  }
  assert_eq!(MapLocation::from_index(15), None);
  assert_eq!(MapLocation::ALL_LOCATIONS.len(), 15);
}

#[test]
fn topology_classification_helpers() {
  assert!(MapLocation::ALLIED_BASE.is_base());
  assert!(MapLocation::OPPOSING_BASE.is_base());
  assert!(!MapLocation::TOP_CENTER.is_base());

  assert!(MapLocation::TOP_NEAR_TOWER.is_lane());
  assert_eq!(MapLocation::TOP_NEAR_TOWER.lane_id(), Some(LaneId::Top));
  assert_eq!(
    MapLocation::TOP_NEAR_TOWER.lane_sector(),
    Some(LaneSector::NearTower)
  );

  assert!(MapLocation::TOP_RIVER.is_river());
  assert!(MapLocation::BOT_JUNGLE.is_jungle());
}

#[test]
fn graph_adjacency_is_symmetric_for_all_pairs() {
  for &a in &MapLocation::ALL_LOCATIONS {
    for &b in &MapLocation::ALL_LOCATIONS {
      assert_eq!(
        is_adjacent(a, b),
        is_adjacent(b, a),
        "Adjacency between {a} and {b} must be symmetric"
      );
    }
  }
}

#[test]
fn graph_self_adjacency_is_false() {
  for &location in &MapLocation::ALL_LOCATIONS {
    assert!(!is_adjacent(location, location));
  }
}

#[test]
fn graph_adjacent_neighbors_matches_is_adjacent() {
  for &location in &MapLocation::ALL_LOCATIONS {
    let neighbors = adjacent_neighbors(location);
    assert!(!neighbors.is_empty());
    for &neighbor in &neighbors {
      assert!(is_adjacent(location, neighbor));
    }
  }
}

#[test]
fn graph_distance_and_pathfinding() {
  // Distance to self is 0
  assert_eq!(
    distance_in_beats(MapLocation::ALLIED_BASE, MapLocation::ALLIED_BASE),
    0
  );

  // Direct adjacent distance is 1
  assert_eq!(
    distance_in_beats(MapLocation::ALLIED_BASE, MapLocation::MID_NEAR_TOWER),
    1
  );
  assert_eq!(
    distance_in_beats(MapLocation::MID_NEAR_TOWER, MapLocation::MID_CENTER),
    1
  );

  // 2-hop distance
  assert_eq!(
    distance_in_beats(MapLocation::ALLIED_BASE, MapLocation::MID_CENTER),
    2
  );

  // Top center to Mid center via Top river
  let route = compute_shortest_route(MapLocation::TOP_CENTER, MapLocation::MID_CENTER).unwrap();
  assert_eq!(route.duration_beats(), 2);
  assert_eq!(route.origin(), MapLocation::TOP_CENTER);
  assert_eq!(route.destination(), MapLocation::MID_CENTER);
  assert_eq!(
    route.steps(),
    &[
      MapLocation::TOP_CENTER,
      MapLocation::TOP_RIVER,
      MapLocation::MID_CENTER
    ]
  );
}

#[test]
fn graph_same_location_pathfinding_returns_error() {
  let err = compute_shortest_route(MapLocation::BOT_CENTER, MapLocation::BOT_CENTER).unwrap_err();
  assert_eq!(
    err,
    MapGraphError::SameLocation {
      location: MapLocation::BOT_CENTER
    }
  );
}

#[test]
fn travel_route_validation() {
  let valid_steps = vec![
    MapLocation::ALLIED_BASE,
    MapLocation::TOP_NEAR_TOWER,
    MapLocation::TOP_CENTER,
  ];
  let route = TravelRoute::from_steps(valid_steps).unwrap();
  assert_eq!(route.duration_beats(), 2);
  assert_eq!(route.step_at_progress(0), MapLocation::ALLIED_BASE);
  assert_eq!(route.step_at_progress(1), MapLocation::TOP_NEAR_TOWER);
  assert_eq!(route.step_at_progress(2), MapLocation::TOP_CENTER);
  assert_eq!(route.step_at_progress(5), MapLocation::TOP_CENTER);

  // Non-adjacent steps are rejected
  let invalid_steps = vec![MapLocation::ALLIED_BASE, MapLocation::TOP_FAR_SIDE];
  let err = TravelRoute::from_steps(invalid_steps).unwrap_err();
  assert_eq!(
    err,
    MapGraphError::InvalidRouteStep {
      from: MapLocation::ALLIED_BASE,
      to: MapLocation::TOP_FAR_SIDE
    }
  );
}

#[test]
fn transit_state_lifecycle_and_advance() {
  let mut transit =
    TransitState::new(MapLocation::TOP_NEAR_TOWER, MapLocation::TOP_FAR_SIDE).unwrap();
  assert_eq!(transit.total_beats(), 2);
  assert_eq!(transit.remaining_beats(), 2);
  assert_eq!(transit.progress_beats(), 0);
  assert_eq!(transit.current_step_location(), MapLocation::TOP_NEAR_TOWER);

  // Advance 1 beat
  let reached = transit.advance(1);
  assert!(!reached);
  assert_eq!(transit.progress_beats(), 1);
  assert_eq!(transit.remaining_beats(), 1);
  assert_eq!(transit.current_step_location(), MapLocation::TOP_CENTER);

  // Advance final beat
  let reached = transit.advance(1);
  assert!(reached);
  assert_eq!(transit.progress_beats(), 2);
  assert_eq!(transit.remaining_beats(), 0);
  assert_eq!(transit.current_step_location(), MapLocation::TOP_FAR_SIDE);
}

#[test]
fn transit_state_abort_redirection() {
  let mut transit =
    TransitState::new(MapLocation::TOP_NEAR_TOWER, MapLocation::MID_CENTER).unwrap();
  transit.advance(1); // Now at Top Jungle or Top Center

  let current = transit.current_step_location();
  let aborted = transit.abort_to(MapLocation::TOP_NEAR_TOWER).unwrap();
  assert_eq!(aborted.origin(), current);
  assert_eq!(aborted.destination(), MapLocation::TOP_NEAR_TOWER);
}

#[test]
fn transition_travel_initiate_and_complete() {
  let actor = ActorId::new(1);
  let initial = ActorLocation::stationary(MapLocation::TOP_CENTER);

  // Initiate rotation to Mid Center (2 beats total, advancing 1 beat)
  let cmd = TravelCommand::InitiateRotation {
    destination: MapLocation::MID_CENTER,
  };
  let result = transition_travel(actor, &initial, cmd, 1).unwrap();

  assert!(result.next_location.is_in_transit());
  assert_eq!(
    result.next_location.current_location(),
    MapLocation::TOP_RIVER
  );
  assert_eq!(result.events.len(), 2);
  assert_eq!(
    result.events[0],
    TravelEvent::RotationInitiated {
      actor,
      from: MapLocation::TOP_CENTER,
      to: MapLocation::MID_CENTER,
      total_beats: 2,
    }
  );

  // Continue transit to arrival
  let cmd2 = TravelCommand::ContinueTransit;
  let result2 = transition_travel(actor, &result.next_location, cmd2, 1).unwrap();

  assert!(!result2.next_location.is_in_transit());
  assert_eq!(
    result2.next_location,
    ActorLocation::Stationary(MapLocation::MID_CENTER)
  );
  assert_eq!(
    result2.events,
    vec![TravelEvent::RotationCompleted {
      actor,
      destination: MapLocation::MID_CENTER
    }]
  );
  assert_eq!(
    result2.effects,
    vec![TravelEffect::ArrivalAtDestination {
      actor,
      destination: MapLocation::MID_CENTER
    }]
  );
}

#[test]
fn transition_travel_abort_rotation() {
  let actor = ActorId::new(1);
  let mut transit =
    TransitState::new(MapLocation::TOP_NEAR_TOWER, MapLocation::TOP_FAR_SIDE).unwrap();
  transit.advance(1); // At Top Center

  let current_loc = ActorLocation::InTransit(transit);
  let abort_cmd = TravelCommand::AbortRotation {
    fallback: MapLocation::TOP_NEAR_TOWER,
  };

  let result = transition_travel(actor, &current_loc, abort_cmd, 1).unwrap();
  assert_eq!(
    result.next_location,
    ActorLocation::Stationary(MapLocation::TOP_NEAR_TOWER)
  );
  assert!(
    result
      .events
      .iter()
      .any(|e| matches!(e, TravelEvent::RotationAborted { .. }))
  );
}

#[test]
fn transition_travel_validation_errors() {
  let actor = ActorId::new(1);
  let stationary = ActorLocation::Stationary(MapLocation::MID_CENTER);

  // Rotating to same location fails
  let err = transition_travel(
    actor,
    &stationary,
    TravelCommand::InitiateRotation {
      destination: MapLocation::MID_CENTER,
    },
    1,
  )
  .unwrap_err();
  assert_eq!(
    err,
    TravelError::AlreadyAtDestination {
      location: MapLocation::MID_CENTER
    }
  );

  // Continuing when stationary fails
  let err2 = transition_travel(actor, &stationary, TravelCommand::ContinueTransit, 1).unwrap_err();
  assert_eq!(err2, TravelError::CannotContinueWhenStationary);

  // Initiating rotation when already in transit fails
  let transit = TransitState::new(MapLocation::TOP_NEAR_TOWER, MapLocation::TOP_FAR_SIDE).unwrap();
  let in_transit = ActorLocation::InTransit(transit);
  let err3 = transition_travel(
    actor,
    &in_transit,
    TravelCommand::InitiateRotation {
      destination: MapLocation::BOT_CENTER,
    },
    1,
  )
  .unwrap_err();
  assert_eq!(
    err3,
    TravelError::CannotInitiateWhenInTransit {
      current_destination: MapLocation::TOP_FAR_SIDE
    }
  );
}

#[test]
fn match_map_state_observation_and_fog_of_war_redaction() {
  let ally1 = ActorId::new(1);
  let ally2 = ActorId::new(2);
  let enemy1 = ActorId::new(3); // In same zone (visible)
  let enemy2 = ActorId::new(4); // In fog (unknown)

  let state = MatchMapState::new(
    1,
    vec![ally1, ally2],
    vec![enemy1, enemy2],
    vec![
      (ally1, ActorLocation::Stationary(MapLocation::MID_CENTER)),
      (
        ally2,
        ActorLocation::Stationary(MapLocation::BOT_NEAR_TOWER),
      ),
      (enemy1, ActorLocation::Stationary(MapLocation::MID_CENTER)),
      (enemy2, ActorLocation::Stationary(MapLocation::TOP_FAR_SIDE)),
    ],
  );

  let obs = state.observe(ally1).unwrap();
  assert_eq!(obs.observer, ally1);
  assert_eq!(obs.observer_team, TeamSide::Allied);
  assert_eq!(
    obs.self_location,
    ActorLocation::Stationary(MapLocation::MID_CENTER)
  );

  // Ally 2 is visible
  assert_eq!(obs.allied_locations.len(), 1);
  assert_eq!(obs.allied_locations[0].0, ally2);

  // Enemy 1 is in same sector -> Observed
  let sighting_enemy1 = obs
    .opposing_sightings
    .iter()
    .find(|(id, _)| *id == enemy1)
    .unwrap();
  assert_eq!(
    sighting_enemy1.1,
    OpponentSighting::Observed {
      location: MapLocation::MID_CENTER,
      in_transit: false,
    }
  );

  // Enemy 2 is in Top Far Side (no ally vision) -> Unknown
  let sighting_enemy2 = obs
    .opposing_sightings
    .iter()
    .find(|(id, _)| *id == enemy2)
    .unwrap();
  assert_eq!(sighting_enemy2.1, OpponentSighting::Unknown);
}

#[test]
fn match_map_state_hash_determinism() {
  let a1 = ActorId::new(1);
  let a2 = ActorId::new(2);

  let state1 = MatchMapState::new(
    1,
    vec![a1],
    vec![a2],
    vec![
      (a1, ActorLocation::Stationary(MapLocation::TOP_CENTER)),
      (a2, ActorLocation::Stationary(MapLocation::BOT_CENTER)),
    ],
  );

  let state2 = MatchMapState::new(
    1,
    vec![a1],
    vec![a2],
    vec![
      (a2, ActorLocation::Stationary(MapLocation::BOT_CENTER)),
      (a1, ActorLocation::Stationary(MapLocation::TOP_CENTER)),
    ],
  );

  // Hashes must be identical regardless of insertion order
  assert_eq!(state1.hash(), state2.hash());
}

#[test]
fn catalog_all_canonical_scenarios_execute_successfully() {
  for scenario in MapTravelCatalog::all() {
    let result = scenario.execute().unwrap();
    assert_eq!(result.scenario_id, scenario.scenario_id);
    assert_eq!(
      result.terminal_locations,
      scenario.expected_terminal_locations
    );
  }
}
