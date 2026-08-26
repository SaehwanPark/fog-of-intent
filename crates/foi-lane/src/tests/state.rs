use super::*;

#[test]
fn lane_resources_are_constructed_as_one_typed_aggregate() {
  let resources = LaneResources::new(
    LaneMana::new(4).expect("bounded mana"),
    LaneGold::new(3).expect("bounded gold"),
    LaneExperience::new(7).expect("bounded experience"),
    LaneCooldown::new(2).expect("bounded cooldown"),
  );
  let player = PlayerLaneState::new(
    PLAYER_LANER,
    LaneHealth::new(8).expect("bounded health"),
    resources,
    LanePosition::Center,
  );
  assert_eq!(player.resources(), resources);
  assert_eq!(player.mana(), resources.mana());
  assert_eq!(player.gold(), resources.gold());
  assert_eq!(player.experience(), resources.experience());
  assert_eq!(player.cooldown(), resources.cooldown());
}

#[test]
fn actor_roster_is_stable_and_does_not_change_lane_state_hash() {
  let roster = LaneActorRoster::initial();
  assert_eq!(
    roster.entries(),
    [
      (LaneActorRole::HumanLaner, PLAYER_LANER),
      (LaneActorRole::OpposingLaner, OPPONENT_LANER),
      (LaneActorRole::AlliedAutonomous, ALLIED_AUTONOMOUS_ACTOR),
      (
        LaneActorRole::OpposingJungleThreat,
        OPPOSING_JUNGLE_THREAT_ACTOR
      ),
    ]
  );
  assert_eq!(
    roster.actor(LaneActorRole::OpposingJungleThreat),
    OPPOSING_JUNGLE_THREAT_ACTOR
  );
  assert_eq!(
    LaneSnapshot::initial().hash(),
    LaneSnapshot::initial().hash()
  );
}

#[test]
fn lane_status_cannot_represent_correlated_phase_and_outcome_pairs() {
  assert_eq!(LaneStatus::Open.phase(), LanePhase::Open);
  assert_eq!(LaneStatus::Open.outcome(), None);
  assert_eq!(
    LaneStatus::Resolved(LaneOutcome::HeldSpace).phase(),
    LanePhase::Resolved
  );
  assert_eq!(
    LaneStatus::Resolved(LaneOutcome::HeldSpace).outcome(),
    Some(LaneOutcome::HeldSpace)
  );
  assert_eq!(LaneSnapshot::initial().status(), LaneStatus::Open);
}

#[test]
fn lane_hash_encodings_are_stable_for_v3_initial_and_non_default_states() {
  let initial = LaneSnapshot::initial();
  let resources = LaneResources::new(
    LaneMana::new(5).expect("bounded mana"),
    LaneGold::new(2).expect("bounded gold"),
    LaneExperience::new(3).expect("bounded experience"),
    LaneCooldown::new(4).expect("bounded cooldown"),
  );
  let resolved = LaneSnapshot::new(
    M2_LANE_RULESET,
    Turn::new(1),
    LaneStatus::Resolved(LaneOutcome::HeldSpace),
    PlayerLaneState::new(
      PLAYER_LANER,
      LaneHealth::new(7).expect("bounded health"),
      resources,
      LanePosition::Center,
    ),
    OpponentTruth::new(
      OPPONENT_LANER,
      LaneHealth::new(5).expect("bounded health"),
      LanePosition::Center,
      OpponentPosture::Aggressive,
    ),
    WaveState::new(WavePressure::new(2).expect("bounded pressure")),
    JungleThreatTruth::InLane,
  );
  assert_eq!(initial.hash().value(), 6_571_807_888_986_103_628);
  assert_eq!(resolved.hash().value(), 66_596_859_370_032_817);
  assert_ne!(initial.hash(), resolved.hash());
  assert_eq!(initial.hash(), LaneSnapshot::initial().hash());
}
