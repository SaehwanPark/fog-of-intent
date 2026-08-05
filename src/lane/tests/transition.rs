#[test]
    fn two_beat_window_closes_on_commit_and_replays_with_a_distinct_hash() {
        let state = two_beat_state();
        assert_eq!(state.window(), LaneWindow::TwoBeats);
        assert!(state.window().closes_on_commit());
        assert_ne!(state.hash(), LaneSnapshot::initial().hash());

        let player_receipt = observe_player(&state, ObservationId::new(9));
        assert_eq!(player_receipt.observation().window(), LaneWindow::TwoBeats);
        let allied_receipt = observe_allied(&state, ObservationId::new(9));
        assert_eq!(allied_receipt.observation().window(), LaneWindow::TwoBeats);
        let proposal = scripted_allied_proposal(allied_receipt.observation(), trace(3, 3))
            .expect("allied policy supports the bounded longer window");
        assert_eq!(
            proposal.candidates().map(AlliedCandidate::intent),
            [LaneIntent::Stabilize, LaneIntent::Contest]
        );

        let request =
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        let result = history
            .append(
                &player_receipt,
                &request,
                inputs(0, 1, LaneWaveResult::Held),
            )
            .expect("two-beat transition");
        assert_eq!(result.next_state().turn(), Turn::new(2));
        assert_eq!(result.next_state().window(), LaneWindow::TwoBeats);
        assert_eq!(result.next_state().phase(), LanePhase::Resolved);
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn legal_unfavorable_contest_activates_fallback() {
        let state = LaneSnapshot::initial();
        let (receipt, contest_request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &contest_request).expect("valid");
        let result = transition_lane(&state, &validated, &inputs(3, 0, LaneWaveResult::Lost))
            .expect("execution is legal");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert!(result.debrief().fallback_activated());
        assert!(
            result
                .events()
                .iter()
                .any(|event| { matches!(event, LaneEvent::FallbackActivated { .. }) })
        );
        assert_eq!(
            result
                .effects()
                .iter()
                .find_map(|effect| match effect {
                    LaneEffect::PositionChanged { provenance, .. } => Some(*provenance),
                    _ => None,
                })
                .expect("fallback position effect"),
            LaneEffectProvenance::indirect_immediate()
        );
    }

#[test]
    fn explicit_effects_are_direct_immediate_and_have_no_delayed_emission() {
        let state = LaneSnapshot::initial();
        let (receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &stabilize_request).expect("valid");
        let result = transition_lane(&state, &validated, &inputs(1, 1, LaneWaveResult::Advanced))
            .expect("transition");
        assert_eq!(result.effects().len(), 6);
        assert!(result.effects().iter().all(|effect| {
            let provenance = effect.provenance();
            provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        }));
        assert!(
            result
                .effects()
                .iter()
                .all(|effect| { effect.provenance().timing() != LaneEffectTiming::Delayed })
        );
    }

#[test]
    fn contest_mana_spend_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, contest_request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &contest_request).expect("valid");
        let spent = LaneMana::new(1).expect("bounded spend");
        let resolved_inputs = inputs(0, 1, LaneWaveResult::Advanced).with_mana_spent(spent);
        let result =
            transition_lane(&state, &validated, &resolved_inputs).expect("contest spend is valid");
        assert_eq!(
            result.next_state().player().mana(),
            LaneMana::new(5).unwrap()
        );
        assert_eq!(result.debrief().mana_spent(), spent);
        assert!(result.events().iter().any(|event| matches!(
            event,
            LaneEvent::ManaSpent {
                actor: PLAYER_LANER,
                amount,
                ..
            } if *amount == spent
        )));
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::ManaChanged {
                actor: PLAYER_LANER,
                before,
                after,
                cause: LaneEffectCause::Execution(_),
                provenance,
            } if *before == LaneMana::full()
                && *after == LaneMana::new(5).unwrap()
                && *provenance == LaneEffectProvenance::direct_immediate()
        )));
        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &contest_request, resolved_inputs)
            .expect("append");
        assert_eq!(
            history.verify_replay().expect("replay"),
            result.next_state()
        );
    }

#[test]
    fn mana_spend_rejects_wrong_intent_and_insufficient_resource() {
        let state = LaneSnapshot::initial();
        let (stabilize_receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
        let stabilize =
            validate_lane_request(&state, &stabilize_receipt, &stabilize_request).expect("valid");
        let spent = LaneMana::new(1).expect("bounded spend");
        assert_eq!(
            transition_lane(
                &state,
                &stabilize,
                &inputs(0, 0, LaneWaveResult::Held).with_mana_spent(spent),
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ManaSpentWithoutContest {
                    intent: LaneIntent::Stabilize,
                    spent,
                }
            ))
        );

        let empty_player = PlayerLaneState::new_with_mana(
            state.player().id(),
            state.player().health(),
            LaneMana::zero(),
            state.player().position(),
        );
        let empty = LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            state.phase(),
            empty_player,
            state.opponent(),
            state.wave(),
            state.jungle_threat(),
            state.terminal_outcome(),
        );
        let (receipt, request) = request(&empty, LaneIntent::Contest);
        let validated = validate_lane_request(&empty, &receipt, &request).expect("valid");
        assert_eq!(
            transition_lane(
                &empty,
                &validated,
                &inputs(0, 0, LaneWaveResult::Held).with_mana_spent(spent),
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ManaExceedsAvailable {
                    spent,
                    available: LaneMana::zero(),
                }
            ))
        );
    }

#[test]
    fn invalid_requests_fail_before_transition() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let wrong_actor =
            LaneIntentRequest::new(ActorId::new(8), request.observation_id(), request.intent());
        assert!(matches!(
            validate_lane_request(&state, &receipt, &wrong_actor),
            Err(LaneValidationError::WrongActor { .. })
        ));
        let wrong_turn = LaneIntentCommand::new(
            PLAYER_LANER,
            Turn::new(1),
            M2_LANE_RULESET,
            request.observation_id(),
            state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&state, &receipt, &wrong_turn),
            Err(LaneValidationError::WrongTurn {
                expected: state.turn(),
                actual: Turn::new(1),
            })
        );
        let wrong_ruleset = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            RulesetId::new(99),
            request.observation_id(),
            state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&state, &receipt, &wrong_ruleset),
            Err(LaneValidationError::WrongRuleset {
                expected: M2_LANE_RULESET,
                actual: RulesetId::new(99),
            })
        );
        let stale_hash = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            M2_LANE_RULESET,
            request.observation_id(),
            StateHash::from_raw(0),
            request.intent(),
        );
        assert!(matches!(
            validate_lane_command(&state, &receipt, &stale_hash),
            Err(LaneValidationError::StateHashMismatch { .. })
        ));
        let both_wrong = LaneIntentCommand::new(
            PLAYER_LANER,
            Turn::new(1),
            RulesetId::new(99),
            request.observation_id(),
            state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&state, &receipt, &both_wrong),
            Err(LaneValidationError::WrongTurn {
                expected: state.turn(),
                actual: Turn::new(1),
            })
        );
        let invalid_state = LaneSnapshot::new(
            M2_LANE_RULESET,
            state.turn(),
            LanePhase::Open,
            PlayerLaneState::new(
                ActorId::new(8),
                state.player().health(),
                state.player().position(),
            ),
            state.opponent(),
            state.wave(),
            state.jungle_threat(),
            None,
        );
        let invalid_receipt = observe_player(&invalid_state, request.observation_id());
        let invalid_command = LaneIntentCommand::new(
            PLAYER_LANER,
            invalid_state.turn(),
            M2_LANE_RULESET,
            request.observation_id(),
            invalid_state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&invalid_state, &invalid_receipt, &invalid_command),
            Err(LaneValidationError::InvalidState)
        );
        assert!(matches!(
            LaneHistory::new(invalid_state),
            Err(LaneHistoryError::InvalidInitialState)
        ));
        let resolved = transition_lane(
            &state,
            &validate_lane_request(&state, &receipt, &request).expect("valid"),
            &inputs(0, 0, LaneWaveResult::Held),
        )
        .expect("valid transition");
        assert_eq!(resolved.next_state().phase(), LanePhase::Resolved);
        let resolved_receipt = observe_player(&resolved.next_state(), ObservationId::new(9));
        assert_eq!(
            validate_lane_request(&resolved.next_state(), &resolved_receipt, &request),
            Err(LaneValidationError::WindowAlreadyResolved)
        );
    }

#[test]
    fn malformed_execution_does_not_change_state() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let too_much = inputs(9, 0, LaneWaveResult::Held);
        assert!(matches!(
            transition_lane(&state, &validated, &too_much),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::SelfDamageExceedsHealth { .. }
            ))
        ));
        assert_eq!(state.phase(), LanePhase::Open);
        assert_eq!(state.terminal_outcome(), None);
    }

#[test]
    fn forced_out_and_wave_boundaries_remain_explicit() {
        let state = LaneSnapshot::initial();
        let (receipt, contest_request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &contest_request).expect("valid");
        let forced_out = transition_lane(&state, &validated, &inputs(8, 0, LaneWaveResult::Held))
            .expect("damage reaches zero health");
        assert_eq!(forced_out.outcome(), LaneOutcome::ForcedOut);
        assert_eq!(
            forced_out.next_state().player().health(),
            LaneHealth::zero()
        );

        let at_zero = LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            WaveState::new(WavePressure::new(0).expect("bounded")),
            state.jungle_threat(),
            None,
        );
        let (zero_receipt, zero_request) = request(&at_zero, LaneIntent::Contest);
        let zero_validated =
            validate_lane_request(&at_zero, &zero_receipt, &zero_request).expect("valid");
        assert_eq!(
            transition_lane(
                &at_zero,
                &zero_validated,
                &inputs(0, 0, LaneWaveResult::Lost)
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::WaveUnderflow {
                    pressure: WavePressure(0)
                }
            ))
        );

        let at_max = LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            WaveState::new(WavePressure::new(3).expect("bounded")),
            state.jungle_threat(),
            None,
        );
        let (max_receipt, max_request) = request(&at_max, LaneIntent::Contest);
        let max_validated =
            validate_lane_request(&at_max, &max_receipt, &max_request).expect("valid");
        assert!(matches!(
            transition_lane(
                &at_max,
                &max_validated,
                &inputs(0, 0, LaneWaveResult::Advanced)
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::WaveOverflow { .. }
            ))
        ));
    }

#[test]
    fn identical_inputs_and_neutral_stream_changes_are_deterministic() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let first_inputs = inputs(1, 2, LaneWaveResult::Advanced);
        let second_inputs = LaneResolvedInputs::new(
            trace(101, 101),
            trace(102, 102),
            trace(103, 103),
            trace(104, 104),
            first_inputs.execution(),
        );
        let first = transition_lane(&state, &validated, &first_inputs).expect("transition");
        let second = transition_lane(&state, &validated, &second_inputs).expect("transition");
        assert_eq!(first, second);
        assert_eq!(first.state_hash(), first.next_state().hash());
    }

#[test]
    fn mana_is_bounded_visible_and_binds_non_full_hashes_and_policy_inputs() {
        let full = LaneSnapshot::initial();
        let reduced_player = PlayerLaneState::new_with_mana(
            full.player().id(),
            full.player().health(),
            LaneMana::new(4).expect("bounded mana"),
            full.player().position(),
        );
        let reduced = LaneSnapshot::new(
            full.ruleset(),
            full.turn(),
            full.phase(),
            reduced_player,
            full.opponent(),
            full.wave(),
            full.jungle_threat(),
            full.terminal_outcome(),
        );
        assert_eq!(
            observe_player(&full, ObservationId::new(1))
                .observation()
                .self_mana(),
            LaneMana::full()
        );
        assert_eq!(
            observe_player(&reduced, ObservationId::new(1))
                .observation()
                .self_mana(),
            LaneMana::new(4).expect("bounded mana")
        );
        let full_allied = observe_allied(&full, ObservationId::new(1)).observation();
        let reduced_allied = observe_allied(&reduced, ObservationId::new(1)).observation();
        assert_eq!(full_allied.laner_mana(), LaneMana::full());
        assert_eq!(
            reduced_allied.laner_mana(),
            LaneMana::new(4).expect("bounded mana")
        );
        assert_ne!(full.hash(), reduced.hash());
        assert_ne!(
            allied_input_identity(full_allied, trace(3, 3)).visible_digest(),
            allied_input_identity(reduced_allied, trace(3, 3)).visible_digest()
        );
    }
