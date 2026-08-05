#[test]
fn player_lane_constructors_preserve_resource_defaults() {
    let id = PLAYER_LANER;
    let health = LaneHealth::new(8).expect("bounded health");
    let position = LanePosition::Center;
    let mana = LaneMana::new(4).expect("bounded mana");
    let gold = LaneGold::new(3).expect("bounded gold");
    let experience = LaneExperience::new(7).expect("bounded experience");
    let cooldown = LaneCooldown::new(2).expect("bounded cooldown");
    let bounty = LaneBounty::new(5).expect("bounded bounty");
    let level = LaneLevel::new(3).expect("bounded level");
    let minion_kills = LaneMinionKills::new(9).expect("bounded minion kills");

    assert_eq!(
        PlayerLaneState::new(id, health, position),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            LaneMana::full(),
            LaneGold::zero(),
            LaneExperience::zero(),
            LaneCooldown::zero(),
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            position,
        )
    );
    assert_eq!(
        PlayerLaneState::new_with_mana(id, health, mana, position),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            mana,
            LaneGold::zero(),
            LaneExperience::zero(),
            LaneCooldown::zero(),
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            position,
        )
    );
    assert_eq!(
        PlayerLaneState::new_with_resources(id, health, mana, gold, position),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            mana,
            gold,
            LaneExperience::zero(),
            LaneCooldown::zero(),
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            position,
        )
    );
    assert_eq!(
        PlayerLaneState::new_with_all_resources(id, health, mana, gold, experience, position),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            mana,
            gold,
            experience,
            LaneCooldown::zero(),
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            position,
        )
    );
    assert_eq!(
        PlayerLaneState::new_with_complete_state(
            id, health, mana, gold, experience, cooldown, position
        ),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            mana,
            gold,
            experience,
            cooldown,
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            position,
        )
    );
    assert_eq!(
        PlayerLaneState::new_with_full_state(
            id, health, mana, gold, experience, cooldown, bounty, position
        ),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            mana,
            gold,
            experience,
            cooldown,
            bounty,
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            position,
        )
    );
    assert_eq!(
        PlayerLaneState::new_with_entire_state(
            id, health, mana, gold, experience, cooldown, bounty, level, position
        ),
        PlayerLaneState::new_with_absolute_state(
            id,
            health,
            mana,
            gold,
            experience,
            cooldown,
            bounty,
            level,
            LaneMinionKills::zero(),
            position,
        )
    );
    let absolute = PlayerLaneState::new_with_absolute_state(
        id,
        health,
        mana,
        gold,
        experience,
        cooldown,
        bounty,
        level,
        minion_kills,
        position,
    );
    assert_eq!(absolute.mana(), mana);
    assert_eq!(absolute.gold(), gold);
    assert_eq!(absolute.experience(), experience);
    assert_eq!(absolute.cooldown(), cooldown);
    assert_eq!(absolute.bounty(), bounty);
    assert_eq!(absolute.level(), level);
    assert_eq!(absolute.minion_kills(), minion_kills);
}

#[test]
fn lane_hash_encodings_remain_stable_across_the_module_split() {
    assert_eq!(
        LaneSnapshot::initial().hash().value(),
        18_346_439_562_823_728_570
    );

    let resolved = LaneSnapshot::new(
        M2_LANE_RULESET,
        Turn::new(1),
        LanePhase::Resolved,
        PlayerLaneState::new_with_absolute_state(
            PLAYER_LANER,
            LaneHealth::new(7).expect("bounded health"),
            LaneMana::new(5).expect("bounded mana"),
            LaneGold::new(2).expect("bounded gold"),
            LaneExperience::new(3).expect("bounded experience"),
            LaneCooldown::new(4).expect("bounded cooldown"),
            LaneBounty::new(5).expect("bounded bounty"),
            LaneLevel::new(3).expect("bounded level"),
            LaneMinionKills::new(6).expect("bounded minion kills"),
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
        Some(LaneOutcome::HeldSpace),
    );
    assert_eq!(resolved.hash().value(), 17_489_665_152_642_147_642);

    let allied = observe_allied(&LaneSnapshot::initial(), ObservationId::new(9)).observation();
    assert_eq!(
        allied_input_identity(allied, trace(3, 3))
            .visible_digest()
            .value(),
        2_696_744_198_952_712_513
    );
}
