use super::*;

fn advance_wave(pressure: WavePressure) -> Result<WavePressure, LaneExecutionError> {
    WavePressure::new(pressure.value() + 1)
        .map_err(|_| LaneExecutionError::WaveOverflow { pressure })
}

fn lose_wave(pressure: WavePressure) -> Result<WavePressure, LaneExecutionError> {
    pressure
        .value()
        .checked_sub(1)
        .map(WavePressure)
        .ok_or(LaneExecutionError::WaveUnderflow { pressure })
}

fn apply_player_resources(
    before: LaneResources,
    deltas: LaneResourceInputs,
    window_beats: u32,
    intent: LaneIntent,
) -> Result<LaneResources, LaneExecutionError> {
    if deltas.mana_spent != LaneMana::zero() && intent != LaneIntent::Contest {
        return Err(LaneExecutionError::ManaSpentWithoutContest {
            intent,
            spent: deltas.mana_spent,
        });
    }
    let mana = before.mana().subtract(deltas.mana_spent).ok_or(
        LaneExecutionError::ManaExceedsAvailable {
            spent: deltas.mana_spent,
            available: before.mana(),
        },
    )?;
    let gold = before
        .gold()
        .add(deltas.gold_earned)
        .ok_or(LaneExecutionError::GoldOverflow {
            earned: deltas.gold_earned,
            current: before.gold(),
        })?;
    let experience = before.experience().add(deltas.experience_gained).ok_or(
        LaneExecutionError::ExperienceOverflow {
            gained: deltas.experience_gained,
            current: before.experience(),
        },
    )?;
    let ticked_cooldown = before.cooldown().tick(window_beats);
    let cooldown =
        ticked_cooldown
            .add(deltas.cooldown_set)
            .ok_or(LaneExecutionError::CooldownOverflow {
                set: deltas.cooldown_set,
                current: ticked_cooldown,
            })?;
    Ok(LaneResources::new(mana, gold, experience, cooldown))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ResolvedLaneExecution {
    pub(super) next_state: LaneSnapshot,
    pub(super) outcome: LaneOutcome,
    pub(super) fallback_activated: bool,
    pub(super) delayed_effects_resolved: Vec<LaneDelayedEffect>,
    pub(super) delayed_effect_queued: Option<LaneDelayedEffect>,
}

pub(super) fn resolve_lane_execution(
    state: &LaneSnapshot,
    command: &ValidatedLaneIntent,
    execution: LaneExecutionInputs,
) -> Result<ResolvedLaneExecution, LaneTransitionError> {
    let player = state.player;
    let opponent = state.opponent;
    if execution.self_damage.0 > player.health.0 {
        return Err(LaneTransitionError::Execution(
            LaneExecutionError::SelfDamageExceedsHealth {
                damage: execution.self_damage,
                health: player.health,
            },
        ));
    }
    if execution.opponent_damage.0 > opponent.health.0 {
        return Err(LaneTransitionError::Execution(
            LaneExecutionError::OpponentDamageExceedsHealth {
                damage: execution.opponent_damage,
                health: opponent.health,
            },
        ));
    }
    let mut after_resources = apply_player_resources(
        player.resources(),
        execution.resources,
        state.window.beats(),
        command.command.intent,
    )
    .map_err(LaneTransitionError::Execution)?;
    let after_wave = match execution.wave_result {
        LaneWaveResult::Advanced => advance_wave(state.wave.pressure),
        LaneWaveResult::Held => Ok(state.wave.pressure),
        LaneWaveResult::Lost => lose_wave(state.wave.pressure),
    }
    .map_err(LaneTransitionError::Execution)?;
    let mut after_player_health = player
        .health
        .subtract(execution.self_damage)
        .expect("validated damage must be subtractable");
    let after_opponent_health = opponent
        .health
        .subtract(execution.opponent_damage)
        .expect("validated damage must be subtractable");

    let beats = state.window.beats();
    let mut next_delayed_effects = LaneDelayedEffects::empty();
    let mut delayed_effects_resolved = Vec::new();
    for effect in state.delayed_effects.items().iter().flatten() {
        if u32::from(effect.delay_beats()) <= beats {
            delayed_effects_resolved.push(*effect);
            match effect.kind() {
                LaneDelayedEffectKind::SelfHealthRegen { amount } => {
                    let value = (after_player_health.0 + amount.0).min(MAX_LANE_HEALTH);
                    after_player_health = LaneHealth::new(value).expect("bounded health");
                }
                LaneDelayedEffectKind::SelfManaRegen { amount } => {
                    let mana = after_resources
                        .mana()
                        .add(amount)
                        .unwrap_or(LaneMana::full());
                    after_resources = LaneResources::new(
                        mana,
                        after_resources.gold(),
                        after_resources.experience(),
                        after_resources.cooldown(),
                    );
                }
                LaneDelayedEffectKind::SelfCooldownReduction { amount } => {
                    let cooldown = after_resources.cooldown().tick(u32::from(amount.value()));
                    after_resources = LaneResources::new(
                        after_resources.mana(),
                        after_resources.gold(),
                        after_resources.experience(),
                        cooldown,
                    );
                }
            }
        } else {
            let remaining = LaneDelay::new(effect.delay_beats() - beats as u8)
                .expect("remaining delay is non-zero");
            next_delayed_effects
                .push(LaneDelayedEffect::new_with_origin(
                    remaining,
                    effect.kind(),
                    effect.origin(),
                ))
                .expect("valid queue bounds");
        }
    }
    let mut delayed_effect_queued = None;
    if let Some(new_delayed) = execution.delayed_effect {
        next_delayed_effects.push(new_delayed).map_err(|_| {
            LaneTransitionError::Execution(LaneExecutionError::DelayedEffectOverflow)
        })?;
        delayed_effect_queued = Some(new_delayed);
    }

    let fallback_activated =
        command.command.intent == LaneIntent::Contest && execution.self_damage.0 >= 2;
    let after_position = match command.command.intent {
        LaneIntent::Stabilize => LanePosition::NearTower,
        LaneIntent::Contest if fallback_activated => LanePosition::NearTower,
        LaneIntent::Contest => LanePosition::Center,
        LaneIntent::Yield | LaneIntent::Recall | LaneIntent::Withdraw => LanePosition::NearTower,
    };
    let outcome = if after_player_health == LaneHealth::zero() {
        LaneOutcome::ForcedOut
    } else if after_position == LanePosition::NearTower {
        LaneOutcome::YieldedSpace
    } else {
        LaneOutcome::HeldSpace
    };
    let next_turn = state
        .turn
        .value()
        .checked_add(state.window.beats())
        .ok_or(LaneTransitionError::TurnOverflow)?;
    let next_player = PlayerLaneState::from_resources(
        player.id,
        after_player_health,
        after_resources,
        after_position,
    );
    let next_opponent = OpponentTruth::new(
        opponent.id,
        after_opponent_health,
        opponent.position,
        opponent.posture,
    );
    let next_state = LaneSnapshot::new_with_delayed_effects(
        state.ruleset,
        Turn::new(next_turn),
        state.window,
        LaneStatus::Resolved(outcome),
        next_player,
        next_opponent,
        WaveState::new(after_wave),
        state.jungle_threat,
        next_delayed_effects,
    );
    Ok(ResolvedLaneExecution {
        next_state,
        outcome,
        fallback_activated,
        delayed_effects_resolved,
        delayed_effect_queued,
    })
}
