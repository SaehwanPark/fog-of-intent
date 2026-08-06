use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LaneValidationError {
    WrongActor {
        expected: ActorId,
        actual: ActorId,
    },
    WrongTurn {
        expected: Turn,
        actual: Turn,
    },
    WrongRuleset {
        expected: RulesetId,
        actual: RulesetId,
    },
    StaleObservation,
    StateHashMismatch {
        expected: StateHash,
        actual: StateHash,
    },
    InvalidState,
    WindowAlreadyResolved,
    UnsupportedIntent,
}

pub fn validate_lane_request(
    state: &LaneSnapshot,
    receipt: &LaneObservationReceipt,
    request: &LaneIntentRequest,
) -> Result<ValidatedLaneIntent, LaneValidationError> {
    let command = LaneIntentCommand::new_with_full_intent(
        request.actor,
        state.turn,
        M2_LANE_RULESET,
        request.observation_id,
        state.hash(),
        request.intent,
        request.target_focus,
        request.commitment,
        request.ping_signal,
        request.abort_condition,
        request.fallback_behavior,
    );
    validate_lane_command(state, receipt, &command)
}

pub fn validate_lane_command(
    state: &LaneSnapshot,
    receipt: &LaneObservationReceipt,
    command: &LaneIntentCommand,
) -> Result<ValidatedLaneIntent, LaneValidationError> {
    if command.actor != PLAYER_LANER {
        return Err(LaneValidationError::WrongActor {
            expected: PLAYER_LANER,
            actual: command.actor,
        });
    }
    if !state.is_valid_lane_state() {
        return Err(LaneValidationError::InvalidState);
    }
    let observation = receipt.observation;
    if observation.observer != command.actor
        || observation.observation_id != command.observation_id
        || observation.schema != M2_OBSERVATION_SCHEMA
        || observation.turn != state.turn
        || observation.window != state.window
        || receipt.source_state_hash != state.hash()
    {
        return Err(LaneValidationError::StaleObservation);
    }
    if !observation.available_intents.contains(&command.intent)
        && observation.available_threat_response != Some(command.intent)
    {
        return Err(LaneValidationError::UnsupportedIntent);
    }
    if state.phase != LanePhase::Open {
        return Err(LaneValidationError::WindowAlreadyResolved);
    }
    if command.turn != state.turn {
        return Err(LaneValidationError::WrongTurn {
            expected: state.turn,
            actual: command.turn,
        });
    }
    if command.ruleset != M2_LANE_RULESET {
        return Err(LaneValidationError::WrongRuleset {
            expected: M2_LANE_RULESET,
            actual: command.ruleset,
        });
    }
    if state.ruleset != M2_LANE_RULESET {
        return Err(LaneValidationError::WrongRuleset {
            expected: M2_LANE_RULESET,
            actual: state.ruleset,
        });
    }
    let actual_hash = state.hash();
    if command.host_prior_state_hash != actual_hash {
        return Err(LaneValidationError::StateHashMismatch {
            expected: actual_hash,
            actual: command.host_prior_state_hash,
        });
    }
    Ok(ValidatedLaneIntent {
        command: *command,
        validated_snapshot: *state,
    })
}
