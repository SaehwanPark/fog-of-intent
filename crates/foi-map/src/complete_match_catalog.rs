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

/// Current identity of the canonical complete-match catalog.
///
/// `v2` re-plans both scenarios so their declared force stands where it lands: under
/// presence-gated resolution (`m9-complete-match-v2`) a plan that declares force nobody
/// can apply now delivers nothing instead of winning anyway.
pub const M9_COMPLETE_MATCH_CATALOG_SCHEMA_V2: &str = "m9-complete-match-catalog-v2";

/// Retired identity of the catalog, whose scenarios applied full force from anywhere on
/// the map.
pub const M9_COMPLETE_MATCH_CATALOG_SCHEMA_V1: &str = "m9-complete-match-catalog-v1";

/// Catalog of registered canonical complete-match scenarios for M9.
pub struct CompleteMatchCatalog;

impl CompleteMatchCatalog {
  /// `v2` ids: each scenario's action script changed, so its identity changed with it.
  pub const SCENARIO_ALLIED_SNOWBALL_VICTORY: &'static str = "scenario-complete-allied-snowball-v2";
  pub const SCENARIO_COMEBACK_CONCESSION: &'static str = "scenario-complete-comeback-concession-v2";
  /// Interactive-only teaching scenario. It is resolved by [`Self::find`] and never
  /// returned by [`Self::all`]: `all()` is the benchmark set that the print-and-exit
  /// replay transcript executes, and a tutorial is not a benchmark.
  pub const SCENARIO_ONBOARDING_V1: &'static str = "scenario-complete-onboarding-v1";

  /// Allied early pressure: rotations set up river vision, the Drake is
  /// secured, the Mid lane is sieged through to the Nexus, and the match
  /// concludes by `NexusDemolished`.
  ///
  /// Every siege here is backed by actors standing where the damage lands. The mid
  /// laner holds the lane centre for the outer turret, then the jungler joins the far
  /// side for the inner tier, and two allied actors stand at the enemy base for the
  /// inhibitor tier and the Nexus — one actor delivers `FORCE_PER_PRESENT_ACTOR` per
  /// turn, so the 4 500-health inhibitor turret and the 6 000-health Nexus need a roster
  /// behind them rather than a bigger number.
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
      CompleteMatchAction::Rotate {
        actor: jungler,
        destination: MapLocation::MID_FAR_SIDE,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_500,
      },
      CompleteMatchAction::Rotate {
        actor: mid_laner,
        destination: MapLocation::OPPOSING_BASE,
      },
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
  ///
  /// The allied roster is three actors here rather than the two this scenario used
  /// before presence-gated resolution, and that is the point of the change: one actor
  /// delivers `FORCE_PER_PRESENT_ACTOR` per turn, so the 4 500-health inhibitor turret
  /// and every 4 000-health inner turret need a second actor standing in the enemy base
  /// sector, which is adjacent to all three lane far-sides. Two actors hold the base
  /// while the third walks the rivers that touch the lane centres and the objectives.
  pub fn comeback_concession() -> CompleteMatchPlan {
    let jungler = ActorId::new(1);
    let top_laner = ActorId::new(2);
    let mid_laner = ActorId::new(3);
    let opp_jungler = ActorId::new(4);
    let initial = CompleteMatchState::new(
      1,
      vec![jungler, top_laner, mid_laner],
      vec![opp_jungler],
      vec![
        (jungler, ActorLocation::Stationary(MapLocation::ALLIED_BASE)),
        (
          top_laner,
          ActorLocation::Stationary(MapLocation::TOP_NEAR_TOWER),
        ),
        (
          mid_laner,
          ActorLocation::Stationary(MapLocation::MID_NEAR_TOWER),
        ),
        (
          opp_jungler,
          ActorLocation::Stationary(MapLocation::BOT_RIVER),
        ),
      ],
    );

    let idle = CompleteMatchAction::ContestObjectives {
      allied_intent: None,
      opposing_intent: None,
    };
    let contest = |allied_intent, opposing_intent| CompleteMatchAction::ContestObjectives {
      allied_intent,
      opposing_intent,
    };
    let rotate = |actor, destination| CompleteMatchAction::Rotate { actor, destination };
    let siege = |side, tier, lane, raw_damage| CompleteMatchAction::SiegeStructure {
      side,
      tier,
      lane: Some(lane),
      raw_damage,
    };
    let mut actions = vec![
      // Turns 1-3: each team takes the positions it intends to fight from.
      rotate(jungler, MapLocation::TOP_RIVER),
      rotate(top_laner, MapLocation::TOP_FAR_SIDE),
      rotate(mid_laner, MapLocation::BOT_RIVER),
      // The opposing actor sits in bot river, where it can take the respawned
      // Drake and reach the allied bot and mid outer turrets.
      contest(
        None,
        Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
      ),
      siege(
        TeamSide::Opposing,
        StructureTier::OuterTurret,
        LaneId::Bot,
        4_000,
      ),
      idle,
      siege(
        TeamSide::Opposing,
        StructureTier::OuterTurret,
        LaneId::Mid,
        4_000,
      ),
      contest(
        None,
        Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
      ),
      CompleteMatchAction::PlaceWard {
        team: TeamSide::Allied,
        placed_by: jungler,
        location: MapLocation::TOP_RIVER,
        duration_turns: DEFAULT_WARD_DURATION_TURNS,
      },
      // The Drake spawns on its fourth contest tick and the opposing actor is the
      // only actor standing in its sector, so it takes the lead.
      contest(
        None,
        Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
      ),
      idle,
      // Allied opens all three lanes while its actors still stand in the rivers that
      // touch the lane centres: the outer tier is all a river actor can back up.
      siege(
        TeamSide::Allied,
        StructureTier::OuterTurret,
        LaneId::Top,
        4_000,
      ),
      idle,
      siege(
        TeamSide::Allied,
        StructureTier::OuterTurret,
        LaneId::Mid,
        4_000,
      ),
      siege(
        TeamSide::Allied,
        StructureTier::OuterTurret,
        LaneId::Bot,
        4_000,
      ),
      // Two allied actors stand at the Herald, so the declared burst lands.
      contest(
        Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::TopRiverObjective,
          damage: 5_500,
        }),
        None,
      ),
      idle,
      // The bot-river actor takes the respawned Drake one tick after it returns.
      idle,
      contest(
        Some(ObjectiveIntent::Engage {
          objective: ObjectiveKind::BotRiverObjective,
          damage: 4_000,
        }),
        None,
      ),
      idle,
      idle,
      contest(
        Some(ObjectiveIntent::SecureBurst {
          objective: ObjectiveKind::TopRiverObjective,
          burst_damage: 5_500,
        }),
        None,
      ),
    ];
    // Turns 23-33: the two top-side actors move into the enemy base sector, which is
    // adjacent to every lane far-side. Two present actors deliver 7 000 force per turn,
    // which is what a 4 000-health inner turret, a 4 500-health inhibitor turret, and a
    // 3 000-health inhibitor each cost. The three inhibitors go back-to-back so none
    // respawns before the terminal evaluation.
    actions.push(rotate(jungler, MapLocation::OPPOSING_BASE));
    actions.push(rotate(top_laner, MapLocation::OPPOSING_BASE));
    let mut lane_sieges: Vec<CompleteMatchAction> = [LaneId::Top, LaneId::Mid, LaneId::Bot]
      .iter()
      .flat_map(|lane| {
        [
          (StructureTier::InnerTurret, 4_500u32),
          (StructureTier::InhibitorTurret, 5_000),
        ]
        .iter()
        .map(move |(tier, damage)| siege(TeamSide::Allied, *tier, *lane, *damage))
        .collect::<Vec<_>>()
      })
      .collect();
    for lane in [LaneId::Top, LaneId::Mid, LaneId::Bot] {
      lane_sieges.push(siege(
        TeamSide::Allied,
        StructureTier::Inhibitor,
        lane,
        3_500,
      ));
    }
    actions.extend(lane_sieges);
    actions.push(CompleteMatchAction::EvaluateTerminal);

    CompleteMatchPlan {
      scenario_id: Self::SCENARIO_COMEBACK_CONCESSION,
      initial,
      actions,
    }
  }

  /// First-contact teaching match, and the named exception to the M9 breadth freeze
  /// (decision `D8`): a newcomer's opening session should not be a fourteen-turn
  /// benchmark. Three allied actors stand already in position to take the mid outer
  /// turret on turn one, and the deep tiers are deliberately one rotation short of
  /// the force they need, so the second lesson is presence rather than an error: the
  /// siege runs, delivers less than it declared, and says so in a `force-capped`
  /// turn note. The opposing actor never acts — the interactive host executes the
  /// player's commands, and no opposing policy runs — so nothing can be lost here.
  ///
  /// `actions` is empty by design. The scripted action list exists for the replay-
  /// verified benchmarks; in a teaching scenario the player writes the script.
  pub fn onboarding_v1() -> CompleteMatchPlan {
    let mid_laner = ActorId::new(1);
    let jungler = ActorId::new(2);
    let support = ActorId::new(3);
    let opp_mid = ActorId::new(4);
    let initial = CompleteMatchState::new(
      1,
      vec![mid_laner, jungler, support],
      vec![opp_mid],
      vec![
        (
          mid_laner,
          ActorLocation::Stationary(MapLocation::MID_CENTER),
        ),
        (
          jungler,
          ActorLocation::Stationary(MapLocation::MID_FAR_SIDE),
        ),
        (support, ActorLocation::Stationary(MapLocation::ALLIED_BASE)),
        (
          opp_mid,
          ActorLocation::Stationary(MapLocation::OPPOSING_BASE),
        ),
      ],
    );

    CompleteMatchPlan {
      scenario_id: Self::SCENARIO_ONBOARDING_V1,
      initial,
      actions: Vec::new(),
    }
  }

  /// Resolve a scenario ID, including interactive-only plans such as the onboarding
  /// match that [`Self::all`] deliberately excludes.
  pub fn find(scenario_id: &str) -> Option<CompleteMatchPlan> {
    match scenario_id {
      Self::SCENARIO_ALLIED_SNOWBALL_VICTORY => Some(Self::allied_snowball_victory()),
      Self::SCENARIO_COMEBACK_CONCESSION => Some(Self::comeback_concession()),
      Self::SCENARIO_ONBOARDING_V1 => Some(Self::onboarding_v1()),
      _ => None,
    }
  }

  /// The canonical benchmark plans: everything the print-and-exit replay transcript
  /// executes, and every plan whose hashes are quoted as evidence. Teaching plans
  /// resolved by [`Self::find`] are excluded, so adding one never changes that
  /// transcript or the benchmark set.
  pub fn all() -> Vec<CompleteMatchPlan> {
    vec![Self::allied_snowball_victory(), Self::comeback_concession()]
  }
}
