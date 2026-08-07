use super::*;

pub const M2_OBJECTIVE_SCHEMA: &str = "m2-terminal-objective-v1";
pub const M2_HOLD_LANE_GOAL_ID: &str = "m2-hold-lane-space-v1";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScenarioGoal {
    HoldLaneSpaceThroughWindow,
}

impl ScenarioGoal {
    pub fn goal_id(self) -> &'static str {
        match self {
            Self::HoldLaneSpaceThroughWindow => M2_HOLD_LANE_GOAL_ID,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectiveCoordination {
    NotApplicable,
    Resolved(CoordinationDisposition),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectiveEvaluationInputs {
    pub(crate) replay_id: &'static str,
    pub(crate) prior_state_hash: StateHash,
    pub(crate) terminal_state_hash: StateHash,
    pub(crate) outcome: LaneOutcome,
    pub(crate) player_position: LanePosition,
    pub(crate) player_health: LaneHealth,
    pub(crate) intent: LaneIntent,
    pub(crate) wave_result: LaneWaveResult,
    pub(crate) coordination: ObjectiveCoordination,
    pub(crate) execution_trace: InputTrace,
}

impl ObjectiveEvaluationInputs {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        replay_id: &'static str,
        prior_state_hash: StateHash,
        terminal_state_hash: StateHash,
        outcome: LaneOutcome,
        player_position: LanePosition,
        player_health: LaneHealth,
        intent: LaneIntent,
        wave_result: LaneWaveResult,
        coordination: ObjectiveCoordination,
        execution_trace: InputTrace,
    ) -> Self {
        Self {
            replay_id,
            prior_state_hash,
            terminal_state_hash,
            outcome,
            player_position,
            player_health,
            intent,
            wave_result,
            coordination,
            execution_trace,
        }
    }

    pub fn replay_id(self) -> &'static str {
        self.replay_id
    }

    pub fn prior_state_hash(self) -> StateHash {
        self.prior_state_hash
    }

    pub fn terminal_state_hash(self) -> StateHash {
        self.terminal_state_hash
    }

    pub fn outcome(self) -> LaneOutcome {
        self.outcome
    }

    pub fn player_position(self) -> LanePosition {
        self.player_position
    }

    pub fn player_health(self) -> LaneHealth {
        self.player_health
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub fn coordination(self) -> ObjectiveCoordination {
        self.coordination
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectiveInputIdentity {
    pub(crate) schema: &'static str,
    pub(crate) goal: ScenarioGoal,
    pub(crate) replay_id: &'static str,
    pub(crate) prior_state_hash: StateHash,
    pub(crate) terminal_state_hash: StateHash,
    pub(crate) visible_digest: StateHash,
}

impl ObjectiveInputIdentity {
    pub fn schema(self) -> &'static str {
        self.schema
    }

    pub fn goal(self) -> ScenarioGoal {
        self.goal
    }

    pub fn replay_id(self) -> &'static str {
        self.replay_id
    }

    pub fn prior_state_hash(self) -> StateHash {
        self.prior_state_hash
    }

    pub fn terminal_state_hash(self) -> StateHash {
        self.terminal_state_hash
    }

    pub fn visible_digest(self) -> StateHash {
        self.visible_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectiveCriterion {
    SpaceHeld,
    SurvivedBeat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectiveCriterionStatus {
    Met,
    NotMet,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectiveCriterionResult {
    pub(crate) criterion: ObjectiveCriterion,
    pub(crate) status: ObjectiveCriterionStatus,
}

impl ObjectiveCriterionResult {
    pub fn criterion(self) -> ObjectiveCriterion {
        self.criterion
    }

    pub fn status(self) -> ObjectiveCriterionStatus {
        self.status
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectiveDisposition {
    GoalAchieved,
    GoalPartiallyAchieved,
    GoalMissed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectiveAttributionLimit {
    CommittedFactsOnly,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectiveReport {
    pub(crate) schema: &'static str,
    pub(crate) goal: ScenarioGoal,
    pub(crate) criteria: [ObjectiveCriterionResult; 2],
    pub(crate) disposition: ObjectiveDisposition,
    pub(crate) intent: LaneIntent,
    pub(crate) coordination: ObjectiveCoordination,
    pub(crate) attribution_limit: ObjectiveAttributionLimit,
}

impl ObjectiveReport {
    pub fn schema(self) -> &'static str {
        self.schema
    }

    pub fn goal(self) -> ScenarioGoal {
        self.goal
    }

    pub fn criteria(self) -> [ObjectiveCriterionResult; 2] {
        self.criteria
    }

    pub fn disposition(self) -> ObjectiveDisposition {
        self.disposition
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn coordination(self) -> ObjectiveCoordination {
        self.coordination
    }

    pub fn attribution_limit(self) -> ObjectiveAttributionLimit {
        self.attribution_limit
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TerminalObjectiveReview {
    pub(crate) goal: ScenarioGoal,
    pub(crate) input_identity: ObjectiveInputIdentity,
    pub(crate) criteria: [ObjectiveCriterionResult; 2],
    pub(crate) disposition: ObjectiveDisposition,
    pub(crate) intent: LaneIntent,
    pub(crate) coordination: ObjectiveCoordination,
    pub(crate) execution_trace: InputTrace,
    pub(crate) attribution_limit: ObjectiveAttributionLimit,
}

impl TerminalObjectiveReview {
    pub fn goal(self) -> ScenarioGoal {
        self.goal
    }

    pub fn input_identity(self) -> ObjectiveInputIdentity {
        self.input_identity
    }

    pub fn criteria(self) -> [ObjectiveCriterionResult; 2] {
        self.criteria
    }

    pub fn disposition(self) -> ObjectiveDisposition {
        self.disposition
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn coordination(self) -> ObjectiveCoordination {
        self.coordination
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }

    pub fn attribution_limit(self) -> ObjectiveAttributionLimit {
        self.attribution_limit
    }

    pub fn report(self) -> ObjectiveReport {
        ObjectiveReport {
            schema: M2_OBJECTIVE_SCHEMA,
            goal: self.goal,
            criteria: self.criteria,
            disposition: self.disposition,
            intent: self.intent,
            coordination: self.coordination,
            attribution_limit: self.attribution_limit,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectiveError {
    UnsupportedReplayId,
    InvalidGoal,
    ReviewMismatch,
}

fn objective_visible_digest(inputs: ObjectiveEvaluationInputs) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, inputs.replay_id.as_bytes());
    hash = hash_bytes(hash, &inputs.prior_state_hash.value().to_le_bytes());
    hash = hash_bytes(hash, &inputs.terminal_state_hash.value().to_le_bytes());
    hash = hash_bytes(hash, &[outcome_tag(Some(inputs.outcome))]);
    hash = hash_bytes(hash, &[position_tag(inputs.player_position)]);
    hash = hash_bytes(hash, &[inputs.player_health.value()]);
    hash = hash_bytes(hash, &[intent_tag(inputs.intent)]);
    hash = hash_bytes(hash, &[wave_result_tag(inputs.wave_result)]);
    hash = hash_bytes(hash, &[objective_coordination_tag(inputs.coordination)]);
    if let ObjectiveCoordination::Resolved(disposition) = inputs.coordination {
        hash = hash_bytes(hash, &[coordination_disposition_tag(disposition)]);
    }
    hash = hash_bytes(hash, &[inputs.execution_trace.stream().value()]);
    hash = hash_bytes(hash, &inputs.execution_trace.draw().value().to_le_bytes());
    StateHash::from_raw(hash)
}

pub fn evaluate_terminal_objective(
    goal: ScenarioGoal,
    inputs: &ObjectiveEvaluationInputs,
) -> Result<TerminalObjectiveReview, ObjectiveError> {
    if inputs.replay_id != M2_REPLAY_ID && inputs.replay_id != M2_COORDINATION_REPLAY_ID {
        return Err(ObjectiveError::UnsupportedReplayId);
    }
    let (space_held, survived_beat) = match goal {
        ScenarioGoal::HoldLaneSpaceThroughWindow => (
            inputs.player_position == LanePosition::Center,
            inputs.player_health != LaneHealth::zero(),
        ),
    };
    let criteria = [
        ObjectiveCriterionResult {
            criterion: ObjectiveCriterion::SpaceHeld,
            status: if space_held {
                ObjectiveCriterionStatus::Met
            } else {
                ObjectiveCriterionStatus::NotMet
            },
        },
        ObjectiveCriterionResult {
            criterion: ObjectiveCriterion::SurvivedBeat,
            status: if survived_beat {
                ObjectiveCriterionStatus::Met
            } else {
                ObjectiveCriterionStatus::NotMet
            },
        },
    ];
    let disposition = match (space_held, survived_beat) {
        (true, true) => ObjectiveDisposition::GoalAchieved,
        (true, false) => ObjectiveDisposition::GoalPartiallyAchieved,
        (false, _) => ObjectiveDisposition::GoalMissed,
    };
    Ok(TerminalObjectiveReview {
        goal,
        input_identity: ObjectiveInputIdentity {
            schema: M2_OBJECTIVE_SCHEMA,
            goal,
            replay_id: inputs.replay_id,
            prior_state_hash: inputs.prior_state_hash,
            terminal_state_hash: inputs.terminal_state_hash,
            visible_digest: objective_visible_digest(*inputs),
        },
        criteria,
        disposition,
        intent: inputs.intent,
        coordination: inputs.coordination,
        execution_trace: inputs.execution_trace,
        attribution_limit: ObjectiveAttributionLimit::CommittedFactsOnly,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveReviewRecord {
    pub(crate) goal: ScenarioGoal,
    pub(crate) source_replay_id: &'static str,
    pub(crate) source_record_identity: StateHash,
    pub(crate) inputs: ObjectiveEvaluationInputs,
    pub(crate) review: TerminalObjectiveReview,
}

impl ObjectiveReviewRecord {
    pub fn goal(&self) -> ScenarioGoal {
        self.goal
    }

    pub fn source_replay_id(&self) -> &'static str {
        self.source_replay_id
    }

    pub fn source_record_identity(&self) -> StateHash {
        self.source_record_identity
    }

    pub fn inputs(&self) -> ObjectiveEvaluationInputs {
        self.inputs
    }

    pub fn review(&self) -> TerminalObjectiveReview {
        self.review
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StrategyFixtureId {
    HappyPath,
    RiskTaking,
    Conservative,
}

impl StrategyFixtureId {
    pub fn id(self) -> &'static str {
        match self {
            Self::HappyPath => "m2-strategy-happy-path-v3",
            Self::RiskTaking => "m2-strategy-risk-taking-v3",
            Self::Conservative => "m2-strategy-conservative-v3",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StrategyFixture {
    pub(crate) id: StrategyFixtureId,
    pub(crate) player_intent: LaneIntent,
    pub(crate) response: ProposalResponse,
    pub(crate) coordination_inputs: CoordinationResolutionInputs,
    pub(crate) lane_inputs: LaneResolvedInputs,
    pub(crate) expected_objective: ObjectiveDisposition,
    pub(crate) expected_outcome: LaneOutcome,
}

impl StrategyFixture {
    pub fn id(self) -> StrategyFixtureId {
        self.id
    }

    pub fn player_intent(self) -> LaneIntent {
        self.player_intent
    }

    pub fn response(self) -> ProposalResponse {
        self.response
    }

    pub fn coordination_inputs(self) -> CoordinationResolutionInputs {
        self.coordination_inputs
    }

    pub fn lane_inputs(self) -> LaneResolvedInputs {
        self.lane_inputs
    }

    pub fn expected_objective(self) -> ObjectiveDisposition {
        self.expected_objective
    }

    pub fn expected_outcome(self) -> LaneOutcome {
        self.expected_outcome
    }
}

pub struct StrategyFixtureRun {
    pub(crate) history: CoordinatedLaneHistory,
    pub(crate) objective: ObjectiveReviewRecord,
}

impl StrategyFixtureRun {
    pub fn history(&self) -> &CoordinatedLaneHistory {
        &self.history
    }

    pub fn objective(&self) -> &ObjectiveReviewRecord {
        &self.objective
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrategyFixtureError {
    UnsupportedFixture,
    Coordination(CoordinationError),
    Objective(ObjectiveError),
    UnexpectedOutcome,
    UnexpectedObjective,
}

pub fn strategy_fixture(id: StrategyFixtureId) -> Result<StrategyFixture, StrategyFixtureError> {
    let policy_trace = InputTrace::new(StreamId::new(13), DrawId::new(0));
    let coordination_trace = InputTrace::new(StreamId::new(14), DrawId::new(0));
    let (intent, response_kind, execution, expected_objective, expected_outcome) = match id {
        StrategyFixtureId::HappyPath => (
            LaneIntent::Contest,
            FollowThrough::AllyCommitted,
            LaneExecutionInputs::new(
                InputTrace::new(StreamId::new(15), DrawId::new(0)),
                LaneDamage::zero(),
                LaneDamage::new(2).map_err(|_| StrategyFixtureError::UnsupportedFixture)?,
                LaneWaveResult::Advanced,
            ),
            ObjectiveDisposition::GoalAchieved,
            LaneOutcome::HeldSpace,
        ),
        StrategyFixtureId::RiskTaking => (
            LaneIntent::Contest,
            FollowThrough::NotRequested,
            LaneExecutionInputs::new(
                InputTrace::new(StreamId::new(15), DrawId::new(1)),
                LaneDamage::new(3).map_err(|_| StrategyFixtureError::UnsupportedFixture)?,
                LaneDamage::zero(),
                LaneWaveResult::Lost,
            ),
            ObjectiveDisposition::GoalMissed,
            LaneOutcome::YieldedSpace,
        ),
        StrategyFixtureId::Conservative => (
            LaneIntent::Stabilize,
            FollowThrough::NotRequested,
            LaneExecutionInputs::new(
                InputTrace::new(StreamId::new(15), DrawId::new(2)),
                LaneDamage::zero(),
                LaneDamage::zero(),
                LaneWaveResult::Held,
            ),
            ObjectiveDisposition::GoalMissed,
            LaneOutcome::YieldedSpace,
        ),
    };
    let lane_inputs = LaneResolvedInputs::new(
        InputTrace::new(StreamId::new(11), DrawId::new(0)),
        InputTrace::new(StreamId::new(12), DrawId::new(0)),
        policy_trace,
        coordination_trace,
        execution,
    );
    let allied_receipt = observe_allied(&LaneSnapshot::initial(), ObservationId::new(9));
    let proposal = scripted_allied_proposal(allied_receipt.observation(), policy_trace)
        .map_err(|_| StrategyFixtureError::UnsupportedFixture)?;
    let proposal_id = proposal.id();
    let response = match response_kind {
        FollowThrough::NotRequested => ProposalResponse::Reject { proposal_id },
        FollowThrough::AllyCommitted => ProposalResponse::Accept { proposal_id },
        FollowThrough::AllyDeclined => ProposalResponse::Accept { proposal_id },
    };
    Ok(StrategyFixture {
        id,
        player_intent: intent,
        response,
        coordination_inputs: CoordinationResolutionInputs::new(coordination_trace, response_kind),
        lane_inputs,
        expected_objective,
        expected_outcome,
    })
}

pub fn run_strategy_fixture(
    fixture: StrategyFixture,
) -> Result<StrategyFixtureRun, StrategyFixtureError> {
    let state = LaneSnapshot::initial();
    let player_receipt = observe_player(&state, ObservationId::new(9));
    let allied_receipt = observe_allied(&state, ObservationId::new(9));
    let proposal =
        scripted_allied_proposal(allied_receipt.observation(), fixture.lane_inputs.policy())
            .map_err(|_| StrategyFixtureError::UnsupportedFixture)?;
    let offer =
        offer_allied_proposal(proposal).map_err(|_| StrategyFixtureError::UnsupportedFixture)?;
    let request = CoordinatedLaneRequest::new(
        LaneIntentRequest::new(
            PLAYER_LANER,
            player_receipt.observation().observation_id(),
            fixture.player_intent,
        ),
        fixture.response,
    );
    let mut history =
        CoordinatedLaneHistory::new(state).map_err(StrategyFixtureError::Coordination)?;
    let result = history
        .append(
            &player_receipt,
            &allied_receipt,
            &offer,
            &request,
            fixture.coordination_inputs,
            fixture.lane_inputs,
        )
        .map_err(StrategyFixtureError::Coordination)?;
    if result.lane().outcome() != fixture.expected_outcome {
        return Err(StrategyFixtureError::UnexpectedOutcome);
    }
    let objective = review_coordinated_objective(
        ScenarioGoal::HoldLaneSpaceThroughWindow,
        history
            .records()
            .first()
            .ok_or(StrategyFixtureError::UnexpectedOutcome)?,
    )
    .map_err(StrategyFixtureError::Objective)?;
    if objective.review().disposition() != fixture.expected_objective {
        return Err(StrategyFixtureError::UnexpectedObjective);
    }
    Ok(StrategyFixtureRun { history, objective })
}

pub fn review_lane_objective(
    goal: ScenarioGoal,
    record: &LaneTransitionRecord,
) -> Result<ObjectiveReviewRecord, ObjectiveError> {
    if record.replay_id() != M2_REPLAY_ID {
        return Err(ObjectiveError::UnsupportedReplayId);
    }
    let inputs = objective_inputs_from_lane_record(record, M2_REPLAY_ID);
    let review = evaluate_terminal_objective(goal, &inputs)?;
    Ok(ObjectiveReviewRecord {
        goal,
        source_replay_id: M2_REPLAY_ID,
        source_record_identity: lane_record_identity(record),
        inputs,
        review,
    })
}

pub fn review_coordinated_objective(
    goal: ScenarioGoal,
    record: &CoordinatedLaneRecord,
) -> Result<ObjectiveReviewRecord, ObjectiveError> {
    if record.replay_id() != M2_COORDINATION_REPLAY_ID
        || record.base_record().replay_id() != M2_REPLAY_ID
    {
        return Err(ObjectiveError::UnsupportedReplayId);
    }
    let inputs = objective_inputs_from_lane_record(record.base_record(), M2_COORDINATION_REPLAY_ID)
        .with_coordination(ObjectiveCoordination::Resolved(
            record.result().coordination().disposition(),
        ));
    let review = evaluate_terminal_objective(goal, &inputs)?;
    Ok(ObjectiveReviewRecord {
        goal,
        source_replay_id: M2_COORDINATION_REPLAY_ID,
        source_record_identity: record.base_record_identity(),
        inputs,
        review,
    })
}

impl ObjectiveReviewRecord {
    pub fn verify_lane(&self, record: &LaneTransitionRecord) -> Result<(), ObjectiveError> {
        if record.replay_id() != M2_REPLAY_ID {
            return Err(ObjectiveError::UnsupportedReplayId);
        }
        let expected_inputs = objective_inputs_from_lane_record(record, M2_REPLAY_ID);
        let expected = evaluate_terminal_objective(self.goal, &expected_inputs)?;
        if self.source_replay_id != M2_REPLAY_ID
            || self.source_record_identity != lane_record_identity(record)
            || self.inputs != expected_inputs
            || self.review != expected
        {
            return Err(ObjectiveError::ReviewMismatch);
        }
        Ok(())
    }

    pub fn verify_coordinated(&self, record: &CoordinatedLaneRecord) -> Result<(), ObjectiveError> {
        if record.replay_id() != M2_COORDINATION_REPLAY_ID
            || record.base_record().replay_id() != M2_REPLAY_ID
        {
            return Err(ObjectiveError::UnsupportedReplayId);
        }
        if lane_record_identity(record.base_record()) != record.base_record_identity() {
            return Err(ObjectiveError::ReviewMismatch);
        }
        if record.result().lane != *record.base_record().result() {
            return Err(ObjectiveError::ReviewMismatch);
        }
        let expected_inputs =
            objective_inputs_from_lane_record(record.base_record(), M2_COORDINATION_REPLAY_ID)
                .with_coordination(ObjectiveCoordination::Resolved(
                    record.result().coordination().disposition(),
                ));
        let expected = evaluate_terminal_objective(self.goal, &expected_inputs)?;
        if self.source_replay_id != M2_COORDINATION_REPLAY_ID
            || self.source_record_identity != record.base_record_identity()
            || self.inputs != expected_inputs
            || self.review != expected
        {
            return Err(ObjectiveError::ReviewMismatch);
        }
        Ok(())
    }
}

fn objective_inputs_from_lane_record(
    record: &LaneTransitionRecord,
    replay_id: &'static str,
) -> ObjectiveEvaluationInputs {
    ObjectiveEvaluationInputs::new(
        replay_id,
        record.prior_state_hash,
        record.result.state_hash,
        record.result.outcome,
        record.result.next_state.player().position(),
        record.result.next_state.player().health(),
        record.command.intent,
        record.inputs.execution.wave_result,
        ObjectiveCoordination::NotApplicable,
        record.inputs.execution.trace,
    )
}

impl ObjectiveEvaluationInputs {
    fn with_coordination(self, coordination: ObjectiveCoordination) -> Self {
        Self {
            coordination,
            ..self
        }
    }
}

fn objective_coordination_tag(value: ObjectiveCoordination) -> u8 {
    match value {
        ObjectiveCoordination::NotApplicable => 0,
        ObjectiveCoordination::Resolved(_) => 1,
    }
}

fn coordination_disposition_tag(value: CoordinationDisposition) -> u8 {
    match value {
        CoordinationDisposition::PlayerRejected => 0,
        CoordinationDisposition::AcceptedOffer => 1,
        CoordinationDisposition::AllyDeclined => 2,
        CoordinationDisposition::CounterAccepted => 3,
        CoordinationDisposition::CounterRejected => 4,
    }
}

pub(crate) fn response_review(response: ProposalResponse) -> CoordinatedResponseReview {
    match response {
        ProposalResponse::Accept { .. } => CoordinatedResponseReview::Accepted,
        ProposalResponse::Reject { .. } => CoordinatedResponseReview::Rejected,
        ProposalResponse::Counter { .. } => CoordinatedResponseReview::Countered,
    }
}
