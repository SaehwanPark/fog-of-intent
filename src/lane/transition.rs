use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneWaveResult {
    Advanced,
    Held,
    Lost,
}

/// Resource deltas resolved at the execution boundary for one lane window.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneResourceInputs {
    pub(crate) mana_spent: LaneMana,
    pub(crate) gold_earned: LaneGold,
    pub(crate) experience_gained: LaneExperience,
    pub(crate) cooldown_set: LaneCooldown,
}

impl Default for LaneResourceInputs {
    fn default() -> Self {
        Self::zero()
    }
}

impl LaneResourceInputs {
    pub const fn zero() -> Self {
        Self {
            mana_spent: LaneMana::zero(),
            gold_earned: LaneGold::zero(),
            experience_gained: LaneExperience::zero(),
            cooldown_set: LaneCooldown::zero(),
        }
    }

    pub const fn new(
        mana_spent: LaneMana,
        gold_earned: LaneGold,
        experience_gained: LaneExperience,
        cooldown_set: LaneCooldown,
    ) -> Self {
        Self {
            mana_spent,
            gold_earned,
            experience_gained,
            cooldown_set,
        }
    }

    pub const fn mana_spent(self) -> LaneMana {
        self.mana_spent
    }

    pub const fn gold_earned(self) -> LaneGold {
        self.gold_earned
    }

    pub const fn experience_gained(self) -> LaneExperience {
        self.experience_gained
    }

    pub const fn cooldown_set(self) -> LaneCooldown {
        self.cooldown_set
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneExecutionInputs {
    pub(crate) trace: InputTrace,
    pub(crate) self_damage: LaneDamage,
    pub(crate) opponent_damage: LaneDamage,
    pub(crate) wave_result: LaneWaveResult,
    pub(crate) resources: LaneResourceInputs,
    pub(crate) delayed_effect: Option<LaneDelayedEffect>,
}

impl LaneExecutionInputs {
    pub const fn new(
        trace: InputTrace,
        self_damage: LaneDamage,
        opponent_damage: LaneDamage,
        wave_result: LaneWaveResult,
    ) -> Self {
        Self {
            trace,
            self_damage,
            opponent_damage,
            wave_result,
            resources: LaneResourceInputs::zero(),
            delayed_effect: None,
        }
    }

    pub const fn with_resource_inputs(mut self, resources: LaneResourceInputs) -> Self {
        self.resources = resources;
        self
    }

    pub const fn resource_inputs(self) -> LaneResourceInputs {
        self.resources
    }

    pub const fn with_delayed_effect(mut self, delayed_effect: LaneDelayedEffect) -> Self {
        self.delayed_effect = Some(delayed_effect);
        self
    }

    pub const fn delayed_effect(self) -> Option<LaneDelayedEffect> {
        self.delayed_effect
    }

    pub const fn with_mana_spent(mut self, mana_spent: LaneMana) -> Self {
        self.resources.mana_spent = mana_spent;
        self
    }

    pub const fn with_gold_earned(mut self, gold_earned: LaneGold) -> Self {
        self.resources.gold_earned = gold_earned;
        self
    }

    pub const fn with_experience_gained(mut self, experience_gained: LaneExperience) -> Self {
        self.resources.experience_gained = experience_gained;
        self
    }

    pub const fn with_cooldown_set(mut self, cooldown_set: LaneCooldown) -> Self {
        self.resources.cooldown_set = cooldown_set;
        self
    }

    pub const fn trace(self) -> InputTrace {
        self.trace
    }

    pub const fn self_damage(self) -> LaneDamage {
        self.self_damage
    }

    pub const fn opponent_damage(self) -> LaneDamage {
        self.opponent_damage
    }

    pub const fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub const fn mana_spent(self) -> LaneMana {
        self.resources.mana_spent()
    }

    pub const fn gold_earned(self) -> LaneGold {
        self.resources.gold_earned()
    }

    pub const fn experience_gained(self) -> LaneExperience {
        self.resources.experience_gained()
    }

    pub const fn cooldown_set(self) -> LaneCooldown {
        self.resources.cooldown_set()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneResolvedInputs {
    pub(crate) environment: InputTrace,
    pub(crate) observation: InputTrace,
    pub(crate) policy: InputTrace,
    pub(crate) coordination: InputTrace,
    pub(crate) execution: LaneExecutionInputs,
}

impl LaneResolvedInputs {
    pub const fn new(
        environment: InputTrace,
        observation: InputTrace,
        policy: InputTrace,
        coordination: InputTrace,
        execution: LaneExecutionInputs,
    ) -> Self {
        Self {
            environment,
            observation,
            policy,
            coordination,
            execution,
        }
    }

    pub const fn execution(self) -> LaneExecutionInputs {
        self.execution
    }

    pub const fn with_resource_inputs(mut self, resources: LaneResourceInputs) -> Self {
        self.execution = self.execution.with_resource_inputs(resources);
        self
    }

    pub const fn with_mana_spent(mut self, mana_spent: LaneMana) -> Self {
        self.execution = self.execution.with_mana_spent(mana_spent);
        self
    }

    pub const fn environment(self) -> InputTrace {
        self.environment
    }

    pub const fn observation(self) -> InputTrace {
        self.observation
    }

    pub const fn policy(self) -> InputTrace {
        self.policy
    }

    pub const fn coordination(self) -> InputTrace {
        self.coordination
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneEffectCause {
    Intent,
    Fallback,
    Execution(InputTrace),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneEffectRelation {
    Direct,
    Indirect,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneEffectTiming {
    Immediate,
    Delayed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneEffectProvenance {
    pub(crate) relation: LaneEffectRelation,
    pub(crate) timing: LaneEffectTiming,
}

impl LaneEffectProvenance {
    pub const fn direct_immediate() -> Self {
        Self {
            relation: LaneEffectRelation::Direct,
            timing: LaneEffectTiming::Immediate,
        }
    }

    pub const fn indirect_immediate() -> Self {
        Self {
            relation: LaneEffectRelation::Indirect,
            timing: LaneEffectTiming::Immediate,
        }
    }

    pub const fn direct_delayed() -> Self {
        Self {
            relation: LaneEffectRelation::Direct,
            timing: LaneEffectTiming::Delayed,
        }
    }

    pub const fn indirect_delayed() -> Self {
        Self {
            relation: LaneEffectRelation::Indirect,
            timing: LaneEffectTiming::Delayed,
        }
    }

    pub const fn relation(self) -> LaneEffectRelation {
        self.relation
    }

    pub const fn timing(self) -> LaneEffectTiming {
        self.timing
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneEvent {
    IntentCommitted {
        actor: ActorId,
        intent: LaneIntent,
    },
    TargetFocusSelected {
        actor: ActorId,
        focus: LaneTargetFocus,
    },
    CommitmentSelected {
        actor: ActorId,
        commitment: LaneCommitment,
    },
    PingSignalSelected {
        actor: ActorId,
        ping_signal: LanePingSignal,
    },
    AbortConditionSelected {
        actor: ActorId,
        abort_condition: LaneAbortCondition,
    },
    AbortConditionTriggered {
        actor: ActorId,
        abort_condition: LaneAbortCondition,
    },
    FallbackBehaviorSelected {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
    },
    FallbackBehaviorSet {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
    },
    FallbackBehaviorTriggered {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
    },
    PlayerDamaged {
        target: ActorId,
        amount: LaneDamage,
        trace: InputTrace,
    },
    OpponentDamaged {
        target: ActorId,
        amount: LaneDamage,
        trace: InputTrace,
    },
    ManaSpent {
        actor: ActorId,
        amount: LaneMana,
        trace: InputTrace,
    },
    GoldEarned {
        actor: ActorId,
        amount: LaneGold,
        trace: InputTrace,
    },
    ExperienceGained {
        actor: ActorId,
        amount: LaneExperience,
        trace: InputTrace,
    },
    CooldownTicked {
        actor: ActorId,
        amount: u32,
        trace: InputTrace,
    },
    CooldownSet {
        actor: ActorId,
        amount: LaneCooldown,
        trace: InputTrace,
    },
    DelayedEffectQueued {
        actor: ActorId,
        effect: LaneDelayedEffect,
        trace: InputTrace,
    },
    DelayedEffectResolved {
        actor: ActorId,
        effect: LaneDelayedEffect,
        trace: InputTrace,
    },
    WaveResolved {
        before: WavePressure,
        after: WavePressure,
        trace: InputTrace,
    },
    FallbackActivated {
        actor: ActorId,
        intent: LaneIntent,
    },
    WindowResolved {
        outcome: LaneOutcome,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneEffect {
    HealthChanged {
        actor: ActorId,
        before: LaneHealth,
        after: LaneHealth,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    WavePressureChanged {
        before: WavePressure,
        after: WavePressure,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    ManaChanged {
        actor: ActorId,
        before: LaneMana,
        after: LaneMana,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    GoldChanged {
        actor: ActorId,
        before: LaneGold,
        after: LaneGold,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    ExperienceChanged {
        actor: ActorId,
        before: LaneExperience,
        after: LaneExperience,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    CooldownChanged {
        actor: ActorId,
        before: LaneCooldown,
        after: LaneCooldown,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    PositionChanged {
        actor: ActorId,
        before: LanePosition,
        after: LanePosition,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    DelayedEffectQueued {
        actor: ActorId,
        effect: LaneDelayedEffect,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    DelayedEffectResolved {
        actor: ActorId,
        effect: LaneDelayedEffect,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    TargetFocusSet {
        actor: ActorId,
        focus: LaneTargetFocus,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    CommitmentSet {
        actor: ActorId,
        commitment: LaneCommitment,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    PingSignalSet {
        actor: ActorId,
        ping_signal: LanePingSignal,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    AbortConditionSet {
        actor: ActorId,
        abort_condition: LaneAbortCondition,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    FallbackBehaviorSet {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
}

impl LaneEffect {
    pub const fn provenance(self) -> LaneEffectProvenance {
        match self {
            Self::HealthChanged { provenance, .. }
            | Self::WavePressureChanged { provenance, .. }
            | Self::ManaChanged { provenance, .. }
            | Self::GoldChanged { provenance, .. }
            | Self::ExperienceChanged { provenance, .. }
            | Self::CooldownChanged { provenance, .. }
            | Self::PositionChanged { provenance, .. }
            | Self::DelayedEffectQueued { provenance, .. }
            | Self::DelayedEffectResolved { provenance, .. }
            | Self::TargetFocusSet { provenance, .. }
            | Self::CommitmentSet { provenance, .. }
            | Self::PingSignalSet { provenance, .. }
            | Self::AbortConditionSet { provenance, .. }
            | Self::FallbackBehaviorSet { provenance, .. } => provenance,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LaneExecutionError {
    SelfDamageExceedsHealth {
        damage: LaneDamage,
        health: LaneHealth,
    },
    OpponentDamageExceedsHealth {
        damage: LaneDamage,
        health: LaneHealth,
    },
    WaveOverflow {
        pressure: WavePressure,
    },
    WaveUnderflow {
        pressure: WavePressure,
    },
    ManaSpentWithoutContest {
        intent: LaneIntent,
        spent: LaneMana,
    },
    ManaExceedsAvailable {
        spent: LaneMana,
        available: LaneMana,
    },
    GoldOverflow {
        earned: LaneGold,
        current: LaneGold,
    },
    ExperienceOverflow {
        gained: LaneExperience,
        current: LaneExperience,
    },
    CooldownOverflow {
        set: LaneCooldown,
        current: LaneCooldown,
    },
    DelayedEffectOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LaneTransitionError {
    StaleValidation {
        expected: StateHash,
        actual: StateHash,
    },
    WrongPhase,
    Execution(LaneExecutionError),
    TurnOverflow,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneCoordinationReview {
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneDecisionReview {
    InformationConsistent,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDebrief {
    pub(crate) decision: LaneDecisionReview,
    pub(crate) coordination: LaneCoordinationReview,
    pub(crate) intent: LaneIntent,
    pub(crate) target_focus: LaneTargetFocus,
    pub(crate) commitment: LaneCommitment,
    pub(crate) ping_signal: LanePingSignal,
    pub(crate) abort_condition: LaneAbortCondition,
    pub(crate) fallback_behavior: LaneFallbackBehavior,
    pub(crate) self_damage: LaneDamage,
    pub(crate) resources: LaneResourceInputs,
    pub(crate) wave_result: LaneWaveResult,
    pub(crate) fallback_activated: bool,
    pub(crate) delayed_effects_queued: u8,
    pub(crate) delayed_effects_resolved: u8,
    pub(crate) execution_trace: InputTrace,
}

impl LaneDebrief {
    pub const fn decision(self) -> LaneDecisionReview {
        self.decision
    }

    pub const fn coordination(self) -> LaneCoordinationReview {
        self.coordination
    }

    pub const fn intent(self) -> LaneIntent {
        self.intent
    }

    pub const fn target_focus(self) -> LaneTargetFocus {
        self.target_focus
    }

    pub const fn commitment(self) -> LaneCommitment {
        self.commitment
    }

    pub const fn ping_signal(self) -> LanePingSignal {
        self.ping_signal
    }

    pub const fn abort_condition(self) -> LaneAbortCondition {
        self.abort_condition
    }

    pub const fn fallback_behavior(self) -> LaneFallbackBehavior {
        self.fallback_behavior
    }

    pub const fn self_damage(self) -> LaneDamage {
        self.self_damage
    }

    pub const fn resource_inputs(self) -> LaneResourceInputs {
        self.resources
    }

    pub const fn mana_spent(self) -> LaneMana {
        self.resources.mana_spent()
    }

    pub const fn gold_earned(self) -> LaneGold {
        self.resources.gold_earned()
    }

    pub const fn experience_gained(self) -> LaneExperience {
        self.resources.experience_gained()
    }

    pub const fn cooldown_set(self) -> LaneCooldown {
        self.resources.cooldown_set()
    }

    pub const fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub const fn fallback_activated(self) -> bool {
        self.fallback_activated
    }

    pub const fn delayed_effects_queued(self) -> u8 {
        self.delayed_effects_queued
    }

    pub const fn delayed_effects_resolved(self) -> u8 {
        self.delayed_effects_resolved
    }

    pub const fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneTransitionResult {
    pub(crate) next_state: LaneSnapshot,
    pub(crate) events: Vec<LaneEvent>,
    pub(crate) effects: Vec<LaneEffect>,
    pub(crate) outcome: LaneOutcome,
    pub(crate) debrief: LaneDebrief,
    pub(crate) state_hash: StateHash,
}

impl LaneTransitionResult {
    pub fn next_state(&self) -> LaneSnapshot {
        self.next_state
    }

    pub fn events(&self) -> &[LaneEvent] {
        &self.events
    }

    pub fn effects(&self) -> &[LaneEffect] {
        &self.effects
    }

    pub const fn outcome(&self) -> LaneOutcome {
        self.outcome
    }

    pub const fn debrief(&self) -> LaneDebrief {
        self.debrief
    }

    pub const fn state_hash(&self) -> StateHash {
        self.state_hash
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinatedEvent {
    ProposalOffered {
        proposal_id: ProposalId,
        proposer: ActorId,
        target: ActorId,
    },
    ProposalResponded {
        proposal_id: ProposalId,
        response: ProposalResponse,
    },
    CoordinationResolved {
        proposal_id: ProposalId,
        disposition: CoordinationDisposition,
        trace: InputTrace,
    },
    Lane(LaneEvent),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinatedEffect {
    SupportCommitted {
        proposal_id: ProposalId,
        proposer: ActorId,
        target: ActorId,
        support: AlliedSupport,
        cause: InputTrace,
    },
    SupportUnavailable {
        proposal_id: ProposalId,
        disposition: CoordinationDisposition,
        cause: InputTrace,
    },
    Lane(LaneEffect),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinatedDecisionReview {
    InformationConsistent,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinatedResponseReview {
    Accepted,
    Rejected,
    Countered,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinatedExecutionReview {
    ConditionalOnCoordination { trace: InputTrace },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinatedLuckReview {
    ExplicitExecutionInput { trace: InputTrace },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinatedDebrief {
    pub(crate) decision: CoordinatedDecisionReview,
    pub(crate) response: CoordinatedResponseReview,
    pub(crate) coordination: CoordinationDisposition,
    pub(crate) execution: CoordinatedExecutionReview,
    pub(crate) luck: CoordinatedLuckReview,
}

impl CoordinatedDebrief {
    pub const fn decision(self) -> CoordinatedDecisionReview {
        self.decision
    }

    pub const fn response(self) -> CoordinatedResponseReview {
        self.response
    }

    pub const fn coordination(self) -> CoordinationDisposition {
        self.coordination
    }

    pub const fn execution(self) -> CoordinatedExecutionReview {
        self.execution
    }

    pub const fn luck(self) -> CoordinatedLuckReview {
        self.luck
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatedTransitionResult {
    pub(crate) lane: LaneTransitionResult,
    pub(crate) coordination: CoordinationResolution,
    pub(crate) events: Vec<CoordinatedEvent>,
    pub(crate) effects: Vec<CoordinatedEffect>,
    pub(crate) debrief: CoordinatedDebrief,
}

impl CoordinatedTransitionResult {
    pub fn lane(&self) -> &LaneTransitionResult {
        &self.lane
    }

    pub const fn coordination(&self) -> CoordinationResolution {
        self.coordination
    }

    pub fn events(&self) -> &[CoordinatedEvent] {
        &self.events
    }

    pub fn effects(&self) -> &[CoordinatedEffect] {
        &self.effects
    }

    pub const fn debrief(&self) -> CoordinatedDebrief {
        self.debrief
    }

    pub fn next_state(&self) -> LaneSnapshot {
        self.lane.next_state()
    }

    pub const fn state_hash(&self) -> StateHash {
        self.lane.state_hash()
    }
}

use super::evaluation::resolve_lane_execution;
use super::result::build_transition_result;

pub fn transition_lane(
    state: &LaneSnapshot,
    command: &ValidatedLaneIntent,
    inputs: &LaneResolvedInputs,
) -> Result<LaneTransitionResult, LaneTransitionError> {
    if command.validated_snapshot != *state {
        return Err(LaneTransitionError::StaleValidation {
            expected: command.validated_snapshot.hash(),
            actual: state.hash(),
        });
    }
    if state.status != LaneStatus::Open {
        return Err(LaneTransitionError::WrongPhase);
    }
    let execution = inputs.execution;
    let resolved = resolve_lane_execution(state, command, execution)?;
    let trace = execution.trace;
    Ok(build_transition_result(
        state, command, execution, resolved, trace,
    ))
}
