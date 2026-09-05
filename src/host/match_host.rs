//! Interactive multi-lane tactical match host.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This host manages interactive multi-lane match execution, supporting
//! tactical intent planning (`rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`),
//! commitment, step-by-step turn advancement, event/effect tracking, and match debriefs.

use crate::kernel::{ActorId, StateHash};
use crate::map::complete_match::{
  CommitStrength, CompleteMatchAction, CompleteMatchError, CompleteMatchPlan, CompleteMatchResult,
  CompleteMatchState, M9_COMPLETE_MATCH_SCHEMA_V2, MatchPhaseKind, MatchPhaseRecord,
  PRESENCE_REACH_BEATS, deliverable_force,
};
use crate::map::complete_match_catalog::CompleteMatchCatalog;
use crate::map::contest::ObjectiveIntent;
use crate::map::objective::{ObjectiveKind, ObjectiveStatus};
use crate::map::state::{MatchMapState, OpponentSighting};
use crate::map::structures::{ObservedStructure, StructureTier};
use crate::map::topology::{LaneId, MapLocation, TeamSide};
use crate::map::victory::{MatchStatus, MatchTerminalEvaluation, MatchVictoryCondition};
use crate::map::vision::DEFAULT_WARD_DURATION_TURNS;

/// Schema identifier for the interactive match host.
/// Identity of the interactive match host contract, including its observation
/// projection. `v2` reports defensive structures through the team's sight as coarse
/// health bands instead of exact global health (`docs/decision_brief_20260830.md`
/// decision D3). `v3` resolves declared force against the actors standing in the target
/// sector (decision D2): an over-declared siege or contest delivers less than it names,
/// an unbacked declaration is refused at staging, and the turn note reports the cap. `v4`
/// adds the commit-strength token vocabulary to the force slot of `contest` and `siege`
/// (decision D5); raw integers keep working unchanged, so a `v3` script is still a `v4`
/// script. `v5` narrows the grammar to the one commander this session has always had:
/// a staged line may only order an allied actor that exists in the scenario roster, so
/// `rotate` of an opposing or unknown actor, a ward placed by either or for the opposing
/// team, and a siege attacked by `opposing` are now refused before staging instead of
/// being accepted. Removing accepted input is breaking, so the identity moves; the
/// scripted benchmark plans use structured actions, never this grammar, so no recorded
/// plan is affected and the map ruleset and both `-v2` scenario ids are unchanged.
pub const CLI_MATCH_HOST_SCHEMA: &str = "m9-interactive-match-host-v7";

/// Force a `contest` or `siege` declares when the player names no amount.
///
/// This is the integer the interactive host shipped with before commit-strength tokens
/// existed, kept so an existing script or MCP caller asking for the default still means
/// exactly what it meant then. It is deliberately not a token value: no token spells 4 000,
/// and quietly re-pricing a bare command would be a balance change wearing a vocabulary
/// costume.
pub const LEGACY_DEFAULT_FORCE: u32 = 4_000;

/// Default interactive match scenario ID.
pub const CLI_INTERACTIVE_MATCH_SCENARIO_ID: &str = "m9-interactive-match-v1";

/// Interactive onboarding session ID a player types. It resolves the teaching plan
/// `scenario-complete-onboarding-v1`; the debrief prints the plan identity rather than
/// this session ID, exactly as the benchmark session prints its plan identity.
pub const CLI_INTERACTIVE_MATCH_ONBOARDING_SCENARIO_ID: &str = "m9-match-onboarding-v1";

/// Actor-visible location certainty in a match observation.
///
/// Allied actors are always reported as currently observed. Opponents use the
/// map's fog-of-war projection, so an unseen opponent is represented without a
/// location rather than exposing authoritative state. There is deliberately no
/// stale-position case: the projection reports a current sighting or nothing,
/// and a player never has to work out how old a location is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchActorLocation {
  Observed(MapLocation),
  Unknown,
}

/// Host-side explanation printed alongside an advance that recorded nothing.
///
/// The authoritative transitions stay silent about turns that change nothing
/// (`docs/decision_brief_20260830.md` decision D4): the reason is a property of
/// the composed session, so the host derives it after the turn from facts the
/// observer can already read back through `observe` (objective status, ward
/// state, the committed intent). A note therefore adds no hidden information
/// and never claims authority over what happened.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchTurnNote {
  /// The turn committed an explicit idle intent.
  IdleWithoutAction,
  /// A ward was placed; warding is recorded as a phase, not as an event.
  WardPlacement,
  /// Termination was evaluated without any other change this turn.
  TerminalEvaluation,
  /// The declared force targeted an objective that is not on the map yet.
  ObjectiveUnspawned {
    objective: ObjectiveKind,
    turns_until_spawn: u32,
  },
  /// The declared force targeted an objective that is already secured.
  ObjectiveSecured {
    objective: ObjectiveKind,
    secured_by: TeamSide,
    secured_turn: u32,
    turns_until_respawn: u32,
  },
  /// The target objective was active, but the declared force resolved to zero.
  ZeroDeclaredForce { objective: ObjectiveKind },
  /// Declared force was more than the actors standing in reach could deliver.
  ForceCapped {
    sector: &'static str,
    declared: u32,
    present: usize,
    delivered: u32,
  },
  /// Nothing else explained the empty turn; the counters are still accurate.
  Unattributed,
}

impl MatchTurnNote {
  /// Stable machine-readable label for scripts, MCP clients, and tests.
  pub const fn code(self) -> &'static str {
    match self {
      Self::IdleWithoutAction => "idle-without-action",
      Self::WardPlacement => "ward-placement-recorded-as-phase",
      Self::TerminalEvaluation => "terminal-evaluation-only",
      Self::ObjectiveUnspawned { .. } => "objective-unspawned",
      Self::ObjectiveSecured { .. } => "objective-secured",
      Self::ZeroDeclaredForce { .. } => "zero-declared-force",
      Self::ForceCapped { .. } => "force-capped",
      Self::Unattributed => "unattributed",
    }
  }

  /// Human-readable explanation in the observer's own vocabulary.
  pub fn detail(&self) -> String {
    match self {
      Self::IdleWithoutAction => "no action was committed this turn, so nothing changed".to_owned(),
      Self::WardPlacement => {
        "the ward is placed and counting down; warding is recorded as a phase, not an event"
          .to_owned()
      }
      Self::TerminalEvaluation => {
        "termination was evaluated; the match is still in progress".to_owned()
      }
      Self::ObjectiveUnspawned {
        objective,
        turns_until_spawn,
      } => format!(
        "{} is not on the map yet (spawns in {} turn(s)), so the declared force had nothing to hit",
        objective.as_str(),
        turns_until_spawn
      ),
      Self::ObjectiveSecured {
        objective,
        secured_by,
        secured_turn,
        turns_until_respawn,
      } => format!(
        "{} was secured by {} on turn {} and respawns in {} turn(s), so the declared force had nothing to hit",
        objective.as_str(),
        secured_by.as_str(),
        secured_turn,
        turns_until_respawn
      ),
      Self::ZeroDeclaredForce { objective } => format!(
        "{} is active, but the declared force was zero, so nothing landed",
        objective.as_str()
      ),
      Self::ForceCapped {
        sector,
        declared,
        present,
        delivered,
      } => format!(
        "declared {declared} force at {sector} but only {present} actor(s) stood within reach, so {delivered} landed"
      ),
      Self::Unattributed => {
        "the committed action resolved without recording an event or effect".to_owned()
      }
    }
  }
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
  /// Defensive structures as this team can see them: coarse bands under sight, own
  /// structures always, and no exact health anywhere.
  pub structures: Vec<ObservedStructure>,
  pub top_objective_status: &'static str,
  pub bot_objective_status: &'static str,
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
    /// Explains an advance that recorded nothing; `None` when the turn did work.
    note: Option<MatchTurnNote>,
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
  UnknownCommand {
    verb: String,
  },
  InvalidSyntax {
    message: String,
  },
  MissingAction,
  MissingCommittedAction,
  NothingToUndo,
  MatchAlreadyConcluded,
  MatchDidNotTerminate,
  ExecutionFailed(CompleteMatchError),
  /// A force declaration no own actor can back up was refused before staging.
  ForceWithoutPresence {
    message: String,
  },
  /// An order the single commander of this session cannot give was refused before
  /// staging: it named an actor outside the allied roster, a ward for the opposing
  /// team, or an opposing attack.
  NotCommandable {
    message: String,
  },
  DebriefUnavailable,
  UnknownHelpTopic {
    topic: String,
  },
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

  /// Create the short first-contact session, the named exception to the M9 breadth
  /// freeze (decision `D8`).
  pub fn onboarding_session() -> Self {
    Self::new(CompleteMatchCatalog::onboarding_v1())
  }

  /// Whether this session is the onboarding teaching match, which is what selects the
  /// opening briefing. Scenario identity only: this decides which banner to print,
  /// never which action is legal.
  pub fn is_onboarding(&self) -> bool {
    self.scenario_id == CompleteMatchCatalog::SCENARIO_ONBOARDING_V1
  }

  /// Create an interactive match session from a session ID or a registered plan ID.
  pub fn from_scenario_id(id: &str) -> Option<Self> {
    if id == CLI_INTERACTIVE_MATCH_ONBOARDING_SCENARIO_ID {
      return Some(Self::onboarding_session());
    }
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
    // Ward coverage reaches the player only through the redacted projection, so a
    // placed ward reveals nothing until it is paired with an opposing location there.
    let ward_coverage: Vec<(TeamSide, MapLocation)> = self
      .state
      .vision()
      .team_wards(TeamSide::Allied)
      .map(|ward| (ward.team, ward.location))
      .collect();
    let map_observation =
      observer.and_then(|actor| self.state.map().observe_with_wards(actor, &ward_coverage));
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
              OpponentSighting::Unknown => MatchActorLocation::Unknown,
            },
          )
      };
      actor_locs.push((*actor, is_allied, location));
    }
    // Sort actor locations by actor ID for deterministic display
    actor_locs.sort_by_key(|(a, _, _)| a.value());

    // Structures are projected through the same sight rule that redacts opponent
    // locations, so the host never decides visibility for itself.
    let structures = self.state.structures().observe_for(
      TeamSide::Allied,
      &self
        .state
        .map()
        .sector_sight(TeamSide::Allied, &ward_coverage),
    );

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
      structures,
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
        if tokens.len() != 3 {
          return Err(CliMatchError::InvalidSyntax {
            message: "usage: plan rotate <actor_id> <destination> (e.g. rotate 1 bot_river)".into(),
          });
        }
        let actor = parse_actor_id(tokens[1])?;
        let destination = parse_map_location(tokens[2])?;
        let desc = format!("rotate actor {} to {}", actor.value(), destination.as_str());
        let action = CompleteMatchAction::Rotate { actor, destination };
        self.stage(action, desc)
      }
      "ward" => {
        // syntax: ward <location>
        // or: ward <actor_id> <location> [turns]
        // or: ward <team> <actor_id> <location> [turns]
        // A leading team token decides the shape; anything else is actor-first.
        let (team, placed_by, location, duration_turns) = if tokens.len() == 2 {
          (
            TeamSide::Allied,
            ActorId::new(1),
            parse_map_location(tokens[1])?,
            DEFAULT_WARD_DURATION_TURNS,
          )
        } else if tokens.len() >= 4 && is_team_side_token(tokens[1]) {
          let team = parse_team_side(tokens[1])?;
          let actor = parse_actor_id(tokens[2])?;
          let location = parse_map_location(tokens[3])?;
          let duration = if tokens.len() == 5 {
            tokens[4]
              .parse::<u32>()
              .map_err(|_| CliMatchError::InvalidSyntax {
                message: "invalid ward duration; expected integer turns".into(),
              })?
          } else {
            if tokens.len() > 5 {
              return Err(CliMatchError::InvalidSyntax {
                message: format!(
                  "unexpected argument '{}'; usage: plan ward <team> <actor_id> <location> \
                   [turns]",
                  tokens[5]
                ),
              });
            }
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
        } else if tokens.len() == 4 {
          // Actor-first with an explicit duration, the spelling HOW_TO_PLAY.md documents.
          let actor = parse_actor_id(tokens[1])?;
          let location = parse_map_location(tokens[2])?;
          let duration = tokens[3]
            .parse::<u32>()
            .map_err(|_| CliMatchError::InvalidSyntax {
              message: "invalid ward duration; expected integer turns".into(),
            })?;
          (TeamSide::Allied, actor, location, duration)
        } else {
          return Err(CliMatchError::InvalidSyntax {
            message:
              "usage: plan ward <location> or plan ward <actor_id> <location> [turns] or plan \
               ward <team> <actor_id> <location> [turns]"
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
        self.stage(action, desc)
      }
      "contest" => {
        // syntax: contest <top|bot> [light|committed|all-in|damage] [burst]
        if tokens.len() < 2 {
          return Err(CliMatchError::InvalidSyntax {
            message: "usage: plan contest <top|bot> [light|committed|all-in|damage] [burst]".into(),
          });
        }
        if tokens.len() > 4 {
          return Err(CliMatchError::InvalidSyntax {
            message: format!(
              "unexpected argument '{}'; usage: plan contest <top|bot> \
               [light|committed|all-in|damage] [burst]",
              tokens[4]
            ),
          });
        }
        let objective = parse_objective_kind(tokens[1])?;
        let (damage, strength) = if tokens.len() >= 3 {
          parse_force(tokens[2], self.state.map().team_size(TeamSide::Allied))?
        } else {
          (LEGACY_DEFAULT_FORCE, None)
        };
        let is_burst = if tokens.len() == 4 {
          if !tokens[3].eq_ignore_ascii_case("burst") {
            return Err(CliMatchError::InvalidSyntax {
              message: format!(
                "unexpected argument '{}'; after the force only 'burst' is accepted",
                tokens[3]
              ),
            });
          }
          true
        } else {
          false
        };
        let intent = if is_burst {
          ObjectiveIntent::SecureBurst {
            objective,
            burst_damage: damage,
          }
        } else {
          ObjectiveIntent::Engage { objective, damage }
        };
        let desc = format!(
          "contest {} (damage={}, {}burst={})",
          match objective {
            ObjectiveKind::TopRiverObjective => "top_river_objective",
            ObjectiveKind::BotRiverObjective => "bot_river_objective",
          },
          damage,
          strength.map_or("".to_owned(), |strength| format!("strength={}, ", strength)),
          is_burst
        );
        let action = CompleteMatchAction::ContestObjectives {
          allied_intent: Some(intent),
          opposing_intent: None,
        };
        self.stage(action, desc)
      }
      "siege" => {
        // syntax: siege [side] <tier> [lane] [light|committed|all-in|damage]
        // e.g. siege outer mid committed  OR  siege allied outer mid 4000  OR  siege nexus all-in
        if tokens.len() < 3 {
          return Err(CliMatchError::InvalidSyntax {
            message: "usage: plan siege <outer|inner|inhibitor_turret|inhibitor|nexus> [lane] \
                      [light|committed|all-in|damage]"
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

        let (raw_damage, strength) = if idx < tokens.len() {
          let (raw_damage, strength) = parse_force(tokens[idx], self.state.map().team_size(side))?;
          idx += 1;
          if idx < tokens.len() {
            return Err(CliMatchError::InvalidSyntax {
              message: format!(
                "unexpected argument '{}'; usage: plan siege [side] <tier> [lane] \
                 [light|committed|all-in|damage]",
                tokens[idx]
              ),
            });
          }
          (raw_damage, strength)
        } else {
          (LEGACY_DEFAULT_FORCE, None)
        };

        let target_side = side.opposing();
        let desc = format!(
          "siege {:?} {:?}{} for {} damage ({})",
          target_side,
          tier,
          lane.map_or("".into(), |l| format!(" on {:?}", l)),
          raw_damage,
          strength.map_or(format!("attacker={:?}", side), |strength| {
            format!("attacker={:?}, strength={}", side, strength)
          })
        );
        let action = CompleteMatchAction::SiegeStructure {
          side,
          tier,
          lane,
          raw_damage,
        };
        self.stage(action, desc)
      }
      "evaluate" => {
        let desc = "evaluate terminal victory conditions".to_string();
        let action = CompleteMatchAction::EvaluateTerminal;
        self.stage(action, desc)
      }
      "idle" | "hold" | "pass" => {
        let desc = "idle (no tactical contest action)".to_string();
        let action = CompleteMatchAction::ContestObjectives {
          allied_intent: None,
          opposing_intent: None,
        };
        self.stage(action, desc)
      }
      other => Err(CliMatchError::UnknownCommand {
        verb: other.to_string(),
      }),
    }
  }

  /// Stage a parsed action, refusing orders this session's single commander cannot
  /// give before refusing force that no own actor can back up.
  ///
  /// The authority would apply such a declaration as zero damage. Committing a turn to
  /// deliver nothing is never what a player means, and both inputs to this check - the
  /// team's own positions and the static map - are facts the player already holds, so
  /// it refuses nothing the player could not have worked out alone.
  fn stage(
    &mut self,
    action: CompleteMatchAction,
    desc: String,
  ) -> Result<CliMatchOutput, CliMatchError> {
    if let Some(message) = uncommandable_order(&action, self.state.map()) {
      return Err(CliMatchError::NotCommandable { message });
    }
    if let Some((sector, team)) = self.state.force_declaration(&action) {
      let present = self
        .state
        .map()
        .presence_within(team, sector, PRESENCE_REACH_BEATS);
      if present == 0 {
        return Err(CliMatchError::ForceWithoutPresence {
          message: format!(
            "no {} actor stands in {} or a neighbouring sector, so this action would deliver no force; rotate first",
            team.as_str(),
            sector.as_str()
          ),
        });
      }
    }
    self.staged_action = Some((action, desc.clone()));
    Ok(CliMatchOutput::DraftStaged { description: desc })
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
    // Presence is read before the transition runs, because the transition is what would
    // otherwise change the picture the turn note describes.
    let declaration = self
      .state
      .force_declaration(&action)
      .map(|(sector, team)| (sector, declared_total(&action), team));
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

    // Derived after the terminal evaluation so the note reflects whether this
    // very turn concluded the match.
    let note = explain_quiet_turn(
      &action,
      &self.state,
      action_turn,
      events,
      effects,
      self.is_concluded(),
    )
    .or_else(|| {
      declaration.and_then(|(sector, declared, team)| {
        let present = self
          .state
          .map()
          .presence_within(team, sector, PRESENCE_REACH_BEATS);
        force_cap_note(
          sector,
          declared,
          present,
          deliverable_force(present, declared),
        )
      })
    });

    Ok(CliMatchOutput::Advanced {
      turn: action_turn,
      kind,
      events,
      effects,
      concluded: self.is_concluded(),
      note,
    })
  }

  fn apply_debrief(&self) -> Result<CliMatchOutput, CliMatchError> {
    if let Some((winner, condition, final_turn)) = self.conclusion {
      let final_hash = self.state.combined_hash();
      Ok(CliMatchOutput::Debrief(CompleteMatchResult {
        scenario_id: self.scenario_id,
        schema: M9_COMPLETE_MATCH_SCHEMA_V2,
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

/// Explain a force declaration that fewer actors than its player assumed could back.
///
/// The turn recorded something, so this is not a quiet-turn note: it names the roster
/// that stood behind the declaration, which is the fact the declaration cannot show.
fn force_cap_note(
  sector: MapLocation,
  declared: u32,
  present: usize,
  delivered: u32,
) -> Option<MatchTurnNote> {
  if delivered == declared {
    None
  } else {
    Some(MatchTurnNote::ForceCapped {
      sector: sector.as_str(),
      declared,
      present,
      delivered,
    })
  }
}

/// Explain a turn that recorded nothing, using only observer-visible facts.
///
/// Ward placement and terminal evaluation always record zero events and effects
/// by design, so they are always annotated. Any other quiet turn is annotated
/// with the reason derived from the committed intent and post-turn state.
fn explain_quiet_turn(
  action: &CompleteMatchAction,
  state: &CompleteMatchState,
  action_turn: u32,
  events: usize,
  effects: usize,
  concluded: bool,
) -> Option<MatchTurnNote> {
  let quiet = events == 0 && effects == 0;
  match action {
    CompleteMatchAction::PlaceWard { .. } => Some(MatchTurnNote::WardPlacement),
    CompleteMatchAction::EvaluateTerminal if !concluded => Some(MatchTurnNote::TerminalEvaluation),
    // The host stages an explicit idle as a contest turn with no declared intent.
    CompleteMatchAction::ContestObjectives {
      allied_intent: None,
      opposing_intent: None,
    } if quiet => Some(MatchTurnNote::IdleWithoutAction),
    CompleteMatchAction::ContestObjectives {
      allied_intent: Some(intent),
      ..
    } => contest_note(intent, state, action_turn, quiet),
    _ => quiet.then_some(MatchTurnNote::Unattributed),
  }
}

/// Explain an objective-contest turn whose declared intent did no work.
///
/// The target objective's status outranks a zero declaration: "that objective
/// is not on the map" explains more than "you declared zero". A zero declaration
/// is reported even when the turn recorded a spawn or ward-expiry event, because
/// the declared force itself still did nothing. Statuses are read after the turn
/// because the transition ticks spawn timers first, so a force that landed on an
/// objective spawning this very turn must not be reported as missing a target.
fn contest_note(
  intent: &ObjectiveIntent,
  state: &CompleteMatchState,
  action_turn: u32,
  quiet: bool,
) -> Option<MatchTurnNote> {
  if matches!(intent, ObjectiveIntent::ConcedeAndTrade { .. }) {
    // A trade records its concession and execution events, so a quiet trade
    // turn has no observer-visible explanation to give.
    return quiet.then_some(MatchTurnNote::Unattributed);
  }
  let objective = intent_objective(intent);
  match state.objectives().get(objective).status {
    // Still unspawned after this turn's spawn tick: nothing was there to hit.
    ObjectiveStatus::Unspawned { turns_until_spawn } => Some(MatchTurnNote::ObjectiveUnspawned {
      objective,
      turns_until_spawn,
    }),
    ObjectiveStatus::Secured {
      secured_by,
      secured_turn,
      turns_until_respawn,
    } if secured_turn != action_turn => Some(MatchTurnNote::ObjectiveSecured {
      objective,
      secured_by,
      secured_turn,
      turns_until_respawn,
    }),
    // Secured during this very turn: the declared force is what secured it.
    ObjectiveStatus::Secured { .. } => None,
    ObjectiveStatus::Active { .. } => match declared_force(intent) {
      Some(0) => Some(MatchTurnNote::ZeroDeclaredForce { objective }),
      Some(_) => None,
      None => quiet.then_some(MatchTurnNote::Unattributed),
    },
  }
}

/// Total force an action declares across both teams' intents.
///
/// The interactive session is single-sided, so this is the force the acting player
/// should expect to see land.
fn declared_total(action: &CompleteMatchAction) -> u32 {
  match action {
    CompleteMatchAction::SiegeStructure { raw_damage, .. } => *raw_damage,
    CompleteMatchAction::ContestObjectives {
      allied_intent,
      opposing_intent,
    } => {
      allied_intent.as_ref().and_then(declared_force).unwrap_or(0)
        + opposing_intent
          .as_ref()
          .and_then(declared_force)
          .unwrap_or(0)
    }
    _ => 0,
  }
}

/// Force a direct engagement intent declares, when it declares a magnitude.
fn declared_force(intent: &ObjectiveIntent) -> Option<u32> {
  match intent {
    ObjectiveIntent::Engage { damage, .. } => Some(*damage),
    ObjectiveIntent::SecureBurst { burst_damage, .. } => Some(*burst_damage),
    ObjectiveIntent::ZoneOpponents { .. } | ObjectiveIntent::ConcedeAndTrade { .. } => None,
  }
}

/// Objective a declared contest intent is aimed at, including trade intents.
fn intent_objective(intent: &ObjectiveIntent) -> ObjectiveKind {
  match intent {
    ObjectiveIntent::Engage { objective, .. }
    | ObjectiveIntent::SecureBurst { objective, .. }
    | ObjectiveIntent::ZoneOpponents { objective, .. } => *objective,
    ObjectiveIntent::ConcedeAndTrade { conceded, .. } => *conceded,
  }
}

/// Name the allied roster an observation prints, so a refusal points at the ids the
/// player can read back rather than at a range they cannot derive.
fn allied_roster_names(map: &MatchMapState) -> String {
  let mut ids: Vec<u8> = map
    .actor_locations()
    .iter()
    .filter(|(actor, _)| map.is_allied(*actor))
    .map(|(actor, _)| actor.value())
    .collect();
  ids.sort_unstable();
  ids.iter().map(u8::to_string).collect::<Vec<_>>().join(", ")
}

/// Explain why this staged line orders a team or actor the session does not command.
///
/// The interactive session has one commander: the allied team. The scripted benchmark
/// authority is untouched - recorded plans carry structured actions and never pass
/// through this grammar - but a typed line could order an opposing actor, name a ward
/// placer outside the roster, buy vision for the opposing team, or price an enemy
/// attack in the player's own turns. The enemy "never acts" is the session's own
/// promise, and an execution-time failure also reported the true position the fog was
/// withholding (`rotate 4` answered "already at destination" for an actor `observe`
/// printed as `location=unknown`). So the refusal happens here, before staging, on one
/// fact the player can read back from `observe`: the allied roster.
fn uncommandable_order(action: &CompleteMatchAction, map: &MatchMapState) -> Option<String> {
  let outside_roster = |actor: ActorId| {
    format!(
      "actor {} is not one of your actors; this session commands only the allied roster ({})",
      actor.value(),
      allied_roster_names(map),
    )
  };
  match action {
    CompleteMatchAction::Rotate { actor, .. } if !map.is_allied(*actor) => {
      Some(outside_roster(*actor))
    }
    CompleteMatchAction::PlaceWard {
      team: TeamSide::Opposing,
      ..
    } => Some(
      "a ward can only be placed for the team this session commands (allied); the opposing \
       side never acts here"
        .to_owned(),
    ),
    CompleteMatchAction::PlaceWard { placed_by, .. } if !map.is_allied(*placed_by) => {
      Some(outside_roster(*placed_by))
    }
    CompleteMatchAction::SiegeStructure {
      side: TeamSide::Opposing,
      ..
    } => Some(
      "siege names the attacking side, and this session commands only the allied team; the \
       opposing side never acts here"
        .to_owned(),
    ),
    _ => None,
  }
}

/// Resolve the force slot of `contest` or `siege` as a commit-strength token or an
/// integer.
///
/// The token is the player-facing spelling and the integer is the expert and automation
/// alias (decision D5). The host is the only place that turns a token into a number, so a
/// human, a recorded script, and an agent asking through MCP cannot drift apart. The
/// resolved figure is a *declaration*: the presence rule still decides how much of it
/// lands.
fn parse_force(
  token: &str,
  roster_size: usize,
) -> Result<(u32, Option<CommitStrength>), CliMatchError> {
  if let Some(strength) = CommitStrength::parse(token) {
    return Ok((strength.declared_force(roster_size), Some(strength)));
  }
  token
    .parse::<u32>()
    .map(|amount| (amount, None))
    .map_err(|_| CliMatchError::InvalidSyntax {
      message: format!(
        "invalid force '{token}'; expected light, committed, all-in, or an exact integer"
      ),
    })
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

/// True when a token is one the ward grammar would read as a leading team side.
/// Must mirror the vocabulary `parse_team_side` accepts, so the ward shape choice
/// never disagrees with the parser that follows it.
fn is_team_side_token(token: &str) -> bool {
  matches!(
    token.to_ascii_lowercase().as_str(),
    "allied" | "blue" | "team_a" | "opposing" | "red" | "team_b"
  )
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

/// Parse a sector name typed by a player.
///
/// Two spellings resolve: the canonical names the observation prints
/// (`lane:mid:far-side`, `base:opposing`, `river:bot`, …) and the shorter underscore
/// aliases this command loop has always accepted. Printed output has to be typeable
/// back into the game — a player who reads `lane:mid:far-side` on screen should not
/// have to guess that the verb wants `mid_far_side`. Acceptance is additive: no token
/// that resolved before now resolves elsewhere.
fn parse_map_location(token: &str) -> Result<MapLocation, CliMatchError> {
  let spoken = token.to_ascii_lowercase();
  if let Some(location) = MapLocation::ALL_LOCATIONS
    .iter()
    .find(|location| location.as_str() == spoken)
  {
    return Ok(*location);
  }
  match spoken.replace('-', "_").as_str() {
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
  use crate::map::structures::{ObservedStructureStatus, StructureHealthBand};

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
  fn placed_ward_reveals_the_opposing_actor_it_covers() {
    let mut host = CliMatchHost::default_session();
    // The canonical scenario parks opposing actor 4 at mid_far_side with no allied
    // presence there, so the ward - not an ally's position - must be what reveals it.
    assert_eq!(
      host
        .observation_report()
        .actor_locations
        .iter()
        .find(|(actor, _, _)| actor.value() == 4)
        .map(|(_, _, location)| *location),
      Some(MatchActorLocation::Unknown)
    );

    host
      .apply_line("ward allied 3 mid_far_side 3")
      .expect("ward should stage");
    host.apply_line("commit").expect("commit should succeed");
    host
      .apply_line("advance")
      .expect("advance should place the ward");

    assert_eq!(
      host
        .observation_report()
        .actor_locations
        .iter()
        .find(|(actor, _, _)| actor.value() == 4)
        .map(|(_, _, location)| *location),
      Some(MatchActorLocation::Observed(MapLocation::MID_FAR_SIDE)),
      "a ward on the opposing actor's sector must buy the player information"
    );
  }

  #[test]
  fn an_ward_elsewhere_leaves_the_opposing_actor_in_fog() {
    let mut host = CliMatchHost::default_session();

    host
      .apply_line("ward allied 3 bot_river 3")
      .expect("ward should stage");
    host.apply_line("commit").expect("commit should succeed");
    host
      .apply_line("advance")
      .expect("advance should place the ward");

    assert_eq!(
      host
        .observation_report()
        .actor_locations
        .iter()
        .find(|(actor, _, _)| actor.value() == 4)
        .map(|(_, _, location)| *location),
      Some(MatchActorLocation::Unknown),
      "ward coverage must not reveal an opponent standing in another sector"
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
      "rotate 1 mid_far_side",
      "siege inner mid 4500",
      "rotate 2 opposing_base",
      "siege inhibitor_turret mid 5000",
      "siege inhibitor mid 3500",
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

  /// The teaching scenario played the way its briefing suggests: two sieges land on
  /// turns one and two because the mid actor already stands in reach, one rotation
  /// walks the force to the enemy base, and the last three tiers fall in order. The
  /// final blow carries the lesson in its own numbers - `all-in` declared 10 500 and
  /// presence delivered 7 000 - and the match still concludes six turns in.
  #[test]
  fn the_onboarding_session_wins_in_six_turns_after_one_rotation() {
    let mut host = CliMatchHost::onboarding_session();
    assert!(host.is_onboarding());

    let lines = [
      "siege outer mid light",
      "siege inner mid committed",
      "rotate 1 lane:mid:far-side",
      "siege inhibitor_turret mid committed",
      "siege inhibitor mid light",
      "siege nexus all-in",
      "evaluate",
    ];
    let mut cap_note = None;
    for (index, line) in lines.iter().enumerate() {
      host
        .apply_line(line)
        .unwrap_or_else(|error| panic!("'{line}' should stage: {error:?}"));
      let advanced = host.apply_line("advance").expect("advance should succeed");
      let CliMatchOutput::Advanced {
        turn,
        concluded,
        note,
        ..
      } = advanced
      else {
        panic!("expected an advance for '{line}'");
      };
      let expected_turn = u32::try_from(index).expect("the script is shorter than u32::MAX") + 1;
      assert_eq!(turn, expected_turn);
      assert_eq!(concluded, *line == "evaluate");
      if *line == "siege nexus all-in" {
        cap_note = note;
      }
    }

    let expected = MatchTurnNote::ForceCapped {
      sector: "base:opposing",
      declared: 10_500,
      present: 2,
      delivered: 7_000,
    };
    assert_eq!(cap_note, Some(expected));

    let debrief = host.apply_line("debrief").expect("debrief should succeed");
    let CliMatchOutput::Debrief(result) = debrief else {
      panic!("expected debrief output");
    };
    assert_eq!(result.scenario_id, "scenario-complete-onboarding-v1");
    assert_eq!(result.winner, TeamSide::Allied);
    assert_eq!(result.condition, MatchVictoryCondition::NexusDemolished);
    assert_eq!(result.final_turn, 6);
    assert_eq!(result.allied_objectives_secured, 0);
  }

  /// The teaching session is selected by its session ID or by its plan ID, and the
  /// benchmark session is never mistaken for it.
  #[test]
  fn the_teaching_session_resolves_by_either_name_and_the_benchmark_does_not() {
    let by_session = CliMatchHost::from_scenario_id(CLI_INTERACTIVE_MATCH_ONBOARDING_SCENARIO_ID)
      .expect("the session id should resolve");
    let by_plan = CliMatchHost::from_scenario_id("scenario-complete-onboarding-v1")
      .expect("the plan id should resolve");
    assert!(by_session.is_onboarding());
    assert!(by_plan.is_onboarding());
    assert!(!CliMatchHost::default_session().is_onboarding());
  }

  /// A sector the observation prints has to be typeable back into `rotate`. Before
  /// this, `observe` showed `lane:mid:far-side` while the verb accepted only
  /// `mid_far_side` - a first-session player reads one and types the other.
  #[test]
  fn every_sector_the_observation_prints_can_be_typed_back_into_a_rotation() {
    for location in MapLocation::ALL_LOCATIONS {
      let printed = location.as_str();
      assert_eq!(parse_map_location(printed), Ok(location), "{printed}");
    }
    assert_eq!(
      parse_map_location("mid_far_side"),
      Ok(MapLocation::MID_FAR_SIDE),
      "the underscore alias still resolves"
    );
    assert_eq!(
      parse_map_location("Mid-Far-Side"),
      Ok(MapLocation::MID_FAR_SIDE),
      "dashes are normalised, case is not significant"
    );
    assert!(parse_map_location("lane:mid:nowhere").is_err());
  }

  /// Stage, commit, and advance one command, returning the turn's note.
  fn note_for(host: &mut CliMatchHost, command: &str) -> Option<MatchTurnNote> {
    host
      .apply_line(command)
      .unwrap_or_else(|error| panic!("'{command}' should stage: {error:?}"));
    host.apply_line("commit").expect("commit should succeed");
    let output = host
      .apply_line("advance")
      .unwrap_or_else(|error| panic!("'{command}' should advance: {error:?}"));
    let CliMatchOutput::Advanced { note, .. } = output else {
      panic!("expected an advance output for '{command}'");
    };
    note
  }

  #[test]
  fn quiet_turns_explain_why_nothing_happened() {
    let mut host = CliMatchHost::default_session();
    assert_eq!(
      note_for(&mut host, "contest bot 4000"),
      Some(MatchTurnNote::ObjectiveUnspawned {
        objective: ObjectiveKind::BotRiverObjective,
        turns_until_spawn: 3,
      })
    );
    assert_eq!(
      note_for(&mut host, "ward bot_river"),
      Some(MatchTurnNote::WardPlacement)
    );
    assert_eq!(
      note_for(&mut host, "idle"),
      Some(MatchTurnNote::IdleWithoutAction)
    );
    assert_eq!(
      note_for(&mut host, "evaluate"),
      Some(MatchTurnNote::TerminalEvaluation)
    );
  }

  #[test]
  fn a_turn_that_delivers_force_prints_no_note() {
    let mut host = CliMatchHost::default_session();
    for _ in 0..3 {
      assert_eq!(
        note_for(&mut host, "idle"),
        Some(MatchTurnNote::IdleWithoutAction)
      );
    }
    // The drake is on the map by turn 4, so the declared force lands and there
    // is nothing to explain; the siege lands too. Both declarations sit at the
    // one-actor delivery cap, so nothing is left over to explain.
    assert_eq!(note_for(&mut host, "contest bot 3500"), None);
    assert_eq!(note_for(&mut host, "siege outer mid 3500"), None);
  }

  #[test]
  fn a_commit_token_stages_the_force_the_matching_integer_stages() {
    let mut tokenised = CliMatchHost::default_session();
    let mut numeric = CliMatchHost::default_session();
    let staged = |host: &mut CliMatchHost, command: &str| match host.apply_line(command) {
      Ok(CliMatchOutput::DraftStaged { description }) => description,
      other => panic!("expected a staged draft, got {other:?}"),
    };
    let by_token = staged(&mut tokenised, "siege outer mid committed");
    let by_integer = staged(&mut numeric, "siege outer mid 7000");
    // The draft is where a player sees what will be declared, so the token must name the
    // same figure the integer names - and keep the token visible next to it.
    assert!(by_integer.contains("for 7000 damage"), "{by_integer}");
    assert!(by_token.contains("for 7000 damage"), "{by_token}");
    assert!(by_token.contains("strength=committed"), "{by_token}");

    // Both hosts now hold the same declaration, so the same turn must produce the same
    // authority state: the token is a spelling, not a second economy.
    for host in [&mut tokenised, &mut numeric] {
      host.apply_line("advance").expect("advance tokenised");
    }
    assert_eq!(
      tokenised.state.combined_hash(),
      numeric.state.combined_hash()
    );
  }

  #[test]
  fn an_unknown_force_word_names_the_tokens_it_accepts() {
    let mut host = CliMatchHost::default_session();
    let error = host
      .apply_line("siege outer mid heavy")
      .expect_err("a made-up force word must not stage");
    let message = match error {
      CliMatchError::InvalidSyntax { message } => message,
      other => panic!("expected invalid syntax, got {other:?}"),
    };
    for token in ["light", "committed", "all-in"] {
      assert!(message.contains(token), "{message} omits {token}");
    }
    assert!(
      message.contains("heavy"),
      "the rejection should quote the word that failed: {message}"
    );
  }

  #[test]
  fn an_over_declared_turn_reports_the_force_cap() {
    let mut host = CliMatchHost::default_session();
    for _ in 0..3 {
      note_for(&mut host, "idle");
    }
    // Exactly one allied actor stands within reach of the bot river, so the extra
    // 500 force has nobody to carry it.
    assert_eq!(
      note_for(&mut host, "contest bot 4000"),
      Some(MatchTurnNote::ForceCapped {
        sector: "river:bot",
        declared: 4_000,
        present: 1,
        delivered: 3_500,
      })
    );
  }

  #[test]
  fn force_without_presence_is_refused_before_it_is_committed() {
    let mut host = CliMatchHost::default_session();
    // Nobody stands near the enemy Nexus on turn 1, and the player can see that from
    // their own roster: the declaration is refused instead of quietly wasting a turn.
    let error = host.apply_line("siege nexus 6500").unwrap_err();
    assert!(matches!(error, CliMatchError::ForceWithoutPresence { .. }));
    assert_eq!(
      host.turn(),
      1,
      "a refused declaration must not commit a turn"
    );
    assert!(!host.is_concluded());
  }

  #[test]
  fn zero_declared_force_is_explained_against_an_active_objective() {
    let mut host = CliMatchHost::default_session();
    for _ in 0..3 {
      note_for(&mut host, "idle");
    }
    assert_eq!(
      note_for(&mut host, "contest bot 0"),
      Some(MatchTurnNote::ZeroDeclaredForce {
        objective: ObjectiveKind::BotRiverObjective
      })
    );
  }

  // --- Fog-projected structure observations ---------------------------------

  /// Project the current observation and pick one structure out of it.
  fn projected_structure(
    host: &mut CliMatchHost,
    side: TeamSide,
    tier: StructureTier,
    lane: Option<LaneId>,
  ) -> ObservedStructureStatus {
    let CliMatchOutput::Observation(report) = host.apply_line("observe").expect("observe") else {
      panic!("expected observation output");
    };
    report
      .structures
      .iter()
      .find(|structure| structure.side == side && structure.tier == tier && structure.lane == lane)
      .expect("every structure is projected, seen or not")
      .status
  }

  #[test]
  fn structure_observations_obey_team_sight() {
    let mut host = CliMatchHost::default_session();
    // Own structures are always projected, and the allied deployment also sees the mid
    // lane centre, where both teams' outer tier stands.
    let pristine = ObservedStructureStatus::Standing {
      band: StructureHealthBand::Pristine,
    };
    assert_eq!(
      projected_structure(&mut host, TeamSide::Allied, StructureTier::Nexus, None),
      pristine
    );
    assert_eq!(
      projected_structure(
        &mut host,
        TeamSide::Opposing,
        StructureTier::OuterTurret,
        Some(LaneId::Mid),
      ),
      pristine
    );
    // The opposing base is fogged: the projection does not even report whether the
    // nexus still stands.
    assert_eq!(
      projected_structure(&mut host, TeamSide::Opposing, StructureTier::Nexus, None),
      ObservedStructureStatus::NotVisible
    );

    // A ward buys sight of a sector, and with it the coarse state of what stands there.
    host.apply_line("ward mid_far_side").expect("stage ward");
    host.apply_line("commit").expect("commit");
    host.apply_line("advance").expect("advance");
    assert_eq!(
      projected_structure(
        &mut host,
        TeamSide::Opposing,
        StructureTier::InnerTurret,
        Some(LaneId::Mid),
      ),
      pristine
    );
  }

  #[test]
  fn sieged_structures_are_reported_as_bands_not_exact_health() {
    let mut host = CliMatchHost::default_session();
    // The outer tier stands in the lane centre the allied team already sees, so each
    // siege lands as a band change: 1500/3500, then 300/3500 of maximum health.
    for (damage, band) in [
      (2_000, StructureHealthBand::Chipped),
      (1_200, StructureHealthBand::Failing),
    ] {
      host
        .apply_line(&format!("siege outer mid {damage}"))
        .expect("stage siege");
      host.apply_line("commit").expect("commit");
      host.apply_line("advance").expect("advance");
      assert_eq!(
        projected_structure(
          &mut host,
          TeamSide::Opposing,
          StructureTier::OuterTurret,
          Some(LaneId::Mid),
        ),
        ObservedStructureStatus::Standing { band }
      );
    }

    host
      .apply_line("siege outer mid 400")
      .expect("stage the finishing siege");
    host.apply_line("commit").expect("commit");
    host.apply_line("advance").expect("advance");
    assert_eq!(
      projected_structure(
        &mut host,
        TeamSide::Opposing,
        StructureTier::OuterTurret,
        Some(LaneId::Mid),
      ),
      ObservedStructureStatus::Destroyed
    );
  }

  /// Trailing tokens past a verb's slots must not be silently dropped, and the two
  /// intensity slots disagreeing is a question the parser cannot answer for the player.
  #[test]
  fn no_verb_accepts_arguments_past_its_slots() {
    let mut host = CliMatchHost::default_session();
    let rejected = [
      "rotate 1 bot_river extra",
      "contest top light committed",
      "contest top committed light",
      "contest top light burst extra",
      "siege outer mid light committed",
      "siege outer mid light extra",
      "siege allied outer mid 4000 committed",
      "siege nexus all-in extra",
      "ward allied 1 bot 5 extra",
    ];
    for line in rejected {
      let err = host
        .apply_line(line)
        .expect_err("line should be refused, not silently truncated");
      assert!(
        matches!(err, CliMatchError::InvalidSyntax { .. }),
        "'{line}' gave {err:?}"
      );
    }
    // The shapes the strict grammar still accepts. Siege lines may reach the presence
    // check instead of staging, which is past the grammar and still proves the point:
    // only refusals named InvalidSyntax would mean the parser dropped or misread a token.
    for line in [
      "rotate 1 bot_river",
      "undo",
      "contest top light burst",
      "undo",
      "ward allied 1 bot 5",
      "undo",
    ] {
      host
        .apply_line(line)
        .unwrap_or_else(|err| panic!("'{line}' should parse, got {err:?}"));
    }
    for line in [
      "siege outer mid light",
      "siege allied outer mid 4000",
      "siege nexus all-in",
    ] {
      match host.apply_line(line) {
        Ok(_) => {
          host
            .apply_line("undo")
            .expect("undo should clear the draft");
        }
        Err(CliMatchError::ForceWithoutPresence { .. }) => {}
        Err(err) => panic!("'{line}' should parse, got {err:?}"),
      }
    }
  }

  /// HOW_TO_PLAY.md documents `ward [allied] <actor> <location> [turns]`; the actor-first
  /// spelling with an explicit turns value is part of that documented grammar.
  #[test]
  fn the_documented_ward_shape_with_turns_stages() {
    let mut host = CliMatchHost::default_session();
    let staged = host
      .apply_line("ward 1 bot 5")
      .expect("documented ward shape should stage");
    let CliMatchOutput::DraftStaged { description } = staged else {
      panic!("expected a staged ward");
    };
    assert!(
      description.contains("5 turns"),
      "description: {description}"
    );
    assert!(
      description.contains("actor 1"),
      "description: {description}"
    );
    host
      .apply_line("undo")
      .expect("undo should clear the draft");

    let err = host
      .apply_line("ward 1 bot five")
      .expect_err("non-integer turns must be refused");
    assert!(
      matches!(err, CliMatchError::InvalidSyntax { .. }),
      "got {err:?}"
    );
    // Team aliases keep working in the team-first shape.
    host
      .apply_line("ward blue 1 bot 5")
      .expect("team alias shape should still stage");
    host
      .apply_line("undo")
      .expect("undo should clear the draft");
  }

  /// Nothing reaches the simulation before `advance`, so a committed plan that has not
  /// advanced is still rewound by `undo`; the verb list says exactly that.
  #[test]
  fn undo_rewinds_a_committed_plan_only_until_advance() {
    let mut host = CliMatchHost::default_session();
    host
      .apply_line("rotate 1 bot_river")
      .expect("rotate should stage");
    host
      .apply_line("commit")
      .expect("commit should lock the plan");
    let undone = host
      .apply_line("undo")
      .expect("undo should rewind pre-advance");
    assert!(matches!(undone, CliMatchOutput::Undone));
    let err = host
      .apply_line("undo")
      .expect_err("a second undo has nothing left to rewind");
    assert!(matches!(err, CliMatchError::NothingToUndo));
  }

  #[test]
  fn the_session_refuses_orders_it_does_not_command() {
    let mut host = CliMatchHost::default_session();
    // An opposing actor, a phantom actor, a ward for the enemy team, and an enemy
    // attack are all refused before staging: the session has one commander.
    for line in [
      "rotate 4 lane:mid:center",
      "rotate 99 base:allied",
      "rotate 0 base:allied",
      "ward 99 base:allied",
      "ward allied 99 base:allied 3",
      "ward opposing 1 bot_river 3",
      "siege opposing outer mid",
    ] {
      let err = host
        .apply_line(line)
        .expect_err("orders outside the commander must be refused");
      assert!(
        matches!(err, CliMatchError::NotCommandable { .. }),
        "line {line} produced {err:?}"
      );
      // Refused before staging, so nothing lingers for the next probe.
      assert!(
        matches!(host.apply_line("undo"), Err(CliMatchError::NothingToUndo)),
        "line {line} must not have staged anything"
      );
    }
  }

  #[test]
  fn an_opposing_rotation_cannot_probe_a_fogged_position() {
    let mut host = CliMatchHost::default_session();
    // The canonical scenario parks opposing actor 4 at mid_far_side, printed as
    // `location=unknown`. A rotation to its true sector must refuse exactly like any
    // other rotation: no message may report where the fog withholds the actor.
    let err = host
      .apply_line("rotate 4 mid_far_side")
      .expect_err("an opposing actor is not the player's to rotate");
    let CliMatchError::NotCommandable { message } = err else {
      panic!("expected a not-commandable refusal, got {err:?}");
    };
    assert!(
      message.contains("not one of your actors"),
      "message: {message}"
    );
    assert!(
      !message.contains("mid_far_side"),
      "message leaked: {message}"
    );
    assert!(!message.contains("far-side"), "message leaked: {message}");
  }

  #[test]
  fn every_order_the_commander_can_give_still_stages() {
    let mut host = CliMatchHost::default_session();
    for line in [
      "rotate 1 bot_river",
      "rotate 2 opposing_base",
      "rotate 3 bot_river",
      "ward bot_river",
      "ward allied 3 bot_river 3",
      "siege outer mid committed",
      "contest bot all-in",
      "idle",
    ] {
      host.apply_line(line).unwrap_or_else(|err| {
        panic!("line {line} should still stage, got {err:?}");
      });
      host
        .apply_line("undo")
        .expect("undo should clear the staged line");
    }
  }
}
