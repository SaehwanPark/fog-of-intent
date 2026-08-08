use super::*;

pub(super) fn trace(stream: u8, draw: u16) -> InputTrace {
  InputTrace::new(StreamId::new(stream), DrawId::new(draw))
}

pub(super) fn inputs(
  self_damage: u8,
  opponent_damage: u8,
  wave_result: LaneWaveResult,
) -> LaneResolvedInputs {
  LaneResolvedInputs::new(
    trace(1, 1),
    trace(2, 2),
    trace(3, 3),
    trace(4, 4),
    LaneExecutionInputs::new(
      trace(5, 0),
      LaneDamage::new(self_damage).expect("damage must be bounded"),
      LaneDamage::new(opponent_damage).expect("damage must be bounded"),
      wave_result,
    ),
  )
}

pub(super) fn request(
  state: &LaneSnapshot,
  intent: LaneIntent,
) -> (LaneObservationReceipt, LaneIntentRequest) {
  let receipt = observe_player(state, ObservationId::new(9));
  let request =
    LaneIntentRequest::new(PLAYER_LANER, receipt.observation().observation_id(), intent);
  (receipt, request)
}

pub(super) fn river_side_state() -> LaneSnapshot {
  let state = LaneSnapshot::initial();
  LaneSnapshot::new(
    state.ruleset(),
    state.turn(),
    LaneStatus::Open,
    state.player(),
    state.opponent(),
    state.wave(),
    JungleThreatTruth::RiverSide,
  )
}

pub(super) fn two_beat_state() -> LaneSnapshot {
  let state = LaneSnapshot::initial();
  LaneSnapshot::new_with_window(
    state.ruleset(),
    state.turn(),
    LaneWindow::TwoBeats,
    LaneStatus::Open,
    state.player(),
    state.opponent(),
    state.wave(),
    state.jungle_threat(),
  )
}

pub(super) fn committed_parent(intent: LaneIntent) -> (LaneHistory, LaneObservationReceipt) {
  let state = LaneSnapshot::initial();
  let (receipt, request) = request(&state, intent);
  let mut parent = LaneHistory::new(state).expect("initial state is valid");
  parent
    .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
    .expect("parent append");
  (parent, receipt)
}

pub(super) fn coordinated_offer(
  state: &LaneSnapshot,
  policy_trace: InputTrace,
) -> (
  LaneObservationReceipt,
  AlliedObservationReceipt,
  AlliedProposalOffer,
) {
  let player_receipt = observe_player(state, ObservationId::new(9));
  let allied_receipt = observe_allied(state, ObservationId::new(9));
  let proposal = scripted_allied_proposal(allied_receipt.observation(), policy_trace)
    .expect("canonical proposal");
  let offer = offer_allied_proposal(proposal).expect("canonical offer");
  (player_receipt, allied_receipt, offer)
}

pub(super) fn counter_to_stabilize(proposal_id: ProposalId) -> ProposalResponse {
  ProposalResponse::Counter {
    proposal_id,
    counter: CounterProposal::RequestIntent {
      requested_intent: LaneIntent::Stabilize,
      target: PLAYER_LANER,
      commitment: CoordinationCommitment::UntilWindowEnd,
      focus: SupportFocus::Wave,
      abort: SupportAbort::IfPlayerHealthAtMost(2),
      fallback: SupportFallback::HoldPosition,
    },
  }
}
