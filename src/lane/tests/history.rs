#[test]
fn history_replays_the_committed_window() {
    let state = LaneSnapshot::initial();
    let (receipt, request) = request(&state, LaneIntent::Contest);
    let mut history = LaneHistory::new(state).expect("initial state is valid");
    history
        .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
        .expect("append");
    assert_eq!(history.records().len(), 1);
    assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn history_requires_open_initial_state_and_v3_record_identity() {
    let state = LaneSnapshot::initial();
    let (receipt, request) = request(&state, LaneIntent::Contest);
    let mut history = LaneHistory::new(state).expect("initial state is valid");
    history
        .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
        .expect("append");
    assert_eq!(history.records()[0].replay_id(), M2_REPLAY_ID);

    let resolved = history.current_state();
    assert_eq!(resolved.status().phase(), LanePhase::Resolved);
    assert!(matches!(
        LaneHistory::new(resolved),
        Err(LaneHistoryError::InvalidInitialState)
    ));

    let mut tampered = history;
    tampered.records[0].replay_id = "m2-one-lane-window-v1";
    assert!(matches!(
        tampered.verify_replay(),
        Err(LaneReplayError::ReplayIdMismatch { index: 0 })
    ));
}
