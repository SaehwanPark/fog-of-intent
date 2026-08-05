#[test]
    fn two_window_scenario_reopens_once_and_replays_both_commits() {
        let initial = LaneSnapshot::initial();
        assert!(matches!(
            reopen_resolved_snapshot(&initial),
            Err(ScenarioError::InvalidReopenState)
        ));
        let mut history = LaneScenarioHistory::new(initial).expect("valid scenario");
        let (first_receipt, first_request) = request(&initial, LaneIntent::Contest);
        let first = history
            .append(
                &first_receipt,
                &first_request,
                inputs(0, 1, LaneWaveResult::Advanced),
            )
            .expect("first window");
        let mut tampered_result = first.clone();
        tampered_result.state_hash = StateHash::from_raw(0);
        assert_eq!(
            reopen_lane_window(&tampered_result),
            Err(ScenarioError::InvalidReopenState)
        );
        let reopened = history.current_state();
        assert_eq!(first.next_state().phase(), LanePhase::Resolved);
        assert_eq!(reopened.phase(), LanePhase::Open);
        assert_eq!(reopened.turn(), Turn::new(1));
        assert_eq!(reopened.player(), first.next_state().player());
        assert_eq!(reopened.opponent(), first.next_state().opponent());
        assert_eq!(reopened.wave(), first.next_state().wave());
        assert_eq!(reopened.jungle_threat(), first.next_state().jungle_threat());
        assert_eq!(reopened.terminal_outcome(), None);
        assert_eq!(history.records()[0].window(), ScenarioWindow::First);
        assert_eq!(history.records()[0].reopened_state(), Some(reopened));
        assert_eq!(
            history.terminal_state(),
            Err(ScenarioError::ScenarioIncomplete)
        );

        let (second_receipt, second_request) = request(&reopened, LaneIntent::Stabilize);
        let second = history
            .append(
                &second_receipt,
                &second_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("second window");
        assert_eq!(history.records().len(), 2);
        assert_eq!(history.records()[1].window(), ScenarioWindow::Second);
        assert_eq!(history.records()[1].reopened_state(), None);
        assert_eq!(history.current_state(), second.next_state());
        assert_eq!(history.current_state().phase(), LanePhase::Resolved);
        assert_eq!(history.terminal_state(), Ok(history.current_state()));
        history.verify_replay().expect("scenario replay");
        assert_eq!(
            review_lane_objective(
                ScenarioGoal::HoldLaneSpaceThroughWindow,
                history.records()[0].transition(),
            )
            .expect("first objective")
            .review()
            .disposition(),
            ObjectiveDisposition::GoalAchieved
        );
        assert!(matches!(
            history.append(
                &second_receipt,
                &second_request,
                inputs(0, 0, LaneWaveResult::Held),
            ),
            Err(ScenarioError::ScenarioComplete)
        ));
    }

#[test]
    fn two_window_scenario_replay_rejects_reopen_and_record_tampering() {
        let initial = LaneSnapshot::initial();
        let mut history = LaneScenarioHistory::new(initial).expect("valid scenario");
        let (first_receipt, first_request) = request(&initial, LaneIntent::Contest);
        history
            .append(
                &first_receipt,
                &first_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("first window");
        let reopened = history.current_state();
        let (second_receipt, second_request) = request(&reopened, LaneIntent::Contest);
        history
            .append(
                &second_receipt,
                &second_request,
                inputs(3, 0, LaneWaveResult::Lost),
            )
            .expect("second window");
        history.records[0].reopened_state = Some(initial);
        assert_eq!(history.verify_replay(), Err(ScenarioError::ReplayMismatch));
    }

#[test]
    fn final_debrief_replays_committed_window_facts_and_redacts_provenance() {
        let build_history = || {
            let initial = LaneSnapshot::initial();
            let mut history = LaneScenarioHistory::new(initial).expect("valid scenario");
            let (first_receipt, first_request) = request(&initial, LaneIntent::Contest);
            history
                .append(
                    &first_receipt,
                    &first_request,
                    inputs(0, 1, LaneWaveResult::Advanced),
                )
                .expect("first window");
            let reopened = history.current_state();
            let (second_receipt, second_request) = request(&reopened, LaneIntent::Stabilize);
            history
                .append(
                    &second_receipt,
                    &second_request,
                    inputs(0, 0, LaneWaveResult::Held),
                )
                .expect("second window");
            history
        };
        let history = build_history();
        let debrief = build_scenario_debrief(&history).expect("debrief");
        assert_eq!(debrief.replay_id(), M2_FINAL_DEBRIEF_REPLAY_ID);
        assert_eq!(debrief.source_replay_id(), M2_TWO_WINDOW_REPLAY_ID);
        assert_eq!(debrief.windows()[0].window(), ScenarioWindow::First);
        assert_eq!(debrief.windows()[0].intent(), LaneIntent::Contest);
        assert_eq!(
            debrief.windows()[0].objective().disposition(),
            ObjectiveDisposition::GoalAchieved
        );
        assert_eq!(debrief.windows()[1].window(), ScenarioWindow::Second);
        assert_eq!(debrief.windows()[1].intent(), LaneIntent::Stabilize);
        assert_eq!(debrief.final_objective(), ObjectiveDisposition::GoalMissed);
        assert_eq!(
            debrief.attribution_limit(),
            FinalDebriefAttributionLimit::CommittedHistoryFactsOnly
        );
        assert!(
            !format!("{:?}", debrief.report())
                .contains(&debrief.source_terminal_state_hash().value().to_string())
        );
        debrief.verify_replay(&history).expect("debrief replay");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));

        let mut tampered = debrief.clone();
        tampered.source_terminal_state_hash = StateHash::from_raw(0);
        assert_eq!(
            tampered.verify_replay(&history),
            Err(ScenarioDebriefError::ReplayMismatch)
        );

        let incomplete = LaneScenarioHistory::new(LaneSnapshot::initial()).expect("valid");
        assert_eq!(
            build_scenario_debrief(&incomplete),
            Err(ScenarioDebriefError::IncompleteHistory)
        );
    }
