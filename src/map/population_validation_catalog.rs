//! Canonical population-validation benchmark scenarios for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! Each scenario declares an explicit validation population and exercises a
//! distinct measurement path through `measure_validation_population`: a fully
//! diverse and engaged population, a narrow passive population that fails
//! every gate, and a population whose one unused mechanic carries an explicit
//! exemption while another goes unexplained. Scenarios are reproducible: the
//! same observations always produce the same report.

use super::composition::{CompositionArchetype, MatchRole};
use super::population_validation::{
  M9_POPULATION_VALIDATION_SCHEMA_V1, MechanicExemption, MechanicKind, PopulationValidationReport,
  ReplaySummary, measure_validation_population,
};

pub const M9_POPULATION_VALIDATION_CATALOG_SCHEMA_V1: &str = "m9-population-validation-catalog-v1";

const ALL_ROLES: [MatchRole; 5] = MatchRole::ALL;

/// Specification of a canonical population-validation benchmark scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopulationScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub expected_distinct_strategies: u32,
  pub expected_inactive_role_count: usize,
  pub expected_unused_mechanic_count: usize,
  pub expected_unexplained_unused_count: usize,
  pub expected_strategy_diversity_passes: bool,
  pub expected_all_mechanics_justified: bool,
}

/// Execution result of running a canonical population-validation scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopulationScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub report: PopulationValidationReport,
  pub distinct_strategies_match: bool,
  pub inactive_role_count_matches: bool,
  pub unused_mechanic_count_matches: bool,
  pub unexplained_unused_count_matches: bool,
  pub strategy_diversity_matches: bool,
  pub justification_matches: bool,
  pub all_expectations_met: bool,
}

/// Catalog of registered canonical population-validation scenarios for M9.
pub struct PopulationValidationCatalog;

impl PopulationValidationCatalog {
  /// Scenario 1: a diverse, fully engaged validation population.
  ///
  /// Four representative replays, one per strategy archetype; every role is
  /// active somewhere; every replay communicates; all eight M9 mechanics are
  /// exercised across the population. Every gate passes.
  pub const SCENARIO_DIVERSE_ENGAGED_POPULATION: PopulationScenarioDefinition =
    PopulationScenarioDefinition {
      scenario_id: "scenario-diverse-engaged-population-v1",
      name: "Diverse Engaged Population",
      description: "Four replays covering all four strategy archetypes with every role active, \
        communication in every replay, and all eight M9 mechanics exercised. Strategy \
        diversity, role activity, communication, and mechanic justification all pass.",
      expected_distinct_strategies: 4,
      expected_inactive_role_count: 0,
      expected_unused_mechanic_count: 0,
      expected_unexplained_unused_count: 0,
      expected_strategy_diversity_passes: true,
      expected_all_mechanics_justified: true,
    };

  /// Scenario 2: a narrow, passive population that fails every gate.
  ///
  /// Three replays of one archetype, the Support role never takes decisions,
  /// no replay communicates, and most mechanics go unused without exemption.
  /// The measurement must surface every failure.
  pub const SCENARIO_NARROW_PASSIVE_POPULATION: PopulationScenarioDefinition =
    PopulationScenarioDefinition {
      scenario_id: "scenario-narrow-passive-population-v1",
      name: "Narrow Passive Population",
      description: "Three one-sided early-aggression replays: a single strategy archetype, an \
        inactive support, silent teams, and only rotation and role tactics exercised. \
        Diversity, role activity, communication, and justification all fail.",
      expected_distinct_strategies: 1,
      expected_inactive_role_count: 1,
      expected_unused_mechanic_count: 6,
      expected_unexplained_unused_count: 6,
      expected_strategy_diversity_passes: false,
      expected_all_mechanics_justified: false,
    };

  /// Scenario 3: an exempted unused mechanic beside an unexplained one.
  ///
  /// Two replays with distinct strategies communicate and stay active, but
  /// neither exercises comeback play or pivotal review. Comeback play carries
  /// a declared exemption (decisive leads, no deficit windows); pivotal
  /// review does not, so justification fails on exactly that mechanic.
  pub const SCENARIO_EXEMPTED_UNUSED_MECHANIC: PopulationScenarioDefinition =
    PopulationScenarioDefinition {
      scenario_id: "scenario-exempted-unused-mechanic-v1",
      name: "Exempted Unused Mechanic",
      description: "Two diverse, communicating replays that never reach a deficit window and \
        skip debrief review. Comeback play is exempted with an explicit reason; pivotal \
        review is not, so exactly one unexplained unused mechanic remains.",
      expected_distinct_strategies: 2,
      expected_inactive_role_count: 0,
      expected_unused_mechanic_count: 2,
      expected_unexplained_unused_count: 1,
      expected_strategy_diversity_passes: true,
      expected_all_mechanics_justified: false,
    };

  pub const ALL_SCENARIOS: [PopulationScenarioDefinition; 3] = [
    Self::SCENARIO_DIVERSE_ENGAGED_POPULATION,
    Self::SCENARIO_NARROW_PASSIVE_POPULATION,
    Self::SCENARIO_EXEMPTED_UNUSED_MECHANIC,
  ];

  pub fn list_scenarios() -> &'static [PopulationScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static PopulationScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Execute a named population-validation benchmark scenario and return the
  /// verifiable measurement report.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<PopulationScenarioExecutionResult, &'static str> {
    let (definition, observations, exemptions): (
      &PopulationScenarioDefinition,
      Vec<ReplaySummary>,
      Vec<MechanicExemption>,
    ) = match scenario_id {
      "scenario-diverse-engaged-population-v1" => (
        &Self::SCENARIO_DIVERSE_ENGAGED_POPULATION,
        Self::diverse_engaged_observations(),
        vec![],
      ),
      "scenario-narrow-passive-population-v1" => (
        &Self::SCENARIO_NARROW_PASSIVE_POPULATION,
        Self::narrow_passive_observations(),
        vec![],
      ),
      "scenario-exempted-unused-mechanic-v1" => (
        &Self::SCENARIO_EXEMPTED_UNUSED_MECHANIC,
        Self::exempted_observations(),
        vec![MechanicExemption {
          mechanic: MechanicKind::ComebackPlay,
          reason: "decisive leads; no deficit windows occurred",
        }],
      ),
      _ => return Err("unknown-population-validation-scenario"),
    };

    let report = measure_validation_population(&observations, &exemptions)
      .map_err(|_| "invalid-population-validation-scenario")?;
    debug_assert_eq!(report.schema, M9_POPULATION_VALIDATION_SCHEMA_V1);

    let distinct_strategies_match =
      report.distinct_strategy_count == definition.expected_distinct_strategies;
    let inactive_role_count_matches =
      report.inactive_roles.len() == definition.expected_inactive_role_count;
    let unused_mechanic_count_matches =
      report.unused_mechanics.len() == definition.expected_unused_mechanic_count;
    let unexplained_unused_count_matches =
      report.unexplained_unused_mechanics.len() == definition.expected_unexplained_unused_count;
    let strategy_diversity_matches =
      report.strategy_diversity_passes == definition.expected_strategy_diversity_passes;
    let justification_matches =
      report.all_required_mechanics_justified == definition.expected_all_mechanics_justified;
    let all_expectations_met = distinct_strategies_match
      && inactive_role_count_matches
      && unused_mechanic_count_matches
      && unexplained_unused_count_matches
      && strategy_diversity_matches
      && justification_matches;

    Ok(PopulationScenarioExecutionResult {
      scenario_id: definition.scenario_id,
      report,
      distinct_strategies_match,
      inactive_role_count_matches,
      unused_mechanic_count_matches,
      unexplained_unused_count_matches,
      strategy_diversity_matches,
      justification_matches,
      all_expectations_met,
    })
  }

  fn diverse_engaged_observations() -> Vec<ReplaySummary> {
    vec![
      ReplaySummary {
        replay_id: "replay-early-pick-blitz",
        strategy: CompositionArchetype::EarlyPick,
        active_roles: &ALL_ROLES,
        communication_events: 14,
        mechanics_used: &[
          MechanicKind::Rotation,
          MechanicKind::ObjectiveContest,
          MechanicKind::VisionControl,
          MechanicKind::RoleTactics,
          MechanicKind::TeamCommunication,
        ],
      },
      ReplaySummary {
        replay_id: "replay-teamfight-scaling",
        strategy: CompositionArchetype::TeamfightScaling,
        active_roles: &ALL_ROLES,
        communication_events: 11,
        mechanics_used: &[
          MechanicKind::Rotation,
          MechanicKind::ObjectiveContest,
          MechanicKind::StructureSiege,
          MechanicKind::ComebackPlay,
          MechanicKind::RoleTactics,
          MechanicKind::TeamCommunication,
          MechanicKind::PivotalReview,
        ],
      },
      ReplaySummary {
        replay_id: "replay-split-pressure",
        strategy: CompositionArchetype::SplitPush,
        active_roles: &[MatchRole::TopLaner, MatchRole::Jungler, MatchRole::MidLaner],
        communication_events: 6,
        mechanics_used: &[
          MechanicKind::Rotation,
          MechanicKind::StructureSiege,
          MechanicKind::VisionControl,
          MechanicKind::RoleTactics,
          MechanicKind::TeamCommunication,
          MechanicKind::PivotalReview,
        ],
      },
      ReplaySummary {
        replay_id: "replay-poke-siege",
        strategy: CompositionArchetype::PokeSiege,
        active_roles: &[MatchRole::MidLaner, MatchRole::BotCarry, MatchRole::Support],
        communication_events: 9,
        mechanics_used: &[
          MechanicKind::VisionControl,
          MechanicKind::StructureSiege,
          MechanicKind::ObjectiveContest,
          MechanicKind::RoleTactics,
          MechanicKind::TeamCommunication,
        ],
      },
    ]
  }

  fn narrow_passive_observations() -> Vec<ReplaySummary> {
    const FOUR_ROLES: [MatchRole; 4] = [
      MatchRole::TopLaner,
      MatchRole::Jungler,
      MatchRole::MidLaner,
      MatchRole::BotCarry,
    ];
    vec![
      ReplaySummary {
        replay_id: "replay-narrow-one",
        strategy: CompositionArchetype::EarlyPick,
        active_roles: &FOUR_ROLES,
        communication_events: 0,
        mechanics_used: &[MechanicKind::Rotation, MechanicKind::RoleTactics],
      },
      ReplaySummary {
        replay_id: "replay-narrow-two",
        strategy: CompositionArchetype::EarlyPick,
        active_roles: &FOUR_ROLES,
        communication_events: 0,
        mechanics_used: &[MechanicKind::Rotation, MechanicKind::RoleTactics],
      },
      ReplaySummary {
        replay_id: "replay-narrow-three",
        strategy: CompositionArchetype::EarlyPick,
        active_roles: &FOUR_ROLES,
        communication_events: 0,
        mechanics_used: &[MechanicKind::RoleTactics],
      },
    ]
  }

  fn exempted_observations() -> Vec<ReplaySummary> {
    vec![
      ReplaySummary {
        replay_id: "replay-split-clean",
        strategy: CompositionArchetype::SplitPush,
        active_roles: &ALL_ROLES,
        communication_events: 8,
        mechanics_used: &[
          MechanicKind::Rotation,
          MechanicKind::VisionControl,
          MechanicKind::StructureSiege,
          MechanicKind::RoleTactics,
          MechanicKind::TeamCommunication,
          MechanicKind::ObjectiveContest,
        ],
      },
      ReplaySummary {
        replay_id: "replay-poke-clean",
        strategy: CompositionArchetype::PokeSiege,
        active_roles: &ALL_ROLES,
        communication_events: 5,
        mechanics_used: &[
          MechanicKind::Rotation,
          MechanicKind::VisionControl,
          MechanicKind::StructureSiege,
          MechanicKind::RoleTactics,
          MechanicKind::TeamCommunication,
          MechanicKind::ObjectiveContest,
        ],
      },
    ]
  }
}
