//! Focused tests for M9 composed complete-match execution.
//!
//! Covers:
//! - Both canonical complete-match plans terminate through their victory
//!   conditions with the expected winner, objectives, and turn counts
//! - Replay determinism: identical plans reproduce identical results and
//!   combined hashes from the initial state
//! - The combined hash commits every subsystem: perturbing any one subsystem
//!   changes it
//! - Every phase kind (rotation, warding, objective contest, structure
//!   siege, terminal evaluation) appears in the phase logs
//! - Fail-closed behavior: empty plans, unterminated plans, actions after
//!   conclusion, untracked-actor rotations, and subsystem rejections
//! - Markdown rendering contains match labels without hidden state

use crate::kernel::ActorId;
use crate::map::complete_match::{
  CompleteMatchAction, CompleteMatchError, CompleteMatchPlan, CompleteMatchState,
  M9_COMPLETE_MATCH_SCHEMA_V2, MatchPhaseKind,
};
use crate::map::complete_match_catalog::CompleteMatchCatalog;
use crate::map::contest::ObjectiveIntent;
use crate::map::objective::{ObjectiveKind, ObjectiveStatus};
use crate::map::structures::StructureStatus;
use crate::map::structures::StructureTier;
use crate::map::topology::{LaneId, MapLocation, TeamSide};
use crate::map::travel::ActorLocation;
use crate::map::victory::MatchVictoryCondition;

fn single_actor_state() -> CompleteMatchState {
  let actor = ActorId::new(1);
  CompleteMatchState::new(
    1,
    vec![actor],
    vec![],
    vec![(actor, ActorLocation::Stationary(MapLocation::ALLIED_BASE))],
  )
}

/// A roster positioned to back a full enemy mid-ladder siege.
///
/// Presence caps delivery, so a fixture that expects a ladder to fall needs actors in
/// the sectors the damage lands in: the lane far-side sector reaches the outer tier,
/// the inner tier, and the enemy base, and two present actors are what a 4 500-health
/// inhibitor turret and a 6 000-health Nexus require.
fn siege_roster_state() -> CompleteMatchState {
  let forward = ActorId::new(1);
  let second = ActorId::new(2);
  let deep = ActorId::new(3);
  CompleteMatchState::new(
    1,
    vec![forward, second, deep],
    vec![],
    vec![
      (
        forward,
        ActorLocation::Stationary(MapLocation::MID_FAR_SIDE),
      ),
      (second, ActorLocation::Stationary(MapLocation::MID_FAR_SIDE)),
      (deep, ActorLocation::Stationary(MapLocation::OPPOSING_BASE)),
    ],
  )
}

fn terminating_plan() -> CompleteMatchPlan {
  CompleteMatchPlan {
    scenario_id: "test-nexus-plan",
    initial: siege_roster_state(),
    actions: vec![
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_000,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_500,
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
    ],
  }
}

// --- Canonical scenarios ---

#[test]
fn allied_snowball_terminates_by_nexus_demolition() {
  let plan = CompleteMatchCatalog::allied_snowball_victory();
  let result = plan.execute().expect("snowball plan executes");
  assert_eq!(result.schema, M9_COMPLETE_MATCH_SCHEMA_V2);
  assert_eq!(result.winner, TeamSide::Allied);
  assert_eq!(result.condition, MatchVictoryCondition::NexusDemolished);
  assert_eq!(result.allied_objectives_secured, 1);
  assert_eq!(result.opposing_objectives_secured, 0);
  // The Nexus falls to the turn-14 siege; the turn-15 evaluation confirms
  // the subsystem conclusion and reports its turn.
  assert_eq!(result.final_turn, 14);
  // Every mechanic family appears in the phase log.
  let kinds: Vec<MatchPhaseKind> = result.phases.iter().map(|phase| phase.kind).collect();
  for expected in [
    MatchPhaseKind::Rotation,
    MatchPhaseKind::Warding,
    MatchPhaseKind::ObjectiveContest,
    MatchPhaseKind::StructureSiege,
    MatchPhaseKind::TerminalEvaluation,
  ] {
    assert!(kinds.contains(&expected), "missing phase {expected}");
  }
}

#[test]
fn comeback_concession_terminates_by_concession() {
  let plan = CompleteMatchCatalog::comeback_concession();
  let result = plan.execute().expect("comeback plan executes");
  assert_eq!(result.winner, TeamSide::Allied);
  assert_eq!(result.condition, MatchVictoryCondition::MatchConceded);
  assert_eq!(result.allied_objectives_secured, 3);
  assert_eq!(result.opposing_objectives_secured, 1);
  // Presence-gated resolution made this scenario longer: the roster has to walk into
  // the enemy base before the deep tiers can fall, which is the point of the change.
  assert_eq!(result.final_turn, 34);
}

#[test]
fn catalog_find_and_all_are_consistent() {
  assert_eq!(CompleteMatchCatalog::all().len(), 2);
  assert!(CompleteMatchCatalog::find("scenario-complete-allied-snowball-v2").is_some());
  assert!(CompleteMatchCatalog::find("scenario-complete-comeback-concession-v2").is_some());
  assert!(CompleteMatchCatalog::find("missing").is_none());
  for plan in CompleteMatchCatalog::all() {
    let result = plan.execute().expect("catalog plan executes");
    assert_eq!(result.scenario_id, plan.scenario_id);
  }
}

// --- Replay determinism ---

#[test]
fn identical_plans_replay_to_identical_results_and_hashes() {
  for plan in CompleteMatchCatalog::all() {
    let first = plan.execute().expect("first run");
    let second = plan.execute().expect("replay run");
    assert_eq!(
      first, second,
      "{} must replay identically",
      plan.scenario_id
    );
    assert_eq!(first.initial_hash, second.initial_hash);
    assert_eq!(first.final_hash, second.final_hash);
    assert_ne!(first.initial_hash, first.final_hash);
  }
  let plan = terminating_plan();
  let first = plan.execute().expect("first run");
  let second = plan.execute().expect("replay run");
  assert_eq!(first, second);
}

// --- Combined hash commitment ---

#[test]
fn final_hash_commits_subsystem_changes() {
  // Two plans identical except for the ward's location: with no contest
  // actions the wards never expire, so a final-hash difference proves the
  // combined hash commits vision state.
  let actor = ActorId::new(1);
  let ward_plan = |ward_location: MapLocation| CompleteMatchPlan {
    scenario_id: "hash-vision-check",
    initial: siege_roster_state(),
    actions: vec![
      CompleteMatchAction::Rotate {
        actor,
        destination: MapLocation::MID_NEAR_TOWER,
      },
      CompleteMatchAction::PlaceWard {
        team: TeamSide::Allied,
        placed_by: actor,
        location: ward_location,
        duration_turns: 5,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::OuterTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_000,
      },
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_500,
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
    ],
  };
  let top_river = ward_plan(MapLocation::TOP_RIVER)
    .execute()
    .expect("top-river run");
  let bot_river = ward_plan(MapLocation::BOT_RIVER)
    .execute()
    .expect("bot-river run");
  assert_ne!(
    top_river.final_hash, bot_river.final_hash,
    "the combined hash must commit vision state"
  );

  // Differing team membership produces differing initial hashes: the same
  // actor at the same location, but assigned to the opposing team.
  let actor = ActorId::new(1);
  let allied_actor = CompleteMatchState::new(
    1,
    vec![actor],
    vec![],
    vec![(actor, ActorLocation::Stationary(MapLocation::ALLIED_BASE))],
  );
  let opposing_actor = CompleteMatchState::new(
    1,
    vec![],
    vec![actor],
    vec![(actor, ActorLocation::Stationary(MapLocation::ALLIED_BASE))],
  );
  assert_ne!(
    allied_actor.combined_hash(),
    opposing_actor.combined_hash(),
    "the combined hash must commit team membership"
  );
  assert_eq!(
    allied_actor.clone().combined_hash(),
    allied_actor.combined_hash()
  );
}

#[test]
fn ward_id_history_is_committed_by_the_hash() {
  // Two terminated plans whose active-ward sets coincide but whose placement
  // histories differ must hash differently.
  let actor = ActorId::new(1);
  let siege = |tier, damage, lane| CompleteMatchAction::SiegeStructure {
    side: TeamSide::Allied,
    tier,
    lane,
    raw_damage: damage,
  };
  let sieges = vec![
    siege(StructureTier::OuterTurret, 4_000, Some(LaneId::Mid)),
    siege(StructureTier::InnerTurret, 4_500, Some(LaneId::Mid)),
    siege(StructureTier::InhibitorTurret, 5_000, Some(LaneId::Mid)),
    siege(StructureTier::Inhibitor, 3_500, Some(LaneId::Mid)),
    siege(StructureTier::Nexus, 6_500, None),
  ];
  let ward = |duration| CompleteMatchAction::PlaceWard {
    team: TeamSide::Allied,
    placed_by: actor,
    location: MapLocation::TOP_RIVER,
    duration_turns: duration,
  };
  // Early one-turn ward expires during the turn-3 contest tick; a second
  // ward at the same spot leaves one active ward with a later id.
  let early = CompleteMatchPlan {
    scenario_id: "ward-history-early",
    initial: siege_roster_state(),
    actions: vec![
      ward(1),
      CompleteMatchAction::ContestObjectives {
        allied_intent: None,
        opposing_intent: None,
      },
      CompleteMatchAction::ContestObjectives {
        allied_intent: None,
        opposing_intent: None,
      },
      ward(9),
      siege(StructureTier::OuterTurret, 4_000, Some(LaneId::Mid)),
      siege(StructureTier::InnerTurret, 4_500, Some(LaneId::Mid)),
      siege(StructureTier::InhibitorTurret, 5_000, Some(LaneId::Mid)),
      siege(StructureTier::Inhibitor, 3_500, Some(LaneId::Mid)),
      siege(StructureTier::Nexus, 6_500, None),
      CompleteMatchAction::EvaluateTerminal,
    ],
  }
  .execute()
  .expect("early-ward run");
  // Late single ward: same active set shape, different id sequence.
  let late = CompleteMatchPlan {
    scenario_id: "ward-history-late",
    initial: siege_roster_state(),
    actions: {
      let mut actions = vec![
        CompleteMatchAction::Rotate {
          actor,
          destination: MapLocation::MID_NEAR_TOWER,
        },
        CompleteMatchAction::Rotate {
          actor,
          destination: MapLocation::ALLIED_BASE,
        },
        ward(9),
      ];
      actions.extend(sieges);
      actions.push(CompleteMatchAction::EvaluateTerminal);
      actions
    },
  }
  .execute()
  .expect("late-ward run");
  assert_ne!(
    early.final_hash, late.final_hash,
    "the combined hash must commit ward placement history"
  );
}

// --- Presence-gated delivery ---

#[test]
fn force_without_presence_delivers_nothing() {
  let mut state = single_actor_state();
  // Nobody stands near the enemy Nexus. The siege is recorded, applies no damage, and
  // is not an error: presence removes force, not legality.
  let (kind, events, effects) = state
    .apply_action(&CompleteMatchAction::SiegeStructure {
      side: TeamSide::Allied,
      tier: StructureTier::Nexus,
      lane: None,
      raw_damage: 6_500,
    })
    .expect("an unbacked siege is recorded, not rejected");
  assert_eq!(kind, MatchPhaseKind::StructureSiege);
  assert_eq!(events, 0);
  assert_eq!(effects, 0);
  assert!(
    state
      .structures()
      .get_structure(TeamSide::Opposing, None, StructureTier::Nexus)
      .is_some_and(|entry| entry.status.is_standing()),
    "an unbacked siege must leave the Nexus standing"
  );

  // An objective intent with nobody in its sector delivers nothing either, while the
  // contest transition still runs so spawn timers keep their cadence.
  let mut state = single_actor_state();
  let idle = CompleteMatchAction::ContestObjectives {
    allied_intent: None,
    opposing_intent: None,
  };
  for _ in 0..5 {
    state.apply_action(&idle).expect("uncommitted contest");
  }
  state
    .apply_action(&CompleteMatchAction::ContestObjectives {
      allied_intent: Some(ObjectiveIntent::Engage {
        objective: ObjectiveKind::BotRiverObjective,
        damage: 4_000,
      }),
      opposing_intent: None,
    })
    .expect("the Drake has spawned by now");
  assert_eq!(
    state
      .objectives()
      .get(ObjectiveKind::BotRiverObjective)
      .status,
    ObjectiveStatus::Active {
      current_health: 3_500,
      max_health: 3_500,
      engaged_by: None
    },
    "declared force with no present actor must not damage the objective"
  );
}

#[test]
fn declared_force_is_clamped_to_present_actors() {
  let lone = ActorId::new(1);
  let support = ActorId::new(2);
  let siege_inner = CompleteMatchAction::SiegeStructure {
    side: TeamSide::Allied,
    tier: StructureTier::InnerTurret,
    lane: Some(LaneId::Mid),
    raw_damage: 4_500,
  };
  let mut state = CompleteMatchState::new(
    1,
    vec![lone, support],
    vec![],
    vec![
      (lone, ActorLocation::Stationary(MapLocation::MID_FAR_SIDE)),
      (support, ActorLocation::Stationary(MapLocation::ALLIED_BASE)),
    ],
  );
  // The ladder opens first: a lone present actor takes the 3 500-health outer turret.
  state
    .apply_action(&CompleteMatchAction::SiegeStructure {
      side: TeamSide::Allied,
      tier: StructureTier::OuterTurret,
      lane: Some(LaneId::Mid),
      raw_damage: 4_000,
    })
    .expect("the outer tier is backed by the actor in the sector");
  // One present actor delivers one actor's worth of force, so the over-declared 4 500
  // leaves the 4 000-health inner turret standing at the difference.
  state
    .apply_action(&siege_inner)
    .expect("one actor still delivers");
  assert!(
    matches!(
      state
        .structures()
        .get_structure(
          TeamSide::Opposing,
          Some(LaneId::Mid),
          StructureTier::InnerTurret
        )
        .map(|entry| entry.status),
      Some(StructureStatus::Standing {
        current_hp: 500,
        ..
      })
    ),
    "a lone present actor must deliver a partial siege, not the declared total"
  );

  // With a second actor in the sector the same declaration is fully backed.
  state
    .apply_action(&CompleteMatchAction::Rotate {
      actor: support,
      destination: MapLocation::MID_FAR_SIDE,
    })
    .expect("the support rotates into the sector");
  state
    .apply_action(&siege_inner)
    .expect("two present actors back the declaration");
  assert!(
    state
      .structures()
      .get_structure(
        TeamSide::Opposing,
        Some(LaneId::Mid),
        StructureTier::InnerTurret
      )
      .is_some_and(|entry| !entry.status.is_standing()),
    "a second present actor must back the same declaration"
  );
}

// --- Fail-closed behavior ---

#[test]
fn empty_plans_are_rejected() {
  let plan = CompleteMatchPlan {
    scenario_id: "empty",
    initial: single_actor_state(),
    actions: vec![],
  };
  assert_eq!(plan.execute(), Err(CompleteMatchError::EmptyPlan));
}

#[test]
fn unterminated_plans_are_rejected() {
  let mut plan = terminating_plan();
  plan.actions.pop(); // drop the EvaluateTerminal action
  assert_eq!(
    plan.execute(),
    Err(CompleteMatchError::MatchDidNotTerminate)
  );
}

#[test]
fn actions_after_a_subsystem_conclusion_are_rejected() {
  // The Nexus falls at the turn-5 siege; a further rotation must fail closed
  // even though no EvaluateTerminal has run yet.
  let actor = ActorId::new(1);
  let mut plan = terminating_plan();
  plan.actions.insert(
    5,
    CompleteMatchAction::Rotate {
      actor,
      destination: MapLocation::MID_NEAR_TOWER,
    },
  );
  assert_eq!(
    plan.execute(),
    Err(CompleteMatchError::MatchAlreadyConcluded)
  );
}

#[test]
fn actions_after_conclusion_are_rejected() {
  let mut plan = terminating_plan();
  plan.actions.push(CompleteMatchAction::EvaluateTerminal);
  assert_eq!(
    plan.execute(),
    Err(CompleteMatchError::MatchAlreadyConcluded)
  );
}

#[test]
fn rotating_an_untracked_actor_is_rejected() {
  let plan = CompleteMatchPlan {
    scenario_id: "untracked",
    initial: single_actor_state(),
    actions: vec![
      CompleteMatchAction::Rotate {
        actor: ActorId::new(99),
        destination: MapLocation::MID_NEAR_TOWER,
      },
      CompleteMatchAction::EvaluateTerminal,
    ],
  };
  assert_eq!(plan.execute(), Err(CompleteMatchError::UntrackedActor));
}

#[test]
fn illegal_sieges_are_rejected_through_the_structure_transition() {
  // Inner turret before outer turret violates the vulnerability hierarchy. The
  // fixture roster stands where the damage would land, so the subsystem's legality
  // rejection is what surfaces rather than the presence cap.
  let plan = CompleteMatchPlan {
    scenario_id: "illegal-siege",
    initial: siege_roster_state(),
    actions: vec![
      CompleteMatchAction::SiegeStructure {
        side: TeamSide::Allied,
        tier: StructureTier::InnerTurret,
        lane: Some(LaneId::Mid),
        raw_damage: 4_500,
      },
      CompleteMatchAction::EvaluateTerminal,
    ],
  };
  let error = plan.execute().expect_err("hierarchy violation must fail");
  assert!(matches!(error, CompleteMatchError::Siege(_)));
  assert!(error.to_string().contains("siege failed"));
}

#[test]
fn error_display_covers_every_runner_variant() {
  assert_eq!(
    CompleteMatchError::EmptyPlan.to_string(),
    "empty plan: at least one action is required"
  );
  assert!(
    CompleteMatchError::MatchDidNotTerminate
      .to_string()
      .contains("in progress")
  );
  assert!(
    CompleteMatchError::MatchAlreadyConcluded
      .to_string()
      .contains("no further actions")
  );
  assert!(
    CompleteMatchError::UntrackedActor
      .to_string()
      .contains("absent from the roster")
  );
}

// --- Markdown rendering ---

#[test]
fn markdown_contains_match_labels_without_hidden_state() {
  let plan = CompleteMatchCatalog::allied_snowball_victory();
  let result = plan.execute().expect("snowball plan executes");
  let markdown = result.render_markdown();
  assert!(markdown.contains("# M9 Complete Match Report"));
  assert!(markdown.contains("**Scenario**: `scenario-complete-allied-snowball-v2`"));
  assert!(markdown.contains("**Winner**: Allied"));
  assert!(markdown.contains("**Condition**: `nexus-demolished`"));
  assert!(markdown.contains("**Objectives Secured**: allied 1, opposing 0"));
  assert!(markdown.contains("## Phase Log"));
  assert!(markdown.contains("`rotation`"));
  assert!(markdown.contains("`objective-contest`"));
  assert!(markdown.contains("`structure-siege`"));
  assert!(markdown.contains("`terminal-evaluation`"));
  assert!(!markdown.to_lowercase().contains("chain-of-thought"));
  assert!(!markdown.to_lowercase().contains("hash"));
}
