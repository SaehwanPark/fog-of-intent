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
        Self {
            actor,
            observation_id,
            intent,
            target_focus,
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
        Self {
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
            target_focus,
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
