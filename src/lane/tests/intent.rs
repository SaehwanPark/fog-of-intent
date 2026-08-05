#[test]
    fn both_intents_are_legal_and_produce_distinct_positions() {
        let state = LaneSnapshot::initial();
        let (stabilize_receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
        let stabilize = validate_lane_request(&state, &stabilize_receipt, &stabilize_request)
            .expect("stabilize is legal");
        let stable_result =
            transition_lane(&state, &stabilize, &inputs(0, 1, LaneWaveResult::Held))
                .expect("stabilize transition");
        assert_eq!(stable_result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            stable_result.next_state().player().position(),
            LanePosition::NearTower
        );

        let (contest_receipt, contest_request) = request(&state, LaneIntent::Contest);
        let contest = validate_lane_request(&state, &contest_receipt, &contest_request)
            .expect("contest is legal");
        let contest_result =
            transition_lane(&state, &contest, &inputs(0, 1, LaneWaveResult::Advanced))
                .expect("contest transition");
        assert_eq!(contest_result.outcome(), LaneOutcome::HeldSpace);
        assert_eq!(
            contest_result.next_state().player().position(),
            LanePosition::Center
        );
    }

#[test]
    fn recall_can_be_unfavorable_without_becoming_a_fallback() {
        let state = LaneSnapshot::initial();
        let (receipt, recall_request) = request(&state, LaneIntent::Recall);
        let validated =
            validate_lane_request(&state, &receipt, &recall_request).expect("recall is legal");
        let result = transition_lane(&state, &validated, &inputs(8, 0, LaneWaveResult::Held))
            .expect("fatal recall execution remains legal");
        assert_eq!(result.outcome(), LaneOutcome::ForcedOut);
        assert_eq!(result.debrief().intent(), LaneIntent::Recall);
        assert!(!result.debrief().fallback_activated());
        assert!(
            !result
                .events()
                .iter()
                .any(|event| matches!(event, LaneEvent::FallbackActivated { .. }))
        );
    }

#[test]
    fn recall_is_player_legal_but_not_an_allied_policy_candidate() {
        let state = LaneSnapshot::initial();
        let player_observation = observe_player(&state, ObservationId::new(9)).observation();
        assert_eq!(
            player_observation.available_intents(),
            [
                LaneIntent::Stabilize,
                LaneIntent::Contest,
                LaneIntent::Yield,
                LaneIntent::Recall
            ]
        );
        let allied_observation = observe_allied(&state, ObservationId::new(9)).observation();
        let proposal = scripted_allied_proposal(allied_observation, trace(3, 3))
            .expect("allied policy should accept its observation");
        assert_eq!(
            proposal.candidates().map(AlliedCandidate::intent),
            [LaneIntent::Stabilize, LaneIntent::Contest]
        );

        let (receipt, recall_request) = request(&state, LaneIntent::Recall);
        let validated = validate_lane_request(&state, &receipt, &recall_request)
            .expect("recall is legal for the player");
        let result = transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held))
            .expect("recall transition");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            result.next_state().player().position(),
            LanePosition::NearTower
        );
        assert_eq!(result.debrief().intent(), LaneIntent::Recall);
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::PositionChanged {
                cause: LaneEffectCause::Intent,
                ..
            }
        )));
    }

#[test]
    fn recall_replays_and_preserves_objective_attribution() {
        let state = LaneSnapshot::initial();
        let (receipt, recall_request) = request(&state, LaneIntent::Recall);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(
                &receipt,
                &recall_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("recall append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("recall objective review");
        assert_eq!(review.review().intent(), LaneIntent::Recall);
        assert_eq!(review.review().report().intent(), LaneIntent::Recall);
        review
            .verify_lane(record)
            .expect("replayable objective review");
    }

#[test]
    fn recall_requires_the_current_observation_to_advertise_it() {
        let state = LaneSnapshot::initial();
        let (mut receipt, recall_request) = request(&state, LaneIntent::Recall);
        receipt.observation.available_intents = [
            LaneIntent::Stabilize,
            LaneIntent::Contest,
            LaneIntent::Stabilize,
            LaneIntent::Stabilize,
        ];
        assert_eq!(
            validate_lane_request(&state, &receipt, &recall_request),
            Err(LaneValidationError::UnsupportedIntent)
        );

        let (mut stale_receipt, stale_request) = request(&state, LaneIntent::Recall);
        stale_receipt.source_state_hash = StateHash::from_raw(0);
        assert!(matches!(
            validate_lane_request(&state, &stale_receipt, &stale_request),
            Err(LaneValidationError::StaleObservation)
        ));

        let (current_receipt, current_request) = request(&state, LaneIntent::Recall);
        let validated =
            validate_lane_request(&state, &current_receipt, &current_request).expect("valid");
        let resolved = transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held))
            .expect("transition");
        let resolved_receipt = observe_player(&resolved.next_state(), ObservationId::new(9));
        assert_eq!(
            validate_lane_request(&resolved.next_state(), &resolved_receipt, &current_request),
            Err(LaneValidationError::WindowAlreadyResolved)
        );
    }

#[test]
    fn withdraw_replays_and_preserves_objective_attribution() {
        let state = river_side_state();
        let (receipt, withdraw_request) = request(&state, LaneIntent::Withdraw);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(
                &receipt,
                &withdraw_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("withdraw append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("withdraw objective review");
        assert_eq!(review.review().intent(), LaneIntent::Withdraw);
        review
            .verify_lane(record)
            .expect("replayable objective review");
    }

#[test]
    fn withdraw_requires_current_last_known_threat_and_preserves_explicit_inputs() {
        let unknown = LaneSnapshot::initial();
        let (unknown_receipt, unknown_request) = request(&unknown, LaneIntent::Withdraw);
        assert_eq!(
            validate_lane_request(&unknown, &unknown_receipt, &unknown_request),
            Err(LaneValidationError::UnsupportedIntent)
        );

        let river_side = river_side_state();
        let (mut stale_receipt, stale_request) = request(&river_side, LaneIntent::Withdraw);
        stale_receipt.source_state_hash = StateHash::from_raw(0);
        assert!(matches!(
            validate_lane_request(&river_side, &stale_receipt, &stale_request),
            Err(LaneValidationError::StaleObservation)
        ));

        let (receipt, withdraw_request) = request(&river_side, LaneIntent::Withdraw);
        let validated = validate_lane_request(&river_side, &receipt, &withdraw_request)
            .expect("current last-known report authorizes withdraw");
        let result = transition_lane(&river_side, &validated, &inputs(1, 2, LaneWaveResult::Lost))
            .expect("withdraw transition");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            result.next_state().player().position(),
            LanePosition::NearTower
        );
        assert_eq!(
            result.next_state().player().health(),
            LaneHealth::new(7).unwrap()
        );
        assert_eq!(
            result.next_state().wave().pressure(),
            WavePressure::new(0).unwrap()
        );
        assert_eq!(result.debrief().intent(), LaneIntent::Withdraw);
        assert!(!result.debrief().fallback_activated());
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::PositionChanged {
                cause: LaneEffectCause::Intent,
                ..
            }
        )));
    }

#[test]
    fn yield_is_player_legal_and_resolves_to_near_tower() {
        let state = LaneSnapshot::initial();
        let (receipt, yield_request) = request(&state, LaneIntent::Yield);
        let validated = validate_lane_request(&state, &receipt, &yield_request)
            .expect("yield is legal for the player");
        let result = transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held))
            .expect("yield transition");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            result.next_state().player().position(),
            LanePosition::NearTower
        );
        assert_eq!(result.debrief().intent(), LaneIntent::Yield);
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::PositionChanged {
                cause: LaneEffectCause::Intent,
                ..
            }
        )));
    }

#[test]
    fn yield_replays_and_preserves_objective_attribution() {
        let state = LaneSnapshot::initial();
        let (receipt, yield_request) = request(&state, LaneIntent::Yield);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(&receipt, &yield_request, inputs(0, 0, LaneWaveResult::Held))
            .expect("yield append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("yield objective review");
        assert_eq!(review.review().intent(), LaneIntent::Yield);
        assert_eq!(review.review().report().intent(), LaneIntent::Yield);
        review
            .verify_lane(record)
            .expect("replayable objective review");
    }

#[test]
    fn yield_rejects_mana_spend() {
        let state = LaneSnapshot::initial();
        let (receipt, yield_request) = request(&state, LaneIntent::Yield);
        let validated = validate_lane_request(&state, &receipt, &yield_request).expect("valid");
        let mut invalid_inputs = inputs(0, 0, LaneWaveResult::Held);
        invalid_inputs.execution.mana_spent = LaneMana::new(1).unwrap();
        assert!(matches!(
            transition_lane(&state, &validated, &invalid_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ManaSpentWithoutContest {
                    intent: LaneIntent::Yield,
                    ..
                }
            ))
        ));
    }
