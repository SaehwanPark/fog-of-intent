//! Canonical composed complete-match scenarios for M9.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! Each scenario is one `CompleteMatchPlan` that plays a full match through
//! the real transition families — rotations, warding, objective contests,
//! and structure sieges — and terminates through a match victory condition.
//! Scenarios are reproducible: the same plan always replays to the identical
//! final combined hash.

use super::complete_match::{CompleteMatchAction, CompleteMatchPlan, CompleteMatchState};
use super::contest::ObjectiveIntent;
use super::objective::ObjectiveKind;
use super::structures::StructureTier;
use super::topology::{LaneId, MapLocation, TeamSide};
use super::travel::ActorLocation;
use super::vision::DEFAULT_WARD_DURATION_TURNS;
use crate::kernel::ActorId;

pub const M9_COMPLETE_MATCH_CATALOG_SCHEMA_V1: &str = "m9-complete-match-catalog-v1";

/// Catalog of registered canonical complete-match scenarios for M9.
pub struct CompleteMatchCatalog;

impl CompleteMatchCatalog {
  pub const SCENARIO_ALLIED_SNOWBALL_VICTORY: &'static str = "scenario-complete-allied-snowball-v1";
  pub const SCENARIO_COMEBACK_CONCESSION: &'static str = "scenario-complete-comeback-concession-v1";

  /// Allied early pressure: rotations set up river vision, the Drake is
  /// secured, the Mid lane is sieged through to the Nexus, and the match
  /// concludes by `NexusDemolished`.
  pub fn allied_snowball_victory() -> CompleteMatchPlan {
    let jungler = ActorId::new(1);
    let mid_laner = ActorId::new(2);
    let support = ActorId::new(3);
    let opp_mid = ActorId::new(4);
    let initial = CompleteMatchState::new(
      1,
      vec![jungler, mid_laner, support],
      vec![opp_mid],
      vec![
        (jungler, ActorLocation::Stationary(MapLocation::ALLIED_BASE)),
        (
          mid_laner,
          ActorLocation::Stationary(MapLocation::MID_CENTER),
        ),
        (
          support,
          ActorLocation::Stationary(MapLocation::BOT_NEAR_TOWER),
        ),
        (
          opp_mid,
          ActorLocation::Stationary(MapLocation::MID_FAR_SIDE),
        ),
      ],
    );

    let idle = CompleteMatchAction::ContestObjectives {
      allied_intent: None,
      opposing_intent: None,
    };
    let actions = vec![
      CompleteMatchAction::Rotate {
        actor: jungler,
        destination: MapLocation::BOT_RIVER,
      },
      CompleteMatchAction::PlaceWard {
        team: TeamSide::Allied,
        placed_by: support,
        location: MapLocation::BOT_RIVER,
        duration_turns: DEFAULT_WARD_DURATION_TURNS,
      },
      idle,
      idle,
      idle,
      CompleteMatchAction::ContestObjectives {
        allied_intent: Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
        opposing_intent: None,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_000,
      },
      idle,
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_500,
      },
      idle,
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::InhibitorTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 5_000,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::Inhibitor,
        lane: Some(LaneId::Mid),
        raw_damage: 3_500,
      },
      CompleteMatchAction::Rotate {
        actor: mid_laner,
        destination: MapLocation::OPPOSING_BASE,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::Nexus,
        lane: None,
        raw_damage: 6_500,
      },
      CompleteMatchAction::EvaluateTerminal,
    ];

    CompleteMatchPlan {
      scenario_id: Self::SCENARIO_ALLIED_SNOWBALL_VICTORY,
      initial,
      actions,
    }
  }

  /// A comeback by concession: the opposing side takes an early objective
  /// lead and Allied outer turrets, but Allied answers with objective
  /// control, tears down all three inhibitor lanes, and the match concludes
  /// by `MatchConceded`.
  pub fn comeback_concession() -> CompleteMatchPlan {
    let jungler = ActorId::new(1);
    let top_laner = ActorId::new(2);
    let opp_jungler = ActorId::new(4);
    let initial = CompleteMatchState::new(
      1,
      vec![jungler, top_laner],
      vec![opp_jungler],
      vec![
        (jungler, ActorLocation::Stationary(MapLocation::ALLIED_BASE)),
        (
          top_laner,
          ActorLocation::Stationary(MapLocation::TOP_NEAR_TOWER),
        ),
        (
          opp_jungler,
          ActorLocation::Stationary(MapLocation::TOP_RIVER),
        ),
      ],
    );

    let idle = CompleteMatchAction::ContestObjectives {
      allied_intent: None,
      opposing_intent: None,
    };
    let mut actions = vec![
      idle,
      idle,
      idle,
      CompleteMatchAction::ContestObjectives {
        allied_intent: None,
        opposing_intent: Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Opposing,
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Top),
        raw_damage: 4_000,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Opposing,
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_000,
      },
      idle,
      CompleteMatchAction::PlaceWard {
        team: TeamSide::Allied,
        placed_by: jungler,
        location: MapLocation::TOP_RIVER,
        duration_turns: DEFAULT_WARD_DURATION_TURNS,
      },
      CompleteMatchAction::ContestObjectives {
        allied_intent: Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::TopRiverObjective,
          damage: 5_500,
        }),
        opposing_intent: None,
      },
      idle,
      idle,
      idle,
      CompleteMatchAction::ContestObjectives {
        allied_intent: Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
        opposing_intent: None,
      },
      idle,
      idle,
      CompleteMatchAction::ContestObjectives {
        allied_intent: Some(ObjectiveIntent::SecureBurst {
          objective: ObjectiveKind::TopRiverObjective,
          burst_damage: 5_500,
        }),
        opposing_intent: None,
      },
    ];
    // Turns 17-28: Allied opens all three lanes through the defense
    // hierarchy, then takes the three inhibitors back-to-back so none
    // respawns (five-turn respawn) before the terminal evaluation.
    let mut lane_sieges: Vec<CompleteMatchAction> = [LaneId::Top, LaneId::Mid, LaneId::Bot]
      .iter()
      .flat_map(|lane| {
        [
          (StructureTier::OuterTurret, 4_000u32),
          (StructureTier::InnerTurret, 4_500),
          (StructureTier::InhibitorTurret, 5_000),
        ]
        .iter()
        .map(move |(tier, damage)| CompleteMatchAction::SiegeStructure {
          side: TeamSide::Allied,
          tier: *tier,
          lane: Some(*lane),
          raw_damage: *damage,
        })
        .collect::<Vec<_>>()
      })
      .collect();
    for lane in [LaneId::Top, LaneId::Mid, LaneId::Bot] {
      lane_sieges.push(CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::Inhibitor,
        lane: Some(lane),
        raw_damage: 3_500,
      });
    }
    actions.extend(lane_sieges);
    actions.push(CompleteMatchAction::EvaluateTerminal);

    CompleteMatchPlan {
      scenario_id: Self::SCENARIO_COMEBACK_CONCESSION,
      initial,
      actions,
    }
  }

  pub fn find(scenario_id: &str) -> Option<CompleteMatchPlan> {
    match scenario_id {
      Self::SCENARIO_ALLIED_SNOWBALL_VICTORY => Some(Self::allied_snowball_victory()),
      Self::SCENARIO_COMEBACK_CONCESSION => Some(Self::comeback_concession()),
      _ => None,
    }
  }

  pub fn all() -> Vec<CompleteMatchPlan> {
    vec![Self::allied_snowball_victory(), Self::comeback_concession()]
  }
}
