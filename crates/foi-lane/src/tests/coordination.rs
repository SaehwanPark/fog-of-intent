use super::*;

#[test]
fn allied_policy_is_visible_input_bound_and_hidden_state_invariant() {
  let first = LaneSnapshot::initial();
  let second = LaneSnapshot::new(
    M2_LANE_RULESET,
    first.turn(),
    LaneStatus::Open,
    first.player(),
    OpponentTruth::new(
      OPPONENT_LANER,
      LaneHealth::new(1).expect("bounded"),
      LanePosition::FarSide,
      OpponentPosture::Passive,
    ),
    first.wave(),
    JungleThreatTruth::Absent,
  );
  let first_receipt = observe_allied(&first, ObservationId::new(12));
  let second_receipt = observe_allied(&second, ObservationId::new(12));
  assert_eq!(first_receipt.observation(), second_receipt.observation());
  let first_proposal =
    scripted_allied_proposal(first_receipt.observation(), trace(3, 3)).expect("proposal");
  let second_proposal =
    scripted_allied_proposal(second_receipt.observation(), trace(3, 3)).expect("proposal");
  assert_eq!(first_proposal, second_proposal);
  assert_eq!(
    first_proposal.profile().profile_id(),
    SCRIPTED_ALLIED_PROFILE
  );
  assert_eq!(first_proposal.candidates()[0].score(), 2);
  assert_eq!(first_proposal.candidates()[1].score(), 5);
  assert_eq!(first_proposal.selected_intent(), LaneIntent::Contest);
  assert_eq!(
    offer_allied_proposal(first_proposal)
      .expect("offer")
      .support(),
    AlliedSupport::AssistContest
  );
}

#[test]
fn allied_policy_changes_only_with_declared_visible_features() {
  let state = LaneSnapshot::new(
    M2_LANE_RULESET,
    Turn::new(0),
    LaneStatus::Open,
    PlayerLaneState::new(
      PLAYER_LANER,
      LaneHealth::new(2).expect("bounded"),
      LaneResources::initial(),
      LanePosition::Center,
    ),
    LaneSnapshot::initial().opponent(),
    WaveState::new(WavePressure::new(3).expect("bounded")),
    JungleThreatTruth::InLane,
  );
  let receipt = observe_allied(&state, ObservationId::new(13));
  let proposal = scripted_allied_proposal(receipt.observation(), trace(3, 3)).expect("proposal");
  assert_eq!(proposal.candidates()[0].score(), 6);
  assert_eq!(proposal.candidates()[1].score(), 6);
  assert_eq!(proposal.selected_intent(), LaneIntent::Stabilize);
}

#[test]
fn coordinated_accept_keeps_execution_and_state_in_the_base_lane_contract() {
  let state = LaneSnapshot::initial();
  let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
  let request = CoordinatedLaneRequest::new(
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
    ProposalResponse::Accept {
      proposal_id: offer.proposal().id(),
    },
  );
  let coordination_inputs =
    CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted);
  let lane_inputs = inputs(1, 2, LaneWaveResult::Advanced);
  let coordinated = resolve_coordinated_lane(
    &state,
    &player_receipt,
    &allied_receipt,
    &offer,
    &request,
    &coordination_inputs,
    &lane_inputs,
  )
  .expect("coordinated transition");
  let validated =
    validate_lane_request(&state, &player_receipt, &request.intent()).expect("base request");
  let base = transition_lane(&state, &validated, &lane_inputs).expect("base transition");
  assert_eq!(coordinated.next_state(), base.next_state());
  assert_eq!(coordinated.state_hash(), base.state_hash());
  assert_eq!(
    coordinated.coordination().disposition(),
    CoordinationDisposition::AcceptedOffer
  );
  assert!(matches!(
    coordinated.events()[0],
    CoordinatedEvent::ProposalOffered { .. }
  ));
  assert!(matches!(
    coordinated.effects()[0],
    CoordinatedEffect::SupportCommitted { .. }
  ));
  assert_eq!(
    coordinated.debrief().execution(),
    CoordinatedExecutionReview::ConditionalOnCoordination { trace: trace(5, 0) }
  );
}

#[test]
fn coordination_maps_closed_responses_and_rejects_malformed_inputs() {
  let state = LaneSnapshot::initial();
  let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
  let accepted = CoordinatedLaneRequest::new(
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
    ProposalResponse::Accept {
      proposal_id: offer.proposal().id(),
    },
  );
  assert_eq!(
    resolve_coordination(
      &offer,
      &accepted,
      &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted),
    )
    .expect("accepted")
    .disposition(),
    CoordinationDisposition::AcceptedOffer
  );
  assert_eq!(
    resolve_coordination(
      &offer,
      &accepted,
      &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyDeclined),
    )
    .expect("declined")
    .disposition(),
    CoordinationDisposition::AllyDeclined
  );
  let rejected = CoordinatedLaneRequest::new(
    accepted.intent(),
    ProposalResponse::Reject {
      proposal_id: offer.proposal().id(),
    },
  );
  assert_eq!(
    resolve_coordination(
      &offer,
      &rejected,
      &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::NotRequested),
    )
    .expect("rejected")
    .disposition(),
    CoordinationDisposition::PlayerRejected
  );
  let counter = CoordinatedLaneRequest::new(
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Stabilize),
    counter_to_stabilize(offer.proposal().id()),
  );
  assert_eq!(
    resolve_coordination(
      &offer,
      &counter,
      &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted),
    )
    .expect("counter accepted")
    .disposition(),
    CoordinationDisposition::CounterAccepted
  );
  assert_eq!(
    resolve_coordination(
      &offer,
      &counter,
      &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyDeclined),
    )
    .expect("counter declined")
    .disposition(),
    CoordinationDisposition::CounterRejected
  );
  let invalid_accept = CoordinatedLaneRequest::new(
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Stabilize),
    ProposalResponse::Accept {
      proposal_id: offer.proposal().id(),
    },
  );
  assert_eq!(
    validate_coordinated_request(
      &state,
      &player_receipt,
      &allied_receipt,
      &offer,
      &invalid_accept,
      trace(3, 3),
    ),
    Err(CoordinationError::AcceptIntentMismatch)
  );
  assert_eq!(
    resolve_coordination(
      &offer,
      &accepted,
      &CoordinationResolutionInputs::new(trace(4, 5), FollowThrough::NotRequested),
    ),
    Err(CoordinationError::MalformedFollowThrough)
  );
}

#[test]
fn coordinated_history_replays_and_rejects_tampering() {
  let state = LaneSnapshot::initial();
  let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
  let request = CoordinatedLaneRequest::new(
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
    ProposalResponse::Reject {
      proposal_id: offer.proposal().id(),
    },
  );
  let mut history = CoordinatedLaneHistory::new(state).expect("valid initial state");
  history
    .append(
      &player_receipt,
      &allied_receipt,
      &offer,
      &request,
      CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::NotRequested),
      inputs(1, 1, LaneWaveResult::Held),
    )
    .expect("append");
  assert_eq!(history.records().len(), 1);
  assert_eq!(history.records()[0].replay_id(), M2_COORDINATION_REPLAY_ID);
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
  history.records[0].replay_id = "m2-one-lane-coordination-v1";
  assert_eq!(
    history.verify_replay(),
    Err(CoordinationError::ReplayMismatch)
  );
  history.records[0].replay_id = M2_COORDINATION_REPLAY_ID;
  history.records[0].request = CoordinatedLaneRequest::new(
    request.intent(),
    ProposalResponse::Reject {
      proposal_id: ProposalId(0),
    },
  );
  assert_eq!(
    history.verify_replay(),
    Err(CoordinationError::ReplayMismatch)
  );
  history.records[0].request = request;
  history.records[0].base_record.command = LaneIntentCommand::new(
    PLAYER_LANER,
    state.turn(),
    M2_LANE_RULESET,
    ObservationId::new(9),
    StateHash::from_raw(0),
    LaneIntent::Contest,
  );
  assert_eq!(
    history.verify_replay(),
    Err(CoordinationError::ReplayMismatch)
  );
}
