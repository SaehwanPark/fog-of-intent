//! Abstracted three-lane map topology, travel model, objective cycles, vision control, and contest mechanics.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

pub mod catalog;
pub mod contest;
pub mod graph;
pub mod objective;
pub mod objective_catalog;
pub mod state;
pub mod topology;
pub mod transition;
pub mod travel;
pub mod vision;

#[cfg(test)]
mod tests;

use crate::kernel::RulesetId;

pub const M9_MAP_RULESET: RulesetId = RulesetId::new(9);
pub const M9_MAP_TOPOLOGY_SCHEMA_V1: &str = "m9-map-topology-v1";
pub const M9_TRAVEL_MODEL_SCHEMA_V1: &str = "m9-travel-model-v1";
pub const M9_MAP_OBSERVATION_SCHEMA_V1: &str = "m9-map-observation-v1";
pub const M9_MAP_SCENARIO_CATALOG_SCHEMA_V1: &str = "m9-map-scenario-catalog-v1";
pub const M9_OBJECTIVE_CYCLES_SCHEMA_V1: &str = "m9-objective-cycles-v1";
pub const M9_VISION_CONTROL_SCHEMA_V1: &str = "m9-vision-control-v1";
pub const M9_OBJECTIVE_CONTEST_SCHEMA_V1: &str = "m9-objective-contest-v1";
pub const M9_OBJECTIVE_CATALOG_SCHEMA_V1: &str = "m9-objective-catalog-v1";

pub use catalog::{MapScenarioDefinition, MapScenarioExecutionResult, MapTravelCatalog};
pub use contest::{
  ContestTransitionResult, CrossMapTradeTarget, ObjectiveEffect, ObjectiveEvent, ObjectiveIntent,
  TradeClassification, TradeoffEvaluation, transition_objective_contest,
};
pub use graph::{
  MapGraphError, TravelRoute, adjacent_neighbors, compute_shortest_route, distance_in_beats,
  is_adjacent,
};
pub use objective::{
  DamageOutcome, MatchObjectiveState, ObjectiveEntry, ObjectiveError, ObjectiveKind,
  ObjectiveStatus,
};
pub use objective_catalog::{
  ObjectiveScenarioCatalog, ObjectiveScenarioDefinition, ObjectiveScenarioExecutionResult,
};
pub use state::{MatchMapObservation, MatchMapState, OpponentSighting};
pub use topology::{JungleSide, LaneId, LaneSector, MapLocation, RiverSide, TeamSide};
pub use transition::{TravelEffect, TravelEvent, TravelTransitionResult, transition_travel};
pub use travel::{ActorLocation, TransitState, TravelCommand, TravelError};
pub use vision::{
  DEFAULT_WARD_DURATION_TURNS, MAX_WARDS_PER_TEAM, MapVisionGrid, MapVisionState, VisionCommand,
  VisionCoverage, VisionError, VisionWard,
};
