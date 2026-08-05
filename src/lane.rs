//! The first information-asymmetric M2 lane decision window.
//!
//! This module is a pure, one-window extension of the M1 kernel boundary. A
//! host creates an actor-valid observation, validates a request, resolves
//! execution inputs at the edge, and then invokes the deterministic transition.
//! No function here reads I/O, time, randomness, or hidden state through an
//! actor-facing value.

use std::fmt;

use crate::kernel::{
    ActorId, DrawId, InputTrace, RulesetId, StateHash, StreamId, Turn, hash_bytes,
};

pub const M2_LANE_RULESET: RulesetId = RulesetId::new(2);
pub const M2_OBSERVATION_SCHEMA: &str = "m2-lane-observation-v1";
pub const M2_REPLAY_ID: &str = "m2-one-lane-window-v1";
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const MAX_LANE_HEALTH: u8 = 10;
const MAX_WAVE_PRESSURE: u8 = 3;

pub const PLAYER_LANER: ActorId = ActorId::new(1);
pub const OPPONENT_LANER: ActorId = ActorId::new(2);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneHealth(u8);

impl LaneHealth {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_HEALTH {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_HEALTH,
            })
        }
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    fn subtract(self, amount: LaneDamage) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDamage(u8);

impl LaneDamage {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_HEALTH {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_HEALTH,
            })
        }
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LaneBoundsError {
    pub value: u8,
    pub maximum: u8,
}

impl fmt::Display for LaneBoundsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} exceeds maximum {}", self.value, self.maximum)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WavePressure(u8);

impl WavePressure {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_WAVE_PRESSURE {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_WAVE_PRESSURE,
            })
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }

    fn advance(self) -> Result<Self, LaneExecutionError> {
        Self::new(self.0 + 1).map_err(|_| LaneExecutionError::WaveOverflow { pressure: self })
    }

    fn lose(self) -> Result<Self, LaneExecutionError> {
        self.0
            .checked_sub(1)
            .map(Self)
            .ok_or(LaneExecutionError::WaveUnderflow { pressure: self })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LanePosition {
    NearTower,
    Center,
    FarSide,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OpponentPosture {
    Aggressive,
    Passive,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum JungleThreatTruth {
    Absent,
    RiverSide,
    InLane,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LanePhase {
    Open,
    Resolved,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneOutcome {
    HeldSpace,
    YieldedSpace,
    ForcedOut,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlayerLaneState {
    id: ActorId,
    health: LaneHealth,
    position: LanePosition,
}

impl PlayerLaneState {
    pub fn new(id: ActorId, health: LaneHealth, position: LanePosition) -> Self {
        Self {
            id,
            health,
            position,
        }
    }

    pub fn id(self) -> ActorId {
        self.id
    }

    pub fn health(self) -> LaneHealth {
        self.health
    }

    pub fn position(self) -> LanePosition {
        self.position
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpponentTruth {
    id: ActorId,
    health: LaneHealth,
    position: LanePosition,
    posture: OpponentPosture,
}

impl OpponentTruth {
    pub fn new(
        id: ActorId,
        health: LaneHealth,
        position: LanePosition,
        posture: OpponentPosture,
    ) -> Self {
        Self {
            id,
            health,
            position,
            posture,
        }
    }

    pub fn id(self) -> ActorId {
        self.id
    }

    pub fn health(self) -> LaneHealth {
        self.health
    }

    pub fn position(self) -> LanePosition {
        self.position
    }

    pub fn posture(self) -> OpponentPosture {
        self.posture
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WaveState {
    pressure: WavePressure,
}

impl WaveState {
    pub fn new(pressure: WavePressure) -> Self {
        Self { pressure }
    }

    pub fn pressure(self) -> WavePressure {
        self.pressure
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneSnapshot {
    ruleset: RulesetId,
    turn: Turn,
    phase: LanePhase,
    player: PlayerLaneState,
    opponent: OpponentTruth,
    wave: WaveState,
    jungle_threat: JungleThreatTruth,
    terminal_outcome: Option<LaneOutcome>,
}

impl LaneSnapshot {
    pub fn initial() -> Self {
        Self::new(
            M2_LANE_RULESET,
            Turn::new(0),
            LanePhase::Open,
            PlayerLaneState::new(
                PLAYER_LANER,
                LaneHealth::new(8).expect("fixture health must be bounded"),
                LanePosition::Center,
            ),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(7).expect("fixture health must be bounded"),
                LanePosition::Center,
                OpponentPosture::Aggressive,
            ),
            WaveState::new(WavePressure::new(1).expect("fixture pressure must be bounded")),
            JungleThreatTruth::InLane,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ruleset: RulesetId,
        turn: Turn,
        phase: LanePhase,
        player: PlayerLaneState,
        opponent: OpponentTruth,
        wave: WaveState,
        jungle_threat: JungleThreatTruth,
        terminal_outcome: Option<LaneOutcome>,
    ) -> Self {
        Self {
            ruleset,
            turn,
            phase,
            player,
            opponent,
            wave,
            jungle_threat,
            terminal_outcome,
        }
    }

    pub fn ruleset(self) -> RulesetId {
        self.ruleset
    }

    pub fn turn(self) -> Turn {
        self.turn
    }

    pub fn phase(self) -> LanePhase {
        self.phase
    }

    pub fn player(self) -> PlayerLaneState {
        self.player
    }

    pub fn opponent(self) -> OpponentTruth {
        self.opponent
    }

    pub fn wave(self) -> WaveState {
        self.wave
    }

    pub fn jungle_threat(self) -> JungleThreatTruth {
        self.jungle_threat
    }

    pub fn terminal_outcome(self) -> Option<LaneOutcome> {
        self.terminal_outcome
    }

    pub fn hash(self) -> StateHash {
        let mut hash = FNV_OFFSET_BASIS;
        hash = hash_bytes(hash, &self.ruleset.value().to_le_bytes());
        hash = hash_bytes(hash, &self.turn.value().to_le_bytes());
        hash = hash_bytes(hash, &[phase_tag(self.phase)]);
        hash = hash_bytes(
            hash,
            &[self.player.id().value(), self.player.health().value()],
        );
        hash = hash_bytes(hash, &[position_tag(self.player.position())]);
        hash = hash_bytes(
            hash,
            &[self.opponent.id().value(), self.opponent.health().value()],
        );
        hash = hash_bytes(hash, &[position_tag(self.opponent.position())]);
        hash = hash_bytes(hash, &[posture_tag(self.opponent.posture())]);
        hash = hash_bytes(hash, &[self.wave.pressure().value()]);
        hash = hash_bytes(hash, &[threat_tag(self.jungle_threat)]);
        hash = hash_bytes(hash, &[outcome_tag(self.terminal_outcome)]);
        StateHash::from_raw(hash)
    }

    fn is_valid_lane_state(self) -> bool {
        self.ruleset == M2_LANE_RULESET
            && self.player.id == PLAYER_LANER
            && self.opponent.id == OPPONENT_LANER
            && ((self.phase == LanePhase::Open && self.terminal_outcome.is_none())
                || (self.phase == LanePhase::Resolved && self.terminal_outcome.is_some()))
    }
}

fn phase_tag(phase: LanePhase) -> u8 {
    match phase {
        LanePhase::Open => 0,
        LanePhase::Resolved => 1,
    }
}

fn position_tag(position: LanePosition) -> u8 {
    match position {
        LanePosition::NearTower => 0,
        LanePosition::Center => 1,
        LanePosition::FarSide => 2,
    }
}

fn posture_tag(posture: OpponentPosture) -> u8 {
    match posture {
        OpponentPosture::Aggressive => 0,
        OpponentPosture::Passive => 1,
    }
}

fn threat_tag(threat: JungleThreatTruth) -> u8 {
    match threat {
        JungleThreatTruth::Absent => 0,
        JungleThreatTruth::RiverSide => 1,
        JungleThreatTruth::InLane => 2,
    }
}

fn outcome_tag(outcome: Option<LaneOutcome>) -> u8 {
    match outcome {
        None => 0,
        Some(LaneOutcome::HeldSpace) => 1,
        Some(LaneOutcome::YieldedSpace) => 2,
        Some(LaneOutcome::ForcedOut) => 3,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObservationId(u64);

impl ObservationId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HiddenValue {
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ThreatReport {
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpponentReport {
    last_known_position: Option<LanePosition>,
    last_seen_turn: Option<Turn>,
    health: HiddenValue,
    posture: HiddenValue,
}

impl OpponentReport {
    pub fn last_known_position(self) -> Option<LanePosition> {
        self.last_known_position
    }

    pub fn last_seen_turn(self) -> Option<Turn> {
        self.last_seen_turn
    }

    pub fn health(self) -> HiddenValue {
        self.health
    }

    pub fn posture(self) -> HiddenValue {
        self.posture
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneWindow {
    OneBeat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LanerObservation {
    schema: &'static str,
    observer: ActorId,
    turn: Turn,
    observation_id: ObservationId,
    self_health: LaneHealth,
    self_position: LanePosition,
    wave_pressure: WavePressure,
    opponent: OpponentReport,
    jungle_threat: ThreatReport,
    available_intents: [LaneIntent; 2],
    window: LaneWindow,
}

impl LanerObservation {
    pub fn schema(self) -> &'static str {
        self.schema
    }

    pub fn observer(self) -> ActorId {
        self.observer
    }

    pub fn turn(self) -> Turn {
        self.turn
    }

    pub fn observation_id(self) -> ObservationId {
        self.observation_id
    }

    pub fn self_health(self) -> LaneHealth {
        self.self_health
    }

    pub fn self_position(self) -> LanePosition {
        self.self_position
    }

    pub fn wave_pressure(self) -> WavePressure {
        self.wave_pressure
    }

    pub fn opponent(self) -> OpponentReport {
        self.opponent
    }

    pub fn jungle_threat(self) -> ThreatReport {
        self.jungle_threat
    }

    pub fn available_intents(self) -> [LaneIntent; 2] {
        self.available_intents
    }

    pub fn window(self) -> LaneWindow {
        self.window
    }
}

#[derive(Clone, Copy)]
pub struct LaneObservationReceipt {
    observation: LanerObservation,
    source_state_hash: StateHash,
}

impl fmt::Debug for LaneObservationReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LaneObservationReceipt")
            .field("observation", &self.observation)
            .finish_non_exhaustive()
    }
}

impl LaneObservationReceipt {
    pub fn observation(self) -> LanerObservation {
        self.observation
    }
}

pub fn observe_player(
    state: &LaneSnapshot,
    observation_id: ObservationId,
) -> LaneObservationReceipt {
    LaneObservationReceipt {
        observation: LanerObservation {
            schema: M2_OBSERVATION_SCHEMA,
            observer: PLAYER_LANER,
            turn: state.turn(),
            observation_id,
            self_health: state.player().health(),
            self_position: state.player().position(),
            wave_pressure: state.wave().pressure(),
            opponent: OpponentReport {
                last_known_position: None,
                last_seen_turn: None,
                health: HiddenValue::Unknown,
                posture: HiddenValue::Unknown,
            },
            jungle_threat: ThreatReport::Unknown,
            available_intents: [LaneIntent::Stabilize, LaneIntent::Contest],
            window: LaneWindow::OneBeat,
        },
        source_state_hash: state.hash(),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneIntent {
    Stabilize,
    Contest,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneIntentRequest {
    actor: ActorId,
    observation_id: ObservationId,
    intent: LaneIntent,
}

impl LaneIntentRequest {
    pub fn new(actor: ActorId, observation_id: ObservationId, intent: LaneIntent) -> Self {
        Self {
            actor,
            observation_id,
            intent,
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneIntentCommand {
    actor: ActorId,
    turn: Turn,
    ruleset: RulesetId,
    observation_id: ObservationId,
    host_prior_state_hash: StateHash,
    intent: LaneIntent,
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
        Self {
            actor,
            turn,
            ruleset,
            observation_id,
            host_prior_state_hash,
            intent,
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ValidatedLaneIntent {
    command: LaneIntentCommand,
    validated_snapshot: LaneSnapshot,
}

impl ValidatedLaneIntent {
    pub fn command(self) -> LaneIntentCommand {
        self.command
    }

    pub fn validated_against(self) -> StateHash {
        self.validated_snapshot.hash()
    }
}

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
    let command = LaneIntentCommand::new(
        request.actor,
        state.turn,
        M2_LANE_RULESET,
        request.observation_id,
        state.hash(),
        request.intent,
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
        || receipt.source_state_hash != state.hash()
    {
        return Err(LaneValidationError::StaleObservation);
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneWaveResult {
    Advanced,
    Held,
    Lost,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneExecutionInputs {
    trace: InputTrace,
    self_damage: LaneDamage,
    opponent_damage: LaneDamage,
    wave_result: LaneWaveResult,
}

impl LaneExecutionInputs {
    pub fn new(
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
        }
    }

    pub fn trace(self) -> InputTrace {
        self.trace
    }

    pub fn self_damage(self) -> LaneDamage {
        self.self_damage
    }

    pub fn opponent_damage(self) -> LaneDamage {
        self.opponent_damage
    }

    pub fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneResolvedInputs {
    environment: InputTrace,
    observation: InputTrace,
    policy: InputTrace,
    coordination: InputTrace,
    execution: LaneExecutionInputs,
}

impl LaneResolvedInputs {
    pub fn new(
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

    pub fn execution(self) -> LaneExecutionInputs {
        self.execution
    }

    pub fn environment(self) -> InputTrace {
        self.environment
    }

    pub fn observation(self) -> InputTrace {
        self.observation
    }

    pub fn policy(self) -> InputTrace {
        self.policy
    }

    pub fn coordination(self) -> InputTrace {
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
pub enum LaneEvent {
    IntentCommitted {
        actor: ActorId,
        intent: LaneIntent,
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
    },
    WavePressureChanged {
        before: WavePressure,
        after: WavePressure,
        cause: LaneEffectCause,
    },
    PositionChanged {
        actor: ActorId,
        before: LanePosition,
        after: LanePosition,
        cause: LaneEffectCause,
    },
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
    decision: LaneDecisionReview,
    coordination: LaneCoordinationReview,
    intent: LaneIntent,
    self_damage: LaneDamage,
    wave_result: LaneWaveResult,
    fallback_activated: bool,
    execution_trace: InputTrace,
}

impl LaneDebrief {
    pub fn decision(self) -> LaneDecisionReview {
        self.decision
    }

    pub fn coordination(self) -> LaneCoordinationReview {
        self.coordination
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn self_damage(self) -> LaneDamage {
        self.self_damage
    }

    pub fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub fn fallback_activated(self) -> bool {
        self.fallback_activated
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneTransitionResult {
    next_state: LaneSnapshot,
    events: Vec<LaneEvent>,
    effects: Vec<LaneEffect>,
    outcome: LaneOutcome,
    debrief: LaneDebrief,
    state_hash: StateHash,
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

    pub fn outcome(&self) -> LaneOutcome {
        self.outcome
    }

    pub fn debrief(&self) -> LaneDebrief {
        self.debrief
    }

    pub fn state_hash(&self) -> StateHash {
        self.state_hash
    }
}

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
    if state.phase != LanePhase::Open {
        return Err(LaneTransitionError::WrongPhase);
    }
    let execution = inputs.execution;
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
    let after_wave = match execution.wave_result {
        LaneWaveResult::Advanced => state.wave.pressure.advance(),
        LaneWaveResult::Held => Ok(state.wave.pressure),
        LaneWaveResult::Lost => state.wave.pressure.lose(),
    }
    .map_err(LaneTransitionError::Execution)?;
    let after_player_health = player
        .health
        .subtract(execution.self_damage)
        .expect("validated damage must be subtractable");
    let after_opponent_health = opponent
        .health
        .subtract(execution.opponent_damage)
        .expect("validated damage must be subtractable");
    let fallback_activated =
        command.command.intent == LaneIntent::Contest && execution.self_damage.0 >= 2;
    let after_position = match command.command.intent {
        LaneIntent::Stabilize => LanePosition::NearTower,
        LaneIntent::Contest if fallback_activated => LanePosition::NearTower,
        LaneIntent::Contest => LanePosition::Center,
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
        .checked_add(1)
        .ok_or(LaneTransitionError::TurnOverflow)?;
    let next_player = PlayerLaneState::new(player.id, after_player_health, after_position);
    let next_opponent = OpponentTruth::new(
        opponent.id,
        after_opponent_health,
        opponent.position,
        opponent.posture,
    );
    let next_state = LaneSnapshot::new(
        state.ruleset,
        Turn::new(next_turn),
        LanePhase::Resolved,
        next_player,
        next_opponent,
        WaveState::new(after_wave),
        state.jungle_threat,
        Some(outcome),
    );
    let trace = execution.trace;
    let mut events = vec![LaneEvent::IntentCommitted {
        actor: command.command.actor,
        intent: command.command.intent,
    }];
    let mut effects = Vec::new();
    if execution.self_damage != LaneDamage::zero() {
        events.push(LaneEvent::PlayerDamaged {
            target: player.id,
            amount: execution.self_damage,
            trace,
        });
        effects.push(LaneEffect::HealthChanged {
            actor: player.id,
            before: player.health,
            after: after_player_health,
            cause: LaneEffectCause::Execution(trace),
        });
    }
    if execution.opponent_damage != LaneDamage::zero() {
        events.push(LaneEvent::OpponentDamaged {
            target: opponent.id,
            amount: execution.opponent_damage,
            trace,
        });
        effects.push(LaneEffect::HealthChanged {
            actor: opponent.id,
            before: opponent.health,
            after: after_opponent_health,
            cause: LaneEffectCause::Execution(trace),
        });
    }
    events.push(LaneEvent::WaveResolved {
        before: state.wave.pressure,
        after: after_wave,
        trace,
    });
    if after_wave != state.wave.pressure {
        effects.push(LaneEffect::WavePressureChanged {
            before: state.wave.pressure,
            after: after_wave,
            cause: LaneEffectCause::Execution(trace),
        });
    }
    if after_position != player.position {
        let cause = if fallback_activated {
            LaneEffectCause::Fallback
        } else {
            LaneEffectCause::Intent
        };
        effects.push(LaneEffect::PositionChanged {
            actor: player.id,
            before: player.position,
            after: after_position,
            cause,
        });
    }
    if fallback_activated {
        events.push(LaneEvent::FallbackActivated {
            actor: player.id,
            intent: command.command.intent,
        });
    }
    events.push(LaneEvent::WindowResolved { outcome });
    let debrief = LaneDebrief {
        decision: LaneDecisionReview::InformationConsistent,
        coordination: LaneCoordinationReview::NotApplicable,
        intent: command.command.intent,
        self_damage: execution.self_damage,
        wave_result: execution.wave_result,
        fallback_activated,
        execution_trace: trace,
    };
    Ok(LaneTransitionResult {
        next_state,
        events,
        effects,
        outcome,
        debrief,
        state_hash: next_state.hash(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneTransitionRecord {
    observation: LanerObservation,
    command: LaneIntentCommand,
    inputs: LaneResolvedInputs,
    prior_state_hash: StateHash,
    result: LaneTransitionResult,
}

impl LaneTransitionRecord {
    pub fn observation(&self) -> LanerObservation {
        self.observation
    }

    pub fn command(&self) -> LaneIntentCommand {
        self.command
    }

    pub fn inputs(&self) -> LaneResolvedInputs {
        self.inputs
    }

    pub fn prior_state_hash(&self) -> StateHash {
        self.prior_state_hash
    }

    pub fn result(&self) -> &LaneTransitionResult {
        &self.result
    }
}

pub struct LaneHistory {
    initial_state: LaneSnapshot,
    current_state: LaneSnapshot,
    records: Vec<LaneTransitionRecord>,
}

impl LaneHistory {
    pub fn new(initial_state: LaneSnapshot) -> Result<Self, LaneHistoryError> {
        if !initial_state.is_valid_lane_state() {
            return Err(LaneHistoryError::InvalidInitialState);
        }
        Ok(Self {
            initial_state,
            current_state: initial_state,
            records: Vec::new(),
        })
    }

    pub fn initial_state(&self) -> LaneSnapshot {
        self.initial_state
    }

    pub fn current_state(&self) -> LaneSnapshot {
        self.current_state
    }

    pub fn records(&self) -> &[LaneTransitionRecord] {
        &self.records
    }

    pub fn append(
        &mut self,
        receipt: &LaneObservationReceipt,
        request: &LaneIntentRequest,
        inputs: LaneResolvedInputs,
    ) -> Result<LaneTransitionResult, LaneHistoryError> {
        let index = self.records.len();
        let validated = validate_lane_request(&self.current_state, receipt, request)
            .map_err(|error| LaneHistoryError::Validation { index, error })?;
        let prior_state_hash = self.current_state.hash();
        let result = transition_lane(&self.current_state, &validated, &inputs)
            .map_err(|error| LaneHistoryError::Transition { index, error })?;
        self.current_state = result.next_state();
        self.records.push(LaneTransitionRecord {
            observation: receipt.observation,
            command: validated.command,
            inputs,
            prior_state_hash,
            result: result.clone(),
        });
        Ok(result)
    }

    pub fn verify_replay(&self) -> Result<LaneSnapshot, LaneReplayError> {
        let mut state = self.initial_state;
        for (index, record) in self.records.iter().enumerate() {
            let actual_prior_hash = state.hash();
            if record.prior_state_hash != actual_prior_hash {
                return Err(LaneReplayError::PriorHashMismatch {
                    index,
                    expected: record.prior_state_hash,
                    actual: actual_prior_hash,
                });
            }
            let receipt = observe_player(&state, record.command.observation_id);
            if receipt.observation != record.observation {
                return Err(LaneReplayError::ObservationMismatch { index });
            }
            let validated = validate_lane_command(&state, &receipt, &record.command)
                .map_err(|error| LaneReplayError::Validation { index, error })?;
            let result = transition_lane(&state, &validated, &record.inputs)
                .map_err(|error| LaneReplayError::Transition { index, error })?;
            if result != record.result {
                return Err(LaneReplayError::ResultMismatch { index });
            }
            state = result.next_state();
        }
        if state != self.current_state {
            return Err(LaneReplayError::TerminalStateMismatch {
                expected: self.current_state,
                actual: state,
            });
        }
        Ok(state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LaneHistoryError {
    InvalidInitialState,
    Validation {
        index: usize,
        error: LaneValidationError,
    },
    Transition {
        index: usize,
        error: LaneTransitionError,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LaneReplayError {
    PriorHashMismatch {
        index: usize,
        expected: StateHash,
        actual: StateHash,
    },
    ObservationMismatch {
        index: usize,
    },
    Validation {
        index: usize,
        error: LaneValidationError,
    },
    Transition {
        index: usize,
        error: LaneTransitionError,
    },
    ResultMismatch {
        index: usize,
    },
    TerminalStateMismatch {
        expected: LaneSnapshot,
        actual: LaneSnapshot,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BranchId(u8);

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
    replay_id: &'static str,
    parent_replay_id: &'static str,
    parent_record_index: usize,
    parent_initial_state_hash: StateHash,
    parent_terminal_state_hash: StateHash,
    branch_id: Option<BranchId>,
    alternate_intent: LaneIntent,
    execution_mode: BranchExecutionMode,
    execution_trace: InputTrace,
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

    pub fn branch_id(self) -> Option<BranchId> {
        self.branch_id
    }

    pub fn alternate_intent(self) -> LaneIntent {
        self.alternate_intent
    }

    pub fn execution_mode(self) -> BranchExecutionMode {
        self.execution_mode
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneBranch {
    identity: LaneBranchReplayIdentity,
    execution_selection: BranchExecutionSelection,
    record: LaneTransitionRecord,
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
        let (execution_relation, attribution_limit) = match self.identity.execution_mode {
            BranchExecutionMode::MatchedParent => (
                LaneExecutionRelation::Matched,
                LaneAttributionLimit::MatchedDecisionOnly,
            ),
            BranchExecutionMode::Regenerated => (
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
    Regenerated,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneAttributionLimit {
    MatchedDecisionOnly,
    DecisionAndExecutionChanged,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CounterfactualReview {
    parent_outcome: LaneOutcome,
    branch_outcome: LaneOutcome,
    parent_intent: LaneIntent,
    branch_intent: LaneIntent,
    execution_relation: LaneExecutionRelation,
    decision_comparison: LaneDecisionReview,
    coordination: LaneCoordinationReview,
    attribution_limit: LaneAttributionLimit,
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
    let (inputs, branch_id, execution_mode, execution_trace) = match selection {
        BranchExecutionSelection::MatchedParent { source_record } => {
            if source_record != 0 {
                return Err(LaneBranchError::InvalidBranchPoint);
            }
            let parent_inputs = parent_record.inputs;
            (
                parent_inputs,
                None,
                BranchExecutionMode::MatchedParent,
                parent_inputs.execution.trace,
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
        branch_id,
        alternate_intent: alternate.intent,
        execution_mode,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn trace(stream: u8, draw: u16) -> InputTrace {
        InputTrace::new(StreamId::new(stream), DrawId::new(draw))
    }

    fn inputs(
        self_damage: u8,
        opponent_damage: u8,
        wave_result: LaneWaveResult,
    ) -> LaneResolvedInputs {
        LaneResolvedInputs::new(
            trace(1, 1),
            trace(2, 2),
            trace(3, 3),
            trace(4, 4),
            LaneExecutionInputs::new(
                trace(5, 0),
                LaneDamage::new(self_damage).expect("damage must be bounded"),
                LaneDamage::new(opponent_damage).expect("damage must be bounded"),
                wave_result,
            ),
        )
    }

    fn request(
        state: &LaneSnapshot,
        intent: LaneIntent,
    ) -> (LaneObservationReceipt, LaneIntentRequest) {
        let receipt = observe_player(state, ObservationId::new(9));
        let request =
            LaneIntentRequest::new(PLAYER_LANER, receipt.observation().observation_id(), intent);
        (receipt, request)
    }

    #[test]
    fn observation_redacts_latent_state() {
        let first = LaneSnapshot::initial();
        let second = LaneSnapshot::new(
            M2_LANE_RULESET,
            first.turn(),
            LanePhase::Open,
            first.player(),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(1).expect("bounded"),
                LanePosition::FarSide,
                OpponentPosture::Passive,
            ),
            first.wave(),
            JungleThreatTruth::Absent,
            None,
        );
        let first_observation = observe_player(&first, ObservationId::new(1)).observation();
        let second_observation = observe_player(&second, ObservationId::new(1)).observation();
        assert_eq!(first_observation, second_observation);
        assert_eq!(first_observation.opponent().health(), HiddenValue::Unknown);
        assert_eq!(first_observation.opponent().posture(), HiddenValue::Unknown);
        assert_eq!(first_observation.jungle_threat(), ThreatReport::Unknown);
    }

    #[test]
    fn receipt_debug_does_not_reveal_the_host_state_hash() {
        let state = LaneSnapshot::initial();
        let receipt = observe_player(&state, ObservationId::new(1));
        let debug = format!("{receipt:?}");
        assert!(!debug.contains("source_state_hash"));
        assert!(!debug.contains(&state.hash().value().to_string()));
    }

    #[test]
    fn both_intents_are_legal_and_produce_distinct_positions() {
        let state = LaneSnapshot::initial();
        let (stabilize_receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
        let stabilize = validate_lane_request(&state, &stabilize_receipt, &stabilize_request)
            .expect("stabilize is legal");
        let stable_result =
            transition_lane(&state, &stabilize, &inputs(0, 1, LaneWaveResult::Held))
                .expect("stabilize transition");
        assert_eq!(stable_result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            stable_result.next_state().player().position(),
            LanePosition::NearTower
        );

        let (contest_receipt, contest_request) = request(&state, LaneIntent::Contest);
        let contest = validate_lane_request(&state, &contest_receipt, &contest_request)
            .expect("contest is legal");
        let contest_result =
            transition_lane(&state, &contest, &inputs(0, 1, LaneWaveResult::Advanced))
                .expect("contest transition");
        assert_eq!(contest_result.outcome(), LaneOutcome::HeldSpace);
        assert_eq!(
            contest_result.next_state().player().position(),
            LanePosition::Center
        );
    }

    #[test]
    fn legal_unfavorable_contest_activates_fallback() {
        let state = LaneSnapshot::initial();
        let (receipt, contest_request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &contest_request).expect("valid");
        let result = transition_lane(&state, &validated, &inputs(3, 0, LaneWaveResult::Lost))
            .expect("execution is legal");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert!(result.debrief().fallback_activated());
        assert!(
            result
                .events()
                .iter()
                .any(|event| { matches!(event, LaneEvent::FallbackActivated { .. }) })
        );
    }

    #[test]
    fn invalid_requests_fail_before_transition() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let wrong_actor =
            LaneIntentRequest::new(ActorId::new(8), request.observation_id(), request.intent());
        assert!(matches!(
            validate_lane_request(&state, &receipt, &wrong_actor),
            Err(LaneValidationError::WrongActor { .. })
        ));
        let wrong_turn = LaneIntentCommand::new(
            PLAYER_LANER,
            Turn::new(1),
            M2_LANE_RULESET,
            request.observation_id(),
            state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&state, &receipt, &wrong_turn),
            Err(LaneValidationError::WrongTurn {
                expected: state.turn(),
                actual: Turn::new(1),
            })
        );
        let wrong_ruleset = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            RulesetId::new(99),
            request.observation_id(),
            state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&state, &receipt, &wrong_ruleset),
            Err(LaneValidationError::WrongRuleset {
                expected: M2_LANE_RULESET,
                actual: RulesetId::new(99),
            })
        );
        let stale_hash = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            M2_LANE_RULESET,
            request.observation_id(),
            StateHash::from_raw(0),
            request.intent(),
        );
        assert!(matches!(
            validate_lane_command(&state, &receipt, &stale_hash),
            Err(LaneValidationError::StateHashMismatch { .. })
        ));
        let both_wrong = LaneIntentCommand::new(
            PLAYER_LANER,
            Turn::new(1),
            RulesetId::new(99),
            request.observation_id(),
            state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&state, &receipt, &both_wrong),
            Err(LaneValidationError::WrongTurn {
                expected: state.turn(),
                actual: Turn::new(1),
            })
        );
        let invalid_state = LaneSnapshot::new(
            M2_LANE_RULESET,
            state.turn(),
            LanePhase::Open,
            PlayerLaneState::new(
                ActorId::new(8),
                state.player().health(),
                state.player().position(),
            ),
            state.opponent(),
            state.wave(),
            state.jungle_threat(),
            None,
        );
        let invalid_receipt = observe_player(&invalid_state, request.observation_id());
        let invalid_command = LaneIntentCommand::new(
            PLAYER_LANER,
            invalid_state.turn(),
            M2_LANE_RULESET,
            request.observation_id(),
            invalid_state.hash(),
            request.intent(),
        );
        assert_eq!(
            validate_lane_command(&invalid_state, &invalid_receipt, &invalid_command),
            Err(LaneValidationError::InvalidState)
        );
        assert!(matches!(
            LaneHistory::new(invalid_state),
            Err(LaneHistoryError::InvalidInitialState)
        ));
        let resolved = transition_lane(
            &state,
            &validate_lane_request(&state, &receipt, &request).expect("valid"),
            &inputs(0, 0, LaneWaveResult::Held),
        )
        .expect("valid transition");
        assert_eq!(resolved.next_state().phase(), LanePhase::Resolved);
        let resolved_receipt = observe_player(&resolved.next_state(), ObservationId::new(9));
        assert_eq!(
            validate_lane_request(&resolved.next_state(), &resolved_receipt, &request),
            Err(LaneValidationError::WindowAlreadyResolved)
        );
    }

    #[test]
    fn malformed_execution_does_not_change_state() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let too_much = inputs(9, 0, LaneWaveResult::Held);
        assert!(matches!(
            transition_lane(&state, &validated, &too_much),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::SelfDamageExceedsHealth { .. }
            ))
        ));
        assert_eq!(state.phase(), LanePhase::Open);
        assert_eq!(state.terminal_outcome(), None);
    }

    #[test]
    fn forced_out_and_wave_boundaries_remain_explicit() {
        let state = LaneSnapshot::initial();
        let (receipt, contest_request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &contest_request).expect("valid");
        let forced_out = transition_lane(&state, &validated, &inputs(8, 0, LaneWaveResult::Held))
            .expect("damage reaches zero health");
        assert_eq!(forced_out.outcome(), LaneOutcome::ForcedOut);
        assert_eq!(
            forced_out.next_state().player().health(),
            LaneHealth::zero()
        );

        let at_zero = LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            WaveState::new(WavePressure::new(0).expect("bounded")),
            state.jungle_threat(),
            None,
        );
        let (zero_receipt, zero_request) = request(&at_zero, LaneIntent::Contest);
        let zero_validated =
            validate_lane_request(&at_zero, &zero_receipt, &zero_request).expect("valid");
        assert_eq!(
            transition_lane(
                &at_zero,
                &zero_validated,
                &inputs(0, 0, LaneWaveResult::Lost)
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::WaveUnderflow {
                    pressure: WavePressure(0)
                }
            ))
        );

        let at_max = LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            WaveState::new(WavePressure::new(3).expect("bounded")),
            state.jungle_threat(),
            None,
        );
        let (max_receipt, max_request) = request(&at_max, LaneIntent::Contest);
        let max_validated =
            validate_lane_request(&at_max, &max_receipt, &max_request).expect("valid");
        assert!(matches!(
            transition_lane(
                &at_max,
                &max_validated,
                &inputs(0, 0, LaneWaveResult::Advanced)
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::WaveOverflow { .. }
            ))
        ));
    }

    #[test]
    fn identical_inputs_and_neutral_stream_changes_are_deterministic() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let first_inputs = inputs(1, 2, LaneWaveResult::Advanced);
        let second_inputs = LaneResolvedInputs::new(
            trace(101, 101),
            trace(102, 102),
            trace(103, 103),
            trace(104, 104),
            first_inputs.execution(),
        );
        let first = transition_lane(&state, &validated, &first_inputs).expect("transition");
        let second = transition_lane(&state, &validated, &second_inputs).expect("transition");
        assert_eq!(first, second);
        assert_eq!(first.state_hash(), first.next_state().hash());
    }

    #[test]
    fn history_replays_the_committed_window() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("initial state is valid");
        history
            .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
            .expect("append");
        assert_eq!(history.records().len(), 1);
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

    fn committed_parent(intent: LaneIntent) -> (LaneHistory, LaneObservationReceipt) {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, intent);
        let mut parent = LaneHistory::new(state).expect("initial state is valid");
        parent
            .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
            .expect("parent append");
        (parent, receipt)
    }

    #[test]
    fn matched_branch_replays_and_preserves_parent() {
        let (parent, receipt) = committed_parent(LaneIntent::Contest);
        let parent_records = parent.records.clone();
        let parent_current = parent.current_state();
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );
        let branch = branch_from_window(
            &parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("matched branch");

        assert_eq!(
            branch.identity().replay_id(),
            "m2-one-lane-window-branch-v1"
        );
        assert_eq!(
            branch.identity().execution_mode(),
            BranchExecutionMode::MatchedParent
        );
        assert_eq!(branch.record().inputs(), parent.records()[0].inputs());
        assert_eq!(branch.record().command().intent(), LaneIntent::Stabilize);
        branch.verify_replay(&parent).expect("branch replay");
        let review = branch.review(&parent).expect("counterfactual review");
        assert_eq!(review.parent_outcome(), LaneOutcome::HeldSpace);
        assert_eq!(review.branch_outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            review.attribution_limit(),
            LaneAttributionLimit::MatchedDecisionOnly
        );
        assert_eq!(parent.records, parent_records);
        assert_eq!(parent.current_state(), parent_current);
        assert_eq!(parent.verify_replay(), Ok(parent_current));
    }

    #[test]
    fn regenerated_branch_uses_a_stable_branch_trace() {
        let (parent, receipt) = committed_parent(LaneIntent::Stabilize);
        let branch_id = BranchId::new(7).expect("branch id is bounded");
        let execution = LaneExecutionInputs::new(
            trace(135, 0),
            LaneDamage::new(0).expect("bounded"),
            LaneDamage::new(2).expect("bounded"),
            LaneWaveResult::Advanced,
        );
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Contest,
        );
        let branch = branch_from_window(
            &parent,
            &alternate,
            BranchExecutionSelection::regenerated(branch_id, execution),
        )
        .expect("regenerated branch");
        assert_eq!(branch.identity().branch_id(), Some(branch_id));
        assert_eq!(branch.identity().execution_trace(), trace(135, 0));
        assert_eq!(
            branch.record().inputs().environment(),
            parent.records()[0].inputs().environment()
        );
        assert_eq!(branch.record().inputs().execution().trace(), trace(135, 0));
        assert_eq!(
            branch.review(&parent).expect("review").attribution_limit(),
            LaneAttributionLimit::DecisionAndExecutionChanged
        );
        branch.verify_replay(&parent).expect("branch replay");
    }

    #[test]
    fn branch_rejects_invalid_parent_or_selection_and_detects_tampering() {
        let state = LaneSnapshot::initial();
        let (_receipt, request) = request(&state, LaneIntent::Contest);
        let empty = LaneHistory::new(state).expect("initial state is valid");
        assert!(matches!(
            branch_from_window(&empty, &request, BranchExecutionSelection::matched_parent()),
            Err(LaneBranchError::ParentNotExactlyOneWindow)
        ));

        let (parent, receipt) = committed_parent(LaneIntent::Contest);
        let same_intent = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Contest,
        );
        assert!(matches!(
            branch_from_window(
                &parent,
                &same_intent,
                BranchExecutionSelection::matched_parent()
            ),
            Err(LaneBranchError::NotAnAlternateIntent)
        ));
        assert!(matches!(
            BranchId::new(128),
            Err(LaneBranchError::InvalidBranchId { value: 128 })
        ));

        let bad_execution = LaneExecutionInputs::new(
            trace(5, 0),
            LaneDamage::new(0).expect("bounded"),
            LaneDamage::new(0).expect("bounded"),
            LaneWaveResult::Held,
        );
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );
        assert!(matches!(
            branch_from_window(
                &parent,
                &alternate,
                BranchExecutionSelection::regenerated(
                    BranchId::new(1).expect("bounded"),
                    bad_execution,
                )
            ),
            Err(LaneBranchError::InvalidBranchExecutionIdentity)
        ));

        let mut tampered = branch_from_window(
            &parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("branch");
        tampered.record.command = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            M2_LANE_RULESET,
            receipt.observation().observation_id(),
            StateHash::from_raw(0),
            LaneIntent::Stabilize,
        );
        assert_eq!(
            tampered.verify_replay(&parent),
            Err(LaneBranchError::BranchReplayMismatch)
        );
    }
}
