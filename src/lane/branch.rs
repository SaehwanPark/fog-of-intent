use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BranchId(pub(crate) u8);

impl BranchId {
    pub fn new(value: u8) -> Result<Self, LaneBranchError> {
        if value <= 127 {
            Ok(Self(value))
        } else {
            Err(LaneBranchError::InvalidBranchId { value })
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BranchExecutionMode {
    MatchedParent,
    Regenerated,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneBranchManaPolicy {
    ParentSpendPreserved,
    NonContestSpendCleared,
    ExplicitExecution,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BranchExecutionSelection {
    MatchedParent {
        source_record: usize,
    },
    Regenerated {
        branch_id: BranchId,
        execution: LaneExecutionInputs,
    },
}

impl BranchExecutionSelection {
    pub fn matched_parent() -> Self {
        Self::MatchedParent { source_record: 0 }
    }

    pub fn regenerated(branch_id: BranchId, execution: LaneExecutionInputs) -> Self {
        Self::Regenerated {
            branch_id,
            execution,
        }
    }

    pub fn mode(self) -> BranchExecutionMode {
        match self {
            Self::MatchedParent { .. } => BranchExecutionMode::MatchedParent,
            Self::Regenerated { .. } => BranchExecutionMode::Regenerated,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneBranchReplayIdentity {
    pub(crate) replay_id: &'static str,
    pub(crate) parent_replay_id: &'static str,
    pub(crate) parent_record_index: usize,
    pub(crate) parent_initial_state_hash: StateHash,
    pub(crate) parent_terminal_state_hash: StateHash,
    pub(crate) parent_record_identity: StateHash,
    pub(crate) branch_id: Option<BranchId>,
    pub(crate) alternate_intent: LaneIntent,
    pub(crate) execution_mode: BranchExecutionMode,
    pub(crate) mana_policy: LaneBranchManaPolicy,
    pub(crate) execution_trace: InputTrace,
}

impl LaneBranchReplayIdentity {
    pub fn replay_id(self) -> &'static str {
        self.replay_id
    }

    pub fn parent_replay_id(self) -> &'static str {
        self.parent_replay_id
    }

    pub fn parent_record_index(self) -> usize {
        self.parent_record_index
    }

    pub fn parent_initial_state_hash(self) -> StateHash {
        self.parent_initial_state_hash
    }

    pub fn parent_terminal_state_hash(self) -> StateHash {
        self.parent_terminal_state_hash
    }

    pub fn parent_record_identity(self) -> StateHash {
        self.parent_record_identity
    }

    pub fn branch_id(self) -> Option<BranchId> {
        self.branch_id
    }

    pub fn alternate_intent(self) -> LaneIntent {
        self.alternate_intent
    }

    pub fn execution_mode(self) -> BranchExecutionMode {
        self.execution_mode
    }

    pub fn mana_policy(self) -> LaneBranchManaPolicy {
        self.mana_policy
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneBranch {
    pub(crate) identity: LaneBranchReplayIdentity,
    pub(crate) execution_selection: BranchExecutionSelection,
    pub(crate) record: LaneTransitionRecord,
}

impl LaneBranch {
    pub fn identity(&self) -> LaneBranchReplayIdentity {
        self.identity
    }

    pub fn execution_selection(&self) -> BranchExecutionSelection {
        self.execution_selection
    }

    pub fn record(&self) -> &LaneTransitionRecord {
        &self.record
    }

    pub fn verify_replay(&self, parent: &LaneHistory) -> Result<(), LaneBranchError> {
        let alternate = LaneIntentRequest::new(
            self.record.command.actor,
            self.record.command.observation_id,
            self.record.command.intent,
        );
        let recomputed = branch_from_window(parent, &alternate, self.execution_selection)
            .map_err(|_| LaneBranchError::BranchReplayMismatch)?;
        if recomputed != *self {
            return Err(LaneBranchError::BranchReplayMismatch);
        }
        Ok(())
    }

    pub fn review(&self, parent: &LaneHistory) -> Result<CounterfactualReview, LaneBranchError> {
        self.verify_replay(parent)?;
        let parent_record = parent
            .records
            .first()
            .ok_or(LaneBranchError::ParentNotExactlyOneWindow)?;
        let (execution_relation, attribution_limit) =
            match (self.identity.execution_mode, self.identity.mana_policy) {
                (
                    BranchExecutionMode::MatchedParent,
                    LaneBranchManaPolicy::ParentSpendPreserved,
                ) => (
                    LaneExecutionRelation::Matched,
                    LaneAttributionLimit::MatchedDecisionOnly,
                ),
                (
                    BranchExecutionMode::MatchedParent,
                    LaneBranchManaPolicy::NonContestSpendCleared,
                ) => (
                    LaneExecutionRelation::MatchedWithResourceNormalization,
                    LaneAttributionLimit::DecisionAndResourceChanged,
                ),
                (BranchExecutionMode::MatchedParent, LaneBranchManaPolicy::ExplicitExecution) => (
                    LaneExecutionRelation::MatchedWithResourceNormalization,
                    LaneAttributionLimit::DecisionAndResourceChanged,
                ),
                (BranchExecutionMode::Regenerated, _) => (
                    LaneExecutionRelation::Regenerated,
                    LaneAttributionLimit::DecisionAndExecutionChanged,
                ),
            };
        Ok(CounterfactualReview {
            parent_outcome: parent_record.result.outcome,
            branch_outcome: self.record.result.outcome,
            parent_intent: parent_record.command.intent,
            branch_intent: self.record.command.intent,
            execution_relation,
            decision_comparison: LaneDecisionReview::InformationConsistent,
            coordination: LaneCoordinationReview::NotApplicable,
            attribution_limit,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneExecutionRelation {
    Matched,
    MatchedWithResourceNormalization,
    Regenerated,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneAttributionLimit {
    MatchedDecisionOnly,
    DecisionAndResourceChanged,
    DecisionAndExecutionChanged,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CounterfactualReview {
    pub(crate) parent_outcome: LaneOutcome,
    pub(crate) branch_outcome: LaneOutcome,
    pub(crate) parent_intent: LaneIntent,
    pub(crate) branch_intent: LaneIntent,
    pub(crate) execution_relation: LaneExecutionRelation,
    pub(crate) decision_comparison: LaneDecisionReview,
    pub(crate) coordination: LaneCoordinationReview,
    pub(crate) attribution_limit: LaneAttributionLimit,
}

impl CounterfactualReview {
    pub fn parent_outcome(self) -> LaneOutcome {
        self.parent_outcome
    }

    pub fn branch_outcome(self) -> LaneOutcome {
        self.branch_outcome
    }

    pub fn parent_intent(self) -> LaneIntent {
        self.parent_intent
    }

    pub fn branch_intent(self) -> LaneIntent {
        self.branch_intent
    }

    pub fn execution_relation(self) -> LaneExecutionRelation {
        self.execution_relation
    }

    pub fn decision_comparison(self) -> LaneDecisionReview {
        self.decision_comparison
    }

    pub fn coordination(self) -> LaneCoordinationReview {
        self.coordination
    }

    pub fn attribution_limit(self) -> LaneAttributionLimit {
        self.attribution_limit
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LaneBranchError {
    ParentNotReplayable(LaneReplayError),
    ParentNotExactlyOneWindow,
    InvalidBranchPoint,
    ObservationMismatch,
    BranchActorMismatch,
    BranchObservationMismatch,
    NotAnAlternateIntent,
    NonExecutionInputsChanged,
    InvalidBranchExecutionIdentity,
    ParentExecutionUnavailable,
    InvalidBranchId { value: u8 },
    Validation(LaneValidationError),
    Transition(LaneTransitionError),
    BranchReplayMismatch,
}

pub fn branch_from_window(
    parent: &LaneHistory,
    alternate: &LaneIntentRequest,
    selection: BranchExecutionSelection,
) -> Result<LaneBranch, LaneBranchError> {
    parent
        .verify_replay()
        .map_err(LaneBranchError::ParentNotReplayable)?;
    if parent.records.len() != 1 {
        return Err(LaneBranchError::ParentNotExactlyOneWindow);
    }
    let parent_record = parent
        .records
        .first()
        .ok_or(LaneBranchError::ParentNotExactlyOneWindow)?;
    let branch_point = parent.initial_state;
    if branch_point.phase != LanePhase::Open
        || parent_record.prior_state_hash != branch_point.hash()
        || parent_record.observation
            != observe_player(&branch_point, parent_record.command.observation_id).observation
    {
        return Err(LaneBranchError::InvalidBranchPoint);
    }
    let receipt = observe_player(&branch_point, parent_record.command.observation_id);
    if receipt.observation != parent_record.observation {
        return Err(LaneBranchError::ObservationMismatch);
    }
    if alternate.actor != PLAYER_LANER {
        return Err(LaneBranchError::BranchActorMismatch);
    }
    if alternate.observation_id != parent_record.command.observation_id {
        return Err(LaneBranchError::BranchObservationMismatch);
    }
    if alternate.intent == parent_record.command.intent {
        return Err(LaneBranchError::NotAnAlternateIntent);
    }
    let validated = validate_lane_request(&branch_point, &receipt, alternate)
        .map_err(LaneBranchError::Validation)?;
    let (inputs, branch_id, execution_mode, mana_policy, execution_trace) = match selection {
        BranchExecutionSelection::MatchedParent { source_record } => {
            if source_record != 0 {
                return Err(LaneBranchError::InvalidBranchPoint);
            }
            let parent_inputs = parent_record.inputs;
            let mana_policy = if parent_inputs.execution.mana_spent != LaneMana::zero()
                && alternate.intent != LaneIntent::Contest
            {
                LaneBranchManaPolicy::NonContestSpendCleared
            } else {
                LaneBranchManaPolicy::ParentSpendPreserved
            };
            let inputs = match mana_policy {
                LaneBranchManaPolicy::ParentSpendPreserved => parent_inputs,
                LaneBranchManaPolicy::NonContestSpendCleared => {
                    parent_inputs.with_mana_spent(LaneMana::zero())
                }
                LaneBranchManaPolicy::ExplicitExecution => unreachable!(),
            };
            (
                inputs,
                None,
                BranchExecutionMode::MatchedParent,
                mana_policy,
                inputs.execution.trace,
            )
        }
        BranchExecutionSelection::Regenerated {
            branch_id,
            execution,
        } => {
            let parent_inputs = parent_record.inputs;
            if execution.trace != branch_execution_trace(branch_id) {
                return Err(LaneBranchError::InvalidBranchExecutionIdentity);
            }
            (
                LaneResolvedInputs::new(
                    parent_inputs.environment,
                    parent_inputs.observation,
                    parent_inputs.policy,
                    parent_inputs.coordination,
                    execution,
                ),
                Some(branch_id),
                BranchExecutionMode::Regenerated,
                LaneBranchManaPolicy::ExplicitExecution,
                execution.trace,
            )
        }
    };
    let result =
        transition_lane(&branch_point, &validated, &inputs).map_err(LaneBranchError::Transition)?;
    let identity = LaneBranchReplayIdentity {
        replay_id: "m2-one-lane-window-branch-v1",
        parent_replay_id: M2_REPLAY_ID,
        parent_record_index: 0,
        parent_initial_state_hash: parent.initial_state.hash(),
        parent_terminal_state_hash: parent.current_state.hash(),
        parent_record_identity: lane_record_identity(parent_record),
        branch_id,
        alternate_intent: alternate.intent,
        execution_mode,
        mana_policy,
        execution_trace,
    };
    Ok(LaneBranch {
        identity,
        execution_selection: selection,
        record: LaneTransitionRecord {
            observation: receipt.observation,
            command: validated.command,
            inputs,
            prior_state_hash: branch_point.hash(),
            result,
        },
    })
}

fn branch_execution_trace(branch_id: BranchId) -> InputTrace {
    InputTrace::new(StreamId::new(128 + branch_id.0), DrawId::new(0))
}

pub(crate) fn lane_record_identity(record: &LaneTransitionRecord) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, &[record.command.actor.value()]);
    hash = hash_bytes(hash, &record.command.turn.value().to_le_bytes());
    hash = hash_bytes(hash, &record.command.ruleset.value().to_le_bytes());
    hash = hash_bytes(hash, &record.command.observation_id.value().to_le_bytes());
    hash = hash_bytes(
        hash,
        &record.command.host_prior_state_hash.value().to_le_bytes(),
    );
    hash = hash_bytes(hash, &[intent_tag(record.command.intent)]);
    if record.command.target_focus != LaneTargetFocus::Minions {
        hash = hash_bytes(hash, &[LANE_TARGET_FOCUS_HASH_TAG]);
        hash = hash_bytes(hash, &[target_focus_tag(record.command.target_focus)]);
    }
    if record.command.commitment != LaneCommitment::Standard {
        hash = hash_bytes(hash, &[LANE_COMMITMENT_HASH_TAG]);
        hash = hash_bytes(hash, &[commitment_tag(record.command.commitment)]);
    }
    if record.command.ping_signal != LanePingSignal::None {
        hash = hash_bytes(hash, &[LANE_PING_SIGNAL_HASH_TAG]);
        hash = hash_bytes(hash, &[ping_signal_tag(record.command.ping_signal)]);
    }
    if record.command.abort_condition != LaneAbortCondition::None {
        hash = hash_bytes(hash, &[LANE_ABORT_CONDITION_HASH_TAG]);
        hash = hash_bytes(hash, &[abort_condition_tag(record.command.abort_condition)]);
    }
    if record.command.fallback_behavior != LaneFallbackBehavior::MaintainPlan {
        hash = hash_bytes(hash, &[LANE_FALLBACK_BEHAVIOR_HASH_TAG]);
        hash = hash_bytes(
            hash,
            &[fallback_behavior_tag(record.command.fallback_behavior)],
        );
    }
    hash = hash_bytes(hash, &record.prior_state_hash.value().to_le_bytes());
    for trace in [
        record.inputs.environment,
        record.inputs.observation,
        record.inputs.policy,
        record.inputs.coordination,
        record.inputs.execution.trace,
    ] {
        hash = hash_bytes(hash, &[trace.stream().value()]);
        hash = hash_bytes(hash, &trace.draw().value().to_le_bytes());
    }
    hash = hash_bytes(hash, &[record.inputs.execution.self_damage.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.opponent_damage.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.mana_spent.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.gold_earned.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.experience_gained.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.cooldown_set.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.bounty_earned.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.level_gained.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.minion_kills_gained.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.shield_gained.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.ward_gained.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.potion_gained.value()]);
    hash = hash_bytes(hash, &[record.inputs.execution.potion_spent.value()]);
    hash = hash_bytes(
        hash,
        &[wave_result_tag(record.inputs.execution.wave_result)],
    );
    StateHash::from_raw(hash)
}
