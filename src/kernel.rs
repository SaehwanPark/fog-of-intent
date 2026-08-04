//! A small deterministic transition boundary.
//!
//! The kernel evaluates owned values only. It does not create random values,
//! read time, perform I/O, or persist history. A host supplies validated
//! commands and already-resolved inputs, then may commit the returned result.

use std::fmt;

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

pub const MAX_UNITS: u8 = 10;
pub const CURRENT_RULESET: RulesetId = RulesetId(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorId(u8);

impl ActorId {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Turn(u32);

impl Turn {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RulesetId(u16);

impl RulesetId {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StreamId(u8);

impl StreamId {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DrawId(u16);

impl DrawId {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateHash(u64);

impl StateHash {
    pub(crate) fn from_raw(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Units(u8);

impl Units {
    pub fn new(value: u8) -> Result<Self, BoundsError> {
        if value <= MAX_UNITS {
            Ok(Self(value))
        } else {
            Err(BoundsError {
                value,
                maximum: MAX_UNITS,
            })
        }
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundsError {
    pub value: u8,
    pub maximum: u8,
}

impl fmt::Display for BoundsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} exceeds maximum {}", self.value, self.maximum)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorState {
    id: ActorId,
    energy: Units,
    score: u16,
}

impl ActorState {
    pub fn new(id: ActorId, energy: Units, score: u16) -> Self {
        Self { id, energy, score }
    }

    pub fn id(self) -> ActorId {
        self.id
    }

    pub fn energy(self) -> Units {
        self.energy
    }

    pub fn score(self) -> u16 {
        self.score
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorldState {
    ruleset: RulesetId,
    turn: Turn,
    actor: ActorState,
}

impl WorldState {
    pub fn initial(actor: ActorId, ruleset: RulesetId) -> Self {
        Self {
            ruleset,
            turn: Turn::new(0),
            actor: ActorState::new(
                actor,
                Units::new(MAX_UNITS).expect("MAX_UNITS must be a valid Units value"),
                0,
            ),
        }
    }

    pub fn new(ruleset: RulesetId, turn: Turn, actor: ActorState) -> Self {
        Self {
            ruleset,
            turn,
            actor,
        }
    }

    pub fn ruleset(self) -> RulesetId {
        self.ruleset
    }

    pub fn turn(self) -> Turn {
        self.turn
    }

    pub fn actor(self) -> ActorState {
        self.actor
    }

    pub fn hash(self) -> StateHash {
        let mut hash = FNV_OFFSET_BASIS;
        hash = hash_bytes(hash, &self.ruleset.0.to_le_bytes());
        hash = hash_bytes(hash, &self.turn.0.to_le_bytes());
        hash = hash_bytes(hash, &[self.actor.id.0, self.actor.energy.0]);
        hash = hash_bytes(hash, &self.actor.score.to_le_bytes());
        StateHash(hash)
    }

    fn with_actor(self, actor: ActorState) -> Self {
        Self { actor, ..self }
    }

    fn advance_turn(self) -> Result<Self, TransitionError> {
        let next_turn = self
            .turn
            .0
            .checked_add(1)
            .map(Turn)
            .ok_or(TransitionError::TurnOverflow)?;
        Ok(Self {
            turn: next_turn,
            ..self
        })
    }
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    Hold,
    Gather { spend: Units },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Command {
    actor: ActorId,
    turn: Turn,
    ruleset: RulesetId,
    expected_state_hash: StateHash,
    action: Action,
}

impl Command {
    pub fn new(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        expected_state_hash: StateHash,
        action: Action,
    ) -> Self {
        Self {
            actor,
            turn,
            ruleset,
            expected_state_hash,
            action,
        }
    }

    pub fn hold(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        expected_state_hash: StateHash,
    ) -> Self {
        Self::new(actor, turn, ruleset, expected_state_hash, Action::Hold)
    }

    pub fn gather(
        actor: ActorId,
        turn: Turn,
        ruleset: RulesetId,
        expected_state_hash: StateHash,
        spend: Units,
    ) -> Self {
        Self::new(
            actor,
            turn,
            ruleset,
            expected_state_hash,
            Action::Gather { spend },
        )
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

    pub fn expected_state_hash(self) -> StateHash {
        self.expected_state_hash
    }

    pub fn action(self) -> Action {
        self.action
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ValidatedCommand {
    command: Command,
    validated_state: WorldState,
}

impl ValidatedCommand {
    pub fn command(self) -> Command {
        self.command
    }

    pub fn validated_against(self) -> StateHash {
        self.validated_state.hash()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationError {
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
    StateHashMismatch {
        expected: StateHash,
        actual: StateHash,
    },
    ZeroSpend,
    InsufficientEnergy {
        available: Units,
        requested: Units,
    },
}

pub fn validate_command(
    state: &WorldState,
    command: &Command,
) -> Result<ValidatedCommand, ValidationError> {
    let actor = state.actor();
    if command.actor != actor.id() {
        return Err(ValidationError::WrongActor {
            expected: actor.id(),
            actual: command.actor,
        });
    }
    if command.turn != state.turn {
        return Err(ValidationError::WrongTurn {
            expected: state.turn,
            actual: command.turn,
        });
    }
    if command.ruleset != state.ruleset {
        return Err(ValidationError::WrongRuleset {
            expected: state.ruleset,
            actual: command.ruleset,
        });
    }
    let actual_hash = state.hash();
    if command.expected_state_hash != actual_hash {
        return Err(ValidationError::StateHashMismatch {
            expected: actual_hash,
            actual: command.expected_state_hash,
        });
    }
    if let Action::Gather { spend } = command.action {
        if spend == Units::zero() {
            return Err(ValidationError::ZeroSpend);
        }
        if spend.value() > actor.energy().value() {
            return Err(ValidationError::InsufficientEnergy {
                available: actor.energy(),
                requested: spend,
            });
        }
    }
    Ok(ValidatedCommand {
        command: *command,
        validated_state: *state,
    })
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InputTrace {
    stream: StreamId,
    draw: DrawId,
}

impl InputTrace {
    pub fn new(stream: StreamId, draw: DrawId) -> Self {
        Self { stream, draw }
    }

    pub fn stream(self) -> StreamId {
        self.stream
    }

    pub fn draw(self) -> DrawId {
        self.draw
    }
}

macro_rules! input_category {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct $name {
            trace: InputTrace,
        }

        impl $name {
            pub fn new(trace: InputTrace) -> Self {
                Self { trace }
            }

            pub fn trace(self) -> InputTrace {
                self.trace
            }
        }
    };
}

input_category!(EnvironmentInputs);
input_category!(ObservationInputs);
input_category!(PolicyInputs);
input_category!(CoordinationInputs);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExecutionInputs {
    trace: InputTrace,
    yielded: Units,
}

impl ExecutionInputs {
    pub fn new(trace: InputTrace, yielded: Units) -> Self {
        Self { trace, yielded }
    }

    pub fn trace(self) -> InputTrace {
        self.trace
    }

    pub fn yielded(self) -> Units {
        self.yielded
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResolvedInputs {
    environment: EnvironmentInputs,
    observation: ObservationInputs,
    policy: PolicyInputs,
    coordination: CoordinationInputs,
    execution: ExecutionInputs,
}

impl ResolvedInputs {
    pub fn new(
        environment: EnvironmentInputs,
        observation: ObservationInputs,
        policy: PolicyInputs,
        coordination: CoordinationInputs,
        execution: ExecutionInputs,
    ) -> Self {
        Self {
            environment,
            observation,
            policy,
            coordination,
            execution,
        }
    }

    pub fn for_execution(execution: ExecutionInputs) -> Self {
        let neutral = InputTrace::new(StreamId::new(0), DrawId::new(0));
        Self::new(
            EnvironmentInputs::new(neutral),
            ObservationInputs::new(neutral),
            PolicyInputs::new(neutral),
            CoordinationInputs::new(neutral),
            execution,
        )
    }

    pub fn environment(self) -> EnvironmentInputs {
        self.environment
    }

    pub fn observation(self) -> ObservationInputs {
        self.observation
    }

    pub fn policy(self) -> PolicyInputs {
        self.policy
    }

    pub fn coordination(self) -> CoordinationInputs {
        self.coordination
    }

    pub fn execution(self) -> ExecutionInputs {
        self.execution
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectCause {
    Command,
    Execution(InputTrace),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Held {
        actor: ActorId,
    },
    Gathered {
        actor: ActorId,
        requested: Units,
        yielded: Units,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
    EnergySpent {
        actor: ActorId,
        amount: Units,
        cause: EffectCause,
    },
    ScoreAwarded {
        actor: ActorId,
        amount: Units,
        cause: EffectCause,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionError {
    StaleValidation {
        expected: StateHash,
        actual: StateHash,
    },
    YieldExceedsSpend {
        requested: Units,
        yielded: Units,
    },
    InsufficientEnergy {
        available: Units,
        requested: Units,
    },
    ScoreOverflow,
    TurnOverflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionResult {
    next_state: WorldState,
    events: Vec<Event>,
    effects: Vec<Effect>,
    state_hash: StateHash,
}

impl TransitionResult {
    pub fn next_state(&self) -> WorldState {
        self.next_state
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn effects(&self) -> &[Effect] {
        &self.effects
    }

    pub fn state_hash(&self) -> StateHash {
        self.state_hash
    }
}

pub fn transition(
    state: &WorldState,
    command: &ValidatedCommand,
    inputs: &ResolvedInputs,
) -> Result<TransitionResult, TransitionError> {
    let actual_hash = state.hash();
    if command.validated_state != *state {
        return Err(TransitionError::StaleValidation {
            expected: command.validated_state.hash(),
            actual: actual_hash,
        });
    }

    let actor = state.actor();
    let mut events = Vec::new();
    let mut effects = Vec::new();
    let next_state = match command.command.action {
        Action::Hold => {
            events.push(Event::Held { actor: actor.id() });
            state.advance_turn()?
        }
        Action::Gather { spend } => {
            let yielded = inputs.execution.yielded();
            if yielded.value() > spend.value() {
                return Err(TransitionError::YieldExceedsSpend {
                    requested: spend,
                    yielded,
                });
            }
            let energy =
                actor
                    .energy()
                    .subtract(spend)
                    .ok_or(TransitionError::InsufficientEnergy {
                        available: actor.energy(),
                        requested: spend,
                    })?;
            let score = actor
                .score()
                .checked_add(u16::from(yielded.value()))
                .ok_or(TransitionError::ScoreOverflow)?;
            let execution_cause = EffectCause::Execution(inputs.execution.trace());
            events.push(Event::Gathered {
                actor: actor.id(),
                requested: spend,
                yielded,
            });
            effects.push(Effect::EnergySpent {
                actor: actor.id(),
                amount: spend,
                cause: EffectCause::Command,
            });
            if yielded != Units::zero() {
                effects.push(Effect::ScoreAwarded {
                    actor: actor.id(),
                    amount: yielded,
                    cause: execution_cause,
                });
            }
            state
                .with_actor(ActorState::new(actor.id(), energy, score))
                .advance_turn()?
        }
    };

    let state_hash = next_state.hash();
    Ok(TransitionResult {
        next_state,
        events,
        effects,
        state_hash,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionRecord {
    command: Command,
    inputs: ResolvedInputs,
    prior_state_hash: StateHash,
    result: TransitionResult,
}

impl TransitionRecord {
    pub fn command(&self) -> Command {
        self.command
    }

    pub fn inputs(&self) -> ResolvedInputs {
        self.inputs
    }

    pub fn prior_state_hash(&self) -> StateHash {
        self.prior_state_hash
    }

    pub fn result(&self) -> &TransitionResult {
        &self.result
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoryError {
    Validation {
        index: usize,
        error: ValidationError,
    },
    Transition {
        index: usize,
        error: TransitionError,
    },
}

pub struct History {
    initial_state: WorldState,
    current_state: WorldState,
    records: Vec<TransitionRecord>,
}

impl History {
    pub fn new(initial_state: WorldState) -> Self {
        Self {
            initial_state,
            current_state: initial_state,
            records: Vec::new(),
        }
    }

    pub fn initial_state(&self) -> WorldState {
        self.initial_state
    }

    pub fn current_state(&self) -> WorldState {
        self.current_state
    }

    pub fn records(&self) -> &[TransitionRecord] {
        &self.records
    }

    pub fn append(
        &mut self,
        command: Command,
        inputs: ResolvedInputs,
    ) -> Result<TransitionResult, HistoryError> {
        let index = self.records.len();
        let validated = validate_command(&self.current_state, &command)
            .map_err(|error| HistoryError::Validation { index, error })?;
        let prior_state_hash = self.current_state.hash();
        let result = transition(&self.current_state, &validated, &inputs)
            .map_err(|error| HistoryError::Transition { index, error })?;
        self.current_state = result.next_state();
        self.records.push(TransitionRecord {
            command,
            inputs,
            prior_state_hash,
            result: result.clone(),
        });
        Ok(result)
    }

    pub fn verify_replay(&self) -> Result<WorldState, ReplayError> {
        let mut state = self.initial_state;
        for (index, record) in self.records.iter().enumerate() {
            let actual_prior_hash = state.hash();
            if record.prior_state_hash != actual_prior_hash {
                return Err(ReplayError::PriorHashMismatch {
                    index,
                    expected: record.prior_state_hash,
                    actual: actual_prior_hash,
                });
            }
            let validated = validate_command(&state, &record.command)
                .map_err(|error| ReplayError::Validation { index, error })?;
            let result = transition(&state, &validated, &record.inputs)
                .map_err(|error| ReplayError::Transition { index, error })?;
            if result != record.result {
                return Err(ReplayError::ResultMismatch { index });
            }
            state = result.next_state();
        }
        if state != self.current_state {
            return Err(ReplayError::TerminalStateMismatch {
                expected: self.current_state,
                actual: state,
            });
        }
        Ok(state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayError {
    PriorHashMismatch {
        index: usize,
        expected: StateHash,
        actual: StateHash,
    },
    Validation {
        index: usize,
        error: ValidationError,
    },
    Transition {
        index: usize,
        error: TransitionError,
    },
    ResultMismatch {
        index: usize,
    },
    TerminalStateMismatch {
        expected: WorldState,
        actual: WorldState,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor() -> ActorId {
        ActorId::new(7)
    }

    fn state() -> WorldState {
        WorldState::initial(actor(), CURRENT_RULESET)
    }

    fn units(value: u8) -> Units {
        Units::new(value).expect("test value must be bounded")
    }

    fn gather_command(state: WorldState, spend: u8) -> Command {
        Command::gather(
            actor(),
            state.turn(),
            state.ruleset(),
            state.hash(),
            units(spend),
        )
    }

    fn execution_inputs(yielded: u8) -> ResolvedInputs {
        ResolvedInputs::for_execution(ExecutionInputs::new(
            InputTrace::new(StreamId::new(3), DrawId::new(11)),
            units(yielded),
        ))
    }

    #[test]
    fn hold_advances_turn_without_effects() {
        let prior = state();
        let command = Command::hold(actor(), prior.turn(), prior.ruleset(), prior.hash());
        let validated = validate_command(&prior, &command).expect("hold is valid");
        let result = transition(&prior, &validated, &execution_inputs(0)).expect("transition");

        assert_eq!(result.next_state().turn(), Turn::new(1));
        assert_eq!(result.next_state().actor().energy(), prior.actor().energy());
        assert_eq!(result.events(), &[Event::Held { actor: actor() }]);
        assert!(result.effects().is_empty());
        assert_eq!(result.state_hash(), result.next_state().hash());
    }

    #[test]
    fn invalid_commands_fail_before_transition() {
        let prior = state();
        let zero = Command::gather(
            actor(),
            prior.turn(),
            prior.ruleset(),
            prior.hash(),
            Units::zero(),
        );
        assert_eq!(
            validate_command(&prior, &zero),
            Err(ValidationError::ZeroSpend)
        );

        let low_energy = WorldState::new(
            CURRENT_RULESET,
            Turn::new(0),
            ActorState::new(actor(), units(2), 0),
        );
        let overspend = gather_command(low_energy, 3);
        assert!(matches!(
            validate_command(&low_energy, &overspend),
            Err(ValidationError::InsufficientEnergy { .. })
        ));
    }

    #[test]
    fn wrong_actor_turn_ruleset_and_hash_are_rejected() {
        let prior = state();
        let wrong_actor =
            Command::hold(ActorId::new(8), prior.turn(), prior.ruleset(), prior.hash());
        assert!(matches!(
            validate_command(&prior, &wrong_actor),
            Err(ValidationError::WrongActor { .. })
        ));

        let wrong_turn = Command::hold(actor(), Turn::new(1), prior.ruleset(), prior.hash());
        assert!(matches!(
            validate_command(&prior, &wrong_turn),
            Err(ValidationError::WrongTurn { .. })
        ));

        let wrong_ruleset = Command::hold(actor(), prior.turn(), RulesetId::new(2), prior.hash());
        assert!(matches!(
            validate_command(&prior, &wrong_ruleset),
            Err(ValidationError::WrongRuleset { .. })
        ));

        let stale_hash = Command::hold(
            actor(),
            prior.turn(),
            prior.ruleset(),
            StateHash::from_raw(0),
        );
        assert!(matches!(
            validate_command(&prior, &stale_hash),
            Err(ValidationError::StateHashMismatch { .. })
        ));
    }

    #[test]
    fn zero_yield_is_legal_but_unfavorable() {
        let prior = state();
        let command = gather_command(prior, 4);
        let validated = validate_command(&prior, &command).expect("gather is valid");
        let result = transition(&prior, &validated, &execution_inputs(0)).expect("transition");

        assert_eq!(result.next_state().actor().energy(), units(6));
        assert_eq!(result.next_state().actor().score(), 0);
        assert_eq!(
            result.events(),
            &[Event::Gathered {
                actor: actor(),
                requested: units(4),
                yielded: units(0),
            }]
        );
        assert_eq!(result.effects().len(), 1);
        assert_eq!(
            result.effects()[0],
            Effect::EnergySpent {
                actor: actor(),
                amount: units(4),
                cause: EffectCause::Command,
            }
        );
        assert_eq!(result.state_hash(), result.next_state().hash());
    }

    #[test]
    fn gather_conserves_energy_and_awards_resolved_yield() {
        let prior = state();
        let command = gather_command(prior, 10);
        let validated = validate_command(&prior, &command).expect("gather is valid");
        let result = transition(&prior, &validated, &execution_inputs(10)).expect("transition");

        assert_eq!(result.next_state().actor().energy(), units(0));
        assert_eq!(result.next_state().actor().score(), 10);
        assert_eq!(
            prior.actor().energy().value(),
            result.next_state().actor().energy().value() + 10
        );
        assert_eq!(result.effects().len(), 2);
        assert!(matches!(
            result.effects()[1],
            Effect::ScoreAwarded {
                cause: EffectCause::Execution(_),
                ..
            }
        ));
    }

    #[test]
    fn malformed_resolved_yield_is_rejected() {
        let prior = state();
        let command = gather_command(prior, 3);
        let validated = validate_command(&prior, &command).expect("gather is valid");
        let inputs = execution_inputs(4);

        assert_eq!(
            transition(&prior, &validated, &inputs),
            Err(TransitionError::YieldExceedsSpend {
                requested: units(3),
                yielded: units(4),
            })
        );
    }

    #[test]
    fn validation_is_bound_to_the_exact_prior_state() {
        let prior = state();
        let command = gather_command(prior, 4);
        let validated = validate_command(&prior, &command).expect("gather is valid");
        let changed = WorldState::new(
            CURRENT_RULESET,
            prior.turn(),
            ActorState::new(actor(), units(2), prior.actor().score()),
        );

        assert!(matches!(
            transition(&changed, &validated, &execution_inputs(0)),
            Err(TransitionError::StaleValidation { .. })
        ));
    }

    #[test]
    fn identical_inputs_produce_identical_results_and_hashes() {
        let prior = state();
        let command = gather_command(prior, 4);
        let validated = validate_command(&prior, &command).expect("gather is valid");
        let inputs = execution_inputs(2);

        let first = transition(&prior, &validated, &inputs).expect("transition");
        let second = transition(&prior, &validated, &inputs).expect("transition");

        assert_eq!(first, second);
        assert_eq!(first.state_hash(), first.next_state().hash());
    }

    #[test]
    fn unrelated_input_streams_do_not_change_the_result() {
        let prior = state();
        let command = gather_command(prior, 4);
        let validated = validate_command(&prior, &command).expect("gather is valid");
        let execution =
            ExecutionInputs::new(InputTrace::new(StreamId::new(3), DrawId::new(11)), units(2));
        let first = ResolvedInputs::new(
            EnvironmentInputs::new(InputTrace::new(StreamId::new(1), DrawId::new(1))),
            ObservationInputs::new(InputTrace::new(StreamId::new(2), DrawId::new(2))),
            PolicyInputs::new(InputTrace::new(StreamId::new(3), DrawId::new(3))),
            CoordinationInputs::new(InputTrace::new(StreamId::new(4), DrawId::new(4))),
            execution,
        );
        let second = ResolvedInputs::new(
            EnvironmentInputs::new(InputTrace::new(StreamId::new(101), DrawId::new(101))),
            ObservationInputs::new(InputTrace::new(StreamId::new(102), DrawId::new(102))),
            PolicyInputs::new(InputTrace::new(StreamId::new(103), DrawId::new(103))),
            CoordinationInputs::new(InputTrace::new(StreamId::new(104), DrawId::new(104))),
            execution,
        );

        assert_eq!(
            transition(&prior, &validated, &first),
            transition(&prior, &validated, &second)
        );
    }

    #[test]
    fn history_is_append_only_and_replays_every_transition() {
        let initial = state();
        let mut history = History::new(initial);
        let hold = Command::hold(actor(), initial.turn(), initial.ruleset(), initial.hash());
        let first = history
            .append(hold, execution_inputs(0))
            .expect("hold append");
        let gather = gather_command(first.next_state(), 3);
        history
            .append(gather, execution_inputs(2))
            .expect("gather append");

        assert_eq!(history.records().len(), 2);
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        assert_eq!(
            history.records()[0].result().state_hash(),
            first.state_hash()
        );
    }

    #[test]
    fn history_rejects_duplicate_or_out_of_order_commands() {
        let initial = state();
        let mut history = History::new(initial);
        let hold = Command::hold(actor(), initial.turn(), initial.ruleset(), initial.hash());
        history
            .append(hold, execution_inputs(0))
            .expect("first append");

        assert!(matches!(
            history.append(hold, execution_inputs(0)),
            Err(HistoryError::Validation {
                error: ValidationError::WrongTurn { .. },
                ..
            })
        ));
    }

    #[test]
    fn bounded_units_reject_values_above_the_fixture_limit() {
        assert_eq!(
            Units::new(MAX_UNITS + 1),
            Err(BoundsError {
                value: MAX_UNITS + 1,
                maximum: MAX_UNITS,
            })
        );
    }

    #[test]
    fn exhaustive_bounded_gathers_preserve_energy_and_yield_invariants() {
        for spend in 1..=MAX_UNITS {
            for yielded in 0..=spend {
                let prior = state();
                let command = gather_command(prior, spend);
                let validated = validate_command(&prior, &command).expect("gather is valid");
                let result = transition(&prior, &validated, &execution_inputs(yielded))
                    .expect("bounded gather is valid");
                let next_actor = result.next_state().actor();

                assert!(next_actor.energy().value() <= MAX_UNITS);
                assert_eq!(
                    prior.actor().energy().value(),
                    next_actor.energy().value() + spend
                );
                assert_eq!(next_actor.score(), u16::from(yielded));
            }
        }
    }
}
