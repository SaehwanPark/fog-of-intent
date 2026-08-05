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

