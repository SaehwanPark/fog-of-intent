//! Unit tests for M9 structures, vulnerability hierarchy, siege damage, inhibitor respawns, and state hashing.

use crate::map::structures::{
  MatchStructureState, SiegeIntent, StructureError, StructureStatus, StructureTier,
  transition_structure_siege,
};
use crate::map::topology::{LaneId, TeamSide};

#[test]
fn standard_map_initialization_has_26_standing_structures() {
  let state = MatchStructureState::new_standard_map();
  assert_eq!(state.structures().len(), 26);

  for entry in state.structures() {
    assert!(entry.status.is_standing());
    assert_eq!(entry.status.current_hp(), entry.tier.default_max_hp());
  }

  assert_eq!(state.destroyed_count_for_team(TeamSide::Allied), 0);
  assert_eq!(state.destroyed_count_for_team(TeamSide::Opposing), 0);
  assert!(state.check_nexus_destroyed().is_none());
}

#[test]
fn vulnerability_hierarchy_enforcement() {
  let mut state = MatchStructureState::new_standard_map();

  // Tier 1 (Outer) is initially vulnerable
  assert!(state.is_vulnerable(
    TeamSide::Opposing,
    Some(LaneId::Mid),
    StructureTier::OuterTurret
  ));

  // Tier 2 (Inner), Tier 3 (Inhibitor Turret), Inhibitor, Nexus are initially invulnerable
  assert!(!state.is_vulnerable(
    TeamSide::Opposing,
    Some(LaneId::Mid),
    StructureTier::InnerTurret
  ));
  assert!(!state.is_vulnerable(
    TeamSide::Opposing,
    Some(LaneId::Mid),
    StructureTier::InhibitorTurret
  ));
  assert!(!state.is_vulnerable(
    TeamSide::Opposing,
    Some(LaneId::Mid),
    StructureTier::Inhibitor
  ));
  assert!(!state.is_vulnerable(TeamSide::Opposing, None, StructureTier::Nexus));

  // Attacking invulnerable Tier 2 fails closed
  let err = transition_structure_siege(
    1,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::InnerTurret,
      lane: Some(LaneId::Mid),
      raw_damage: 2000,
    },
    None,
  );
  assert_eq!(err, Err(StructureError::StructureInvulnerable));

  // Destroy Outer Turret
  let res = transition_structure_siege(
    2,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::OuterTurret,
      lane: Some(LaneId::Mid),
      raw_damage: 4000,
    },
    None,
  )
  .unwrap();
  assert!(res.structure_destroyed);

  // Now Tier 2 (Inner) becomes vulnerable in Mid lane
  assert!(state.is_vulnerable(
    TeamSide::Opposing,
    Some(LaneId::Mid),
    StructureTier::InnerTurret
  ));
  // But Tier 2 in Top/Bot remains invulnerable
  assert!(!state.is_vulnerable(
    TeamSide::Opposing,
    Some(LaneId::Top),
    StructureTier::InnerTurret
  ));
}

#[test]
fn defense_mitigation_and_damage_application() {
  let mut state = MatchStructureState::new_standard_map();

  // Attack with 2000 raw damage against 50% (5000 bp) defense mitigation -> 1000 effective damage
  let res = transition_structure_siege(
    3,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::OuterTurret,
      lane: Some(LaneId::Bot),
      raw_damage: 2000,
    },
    Some(SiegeIntent::DefendStructure {
      lane: Some(LaneId::Bot),
      mitigation_bp: 5000,
    }),
  )
  .unwrap();

  assert_eq!(res.effective_damage, 1000);
  assert!(!res.structure_destroyed);

  let bot_outer = state
    .get_structure(
      TeamSide::Opposing,
      Some(LaneId::Bot),
      StructureTier::OuterTurret,
    )
    .unwrap();
  assert_eq!(bot_outer.status.current_hp(), 2500); // 3500 - 1000
}

#[test]
fn inhibitor_destruction_spawns_super_minions_and_respawns_after_countdown() {
  let mut state = MatchStructureState::new_standard_map();

  // Destroy Top Outer, Inner, Inhibitor Turret
  transition_structure_siege(
    4,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::OuterTurret,
      lane: Some(LaneId::Top),
      raw_damage: 4000,
    },
    None,
  )
  .unwrap();
  transition_structure_siege(
    5,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::InnerTurret,
      lane: Some(LaneId::Top),
      raw_damage: 4500,
    },
    None,
  )
  .unwrap();
  transition_structure_siege(
    6,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::InhibitorTurret,
      lane: Some(LaneId::Top),
      raw_damage: 5000,
    },
    None,
  )
  .unwrap();

  // Destroy Inhibitor at turn 7
  let res = transition_structure_siege(
    7,
    &mut state,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::Inhibitor,
      lane: Some(LaneId::Top),
      raw_damage: 3500,
    },
    None,
  )
  .unwrap();

  assert!(res.structure_destroyed);
  assert!(res.super_minions_spawned);
  assert!(state.has_super_minions(TeamSide::Allied, LaneId::Top));
  assert!(!state.has_super_minions(TeamSide::Opposing, LaneId::Top));

  // Opposing Nexus is now vulnerable
  assert!(state.is_vulnerable(TeamSide::Opposing, None, StructureTier::Nexus));

  // Inhibitor status is Respawning with 5 turns
  let inhib = state
    .get_structure(
      TeamSide::Opposing,
      Some(LaneId::Top),
      StructureTier::Inhibitor,
    )
    .unwrap();
  assert_eq!(
    inhib.status,
    StructureStatus::Respawning {
      destroyed_turn: 7,
      remaining_turns: 5,
    }
  );

  // Tick 4 turns -> remaining turns decrements
  for _ in 0..4 {
    let events = state.tick_turn();
    assert!(events.is_empty());
  }

  // Tick 5th turn -> Inhibitor respawns and emits InhibitorRespawned event
  let events = state.tick_turn();
  assert_eq!(events.len(), 1);

  let inhib_after = state
    .get_structure(
      TeamSide::Opposing,
      Some(LaneId::Top),
      StructureTier::Inhibitor,
    )
    .unwrap();
  assert!(inhib_after.status.is_standing());
  assert_eq!(inhib_after.status.current_hp(), 3000);
  assert!(!state.has_super_minions(TeamSide::Allied, LaneId::Top));
}

#[test]
fn state_hash_determinism_and_distinctness() {
  let mut state1 = MatchStructureState::new_standard_map();
  let state2 = MatchStructureState::new_standard_map();

  assert_eq!(state1.compute_hash(1), state2.compute_hash(1));

  // Applying damage to state1 changes its hash distinctly
  transition_structure_siege(
    1,
    &mut state1,
    TeamSide::Allied,
    SiegeIntent::AttackStructure {
      tier: StructureTier::OuterTurret,
      lane: Some(LaneId::Mid),
      raw_damage: 1000,
    },
    None,
  )
  .unwrap();

  assert_ne!(state1.compute_hash(1), state2.compute_hash(1));
}
