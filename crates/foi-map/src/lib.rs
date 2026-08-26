//! Abstracted three-lane map topology, travel model, objective cycles, vision control, and contest mechanics.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

pub mod catalog;
pub mod comeback;
pub mod comeback_catalog;
pub mod complete_match;
pub mod complete_match_catalog;
pub mod composition;
pub mod contest;
pub mod cost_profile;
pub mod decision_density;
pub mod decision_density_catalog;
pub mod graph;
pub mod match_catalog;
pub mod objective;
pub mod objective_catalog;
pub mod pivotal;
pub mod pivotal_catalog;
pub mod population_validation;
pub mod population_validation_catalog;
pub mod role_action;
pub mod role_catalog;
pub mod role_debrief;
pub mod role_observation;
pub mod state;
pub mod structures;
pub mod topology;
pub mod transition;
pub mod travel;
pub mod victory;
pub mod vision;

#[cfg(test)]
pub use crate as map;
pub use foi_kernel as kernel;

#[cfg(test)]
mod tests;

use foi_kernel::RulesetId;

pub const M9_MAP_RULESET: RulesetId = RulesetId::new(9);
pub const M9_MAP_TOPOLOGY_SCHEMA_V1: &str = "m9-map-topology-v1";
pub const M9_TRAVEL_MODEL_SCHEMA_V1: &str = "m9-travel-model-v1";
pub const M9_MAP_OBSERVATION_SCHEMA_V1: &str = "m9-map-observation-v1";
pub const M9_MAP_SCENARIO_CATALOG_SCHEMA_V1: &str = "m9-map-scenario-catalog-v1";
pub const M9_OBJECTIVE_CYCLES_SCHEMA_V1: &str = "m9-objective-cycles-v1";
pub const M9_VISION_CONTROL_SCHEMA_V1: &str = "m9-vision-control-v1";
pub const M9_OBJECTIVE_CONTEST_SCHEMA_V1: &str = "m9-objective-contest-v1";
pub const M9_OBJECTIVE_CATALOG_SCHEMA_V1: &str = "m9-objective-catalog-v1";
pub const M9_TEAM_COMPOSITION_SCHEMA_V1: &str = "m9-team-composition-v1";
pub const M9_MATCH_STRUCTURES_SCHEMA_V1: &str = "m9-match-structures-v1";
pub const M9_MATCH_VICTORY_SCHEMA_V1: &str = "m9-match-victory-v1";
pub const M9_MATCH_SCENARIO_CATALOG_SCHEMA_V1: &str = "m9-match-scenario-catalog-v1";
pub const M9_ROLE_OBSERVATION_SCHEMA_V1: &str = "m9-role-observation-v1";
pub const M9_ROLE_ACTION_SCHEMA_V1: &str = "m9-role-action-v1";
pub const M9_ROLE_DEBRIEF_SCHEMA_V1: &str = "m9-role-debrief-v1";
pub const M9_ROLE_SCENARIO_CATALOG_SCHEMA_V1: &str = "m9-role-scenario-catalog-v1";
pub const M9_COMEBACK_MECHANICS_SCHEMA_V1: &str = "m9-comeback-mechanics-v1";
pub const M9_COMEBACK_CATALOG_SCHEMA_V1: &str = "m9-comeback-catalog-v1";
pub const M9_PIVOTAL_DECISION_SCHEMA_V1: &str = "m9-pivotal-decision-v1";
pub const M9_PIVOTAL_CATALOG_SCHEMA_V1: &str = "m9-pivotal-catalog-v1";
pub const M9_DECISION_DENSITY_SCHEMA_V1: &str = "m9-decision-density-v1";
pub const M9_DECISION_DENSITY_CATALOG_SCHEMA_V1: &str = "m9-decision-density-catalog-v1";
pub const M9_COMPLETE_MATCH_SCHEMA_V1: &str = "m9-complete-match-v1";
pub const M9_COMPLETE_MATCH_CATALOG_SCHEMA_V1: &str = "m9-complete-match-catalog-v1";
pub const M9_POPULATION_VALIDATION_SCHEMA_V1: &str = "m9-population-validation-v1";
pub const M9_POPULATION_VALIDATION_CATALOG_SCHEMA_V1: &str = "m9-population-validation-catalog-v1";

pub use catalog::{MapScenarioDefinition, MapScenarioExecutionResult, MapTravelCatalog};
pub use comeback::{
  ComebackEvaluation, ComebackOpportunityInputs, DeficitLevel, VarianceSeekingBehavior,
  evaluate_comeback_opportunity,
};
pub use comeback_catalog::{
  ComebackCatalog, ComebackScenarioDefinition, ComebackScenarioExecutionResult,
};
pub use complete_match::{
  CompleteMatchAction, CompleteMatchError, CompleteMatchPlan, CompleteMatchResult,
  CompleteMatchState, MatchPhaseKind, MatchPhaseRecord,
};
pub use complete_match_catalog::CompleteMatchCatalog;
pub use composition::{
  CompositionArchetype, CompositionCatalog, CompositionMatchupEvaluation, MatchPhase, MatchRole,
  PowerScalingCurve, RecommendedPosture, TeamComposition,
};
pub use contest::{
  ContestTransitionResult, CrossMapTradeTarget, ObjectiveEffect, ObjectiveEvent, ObjectiveIntent,
  TradeClassification, TradeoffEvaluation, transition_objective_contest,
};
pub use cost_profile::{
  CostProfileError, CostProfileReport, M9_COST_PROFILE_SCHEMA_V1, OperationCounts,
  SCALING_PROBE_STEPS, ScalingProbe, ScenarioCostProfile, profile_catalog_batch,
  profile_scaling_probe, profile_travel_scenario,
};
pub use decision_density::{
  CandidateWindowKind, DECISION_SHARE_MAX_BP, DECISION_SHARE_MIN_BP, DecisionDensityError,
  DecisionDensityReport, EscalationTrigger, MAX_DECISION_GAP_TURNS, ROUTINE_STAKES_CEILING_BP,
  RoutineWindowCandidate, STAKES_BOUND_BP, WindowDisposition, WindowFinding,
  evaluate_decision_density,
};
pub use decision_density_catalog::{
  DecisionDensityCatalog, DecisionDensityScenarioDefinition, DecisionDensityScenarioExecutionResult,
};
pub use graph::{
  MapGraphError, TravelRoute, adjacent_neighbors, compute_shortest_route, distance_in_beats,
  is_adjacent,
};
pub use match_catalog::{
  MatchScenarioCatalog, MatchScenarioDefinition, MatchScenarioExecutionResult,
};
pub use objective::{
  DamageOutcome, MatchObjectiveState, ObjectiveEntry, ObjectiveError, ObjectiveKind,
  ObjectiveStatus,
};
pub use objective_catalog::{
  ObjectiveScenarioCatalog, ObjectiveScenarioDefinition, ObjectiveScenarioExecutionResult,
};
pub use pivotal::{
  DecisionAlignment, NOTABLE_MAX_SWING_BP, PIVOTAL_MAX_SWING_BP, PivotalDecisionError,
  PivotalDecisionFinding, PivotalDecisionReport, PivotalDecisionSample, PivotalTier,
  ROUTINE_MAX_SWING_BP, SwingDirection, VALUE_BOUND_BP, detect_pivotal_decisions,
};
pub use pivotal_catalog::{
  PivotalCatalog, PivotalScenarioDefinition, PivotalScenarioExecutionResult,
};
pub use population_validation::{
  COMMUNICATION_USAGE_FLOOR_BP, MIN_DISTINCT_STRATEGIES, MechanicExemption, MechanicKind,
  PopulationValidationError, PopulationValidationReport, ROLE_ACTIVITY_FLOOR_BP, ReplaySummary,
  measure_validation_population,
};
pub use population_validation_catalog::{
  PopulationScenarioDefinition, PopulationScenarioExecutionResult, PopulationValidationCatalog,
};
pub use role_action::{
  BotCarryIntent, JungleIntent, MidIntent, RoleAction, RoleActionError, RoleIntent, SupportIntent,
  TopIntent, validate_role_action,
};
pub use role_catalog::{RoleScenarioCatalog, RoleScenarioDefinition, RoleScenarioExecutionResult};
pub use role_debrief::{RoleCausalFactor, RoleDebriefPerspective, RoleKpis, RolePerformanceTier};
pub use role_observation::{
  BotCarryContext, JunglerContext, MidLanerContext, RoleMatchObservation, RoleSpecificContext,
  SupportContext, TopLanerContext, WaveStateSummary,
};
pub use state::{MatchMapObservation, MatchMapState, OpponentSighting};
pub use structures::{
  INHIBITOR_RESPAWN_TURNS, MatchStructureState, SiegeIntent, StructureEffect, StructureEntry,
  StructureError, StructureEvent, StructureSiegeResult, StructureStatus, StructureTier,
  transition_structure_siege,
};
pub use topology::{JungleSide, LaneId, LaneSector, MapLocation, RiverSide, TeamSide};
pub use transition::{TravelEffect, TravelEvent, TravelTransitionResult, transition_travel};
pub use travel::{ActorLocation, TransitState, TravelCommand, TravelError};
pub use victory::{MatchStatus, MatchTerminalEvaluation, MatchVictoryCondition};
pub use vision::{
  DEFAULT_WARD_DURATION_TURNS, MAX_WARDS_PER_TEAM, MapVisionGrid, MapVisionState, VisionCommand,
  VisionCoverage, VisionError, VisionWard,
};
