//! Canonical benchmark match scenarios for team composition matchups, base sieges, and victory conditions for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use crate::kernel::StateHash;

use super::structures::{
  MatchStructureState, SiegeIntent, StructureTier, transition_structure_siege,
};
use super::topology::{LaneId, TeamSide};
use super::victory::{MatchStatus, MatchTerminalEvaluation, MatchVictoryCondition};

/// Benchmark scenario specification for a multi-lane team match.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub initial_turn: u32,
  pub expected_final_turn: u32,
  pub expected_winner: TeamSide,
  pub expected_condition: MatchVictoryCondition,
}

/// Execution outcome of running a canonical match scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub final_turn: u32,
  pub match_status: MatchStatus,
  pub total_events: usize,
  pub total_effects: usize,
  pub initial_state_hash: StateHash,
  pub final_state_hash: StateHash,
}

/// Catalog of registered canonical match scenarios for M9.
pub struct MatchScenarioCatalog;

impl MatchScenarioCatalog {
  pub const SCENARIO_EARLY_PICK_SNOWBALL: MatchScenarioDefinition = MatchScenarioDefinition {
    scenario_id: "scenario-early-pick-snowball-v1",
    name: "Early Pick Snowball & Mid Lane Nexus Demolition",
    description: "Allied Early Pick comp dominates early skirmishes, tears through Mid defenses, and finishes the game at turn 18.",
    initial_turn: 1,
    expected_final_turn: 18,
    expected_winner: TeamSide::Allied,
    expected_condition: MatchVictoryCondition::NexusDemolished,
  };

  pub const SCENARIO_SPLIT_PUSH_BASE_RACE: MatchScenarioDefinition = MatchScenarioDefinition {
    scenario_id: "scenario-split-push-base-race-v1",
    name: "Split-Push Bot Base Race Trade",
    description: "Allied Split-Push comp trades an enemy Top Baron for a blistering Bot side-lane push straight to Nexus victory at turn 22.",
    initial_turn: 15,
    expected_final_turn: 22,
    expected_winner: TeamSide::Allied,
    expected_condition: MatchVictoryCondition::NexusDemolished,
  };

  pub const SCENARIO_LATE_GAME_SCALING_COMEBACK: MatchScenarioDefinition =
    MatchScenarioDefinition {
      scenario_id: "scenario-late-game-scaling-comeback-v1",
      name: "Late-Game Teamfight Scaling Comeback",
      description: "Allied Scaling comp defends Tier 3 high ground until turn 25, wins a decisive ace, and marches to victory at turn 28.",
      initial_turn: 20,
      expected_final_turn: 28,
      expected_winner: TeamSide::Allied,
      expected_condition: MatchVictoryCondition::NexusDemolished,
    };

  pub const SCENARIO_SIEGE_INHIBITOR_CONCESSION: MatchScenarioDefinition =
    MatchScenarioDefinition {
      scenario_id: "scenario-siege-inhibitor-concession-v1",
      name: "Triple Inhibitor Siege & Super Minion Concession",
      description: "Allied Poke/Siege comp methodically destroys all 3 enemy inhibitors; overwhelming super minion waves trigger match concession at turn 24.",
      initial_turn: 16,
      expected_final_turn: 24,
      expected_winner: TeamSide::Allied,
      expected_condition: MatchVictoryCondition::MatchConceded,
    };

  pub const ALL_SCENARIOS: [MatchScenarioDefinition; 4] = [
    Self::SCENARIO_EARLY_PICK_SNOWBALL,
    Self::SCENARIO_SPLIT_PUSH_BASE_RACE,
    Self::SCENARIO_LATE_GAME_SCALING_COMEBACK,
    Self::SCENARIO_SIEGE_INHIBITOR_CONCESSION,
  ];

  pub fn list_scenarios() -> &'static [MatchScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static MatchScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Execute a canonical match benchmark scenario and return verifiable outcome metrics.
  pub fn execute_scenario(scenario_id: &str) -> Result<MatchScenarioExecutionResult, &'static str> {
    match scenario_id {
      "scenario-early-pick-snowball-v1" => Self::run_early_pick_snowball(),
      "scenario-split-push-base-race-v1" => Self::run_split_push_base_race(),
      "scenario-late-game-scaling-comeback-v1" => Self::run_late_game_scaling_comeback(),
      "scenario-siege-inhibitor-concession-v1" => Self::run_siege_inhibitor_concession(),
      _ => Err("unknown match scenario identifier"),
    }
  }

  fn run_early_pick_snowball() -> Result<MatchScenarioExecutionResult, &'static str> {
    let mut structures = MatchStructureState::new_standard_map();
    let initial_hash = structures.compute_hash(1);
    let mut total_events = 0;
    let mut total_effects = 0;

    // Turn 8: Demolish Opposing Mid Outer Turret (3500 HP)
    let res = transition_structure_siege(
      8,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4000,
      },
      None,
    )
    .map_err(|_| "failed turn 8 outer turret siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 12: Demolish Opposing Mid Inner Turret (4000 HP)
    let res = transition_structure_siege(
      12,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4500,
      },
      None,
    )
    .map_err(|_| "failed turn 12 inner turret siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 15: Demolish Opposing Mid Inhibitor Turret (4500 HP)
    let res = transition_structure_siege(
      15,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 5000,
      },
      None,
    )
    .map_err(|_| "failed turn 15 inhibitor turret siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 16: Destroy Opposing Mid Inhibitor (3000 HP) -> Spawns Super Minions
    let res = transition_structure_siege(
      16,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Mid),
        raw_damage: 3500,
      },
      None,
    )
    .map_err(|_| "failed turn 16 inhibitor siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();
    assert!(res.super_minions_spawned);

    // Turn 18: Demolish Opposing Nexus (6000 HP) -> Match Concludes
    let res = transition_structure_siege(
      18,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Nexus,
        lane: None,
        raw_damage: 6500,
      },
      None,
    )
    .map_err(|_| "failed turn 18 nexus siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();
    assert!(res.match_concluded);

    let final_hash = structures.compute_hash(18);
    let eval = MatchTerminalEvaluation::evaluate(18, &structures, 3, 1);

    Ok(MatchScenarioExecutionResult {
      scenario_id: "scenario-early-pick-snowball-v1",
      final_turn: 18,
      match_status: eval.status,
      total_events,
      total_effects,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn run_split_push_base_race() -> Result<MatchScenarioExecutionResult, &'static str> {
    let mut structures = MatchStructureState::new_standard_map();
    let initial_hash = structures.compute_hash(15);
    let mut total_events = 0;
    let mut total_effects = 0;

    // Prior turns pre-damaged Bot Outer & Inner
    transition_structure_siege(
      10,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Bot),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      14,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Bot),
        raw_damage: 4500,
      },
      None,
    )
    .unwrap();

    // Turn 19: Demolish Bot Inhibitor Turret during cross-map Baron distraction
    let res = transition_structure_siege(
      19,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Bot),
        raw_damage: 5000,
      },
      None,
    )
    .map_err(|_| "failed turn 19 bot inhibitor turret siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 20: Destroy Bot Inhibitor
    let res = transition_structure_siege(
      20,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Bot),
        raw_damage: 3500,
      },
      None,
    )
    .map_err(|_| "failed turn 20 bot inhibitor siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 22: Destroy Nexus in base race
    let res = transition_structure_siege(
      22,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Nexus,
        lane: None,
        raw_damage: 6500,
      },
      None,
    )
    .map_err(|_| "failed turn 22 nexus base race siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();
    assert!(res.match_concluded);

    let final_hash = structures.compute_hash(22);
    let eval = MatchTerminalEvaluation::evaluate(22, &structures, 2, 2);

    Ok(MatchScenarioExecutionResult {
      scenario_id: "scenario-split-push-base-race-v1",
      final_turn: 22,
      match_status: eval.status,
      total_events,
      total_effects,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn run_late_game_scaling_comeback() -> Result<MatchScenarioExecutionResult, &'static str> {
    let mut structures = MatchStructureState::new_standard_map();
    let initial_hash = structures.compute_hash(20);
    let mut total_events = 0;
    let mut total_effects = 0;

    // Opposing team took Allied Top & Mid Outer Turrets earlier
    transition_structure_siege(
      9,
      &mut structures,
      TeamSide::Opposing,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Top),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      13,
      &mut structures,
      TeamSide::Opposing,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();

    // Turn 24: Allied team tears down Opposing Mid Outer & Inner after winning teamfight
    transition_structure_siege(
      24,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      25,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4500,
      },
      None,
    )
    .unwrap();

    // Turn 26: Demolish Mid Inhibitor Turret
    let res = transition_structure_siege(
      26,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 5000,
      },
      None,
    )
    .map_err(|_| "failed turn 26 inhibitor turret siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 27: Destroy Mid Inhibitor
    let res = transition_structure_siege(
      27,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Mid),
        raw_damage: 3500,
      },
      None,
    )
    .map_err(|_| "failed turn 27 inhibitor siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    // Turn 28: Destroy Nexus
    let res = transition_structure_siege(
      28,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Nexus,
        lane: None,
        raw_damage: 6500,
      },
      None,
    )
    .map_err(|_| "failed turn 28 nexus siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    let final_hash = structures.compute_hash(28);
    let eval = MatchTerminalEvaluation::evaluate(28, &structures, 4, 1);

    Ok(MatchScenarioExecutionResult {
      scenario_id: "scenario-late-game-scaling-comeback-v1",
      final_turn: 28,
      match_status: eval.status,
      total_events,
      total_effects,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }

  fn run_siege_inhibitor_concession() -> Result<MatchScenarioExecutionResult, &'static str> {
    let mut structures = MatchStructureState::new_standard_map();
    let initial_hash = structures.compute_hash(16);
    let mut total_events = 0;
    let mut total_effects = 0;

    // Destroy Top Lane defenses
    transition_structure_siege(
      10,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Top),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      12,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Top),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      14,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Top),
        raw_damage: 5000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      15,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Top),
        raw_damage: 3500,
      },
      None,
    )
    .unwrap();

    // Destroy Mid Lane defenses
    transition_structure_siege(
      16,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      17,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      18,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 5000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      19,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Mid),
        raw_damage: 3500,
      },
      None,
    )
    .unwrap();

    // Destroy Bot Lane defenses
    transition_structure_siege(
      20,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Bot),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      21,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Bot),
        raw_damage: 4000,
      },
      None,
    )
    .unwrap();
    transition_structure_siege(
      22,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Bot),
        raw_damage: 5000,
      },
      None,
    )
    .unwrap();
    let res = transition_structure_siege(
      24,
      &mut structures,
      TeamSide::Allied,
      SiegeIntent::AttackStructure {
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Bot),
        raw_damage: 3500,
      },
      None,
    )
    .map_err(|_| "failed turn 24 bot inhibitor siege")?;
    total_events += res.events.len();
    total_effects += res.effects.len();

    let final_hash = structures.compute_hash(24);
    // Allied secured 4 objectives vs 1 opposing, all 3 opposing inhibitors down -> triggers MatchConceded
    let eval = MatchTerminalEvaluation::evaluate(24, &structures, 4, 1);

    Ok(MatchScenarioExecutionResult {
      scenario_id: "scenario-siege-inhibitor-concession-v1",
      final_turn: 24,
      match_status: eval.status,
      total_events,
      total_effects,
      initial_state_hash: initial_hash,
      final_state_hash: final_hash,
    })
  }
}
