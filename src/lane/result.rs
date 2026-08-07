use super::evaluation::ResolvedLaneExecution;
use super::projection::{project_lane_effects, project_lane_events};
use super::*;

pub(super) fn build_transition_result(
    state: &LaneSnapshot,
    command: &ValidatedLaneIntent,
    execution: LaneExecutionInputs,
    resolved: ResolvedLaneExecution,
    trace: InputTrace,
) -> LaneTransitionResult {
    let debrief = LaneDebrief {
        decision: LaneDecisionReview::InformationConsistent,
        coordination: LaneCoordinationReview::NotApplicable,
        intent: command.command.intent,
        target_focus: command.command.target_focus,
        commitment: command.command.commitment,
        ping_signal: command.command.ping_signal,
        abort_condition: command.command.abort_condition,
        fallback_behavior: command.command.fallback_behavior,
        self_damage: execution.self_damage,
        resources: execution.resources,
        wave_result: execution.wave_result,
        fallback_activated: resolved.fallback_activated,
        delayed_effects_queued: u8::from(resolved.delayed_effect_queued.is_some()),
        delayed_effects_resolved: resolved.delayed_effects_resolved.len() as u8,
        delayed_effect_origins: LaneDelayedEffectOrigins::from_effects(
            &resolved.delayed_effects_resolved,
        ),
        execution_trace: trace,
    };
    let events = project_lane_events(state, command, execution, &resolved, trace);
    let effects = project_lane_effects(state, command, execution, &resolved, trace);
    LaneTransitionResult {
        next_state: resolved.next_state,
        events,
        effects,
        outcome: resolved.outcome,
        debrief,
        state_hash: resolved.next_state.hash(),
    }
}
