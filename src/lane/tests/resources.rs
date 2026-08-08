use super::*;

#[test]
fn retained_resource_bounds_and_overflow_are_enforced() {
  assert!(LaneMana::new(MAX_LANE_MANA).is_ok());
  assert!(LaneMana::new(MAX_LANE_MANA + 1).is_err());
  assert!(LaneGold::new(MAX_LANE_GOLD).is_ok());
  assert!(LaneGold::new(MAX_LANE_GOLD + 1).is_err());
  assert!(LaneExperience::new(MAX_LANE_EXPERIENCE).is_ok());
  assert!(LaneExperience::new(MAX_LANE_EXPERIENCE + 1).is_err());
  assert!(LaneCooldown::new(MAX_LANE_COOLDOWN).is_ok());
  assert!(LaneCooldown::new(MAX_LANE_COOLDOWN + 1).is_err());
}

#[test]
fn retained_resource_additions_are_checked_at_their_bounds() {
  let one_gold = LaneGold::new(1).expect("bounded gold");
  let max_gold = LaneGold::new(MAX_LANE_GOLD).expect("bounded gold");
  assert_eq!(
    LaneGold::new(MAX_LANE_GOLD - 1)
      .expect("bounded gold")
      .add(one_gold),
    Some(max_gold)
  );
  assert_eq!(max_gold.add(one_gold), None);

  let one_experience = LaneExperience::new(1).expect("bounded experience");
  let max_experience = LaneExperience::new(MAX_LANE_EXPERIENCE).expect("bounded experience");
  assert_eq!(
    LaneExperience::new(MAX_LANE_EXPERIENCE - 1)
      .expect("bounded experience")
      .add(one_experience),
    Some(max_experience)
  );
  assert_eq!(max_experience.add(one_experience), None);

  let one_cooldown = LaneCooldown::new(1).expect("bounded cooldown");
  let max_cooldown = LaneCooldown::new(MAX_LANE_COOLDOWN).expect("bounded cooldown");
  assert_eq!(
    LaneCooldown::new(MAX_LANE_COOLDOWN - 1)
      .expect("bounded cooldown")
      .add(one_cooldown),
    Some(max_cooldown)
  );
  assert_eq!(max_cooldown.add(one_cooldown), None);
}

#[test]
fn cooldown_tick_saturates_for_large_u32_values() {
  let cooldown = LaneCooldown::new(7).expect("bounded cooldown");
  assert_eq!(cooldown.tick(1).value(), 6);
  assert_eq!(cooldown.tick(255).value(), 0);
  assert_eq!(cooldown.tick(u32::MAX).value(), 0);
}

#[test]
fn zero_delay_is_rejected_and_non_zero_delay_is_preserved() {
  assert!(LaneDelay::new(0).is_err());
  let delay = LaneDelay::new(2).expect("two beats");
  let effect = LaneDelayedEffect::new(
    delay,
    LaneDelayedEffectKind::SelfHealthRegen {
      amount: LaneHealth::new(1).expect("bounded health"),
    },
  );
  assert_eq!(effect.delay(), delay);
  assert_eq!(effect.delay_beats(), 2);
}

#[test]
fn delayed_effect_remaining_time_fails_closed_at_expiry() {
  let delay = LaneDelay::new(2).expect("two beats");
  assert_eq!(delay.remaining_after(1), LaneDelay::new(1).ok());
  assert_eq!(delay.remaining_after(2), None);
  assert_eq!(delay.remaining_after(u32::MAX), None);
}

#[test]
fn delayed_effect_queue_has_a_bounded_capacity() {
  let effect = LaneDelayedEffect::new(
    LaneDelay::new(1).expect("one beat"),
    LaneDelayedEffectKind::SelfManaRegen {
      amount: LaneMana::new(1).expect("bounded mana"),
    },
  );
  let mut queue = LaneDelayedEffects::empty();
  for _ in 0..MAX_DELAYED_EFFECTS {
    queue.push(effect).expect("queue capacity");
  }
  assert!(queue.push(effect).is_err());
}

#[test]
fn delayed_effects_tick_and_resolve_in_one_and_two_beat_windows() {
  let effect = LaneDelayedEffect::new_with_origin(
    LaneDelay::new(2).expect("two beats"),
    LaneDelayedEffectKind::SelfHealthRegen {
      amount: LaneHealth::new(1).expect("bounded health"),
    },
    trace(9, 9),
  );
  let mut queue = LaneDelayedEffects::empty();
  queue.push(effect).expect("queue effect");
  let state = LaneSnapshot::new_with_delayed_effects(
    M2_LANE_RULESET,
    Turn::new(0),
    LaneWindow::OneBeat,
    LaneStatus::Open,
    LaneSnapshot::initial().player(),
    LaneSnapshot::initial().opponent(),
    LaneSnapshot::initial().wave(),
    LaneSnapshot::initial().jungle_threat(),
    queue,
  );
  let receipt = observe_player(&state, ObservationId::new(1));
  let request = LaneIntentRequest::new(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Yield,
  );
  let validated = validate_lane_request(&state, &receipt, &request).expect("valid request");
  let inputs = LaneResolvedInputs::new(
    trace(1, 1),
    trace(2, 2),
    trace(3, 3),
    trace(4, 4),
    LaneExecutionInputs::new(
      trace(5, 5),
      LaneDamage::zero(),
      LaneDamage::zero(),
      LaneWaveResult::Held,
    ),
  );
  let result = transition_lane(&state, &validated, &inputs).expect("transition");
  assert_eq!(result.debrief().delayed_effects_resolved(), 0);
  assert_eq!(result.next_state().delayed_effects().count(), 1);
  let two_beat_state = LaneSnapshot::new_with_delayed_effects(
    M2_LANE_RULESET,
    Turn::new(0),
    LaneWindow::TwoBeats,
    LaneStatus::Open,
    state.player(),
    state.opponent(),
    state.wave(),
    state.jungle_threat(),
    queue,
  );
  let receipt = observe_player(&two_beat_state, ObservationId::new(1));
  let request = LaneIntentRequest::new(
    PLAYER_LANER,
    receipt.observation().observation_id(),
    LaneIntent::Yield,
  );
  let validated =
    validate_lane_request(&two_beat_state, &receipt, &request).expect("valid request");
  let result = transition_lane(&two_beat_state, &validated, &inputs).expect("transition");
  assert_eq!(result.debrief().delayed_effects_resolved(), 1);
  assert_eq!(
    result.debrief().delayed_effect_origins().origin(0),
    Some(trace(9, 9))
  );
  assert_eq!(result.next_state().delayed_effects().count(), 0);
}

#[test]
fn delayed_effect_origin_trace_is_bound_preserved_and_attributed_on_resolution() {
  let origin = trace(9, 91);
  let resolution = trace(10, 101);
  let input_effect = LaneDelayedEffect::new(
    LaneDelay::new(2).expect("two beats"),
    LaneDelayedEffectKind::SelfHealthRegen {
      amount: LaneHealth::new(1).expect("bounded health"),
    },
  );
  let execution = LaneExecutionInputs::new(
    origin,
    LaneDamage::zero(),
    LaneDamage::zero(),
    LaneWaveResult::Held,
  )
  .with_delayed_effect(input_effect);
  assert_eq!(
    execution.delayed_effect().expect("queued effect").origin(),
    origin
  );

  let state = LaneSnapshot::initial();
  let queue_effect = LaneDelayedEffect::new_with_origin(
    LaneDelay::new(1).expect("one beat"),
    LaneDelayedEffectKind::SelfHealthRegen {
      amount: LaneHealth::new(1).expect("bounded health"),
    },
    origin,
  );
  let mut queue = LaneDelayedEffects::empty();
  queue.push(queue_effect).expect("queue effect");
  let state = LaneSnapshot::new_with_delayed_effects(
    M2_LANE_RULESET,
    Turn::new(0),
    LaneWindow::TwoBeats,
    LaneStatus::Open,
    state.player(),
    state.opponent(),
    state.wave(),
    state.jungle_threat(),
    queue,
  );
  let (receipt, request) = request(&state, LaneIntent::Yield);
  let validated = validate_lane_request(&state, &receipt, &request).expect("valid request");
  let inputs = LaneResolvedInputs::new(
    trace(1, 1),
    trace(2, 2),
    trace(3, 3),
    trace(4, 4),
    LaneExecutionInputs::new(
      resolution,
      LaneDamage::zero(),
      LaneDamage::zero(),
      LaneWaveResult::Held,
    ),
  );
  let result = transition_lane(&state, &validated, &inputs).expect("transition");
  assert!(result.events().iter().any(|event| matches!(
      event,
      LaneEvent::DelayedEffectResolved { effect, trace, .. }
          if *trace == origin && effect.origin() == origin
  )));
  assert!(result.effects().iter().any(|effect| matches!(
      effect,
      LaneEffect::DelayedEffectResolved {
          effect,
          cause: LaneEffectCause::Execution(trace),
          ..
      } if *trace == origin && effect.origin() == origin
  )));
}

#[test]
fn delayed_effect_origin_changes_hash_identity_and_replay_result() {
  let origin_a = trace(9, 91);
  let origin_b = trace(10, 101);
  let effect = LaneDelayedEffect::new(
    LaneDelay::new(2).expect("two beats"),
    LaneDelayedEffectKind::SelfHealthRegen {
      amount: LaneHealth::new(1).expect("bounded health"),
    },
  );
  let state = LaneSnapshot::initial();
  let make_state = |origin| {
    let mut queue = LaneDelayedEffects::empty();
    queue
      .push(effect.with_origin(origin))
      .expect("queue effect");
    LaneSnapshot::new_with_delayed_effects(
      M2_LANE_RULESET,
      Turn::new(0),
      LaneWindow::OneBeat,
      LaneStatus::Open,
      state.player(),
      state.opponent(),
      state.wave(),
      state.jungle_threat(),
      queue,
    )
  };
  assert_ne!(make_state(origin_a).hash(), make_state(origin_b).hash());

  let make_execution = |origin| {
    let bound = LaneExecutionInputs::new(
      trace(5, 5),
      LaneDamage::zero(),
      LaneDamage::zero(),
      LaneWaveResult::Held,
    )
    .with_delayed_effect(effect);
    LaneExecutionInputs {
      delayed_effect: Some(bound.delayed_effect().expect("effect").with_origin(origin)),
      ..bound
    }
  };
  let (receipt, request) = request(&state, LaneIntent::Yield);
  let mut history_a = LaneHistory::new(state).expect("valid history");
  history_a
    .append(
      &receipt,
      &request,
      LaneResolvedInputs::new(
        trace(1, 1),
        trace(2, 2),
        trace(3, 3),
        trace(4, 4),
        make_execution(origin_a),
      ),
    )
    .expect("append A");
  let mut history_b = LaneHistory::new(state).expect("valid history");
  history_b
    .append(
      &receipt,
      &request,
      LaneResolvedInputs::new(
        trace(1, 1),
        trace(2, 2),
        trace(3, 3),
        trace(4, 4),
        make_execution(origin_b),
      ),
    )
    .expect("append B");
  assert_ne!(
    lane_record_identity(&history_a.records[0]),
    lane_record_identity(&history_b.records[0])
  );
  history_a.verify_replay().expect("replay A");
  let mut tampered = history_a;
  tampered.records[0].inputs.execution.delayed_effect = Some(
    tampered.records[0]
      .inputs
      .execution
      .delayed_effect
      .expect("effect")
      .with_origin(origin_b),
  );
  assert!(matches!(
    tampered.verify_replay(),
    Err(LaneReplayError::ResultMismatch { index: 0 })
  ));
}
