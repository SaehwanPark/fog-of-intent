//! Abstracted three-lane map topology, graph pathfinding, actor travel states, and rotation transitions.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

pub mod catalog;
pub mod graph;
pub mod state;
pub mod topology;
pub mod transition;
pub mod travel;

#[cfg(test)]
mod tests;

use crate::kernel::RulesetId;

pub const M9_MAP_RULESET: RulesetId = RulesetId::new(9);
pub const M9_MAP_TOPOLOGY_SCHEMA_V1: &str = "m9-map-topology-v1";
pub const M9_TRAVEL_MODEL_SCHEMA_V1: &str = "m9-travel-model-v1";
pub const M9_MAP_OBSERVATION_SCHEMA_V1: &str = "m9-map-observation-v1";
pub const M9_MAP_SCENARIO_CATALOG_SCHEMA_V1: &str = "m9-map-scenario-catalog-v1";

pub use catalog::{MapScenarioDefinition, MapScenarioExecutionResult, MapTravelCatalog};
pub use graph::{
  MapGraphError, TravelRoute, adjacent_neighbors, compute_shortest_route, distance_in_beats,
  is_adjacent,
};
pub use state::{MatchMapObservation, MatchMapState, OpponentSighting};
pub use topology::{JungleSide, LaneId, LaneSector, MapLocation, RiverSide, TeamSide};
pub use transition::{TravelEffect, TravelEvent, TravelTransitionResult, transition_travel};
pub use travel::{ActorLocation, TransitState, TravelCommand, TravelError};
