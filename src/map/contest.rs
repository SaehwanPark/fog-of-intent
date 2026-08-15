//! Cross-map objective contests, tradeoff evaluations, and causal event/effect transitions for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype

use core::fmt;

use crate::kernel::ActorId;

use super::objective::{DamageOutcome, MatchObjectiveState, ObjectiveKind};
use super::topology::{JungleSide, LaneId, MapLocation, TeamSide};
use super::vision::MapVisionState;

/// Cross-map strategic targets traded when conceding an objective.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CrossMapTradeTarget {
  /// Trade for the objective on the opposite side of the map (e.g. Herald for Dragon).
  OppositeObjective(ObjectiveKind),
  /// Trade for lane tower damage or wave pressure on an opposing lane.
  OppositeTowerPush(LaneId),
  /// Trade for farming opponent jungle quadrant camps.
  JungleInvadeFarm(JungleSide),
}

impl CrossMapTradeTarget {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::OppositeObjective(ObjectiveKind::TopRiverObjective) => "opposite-objective:top-herald",
      Self::OppositeObjective(ObjectiveKind::BotRiverObjective) => "opposite-objective:bot-drake",
      Self::OppositeTowerPush(LaneId::Top) => "opposite-tower:top",
      Self::OppositeTowerPush(LaneId::Mid) => "opposite-tower:mid",
      Self::OppositeTowerPush(LaneId::Bot) => "opposite-tower:bot",
      Self::JungleInvadeFarm(JungleSide::TopJungle) => "jungle-invade:top",
      Self::JungleInvadeFarm(JungleSide::BotJungle) => "jungle-invade:bot",
    }
  }

  pub const fn location(self) -> MapLocation {
    match self {
      Self::OppositeObjective(kind) => kind.location(),
      Self::OppositeTowerPush(lane) => MapLocation::Lane(lane, super::topology::LaneSector::Center),
      Self::JungleInvadeFarm(jungle) => MapLocation::Jungle(jungle),
    }
  }

  pub const fn base_value_bp(self) -> u32 {
    match self {
      Self::OppositeObjective(kind) => kind.strategic_value_bp(),
      Self::OppositeTowerPush(LaneId::Mid) => 4000,
      Self::OppositeTowerPush(LaneId::Top) => 3500,
      Self::OppositeTowerPush(LaneId::Bot) => 3000,
      Self::JungleInvadeFarm(_) => 2000,
    }
  }
}

impl fmt::Display for CrossMapTradeTarget {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Categorical classification of strategic cross-map trades.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TradeClassification {
  /// Trade resulted in a net strategic advantage (>= +500 bp).
  FavorableTrade,
  /// Trade was approximately equal in strategic value ([-499..=499] bp).
  EvenTrade,
  /// Trade resulted in an unfavorable loss of map value ([-2000..=-500] bp).
  UnfavorableConcession,
  /// Desperation sacrifice under heavy deficit (< -2000 bp).
  DesperationSacrifice,
}

impl TradeClassification {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FavorableTrade => "favorable-trade",
      Self::EvenTrade => "even-trade",
      Self::UnfavorableConcession => "unfavorable-concession",
      Self::DesperationSacrifice => "desperation-sacrifice",
    }
  }
}

impl fmt::Display for TradeClassification {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Exact integer basis-point evaluation of a cross-map strategic tradeoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TradeoffEvaluation {
  pub conceded_objective: ObjectiveKind,
  pub trade_target: CrossMapTradeTarget,
  pub conceded_value_bp: u32,
  pub gained_value_bp: u32,
  pub net_value_delta_bp: i32,
  pub classification: TradeClassification,
}

impl TradeoffEvaluation {
  pub fn evaluate(
    conceded: ObjectiveKind,
    target: CrossMapTradeTarget,
    execution_multiplier_bp: u32,
  ) -> Self {
    let conceded_val = conceded.strategic_value_bp();
    let base_gained = target.base_value_bp();
    // Bounded integer basis-point scaling: base_gained * multiplier / 10,000
    let scaled_u64 =
      u64::from(base_gained) * u64::from(execution_multiplier_bp.min(10_000)) / 10_000;
    let scaled_gained = u32::try_from(scaled_u64).unwrap_or(u32::MAX);

    let gained_i32 = i32::try_from(scaled_gained).unwrap_or(i32::MAX);
    let conceded_i32 = i32::try_from(conceded_val).unwrap_or(i32::MAX);
    let net_delta = gained_i32.saturating_sub(conceded_i32);
    let classification = if net_delta >= 500 {
      TradeClassification::FavorableTrade
    } else if net_delta >= -499 {
      TradeClassification::EvenTrade
    } else if net_delta >= -2000 {
      TradeClassification::UnfavorableConcession
    } else {
      TradeClassification::DesperationSacrifice
    };

    Self {
      conceded_objective: conceded,
      trade_target: target,
      conceded_value_bp: conceded_val,
      gained_value_bp: scaled_gained,
      net_value_delta_bp: net_delta,
      classification,
    }
  }
}

/// Structured causal events emitted during objective contests and vision updates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveEvent {
  ObjectiveSpawned {
    kind: ObjectiveKind,
    turn: u32,
  },
  ObjectiveDamageDealt {
    kind: ObjectiveKind,
    team: TeamSide,
    damage: u32,
    remaining_health: u32,
  },
  ObjectiveSecured {
    kind: ObjectiveKind,
    secured_by: TeamSide,
    turn: u32,
  },
  ObjectiveConceded {
    kind: ObjectiveKind,
    conceded_by: TeamSide,
    turn: u32,
  },
  CrossMapTradeExecuted {
    conceded: ObjectiveKind,
    trade_target: CrossMapTradeTarget,
    net_value_delta_bp: i32,
    classification: TradeClassification,
  },
  WardPlaced {
    ward_id: u32,
    team: TeamSide,
    location: MapLocation,
    placed_by: ActorId,
    turn: u32,
  },
  WardExpired {
    ward_id: u32,
    team: TeamSide,
    location: MapLocation,
  },
  WardCleared {
    ward_id: u32,
    cleared_by_team: TeamSide,
    location: MapLocation,
  },
}

/// Structured attributed effects resulting from objective transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveEffect {
  ObjectiveBuffApplied {
    team: TeamSide,
    kind: ObjectiveKind,
    secure_count: u32,
  },
  CrossMapPressureShifted {
    lane: LaneId,
    pressure_delta: i32,
  },
  VisionGranted {
    team: TeamSide,
    location: MapLocation,
    turns: u32,
  },
}

/// Actor intents during an objective contest window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectiveIntent {
  /// Contribute regular sustained damage toward taking down the objective.
  Engage {
    objective: ObjectiveKind,
    damage: u32,
  },
  /// Execute a burst secure attempt (e.g. smite/finisher spell).
  SecureBurst {
    objective: ObjectiveKind,
    burst_damage: u32,
  },
  /// Zone and hold off opposing actors from approaching the objective.
  ZoneOpponents {
    objective: ObjectiveKind,
    zoning_power: u32,
  },
  /// Concede the objective and execute a cross-map trade on the opposite side.
  ConcedeAndTrade {
    conceded: ObjectiveKind,
    target: CrossMapTradeTarget,
    execution_efficiency_bp: u32,
  },
}

/// Result of evaluating an objective contest turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContestTransitionResult {
  pub events: Vec<ObjectiveEvent>,
  pub effects: Vec<ObjectiveEffect>,
  pub tradeoff: Option<TradeoffEvaluation>,
}

/// Pure deterministic transition evaluating objective actions and vision updates for a turn.
pub fn transition_objective_contest(
  objective_state: &mut MatchObjectiveState,
  vision_state: &mut MapVisionState,
  allied_intent: Option<ObjectiveIntent>,
  opposing_intent: Option<ObjectiveIntent>,
  current_turn: u32,
) -> ContestTransitionResult {
  let mut events = Vec::new();
  let mut effects = Vec::new();
  let mut tradeoff = None;

  // 1. Tick objective spawning timers
  let spawned = objective_state.tick_turn();
  for kind in spawned {
    events.push(ObjectiveEvent::ObjectiveSpawned {
      kind,
      turn: current_turn,
    });
  }

  // 2. Tick vision ward expiry
  let expired_wards = vision_state.tick_turn();
  for ward in expired_wards {
    events.push(ObjectiveEvent::WardExpired {
      ward_id: ward.ward_id,
      team: ward.team,
      location: ward.location,
    });
  }

  // 3. Resolve cross-map trades first if present
  if let Some(ObjectiveIntent::ConcedeAndTrade {
    conceded,
    target,
    execution_efficiency_bp,
  }) = allied_intent
  {
    let eval = TradeoffEvaluation::evaluate(conceded, target, execution_efficiency_bp);
    tradeoff = Some(eval);
    events.push(ObjectiveEvent::ObjectiveConceded {
      kind: conceded,
      conceded_by: TeamSide::Allied,
      turn: current_turn,
    });
    events.push(ObjectiveEvent::CrossMapTradeExecuted {
      conceded,
      trade_target: target,
      net_value_delta_bp: eval.net_value_delta_bp,
      classification: eval.classification,
    });
    match target {
      CrossMapTradeTarget::OppositeTowerPush(lane) => {
        effects.push(ObjectiveEffect::CrossMapPressureShifted {
          lane,
          pressure_delta: 2,
        });
      }
      CrossMapTradeTarget::OppositeObjective(opp_kind) => {
        if let Ok(DamageOutcome::Secured { secured_by }) =
          objective_state.apply_damage(opp_kind, 5000, TeamSide::Allied, current_turn)
        {
          events.push(ObjectiveEvent::ObjectiveSecured {
            kind: opp_kind,
            secured_by,
            turn: current_turn,
          });
          effects.push(ObjectiveEffect::ObjectiveBuffApplied {
            team: TeamSide::Allied,
            kind: opp_kind,
            secure_count: objective_state.get(opp_kind).secure_count_allied,
          });
        }
      }
      CrossMapTradeTarget::JungleInvadeFarm(_) => {}
    }
  }

  // 4. Resolve direct objective engagement / contest damage
  let mut allied_dmg = 0;
  let mut allied_burst = 0;
  let mut allied_target = None;

  match allied_intent {
    Some(ObjectiveIntent::Engage { objective, damage }) => {
      allied_target = Some(objective);
      allied_dmg = damage;
    }
    Some(ObjectiveIntent::SecureBurst {
      objective,
      burst_damage,
    }) => {
      allied_target = Some(objective);
      allied_burst = burst_damage;
    }
    _ => {}
  }

  let mut opp_dmg = 0;
  let mut opp_burst = 0;
  let mut opp_target = None;

  match opposing_intent {
    Some(ObjectiveIntent::Engage { objective, damage }) => {
      opp_target = Some(objective);
      opp_dmg = damage;
    }
    Some(ObjectiveIntent::SecureBurst {
      objective,
      burst_damage,
    }) => {
      opp_target = Some(objective);
      opp_burst = burst_damage;
    }
    _ => {}
  }

  // Apply damage for allied target if active
  if let Some(target) = allied_target {
    let total_dmg = allied_dmg.saturating_add(allied_burst);
    if total_dmg > 0 && objective_state.get(target).status.is_active() {
      match objective_state.apply_damage(target, total_dmg, TeamSide::Allied, current_turn) {
        Ok(DamageOutcome::Damaged { remaining_health }) => {
          events.push(ObjectiveEvent::ObjectiveDamageDealt {
            kind: target,
            team: TeamSide::Allied,
            damage: total_dmg,
            remaining_health,
          });
        }
        Ok(DamageOutcome::Secured { secured_by }) => {
          events.push(ObjectiveEvent::ObjectiveSecured {
            kind: target,
            secured_by,
            turn: current_turn,
          });
          effects.push(ObjectiveEffect::ObjectiveBuffApplied {
            team: secured_by,
            kind: target,
            secure_count: objective_state.get(target).secure_count_allied,
          });
        }
        Err(_) => {}
      }
    }
  }

  // Apply damage for opposing target if active
  if let Some(target) = opp_target {
    let total_dmg = opp_dmg.saturating_add(opp_burst);
    if total_dmg > 0 && objective_state.get(target).status.is_active() {
      match objective_state.apply_damage(target, total_dmg, TeamSide::Opposing, current_turn) {
        Ok(DamageOutcome::Damaged { remaining_health }) => {
          events.push(ObjectiveEvent::ObjectiveDamageDealt {
            kind: target,
            team: TeamSide::Opposing,
            damage: total_dmg,
            remaining_health,
          });
        }
        Ok(DamageOutcome::Secured { secured_by }) => {
          events.push(ObjectiveEvent::ObjectiveSecured {
            kind: target,
            secured_by,
            turn: current_turn,
          });
          effects.push(ObjectiveEffect::ObjectiveBuffApplied {
            team: secured_by,
            kind: target,
            secure_count: objective_state.get(target).secure_count_opposing,
          });
        }
        Err(_) => {}
      }
    }
  }

  ContestTransitionResult {
    events,
    effects,
    tradeoff,
  }
}
