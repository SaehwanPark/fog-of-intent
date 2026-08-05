#[test]
    fn hold_lane_objective_classifies_committed_lane_facts_without_changing_state() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
            .expect("append");
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("objective review");
        assert_eq!(review.source_replay_id(), M2_REPLAY_ID);
        assert_eq!(
            review.review().disposition(),
            ObjectiveDisposition::GoalAchieved
        );
        assert_eq!(
            review.review().criteria()[0].status(),
            ObjectiveCriterionStatus::Met
        );
        assert_eq!(
            review.review().criteria()[1].status(),
            ObjectiveCriterionStatus::Met
        );
        assert_eq!(
            review.review().attribution_limit(),
            ObjectiveAttributionLimit::CommittedFactsOnly
        );
        assert_eq!(review.review().report().schema(), M2_OBJECTIVE_SCHEMA);
        review.verify_lane(record).expect("objective replay");
        assert_eq!(history.current_state(), record.result().next_state());
        assert_eq!(
            record.result().state_hash(),
            record.result().next_state().hash()
        );
    }

#[test]
    fn objective_covers_yielded_forced_out_partial_and_coordination_cases() {
        let direct_partial = ObjectiveEvaluationInputs::new(
            M2_REPLAY_ID,
            StateHash::from_raw(1),
            StateHash::from_raw(2),
            LaneOutcome::HeldSpace,
            LanePosition::Center,
            LaneHealth::zero(),
            LaneIntent::Contest,
            LaneWaveResult::Held,
            ObjectiveCoordination::NotApplicable,
            trace(5, 0),
        );
        assert_eq!(
            evaluate_terminal_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, &direct_partial,)
                .expect("partial")
                .disposition(),
            ObjectiveDisposition::GoalPartiallyAchieved
        );

        let state = LaneSnapshot::initial();
        let (stable_receipt, stable_request) = request(&state, LaneIntent::Stabilize);
        let mut stable_history = LaneHistory::new(state).expect("valid");
        stable_history
            .append(
                &stable_receipt,
                &stable_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("stable append");
        let stable_review = review_lane_objective(
            ScenarioGoal::HoldLaneSpaceThroughWindow,
            &stable_history.records()[0],
        )
        .expect("stable objective");
        assert_eq!(
            stable_review.review().disposition(),
            ObjectiveDisposition::GoalMissed
        );

        let (forced_receipt, forced_request) = request(&state, LaneIntent::Contest);
        let mut forced_history = LaneHistory::new(state).expect("valid");
        forced_history
            .append(
                &forced_receipt,
                &forced_request,
                inputs(8, 0, LaneWaveResult::Held),
            )
            .expect("forced append");
        let forced_review = review_lane_objective(
            ScenarioGoal::HoldLaneSpaceThroughWindow,
            &forced_history.records()[0],
        )
        .expect("forced objective");
        assert_eq!(
            forced_review.review().disposition(),
            ObjectiveDisposition::GoalMissed
        );

        let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
        let coordinated_request = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
            ProposalResponse::Accept {
                proposal_id: offer.proposal().id(),
            },
        );
        let mut coordinated_history = CoordinatedLaneHistory::new(state).expect("valid");
        coordinated_history
            .append(
                &player_receipt,
                &allied_receipt,
                &offer,
                &coordinated_request,
                CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted),
                inputs(1, 1, LaneWaveResult::Held),
            )
            .expect("coordinated append");
        let coordinated_review = review_coordinated_objective(
            ScenarioGoal::HoldLaneSpaceThroughWindow,
            &coordinated_history.records()[0],
        )
        .expect("coordinated objective");
        assert_eq!(
            coordinated_review.review().coordination(),
            ObjectiveCoordination::Resolved(CoordinationDisposition::AcceptedOffer)
        );
        coordinated_review
            .verify_coordinated(&coordinated_history.records()[0])
            .expect("coordinated objective replay");
    }

#[test]
    fn objective_replay_rejects_tampered_inputs_or_review_and_hides_state_hash_from_report() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("valid");
        history
            .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
            .expect("append");
        let record = &history.records()[0];
        let mut review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("review");
        let report = review.review().report();
        assert!(!format!("{report:?}").contains(&state.hash().value().to_string()));
        review.inputs = ObjectiveEvaluationInputs::new(
            M2_REPLAY_ID,
            StateHash::from_raw(999),
            review.inputs.terminal_state_hash(),
            review.inputs.outcome(),
            review.inputs.player_position(),
            review.inputs.player_health(),
            review.inputs.intent(),
            review.inputs.wave_result(),
            review.inputs.coordination(),
            review.inputs.execution_trace(),
        );
        assert_eq!(
            review.verify_lane(record),
            Err(ObjectiveError::ReviewMismatch)
        );

        let unsupported = ObjectiveEvaluationInputs::new(
            "unsupported-replay",
            StateHash::from_raw(1),
            StateHash::from_raw(2),
            LaneOutcome::HeldSpace,
            LanePosition::Center,
            LaneHealth::new(1).expect("bounded"),
            LaneIntent::Contest,
            LaneWaveResult::Held,
            ObjectiveCoordination::NotApplicable,
            trace(5, 0),
        );
        assert_eq!(
            evaluate_terminal_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, &unsupported,),
            Err(ObjectiveError::UnsupportedReplayId)
        );
    }

#[test]
    fn named_strategy_fixtures_are_matched_input_and_replayable() {
        let fixtures = [
            StrategyFixtureId::HappyPath,
            StrategyFixtureId::RiskTaking,
            StrategyFixtureId::Conservative,
        ];
        let mut outcomes = Vec::new();
        for id in fixtures {
            let fixture = strategy_fixture(id).expect("fixture");
            let first = run_strategy_fixture(fixture).expect("first run");
            let second = run_strategy_fixture(fixture).expect("second run");
            assert_eq!(first.objective().review(), second.objective().review());
            assert_eq!(
                first.history().records()[0].result(),
                second.history().records()[0].result()
            );
            first
                .history()
                .verify_replay()
                .expect("fixture history replay");
            first
                .objective()
                .verify_coordinated(&first.history().records()[0])
                .expect("fixture objective replay");
            outcomes.push((
                fixture.id(),
                first.history().records()[0].result().lane().outcome(),
                first.objective().review().disposition(),
            ));
        }
        assert_eq!(
            outcomes,
            vec![
                (
                    StrategyFixtureId::HappyPath,
                    LaneOutcome::HeldSpace,
                    ObjectiveDisposition::GoalAchieved,
                ),
                (
                    StrategyFixtureId::RiskTaking,
                    LaneOutcome::YieldedSpace,
                    ObjectiveDisposition::GoalMissed,
                ),
                (
                    StrategyFixtureId::Conservative,
                    LaneOutcome::YieldedSpace,
                    ObjectiveDisposition::GoalMissed,
                ),
            ]
        );
        let mut tampered = strategy_fixture(StrategyFixtureId::RiskTaking).expect("fixture");
        tampered.expected_outcome = LaneOutcome::HeldSpace;
        assert!(matches!(
            run_strategy_fixture(tampered),
            Err(StrategyFixtureError::UnexpectedOutcome)
        ));
    }
