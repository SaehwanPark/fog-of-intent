use super::evaluation::ResolvedLaneExecution;
use super::*;

pub(super) fn project_lane_events(
  state: &LaneSnapshot,
  command: &ValidatedLaneIntent,
  execution: LaneExecutionInputs,
  resolved: &ResolvedLaneExecution,
  trace: InputTrace,
) -> Vec<LaneEvent> {
  let player = state.player;
  let opponent = state.opponent;
  let mut events = vec![
    LaneEvent::IntentCommitted {
      actor: command.command.actor,
      intent: command.command.intent,
    },
    LaneEvent::TargetFocusSelected {
      actor: command.command.actor,
      focus: command.command.target_focus,
    },
    LaneEvent::CommitmentSelected {
      actor: command.command.actor,
      commitment: command.command.commitment,
    },
    LaneEvent::PingSignalSelected {
      actor: command.command.actor,
      ping_signal: command.command.ping_signal,
    },
    LaneEvent::AbortConditionSelected {
      actor: command.command.actor,
      abort_condition: command.command.abort_condition,
    },
    LaneEvent::FallbackBehaviorSelected {
      actor: command.command.actor,
      fallback_behavior: command.command.fallback_behavior,
    },
  ];
  if command.command.abort_condition != LaneAbortCondition::None {
    events.push(LaneEvent::AbortConditionTriggered {
      actor: command.command.actor,
      abort_condition: command.command.abort_condition,
    });
  }
  if command.command.fallback_behavior != LaneFallbackBehavior::MaintainPlan {
    events.push(LaneEvent::FallbackBehaviorSet {
      actor: command.command.actor,
      fallback_behavior: command.command.fallback_behavior,
    });
  }
  if execution.self_damage != LaneDamage::zero() {
    events.push(LaneEvent::PlayerDamaged {
      target: player.id,
      amount: execution.self_damage,
      trace,
    });
  }
  if execution.opponent_damage != LaneDamage::zero() {
    events.push(LaneEvent::OpponentDamaged {
      target: opponent.id,
      amount: execution.opponent_damage,
      trace,
    });
  }
  if execution.mana_spent() != LaneMana::zero() {
    events.push(LaneEvent::ManaSpent {
      actor: player.id,
      amount: execution.mana_spent(),
      trace,
    });
  }
  if execution.gold_earned() != LaneGold::zero() {
    events.push(LaneEvent::GoldEarned {
      actor: player.id,
      amount: execution.gold_earned(),
      trace,
    });
  }
  if execution.experience_gained() != LaneExperience::zero() {
    events.push(LaneEvent::ExperienceGained {
      actor: player.id,
      amount: execution.experience_gained(),
      trace,
    });
  }
  let next_player = resolved.next_state.player;
  if next_player.cooldown() != player.cooldown() {
    if execution.cooldown_set() != LaneCooldown::zero() {
      events.push(LaneEvent::CooldownSet {
        actor: player.id,
        amount: execution.cooldown_set(),
        trace,
      });
    } else {
      events.push(LaneEvent::CooldownTicked {
        actor: player.id,
        amount: state.window.beats(),
        trace,
      });
    }
  }
  if let Some(queued) = resolved.delayed_effect_queued {
    events.push(LaneEvent::DelayedEffectQueued {
      actor: player.id,
      effect: queued,
      trace,
    });
  }
  for item in &resolved.delayed_effects_resolved {
    events.push(LaneEvent::DelayedEffectResolved {
      actor: player.id,
      effect: *item,
      trace: item.origin(),
    });
  }
  events.push(LaneEvent::WaveResolved {
    before: state.wave.pressure,
    after: resolved.next_state.wave.pressure,
    trace,
  });
  if resolved.fallback_activated {
    events.push(LaneEvent::FallbackActivated {
      actor: player.id,
      intent: command.command.intent,
    });
    if command.command.fallback_behavior != LaneFallbackBehavior::MaintainPlan {
      events.push(LaneEvent::FallbackBehaviorTriggered {
        actor: player.id,
        fallback_behavior: command.command.fallback_behavior,
      });
    }
  }
  events.push(LaneEvent::WindowResolved {
    outcome: resolved.outcome,
  });
  events
}

pub(super) fn project_lane_effects(
  state: &LaneSnapshot,
  command: &ValidatedLaneIntent,
  execution: LaneExecutionInputs,
  resolved: &ResolvedLaneExecution,
  trace: InputTrace,
) -> Vec<LaneEffect> {
  let player = state.player;
  let opponent = state.opponent;
  let next_state = resolved.next_state;
  let next_player = next_state.player;
  let mut effects = vec![
    LaneEffect::TargetFocusSet {
      actor: player.id,
      focus: command.command.target_focus,
      cause: LaneEffectCause::Intent,
      provenance: LaneEffectProvenance::direct_immediate(),
    },
    LaneEffect::CommitmentSet {
      actor: player.id,
      commitment: command.command.commitment,
      cause: LaneEffectCause::Intent,
      provenance: LaneEffectProvenance::direct_immediate(),
    },
    LaneEffect::PingSignalSet {
      actor: player.id,
      ping_signal: command.command.ping_signal,
      cause: LaneEffectCause::Intent,
      provenance: LaneEffectProvenance::direct_immediate(),
    },
    LaneEffect::AbortConditionSet {
      actor: player.id,
      abort_condition: command.command.abort_condition,
      cause: LaneEffectCause::Intent,
      provenance: LaneEffectProvenance::direct_immediate(),
    },
    LaneEffect::FallbackBehaviorSet {
      actor: player.id,
      fallback_behavior: command.command.fallback_behavior,
      cause: LaneEffectCause::Intent,
      provenance: LaneEffectProvenance::direct_immediate(),
    },
  ];
  if execution.self_damage != LaneDamage::zero() {
    effects.push(LaneEffect::HealthChanged {
      actor: player.id,
      before: player.health,
      after: next_player.health,
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if execution.opponent_damage != LaneDamage::zero() {
    effects.push(LaneEffect::HealthChanged {
      actor: opponent.id,
      before: opponent.health,
      after: next_state.opponent.health,
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if execution.mana_spent() != LaneMana::zero() {
    effects.push(LaneEffect::ManaChanged {
      actor: player.id,
      before: player.mana(),
      after: next_player.mana(),
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if execution.gold_earned() != LaneGold::zero() {
    effects.push(LaneEffect::GoldChanged {
      actor: player.id,
      before: player.gold(),
      after: next_player.gold(),
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if execution.experience_gained() != LaneExperience::zero() {
    effects.push(LaneEffect::ExperienceChanged {
      actor: player.id,
      before: player.experience(),
      after: next_player.experience(),
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if next_player.cooldown() != player.cooldown() {
    effects.push(LaneEffect::CooldownChanged {
      actor: player.id,
      before: player.cooldown(),
      after: next_player.cooldown(),
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if let Some(queued) = resolved.delayed_effect_queued {
    effects.push(LaneEffect::DelayedEffectQueued {
      actor: player.id,
      effect: queued,
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  for item in &resolved.delayed_effects_resolved {
    effects.push(LaneEffect::DelayedEffectResolved {
      actor: player.id,
      effect: *item,
      cause: LaneEffectCause::Execution(item.origin()),
      provenance: LaneEffectProvenance::direct_delayed(),
    });
  }
  if next_state.wave.pressure != state.wave.pressure {
    effects.push(LaneEffect::WavePressureChanged {
      before: state.wave.pressure,
      after: next_state.wave.pressure,
      cause: LaneEffectCause::Execution(trace),
      provenance: LaneEffectProvenance::direct_immediate(),
    });
  }
  if next_player.position != player.position {
    let cause = if resolved.fallback_activated {
      LaneEffectCause::Fallback
    } else {
      LaneEffectCause::Intent
    };
    let provenance = if resolved.fallback_activated {
      LaneEffectProvenance::indirect_immediate()
    } else {
      LaneEffectProvenance::direct_immediate()
    };
    effects.push(LaneEffect::PositionChanged {
      actor: player.id,
      before: player.position,
      after: next_player.position,
      cause,
      provenance,
    });
  }
  effects
}
