use super::*;

#[test]
fn both_intents_are_legal_and_produce_distinct_positions() {
  let state = LaneSnapshot::initial();
  let (stabilize_receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
  let stabilize = validate_lane_request(&state, &stabilize_receipt, &stabilize_request)
    .expect("stabilize is legal");
  let stable_result = transition_lane(&state, &stabilize, &inputs(0, 1, LaneWaveResult::Held))
    .expect("stabilize transition");
  assert_eq!(stable_result.outcome(), LaneOutcome::YieldedSpace);
  assert_eq!(
    stable_result.next_state().player().position(),
    LanePosition::NearTower
  );

  let (contest_receipt, contest_request) = request(&state, LaneIntent::Contest);
  let contest =
    validate_lane_request(&state, &contest_receipt, &contest_request).expect("contest is legal");
  let contest_result = transition_lane(&state, &contest, &inputs(0, 1, LaneWaveResult::Advanced))
    .expect("contest transition");
  assert_eq!(contest_result.outcome(), LaneOutcome::HeldSpace);
  assert_eq!(
    contest_result.next_state().player().position(),
    LanePosition::Center
  );
}

#[test]
fn stale_observer_identity_is_rejected() {
  let state = LaneSnapshot::initial();
  let (mut receipt, request) = request(&state, LaneIntent::Stabilize);
  receipt.observation.observer = OPPONENT_LANER;

  assert_eq!(
    validate_lane_request(&state, &receipt, &request),
    Err(LaneValidationError::StaleObservation)
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
  let validated = validate_lane_request(&state, &current_receipt, &current_request).expect("valid");
  let resolved =
    transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held)).expect("transition");
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
  let validated =
    validate_lane_request(&state, &receipt, &yield_request).expect("yield is legal for the player");
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
  invalid_inputs.execution = invalid_inputs
    .execution
    .with_mana_spent(LaneMana::new(1).unwrap());
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

#[test]
fn target_focus_defaults_to_minions_and_replays() {
  let state = LaneSnapshot::initial();
  let (receipt, req) = request(&state, LaneIntent::Stabilize);
  assert_eq!(req.target_focus(), LaneTargetFocus::Minions);
  let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");
  assert_eq!(validated.command().target_focus(), LaneTargetFocus::Minions);

  let result =
    transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held)).expect("transition");
  assert_eq!(result.debrief().target_focus(), LaneTargetFocus::Minions);
  assert!(result.events().iter().any(|e| matches!(
    e,
    LaneEvent::TargetFocusSelected {
      focus: LaneTargetFocus::Minions,
      ..
    }
  )));
  assert!(result.effects().iter().any(|e| matches!(
    e,
    LaneEffect::TargetFocusSet {
      focus: LaneTargetFocus::Minions,
      ..
    }
  )));

  let mut history = LaneHistory::new(state).expect("valid initial state");
  history
    .append(&receipt, &req, inputs(0, 0, LaneWaveResult::Held))
    .expect("append");
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn target_focus_opposing_laner_and_tower_are_valid_and_bind_record_identity() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));

  let default_req = LaneIntentRequest::new_with_target_focus(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Contest,
    LaneTargetFocus::Minions,
  );
  let laner_req = LaneIntentRequest::new_with_target_focus(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Contest,
    LaneTargetFocus::OpposingLaner,
  );
  let tower_req = LaneIntentRequest::new_with_target_focus(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Contest,
    LaneTargetFocus::Tower,
  );

  let default_val = validate_lane_request(&state, &receipt, &default_req).expect("valid");
  let laner_val = validate_lane_request(&state, &receipt, &laner_req).expect("valid");
  let tower_val = validate_lane_request(&state, &receipt, &tower_req).expect("valid");

  let default_res = transition_lane(&state, &default_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("default transition");
  let laner_res = transition_lane(&state, &laner_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("laner transition");
  let tower_res = transition_lane(&state, &tower_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("tower transition");

  assert_eq!(
    default_res.debrief().target_focus(),
    LaneTargetFocus::Minions
  );
  assert_eq!(
    laner_res.debrief().target_focus(),
    LaneTargetFocus::OpposingLaner
  );
  assert_eq!(tower_res.debrief().target_focus(), LaneTargetFocus::Tower);

  let mut h_default = LaneHistory::new(state).unwrap();
  h_default
    .append(&receipt, &default_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_laner = LaneHistory::new(state).unwrap();
  h_laner
    .append(&receipt, &laner_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_tower = LaneHistory::new(state).unwrap();
  h_tower
    .append(&receipt, &tower_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_laner.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_tower.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_laner.records()[0]),
    lane_record_identity(&h_tower.records()[0])
  );

  assert_eq!(h_laner.verify_replay(), Ok(h_laner.current_state()));
  assert_eq!(h_tower.verify_replay(), Ok(h_tower.current_state()));
}

#[test]
fn laner_observation_advertises_available_target_focuses() {
  let state = LaneSnapshot::initial();
  let obs = observe_player(&state, ObservationId::new(42)).observation();
  assert_eq!(
    obs.available_target_focuses(),
    [
      LaneTargetFocus::Minions,
      LaneTargetFocus::OpposingLaner,
      LaneTargetFocus::Tower,
    ]
  );
}

#[test]
fn commitment_defaults_to_standard_and_replays() {
  let state = LaneSnapshot::initial();
  let (receipt, req) = request(&state, LaneIntent::Stabilize);
  assert_eq!(req.commitment(), LaneCommitment::Standard);
  let validated = validate_lane_request(&state, &receipt, &req).expect("valid request");
  assert_eq!(validated.command().commitment(), LaneCommitment::Standard);

  let result =
    transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held)).expect("transition");
  assert_eq!(result.debrief().commitment(), LaneCommitment::Standard);
  assert!(result.events().iter().any(|e| matches!(
    e,
    LaneEvent::CommitmentSelected {
      commitment: LaneCommitment::Standard,
      ..
    }
  )));
  assert!(result.effects().iter().any(|e| matches!(
    e,
    LaneEffect::CommitmentSet {
      commitment: LaneCommitment::Standard,
      ..
    }
  )));

  let mut history = LaneHistory::new(state).expect("valid initial state");
  history
    .append(&receipt, &req, inputs(0, 0, LaneWaveResult::Held))
    .expect("append");
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn commitment_cautious_and_aggressive_are_valid_and_bind_record_identity() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));

  let default_req = LaneIntentRequest::new_with_commitment(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Contest,
    LaneCommitment::Standard,
  );
  let cautious_req = LaneIntentRequest::new_with_commitment(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Contest,
    LaneCommitment::Cautious,
  );
  let aggressive_req = LaneIntentRequest::new_with_commitment(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Contest,
    LaneCommitment::Aggressive,
  );

  let default_val = validate_lane_request(&state, &receipt, &default_req).expect("valid");
  let cautious_val = validate_lane_request(&state, &receipt, &cautious_req).expect("valid");
  let aggressive_val = validate_lane_request(&state, &receipt, &aggressive_req).expect("valid");

  let default_res = transition_lane(&state, &default_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("default transition");
  let cautious_res = transition_lane(&state, &cautious_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("cautious transition");
  let aggressive_res =
    transition_lane(&state, &aggressive_val, &inputs(0, 0, LaneWaveResult::Held))
      .expect("aggressive transition");

  assert_eq!(default_res.debrief().commitment(), LaneCommitment::Standard);
  assert_eq!(
    cautious_res.debrief().commitment(),
    LaneCommitment::Cautious
  );
  assert_eq!(
    aggressive_res.debrief().commitment(),
    LaneCommitment::Aggressive
  );

  let mut h_default = LaneHistory::new(state).unwrap();
  h_default
    .append(&receipt, &default_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_cautious = LaneHistory::new(state).unwrap();
  h_cautious
    .append(&receipt, &cautious_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_aggressive = LaneHistory::new(state).unwrap();
  h_aggressive
    .append(
      &receipt,
      &aggressive_req,
      inputs(0, 0, LaneWaveResult::Held),
    )
    .unwrap();

  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_cautious.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_aggressive.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_cautious.records()[0]),
    lane_record_identity(&h_aggressive.records()[0])
  );

  assert_eq!(h_cautious.verify_replay(), Ok(h_cautious.current_state()));
  assert_eq!(
    h_aggressive.verify_replay(),
    Ok(h_aggressive.current_state())
  );
}

#[test]
fn laner_observation_advertises_available_commitments() {
  let state = LaneSnapshot::initial();
  let obs = observe_player(&state, ObservationId::new(42)).observation();
  assert_eq!(
    obs.available_commitments(),
    [
      LaneCommitment::Standard,
      LaneCommitment::Cautious,
      LaneCommitment::Aggressive,
    ]
  );
}

#[test]
fn ping_signal_defaults_to_none_and_replays() {
  let state = LaneSnapshot::initial();
  let (receipt, request) = request(&state, LaneIntent::Stabilize);
  assert_eq!(request.ping_signal(), LanePingSignal::None);
  let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
  assert_eq!(validated.command().ping_signal(), LanePingSignal::None);

  let result =
    transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held)).expect("transition");
  assert_eq!(result.debrief().ping_signal(), LanePingSignal::None);
  assert!(result.events().contains(&LaneEvent::PingSignalSelected {
    actor: PLAYER_LANER,
    ping_signal: LanePingSignal::None,
  }));
  assert!(result.effects().contains(&LaneEffect::PingSignalSet {
    actor: PLAYER_LANER,
    ping_signal: LanePingSignal::None,
    cause: LaneEffectCause::Intent,
    provenance: LaneEffectProvenance::direct_immediate(),
  }));

  let mut history = LaneHistory::new(state).unwrap();
  history
    .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn ping_signal_danger_on_my_way_assist_enemy_missing_are_valid_and_bind_record_identity() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));
  let default_req =
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(1), LaneIntent::Stabilize);
  let danger_req = LaneIntentRequest::new_with_ping_signal(
    PLAYER_LANER,
    ObservationId::new(1),
    LaneIntent::Stabilize,
    LanePingSignal::Danger,
  );
  let assist_req = LaneIntentRequest::new_with_ping_signal(
    PLAYER_LANER,
    ObservationId::new(1),
    LaneIntent::Stabilize,
    LanePingSignal::Assist,
  );

  let default_val = validate_lane_request(&state, &receipt, &default_req).expect("valid");
  let danger_val = validate_lane_request(&state, &receipt, &danger_req).expect("valid");
  let assist_val = validate_lane_request(&state, &receipt, &assist_req).expect("valid");

  let default_res = transition_lane(&state, &default_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("default transition");
  let danger_res = transition_lane(&state, &danger_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("danger transition");
  let assist_res = transition_lane(&state, &assist_val, &inputs(0, 0, LaneWaveResult::Held))
    .expect("assist transition");

  assert_eq!(default_res.debrief().ping_signal(), LanePingSignal::None);
  assert_eq!(danger_res.debrief().ping_signal(), LanePingSignal::Danger);
  assert_eq!(assist_res.debrief().ping_signal(), LanePingSignal::Assist);

  let mut h_default = LaneHistory::new(state).unwrap();
  h_default
    .append(&receipt, &default_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_danger = LaneHistory::new(state).unwrap();
  h_danger
    .append(&receipt, &danger_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_assist = LaneHistory::new(state).unwrap();
  h_assist
    .append(&receipt, &assist_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_danger.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_assist.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_danger.records()[0]),
    lane_record_identity(&h_assist.records()[0])
  );

  assert_eq!(h_danger.verify_replay(), Ok(h_danger.current_state()));
  assert_eq!(h_assist.verify_replay(), Ok(h_assist.current_state()));
}

#[test]
fn laner_observation_advertises_available_ping_signals() {
  let state = LaneSnapshot::initial();
  let obs = observe_player(&state, ObservationId::new(42)).observation();
  assert_eq!(
    obs.available_ping_signals(),
    [
      LanePingSignal::None,
      LanePingSignal::Danger,
      LanePingSignal::OnMyWay,
      LanePingSignal::Assist,
      LanePingSignal::EnemyMissing,
    ]
  );
}

#[test]
fn abort_condition_defaults_to_none_and_replays() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));
  let request = LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(1), LaneIntent::Stabilize);
  assert_eq!(request.abort_condition(), LaneAbortCondition::None);
  let validated = validate_lane_request(&state, &receipt, &request).unwrap();
  assert_eq!(
    validated.command().abort_condition(),
    LaneAbortCondition::None
  );

  let mut history = LaneHistory::new(state).unwrap();
  let result = history
    .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();
  assert_eq!(result.debrief().abort_condition(), LaneAbortCondition::None);
  assert!(
    result
      .events()
      .contains(&LaneEvent::AbortConditionSelected {
        actor: PLAYER_LANER,
        abort_condition: LaneAbortCondition::None,
      })
  );
  assert!(
    !result
      .events()
      .iter()
      .any(|e| matches!(e, LaneEvent::AbortConditionTriggered { .. }))
  );
  assert!(result.effects().contains(&LaneEffect::AbortConditionSet {
    actor: PLAYER_LANER,
    abort_condition: LaneAbortCondition::None,
    cause: LaneEffectCause::Intent,
    provenance: LaneEffectProvenance::direct_immediate(),
  }));
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn abort_conditions_are_valid_and_bind_record_identity() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));

  let default_req =
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(1), LaneIntent::Contest);
  let health_req = LaneIntentRequest::new_with_abort_condition(
    PLAYER_LANER,
    ObservationId::new(1),
    LaneIntent::Contest,
    LaneAbortCondition::HealthThreshold,
  );
  let threat_req = LaneIntentRequest::new_with_abort_condition(
    PLAYER_LANER,
    ObservationId::new(1),
    LaneIntent::Contest,
    LaneAbortCondition::ThreatSpotted,
  );

  let mut h_default = LaneHistory::new(state).unwrap();
  let default_res = h_default
    .append(&receipt, &default_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_health = LaneHistory::new(state).unwrap();
  let health_res = h_health
    .append(&receipt, &health_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_threat = LaneHistory::new(state).unwrap();
  let threat_res = h_threat
    .append(&receipt, &threat_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  assert_eq!(
    default_res.debrief().abort_condition(),
    LaneAbortCondition::None
  );
  assert_eq!(
    health_res.debrief().abort_condition(),
    LaneAbortCondition::HealthThreshold
  );
  assert_eq!(
    threat_res.debrief().abort_condition(),
    LaneAbortCondition::ThreatSpotted
  );

  assert!(
    health_res
      .events()
      .contains(&LaneEvent::AbortConditionTriggered {
        actor: PLAYER_LANER,
        abort_condition: LaneAbortCondition::HealthThreshold,
      })
  );
  assert!(
    threat_res
      .events()
      .contains(&LaneEvent::AbortConditionTriggered {
        actor: PLAYER_LANER,
        abort_condition: LaneAbortCondition::ThreatSpotted,
      })
  );

  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_health.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_threat.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_health.records()[0]),
    lane_record_identity(&h_threat.records()[0])
  );

  assert_eq!(h_health.verify_replay(), Ok(h_health.current_state()));
  assert_eq!(h_threat.verify_replay(), Ok(h_threat.current_state()));
}

#[test]
fn laner_observation_advertises_available_abort_conditions() {
  let state = LaneSnapshot::initial();
  let obs = observe_player(&state, ObservationId::new(42)).observation();
  assert_eq!(
    obs.available_abort_conditions(),
    [
      LaneAbortCondition::None,
      LaneAbortCondition::HealthThreshold,
      LaneAbortCondition::ThreatSpotted,
      LaneAbortCondition::ResourceDepleted,
    ]
  );
}

#[test]
fn fallback_behavior_defaults_to_maintain_plan_and_replays() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));
  let request = LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(1), LaneIntent::Stabilize);
  assert_eq!(
    request.fallback_behavior(),
    LaneFallbackBehavior::MaintainPlan
  );
  let validated = validate_lane_request(&state, &receipt, &request).unwrap();
  assert_eq!(
    validated.command().fallback_behavior(),
    LaneFallbackBehavior::MaintainPlan
  );

  let mut history = LaneHistory::new(state).unwrap();
  let result = history
    .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();
  assert_eq!(
    result.debrief().fallback_behavior(),
    LaneFallbackBehavior::MaintainPlan
  );
  assert!(
    result
      .events()
      .contains(&LaneEvent::FallbackBehaviorSelected {
        actor: PLAYER_LANER,
        fallback_behavior: LaneFallbackBehavior::MaintainPlan,
      })
  );
  assert!(result.effects().contains(&LaneEffect::FallbackBehaviorSet {
    actor: PLAYER_LANER,
    fallback_behavior: LaneFallbackBehavior::MaintainPlan,
    cause: LaneEffectCause::Intent,
    provenance: LaneEffectProvenance::direct_immediate(),
  }));
  assert_eq!(history.verify_replay(), Ok(history.current_state()));
}

#[test]
fn fallback_behaviors_are_valid_and_bind_record_identity() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(1));

  let default_req =
    LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(1), LaneIntent::Contest);
  let retreat_req = LaneIntentRequest::new_with_fallback_behavior(
    PLAYER_LANER,
    ObservationId::new(1),
    LaneIntent::Contest,
    LaneFallbackBehavior::RetreatToTower,
  );
  let safe_farm_req = LaneIntentRequest::new_with_fallback_behavior(
    PLAYER_LANER,
    ObservationId::new(1),
    LaneIntent::Contest,
    LaneFallbackBehavior::SafeFarm,
  );

  let mut h_default = LaneHistory::new(state).unwrap();
  let default_res = h_default
    .append(&receipt, &default_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_retreat = LaneHistory::new(state).unwrap();
  let retreat_res = h_retreat
    .append(&receipt, &retreat_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  let mut h_safe_farm = LaneHistory::new(state).unwrap();
  let safe_farm_res = h_safe_farm
    .append(&receipt, &safe_farm_req, inputs(0, 0, LaneWaveResult::Held))
    .unwrap();

  assert_eq!(
    default_res.debrief().fallback_behavior(),
    LaneFallbackBehavior::MaintainPlan
  );
  assert_eq!(
    retreat_res.debrief().fallback_behavior(),
    LaneFallbackBehavior::RetreatToTower
  );
  assert_eq!(
    safe_farm_res.debrief().fallback_behavior(),
    LaneFallbackBehavior::SafeFarm
  );

  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_retreat.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_default.records()[0]),
    lane_record_identity(&h_safe_farm.records()[0])
  );
  assert_ne!(
    lane_record_identity(&h_retreat.records()[0]),
    lane_record_identity(&h_safe_farm.records()[0])
  );

  assert_eq!(h_retreat.verify_replay(), Ok(h_retreat.current_state()));
  assert_eq!(h_safe_farm.verify_replay(), Ok(h_safe_farm.current_state()));
}

#[test]
fn laner_observation_advertises_available_fallback_behaviors() {
  let state = LaneSnapshot::initial();
  let obs = observe_player(&state, ObservationId::new(42)).observation();
  assert_eq!(
    obs.available_fallback_behaviors(),
    [
      LaneFallbackBehavior::MaintainPlan,
      LaneFallbackBehavior::RetreatToTower,
      LaneFallbackBehavior::SafeFarm,
      LaneFallbackBehavior::ConserveResources,
    ]
  );
}
