//! Composed complete-match execution for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This module composes the delivered M9 mechanic families — map rotations,
//! vision warding, neutral objective contests, and structure sieges — into
//! one deterministic match run that terminates through the match victory
//! conditions and replays to an identical combined hash. Each action in a
//! `CompleteMatchPlan` drives the corresponding real transition function;
//! the runner never re-implements subsystem rules.
//!
//! The combined hash commits every subsystem state: the map and structure
//! hashes plus serialized objective and ward state and the runner's secure
//! counters. Identical plans therefore always replay to the identical final
//! hash. A plan that ends without a terminal condition, or that continues
//! after the match concluded, fails closed.
//!
//! Rotations resolve within their action using the full BFS route length,
//! modeling a committed rotation that completes over the inter-turn window.
//!
//! Tick model: every action advances the shared turn, the map's internal
//! turn, and structure respawn countdowns. Objective spawn/respawn
//! countdowns and ward expiry tick only inside `ContestObjectives` actions,
//! following the contest transition's existing contract; a plan without
//! contest actions therefore carries wards and unspawned objectives forward
//! unchanged.

use core::fmt;

use super::contest::{ObjectiveIntent, transition_objective_contest};
use super::graph::distance_in_beats;
use super::objective::{MatchObjectiveState, ObjectiveKind, ObjectiveStatus};
use super::state::MatchMapState;
use super::structures::{
  MatchStructureState, SiegeIntent, StructureError, StructureTier, transition_structure_siege,
};
use super::topology::{LaneId, MapLocation, TeamSide};
use super::transition::transition_travel;
use super::travel::{TravelCommand, TravelError};
use super::victory::{MatchStatus, MatchTerminalEvaluation, MatchVictoryCondition};
use super::vision::{MapVisionState, VisionError};
use crate::kernel::{ActorId, StateHash, hash_bytes};

pub const M9_COMPLETE_MATCH_SCHEMA_V1: &str = "m9-complete-match-v1";

/// FNV-1a offset basis for the combined match hash.
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

/// One composed match action; each action occupies one match turn.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteMatchAction {
  /// Commit a rotation from the actor's current location to a destination,
  /// resolving over the full route length within the turn.
  Rotate {
    actor: ActorId,
    destination: MapLocation,
  },
  /// Place a vision ward.
  PlaceWard {
    team: TeamSide,
    placed_by: ActorId,
    location: MapLocation,
    duration_turns: u32,
  },
  /// Run one objective-contest turn, ticking objective spawns and resolving
  /// declared intents.
  ContestObjectives {
    allied_intent: Option<ObjectiveIntent>,
    opposing_intent: Option<ObjectiveIntent>,
  },
  /// Attack one structure through the siege transition.
  SiegeStructure {
    side: TeamSide,
    tier: StructureTier,
    lane: Option<LaneId>,
    raw_damage: u32,
  },
  /// Evaluate match termination; required as the final action.
  EvaluateTerminal,
}

/// Kind label of one executed phase, for the phase log.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchPhaseKind {
  Rotation,
  Warding,
  ObjectiveContest,
  StructureSiege,
  TerminalEvaluation,
}

impl MatchPhaseKind {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Rotation => "rotation",
      Self::Warding => "warding",
      Self::ObjectiveContest => "objective-contest",
      Self::StructureSiege => "structure-siege",
      Self::TerminalEvaluation => "terminal-evaluation",
    }
  }
}

impl fmt::Display for MatchPhaseKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Log record for one executed action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchPhaseRecord {
  pub turn: u32,
  pub kind: MatchPhaseKind,
  pub events: usize,
  pub effects: usize,
}

/// Typed fail-closed error for composed match execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteMatchError {
  /// The plan has no actions.
  EmptyPlan,
  /// The plan ended while the match was still in progress.
  MatchDidNotTerminate,
  /// An action was attempted after the match had concluded.
  MatchAlreadyConcluded,
  /// A rotation was requested for an actor absent from the roster.
  UntrackedActor,
  /// A rotation was rejected by the travel transition.
  Travel(TravelError),
  /// A ward placement was rejected by the vision state.
  Vision(VisionError),
  /// A siege was rejected by the structure transition.
  Siege(StructureError),
}

impl fmt::Display for CompleteMatchError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyPlan => f.write_str("empty plan: at least one action is required"),
      Self::MatchDidNotTerminate => {
        f.write_str("match did not terminate: the plan ended with the match in progress")
      }
      Self::MatchAlreadyConcluded => {
        f.write_str("match already concluded: no further actions may execute")
      }
      Self::UntrackedActor => {
        f.write_str("untracked actor: rotation requested for an actor absent from the roster")
      }
      Self::Travel(error) => write!(f, "rotation failed: {error}"),
      Self::Vision(error) => write!(f, "ward placement failed: {error}"),
      Self::Siege(error) => write!(f, "siege failed: {error}"),
    }
  }
}

impl From<TravelError> for CompleteMatchError {
  fn from(error: TravelError) -> Self {
    Self::Travel(error)
  }
}

impl From<VisionError> for CompleteMatchError {
  fn from(error: VisionError) -> Self {
    Self::Vision(error)
  }
}

impl From<StructureError> for CompleteMatchError {
  fn from(error: StructureError) -> Self {
    Self::Siege(error)
  }
}

/// Integrated authoritative state of one composed match.
///
/// Each subsystem stays owned by its own state machine; the composed state
/// only sequences them and commits their combined shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteMatchState {
  turn: u32,
  map: MatchMapState,
  objectives: MatchObjectiveState,
  vision: MapVisionState,
  structures: MatchStructureState,
  allied_objectives_secured: u32,
  opposing_objectives_secured: u32,
}

impl CompleteMatchState {
  /// Start a match from the given roster and locations with fresh objective,
  /// vision, and structure state.
  pub fn new(
    initial_turn: u32,
    allied_actors: Vec<ActorId>,
    opposing_actors: Vec<ActorId>,
    initial_locations: Vec<(ActorId, super::travel::ActorLocation)>,
  ) -> Self {
    Self {
      turn: initial_turn,
      map: MatchMapState::new(
        initial_turn,
        allied_actors,
        opposing_actors,
        initial_locations,
      ),
      objectives: MatchObjectiveState::new(),
      vision: MapVisionState::new(),
      structures: MatchStructureState::new_standard_map(),
      allied_objectives_secured: 0,
      opposing_objectives_secured: 0,
    }
  }

  pub fn turn(&self) -> u32 {
    self.turn
  }

  pub fn allied_objectives_secured(&self) -> u32 {
    self.allied_objectives_secured
  }

  pub fn opposing_objectives_secured(&self) -> u32 {
    self.opposing_objectives_secured
  }

  /// Deterministic FNV-1a hash committing every subsystem state plus the
  /// turn and secure counters.
  pub fn combined_hash(&self) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, &self.turn.to_le_bytes());
    hash = hash_bytes(hash, &self.map.hash().value().to_le_bytes());
    hash = hash_bytes(
      hash,
      &self
        .structures
        .compute_hash(self.turn)
        .value()
        .to_le_bytes(),
    );
    for entry in self.objectives.entries() {
      hash = hash_bytes(hash, &[objective_kind_tag(entry.kind)]);
      match entry.status {
        ObjectiveStatus::Unspawned { turns_until_spawn } => {
          hash = hash_bytes(hash, &[0]);
          hash = hash_bytes(hash, &turns_until_spawn.to_le_bytes());
        }
        ObjectiveStatus::Active {
          current_health,
          max_health,
          engaged_by,
        } => {
          hash = hash_bytes(hash, &[1]);
          hash = hash_bytes(hash, &current_health.to_le_bytes());
          hash = hash_bytes(hash, &max_health.to_le_bytes());
          hash = hash_bytes(hash, &[engaged_by.map_or(0, team_side_tag)]);
        }
        ObjectiveStatus::Secured {
          secured_by,
          secured_turn,
          turns_until_respawn,
        } => {
          hash = hash_bytes(hash, &[2]);
          hash = hash_bytes(hash, &[team_side_tag(secured_by)]);
          hash = hash_bytes(hash, &secured_turn.to_le_bytes());
          hash = hash_bytes(hash, &turns_until_respawn.to_le_bytes());
        }
      }
      hash = hash_bytes(hash, &entry.secure_count_allied.to_le_bytes());
      hash = hash_bytes(hash, &entry.secure_count_opposing.to_le_bytes());
    }
    // Team membership per tracked actor commits the rosters even though the
    // map's own hash covers only locations.
    for (actor, _location) in self.map.actor_locations() {
      hash = hash_bytes(hash, &[actor.value(), u8::from(self.map.is_allied(*actor))]);
    }
    for ward in self.vision.active_wards() {
      hash = hash_bytes(hash, &ward.ward_id.to_le_bytes());
      hash = hash_bytes(
        hash,
        &[
          team_side_tag(ward.team),
          u8::try_from(ward.location.index()).unwrap_or(u8::MAX),
        ],
      );
      hash = hash_bytes(hash, &ward.remaining_turns.to_le_bytes());
    }
    // The ward-id sequence commits placement history beyond the active set.
    hash = hash_bytes(hash, &self.vision.next_ward_id().to_le_bytes());
    hash = hash_bytes(hash, &self.allied_objectives_secured.to_le_bytes());
    hash = hash_bytes(hash, &self.opposing_objectives_secured.to_le_bytes());
    StateHash::from_raw(hash)
  }
}

const fn objective_kind_tag(kind: ObjectiveKind) -> u8 {
  match kind {
    ObjectiveKind::TopRiverObjective => 1,
    ObjectiveKind::BotRiverObjective => 2,
  }
}

const fn team_side_tag(side: TeamSide) -> u8 {
  match side {
    TeamSide::Allied => 1,
    TeamSide::Opposing => 2,
  }
}

/// A scripted composed match: initial state plus an ordered action plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteMatchPlan {
  pub scenario_id: &'static str,
  pub initial: CompleteMatchState,
  pub actions: Vec<CompleteMatchAction>,
}

/// Executed outcome of one composed complete match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteMatchResult {
  pub scenario_id: &'static str,
  pub schema: &'static str,
  pub initial_hash: StateHash,
  pub final_hash: StateHash,
  pub final_turn: u32,
  pub winner: TeamSide,
  pub condition: MatchVictoryCondition,
  pub phases: Vec<MatchPhaseRecord>,
  pub total_events: usize,
  pub total_effects: usize,
  pub allied_objectives_secured: u32,
  pub opposing_objectives_secured: u32,
}

impl CompleteMatchResult {
  /// Render a structured Markdown summary of the completed match.
  ///
  /// Does not include hashes, resolved inputs, or private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# M9 Complete Match Report\n\n");
    out.push_str(&format!("- **Scenario**: `{}`\n", self.scenario_id));
    out.push_str(&format!("- **Final Turn**: {}\n", self.final_turn));
    out.push_str(&format!("- **Winner**: {:?}\n", self.winner));
    out.push_str(&format!("- **Condition**: `{}`\n", self.condition.as_str()));
    out.push_str(&format!(
      "- **Objectives Secured**: allied {}, opposing {}\n",
      self.allied_objectives_secured, self.opposing_objectives_secured
    ));
    out.push_str(&format!(
      "- **Totals**: {} events, {} effects across {} phases\n",
      self.total_events,
      self.total_effects,
      self.phases.len()
    ));
    out.push_str("\n## Phase Log\n\n");
    for (position, phase) in self.phases.iter().enumerate() {
      out.push_str(&format!(
        "{}. turn {} — `{}`: {} events, {} effects\n",
        position + 1,
        phase.turn,
        phase.kind,
        phase.events,
        phase.effects
      ));
    }
    out
  }
}

impl CompleteMatchPlan {
  /// Execute the plan deterministically from its initial state.
  ///
  /// Fail-closed: an empty plan, an action after conclusion, an unterminated
  /// plan end, or any subsystem rejection aborts the whole run.
  pub fn execute(&self) -> Result<CompleteMatchResult, CompleteMatchError> {
    if self.actions.is_empty() {
      return Err(CompleteMatchError::EmptyPlan);
    }
    let initial_hash = self.initial.combined_hash();
    let mut state = self.initial.clone();
    let mut phases = Vec::with_capacity(self.actions.len());
    let mut total_events = 0usize;
    let mut total_effects = 0usize;
    let mut conclusion: Option<(TeamSide, MatchVictoryCondition, u32)> = None;
    // Set when a subsystem already decided the match (a Nexus fell): only
    // the closing EvaluateTerminal may follow, and the recorded final turn
    // is the turn the subsystem conclusion happened, not the evaluation turn.
    let mut subsystem_conclusion_turn: Option<u32> = None;

    for action in &self.actions {
      if conclusion.is_some() {
        return Err(CompleteMatchError::MatchAlreadyConcluded);
      }
      if subsystem_conclusion_turn.is_some()
        && !matches!(action, CompleteMatchAction::EvaluateTerminal)
      {
        return Err(CompleteMatchError::MatchAlreadyConcluded);
      }
      let action_turn = state.turn;
      let (kind, events, effects) = state.apply_action(action)?;
      total_events += events;
      total_effects += effects;
      phases.push(MatchPhaseRecord {
        turn: action_turn,
        kind,
        events,
        effects,
      });
      if state.structures.check_nexus_destroyed().is_some() && subsystem_conclusion_turn.is_none() {
        subsystem_conclusion_turn = Some(action_turn);
      }
      if let CompleteMatchAction::EvaluateTerminal = action {
        let evaluation = MatchTerminalEvaluation::evaluate(
          subsystem_conclusion_turn.unwrap_or(action_turn),
          &state.structures,
          usize::try_from(state.allied_objectives_secured).expect("fits usize"),
          usize::try_from(state.opposing_objectives_secured).expect("fits usize"),
        );
        if let MatchStatus::Concluded {
          winner,
          condition,
          final_turn,
        } = evaluation.status
        {
          conclusion = Some((winner, condition, final_turn));
        }
      }
    }

    let (winner, condition, final_turn) =
      conclusion.ok_or(CompleteMatchError::MatchDidNotTerminate)?;
    let final_hash = state.combined_hash();

    Ok(CompleteMatchResult {
      scenario_id: self.scenario_id,
      schema: M9_COMPLETE_MATCH_SCHEMA_V1,
      initial_hash,
      final_hash,
      final_turn,
      winner,
      condition,
      phases,
      total_events,
      total_effects,
      allied_objectives_secured: state.allied_objectives_secured,
      opposing_objectives_secured: state.opposing_objectives_secured,
    })
  }
}

impl CompleteMatchState {
  /// Apply one action at the current turn and advance the turn.
  fn apply_action(
    &mut self,
    action: &CompleteMatchAction,
  ) -> Result<(MatchPhaseKind, usize, usize), CompleteMatchError> {
    let (kind, events, effects) = match action {
      CompleteMatchAction::Rotate { actor, destination } => {
        let current = self
          .map
          .get_actor_location(*actor)
          .cloned()
          .ok_or(CompleteMatchError::UntrackedActor)?;
        let beats = distance_in_beats(current.current_location(), *destination);
        let result = transition_travel(
          *actor,
          &current,
          TravelCommand::InitiateRotation {
            destination: *destination,
          },
          beats,
        )?;
        self.map.set_actor_location(*actor, result.next_location);
        (
          MatchPhaseKind::Rotation,
          result.events.len(),
          result.effects.len(),
        )
      }
      CompleteMatchAction::PlaceWard {
        team,
        placed_by,
        location,
        duration_turns,
      } => {
        // place_ward emits no subsystem events or effects; the phase record
        // itself is the evidence of the warding action.
        self
          .vision
          .place_ward(*team, *placed_by, *location, self.turn, *duration_turns)?;
        (MatchPhaseKind::Warding, 0, 0)
      }
      CompleteMatchAction::ContestObjectives {
        allied_intent,
        opposing_intent,
      } => {
        let result = transition_objective_contest(
          &mut self.objectives,
          &mut self.vision,
          *allied_intent,
          *opposing_intent,
          self.turn,
        );
        for event in &result.events {
          if let super::contest::ObjectiveEvent::ObjectiveSecured { secured_by, .. } = event {
            match secured_by {
              TeamSide::Allied => {
                self.allied_objectives_secured = self.allied_objectives_secured.saturating_add(1)
              }
              TeamSide::Opposing => {
                self.opposing_objectives_secured =
                  self.opposing_objectives_secured.saturating_add(1)
              }
            }
          }
        }
        (
          MatchPhaseKind::ObjectiveContest,
          result.events.len(),
          result.effects.len(),
        )
      }
      CompleteMatchAction::SiegeStructure {
        side,
        tier,
        lane,
        raw_damage,
      } => {
        let result = transition_structure_siege(
          self.turn,
          &mut self.structures,
          *side,
          SiegeIntent::AttackStructure {
            tier: *tier,
            lane: *lane,
            raw_damage: *raw_damage,
          },
          None,
        )?;
        (
          MatchPhaseKind::StructureSiege,
          result.events.len(),
          result.effects.len(),
        )
      }
      CompleteMatchAction::EvaluateTerminal => (MatchPhaseKind::TerminalEvaluation, 0, 0),
    };

    // Every action occupies one turn: advance the shared clock, the map's
    // internal turn, and structure respawn countdowns. Objective spawn and
    // ward expiry tick inside the contest transition, so the runner does not
    // tick them here.
    self.turn = self.turn.saturating_add(1);
    self.map.advance_turn();
    let respawn_events = self.structures.tick_turn();

    Ok((kind, events + respawn_events.len(), effects))
  }
}
