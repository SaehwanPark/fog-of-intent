#[test]
    fn gold_is_bounded_and_default_zero() {
        assert_eq!(LaneGold::zero().value(), 0);
        assert_eq!(LaneGold::new(20).unwrap().value(), 20);
        assert!(LaneGold::new(21).is_err());
        let zero = LaneGold::zero();
        let earned = LaneGold::new(5).unwrap();
        assert_eq!(zero.add(earned), Some(earned));
        assert_eq!(earned.add(LaneGold::new(16).unwrap()), None);
        assert_eq!(
            earned.subtract(LaneGold::new(2).unwrap()),
            Some(LaneGold::new(3).unwrap())
        );
    }

#[test]
    fn gold_earned_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().gold(), LaneGold::zero());

        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut gold_inputs = inputs(1, 2, LaneWaveResult::Held);
        gold_inputs.execution = gold_inputs
            .execution
            .with_gold_earned(LaneGold::new(5).unwrap());

        let result =
            transition_lane(&state, &validated, &gold_inputs).expect("transition with gold");
        assert_eq!(
            result.next_state().player().gold(),
            LaneGold::new(5).unwrap()
        );
        assert_ne!(state.hash(), result.next_state().hash());

        let player_obs = observe_player(&result.next_state(), ObservationId::new(1)).observation();
        assert_eq!(player_obs.self_gold(), LaneGold::new(5).unwrap());
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(allied_obs.laner_gold(), LaneGold::new(5).unwrap());

        assert_eq!(result.debrief().gold_earned(), LaneGold::new(5).unwrap());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::GoldEarned { amount, .. } if *amount == LaneGold::new(5).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::GoldChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneGold::zero()
                && *after == LaneGold::new(5).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &request, gold_inputs)
            .expect("append gold execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn gold_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.gold_earned = LaneGold(21); // bypass constructor for error test

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::GoldOverflow { .. }
            ))
        ));
    }

#[test]
    fn experience_is_bounded_and_default_zero() {
        assert_eq!(LaneExperience::zero().value(), 0);
        assert_eq!(LaneExperience::new(50).unwrap().value(), 50);
        assert!(LaneExperience::new(51).is_err());
        let zero = LaneExperience::zero();
        let gained = LaneExperience::new(15).unwrap();
        assert_eq!(gained.add(LaneExperience::new(36).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneExperience::new(5).unwrap()),
            Some(LaneExperience::new(10).unwrap())
        );
        assert_eq!(zero.subtract(gained), None);
    }

#[test]
    fn experience_gained_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().experience(), LaneExperience::zero());

        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut xp_inputs = inputs(1, 2, LaneWaveResult::Held);
        xp_inputs.execution = xp_inputs
            .execution
            .with_experience_gained(LaneExperience::new(15).unwrap());

        let result = transition_lane(&state, &validated, &xp_inputs).expect("valid transition");
        assert_eq!(
            result.next_state().player().experience(),
            LaneExperience::new(15).unwrap()
        );
        let player_obs = observe_player(&result.next_state(), ObservationId::new(2)).observation();
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(
            player_obs.self_experience(),
            LaneExperience::new(15).unwrap()
        );
        assert_eq!(
            allied_obs.laner_experience(),
            LaneExperience::new(15).unwrap()
        );
        assert_eq!(
            result.debrief().experience_gained(),
            LaneExperience::new(15).unwrap()
        );

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::ExperienceGained { amount, .. } if *amount == LaneExperience::new(15).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::ExperienceChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneExperience::zero()
                && *after == LaneExperience::new(15).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &request, xp_inputs)
            .expect("append xp execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn experience_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.experience_gained = LaneExperience(51);

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ExperienceOverflow { .. }
            ))
        ));
    }

#[test]
    fn cooldown_is_bounded_and_default_zero() {
        assert_eq!(LaneCooldown::zero().value(), 0);
        assert_eq!(LaneCooldown::new(10).unwrap().value(), 10);
        assert!(LaneCooldown::new(11).is_err());
        let cd = LaneCooldown::new(3).unwrap();
        assert_eq!(cd.tick(1).value(), 2);
        assert_eq!(cd.tick(5).value(), 0);
        assert_eq!(
            cd.add(LaneCooldown::new(7).unwrap()),
            Some(LaneCooldown::new(10).unwrap())
        );
        assert_eq!(cd.add(LaneCooldown::new(8).unwrap()), None);
    }

#[test]
    fn cooldown_set_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().cooldown(), LaneCooldown::zero());

        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut cd_inputs = inputs(1, 2, LaneWaveResult::Held);
        cd_inputs.execution = cd_inputs
            .execution
            .with_cooldown_set(LaneCooldown::new(3).unwrap());

        let result = transition_lane(&state, &validated, &cd_inputs).expect("valid transition");
        assert_eq!(
            result.next_state().player().cooldown(),
            LaneCooldown::new(3).unwrap()
        );
        let player_obs = observe_player(&result.next_state(), ObservationId::new(2)).observation();
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(player_obs.self_cooldown(), LaneCooldown::new(3).unwrap());
        assert_eq!(allied_obs.laner_cooldown(), LaneCooldown::new(3).unwrap());
        assert_eq!(
            result.debrief().cooldown_set(),
            LaneCooldown::new(3).unwrap()
        );

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::CooldownSet { amount, .. } if *amount == LaneCooldown::new(3).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::CooldownChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneCooldown::zero()
                && *after == LaneCooldown::new(3).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &request, cd_inputs)
            .expect("append cd execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn cooldown_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.cooldown_set = LaneCooldown(11);

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::CooldownOverflow { .. }
            ))
        ));
    }

#[test]
    fn bounty_is_bounded_and_default_zero() {
        assert_eq!(LaneBounty::zero().value(), 0);
        assert_eq!(LaneBounty::new(100).unwrap().value(), 100);
        assert!(LaneBounty::new(101).is_err());
    }

#[test]
    fn bounty_earned_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().bounty(), LaneBounty::zero());

        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut bounty_inputs = inputs(1, 2, LaneWaveResult::Held);
        bounty_inputs.execution = bounty_inputs
            .execution
            .with_bounty_earned(LaneBounty::new(50).unwrap());

        let result = transition_lane(&state, &validated, &bounty_inputs).expect("valid transition");
        assert_eq!(
            result.next_state().player().bounty(),
            LaneBounty::new(50).unwrap()
        );
        let player_obs = observe_player(&result.next_state(), ObservationId::new(2)).observation();
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(player_obs.self_bounty(), LaneBounty::new(50).unwrap());
        assert_eq!(allied_obs.laner_bounty(), LaneBounty::new(50).unwrap());
        assert_eq!(
            result.debrief().bounty_earned(),
            LaneBounty::new(50).unwrap()
        );

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::BountyEarned { amount, .. } if *amount == LaneBounty::new(50).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::BountyChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneBounty::zero()
                && *after == LaneBounty::new(50).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &request, bounty_inputs)
            .expect("append bounty execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn bounty_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.bounty_earned = LaneBounty(101);

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::BountyOverflow { .. }
            ))
        ));
    }

#[test]
    fn level_is_bounded_and_default_initial() {
        assert_eq!(LaneLevel::initial().value(), 1);
        assert_eq!(LaneLevel::zero().value(), 0);
        assert_eq!(LaneLevel::new(18).unwrap().value(), 18);
        assert!(LaneLevel::new(19).is_err());
    }

#[test]
    fn level_gained_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().level(), LaneLevel::initial());

        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut level_inputs = inputs(1, 2, LaneWaveResult::Held);
        level_inputs.execution = level_inputs
            .execution
            .with_level_gained(LaneLevel::new(2).unwrap());

        let result = transition_lane(&state, &validated, &level_inputs).expect("valid transition");
        assert_eq!(
            result.next_state().player().level(),
            LaneLevel::new(3).unwrap()
        );
        let player_obs = observe_player(&result.next_state(), ObservationId::new(2)).observation();
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(player_obs.self_level(), LaneLevel::new(3).unwrap());
        assert_eq!(allied_obs.laner_level(), LaneLevel::new(3).unwrap());
        assert_eq!(result.debrief().level_gained(), LaneLevel::new(2).unwrap());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::LevelGained { amount, .. } if *amount == LaneLevel::new(2).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::LevelChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneLevel::initial()
                && *after == LaneLevel::new(3).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &request, level_inputs)
            .expect("append level execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn level_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.level_gained = LaneLevel(18);

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::LevelOverflow { .. }
            ))
        ));
    }

#[test]
    fn minion_kills_is_bounded_and_default_zero() {
        assert_eq!(LaneMinionKills::zero().value(), 0);
        assert_eq!(LaneMinionKills::new(200).unwrap().value(), 200);
        assert!(LaneMinionKills::new(201).is_err());
    }

#[test]
    fn minion_kills_gained_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().minion_kills(), LaneMinionKills::zero());

        let (receipt, req) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");
        let mut mk_inputs = inputs(1, 2, LaneWaveResult::Held);
        mk_inputs.execution = mk_inputs
            .execution
            .with_minion_kills_gained(LaneMinionKills::new(12).unwrap());

        let result = transition_lane(&state, &validated, &mk_inputs).expect("valid transition");
        assert_eq!(
            result.next_state().player().minion_kills(),
            LaneMinionKills::new(12).unwrap()
        );
        let player_obs = observe_player(&result.next_state(), ObservationId::new(2)).observation();
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(
            player_obs.self_minion_kills(),
            LaneMinionKills::new(12).unwrap()
        );
        assert_eq!(
            allied_obs.laner_minion_kills(),
            LaneMinionKills::new(12).unwrap()
        );
        assert_eq!(
            result.debrief().minion_kills_gained(),
            LaneMinionKills::new(12).unwrap()
        );

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::MinionKillsGained { amount, .. } if *amount == LaneMinionKills::new(12).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::MinionKillsChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneMinionKills::zero()
                && *after == LaneMinionKills::new(12).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &req, mk_inputs)
            .expect("append minion kills execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn minion_kills_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.minion_kills_gained = LaneMinionKills(201);

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::MinionKillsOverflow { .. }
            ))
        ));
    }

#[test]
    fn delayed_effects_are_bounded_and_default_empty() {
        let empty = LaneDelayedEffects::empty();
        assert_eq!(empty.count(), 0);
        assert!(empty.is_empty());
        let effect = LaneDelayedEffect::new(
            1,
            LaneDelayedEffectKind::SelfHealthRegen {
                amount: LaneHealth::new(2).unwrap(),
            },
        );
        assert_eq!(effect.delay_beats(), 1);
        assert_eq!(
            effect.kind(),
            LaneDelayedEffectKind::SelfHealthRegen {
                amount: LaneHealth::new(2).unwrap()
            }
        );

        let mut queue = LaneDelayedEffects::empty();
        for _ in 0..4 {
            assert!(queue.push(effect).is_ok());
        }
        assert_eq!(queue.count(), 4);
        assert!(queue.push(effect).is_err());
    }

#[test]
    fn delayed_effect_queues_ticks_resolves_and_replays() {
        let state = LaneSnapshot::initial(); // player health 8
        let (receipt, req) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        // Window 1: take 3 damage (health becomes 5) and queue a 1-beat health regen of +2
        let regen = LaneDelayedEffect::new(
            1,
            LaneDelayedEffectKind::SelfHealthRegen {
                amount: LaneHealth::new(2).unwrap(),
            },
        );
        let mut inputs_w1 = inputs(3, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_delayed_effect(regen);

        let res1 = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(res1.next_state().player().health(), LaneHealth::new(5).unwrap());
        assert_eq!(res1.next_state().delayed_effects().count(), 1);
        assert_eq!(res1.debrief().delayed_effects_queued(), 1);
        assert_eq!(res1.debrief().delayed_effects_resolved(), 0);
        assert!(res1.events().iter().any(|e| matches!(
            e,
            LaneEvent::DelayedEffectQueued { effect, .. } if *effect == regen
        )));
        assert!(res1.effects().iter().any(|e| matches!(
            e,
            LaneEffect::DelayedEffectQueued { effect, provenance, .. }
                if *effect == regen && provenance.timing() == LaneEffectTiming::Immediate
        )));

        // Window 2: transition from reopened state; delayed effect should resolve (health 5 -> 7)
        let s2 = reopen_lane_window(&res1).expect("reopen");
        let (rec2, req2) = request(&s2, LaneIntent::Contest);
        let val2 = validate_lane_request(&s2, &rec2, &req2).expect("valid 2");
        let inputs_w2 = inputs(0, 0, LaneWaveResult::Held);

        let res2 = transition_lane(&s2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(res2.next_state().player().health(), LaneHealth::new(7).unwrap());
        assert_eq!(res2.next_state().delayed_effects().count(), 0);
        assert_eq!(res2.debrief().delayed_effects_queued(), 0);
        assert_eq!(res2.debrief().delayed_effects_resolved(), 1);
        assert!(res2.events().iter().any(|e| matches!(
            e,
            LaneEvent::DelayedEffectResolved { effect, .. } if *effect == regen
        )));
        assert!(res2.effects().iter().any(|e| matches!(
            e,
            LaneEffect::DelayedEffectResolved { effect, provenance, .. }
                if *effect == regen
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Delayed
        )));

        let mut scenario_history = LaneScenarioHistory::new(state).expect("valid scenario history");
        scenario_history
            .append(&receipt, &req, inputs_w1)
            .expect("append 1");
        scenario_history
            .append(&rec2, &req2, inputs_w2)
            .expect("append 2");
        assert_eq!(
            scenario_history.verify_replay(),
            Ok(scenario_history.current_state())
        );
    }

#[test]
    fn delayed_effect_overflow_is_rejected() {
        let mut state = LaneSnapshot::initial();
        let regen = LaneDelayedEffect::new(
            2,
            LaneDelayedEffectKind::SelfHealthRegen {
                amount: LaneHealth::new(1).unwrap(),
            },
        );
        for _ in 0..4 {
            state.delayed_effects.push(regen).unwrap();
        }

        let (receipt, req) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");
        let mut overflow_inputs = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs.execution = overflow_inputs.execution.with_delayed_effect(regen);

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::DelayedEffectOverflow
            ))
        ));
    }

    #[test]
    fn shield_is_bounded_and_default_zero() {
        let player = PlayerLaneState::new(PLAYER_LANER, LaneHealth::new(8).unwrap(), LanePosition::Center);
        assert_eq!(player.shield(), LaneShield::zero());
        assert_eq!(player.shield().value(), 0);

        let valid = LaneShield::new(25).expect("valid shield");
        assert_eq!(valid.value(), 25);

        let overflow = LaneShield::new(MAX_LANE_SHIELD + 1);
        assert_eq!(
            overflow,
            Err(LaneBoundsError {
                value: MAX_LANE_SHIELD + 1,
                maximum: MAX_LANE_SHIELD,
            })
        );
    }

    #[test]
    fn shield_gained_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_shield(), LaneShield::zero());

        let allied_obs = observe_allied(&state, ObservationId::new(1));
        assert_eq!(allied_obs.observation().laner_shield(), LaneShield::zero());

        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let shield = LaneShield::new(15).expect("valid shield");
        let mut resolved_inputs = inputs(1, 1, LaneWaveResult::Held);
        resolved_inputs.execution = resolved_inputs.execution.with_shield_gained(shield);

        let result = transition_lane(&state, &validated, &resolved_inputs).expect("transition");
        let next = result.next_state();
        assert_eq!(next.player().shield(), shield);
        assert_eq!(result.debrief().shield_gained(), shield);

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::ShieldGained { actor, amount, .. }
                if *actor == PLAYER_LANER && *amount == shield
        )));

        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::ShieldChanged {
                actor,
                before,
                after,
                provenance,
                ..
            } if *actor == PLAYER_LANER
                && *before == LaneShield::zero()
                && *after == shield
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, resolved_inputs).expect("append");
        assert_eq!(history.verify_replay(), Ok(next));
    }

    #[test]
    fn shield_overflow_is_rejected() {
        let player = PlayerLaneState::new_with_absolute_state(
            PLAYER_LANER,
            LaneHealth::new(8).unwrap(),
            LaneMana::full(),
            LaneGold::zero(),
            LaneExperience::zero(),
            LaneCooldown::zero(),
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            LanePosition::Center,
        );
        let state = LaneSnapshot::new_with_window(
            M2_LANE_RULESET,
            Turn::new(1),
            LaneWindow::OneBeat,
            LanePhase::Open,
            player,
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(8).unwrap(),
                LanePosition::Center,
                OpponentPosture::Passive,
            ),
            WaveState::new(WavePressure::new(1).unwrap()),
            JungleThreatTruth::Absent,
            None,
        );

        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let shield = LaneShield::new(MAX_LANE_SHIELD).expect("max shield");
        let mut overflow_inputs = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs.execution = overflow_inputs.execution.with_shield_gained(shield);

        let overflow_inputs = overflow_inputs.with_mana_spent(LaneMana::zero());

        // First transition to get state with MAX_LANE_SHIELD
        let result = transition_lane(&state, &validated, &overflow_inputs).expect("transition 1");
        let full_shield_state = reopen_lane_window(&result).expect("reopen");

        let (rec2, req2) = request(&full_shield_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&full_shield_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_shield_gained(LaneShield::new(1).unwrap());

        assert!(matches!(
            transition_lane(&full_shield_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ShieldOverflow { .. }
            ))
        ));
    }

    #[test]
    fn ward_is_bounded_and_default_zero() {
        let player = PlayerLaneState::new(PLAYER_LANER, LaneHealth::new(8).unwrap(), LanePosition::Center);
        assert_eq!(player.ward(), LaneWard::zero());
        assert_eq!(player.ward().value(), 0);

        let valid = LaneWard::new(3).expect("valid ward");
        assert_eq!(valid.value(), 3);

        let overflow = LaneWard::new(MAX_LANE_WARD + 1);
        assert_eq!(
            overflow,
            Err(LaneBoundsError {
                value: MAX_LANE_WARD + 1,
                maximum: MAX_LANE_WARD,
            })
        );
    }

    #[test]
    fn ward_gained_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_ward(), LaneWard::zero());

        let allied_obs = observe_allied(&state, ObservationId::new(1));
        assert_eq!(allied_obs.observation().laner_ward(), LaneWard::zero());

        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let ward = LaneWard::new(2).expect("valid ward");
        let mut resolved_inputs = inputs(1, 1, LaneWaveResult::Held);
        resolved_inputs.execution = resolved_inputs.execution.with_ward_gained(ward);

        let result = transition_lane(&state, &validated, &resolved_inputs).expect("transition");
        let next = result.next_state();
        assert_eq!(next.player().ward(), ward);
        assert_eq!(result.debrief().ward_gained(), ward);

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::WardGained { actor, amount, .. }
                if *actor == PLAYER_LANER && *amount == ward
        )));

        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::WardChanged {
                actor,
                before,
                after,
                provenance,
                ..
            } if *actor == PLAYER_LANER
                && *before == LaneWard::zero()
                && *after == ward
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, resolved_inputs).expect("append");
        assert_eq!(history.verify_replay(), Ok(next));
    }

    #[test]
    fn ward_overflow_is_rejected() {
        let player = PlayerLaneState::new_with_absolute_state(
            PLAYER_LANER,
            LaneHealth::new(8).unwrap(),
            LaneMana::full(),
            LaneGold::zero(),
            LaneExperience::zero(),
            LaneCooldown::zero(),
            LaneBounty::zero(),
            LaneLevel::initial(),
            LaneMinionKills::zero(),
            LanePosition::Center,
        );
        let state = LaneSnapshot::new_with_window(
            M2_LANE_RULESET,
            Turn::new(1),
            LaneWindow::OneBeat,
            LanePhase::Open,
            player,
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(8).unwrap(),
                LanePosition::Center,
                OpponentPosture::Passive,
            ),
            WaveState::new(WavePressure::new(1).unwrap()),
            JungleThreatTruth::Absent,
            None,
        );

        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let ward = LaneWard::new(MAX_LANE_WARD).expect("max ward");
        let mut overflow_inputs = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs.execution = overflow_inputs.execution.with_ward_gained(ward);

        let overflow_inputs = overflow_inputs.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &overflow_inputs).expect("transition 1");
        let full_ward_state = reopen_lane_window(&result).expect("reopen");

        let (rec2, req2) = request(&full_ward_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&full_ward_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_ward_gained(LaneWard::new(1).unwrap());

        assert!(matches!(
            transition_lane(&full_ward_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::WardOverflow { .. }
            ))
        ));
    }

    #[test]
    fn potion_is_bounded_and_default_zero() {
        let player = PlayerLaneState::new(PLAYER_LANER, LaneHealth::new(8).unwrap(), LanePosition::Center);
        assert_eq!(player.potion(), LanePotion::zero());
        assert_eq!(player.potion().value(), 0);

        let valid = LanePotion::new(5).expect("valid potion");
        assert_eq!(valid.value(), 5);

        let err = LanePotion::new(6).unwrap_err();
        assert_eq!(err.value, 6);
        assert_eq!(err.maximum, MAX_LANE_POTION);
    }

    #[test]
    fn potion_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let potion_gained = LanePotion::new(3).expect("3 potions");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1
            .execution
            .with_potion_gained(potion_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().potion(), potion_gained);
        assert_eq!(result.debrief().potion_gained(), potion_gained);
        assert_eq!(result.debrief().potion_spent(), LanePotion::zero());
        assert_ne!(result.next_state().hash(), state.hash());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::PotionGained { amount, .. } if *amount == potion_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::PotionChanged { before, after, provenance, .. }
                if *before == LanePotion::zero()
                    && *after == potion_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let player_obs = observe_player(&state_w2, ObservationId::new(2));
        assert_eq!(player_obs.observation().self_potion(), potion_gained);
        let allied_obs = observe_allied(&state_w2, ObservationId::new(3));
        assert_eq!(allied_obs.observation().laner_potion(), potion_gained);

        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid");

        let potion_spent = LanePotion::new(2).expect("2 potions");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2
            .execution
            .with_potion_spent(potion_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().potion(), LanePotion::new(1).unwrap());
        assert_eq!(result2.debrief().potion_spent(), potion_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::PotionSpent { amount, .. } if *amount == potion_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn potion_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_potion_spent(LanePotion::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientPotion { .. }
            ))
        ));

        // Test overflow: gain 5 potions, then try to gain 1 more in next window
        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_potion_gained(LanePotion::new(MAX_LANE_POTION).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_potion_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_potion_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_potion_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_potion_gained(LanePotion::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_potion_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::PotionOverflow { .. }
            ))
        ));
    }

    #[test]
    fn elixir_is_bounded_and_default_zero() {
        assert_eq!(LaneElixir::zero().value(), 0);

        let valid = LaneElixir::new(5).expect("valid elixir");
        assert_eq!(valid.value(), 5);

        let err = LaneElixir::new(6).unwrap_err();
        assert_eq!(err.value, 6);
        assert_eq!(err.maximum, MAX_LANE_ELIXIR);
    }

    #[test]
    fn elixir_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let elixir_gained = LaneElixir::new(3).expect("3 elixirs");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1
            .execution
            .with_elixir_gained(elixir_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().elixir(), elixir_gained);
        assert_eq!(result.debrief().elixir_gained(), elixir_gained);
        assert_eq!(result.debrief().elixir_spent(), LaneElixir::zero());
        assert_ne!(result.next_state().hash(), state.hash());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::ElixirGained { amount, .. } if *amount == elixir_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::ElixirChanged { before, after, provenance, .. }
                if *before == LaneElixir::zero()
                    && *after == elixir_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let player_obs = observe_player(&state_w2, ObservationId::new(2));
        assert_eq!(player_obs.observation().self_elixir(), elixir_gained);
        let allied_obs = observe_allied(&state_w2, ObservationId::new(3));
        assert_eq!(allied_obs.observation().laner_elixir(), elixir_gained);

        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid");

        let elixir_spent = LaneElixir::new(2).expect("2 elixirs");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2
            .execution
            .with_elixir_spent(elixir_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().elixir(), LaneElixir::new(1).unwrap());
        assert_eq!(result2.debrief().elixir_spent(), elixir_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::ElixirSpent { amount, .. } if *amount == elixir_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn elixir_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_elixir_spent(LaneElixir::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientElixir { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_elixir_gained(LaneElixir::new(MAX_LANE_ELIXIR).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_elixir_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_elixir_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_elixir_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_elixir_gained(LaneElixir::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_elixir_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ElixirOverflow { .. }
            ))
        ));
    }

    #[test]
    fn trinket_is_bounded_and_default_zero() {
        assert_eq!(LaneTrinket::zero().value(), 0);
        assert_eq!(LaneTrinket::new(5).unwrap().value(), 5);
        assert!(LaneTrinket::new(6).is_err());

        let zero = LaneTrinket::zero();
        let gained = LaneTrinket::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneTrinket::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneTrinket::new(1).unwrap()),
            Some(LaneTrinket::new(2).unwrap())
        );
        assert_eq!(zero.subtract(gained), None);
    }

    #[test]
    fn trinket_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let trinket_gained = LaneTrinket::new(3).expect("3 trinkets");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1
            .execution
            .with_trinket_gained(trinket_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().trinket(), trinket_gained);
        assert_eq!(result.debrief().trinket_gained(), trinket_gained);
        assert_eq!(result.debrief().trinket_spent(), LaneTrinket::zero());
        assert_ne!(result.next_state().hash(), state.hash());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::TrinketGained { amount, .. } if *amount == trinket_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::TrinketChanged { before, after, provenance, .. }
                if *before == LaneTrinket::zero()
                    && *after == trinket_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let player_obs = observe_player(&state_w2, ObservationId::new(2));
        assert_eq!(player_obs.observation().self_trinket(), trinket_gained);
        let allied_obs = observe_allied(&state_w2, ObservationId::new(3));
        assert_eq!(allied_obs.observation().laner_trinket(), trinket_gained);

        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid");

        let trinket_spent = LaneTrinket::new(2).expect("2 trinkets");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2
            .execution
            .with_trinket_spent(trinket_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().trinket(), LaneTrinket::new(1).unwrap());
        assert_eq!(result2.debrief().trinket_spent(), trinket_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::TrinketSpent { amount, .. } if *amount == trinket_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn trinket_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_trinket_spent(LaneTrinket::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientTrinket { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_trinket_gained(LaneTrinket::new(MAX_LANE_TRINKET).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_trinket_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_trinket_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_trinket_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_trinket_gained(LaneTrinket::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_trinket_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::TrinketOverflow { .. }
            ))
        ));
    }

    #[test]
    fn relic_is_bounded_and_default_zero() {
        assert_eq!(LaneRelic::zero().value(), 0);
        assert_eq!(LaneRelic::new(5).unwrap().value(), 5);
        assert!(LaneRelic::new(6).is_err());

        let zero = LaneRelic::zero();
        let gained = LaneRelic::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneRelic::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneRelic::new(1).unwrap()),
            Some(LaneRelic::new(2).unwrap())
        );
        assert_eq!(zero.subtract(LaneRelic::new(1).unwrap()), None);

        let state = LaneSnapshot::initial();
        assert_eq!(state.player().relic(), LaneRelic::zero());
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_relic(), LaneRelic::zero());
        let allied = observe_allied(&state, ObservationId::new(2));
        assert_eq!(allied.observation().laner_relic(), LaneRelic::zero());
    }

    #[test]
    fn relic_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");

        let relic_gained = LaneRelic::new(3).expect("3 relics");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_relic_gained(relic_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().relic(), relic_gained);
        assert_eq!(result.debrief().relic_gained(), relic_gained);
        assert_eq!(result.debrief().relic_spent(), LaneRelic::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::RelicGained { amount, .. } if *amount == relic_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::RelicChanged { before, after, provenance, .. }
                if *before == LaneRelic::zero()
                    && *after == relic_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let relic_spent = LaneRelic::new(2).expect("2 relics");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_relic_spent(relic_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().relic(), LaneRelic::new(1).unwrap());
        assert_eq!(result2.debrief().relic_spent(), relic_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::RelicSpent { amount, .. } if *amount == relic_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn relic_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_relic_spent(LaneRelic::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientRelic { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_relic_gained(LaneRelic::new(MAX_LANE_RELIC).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_relic_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_relic_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_relic_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_relic_gained(LaneRelic::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_relic_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::RelicOverflow { .. }
            ))
        ));
    }

    #[test]
    fn charm_is_bounded_and_default_zero() {
        assert_eq!(LaneCharm::zero().value(), 0);
        assert_eq!(LaneCharm::new(5).unwrap().value(), 5);
        assert!(LaneCharm::new(6).is_err());

        let zero = LaneCharm::zero();
        let gained = LaneCharm::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneCharm::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneCharm::new(1).unwrap()),
            Some(LaneCharm::new(2).unwrap())
        );
        assert_eq!(zero.subtract(LaneCharm::new(1).unwrap()), None);

        let state = LaneSnapshot::initial();
        assert_eq!(state.player().charm(), LaneCharm::zero());
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_charm(), LaneCharm::zero());
        let allied = observe_allied(&state, ObservationId::new(2));
        assert_eq!(allied.observation().laner_charm(), LaneCharm::zero());
    }

    #[test]
    fn charm_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");

        let charm_gained = LaneCharm::new(3).expect("3 charms");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_charm_gained(charm_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().charm(), charm_gained);
        assert_eq!(result.debrief().charm_gained(), charm_gained);
        assert_eq!(result.debrief().charm_spent(), LaneCharm::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::CharmGained { amount, .. } if *amount == charm_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::CharmChanged { before, after, provenance, .. }
                if *before == LaneCharm::zero()
                    && *after == charm_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let charm_spent = LaneCharm::new(2).expect("2 charms");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_charm_spent(charm_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().charm(), LaneCharm::new(1).unwrap());
        assert_eq!(result2.debrief().charm_spent(), charm_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::CharmSpent { amount, .. } if *amount == charm_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn charm_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_charm_spent(LaneCharm::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientCharm { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_charm_gained(LaneCharm::new(MAX_LANE_CHARM).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_charm_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_charm_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_charm_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_charm_gained(LaneCharm::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_charm_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::CharmOverflow { .. }
            ))
        ));
    }

    #[test]
    fn scroll_is_bounded_and_default_zero() {
        assert_eq!(LaneScroll::zero().value(), 0);
        assert_eq!(LaneScroll::new(5).unwrap().value(), 5);
        assert!(LaneScroll::new(6).is_err());

        let zero = LaneScroll::zero();
        let gained = LaneScroll::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneScroll::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneScroll::new(1).unwrap()),
            Some(LaneScroll::new(2).unwrap())
        );
        assert_eq!(zero.subtract(LaneScroll::new(1).unwrap()), None);

        let state = LaneSnapshot::initial();
        assert_eq!(state.player().scroll(), LaneScroll::zero());
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_scroll(), LaneScroll::zero());
        let allied = observe_allied(&state, ObservationId::new(2));
        assert_eq!(allied.observation().laner_scroll(), LaneScroll::zero());
    }

    #[test]
    fn scroll_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");

        let scroll_gained = LaneScroll::new(3).expect("3 scrolls");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_scroll_gained(scroll_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().scroll(), scroll_gained);
        assert_eq!(result.debrief().scroll_gained(), scroll_gained);
        assert_eq!(result.debrief().scroll_spent(), LaneScroll::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::ScrollGained { amount, .. } if *amount == scroll_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::ScrollChanged { before, after, provenance, .. }
                if *before == LaneScroll::zero()
                    && *after == scroll_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let scroll_spent = LaneScroll::new(2).expect("2 scrolls");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_scroll_spent(scroll_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().scroll(), LaneScroll::new(1).unwrap());
        assert_eq!(result2.debrief().scroll_spent(), scroll_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::ScrollSpent { amount, .. } if *amount == scroll_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn scroll_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_scroll_spent(LaneScroll::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientScroll { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_scroll_gained(LaneScroll::new(MAX_LANE_SCROLL).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_scroll_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_scroll_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_scroll_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_scroll_gained(LaneScroll::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_scroll_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ScrollOverflow { .. }
            ))
        ));
    }

    #[test]
    fn tome_is_bounded_and_default_zero() {
        assert_eq!(LaneTome::zero().value(), 0);
        assert_eq!(LaneTome::new(5).unwrap().value(), 5);
        assert!(LaneTome::new(6).is_err());

        let zero = LaneTome::zero();
        let gained = LaneTome::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneTome::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneTome::new(1).unwrap()),
            Some(LaneTome::new(2).unwrap())
        );
        assert_eq!(zero.subtract(LaneTome::new(1).unwrap()), None);

        let state = LaneSnapshot::initial();
        assert_eq!(state.player().tome(), LaneTome::zero());
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_tome(), LaneTome::zero());
        let allied = observe_allied(&state, ObservationId::new(2));
        assert_eq!(allied.observation().laner_tome(), LaneTome::zero());
    }

    #[test]
    fn tome_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");

        let tome_gained = LaneTome::new(3).expect("3 tomes");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_tome_gained(tome_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().tome(), tome_gained);
        assert_eq!(result.debrief().tome_gained(), tome_gained);
        assert_eq!(result.debrief().tome_spent(), LaneTome::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::TomeGained { amount, .. } if *amount == tome_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::TomeChanged { before, after, provenance, .. }
                if *before == LaneTome::zero()
                    && *after == tome_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let tome_spent = LaneTome::new(2).expect("2 tomes");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_tome_spent(tome_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().tome(), LaneTome::new(1).unwrap());
        assert_eq!(result2.debrief().tome_spent(), tome_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::TomeSpent { amount, .. } if *amount == tome_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn tome_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_tome_spent(LaneTome::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientTome { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_tome_gained(LaneTome::new(MAX_LANE_TOME).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_tome_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_tome_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_tome_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_tome_gained(LaneTome::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_tome_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::TomeOverflow { .. }
            ))
        ));
    }

    #[test]
    fn rune_is_bounded_and_default_zero() {
        assert_eq!(LaneRune::zero().value(), 0);
        assert_eq!(LaneRune::new(5).unwrap().value(), 5);
        assert!(LaneRune::new(6).is_err());

        let zero = LaneRune::zero();
        let gained = LaneRune::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneRune::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneRune::new(1).unwrap()),
            Some(LaneRune::new(2).unwrap())
        );
        assert_eq!(zero.subtract(LaneRune::new(1).unwrap()), None);

        let state = LaneSnapshot::initial();
        assert_eq!(state.player().rune(), LaneRune::zero());
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_rune(), LaneRune::zero());
        let allied = observe_allied(&state, ObservationId::new(2));
        assert_eq!(allied.observation().laner_rune(), LaneRune::zero());
    }

    #[test]
    fn rune_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");

        let rune_gained = LaneRune::new(3).expect("3 runes");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_rune_gained(rune_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().rune(), rune_gained);
        assert_eq!(result.debrief().rune_gained(), rune_gained);
        assert_eq!(result.debrief().rune_spent(), LaneRune::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::RuneGained { amount, .. } if *amount == rune_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::RuneChanged { before, after, provenance, .. }
                if *before == LaneRune::zero()
                    && *after == rune_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let rune_spent = LaneRune::new(2).expect("2 runes");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_rune_spent(rune_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().rune(), LaneRune::new(1).unwrap());
        assert_eq!(result2.debrief().rune_spent(), rune_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::RuneSpent { amount, .. } if *amount == rune_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn rune_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_rune_spent(LaneRune::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientRune { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_rune_gained(LaneRune::new(MAX_LANE_RUNE).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_rune_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_rune_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_rune_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_rune_gained(LaneRune::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_rune_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::RuneOverflow { .. }
            ))
        ));
    }

    #[test]
    fn sigil_is_bounded_and_default_zero() {
        assert_eq!(LaneSigil::zero().value(), 0);
        assert_eq!(LaneSigil::new(5).unwrap().value(), 5);
        assert!(LaneSigil::new(6).is_err());

        let zero = LaneSigil::zero();
        let gained = LaneSigil::new(3).unwrap();
        assert_eq!(zero.add(gained), Some(gained));
        assert_eq!(gained.add(LaneSigil::new(3).unwrap()), None);
        assert_eq!(
            gained.subtract(LaneSigil::new(1).unwrap()),
            Some(LaneSigil::new(2).unwrap())
        );
        assert_eq!(zero.subtract(LaneSigil::new(1).unwrap()), None);

        let state = LaneSnapshot::initial();
        assert_eq!(state.player().sigil(), LaneSigil::zero());
        let obs = observe_player(&state, ObservationId::new(1));
        assert_eq!(obs.observation().self_sigil(), LaneSigil::zero());
        let allied = observe_allied(&state, ObservationId::new(2));
        assert_eq!(allied.observation().laner_sigil(), LaneSigil::zero());
    }

    #[test]
    fn sigil_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");

        let sigil_gained = LaneSigil::new(3).expect("3 sigils");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_sigil_gained(sigil_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().sigil(), sigil_gained);
        assert_eq!(result.debrief().sigil_gained(), sigil_gained);
        assert_eq!(result.debrief().sigil_spent(), LaneSigil::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::SigilGained { amount, .. } if *amount == sigil_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::SigilChanged { before, after, provenance, .. }
                if *before == LaneSigil::zero()
                    && *after == sigil_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let sigil_spent = LaneSigil::new(2).expect("2 sigils");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_sigil_spent(sigil_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().sigil(), LaneSigil::new(1).unwrap());
        assert_eq!(result2.debrief().sigil_spent(), sigil_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::SigilSpent { amount, .. } if *amount == sigil_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn sigil_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_sigil_spent(LaneSigil::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientSigil { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_sigil_gained(LaneSigil::new(MAX_LANE_SIGIL).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_sigil_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_sigil_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_sigil_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_sigil_gained(LaneSigil::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_sigil_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::SigilOverflow { .. }
            ))
        ));
    }

    #[test]
    fn talisman_is_bounded_and_default_zero() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().talisman(), LaneTalisman::zero());
        assert_eq!(state.player().talisman().value(), 0);

        let obs = observe_player(&state, ObservationId::new(1)).observation();
        assert_eq!(obs.self_talisman(), LaneTalisman::zero());

        let allied = observe_allied(&state, ObservationId::new(1)).observation();
        assert_eq!(allied.laner_talisman(), LaneTalisman::zero());

        assert!(LaneTalisman::new(MAX_LANE_TALISMAN).is_ok());
        assert!(LaneTalisman::new(MAX_LANE_TALISMAN + 1).is_err());
    }

    #[test]
    fn talisman_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let talisman_gained = LaneTalisman::new(3).expect("3 talismans");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_talisman_gained(talisman_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().talisman(), talisman_gained);
        assert_eq!(result.debrief().talisman_gained(), talisman_gained);
        assert_eq!(result.debrief().talisman_spent(), LaneTalisman::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::TalismanGained { amount, .. } if *amount == talisman_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::TalismanChanged { before, after, provenance, .. }
                if *before == LaneTalisman::zero()
                    && *after == talisman_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let talisman_spent = LaneTalisman::new(2).expect("2 talismans");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_talisman_spent(talisman_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().talisman(), LaneTalisman::new(1).unwrap());
        assert_eq!(result2.debrief().talisman_spent(), talisman_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::TalismanSpent { amount, .. } if *amount == talisman_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn talisman_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_talisman_spent(LaneTalisman::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientTalisman { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_talisman_gained(LaneTalisman::new(MAX_LANE_TALISMAN).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_talisman_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_talisman_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_talisman_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_talisman_gained(LaneTalisman::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_talisman_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::TalismanOverflow { .. }
            ))
        ));
    }

    #[test]
    fn amulet_is_bounded_and_default_zero() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().amulet(), LaneAmulet::zero());
        assert_eq!(state.player().amulet().value(), 0);

        let obs = observe_player(&state, ObservationId::new(1)).observation();
        assert_eq!(obs.self_amulet(), LaneAmulet::zero());

        let allied = observe_allied(&state, ObservationId::new(1)).observation();
        assert_eq!(allied.laner_amulet(), LaneAmulet::zero());

        assert!(LaneAmulet::new(MAX_LANE_AMULET).is_ok());
        assert!(LaneAmulet::new(MAX_LANE_AMULET + 1).is_err());
    }

    #[test]
    fn amulet_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let amulet_gained = LaneAmulet::new(3).expect("3 amulets");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_amulet_gained(amulet_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().amulet(), amulet_gained);
        assert_eq!(result.debrief().amulet_gained(), amulet_gained);
        assert_eq!(result.debrief().amulet_spent(), LaneAmulet::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::AmuletGained { amount, .. } if *amount == amulet_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::AmuletChanged { before, after, provenance, .. }
                if *before == LaneAmulet::zero()
                    && *after == amulet_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let amulet_spent = LaneAmulet::new(2).expect("2 amulets");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_amulet_spent(amulet_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().amulet(), LaneAmulet::new(1).unwrap());
        assert_eq!(result2.debrief().amulet_spent(), amulet_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::AmuletSpent { amount, .. } if *amount == amulet_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn amulet_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_amulet_spent(LaneAmulet::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientAmulet { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_amulet_gained(LaneAmulet::new(MAX_LANE_AMULET).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_amulet_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_amulet_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_amulet_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_amulet_gained(LaneAmulet::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_amulet_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::AmuletOverflow { .. }
            ))
        ));
    }

    #[test]
    fn phial_is_bounded_and_default_zero() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().phial(), LanePhial::zero());
        assert_eq!(state.player().phial().value(), 0);

        let obs = observe_player(&state, ObservationId::new(1)).observation();
        assert_eq!(obs.self_phial(), LanePhial::zero());

        let allied = observe_allied(&state, ObservationId::new(1)).observation();
        assert_eq!(allied.laner_phial(), LanePhial::zero());

        assert!(LanePhial::new(MAX_LANE_PHIAL).is_ok());
        assert!(LanePhial::new(MAX_LANE_PHIAL + 1).is_err());
    }

    #[test]
    fn phial_gained_and_spent_are_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let phial_gained = LanePhial::new(3).expect("3 phials");
        let mut inputs_w1 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w1.execution = inputs_w1.execution.with_phial_gained(phial_gained);
        let inputs_w1 = inputs_w1.with_mana_spent(LaneMana::zero());

        let result = transition_lane(&state, &validated, &inputs_w1).expect("transition 1");
        assert_eq!(result.next_state().player().phial(), phial_gained);
        assert_eq!(result.debrief().phial_gained(), phial_gained);
        assert_eq!(result.debrief().phial_spent(), LanePhial::zero());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::PhialGained { amount, .. } if *amount == phial_gained
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::PhialChanged { before, after, provenance, .. }
                if *before == LanePhial::zero()
                    && *after == phial_gained
                    && provenance.relation() == LaneEffectRelation::Direct
                    && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let state_w2 = reopen_lane_window(&result).expect("reopen");
        let (rec2, req2) = request(&state_w2, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&state_w2, &rec2, &req2).expect("valid 2");

        let phial_spent = LanePhial::new(2).expect("2 phials");
        let mut inputs_w2 = inputs(1, 1, LaneWaveResult::Held);
        inputs_w2.execution = inputs_w2.execution.with_phial_spent(phial_spent);
        let inputs_w2 = inputs_w2.with_mana_spent(LaneMana::zero());

        let result2 = transition_lane(&state_w2, &val2, &inputs_w2).expect("transition 2");
        assert_eq!(result2.next_state().player().phial(), LanePhial::new(1).unwrap());
        assert_eq!(result2.debrief().phial_spent(), phial_spent);

        assert!(result2.events().iter().any(|e| matches!(
            e,
            LaneEvent::PhialSpent { amount, .. } if *amount == phial_spent
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history.append(&receipt, &req, inputs_w1).expect("append 1");
        assert_eq!(history.verify_replay(), Ok(result.next_state()));
    }

    #[test]
    fn phial_overflow_and_insufficient_are_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, req) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &req).expect("valid");

        let mut insufficient_inputs = inputs(1, 1, LaneWaveResult::Held);
        insufficient_inputs.execution = insufficient_inputs
            .execution
            .with_phial_spent(LanePhial::new(1).unwrap());
        let insufficient_inputs = insufficient_inputs.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&state, &validated, &insufficient_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::InsufficientPhial { .. }
            ))
        ));

        let mut gain_max_inputs = inputs(1, 1, LaneWaveResult::Held);
        gain_max_inputs.execution = gain_max_inputs
            .execution
            .with_phial_gained(LanePhial::new(MAX_LANE_PHIAL).unwrap());
        let gain_max_inputs = gain_max_inputs.with_mana_spent(LaneMana::zero());

        let res1 = transition_lane(&state, &validated, &gain_max_inputs).expect("transition max");
        let max_phial_state = reopen_lane_window(&res1).expect("reopen");

        let (rec2, req2) = request(&max_phial_state, LaneIntent::Stabilize);
        let val2 = validate_lane_request(&max_phial_state, &rec2, &req2).expect("valid");

        let mut overflow_inputs_2 = inputs(1, 1, LaneWaveResult::Held);
        overflow_inputs_2.execution = overflow_inputs_2
            .execution
            .with_phial_gained(LanePhial::new(1).unwrap());
        let overflow_inputs_2 = overflow_inputs_2.with_mana_spent(LaneMana::zero());

        assert!(matches!(
            transition_lane(&max_phial_state, &val2, &overflow_inputs_2),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::PhialOverflow { .. }
            ))
        ));
    }




