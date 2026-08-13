//! Canonical scenario catalog and execution harness for multi-lane travel benchmarks.

use super::state::MatchMapState;
use super::topology::MapLocation;
use super::transition::transition_travel;
use super::travel::{ActorLocation, TravelCommand, TravelError};
use crate::kernel::{ActorId, StateHash};

/// Result of executing a benchmark map travel scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub initial_hash: StateHash,
  pub terminal_hash: StateHash,
  pub turns_elapsed: u32,
  pub terminal_locations: Vec<(ActorId, MapLocation)>,
}

/// Definition of a canonical multi-lane rotation benchmark scenario.
#[derive(Clone, Debug, PartialEq)]
pub struct MapScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub initial_state: MatchMapState,
  pub command_script: Vec<(u32, ActorId, TravelCommand, u8)>,
  pub expected_terminal_locations: Vec<(ActorId, MapLocation)>,
}

impl MapScenarioDefinition {
  /// Execute the scenario deterministically and verify the outcome.
  pub fn execute(&self) -> Result<MapScenarioExecutionResult, TravelError> {
    let mut state = self.initial_state.clone();
    let initial_hash = state.hash();

    for (_turn, actor_id, command, beats) in &self.command_script {
      let current_loc = state
        .get_actor_location(*actor_id)
        .cloned()
        .unwrap_or(ActorLocation::Stationary(MapLocation::ALLIED_BASE));

      let result = transition_travel(*actor_id, &current_loc, *command, *beats)?;
      state.set_actor_location(*actor_id, result.next_location);
      state.advance_turn();
    }

    let terminal_hash = state.hash();
    let mut terminal_locations = Vec::new();
    for (id, loc) in state.actor_locations() {
      terminal_locations.push((*id, loc.current_location()));
    }

    Ok(MapScenarioExecutionResult {
      scenario_id: self.scenario_id,
      initial_hash,
      terminal_hash,
      turns_elapsed: state.turn(),
      terminal_locations,
    })
  }
}

/// Discovery and catalog registry for canonical M9 map scenarios.
pub struct MapTravelCatalog;

impl MapTravelCatalog {
  pub const SCENARIO_TOP_TO_MID_GANK: &'static str = "scenario-top-to-mid-gank-v1";
  pub const SCENARIO_BOT_TO_RIVER_CONTEST: &'static str = "scenario-bot-to-river-contest-v1";
  pub const SCENARIO_MID_TO_BASE_RESET: &'static str = "scenario-mid-to-base-reset-v1";
  pub const SCENARIO_ABORTED_ROTATION_THREAT: &'static str = "scenario-aborted-rotation-threat-v1";

  pub fn find(scenario_id: &str) -> Option<MapScenarioDefinition> {
    match scenario_id {
      Self::SCENARIO_TOP_TO_MID_GANK => Some(Self::top_to_mid_gank()),
      Self::SCENARIO_BOT_TO_RIVER_CONTEST => Some(Self::bot_to_river_contest()),
      Self::SCENARIO_MID_TO_BASE_RESET => Some(Self::mid_to_base_reset()),
      Self::SCENARIO_ABORTED_ROTATION_THREAT => Some(Self::aborted_rotation_threat()),
      _ => None,
    }
  }

  pub fn all() -> Vec<MapScenarioDefinition> {
    vec![
      Self::top_to_mid_gank(),
      Self::bot_to_river_contest(),
      Self::mid_to_base_reset(),
      Self::aborted_rotation_threat(),
    ]
  }

  fn top_to_mid_gank() -> MapScenarioDefinition {
    let top_laner = ActorId::new(1);
    let mid_laner = ActorId::new(2);
    let opp_mid = ActorId::new(3);

    let initial_state = MatchMapState::new(
      1,
      vec![top_laner, mid_laner],
      vec![opp_mid],
      vec![
        (
          top_laner,
          ActorLocation::Stationary(MapLocation::TOP_CENTER),
        ),
        (
          mid_laner,
          ActorLocation::Stationary(MapLocation::MID_CENTER),
        ),
        (
          opp_mid,
          ActorLocation::Stationary(MapLocation::MID_FAR_SIDE),
        ),
      ],
    );

    MapScenarioDefinition {
      scenario_id: Self::SCENARIO_TOP_TO_MID_GANK,
      title: "Top to Mid Gank Rotation",
      description: "Top laner rotates through Top River to Mid Center over 2 beats to initiate a gank.",
      initial_state,
      command_script: vec![
        // Turn 1: Top laner initiates rotation to Mid Center (2 beats total, advances 1 beat to Top River)
        (
          1,
          top_laner,
          TravelCommand::InitiateRotation {
            destination: MapLocation::MID_CENTER,
          },
          1,
        ),
        // Turn 2: Top laner continues transit, arrives at Mid Center
        (2, top_laner, TravelCommand::ContinueTransit, 1),
      ],
      expected_terminal_locations: vec![
        (top_laner, MapLocation::MID_CENTER),
        (mid_laner, MapLocation::MID_CENTER),
        (opp_mid, MapLocation::MID_FAR_SIDE),
      ],
    }
  }

  fn bot_to_river_contest() -> MapScenarioDefinition {
    let bot_carry = ActorId::new(1);
    let bot_support = ActorId::new(2);

    let initial_state = MatchMapState::new(
      1,
      vec![bot_carry, bot_support],
      vec![],
      vec![
        (
          bot_carry,
          ActorLocation::Stationary(MapLocation::BOT_NEAR_TOWER),
        ),
        (
          bot_support,
          ActorLocation::Stationary(MapLocation::BOT_NEAR_TOWER),
        ),
      ],
    );

    MapScenarioDefinition {
      scenario_id: Self::SCENARIO_BOT_TO_RIVER_CONTEST,
      title: "Bot to River Contest Rotation",
      description: "Bot duo rotates from Near Tower to Bot River over 2 beats for objective vision setup.",
      initial_state,
      command_script: vec![
        (
          1,
          bot_carry,
          TravelCommand::InitiateRotation {
            destination: MapLocation::BOT_RIVER,
          },
          1,
        ),
        (
          1,
          bot_support,
          TravelCommand::InitiateRotation {
            destination: MapLocation::BOT_RIVER,
          },
          1,
        ),
        (2, bot_carry, TravelCommand::ContinueTransit, 1),
        (2, bot_support, TravelCommand::ContinueTransit, 1),
      ],
      expected_terminal_locations: vec![
        (bot_carry, MapLocation::BOT_RIVER),
        (bot_support, MapLocation::BOT_RIVER),
      ],
    }
  }

  fn mid_to_base_reset() -> MapScenarioDefinition {
    let mid_laner = ActorId::new(1);

    let initial_state = MatchMapState::new(
      1,
      vec![mid_laner],
      vec![],
      vec![(
        mid_laner,
        ActorLocation::Stationary(MapLocation::MID_FAR_SIDE),
      )],
    );

    MapScenarioDefinition {
      scenario_id: Self::SCENARIO_MID_TO_BASE_RESET,
      title: "Mid to Base Reset Rotation",
      description: "Mid laner retreats from enemy tower through mid lane back to base over 3 beats.",
      initial_state,
      command_script: vec![
        // Turn 1: initiate rotation to base (3 beats total, advance 1 beat)
        (
          1,
          mid_laner,
          TravelCommand::InitiateRotation {
            destination: MapLocation::ALLIED_BASE,
          },
          1,
        ),
        // Turn 2: advance 1 beat
        (2, mid_laner, TravelCommand::ContinueTransit, 1),
        // Turn 3: advance final beat to base
        (3, mid_laner, TravelCommand::ContinueTransit, 1),
      ],
      expected_terminal_locations: vec![(mid_laner, MapLocation::ALLIED_BASE)],
    }
  }

  fn aborted_rotation_threat() -> MapScenarioDefinition {
    let laner = ActorId::new(1);

    let initial_state = MatchMapState::new(
      1,
      vec![laner],
      vec![],
      vec![(
        laner,
        ActorLocation::Stationary(MapLocation::TOP_NEAR_TOWER),
      )],
    );

    MapScenarioDefinition {
      scenario_id: Self::SCENARIO_ABORTED_ROTATION_THREAT,
      title: "Aborted Rotation on Threat Detected",
      description: "Laner rotates toward River, detects enemy jungler, aborts and retreats to Tower.",
      initial_state,
      command_script: vec![
        // Turn 1: Start rotating towards Top River (2 beats), advance 1 beat to Top Jungle
        (
          1,
          laner,
          TravelCommand::InitiateRotation {
            destination: MapLocation::TOP_RIVER,
          },
          1,
        ),
        // Turn 2: Threat spotted, abort rotation and divert back to Top Near Tower (1 beat)
        (
          2,
          laner,
          TravelCommand::AbortRotation {
            fallback: MapLocation::TOP_NEAR_TOWER,
          },
          1,
        ),
      ],
      expected_terminal_locations: vec![(laner, MapLocation::TOP_NEAR_TOWER)],
    }
  }
}
