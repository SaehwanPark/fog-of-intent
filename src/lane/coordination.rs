use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlliedProfileIdentity {
    pub(crate) profile_id: &'static str,
    pub(crate) candidate_rule: &'static str,
    pub(crate) evaluation_rule: &'static str,
    pub(crate) selection_rule: &'static str,
}

impl AlliedProfileIdentity {
    pub const fn scripted_v2() -> Self {
        Self {
            profile_id: SCRIPTED_ALLIED_PROFILE,
            candidate_rule: "available-intents-v3",
            evaluation_rule: "risk-wave-score-v3",
            selection_rule: "max-score-stabilize-tie-v3",
        }
    }

    pub fn profile_id(self) -> &'static str {
        self.profile_id
    }

    pub fn candidate_rule(self) -> &'static str {
        self.candidate_rule
    }

    pub fn evaluation_rule(self) -> &'static str {
        self.evaluation_rule
    }

    pub fn selection_rule(self) -> &'static str {
        self.selection_rule
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AgentInputIdentity {
    pub(crate) profile: AlliedProfileIdentity,
    pub(crate) actor: ActorId,
    pub(crate) ruleset: RulesetId,
    pub(crate) observation_schema: &'static str,
    pub(crate) turn: Turn,
    pub(crate) observation_id: ObservationId,
    pub(crate) visible_digest: StateHash,
    pub(crate) policy_trace: InputTrace,
}

impl AgentInputIdentity {
    pub fn profile(self) -> AlliedProfileIdentity {
        self.profile
    }

    pub fn actor(self) -> ActorId {
        self.actor
    }

    pub fn ruleset(self) -> RulesetId {
        self.ruleset
    }

    pub fn observation_schema(self) -> &'static str {
        self.observation_schema
    }

    pub fn turn(self) -> Turn {
        self.turn
    }

    pub fn observation_id(self) -> ObservationId {
        self.observation_id
    }

    pub fn visible_digest(self) -> StateHash {
        self.visible_digest
    }

    pub fn policy_trace(self) -> InputTrace {
        self.policy_trace
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlliedCandidate {
    pub(crate) intent: LaneIntent,
    pub(crate) score: i16,
    pub(crate) reason: AlliedReasonCode,
}

impl AlliedCandidate {
    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn score(self) -> i16 {
        self.score
    }

    pub fn reason(self) -> AlliedReasonCode {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlliedReasonCode {
    HealthRisk,
    WavePressure,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ProposalId(pub(crate) u64);

impl ProposalId {
    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneIntentProposal {
    pub(crate) id: ProposalId,
    pub(crate) actor: ActorId,
    pub(crate) profile: AlliedProfileIdentity,
    pub(crate) input_identity: AgentInputIdentity,
    pub(crate) candidates: [AlliedCandidate; 2],
    pub(crate) selected_intent: LaneIntent,
}

impl LaneIntentProposal {
    pub fn id(self) -> ProposalId {
        self.id
    }

    pub fn actor(self) -> ActorId {
        self.actor
    }

    pub fn profile(self) -> AlliedProfileIdentity {
        self.profile
    }

    pub fn input_identity(self) -> AgentInputIdentity {
        self.input_identity
    }

    pub fn candidates(self) -> [AlliedCandidate; 2] {
        self.candidates
    }

    pub fn selected_intent(self) -> LaneIntent {
        self.selected_intent
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlliedSupport {
    AssistContest,
    CoverStabilize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinationCommitment {
    UntilWindowEnd,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SupportFocus {
    OpponentAndWave,
    Wave,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SupportAbort {
    IfPlayerYields,
    IfPlayerHealthAtMost(u8),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SupportFallback {
    HoldPosition,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlliedProposalOffer {
    pub(crate) proposal: LaneIntentProposal,
    pub(crate) target: ActorId,
    pub(crate) support: AlliedSupport,
    pub(crate) commitment: CoordinationCommitment,
    pub(crate) focus: SupportFocus,
    pub(crate) abort: SupportAbort,
    pub(crate) fallback: SupportFallback,
}

impl AlliedProposalOffer {
    pub fn proposal(self) -> LaneIntentProposal {
        self.proposal
    }

    pub fn target(self) -> ActorId {
        self.target
    }

    pub fn support(self) -> AlliedSupport {
        self.support
    }

    pub fn commitment(self) -> CoordinationCommitment {
        self.commitment
    }

    pub fn focus(self) -> SupportFocus {
        self.focus
    }

    pub fn abort(self) -> SupportAbort {
        self.abort
    }

    pub fn fallback(self) -> SupportFallback {
        self.fallback
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinatedLanerObservation {
    pub(crate) lane: LanerObservation,
    pub(crate) allied_proposal: AlliedProposalOffer,
}

impl CoordinatedLanerObservation {
    pub fn lane(self) -> LanerObservation {
        self.lane
    }

    pub fn allied_proposal(self) -> AlliedProposalOffer {
        self.allied_proposal
    }
}

fn allied_visible_digest(observation: AlliedLaneObservation) -> StateHash {
    let mut hash = FNV_OFFSET_BASIS;
    hash = hash_bytes(hash, observation.schema.as_bytes());
    hash = hash_bytes(hash, &[observation.observer.value()]);
    hash = hash_bytes(hash, &observation.turn.value().to_le_bytes());
    hash = hash_bytes(hash, &observation.observation_id.value().to_le_bytes());
    hash = hash_bytes(hash, &[observation.laner_health().value()]);
    if observation.laner_mana() != LaneMana::full() {
        hash = hash_bytes(
            hash,
            &[LANE_MANA_HASH_TAG, observation.laner_mana().value()],
        );
    }
    if observation.laner_gold() != LaneGold::zero() {
        hash = hash_bytes(
            hash,
            &[LANE_GOLD_HASH_TAG, observation.laner_gold().value()],
        );
    }
    if observation.laner_experience() != LaneExperience::zero() {
        hash = hash_bytes(
            hash,
            &[
                LANE_EXPERIENCE_HASH_TAG,
                observation.laner_experience().value(),
            ],
        );
    }
    if observation.laner_cooldown() != LaneCooldown::zero() {
        hash = hash_bytes(
            hash,
            &[LANE_COOLDOWN_HASH_TAG, observation.laner_cooldown().value()],
        );
    }
    hash = hash_bytes(hash, &[position_tag(observation.laner_position())]);
    hash = hash_bytes(hash, &[observation.wave_pressure().value()]);
    hash = hash_bytes(hash, &[intent_tag(observation.available_intents()[0])]);
    hash = hash_bytes(hash, &[intent_tag(observation.available_intents()[1])]);
    if observation.window() != LaneWindow::OneBeat {
        hash = hash_bytes(hash, &[window_tag(observation.window())]);
    }
    hash = hash_bytes(hash, &[0, 0, 0]);
    StateHash::from_raw(hash)
}

pub fn allied_input_identity(
    observation: AlliedLaneObservation,
    policy_trace: InputTrace,
) -> AgentInputIdentity {
    AgentInputIdentity {
        profile: AlliedProfileIdentity::scripted_v2(),
        actor: observation.observer,
        ruleset: M2_LANE_RULESET,
        observation_schema: observation.schema,
        turn: observation.turn,
        observation_id: observation.observation_id,
        visible_digest: allied_visible_digest(observation),
        policy_trace,
    }
}

pub fn scripted_allied_proposal(
    observation: AlliedLaneObservation,
    policy_trace: InputTrace,
) -> Result<LaneIntentProposal, AlliedProposalError> {
    let identity = allied_input_identity(observation, policy_trace);
    if observation.schema != M2_ALLIED_OBSERVATION_SCHEMA
        || observation.observer != ALLIED_AUTONOMOUS_ACTOR
        || !matches!(
            observation.window(),
            LaneWindow::OneBeat | LaneWindow::TwoBeats
        )
        || observation.available_intents() != [LaneIntent::Stabilize, LaneIntent::Contest]
    {
        return Err(AlliedProposalError::InvalidObservation);
    }
    let health_risk = (5i16 - i16::from(observation.laner_health().value())).max(0);
    let mana_risk = (3i16 - i16::from(observation.laner_mana().value())).max(0);
    let stabilize_score =
        2 * health_risk + (3 - i16::from(observation.wave_pressure().value())) + mana_risk;
    let contest_score = 2 * i16::from(observation.wave_pressure().value())
        + (i16::from(observation.laner_health().value()) - 5).max(0)
        - mana_risk;
    let candidates = [
        AlliedCandidate {
            intent: LaneIntent::Stabilize,
            score: stabilize_score,
            reason: AlliedReasonCode::HealthRisk,
        },
        AlliedCandidate {
            intent: LaneIntent::Contest,
            score: contest_score,
            reason: AlliedReasonCode::WavePressure,
        },
    ];
    let selected_intent = if contest_score > stabilize_score {
        LaneIntent::Contest
    } else {
        LaneIntent::Stabilize
    };
    let mut proposal_hash = FNV_OFFSET_BASIS;
    proposal_hash = hash_bytes(
        proposal_hash,
        &identity.visible_digest.value().to_le_bytes(),
    );
    proposal_hash = hash_bytes(proposal_hash, &policy_trace.stream().value().to_le_bytes());
    proposal_hash = hash_bytes(proposal_hash, &policy_trace.draw().value().to_le_bytes());
    proposal_hash = hash_bytes(proposal_hash, &[intent_tag(selected_intent)]);
    Ok(LaneIntentProposal {
        id: ProposalId(proposal_hash),
        actor: ALLIED_AUTONOMOUS_ACTOR,
        profile: identity.profile,
        input_identity: identity,
        candidates,
        selected_intent,
    })
}

pub fn offer_allied_proposal(
    proposal: LaneIntentProposal,
) -> Result<AlliedProposalOffer, AlliedProposalError> {
    if proposal.actor != ALLIED_AUTONOMOUS_ACTOR
        || proposal.profile != AlliedProfileIdentity::scripted_v2()
    {
        return Err(AlliedProposalError::InvalidProposal);
    }
    let (support, focus, abort) = match proposal.selected_intent {
        LaneIntent::Contest => (
            AlliedSupport::AssistContest,
            SupportFocus::OpponentAndWave,
            SupportAbort::IfPlayerYields,
        ),
        LaneIntent::Stabilize => (
            AlliedSupport::CoverStabilize,
            SupportFocus::Wave,
            SupportAbort::IfPlayerHealthAtMost(2),
        ),
        LaneIntent::Yield | LaneIntent::Recall | LaneIntent::Withdraw => {
            return Err(AlliedProposalError::InvalidProposal);
        }
    };
    Ok(AlliedProposalOffer {
        proposal,
        target: PLAYER_LANER,
        support,
        commitment: CoordinationCommitment::UntilWindowEnd,
        focus,
        abort,
        fallback: SupportFallback::HoldPosition,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlliedProposalError {
    InvalidObservation,
    InvalidProposal,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CounterProposal {
    RequestIntent {
        requested_intent: LaneIntent,
        target: ActorId,
        commitment: CoordinationCommitment,
        focus: SupportFocus,
        abort: SupportAbort,
        fallback: SupportFallback,
    },
}

impl CounterProposal {
    pub fn requested_intent(self) -> LaneIntent {
        match self {
            Self::RequestIntent {
                requested_intent, ..
            } => requested_intent,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProposalResponse {
    Accept {
        proposal_id: ProposalId,
    },
    Reject {
        proposal_id: ProposalId,
    },
    Counter {
        proposal_id: ProposalId,
        counter: CounterProposal,
    },
}

impl ProposalResponse {
    pub fn proposal_id(self) -> ProposalId {
        match self {
            Self::Accept { proposal_id }
            | Self::Reject { proposal_id }
            | Self::Counter { proposal_id, .. } => proposal_id,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinatedLaneRequest {
    pub(crate) intent: LaneIntentRequest,
    pub(crate) response: ProposalResponse,
}

impl CoordinatedLaneRequest {
    pub fn new(intent: LaneIntentRequest, response: ProposalResponse) -> Self {
        Self { intent, response }
    }

    pub fn intent(self) -> LaneIntentRequest {
        self.intent
    }

    pub fn response(self) -> ProposalResponse {
        self.response
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FollowThrough {
    NotRequested,
    AllyCommitted,
    AllyDeclined,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinationResolutionInputs {
    pub(crate) trace: InputTrace,
    pub(crate) follow_through: FollowThrough,
}

impl CoordinationResolutionInputs {
    pub fn new(trace: InputTrace, follow_through: FollowThrough) -> Self {
        Self {
            trace,
            follow_through,
        }
    }

    pub fn trace(self) -> InputTrace {
        self.trace
    }

    pub fn follow_through(self) -> FollowThrough {
        self.follow_through
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinationDisposition {
    PlayerRejected,
    AcceptedOffer,
    AllyDeclined,
    CounterAccepted,
    CounterRejected,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinationResolution {
    pub(crate) proposal_id: ProposalId,
    pub(crate) response: ProposalResponse,
    pub(crate) disposition: CoordinationDisposition,
    pub(crate) support: Option<AlliedSupport>,
    pub(crate) trace: InputTrace,
}

impl CoordinationResolution {
    pub fn proposal_id(self) -> ProposalId {
        self.proposal_id
    }

    pub fn response(self) -> ProposalResponse {
        self.response
    }

    pub fn disposition(self) -> CoordinationDisposition {
        self.disposition
    }

    pub fn support(self) -> Option<AlliedSupport> {
        self.support
    }

    pub fn trace(self) -> InputTrace {
        self.trace
    }
}

pub fn resolve_coordination(
    offer: &AlliedProposalOffer,
    request: &CoordinatedLaneRequest,
    inputs: &CoordinationResolutionInputs,
) -> Result<CoordinationResolution, CoordinationError> {
    let response = request.response;
    if response.proposal_id() != offer.proposal.id {
        return Err(CoordinationError::ResponseProposalMismatch);
    }
    let (disposition, support) = match response {
        ProposalResponse::Reject { .. } => match inputs.follow_through {
            FollowThrough::NotRequested => (CoordinationDisposition::PlayerRejected, None),
            _ => return Err(CoordinationError::MalformedFollowThrough),
        },
        ProposalResponse::Accept { .. } => match inputs.follow_through {
            FollowThrough::AllyCommitted => {
                (CoordinationDisposition::AcceptedOffer, Some(offer.support))
            }
            FollowThrough::AllyDeclined => (CoordinationDisposition::AllyDeclined, None),
            FollowThrough::NotRequested => return Err(CoordinationError::MalformedFollowThrough),
        },
        ProposalResponse::Counter { counter, .. } => match inputs.follow_through {
            FollowThrough::AllyCommitted => (
                CoordinationDisposition::CounterAccepted,
                Some(counter_support(counter)?),
            ),
            FollowThrough::AllyDeclined => (CoordinationDisposition::CounterRejected, None),
            FollowThrough::NotRequested => return Err(CoordinationError::MalformedFollowThrough),
        },
    };
    Ok(CoordinationResolution {
        proposal_id: offer.proposal.id,
        response,
        disposition,
        support,
        trace: inputs.trace,
    })
}

fn counter_support(counter: CounterProposal) -> Result<AlliedSupport, CoordinationError> {
    match counter {
        CounterProposal::RequestIntent {
            requested_intent: LaneIntent::Contest,
            ..
        } => Ok(AlliedSupport::AssistContest),
        CounterProposal::RequestIntent {
            requested_intent: LaneIntent::Stabilize,
            ..
        } => Ok(AlliedSupport::CoverStabilize),
        CounterProposal::RequestIntent {
            requested_intent: LaneIntent::Yield | LaneIntent::Recall | LaneIntent::Withdraw,
            ..
        } => Err(CoordinationError::UnsupportedCounter),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinationError {
    StaleAlliedObservation,
    InvalidAlliedObservation,
    ProposalNotForWindow,
    ProposalIdMismatch,
    WrongProposer,
    WrongTarget,
    ResponseProposalMismatch,
    AcceptIntentMismatch,
    CounterIntentMismatch,
    UnsupportedCounter,
    DuplicateResponse,
    MalformedFollowThrough,
    CoordinationTraceMismatch,
    PolicyInputMismatch,
    LaneValidation(LaneValidationError),
    LaneTransition(LaneTransitionError),
    HistoryAlreadyHasRecord,
    ReplayMismatch,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ValidatedCoordinatedRequest {
    pub(crate) intent: ValidatedLaneIntent,
    pub(crate) request: CoordinatedLaneRequest,
}

impl ValidatedCoordinatedRequest {
    pub fn intent(self) -> ValidatedLaneIntent {
        self.intent
    }

    pub fn request(self) -> CoordinatedLaneRequest {
        self.request
    }
}

pub fn validate_coordinated_request(
    state: &LaneSnapshot,
    player_receipt: &LaneObservationReceipt,
    allied_receipt: &AlliedObservationReceipt,
    offer: &AlliedProposalOffer,
    request: &CoordinatedLaneRequest,
    policy_trace: InputTrace,
) -> Result<ValidatedCoordinatedRequest, CoordinationError> {
    let allied_observation = allied_receipt.observation;
    if allied_receipt.source_state_hash != state.hash()
        || allied_observation.turn != state.turn
        || allied_observation.schema != M2_ALLIED_OBSERVATION_SCHEMA
    {
        return Err(CoordinationError::StaleAlliedObservation);
    }
    if allied_observation.observer != ALLIED_AUTONOMOUS_ACTOR
        || !matches!(
            allied_observation.window,
            LaneWindow::OneBeat | LaneWindow::TwoBeats
        )
        || allied_observation.available_intents != [LaneIntent::Stabilize, LaneIntent::Contest]
    {
        return Err(CoordinationError::InvalidAlliedObservation);
    }
    let expected_proposal = scripted_allied_proposal(allied_observation, policy_trace)
        .map_err(|_| CoordinationError::InvalidAlliedObservation)?;
    let expected_offer = offer_allied_proposal(expected_proposal)
        .map_err(|_| CoordinationError::InvalidAlliedObservation)?;
    if *offer != expected_offer {
        return Err(CoordinationError::ProposalNotForWindow);
    }
    if offer.proposal.id != request.response.proposal_id() {
        return Err(CoordinationError::ProposalIdMismatch);
    }
    if offer.proposal.actor != ALLIED_AUTONOMOUS_ACTOR {
        return Err(CoordinationError::WrongProposer);
    }
    if offer.target != PLAYER_LANER {
        return Err(CoordinationError::WrongTarget);
    }
    let validated_intent = validate_lane_request(state, player_receipt, &request.intent)
        .map_err(CoordinationError::LaneValidation)?;
    match request.response {
        ProposalResponse::Accept { .. }
            if request.intent.intent != offer.proposal.selected_intent =>
        {
            return Err(CoordinationError::AcceptIntentMismatch);
        }
        ProposalResponse::Counter { counter, .. } => {
            if counter_target(counter) != PLAYER_LANER
                || counter_commitment(counter) != CoordinationCommitment::UntilWindowEnd
                || counter_fallback(counter) != SupportFallback::HoldPosition
                || counter.requested_intent() == offer.proposal.selected_intent
                || counter.requested_intent() != request.intent.intent
            {
                return Err(CoordinationError::CounterIntentMismatch);
            }
            if !counter_shape_matches(counter) {
                return Err(CoordinationError::UnsupportedCounter);
            }
        }
        ProposalResponse::Reject { .. } | ProposalResponse::Accept { .. } => {}
    }
    Ok(ValidatedCoordinatedRequest {
        intent: validated_intent,
        request: *request,
    })
}

fn counter_target(counter: CounterProposal) -> ActorId {
    match counter {
        CounterProposal::RequestIntent { target, .. } => target,
    }
}

fn counter_commitment(counter: CounterProposal) -> CoordinationCommitment {
    match counter {
        CounterProposal::RequestIntent { commitment, .. } => commitment,
    }
}

fn counter_fallback(counter: CounterProposal) -> SupportFallback {
    match counter {
        CounterProposal::RequestIntent { fallback, .. } => fallback,
    }
}

fn counter_shape_matches(counter: CounterProposal) -> bool {
    match counter {
        CounterProposal::RequestIntent {
            requested_intent: LaneIntent::Contest,
            focus: SupportFocus::OpponentAndWave,
            abort: SupportAbort::IfPlayerYields,
            ..
        }
        | CounterProposal::RequestIntent {
            requested_intent: LaneIntent::Stabilize,
            focus: SupportFocus::Wave,
            abort: SupportAbort::IfPlayerHealthAtMost(2),
            ..
        } => true,
        CounterProposal::RequestIntent { .. } => false,
    }
}
pub fn coordinated_laner_observation(
    player_receipt: &LaneObservationReceipt,
    offer: AlliedProposalOffer,
) -> CoordinatedLanerObservation {
    CoordinatedLanerObservation {
        lane: player_receipt.observation,
        allied_proposal: offer,
    }
}

pub fn resolve_coordinated_lane(
    state: &LaneSnapshot,
    player_receipt: &LaneObservationReceipt,
    allied_receipt: &AlliedObservationReceipt,
    offer: &AlliedProposalOffer,
    request: &CoordinatedLaneRequest,
    coordination_inputs: &CoordinationResolutionInputs,
    lane_inputs: &LaneResolvedInputs,
) -> Result<CoordinatedTransitionResult, CoordinationError> {
    let validated = validate_coordinated_request(
        state,
        player_receipt,
        allied_receipt,
        offer,
        request,
        lane_inputs.policy(),
    )?;
    if lane_inputs.coordination() != coordination_inputs.trace() {
        return Err(CoordinationError::CoordinationTraceMismatch);
    }
    let coordination = resolve_coordination(offer, request, coordination_inputs)?;
    let lane = transition_lane(state, &validated.intent, lane_inputs)
        .map_err(CoordinationError::LaneTransition)?;
    let mut events = vec![
        CoordinatedEvent::ProposalOffered {
            proposal_id: offer.proposal.id,
            proposer: offer.proposal.actor,
            target: offer.target,
        },
        CoordinatedEvent::ProposalResponded {
            proposal_id: offer.proposal.id,
            response: request.response,
        },
        CoordinatedEvent::CoordinationResolved {
            proposal_id: offer.proposal.id,
            disposition: coordination.disposition,
            trace: coordination.trace,
        },
    ];
    events.extend(lane.events().iter().copied().map(CoordinatedEvent::Lane));
    let mut effects = Vec::new();
    if let Some(support) = coordination.support {
        effects.push(CoordinatedEffect::SupportCommitted {
            proposal_id: offer.proposal.id,
            proposer: offer.proposal.actor,
            target: offer.target,
            support,
            cause: coordination.trace,
        });
    } else {
        effects.push(CoordinatedEffect::SupportUnavailable {
            proposal_id: offer.proposal.id,
            disposition: coordination.disposition,
            cause: coordination.trace,
        });
    }
    effects.extend(lane.effects().iter().copied().map(CoordinatedEffect::Lane));
    let debrief = CoordinatedDebrief {
        decision: CoordinatedDecisionReview::InformationConsistent,
        response: response_review(request.response),
        coordination: coordination.disposition,
        execution: CoordinatedExecutionReview::ConditionalOnCoordination {
            trace: lane_inputs.execution().trace(),
        },
        luck: CoordinatedLuckReview::ExplicitExecutionInput {
            trace: lane_inputs.execution().trace(),
        },
    };
    Ok(CoordinatedTransitionResult {
        lane,
        coordination,
        events,
        effects,
        debrief,
    })
}
