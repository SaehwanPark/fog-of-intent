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
