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
    let effect = LaneDelayedEffect::new(
        LaneDelay::new(2).expect("two beats"),
        LaneDelayedEffectKind::SelfHealthRegen {
            amount: LaneHealth::new(1).expect("bounded health"),
        },
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
    let validated = validate_lane_request(&two_beat_state, &receipt, &request).expect("valid request");
    let result = transition_lane(&two_beat_state, &validated, &inputs).expect("transition");
    assert_eq!(result.debrief().delayed_effects_resolved(), 1);
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
    assert_eq!(execution.delayed_effect().expect("queued effect").origin(), origin);

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
