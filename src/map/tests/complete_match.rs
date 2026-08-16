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
  M9_COMPLETE_MATCH_SCHEMA_V1, MatchPhaseKind,
};
use crate::map::complete_match_catalog::CompleteMatchCatalog;
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

fn terminating_plan() -> CompleteMatchPlan {
  CompleteMatchPlan {
    scenario_id: "test-nexus-plan",
    initial: single_actor_state(),
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
  assert_eq!(result.schema, M9_COMPLETE_MATCH_SCHEMA_V1);
  assert_eq!(result.winner, TeamSide::Allied);
  assert_eq!(result.condition, MatchVictoryCondition::NexusDemolished);
  assert_eq!(result.allied_objectives_secured, 1);
  assert_eq!(result.opposing_objectives_secured, 0);
  assert_eq!(result.final_turn, 15);
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
  assert_eq!(result.final_turn, 29);
}

#[test]
fn catalog_find_and_all_are_consistent() {
  assert_eq!(CompleteMatchCatalog::all().len(), 2);
  assert!(CompleteMatchCatalog::find("scenario-complete-allied-snowball-v1").is_some());
  assert!(CompleteMatchCatalog::find("scenario-complete-comeback-concession-v1").is_some());
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
    initial: single_actor_state(),
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

  // Differing rosters produce differing initial hashes.
  let two_actors = CompleteMatchState::new(
    1,
    vec![ActorId::new(1), ActorId::new(2)],
    vec![],
    vec![
      (
        ActorId::new(1),
        ActorLocation::Stationary(MapLocation::ALLIED_BASE),
      ),
      (
        ActorId::new(2),
        ActorLocation::Stationary(MapLocation::MID_CENTER),
      ),
    ],
  );
  assert_ne!(
    two_actors.combined_hash(),
    single_actor_state().combined_hash()
  );
  assert_eq!(
    two_actors.clone().combined_hash(),
    two_actors.combined_hash()
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
  // Inner turret before outer turret violates the vulnerability hierarchy.
  let plan = CompleteMatchPlan {
    scenario_id: "illegal-siege",
    initial: single_actor_state(),
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
  assert!(markdown.contains("**Scenario**: `scenario-complete-allied-snowball-v1`"));
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
