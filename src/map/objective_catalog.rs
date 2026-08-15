//! Canonical benchmark scenarios for objective contests, vision control, and cross-map tradeoffs for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use crate::kernel::{ActorId, StateHash, hash_bytes};

use super::contest::{CrossMapTradeTarget, ObjectiveIntent, transition_objective_contest};
use super::objective::{MatchObjectiveState, ObjectiveEntry, ObjectiveKind, ObjectiveStatus};
use super::state::FNV_OFFSET_BASIS;
use super::topology::{MapLocation, TeamSide};
use super::travel::ActorLocation;
use super::vision::{DEFAULT_WARD_DURATION_TURNS, MapVisionState};

/// Benchmark scenario specification for objective contest and vision simulation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectiveScenarioDefinition {
  pub scenario_id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub initial_turn: u32,
  pub expected_final_turn: u32,
}

/// Execution outcome of running a canonical objective scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub final_turn: u32,
  pub total_events: usize,
  pub total_effects: usize,
  pub final_state_hash: StateHash,
  pub initial_state_hash: StateHash,
}

/// Catalog of registered canonical objective scenarios for M9.
pub struct ObjectiveScenarioCatalog;

impl ObjectiveScenarioCatalog {
  pub const SCENARIO_DRAGON_CONTEST: ObjectiveScenarioDefinition = ObjectiveScenarioDefinition {
    scenario_id: "scenario-dragon-contest-v1",
    name: "Bot River Dragon Vision & Secure Contest",
    description: "Allied team establishes vision priority and executes a decisive secure burst on Drake.",
    initial_turn: 3,
    expected_final_turn: 5,
  };

  pub const SCENARIO_CROSS_MAP_TRADE: ObjectiveScenarioDefinition = ObjectiveScenarioDefinition {
    scenario_id: "scenario-cross-map-trade-v1",
    name: "Cross-Map Dragon Concession for Top Herald & Mid Pressure",
    description: "Allied team concedes Bot Drake to secure Top Herald and mid lane tower pressure (+500 bp).",
    initial_turn: 4,
    expected_final_turn: 5,
  };

  pub const SCENARIO_VISION_SETUP_AND_CATCH: ObjectiveScenarioDefinition =
    ObjectiveScenarioDefinition {
      scenario_id: "scenario-vision-setup-and-catch-v1",
      name: "Top River Ward Placement and Flank Detection",
      description: "A defensive river ward spots an enemy rotation early, preventing a deadly mid-lane collapse.",
      initial_turn: 2,
      expected_final_turn: 4,
    };

  pub const SCENARIO_STEALTH_OBJECTIVE_SNEAK: ObjectiveScenarioDefinition =
    ObjectiveScenarioDefinition {
      scenario_id: "scenario-stealth-objective-sneak-v1",
      name: "De-Warding and Undetected Dragon Sneak",
      description: "Allied team clears enemy river vision and secures Drake under fog-of-war cover.",
      initial_turn: 4,
      expected_final_turn: 5,
    };

  pub const ALL_SCENARIOS: [ObjectiveScenarioDefinition; 4] = [
    Self::SCENARIO_DRAGON_CONTEST,
    Self::SCENARIO_CROSS_MAP_TRADE,
    Self::SCENARIO_VISION_SETUP_AND_CATCH,
    Self::SCENARIO_STEALTH_OBJECTIVE_SNEAK,
  ];

  pub fn list_scenarios() -> &'static [ObjectiveScenarioDefinition] {
    &Self::ALL_SCENARIOS
  }

  pub fn get_scenario(id: &str) -> Option<&'static ObjectiveScenarioDefinition> {
    Self::ALL_SCENARIOS.iter().find(|s| s.scenario_id == id)
  }

  /// Compute deterministic FNV-1a hash over combined objective and vision state.
  pub fn compute_hash(
    turn: u32,
    objectives: &MatchObjectiveState,
    vision: &MapVisionState,
  ) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, &turn.to_le_bytes());

    for entry in objectives.entries() {
      let kind_tag: u8 = match entry.kind {
        ObjectiveKind::TopRiverObjective => 1,
        ObjectiveKind::BotRiverObjective => 2,
      };
      let status_tag: u8 = match entry.status {
        ObjectiveStatus::Unspawned { .. } => 1,
        ObjectiveStatus::Active { .. } => 2,
        ObjectiveStatus::Secured { .. } => 3,
      };
      let hp = entry.status.current_health().unwrap_or(0);
      hash = hash_bytes(hash, &[kind_tag, status_tag]);
      hash = hash_bytes(hash, &hp.to_le_bytes());
      hash = hash_bytes(hash, &entry.secure_count_allied.to_le_bytes());
      hash = hash_bytes(hash, &entry.secure_count_opposing.to_le_bytes());
    }

    for ward in vision.active_wards() {
      let team_tag: u8 = match ward.team {
        TeamSide::Allied => 1,
        TeamSide::Opposing => 2,
      };
      let loc_idx: u8 = u8::try_from(ward.location.index()).unwrap_or_default();
      hash = hash_bytes(
        hash,
        &[
          team_tag,
          loc_idx,
          ward.placed_by.value(),
          u8::try_from(ward.remaining_turns).unwrap_or_default(),
        ],
      );
    }

    StateHash::from_raw(hash)
  }

  /// Execute a canonical scenario from the catalog.
  pub fn execute_scenario(
    scenario_id: &str,
  ) -> Result<ObjectiveScenarioExecutionResult, &'static str> {
    match scenario_id {
      "scenario-dragon-contest-v1" => Self::run_dragon_contest(),
      "scenario-cross-map-trade-v1" => Self::run_cross_map_trade(),
      "scenario-vision-setup-and-catch-v1" => Self::run_vision_setup_and_catch(),
      "scenario-stealth-objective-sneak-v1" => Self::run_stealth_objective_sneak(),
      _ => Err("unknown objective scenario id"),
    }
  }

  fn run_dragon_contest() -> Result<ObjectiveScenarioExecutionResult, &'static str> {
    let mut turn = 3;
    let mut objectives = MatchObjectiveState::new_with_entries([
      ObjectiveEntry::new_unspawned(ObjectiveKind::TopRiverObjective),
      ObjectiveEntry {
        kind: ObjectiveKind::BotRiverObjective,
        status: ObjectiveStatus::Unspawned {
          turns_until_spawn: 1,
        },
        secure_count_allied: 0,
        secure_count_opposing: 0,
      },
    ]);
    let mut vision = MapVisionState::new();
    let initial_hash = Self::compute_hash(turn, &objectives, &vision);

    let mut total_events = 0;
    let mut total_effects = 0;

    // Turn 3 -> Turn 4: Allied places ward at Bot River, Drake spawns
    turn = 4;
    vision
      .place_ward(
        TeamSide::Allied,
        ActorId::new(1),
        MapLocation::BOT_RIVER,
        turn,
        DEFAULT_WARD_DURATION_TURNS,
      )
      .map_err(|_| "failed to place ward")?;
    total_events += 1; // WardPlaced
    total_effects += 1; // VisionGranted

    let res1 = transition_objective_contest(
      &mut objectives,
      &mut vision,
      Some(ObjectiveIntent::Engage {
        objective: ObjectiveKind::BotRiverObjective,
        damage: 2000,
      }),
      Some(ObjectiveIntent::Engage {
        objective: ObjectiveKind::BotRiverObjective,
        damage: 500,
      }),
      turn,
    );
    total_events += res1.events.len();
    total_effects += res1.effects.len();

    // Turn 4 -> Turn 5: Allied bursts remaining 1000 HP and secures
    turn = 5;
    let res2 = transition_objective_contest(
      &mut objectives,
      &mut vision,
      Some(ObjectiveIntent::SecureBurst {
        objective: ObjectiveKind::BotRiverObjective,
        burst_damage: 1500,
      }),
      Some(ObjectiveIntent::ZoneOpponents {
        objective: ObjectiveKind::BotRiverObjective,
        zoning_power: 500,
      }),
      turn,
    );
    total_events += res2.events.len();
    total_effects += res2.effects.len();

    let final_hash = Self::compute_hash(turn, &objectives, &vision);
    Ok(ObjectiveScenarioExecutionResult {
      scenario_id: Self::SCENARIO_DRAGON_CONTEST.scenario_id,
      final_turn: turn,
      total_events,
      total_effects,
      final_state_hash: final_hash,
      initial_state_hash: initial_hash,
    })
  }

  fn run_cross_map_trade() -> Result<ObjectiveScenarioExecutionResult, &'static str> {
    let mut turn = 4;
    let mut objectives = MatchObjectiveState::new_with_entries([
      ObjectiveEntry::new_active(ObjectiveKind::TopRiverObjective, 5000),
      ObjectiveEntry::new_active(ObjectiveKind::BotRiverObjective, 3500),
    ]);
    let mut vision = MapVisionState::new();
    let initial_hash = Self::compute_hash(turn, &objectives, &vision);

    let mut total_events = 0;
    let mut total_effects = 0;

    // Turn 4 -> Turn 5: Opponents take Bot Drake; Allied executes cross-map trade for Top Herald
    turn = 5;
    let res = transition_objective_contest(
      &mut objectives,
      &mut vision,
      Some(ObjectiveIntent::ConcedeAndTrade {
        conceded: ObjectiveKind::BotRiverObjective,
        target: CrossMapTradeTarget::OppositeObjective(ObjectiveKind::TopRiverObjective),
        execution_efficiency_bp: 10_000,
      }),
      Some(ObjectiveIntent::SecureBurst {
        objective: ObjectiveKind::BotRiverObjective,
        burst_damage: 3500,
      }),
      turn,
    );
    total_events += res.events.len();
    total_effects += res.effects.len();

    let final_hash = Self::compute_hash(turn, &objectives, &vision);
    Ok(ObjectiveScenarioExecutionResult {
      scenario_id: Self::SCENARIO_CROSS_MAP_TRADE.scenario_id,
      final_turn: turn,
      total_events,
      total_effects,
      final_state_hash: final_hash,
      initial_state_hash: initial_hash,
    })
  }

  fn run_vision_setup_and_catch() -> Result<ObjectiveScenarioExecutionResult, &'static str> {
    let mut turn = 2;
    let mut objectives = MatchObjectiveState::new();
    let mut vision = MapVisionState::new();
    let initial_hash = Self::compute_hash(turn, &objectives, &vision);

    let mut total_events = 0;
    let mut total_effects = 0;

    // Turn 2: Allied mid places ward in Top River
    vision
      .place_ward(
        TeamSide::Allied,
        ActorId::new(2),
        MapLocation::TOP_RIVER,
        turn,
        DEFAULT_WARD_DURATION_TURNS,
      )
      .map_err(|_| "failed to place ward")?;
    total_events += 1; // WardPlaced
    total_effects += 1; // VisionGranted

    // Turn 3: Tick turn, enemy passes through Top River
    turn = 3;
    let grid = vision.compute_team_vision(
      TeamSide::Allied,
      &[(
        ActorId::new(2),
        ActorLocation::Stationary(MapLocation::MID_CENTER),
        TeamSide::Allied,
      )],
      turn,
      None,
    );
    if !grid.is_visible(MapLocation::TOP_RIVER) {
      return Err("ward failed to grant vision at top river");
    }

    let res = transition_objective_contest(&mut objectives, &mut vision, None, None, turn);
    total_events += res.events.len();
    total_effects += res.effects.len();

    turn = 4;
    let res2 = transition_objective_contest(&mut objectives, &mut vision, None, None, turn);
    total_events += res2.events.len();
    total_effects += res2.effects.len();

    let final_hash = Self::compute_hash(turn, &objectives, &vision);
    Ok(ObjectiveScenarioExecutionResult {
      scenario_id: Self::SCENARIO_VISION_SETUP_AND_CATCH.scenario_id,
      final_turn: turn,
      total_events,
      total_effects,
      final_state_hash: final_hash,
      initial_state_hash: initial_hash,
    })
  }

  fn run_stealth_objective_sneak() -> Result<ObjectiveScenarioExecutionResult, &'static str> {
    let mut turn = 4;
    let mut objectives = MatchObjectiveState::new_with_entries([
      ObjectiveEntry::new_unspawned(ObjectiveKind::TopRiverObjective),
      ObjectiveEntry::new_active(ObjectiveKind::BotRiverObjective, 3500),
    ]);
    let mut vision = MapVisionState::new();

    // Opponent had placed a ward at Bot River
    vision
      .place_ward(
        TeamSide::Opposing,
        ActorId::new(5),
        MapLocation::BOT_RIVER,
        turn,
        DEFAULT_WARD_DURATION_TURNS,
      )
      .map_err(|_| "failed to place opposing ward")?;

    let initial_hash = Self::compute_hash(turn, &objectives, &vision);
    let mut total_events = 1; // initial ward placed
    let mut total_effects = 1; // initial vision granted

    // Allied clears opposing ward and starts Drake sneak
    vision
      .clear_ward(MapLocation::BOT_RIVER, TeamSide::Allied)
      .map_err(|_| "failed to clear opposing ward")?;
    total_events += 1; // WardCleared

    let res1 = transition_objective_contest(
      &mut objectives,
      &mut vision,
      Some(ObjectiveIntent::Engage {
        objective: ObjectiveKind::BotRiverObjective,
        damage: 2000,
      }),
      None,
      turn,
    );
    total_events += res1.events.len();
    total_effects += res1.effects.len();

    // Turn 5: Allied finishes sneak burst
    turn = 5;
    let res2 = transition_objective_contest(
      &mut objectives,
      &mut vision,
      Some(ObjectiveIntent::SecureBurst {
        objective: ObjectiveKind::BotRiverObjective,
        burst_damage: 1500,
      }),
      None,
      turn,
    );
    total_events += res2.events.len();
    total_effects += res2.effects.len();

    let final_hash = Self::compute_hash(turn, &objectives, &vision);
    Ok(ObjectiveScenarioExecutionResult {
      scenario_id: Self::SCENARIO_STEALTH_OBJECTIVE_SNEAK.scenario_id,
      final_turn: turn,
      total_events,
      total_effects,
      final_state_hash: final_hash,
      initial_state_hash: initial_hash,
    })
  }
}
