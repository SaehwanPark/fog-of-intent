//! Interactive multi-lane tactical match host.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This host manages interactive multi-lane match execution, supporting
//! tactical intent planning (`rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`),
//! commitment, step-by-step turn advancement, event/effect tracking, and match debriefs.

use crate::kernel::{ActorId, StateHash};
use crate::map::complete_match::{
  CompleteMatchAction, CompleteMatchError, CompleteMatchPlan, CompleteMatchResult,
  CompleteMatchState, M9_COMPLETE_MATCH_SCHEMA_V1, MatchPhaseKind, MatchPhaseRecord,
};
use crate::map::complete_match_catalog::CompleteMatchCatalog;
use crate::map::contest::ObjectiveIntent;
use crate::map::objective::ObjectiveKind;
use crate::map::state::OpponentSighting;
use crate::map::structures::StructureTier;
use crate::map::topology::{LaneId, MapLocation, TeamSide};
use crate::map::victory::{MatchStatus, MatchTerminalEvaluation, MatchVictoryCondition};
use crate::map::vision::DEFAULT_WARD_DURATION_TURNS;

/// Schema identifier for the interactive match host.
pub const CLI_MATCH_HOST_SCHEMA: &str = "m9-interactive-match-host-v1";

/// Default interactive match scenario ID.
pub const CLI_INTERACTIVE_MATCH_SCENARIO_ID: &str = "m9-interactive-match-v1";

/// Actor-visible location certainty in a match observation.
///
/// Allied actors are always reported as currently observed. Opponents use the
/// map's fog-of-war projection, so an unseen opponent is represented without a
/// location rather than exposing authoritative state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchActorLocation {
  Observed(MapLocation),
  LastKnown(MapLocation),
  Unknown,
}

/// Actor-visible observation report for the multi-lane match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchObservationReport {
  pub turn: u32,
  pub concluded: bool,
  pub winner: Option<TeamSide>,
  pub condition: Option<MatchVictoryCondition>,
  pub allied_objectives_secured: u32,
  pub opposing_objectives_secured: u32,
  pub actor_locations: Vec<(ActorId, bool, MatchActorLocation)>,
  pub active_ward_count: usize,
  pub structures_summary: Vec<MatchStructureSummary>,
  pub top_objective_status: &'static str,
  pub bot_objective_status: &'static str,
}

/// Bounded summary of a structure status for display.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchStructureSummary {
  pub side: TeamSide,
  pub tier: StructureTier,
  pub lane: Option<LaneId>,
  pub current_health: u32,
  pub max_health: u32,
  pub standing: bool,
}

/// Actor-valid results returned by [`CliMatchHost::apply_line`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliMatchOutput {
  Help {
    topic: Option<&'static str>,
  },
  Observation(MatchObservationReport),
  DraftStaged {
    description: String,
  },
  Committed {
    description: String,
  },
  Advanced {
    turn: u32,
    kind: MatchPhaseKind,
    events: usize,
    effects: usize,
    concluded: bool,
  },
  Debrief(CompleteMatchResult),
  Undone,
  Quit,
}

/// Errors raised before or while applying a command in the interactive match host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliMatchError {
  Closed,
  EmptyInput,
  UnknownCommand { verb: String },
  InvalidSyntax { message: String },
  MissingAction,
  MissingCommittedAction,
  NothingToUndo,
  MatchAlreadyConcluded,
  MatchDidNotTerminate,
  ExecutionFailed(CompleteMatchError),
  DebriefUnavailable,
  UnknownHelpTopic { topic: String },
}

impl From<CompleteMatchError> for CliMatchError {
  fn from(err: CompleteMatchError) -> Self {
    Self::ExecutionFailed(err)
  }
}

/// Synchronous host managing an interactive multi-lane tactical match.
pub struct CliMatchHost {
  pub(crate) scenario_id: &'static str,
  pub(crate) initial_hash: StateHash,
  pub(crate) state: CompleteMatchState,
  pub(crate) staged_action: Option<(CompleteMatchAction, String)>,
  pub(crate) committed_action: Option<(CompleteMatchAction, String)>,
  pub(crate) phases: Vec<MatchPhaseRecord>,
  pub(crate) total_events: usize,
  pub(crate) total_effects: usize,
  pub(crate) conclusion: Option<(TeamSide, MatchVictoryCondition, u32)>,
  pub(crate) subsystem_conclusion_turn: Option<u32>,
  pub(crate) closed: bool,
}

impl CliMatchHost {
  /// Create a new interactive match host from a complete match plan.
  pub fn new(plan: CompleteMatchPlan) -> Self {
    let initial_hash = plan.initial.combined_hash();
    Self {
      scenario_id: plan.scenario_id,
      initial_hash,
      state: plan.initial,
      staged_action: None,
      committed_action: None,
      phases: Vec::new(),
      total_events: 0,
      total_effects: 0,
      conclusion: None,
      subsystem_conclusion_turn: None,
      closed: false,
    }
  }

  /// Create the canonical default interactive match session (Allied Snowball scenario).
  pub fn default_session() -> Self {
    Self::new(CompleteMatchCatalog::allied_snowball_victory())
  }

  /// Create an interactive match session from scenario ID.
  pub fn from_scenario_id(id: &str) -> Option<Self> {
    CompleteMatchCatalog::find(id).map(Self::new)
  }

  /// Check if the match has concluded.
  pub fn is_concluded(&self) -> bool {
    self.conclusion.is_some()
  }

  /// Current turn of the match.
  pub fn turn(&self) -> u32 {
    self.state.turn()
  }

  /// Build an actor-visible observation report of the current state.
  pub fn observation_report(&self) -> MatchObservationReport {
    let mut actor_locs = Vec::new();
    let observer = self
      .state
      .map()
      .actor_locations()
      .iter()
      .find_map(|(actor, _)| self.state.map().is_allied(*actor).then_some(*actor));
    let map_observation = observer.and_then(|actor| self.state.map().observe(actor));
    for (actor, loc) in self.state.map().actor_locations() {
      let is_allied = self.state.map().is_allied(*actor);
      let location = if is_allied {
        MatchActorLocation::Observed(loc.current_location())
      } else {
        map_observation
          .as_ref()
          .and_then(|observation| {
            observation
              .opposing_sightings
              .iter()
              .find(|(id, _)| *id == *actor)
          })
          .map_or(
            MatchActorLocation::Unknown,
            |(_, sighting)| match sighting {
              OpponentSighting::Observed { location, .. } => {
                MatchActorLocation::Observed(*location)
              }
              OpponentSighting::LastKnown { location, .. } => {
                MatchActorLocation::LastKnown(*location)
              }
              OpponentSighting::Unknown => MatchActorLocation::Unknown,
            },
          )
      };
      actor_locs.push((*actor, is_allied, location));
    }
    // Sort actor locations by actor ID for deterministic display
    actor_locs.sort_by_key(|(a, _, _)| a.value());

    let mut structures = Vec::new();
    for entry in self.state.structures().structures() {
      structures.push(MatchStructureSummary {
        side: entry.team,
        tier: entry.tier,
        lane: entry.lane,
        current_health: entry.status.current_hp(),
        max_health: entry.tier.default_max_hp(),
        standing: entry.status.is_standing(),
      });
    }

    let top_entry = self
      .state
      .objectives()
      .get(ObjectiveKind::TopRiverObjective);
    let top_status = match top_entry.status {
      crate::map::objective::ObjectiveStatus::Unspawned { turns_until_spawn } => {
        if turns_until_spawn == 0 {
          "spawning"
        } else {
          "unspawned"
        }
      }
      crate::map::objective::ObjectiveStatus::Active { current_health, .. } => {
        if current_health > 0 {
          "active"
        } else {
          "vulnerable"
        }
      }
      crate::map::objective::ObjectiveStatus::Secured { .. } => "secured",
    };

    let bot_entry = self
      .state
      .objectives()
      .get(ObjectiveKind::BotRiverObjective);
    let bot_status = match bot_entry.status {
      crate::map::objective::ObjectiveStatus::Unspawned { turns_until_spawn } => {
        if turns_until_spawn == 0 {
          "spawning"
        } else {
          "unspawned"
        }
      }
      crate::map::objective::ObjectiveStatus::Active { current_health, .. } => {
        if current_health > 0 {
          "active"
        } else {
          "vulnerable"
        }
      }
      crate::map::objective::ObjectiveStatus::Secured { .. } => "secured",
    };

    MatchObservationReport {
      turn: self.state.turn(),
      concluded: self.is_concluded(),
      winner: self.conclusion.map(|(w, _, _)| w),
      condition: self.conclusion.map(|(_, c, _)| c),
      allied_objectives_secured: self.state.allied_objectives_secured(),
      opposing_objectives_secured: self.state.opposing_objectives_secured(),
      actor_locations: actor_locs,
      active_ward_count: self.state.vision().active_wards().len(),
      structures_summary: structures,
      top_objective_status: top_status,
      bot_objective_status: bot_status,
    }
  }

  /// Apply a line-oriented command to the match host.
  pub fn apply_line(&mut self, line: &str) -> Result<CliMatchOutput, CliMatchError> {
    if self.closed {
      return Err(CliMatchError::Closed);
    }
    let trimmed = line.trim();
    if trimmed.is_empty() {
      return Err(CliMatchError::EmptyInput);
    }
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    let verb = tokens[0].to_ascii_lowercase();

    match verb.as_str() {
      "help" | "?" => self.apply_help(tokens.get(1).copied()),
      "observe" | "status" | "map" => Ok(CliMatchOutput::Observation(self.observation_report())),
      "plan" => {
        if tokens.len() < 2 {
          return Err(CliMatchError::InvalidSyntax {
            message: "usage: plan <rotate|ward|contest|siege|evaluate|idle> [...]".into(),
          });
        }
        self.parse_and_stage_action(&tokens[1..])
      }
      // Direct action planning shortcuts without "plan" prefix
      "rotate" | "ward" | "contest" | "siege" | "evaluate" | "idle" => {
        self.parse_and_stage_action(&tokens)
      }
      "commit" => self.apply_commit(),
      "advance" => self.apply_advance(),
      "debrief" | "review" => self.apply_debrief(),
      "undo" => self.apply_undo(),
      "quit" | "exit" => {
        self.closed = true;
        Ok(CliMatchOutput::Quit)
      }
      other => Err(CliMatchError::UnknownCommand {
        verb: other.to_string(),
      }),
    }
  }

  fn apply_help(&self, topic: Option<&str>) -> Result<CliMatchOutput, CliMatchError> {
    match topic {
      None => Ok(CliMatchOutput::Help { topic: None }),
      Some(t) => {
        let topic_lower = t.to_ascii_lowercase();
        match topic_lower.as_str() {
          "rotate" | "ward" | "contest" | "siege" | "evaluate" | "idle" | "commit" | "advance"
          | "observe" | "debrief" | "undo" | "quit" => Ok(CliMatchOutput::Help {
            topic: match topic_lower.as_str() {
              "rotate" => Some("rotate"),
              "ward" => Some("ward"),
              "contest" => Some("contest"),
              "siege" => Some("siege"),
              "evaluate" => Some("evaluate"),
              "idle" => Some("idle"),
              "commit" => Some("commit"),
              "advance" => Some("advance"),
              "observe" => Some("observe"),
              "debrief" => Some("debrief"),
              "undo" => Some("undo"),
              "quit" => Some("quit"),
              _ => None,
            },
          }),
          other => Err(CliMatchError::UnknownHelpTopic {
            topic: other.to_string(),
          }),
        }
      }
    }
  }

  fn parse_and_stage_action(&mut self, tokens: &[&str]) -> Result<CliMatchOutput, CliMatchError> {
    if self.conclusion.is_some() {
      return Err(CliMatchError::MatchAlreadyConcluded);
    }
    if self.committed_action.is_some() {
      return Err(CliMatchError::InvalidSyntax {
        message: "an action is already committed; advance or undo before staging another plan"
          .into(),
      });
    }
    if self.staged_action.is_some() {
      return Err(CliMatchError::InvalidSyntax {
        message: "an action is already staged; commit or undo before staging another plan".into(),
      });
    }
    let action_verb = tokens[0].to_ascii_lowercase();
    match action_verb.as_str() {
      "rotate" => {
        // syntax: rotate <actor_id> <destination>
        if tokens.len() < 3 {
          return Err(CliMatchError::InvalidSyntax {
            message: "usage: plan rotate <actor_id> <destination> (e.g. rotate 1 bot_river)".into(),
          });
        }
        let actor = parse_actor_id(tokens[1])?;
        let destination = parse_map_location(tokens[2])?;
        let desc = format!("rotate actor {} to {}", actor.value(), destination.as_str());
        let action = CompleteMatchAction::Rotate { actor, destination };
        self.staged_action = Some((action, desc.clone()));
        Ok(CliMatchOutput::DraftStaged { description: desc })
      }
      "ward" => {
        // syntax: ward [team] <actor_id> <location> [duration]
        // or: ward <location> (defaults to Allied, actor 1, 3 turns)
        let (team, placed_by, location, duration_turns) = if tokens.len() == 2 {
          (
            TeamSide::Allied,
            ActorId::new(1),
            parse_map_location(tokens[1])?,
            DEFAULT_WARD_DURATION_TURNS,
          )
        } else if tokens.len() >= 4 {
          let team = parse_team_side(tokens[1])?;
          let actor = parse_actor_id(tokens[2])?;
          let location = parse_map_location(tokens[3])?;
          let duration = if tokens.len() >= 5 {
            tokens[4]
              .parse::<u32>()
              .map_err(|_| CliMatchError::InvalidSyntax {
                message: "invalid ward duration; expected integer turns".into(),
              })?
          } else {
            DEFAULT_WARD_DURATION_TURNS
          };
          (team, actor, location, duration)
        } else if tokens.len() == 3 {
          let actor = parse_actor_id(tokens[1])?;
          let location = parse_map_location(tokens[2])?;
          (
            TeamSide::Allied,
            actor,
            location,
            DEFAULT_WARD_DURATION_TURNS,
          )
        } else {
          return Err(CliMatchError::InvalidSyntax {
            message:
              "usage: plan ward <location> or plan ward <team> <actor_id> <location> [duration]"
                .into(),
          });
        };
        let desc = format!(
          "place ward at {} by actor {} ({:?}, {} turns)",
          location.as_str(),
          placed_by.value(),
          team,
          duration_turns
        );
        let action = CompleteMatchAction::PlaceWard {
          team,
          placed_by,
          location,
          duration_turns,
        };
        self.staged_action = Some((action, desc.clone()));
        Ok(CliMatchOutput::DraftStaged { description: desc })
      }
      "contest" => {
        // syntax: contest <top|bot> [damage] [burst]
        if tokens.len() < 2 {
          return Err(CliMatchError::InvalidSyntax {
            message: "usage: plan contest <top|bot> [damage] [burst]".into(),
          });
        }
        let objective = parse_objective_kind(tokens[1])?;
        let damage = if tokens.len() >= 3 {
          tokens[2]
            .parse::<u32>()
            .map_err(|_| CliMatchError::InvalidSyntax {
              message: "invalid damage amount; expected integer".into(),
            })?
        } else {
          4_000
        };
        let is_burst = tokens.len() >= 4 && tokens[3].eq_ignore_ascii_case("burst");
        let intent = if is_burst {
          ObjectiveIntent::SecureBurst {
            objective,
            burst_damage: damage,
          }
        } else {
          ObjectiveIntent::Engage { objective, damage }
        };
        let desc = format!(
          "contest {} (damage={}, burst={})",
          match objective {
            ObjectiveKind::TopRiverObjective => "top_river_objective",
            ObjectiveKind::BotRiverObjective => "bot_river_objective",
          },
          damage,
          is_burst
        );
        let action = CompleteMatchAction::ContestObjectives {
          allied_intent: Some(intent),
          opposing_intent: None,
        };
        self.staged_action = Some((action, desc.clone()));
        Ok(CliMatchOutput::DraftStaged { description: desc })
      }
      "siege" => {
        // syntax: siege [side] <tier> [lane] <damage>
        // e.g. siege outer mid 4000  OR  siege allied outer mid 4000  OR  siege nexus 6500
        if tokens.len() < 3 {
          return Err(CliMatchError::InvalidSyntax {
            message:
              "usage: plan siege <outer|inner|inhibitor_turret|inhibitor|nexus> [lane] <damage>"
                .into(),
          });
        }
        let mut idx = 1;
        let side = if tokens[idx].eq_ignore_ascii_case("allied")
          || tokens[idx].eq_ignore_ascii_case("opposing")
        {
          let s = parse_team_side(tokens[idx])?;
          idx += 1;
          s
        } else {
          TeamSide::Allied
        };

        let tier = parse_structure_tier(tokens[idx])?;
        idx += 1;

        let lane = if tier == StructureTier::Nexus {
          None
        } else if idx < tokens.len() && is_lane_name(tokens[idx]) {
          let l = parse_lane_id(tokens[idx])?;
          idx += 1;
          Some(l)
        } else {
          Some(LaneId::Mid)
        };

        let raw_damage = if idx < tokens.len() {
          tokens[idx]
            .parse::<u32>()
            .map_err(|_| CliMatchError::InvalidSyntax {
              message: "invalid siege damage; expected integer".into(),
            })?
        } else {
          4_000
        };

        let target_side = match side {
          TeamSide::Allied => TeamSide::Opposing,
          TeamSide::Opposing => TeamSide::Allied,
        };
        let desc = format!(
          "siege {:?} {:?}{} for {} damage (attacker={:?})",
          target_side,
          tier,
          lane.map_or("".into(), |l| format!(" on {:?}", l)),
          raw_damage,
          side
        );
        let action = CompleteMatchAction::SiegeStructure {
          side,
          tier,
          lane,
          raw_damage,
        };
        self.staged_action = Some((action, desc.clone()));
        Ok(CliMatchOutput::DraftStaged { description: desc })
      }
      "evaluate" => {
        let desc = "evaluate terminal victory conditions".to_string();
        let action = CompleteMatchAction::EvaluateTerminal;
        self.staged_action = Some((action, desc.clone()));
        Ok(CliMatchOutput::DraftStaged { description: desc })
      }
      "idle" | "hold" | "pass" => {
        let desc = "idle (no tactical contest action)".to_string();
        let action = CompleteMatchAction::ContestObjectives {
          allied_intent: None,
          opposing_intent: None,
        };
        self.staged_action = Some((action, desc.clone()));
        Ok(CliMatchOutput::DraftStaged { description: desc })
      }
      other => Err(CliMatchError::UnknownCommand {
        verb: other.to_string(),
      }),
    }
  }

  fn apply_commit(&mut self) -> Result<CliMatchOutput, CliMatchError> {
    if self.conclusion.is_some() {
      return Err(CliMatchError::MatchAlreadyConcluded);
    }
    let (action, desc) = self
      .staged_action
      .take()
      .ok_or(CliMatchError::MissingAction)?;
    self.committed_action = Some((action, desc.clone()));
    Ok(CliMatchOutput::Committed { description: desc })
  }

  fn apply_advance(&mut self) -> Result<CliMatchOutput, CliMatchError> {
    if self.conclusion.is_some() {
      return Err(CliMatchError::MatchAlreadyConcluded);
    }
    // If an action was staged but not explicitly committed, commit it now
    if self.committed_action.is_none() && self.staged_action.is_some() {
      self.committed_action = self.staged_action.take();
    }
    let (action, _desc) = self
      .committed_action
      .take()
      .ok_or(CliMatchError::MissingCommittedAction)?;

    if self.subsystem_conclusion_turn.is_some()
      && !matches!(action, CompleteMatchAction::EvaluateTerminal)
    {
      return Err(CliMatchError::MatchAlreadyConcluded);
    }

    let action_turn = self.state.turn();
    let (kind, events, effects) = self.state.apply_action(&action)?;
    self.total_events += events;
    self.total_effects += effects;
    self.phases.push(MatchPhaseRecord {
      turn: action_turn,
      kind,
      events,
      effects,
    });

    if self.state.structures().check_nexus_destroyed().is_some()
      && self.subsystem_conclusion_turn.is_none()
    {
      self.subsystem_conclusion_turn = Some(action_turn);
    }

    if let CompleteMatchAction::EvaluateTerminal = action {
      let evaluation = MatchTerminalEvaluation::evaluate(
        self.subsystem_conclusion_turn.unwrap_or(action_turn),
        self.state.structures(),
        usize::try_from(self.state.allied_objectives_secured()).expect("fits usize"),
        usize::try_from(self.state.opposing_objectives_secured()).expect("fits usize"),
      );
      if let MatchStatus::Concluded {
        winner,
        condition,
        final_turn,
      } = evaluation.status
      {
        self.conclusion = Some((winner, condition, final_turn));
      }
    }

    Ok(CliMatchOutput::Advanced {
      turn: action_turn,
      kind,
      events,
      effects,
      concluded: self.is_concluded(),
    })
  }

  fn apply_debrief(&self) -> Result<CliMatchOutput, CliMatchError> {
    if let Some((winner, condition, final_turn)) = self.conclusion {
      let final_hash = self.state.combined_hash();
      Ok(CliMatchOutput::Debrief(CompleteMatchResult {
        scenario_id: self.scenario_id,
        schema: M9_COMPLETE_MATCH_SCHEMA_V1,
        initial_hash: self.initial_hash,
        final_hash,
        final_turn,
        winner,
        condition,
        phases: self.phases.clone(),
        total_events: self.total_events,
        total_effects: self.total_effects,
        allied_objectives_secured: self.state.allied_objectives_secured(),
        opposing_objectives_secured: self.state.opposing_objectives_secured(),
      }))
    } else {
      Err(CliMatchError::DebriefUnavailable)
    }
  }

  fn apply_undo(&mut self) -> Result<CliMatchOutput, CliMatchError> {
    if self.staged_action.is_some() || self.committed_action.is_some() {
      self.staged_action = None;
      self.committed_action = None;
      Ok(CliMatchOutput::Undone)
    } else {
      Err(CliMatchError::NothingToUndo)
    }
  }
}

fn parse_actor_id(token: &str) -> Result<ActorId, CliMatchError> {
  match token.to_ascii_lowercase().as_str() {
    "jungler" | "jungle" | "jg" | "1" => Ok(ActorId::new(1)),
    "mid" | "midlaner" | "2" => Ok(ActorId::new(2)),
    "support" | "sup" | "supp" | "3" => Ok(ActorId::new(3)),
    "top" | "toplaner" | "4" => Ok(ActorId::new(4)),
    "bot" | "botcarry" | "adc" | "5" => Ok(ActorId::new(5)),
    other => other
      .parse::<u8>()
      .map(ActorId::new)
      .map_err(|_| CliMatchError::InvalidSyntax {
        message: format!("unknown actor id '{token}'; expected 1..=10 or role name"),
      }),
  }
}

fn parse_team_side(token: &str) -> Result<TeamSide, CliMatchError> {
  match token.to_ascii_lowercase().as_str() {
    "allied" | "blue" | "team_a" => Ok(TeamSide::Allied),
    "opposing" | "red" | "team_b" => Ok(TeamSide::Opposing),
    _ => Err(CliMatchError::InvalidSyntax {
      message: format!("unknown team side '{token}'; expected 'allied' or 'opposing'"),
    }),
  }
}

fn parse_map_location(token: &str) -> Result<MapLocation, CliMatchError> {
  match token.to_ascii_lowercase().as_str() {
    "allied_base" | "base_allied" | "base" => Ok(MapLocation::ALLIED_BASE),
    "top_near_tower" => Ok(MapLocation::TOP_NEAR_TOWER),
    "top_center" | "top" => Ok(MapLocation::TOP_CENTER),
    "top_far_side" => Ok(MapLocation::TOP_FAR_SIDE),
    "top_river" => Ok(MapLocation::TOP_RIVER),
    "mid_near_tower" => Ok(MapLocation::MID_NEAR_TOWER),
    "mid_center" | "mid" => Ok(MapLocation::MID_CENTER),
    "mid_far_side" => Ok(MapLocation::MID_FAR_SIDE),
    "bot_near_tower" => Ok(MapLocation::BOT_NEAR_TOWER),
    "bot_center" | "bot" => Ok(MapLocation::BOT_CENTER),
    "bot_far_side" => Ok(MapLocation::BOT_FAR_SIDE),
    "bot_river" => Ok(MapLocation::BOT_RIVER),
    "allied_top_jungle" | "opposing_top_jungle" | "top_jungle" => Ok(MapLocation::TOP_JUNGLE),
    "allied_bot_jungle" | "opposing_bot_jungle" | "bot_jungle" => Ok(MapLocation::BOT_JUNGLE),
    "opposing_base" | "enemy_base" => Ok(MapLocation::OPPOSING_BASE),
    _ => Err(CliMatchError::InvalidSyntax {
      message: format!("unknown map location '{token}'"),
    }),
  }
}

fn parse_objective_kind(token: &str) -> Result<ObjectiveKind, CliMatchError> {
  match token.to_ascii_lowercase().as_str() {
    "top" | "top_river" | "herald" | "baron" | "top_objective" => {
      Ok(ObjectiveKind::TopRiverObjective)
    }
    "bot" | "bot_river" | "dragon" | "drake" | "bot_objective" => {
      Ok(ObjectiveKind::BotRiverObjective)
    }
    _ => Err(CliMatchError::InvalidSyntax {
      message: format!("unknown objective '{token}'; expected 'top' (Baron) or 'bot' (Dragon)"),
    }),
  }
}

fn parse_structure_tier(token: &str) -> Result<StructureTier, CliMatchError> {
  match token.to_ascii_lowercase().as_str() {
    "outer" | "outer_turret" | "t1" => Ok(StructureTier::OuterTurret),
    "inner" | "inner_turret" | "t2" => Ok(StructureTier::InnerTurret),
    "inhibitor_turret" | "inhib_turret" | "t3" => Ok(StructureTier::InhibitorTurret),
    "inhibitor" | "inhib" => Ok(StructureTier::Inhibitor),
    "nexus" | "core" => Ok(StructureTier::Nexus),
    _ => Err(CliMatchError::InvalidSyntax {
      message: format!(
        "unknown structure tier '{token}'; expected outer, inner, inhibitor_turret, inhibitor, or nexus"
      ),
    }),
  }
}

fn is_lane_name(token: &str) -> bool {
  matches!(
    token.to_ascii_lowercase().as_str(),
    "top" | "mid" | "bot" | "middle" | "bottom"
  )
}

fn parse_lane_id(token: &str) -> Result<LaneId, CliMatchError> {
  match token.to_ascii_lowercase().as_str() {
    "top" => Ok(LaneId::Top),
    "mid" | "middle" => Ok(LaneId::Mid),
    "bot" | "bottom" => Ok(LaneId::Bot),
    _ => Err(CliMatchError::InvalidSyntax {
      message: format!("unknown lane '{token}'; expected top, mid, or bot"),
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn match_host_initializes_and_observes() {
    let mut host = CliMatchHost::default_session();
    assert_eq!(host.turn(), 1);
    assert!(!host.is_concluded());

    let output = host.apply_line("observe").expect("observe should succeed");
    let CliMatchOutput::Observation(obs) = output else {
      panic!("expected observation output");
    };
    assert_eq!(obs.turn, 1);
    assert!(!obs.concluded);
    assert_eq!(obs.allied_objectives_secured, 0);
  }

  #[test]
  fn match_observation_redacts_unseen_opponents() {
    let host = CliMatchHost::default_session();
    let report = host.observation_report();

    assert_eq!(
      report
        .actor_locations
        .iter()
        .find(|(actor, _, _)| actor.value() == 4)
        .map(|(_, _, location)| *location),
      Some(MatchActorLocation::Unknown)
    );
  }

  #[test]
  fn match_host_plans_commits_and_advances_rotation() {
    let mut host = CliMatchHost::default_session();

    let plan_res = host
      .apply_line("plan rotate jungler bot_river")
      .expect("plan rotate should succeed");
    assert!(matches!(plan_res, CliMatchOutput::DraftStaged { .. }));

    let commit_res = host.apply_line("commit").expect("commit should succeed");
    assert!(matches!(commit_res, CliMatchOutput::Committed { .. }));

    let advance_res = host.apply_line("advance").expect("advance should succeed");
    let CliMatchOutput::Advanced {
      turn,
      kind,
      concluded,
      ..
    } = advance_res
    else {
      panic!("expected advanced output");
    };
    assert_eq!(turn, 1);
    assert_eq!(kind, MatchPhaseKind::Rotation);
    assert!(!concluded);
    assert_eq!(host.turn(), 2);
  }

  #[test]
  fn match_host_plans_ward_contest_and_siege() {
    let mut host = CliMatchHost::default_session();

    // Rotate jungler
    host.apply_line("rotate 1 bot_river").unwrap();
    host.apply_line("advance").unwrap();

    // Place ward
    host.apply_line("ward allied 3 bot_river 3").unwrap();
    host.apply_line("advance").unwrap();

    // Idle
    host.apply_line("idle").unwrap();
    host.apply_line("advance").unwrap();
    host.apply_line("idle").unwrap();
    host.apply_line("advance").unwrap();
    host.apply_line("idle").unwrap();
    host.apply_line("advance").unwrap();

    // Contest objective
    host.apply_line("contest bot 4000").unwrap();
    host.apply_line("advance").unwrap();
    assert_eq!(host.state.allied_objectives_secured(), 1);

    // Siege outer turret
    host.apply_line("siege outer mid 4000").unwrap();
    host.apply_line("advance").unwrap();

    let obs = host.observation_report();
    assert_eq!(obs.allied_objectives_secured, 1);
  }

  #[test]
  fn match_host_describes_siege_target_and_rejects_staged_replacement() {
    let mut host = CliMatchHost::default_session();

    let staged = host.apply_line("siege outer mid 4000").unwrap();
    let CliMatchOutput::DraftStaged { description } = staged else {
      panic!("expected staged siege output");
    };
    assert!(description.contains("Opposing OuterTurret"));
    assert!(description.contains("attacker=Allied"));

    let replacement = host.apply_line("idle").unwrap_err();
    assert_eq!(
      replacement,
      CliMatchError::InvalidSyntax {
        message: "an action is already staged; commit or undo before staging another plan".into(),
      }
    );
  }

  #[test]
  fn match_host_requires_terminal_state_before_debrief() {
    let mut host = CliMatchHost::default_session();
    assert_eq!(
      host.apply_line("debrief").unwrap_err(),
      CliMatchError::DebriefUnavailable
    );
  }

  #[test]
  fn match_host_plays_full_snowball_match_to_victory() {
    let mut host = CliMatchHost::default_session();
    let commands = [
      "rotate 1 bot_river",
      "ward allied 3 bot_river 3",
      "idle",
      "idle",
      "idle",
      "contest bot 4000",
      "siege outer mid 4000",
      "idle",
      "siege inner mid 4500",
      "idle",
      "siege inhibitor_turret mid 5000",
      "siege inhibitor mid 3500",
      "rotate 2 opposing_base",
      "siege nexus 6500",
      "evaluate",
    ];

    for cmd in commands {
      host.apply_line(cmd).expect("command should succeed");
      let adv = host.apply_line("advance").expect("advance should succeed");
      if cmd == "evaluate" {
        let CliMatchOutput::Advanced { concluded, .. } = adv else {
          panic!("expected advanced output");
        };
        assert!(concluded);
      }
    }

    assert!(host.is_concluded());
    let debrief = host.apply_line("debrief").expect("debrief should succeed");
    let CliMatchOutput::Debrief(res) = debrief else {
      panic!("expected debrief output");
    };
    assert_eq!(res.winner, TeamSide::Allied);
    assert_eq!(res.condition, MatchVictoryCondition::NexusDemolished);
    assert_eq!(res.allied_objectives_secured, 1);
  }

  #[test]
  fn match_host_handles_undo_and_syntax_errors() {
    let mut host = CliMatchHost::default_session();

    let undo_err = host.apply_line("undo").unwrap_err();
    assert_eq!(undo_err, CliMatchError::NothingToUndo);

    host.apply_line("rotate 1 bot_river").unwrap();
    let undo_ok = host.apply_line("undo").unwrap();
    assert_eq!(undo_ok, CliMatchOutput::Undone);

    let unknown_err = host.apply_line("invalid_verb").unwrap_err();
    assert!(matches!(unknown_err, CliMatchError::UnknownCommand { .. }));
  }
}
