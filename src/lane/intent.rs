use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneIntent {
    Stabilize,
    Contest,
    Yield,
    Recall,
    Withdraw,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneIntentRequest {
    pub(crate) actor: ActorId,
    pub(crate) observation_id: ObservationId,
    pub(crate) intent: LaneIntent,
    pub(crate) target_focus: LaneTargetFocus,
    pub(crate) commitment: LaneCommitment,
    pub(crate) ping_signal: LanePingSignal,
}

impl LaneIntentRequest {
    pub fn new(actor: ActorId, observation_id: ObservationId, intent: LaneIntent) -> Self {
        Self::new_with_target_focus(
            actor,
            observation_id,
            intent,
            LaneTargetFocus::default_focus(),
        )
    }

    pub fn new_with_target_focus(
        actor: ActorId,
        observation_id: ObservationId,
        intent: LaneIntent,
        target_focus: LaneTargetFocus,
    ) -> Self {
        Self::new_with_focus_and_commitment(
            actor,
            observation_id,
            intent,
            target_focus,
            LaneCommitment::default_commitment(),
        )
    }

    pub fn new_with_commitment(
        actor: ActorId,
        observation_id: ObservationId,
        intent: LaneIntent,
        commitment: LaneCommitment,
    ) -> Self {
        Self::new_with_focus_and_commitment(
            actor,
            observation_id,
            intent,
            LaneTargetFocus::default_focus(),
            commitment,
        )
    }

    pub fn new_with_ping_signal(
        actor: ActorId,
        observation_id: ObservationId,
        intent: LaneIntent,
        ping_signal: LanePingSignal,
    ) -> Self {
        Self::new_with_full_intent(
            actor,
            observation_id,
            intent,
            LaneTargetFocus::default_focus(),
            LaneCommitment::default_commitment(),
            ping_signal,
        )
    }

    pub fn new_with_focus_and_commitment(
        actor: ActorId,
        observation_id: ObservationId,
        intent: LaneIntent,
        target_focus: LaneTargetFocus,
        commitment: LaneCommitment,
    ) -> Self {
        Self::new_with_full_intent(
            actor,
            observation_id,
            intent,
            target_focus,
            commitment,
            LanePingSignal::default_signal(),
        )
    }

    pub fn new_with_full_intent(
        actor: ActorId,
        observation_id: ObservationId,
        intent: LaneIntent,
        target_focus: LaneTargetFocus,
        commitment: LaneCommitment,
        ping_signal: LanePingSignal,
    ) -> Self {
        Self {
            actor,
            observation_id,
            intent,
            target_focus,
            commitment,
            ping_signal,
        }
    }

    pub fn actor(self) -> ActorId {
        self.actor
    }

    pub fn observation_id(self) -> ObservationId {
        self.observation_id
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn target_focus(self) -> LaneTargetFocus {
        self.target_focus
    }

    pub fn commitment(self) -> LaneCommitment {
        self.commitment
    }

    pub fn ping_signal(self) -> LanePingSignal {
        self.ping_signal
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneIntentCommand {
    pub(crate) actor: ActorId,
    pub(crate) turn: Turn,
    pub(crate) ruleset: RulesetId,
    pub(crate) observation_id: ObservationId,
    pub(crate) host_prior_state_hash: StateHash,
    pub(crate) intent: LaneIntent,
    pub(crate) target_focus: LaneTargetFocus,
    pub(crate) commitment: LaneCommitment,
    pub(crate) ping_signal: LanePingSignal,
}

impl LaneIntentCommand {
    pub fn new(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        observation_id: ObservationId,
        host_prior_state_hash: StateHash,
        intent: LaneIntent,
    ) -> Self {
        Self::new_with_target_focus(
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            LaneTargetFocus::default_focus(),
        )
    }

    pub fn new_with_target_focus(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        observation_id: ObservationId,
        host_prior_state_hash: StateHash,
        intent: LaneIntent,
        target_focus: LaneTargetFocus,
    ) -> Self {
        Self::new_with_focus_and_commitment(
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            target_focus,
            LaneCommitment::default_commitment(),
        )
    }

    pub fn new_with_commitment(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        observation_id: ObservationId,
        host_prior_state_hash: StateHash,
        intent: LaneIntent,
        commitment: LaneCommitment,
    ) -> Self {
        Self::new_with_focus_and_commitment(
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            LaneTargetFocus::default_focus(),
            commitment,
        )
    }

    pub fn new_with_ping_signal(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        observation_id: ObservationId,
        host_prior_state_hash: StateHash,
        intent: LaneIntent,
        ping_signal: LanePingSignal,
    ) -> Self {
        Self::new_with_full_intent(
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            LaneTargetFocus::default_focus(),
            LaneCommitment::default_commitment(),
            ping_signal,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_focus_and_commitment(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        observation_id: ObservationId,
        host_prior_state_hash: StateHash,
        intent: LaneIntent,
        target_focus: LaneTargetFocus,
        commitment: LaneCommitment,
    ) -> Self {
        Self::new_with_full_intent(
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            target_focus,
            commitment,
            LanePingSignal::default_signal(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_full_intent(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        observation_id: ObservationId,
        host_prior_state_hash: StateHash,
        intent: LaneIntent,
        target_focus: LaneTargetFocus,
        commitment: LaneCommitment,
        ping_signal: LanePingSignal,
    ) -> Self {
        Self {
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            target_focus,
            commitment,
            ping_signal,
        }
    }

    pub fn actor(self) -> ActorId {
        self.actor
    }

    pub fn turn(self) -> Turn {
        self.turn
    }

    pub fn ruleset(self) -> RulesetId {
        self.ruleset
    }

    pub fn observation_id(self) -> ObservationId {
        self.observation_id
    }

    pub fn host_prior_state_hash(self) -> StateHash {
        self.host_prior_state_hash
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn target_focus(self) -> LaneTargetFocus {
        self.target_focus
    }

    pub fn commitment(self) -> LaneCommitment {
        self.commitment
    }

    pub fn ping_signal(self) -> LanePingSignal {
        self.ping_signal
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ValidatedLaneIntent {
    pub(crate) command: LaneIntentCommand,
    pub(crate) validated_snapshot: LaneSnapshot,
}

impl ValidatedLaneIntent {
    pub fn command(self) -> LaneIntentCommand {
        self.command
    }

    pub fn validated_against(self) -> StateHash {
        self.validated_snapshot.hash()
    }
}
