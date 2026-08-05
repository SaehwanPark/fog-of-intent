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
pub const M2_ALLIED_OBSERVATION_SCHEMA: &str = "m2-allied-proposal-observation-v1";
pub const M2_REPLAY_ID: &str = "m2-one-lane-window-v1";
pub const M2_COORDINATION_REPLAY_ID: &str = "m2-one-lane-coordination-v1";
pub const SCRIPTED_ALLIED_PROFILE: &str = "scripted-allied-proposal-v1";
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const MAX_LANE_HEALTH: u8 = 10;
const MAX_LANE_MANA: u8 = 6;
const MAX_LANE_GOLD: u8 = 20;
const MAX_WAVE_PRESSURE: u8 = 3;
const LANE_MANA_HASH_TAG: u8 = 0x4d;
const LANE_GOLD_HASH_TAG: u8 = 0x47;

pub const PLAYER_LANER: ActorId = ActorId::new(1);
pub const OPPONENT_LANER: ActorId = ActorId::new(2);
pub const ALLIED_AUTONOMOUS_ACTOR: ActorId = ActorId::new(3);

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
pub struct LaneMana(u8);

impl LaneMana {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_MANA {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_MANA,
            })
        }
    }

    pub const fn full() -> Self {
        Self(MAX_LANE_MANA)
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    fn subtract(self, amount: Self) -> Option<Self> {
        self.0.checked_sub(amount.0).map(Self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneGold(u8);

impl LaneGold {
    pub fn new(value: u8) -> Result<Self, LaneBoundsError> {
        if value <= MAX_LANE_GOLD {
            Ok(Self(value))
        } else {
            Err(LaneBoundsError {
                value,
                maximum: MAX_LANE_GOLD,
            })
        }
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    fn add(self, amount: Self) -> Option<Self> {
        let total = (self.0 as u16) + (amount.0 as u16);
        if total <= MAX_LANE_GOLD as u16 {
            Some(Self(total as u8))
        } else {
            None
        }
    }

    pub fn subtract(self, amount: Self) -> Option<Self> {
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
    mana: LaneMana,
    gold: LaneGold,
    position: LanePosition,
}

impl PlayerLaneState {
    pub fn new(id: ActorId, health: LaneHealth, position: LanePosition) -> Self {
        Self::new_with_resources(id, health, LaneMana::full(), LaneGold::zero(), position)
    }

    pub fn new_with_mana(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        position: LanePosition,
    ) -> Self {
        Self::new_with_resources(id, health, mana, LaneGold::zero(), position)
    }

    pub fn new_with_resources(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        position: LanePosition,
    ) -> Self {
        Self {
            id,
            health,
            mana,
            gold,
            position,
        }
    }

    pub fn id(self) -> ActorId {
        self.id
    }

    pub fn health(self) -> LaneHealth {
        self.health
    }

    pub fn mana(self) -> LaneMana {
        self.mana
    }

    pub fn gold(self) -> LaneGold {
        self.gold
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
    window: LaneWindow,
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
        Self::new_with_window(
            ruleset,
            turn,
            LaneWindow::OneBeat,
            phase,
            player,
            opponent,
            wave,
            jungle_threat,
            terminal_outcome,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_window(
        ruleset: RulesetId,
        turn: Turn,
        window: LaneWindow,
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
            window,
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

    pub fn window(self) -> LaneWindow {
        self.window
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
        if self.window != LaneWindow::OneBeat {
            hash = hash_bytes(hash, &[window_tag(self.window)]);
        }
        hash = hash_bytes(hash, &[phase_tag(self.phase)]);
        hash = hash_bytes(
            hash,
            &[self.player.id().value(), self.player.health().value()],
        );
        if self.player.mana() != LaneMana::full() {
            hash = hash_bytes(hash, &[LANE_MANA_HASH_TAG, self.player.mana().value()]);
        }
        if self.player.gold() != LaneGold::zero() {
            hash = hash_bytes(hash, &[LANE_GOLD_HASH_TAG, self.player.gold().value()]);
        }
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
pub enum JungleThreatRegion {
    RiverSide,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ThreatReport {
    Unknown,
    LastKnown {
        region: JungleThreatRegion,
        last_seen_turn: Turn,
    },
}

impl ThreatReport {
    pub fn last_known_region(self) -> Option<JungleThreatRegion> {
        match self {
            Self::Unknown => None,
            Self::LastKnown { region, .. } => Some(region),
        }
    }

    pub fn last_seen_turn(self) -> Option<Turn> {
        match self {
            Self::Unknown => None,
            Self::LastKnown { last_seen_turn, .. } => Some(last_seen_turn),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpponentReport {
    last_known_position: Option<LanePosition>,
    last_seen_turn: Option<Turn>,
    health: HiddenValue,
    posture: HiddenValue,
}

impl OpponentReport {
    fn unknown() -> Self {
        Self {
            last_known_position: None,
            last_seen_turn: None,
            health: HiddenValue::Unknown,
            posture: HiddenValue::Unknown,
        }
    }

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
    TwoBeats,
}

impl LaneWindow {
    pub const fn beats(self) -> u32 {
        match self {
            Self::OneBeat => 1,
            Self::TwoBeats => 2,
        }
    }

    pub const fn closes_on_commit(self) -> bool {
        true
    }
}

fn window_tag(window: LaneWindow) -> u8 {
    match window {
        LaneWindow::OneBeat => 0,
        LaneWindow::TwoBeats => 1,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LanerObservation {
    schema: &'static str,
    observer: ActorId,
    turn: Turn,
    observation_id: ObservationId,
    self_health: LaneHealth,
    self_mana: LaneMana,
    self_gold: LaneGold,
    self_position: LanePosition,
    wave_pressure: WavePressure,
    opponent: OpponentReport,
    jungle_threat: ThreatReport,
    available_intents: [LaneIntent; 4],
    available_threat_response: Option<LaneIntent>,
    window: LaneWindow,
}

fn player_opponent_report(state: &LaneSnapshot) -> OpponentReport {
    match state.opponent.position {
        LanePosition::FarSide => OpponentReport {
            last_known_position: Some(LanePosition::FarSide),
            last_seen_turn: Some(state.turn),
            ..OpponentReport::unknown()
        },
        LanePosition::NearTower | LanePosition::Center => OpponentReport::unknown(),
    }
}

fn player_threat_report(state: &LaneSnapshot) -> ThreatReport {
    match state.jungle_threat {
        JungleThreatTruth::RiverSide => ThreatReport::LastKnown {
            region: JungleThreatRegion::RiverSide,
            last_seen_turn: state.turn,
        },
        JungleThreatTruth::Absent | JungleThreatTruth::InLane => ThreatReport::Unknown,
    }
}

fn player_threat_response(threat_report: ThreatReport) -> Option<LaneIntent> {
    match threat_report {
        ThreatReport::Unknown => None,
        ThreatReport::LastKnown { .. } => Some(LaneIntent::Withdraw),
    }
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

    pub fn self_mana(self) -> LaneMana {
        self.self_mana
    }

    pub fn self_gold(self) -> LaneGold {
        self.self_gold
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

    pub fn available_intents(self) -> [LaneIntent; 4] {
        self.available_intents
    }

    pub fn available_threat_response(self) -> Option<LaneIntent> {
        self.available_threat_response
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
    let jungle_threat = player_threat_report(state);
    LaneObservationReceipt {
        observation: LanerObservation {
            schema: M2_OBSERVATION_SCHEMA,
            observer: PLAYER_LANER,
            turn: state.turn(),
            observation_id,
            self_health: state.player().health(),
            self_mana: state.player().mana(),
            self_gold: state.player().gold(),
            self_position: state.player().position(),
            wave_pressure: state.wave().pressure(),
            opponent: player_opponent_report(state),
            jungle_threat,
            available_intents: [
                LaneIntent::Stabilize,
                LaneIntent::Contest,
                LaneIntent::Yield,
                LaneIntent::Recall,
            ],
            available_threat_response: player_threat_response(jungle_threat),
            window: state.window(),
        },
        source_state_hash: state.hash(),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlliedLaneObservation {
    schema: &'static str,
    observer: ActorId,
    turn: Turn,
    observation_id: ObservationId,
    laner_health: LaneHealth,
    laner_mana: LaneMana,
    laner_gold: LaneGold,
    laner_position: LanePosition,
    wave_pressure: WavePressure,
    opponent: OpponentReport,
    jungle_threat: ThreatReport,
    available_intents: [LaneIntent; 2],
    window: LaneWindow,
}

impl AlliedLaneObservation {
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

    pub fn laner_health(self) -> LaneHealth {
        self.laner_health
    }

    pub fn laner_mana(self) -> LaneMana {
        self.laner_mana
    }

    pub fn laner_gold(self) -> LaneGold {
        self.laner_gold
    }

    pub fn laner_position(self) -> LanePosition {
        self.laner_position
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
pub struct AlliedObservationReceipt {
    observation: AlliedLaneObservation,
    source_state_hash: StateHash,
}

impl fmt::Debug for AlliedObservationReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlliedObservationReceipt")
            .field("observation", &self.observation)
            .finish_non_exhaustive()
    }
}

impl AlliedObservationReceipt {
    pub fn observation(self) -> AlliedLaneObservation {
        self.observation
    }
}

pub fn observe_allied(
    state: &LaneSnapshot,
    observation_id: ObservationId,
) -> AlliedObservationReceipt {
    AlliedObservationReceipt {
        observation: AlliedLaneObservation {
            schema: M2_ALLIED_OBSERVATION_SCHEMA,
            observer: ALLIED_AUTONOMOUS_ACTOR,
            turn: state.turn(),
            observation_id,
            laner_health: state.player().health(),
            laner_mana: state.player().mana(),
            laner_gold: state.player().gold(),
            laner_position: state.player().position(),
            wave_pressure: state.wave().pressure(),
            opponent: OpponentReport::unknown(),
            jungle_threat: ThreatReport::Unknown,
            available_intents: [LaneIntent::Stabilize, LaneIntent::Contest],
            window: state.window(),
        },
        source_state_hash: state.hash(),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlliedProfileIdentity {
    profile_id: &'static str,
    candidate_rule: &'static str,
    evaluation_rule: &'static str,
    selection_rule: &'static str,
}

impl AlliedProfileIdentity {
    pub const fn scripted_v1() -> Self {
        Self {
            profile_id: SCRIPTED_ALLIED_PROFILE,
            candidate_rule: "available-intents-v1",
            evaluation_rule: "risk-wave-score-v1",
            selection_rule: "max-score-stabilize-tie-v1",
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
    profile: AlliedProfileIdentity,
    actor: ActorId,
    ruleset: RulesetId,
    observation_schema: &'static str,
    turn: Turn,
    observation_id: ObservationId,
    visible_digest: StateHash,
    policy_trace: InputTrace,
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
    intent: LaneIntent,
    score: i16,
    reason: AlliedReasonCode,
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
pub struct ProposalId(u64);

impl ProposalId {
    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneIntentProposal {
    id: ProposalId,
    actor: ActorId,
    profile: AlliedProfileIdentity,
    input_identity: AgentInputIdentity,
    candidates: [AlliedCandidate; 2],
    selected_intent: LaneIntent,
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
    proposal: LaneIntentProposal,
    target: ActorId,
    support: AlliedSupport,
    commitment: CoordinationCommitment,
    focus: SupportFocus,
    abort: SupportAbort,
    fallback: SupportFallback,
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
    lane: LanerObservation,
    allied_proposal: AlliedProposalOffer,
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
    hash = hash_bytes(hash, &[observation.laner_health.value()]);
    if observation.laner_mana != LaneMana::full() {
        hash = hash_bytes(hash, &[LANE_MANA_HASH_TAG, observation.laner_mana.value()]);
    }
    if observation.laner_gold != LaneGold::zero() {
        hash = hash_bytes(hash, &[LANE_GOLD_HASH_TAG, observation.laner_gold.value()]);
    }
    hash = hash_bytes(hash, &[position_tag(observation.laner_position)]);
    hash = hash_bytes(hash, &[observation.wave_pressure.value()]);
    hash = hash_bytes(hash, &[intent_tag(observation.available_intents[0])]);
    hash = hash_bytes(hash, &[intent_tag(observation.available_intents[1])]);
    if observation.window != LaneWindow::OneBeat {
        hash = hash_bytes(hash, &[window_tag(observation.window)]);
    }
    hash = hash_bytes(hash, &[0, 0, 0]);
    StateHash::from_raw(hash)
}

pub fn allied_input_identity(
    observation: AlliedLaneObservation,
    policy_trace: InputTrace,
) -> AgentInputIdentity {
    AgentInputIdentity {
        profile: AlliedProfileIdentity::scripted_v1(),
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
            observation.window,
            LaneWindow::OneBeat | LaneWindow::TwoBeats
        )
        || observation.available_intents != [LaneIntent::Stabilize, LaneIntent::Contest]
    {
        return Err(AlliedProposalError::InvalidObservation);
    }
    let health_risk = (5i16 - i16::from(observation.laner_health.value())).max(0);
    let mana_risk = (3i16 - i16::from(observation.laner_mana.value())).max(0);
    let stabilize_score =
        2 * health_risk + (3 - i16::from(observation.wave_pressure.value())) + mana_risk;
    let contest_score = 2 * i16::from(observation.wave_pressure.value())
        + (i16::from(observation.laner_health.value()) - 5).max(0)
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
        || proposal.profile != AlliedProfileIdentity::scripted_v1()
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
    intent: LaneIntentRequest,
    response: ProposalResponse,
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
    trace: InputTrace,
    follow_through: FollowThrough,
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
    proposal_id: ProposalId,
    response: ProposalResponse,
    disposition: CoordinationDisposition,
    support: Option<AlliedSupport>,
    trace: InputTrace,
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
    intent: ValidatedLaneIntent,
    request: CoordinatedLaneRequest,
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
    mana_spent: LaneMana,
    gold_earned: LaneGold,
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
            mana_spent: LaneMana::zero(),
            gold_earned: LaneGold::zero(),
        }
    }

    pub fn with_mana_spent(mut self, mana_spent: LaneMana) -> Self {
        self.mana_spent = mana_spent;
        self
    }

    pub fn with_gold_earned(mut self, gold_earned: LaneGold) -> Self {
        self.gold_earned = gold_earned;
        self
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

    pub fn mana_spent(self) -> LaneMana {
        self.mana_spent
    }

    pub fn gold_earned(self) -> LaneGold {
        self.gold_earned
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

    pub fn with_mana_spent(mut self, mana_spent: LaneMana) -> Self {
        self.execution = self.execution.with_mana_spent(mana_spent);
        self
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
    relation: LaneEffectRelation,
    timing: LaneEffectTiming,
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

    pub fn relation(self) -> LaneEffectRelation {
        self.relation
    }

    pub fn timing(self) -> LaneEffectTiming {
        self.timing
    }
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
    PositionChanged {
        actor: ActorId,
        before: LanePosition,
        after: LanePosition,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
}

impl LaneEffect {
    pub fn provenance(self) -> LaneEffectProvenance {
        match self {
            Self::HealthChanged { provenance, .. }
            | Self::WavePressureChanged { provenance, .. }
            | Self::ManaChanged { provenance, .. }
            | Self::GoldChanged { provenance, .. }
            | Self::PositionChanged { provenance, .. } => provenance,
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
    mana_spent: LaneMana,
    gold_earned: LaneGold,
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

    pub fn mana_spent(self) -> LaneMana {
        self.mana_spent
    }

    pub fn gold_earned(self) -> LaneGold {
        self.gold_earned
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
    decision: CoordinatedDecisionReview,
    response: CoordinatedResponseReview,
    coordination: CoordinationDisposition,
    execution: CoordinatedExecutionReview,
    luck: CoordinatedLuckReview,
}

impl CoordinatedDebrief {
    pub fn decision(self) -> CoordinatedDecisionReview {
        self.decision
    }

    pub fn response(self) -> CoordinatedResponseReview {
        self.response
    }

    pub fn coordination(self) -> CoordinationDisposition {
        self.coordination
    }

    pub fn execution(self) -> CoordinatedExecutionReview {
        self.execution
    }

    pub fn luck(self) -> CoordinatedLuckReview {
        self.luck
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatedTransitionResult {
    lane: LaneTransitionResult,
    coordination: CoordinationResolution,
    events: Vec<CoordinatedEvent>,
    effects: Vec<CoordinatedEffect>,
    debrief: CoordinatedDebrief,
}

impl CoordinatedTransitionResult {
    pub fn lane(&self) -> &LaneTransitionResult {
        &self.lane
    }

    pub fn coordination(&self) -> CoordinationResolution {
        self.coordination
    }

    pub fn events(&self) -> &[CoordinatedEvent] {
        &self.events
    }

    pub fn effects(&self) -> &[CoordinatedEffect] {
        &self.effects
    }

    pub fn debrief(&self) -> CoordinatedDebrief {
        self.debrief
    }

    pub fn next_state(&self) -> LaneSnapshot {
        self.lane.next_state()
    }

    pub fn state_hash(&self) -> StateHash {
        self.lane.state_hash()
    }
}

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
    schema: &'static str,
    goal: ScenarioGoal,
    replay_id: &'static str,
    prior_state_hash: StateHash,
    terminal_state_hash: StateHash,
    visible_digest: StateHash,
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
    criterion: ObjectiveCriterion,
    status: ObjectiveCriterionStatus,
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
    schema: &'static str,
    goal: ScenarioGoal,
    criteria: [ObjectiveCriterionResult; 2],
    disposition: ObjectiveDisposition,
    intent: LaneIntent,
    coordination: ObjectiveCoordination,
    attribution_limit: ObjectiveAttributionLimit,
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
    goal: ScenarioGoal,
    input_identity: ObjectiveInputIdentity,
    criteria: [ObjectiveCriterionResult; 2],
    disposition: ObjectiveDisposition,
    intent: LaneIntent,
    coordination: ObjectiveCoordination,
    execution_trace: InputTrace,
    attribution_limit: ObjectiveAttributionLimit,
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
    goal: ScenarioGoal,
    source_replay_id: &'static str,
    source_record_identity: StateHash,
    inputs: ObjectiveEvaluationInputs,
    review: TerminalObjectiveReview,
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
            Self::HappyPath => "m2-strategy-happy-path-v1",
            Self::RiskTaking => "m2-strategy-risk-taking-v1",
            Self::Conservative => "m2-strategy-conservative-v1",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StrategyFixture {
    id: StrategyFixtureId,
    player_intent: LaneIntent,
    response: ProposalResponse,
    coordination_inputs: CoordinationResolutionInputs,
    lane_inputs: LaneResolvedInputs,
    expected_objective: ObjectiveDisposition,
    expected_outcome: LaneOutcome,
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
    history: CoordinatedLaneHistory,
    objective: ObjectiveReviewRecord,
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

fn response_review(response: ProposalResponse) -> CoordinatedResponseReview {
    match response {
        ProposalResponse::Accept { .. } => CoordinatedResponseReview::Accepted,
        ProposalResponse::Reject { .. } => CoordinatedResponseReview::Rejected,
        ProposalResponse::Counter { .. } => CoordinatedResponseReview::Countered,
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
    if execution.mana_spent != LaneMana::zero() && command.command.intent != LaneIntent::Contest {
        return Err(LaneTransitionError::Execution(
            LaneExecutionError::ManaSpentWithoutContest {
                intent: command.command.intent,
                spent: execution.mana_spent,
            },
        ));
    }
    let after_player_mana =
        player
            .mana
            .subtract(execution.mana_spent)
            .ok_or(LaneTransitionError::Execution(
                LaneExecutionError::ManaExceedsAvailable {
                    spent: execution.mana_spent,
                    available: player.mana,
                },
            ))?;
    let after_player_gold =
        player
            .gold
            .add(execution.gold_earned)
            .ok_or(LaneTransitionError::Execution(
                LaneExecutionError::GoldOverflow {
                    earned: execution.gold_earned,
                    current: player.gold,
                },
            ))?;
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
        LaneIntent::Yield => LanePosition::NearTower,
        LaneIntent::Recall => LanePosition::NearTower,
        LaneIntent::Withdraw => LanePosition::NearTower,
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
        .checked_add(state.window.beats())
        .ok_or(LaneTransitionError::TurnOverflow)?;
    let next_player = PlayerLaneState::new_with_resources(
        player.id,
        after_player_health,
        after_player_mana,
        after_player_gold,
        after_position,
    );
    let next_opponent = OpponentTruth::new(
        opponent.id,
        after_opponent_health,
        opponent.position,
        opponent.posture,
    );
    let next_state = LaneSnapshot::new_with_window(
        state.ruleset,
        Turn::new(next_turn),
        state.window,
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
            provenance: LaneEffectProvenance::direct_immediate(),
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
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.mana_spent != LaneMana::zero() {
        events.push(LaneEvent::ManaSpent {
            actor: player.id,
            amount: execution.mana_spent,
            trace,
        });
        effects.push(LaneEffect::ManaChanged {
            actor: player.id,
            before: player.mana,
            after: after_player_mana,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.gold_earned != LaneGold::zero() {
        events.push(LaneEvent::GoldEarned {
            actor: player.id,
            amount: execution.gold_earned,
            trace,
        });
        effects.push(LaneEffect::GoldChanged {
            actor: player.id,
            before: player.gold,
            after: after_player_gold,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
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
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if after_position != player.position {
        let cause = if fallback_activated {
            LaneEffectCause::Fallback
        } else {
            LaneEffectCause::Intent
        };
        let provenance = if fallback_activated {
            LaneEffectProvenance::indirect_immediate()
        } else {
            LaneEffectProvenance::direct_immediate()
        };
        effects.push(LaneEffect::PositionChanged {
            actor: player.id,
            before: player.position,
            after: after_position,
            cause,
            provenance,
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
        mana_spent: execution.mana_spent,
        gold_earned: execution.gold_earned,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatedLaneRecord {
    replay_id: &'static str,
    base_record_identity: StateHash,
    player_observation: LanerObservation,
    allied_observation: AlliedLaneObservation,
    offer: AlliedProposalOffer,
    request: CoordinatedLaneRequest,
    coordination_inputs: CoordinationResolutionInputs,
    resolution: CoordinationResolution,
    base_record: LaneTransitionRecord,
    result: CoordinatedTransitionResult,
}

impl CoordinatedLaneRecord {
    pub fn replay_id(&self) -> &'static str {
        self.replay_id
    }

    pub fn base_record_identity(&self) -> StateHash {
        self.base_record_identity
    }

    pub fn player_observation(&self) -> LanerObservation {
        self.player_observation
    }

    pub fn allied_observation(&self) -> AlliedLaneObservation {
        self.allied_observation
    }

    pub fn offer(&self) -> AlliedProposalOffer {
        self.offer
    }

    pub fn request(&self) -> CoordinatedLaneRequest {
        self.request
    }

    pub fn coordination_inputs(&self) -> CoordinationResolutionInputs {
        self.coordination_inputs
    }

    pub fn resolution(&self) -> CoordinationResolution {
        self.resolution
    }

    pub fn base_record(&self) -> &LaneTransitionRecord {
        &self.base_record
    }

    pub fn result(&self) -> &CoordinatedTransitionResult {
        &self.result
    }
}

pub struct CoordinatedLaneHistory {
    initial_state: LaneSnapshot,
    current_state: LaneSnapshot,
    records: Vec<CoordinatedLaneRecord>,
}

impl CoordinatedLaneHistory {
    pub fn new(initial_state: LaneSnapshot) -> Result<Self, CoordinationError> {
        if !initial_state.is_valid_lane_state() || initial_state.phase != LanePhase::Open {
            return Err(CoordinationError::InvalidAlliedObservation);
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

    pub fn records(&self) -> &[CoordinatedLaneRecord] {
        &self.records
    }

    pub fn append(
        &mut self,
        player_receipt: &LaneObservationReceipt,
        allied_receipt: &AlliedObservationReceipt,
        offer: &AlliedProposalOffer,
        request: &CoordinatedLaneRequest,
        coordination_inputs: CoordinationResolutionInputs,
        lane_inputs: LaneResolvedInputs,
    ) -> Result<CoordinatedTransitionResult, CoordinationError> {
        if !self.records.is_empty() {
            return Err(CoordinationError::HistoryAlreadyHasRecord);
        }
        let validated = validate_coordinated_request(
            &self.current_state,
            player_receipt,
            allied_receipt,
            offer,
            request,
            lane_inputs.policy(),
        )?;
        let prior_state_hash = self.current_state.hash();
        let result = resolve_coordinated_lane(
            &self.current_state,
            player_receipt,
            allied_receipt,
            offer,
            request,
            &coordination_inputs,
            &lane_inputs,
        )?;
        let base_record = LaneTransitionRecord {
            observation: player_receipt.observation,
            command: validated.intent.command,
            inputs: lane_inputs,
            prior_state_hash,
            result: result.lane.clone(),
        };
        self.current_state = result.next_state();
        self.records.push(CoordinatedLaneRecord {
            replay_id: M2_COORDINATION_REPLAY_ID,
            base_record_identity: lane_record_identity(&base_record),
            player_observation: player_receipt.observation,
            allied_observation: allied_receipt.observation,
            offer: *offer,
            request: *request,
            coordination_inputs,
            resolution: result.coordination,
            base_record,
            result: result.clone(),
        });
        Ok(result)
    }

    pub fn verify_replay(&self) -> Result<LaneSnapshot, CoordinationError> {
        if self.records.len() > 1 {
            return Err(CoordinationError::ReplayMismatch);
        }
        let mut state = self.initial_state;
        for record in &self.records {
            if record.replay_id != M2_COORDINATION_REPLAY_ID {
                return Err(CoordinationError::ReplayMismatch);
            }
            if record.base_record.prior_state_hash != state.hash() {
                return Err(CoordinationError::ReplayMismatch);
            }
            if lane_record_identity(&record.base_record) != record.base_record_identity {
                return Err(CoordinationError::ReplayMismatch);
            }
            let player_receipt = observe_player(&state, record.base_record.command.observation_id);
            let allied_receipt = observe_allied(&state, record.allied_observation.observation_id);
            let proposal = scripted_allied_proposal(
                allied_receipt.observation,
                record.base_record.inputs.policy(),
            )
            .map_err(|_| CoordinationError::ReplayMismatch)?;
            let offer =
                offer_allied_proposal(proposal).map_err(|_| CoordinationError::ReplayMismatch)?;
            if player_receipt.observation != record.player_observation
                || allied_receipt.observation != record.allied_observation
                || offer != record.offer
            {
                return Err(CoordinationError::ReplayMismatch);
            }
            let result = resolve_coordinated_lane(
                &state,
                &player_receipt,
                &allied_receipt,
                &offer,
                &record.request,
                &record.coordination_inputs,
                &record.base_record.inputs,
            )
            .map_err(|_| CoordinationError::ReplayMismatch)?;
            let validated = validate_coordinated_request(
                &state,
                &player_receipt,
                &allied_receipt,
                &offer,
                &record.request,
                record.base_record.inputs.policy(),
            )
            .map_err(|_| CoordinationError::ReplayMismatch)?;
            let expected_base_record = LaneTransitionRecord {
                observation: player_receipt.observation,
                command: validated.intent.command,
                inputs: record.base_record.inputs,
                prior_state_hash: state.hash(),
                result: result.lane.clone(),
            };
            if result != record.result
                || result.coordination != record.resolution
                || expected_base_record != record.base_record
            {
                return Err(CoordinationError::ReplayMismatch);
            }
            state = result.next_state();
        }
        if state != self.current_state {
            return Err(CoordinationError::ReplayMismatch);
        }
        Ok(state)
    }
}

pub const M2_TWO_WINDOW_REPLAY_ID: &str = "m2-two-window-scenario-v1";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScenarioWindow {
    First,
    Second,
}

impl ScenarioWindow {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::First),
            1 => Some(Self::Second),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneScenarioRecord {
    window: ScenarioWindow,
    start_state: LaneSnapshot,
    transition: LaneTransitionRecord,
    reopened_state: Option<LaneSnapshot>,
}

impl LaneScenarioRecord {
    pub fn window(&self) -> ScenarioWindow {
        self.window
    }

    pub fn start_state(&self) -> LaneSnapshot {
        self.start_state
    }

    pub fn transition(&self) -> &LaneTransitionRecord {
        &self.transition
    }

    pub fn reopened_state(&self) -> Option<LaneSnapshot> {
        self.reopened_state
    }
}

pub struct LaneScenarioHistory {
    replay_id: &'static str,
    initial_state: LaneSnapshot,
    current_state: LaneSnapshot,
    records: Vec<LaneScenarioRecord>,
}

impl LaneScenarioHistory {
    pub fn new(initial_state: LaneSnapshot) -> Result<Self, ScenarioError> {
        if !initial_state.is_valid_lane_state() || initial_state.phase != LanePhase::Open {
            return Err(ScenarioError::InvalidInitialState);
        }
        Ok(Self {
            replay_id: M2_TWO_WINDOW_REPLAY_ID,
            initial_state,
            current_state: initial_state,
            records: Vec::new(),
        })
    }

    pub fn replay_id(&self) -> &'static str {
        self.replay_id
    }

    pub fn initial_state(&self) -> LaneSnapshot {
        self.initial_state
    }

    pub fn current_state(&self) -> LaneSnapshot {
        self.current_state
    }

    pub fn records(&self) -> &[LaneScenarioRecord] {
        &self.records
    }

    pub fn terminal_state(&self) -> Result<LaneSnapshot, ScenarioError> {
        if self.records.len() != 2 {
            return Err(ScenarioError::ScenarioIncomplete);
        }
        Ok(self.current_state)
    }

    pub fn append(
        &mut self,
        receipt: &LaneObservationReceipt,
        request: &LaneIntentRequest,
        inputs: LaneResolvedInputs,
    ) -> Result<LaneTransitionResult, ScenarioError> {
        let index = self.records.len();
        let window = ScenarioWindow::from_index(index).ok_or(ScenarioError::ScenarioComplete)?;
        let start_state = self.current_state;
        if start_state.phase != LanePhase::Open {
            return Err(ScenarioError::WindowNotOpen);
        }
        let validated = validate_lane_request(&start_state, receipt, request)
            .map_err(|error| ScenarioError::Validation { window, error })?;
        let result = transition_lane(&start_state, &validated, &inputs)
            .map_err(|error| ScenarioError::Transition { window, error })?;
        let reopened_state = if window == ScenarioWindow::First {
            Some(reopen_lane_window(&result)?)
        } else {
            None
        };
        self.current_state = reopened_state.unwrap_or_else(|| result.next_state());
        self.records.push(LaneScenarioRecord {
            window,
            start_state,
            transition: LaneTransitionRecord {
                observation: receipt.observation,
                command: validated.command,
                inputs,
                prior_state_hash: start_state.hash(),
                result: result.clone(),
            },
            reopened_state,
        });
        Ok(result)
    }

    pub fn verify_replay(&self) -> Result<LaneSnapshot, ScenarioError> {
        if self.replay_id != M2_TWO_WINDOW_REPLAY_ID || self.records.len() > 2 {
            return Err(ScenarioError::ReplayMismatch);
        }
        let mut state = self.initial_state;
        for (index, record) in self.records.iter().enumerate() {
            let expected_window =
                ScenarioWindow::from_index(index).ok_or(ScenarioError::ReplayMismatch)?;
            if record.window != expected_window || record.start_state != state {
                return Err(ScenarioError::ReplayMismatch);
            }
            let receipt = observe_player(&state, record.transition.command.observation_id);
            if receipt.observation != record.transition.observation {
                return Err(ScenarioError::ReplayMismatch);
            }
            let validated = validate_lane_command(&state, &receipt, &record.transition.command)
                .map_err(|_| ScenarioError::ReplayMismatch)?;
            let result = transition_lane(&state, &validated, &record.transition.inputs)
                .map_err(|_| ScenarioError::ReplayMismatch)?;
            let expected_reopened = if expected_window == ScenarioWindow::First {
                Some(reopen_lane_window(&result)?)
            } else {
                None
            };
            let expected_record = LaneTransitionRecord {
                observation: receipt.observation,
                command: validated.command,
                inputs: record.transition.inputs,
                prior_state_hash: state.hash(),
                result: result.clone(),
            };
            if record.transition != expected_record || record.reopened_state != expected_reopened {
                return Err(ScenarioError::ReplayMismatch);
            }
            state = expected_reopened.unwrap_or_else(|| result.next_state());
        }
        if state != self.current_state {
            return Err(ScenarioError::ReplayMismatch);
        }
        Ok(state)
    }
}

pub fn reopen_lane_window(result: &LaneTransitionResult) -> Result<LaneSnapshot, ScenarioError> {
    let resolved = result.next_state();
    if result.state_hash() != resolved.hash()
        || result.outcome()
            != resolved
                .terminal_outcome()
                .ok_or(ScenarioError::InvalidReopenState)?
    {
        return Err(ScenarioError::InvalidReopenState);
    }
    reopen_resolved_snapshot(&resolved)
}

fn reopen_resolved_snapshot(resolved: &LaneSnapshot) -> Result<LaneSnapshot, ScenarioError> {
    if !resolved.is_valid_lane_state()
        || resolved.phase != LanePhase::Resolved
        || resolved.terminal_outcome.is_none()
    {
        return Err(ScenarioError::InvalidReopenState);
    }
    Ok(LaneSnapshot::new(
        resolved.ruleset,
        resolved.turn,
        LanePhase::Open,
        resolved.player,
        resolved.opponent,
        resolved.wave,
        resolved.jungle_threat,
        None,
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScenarioError {
    InvalidInitialState,
    InvalidReopenState,
    WindowNotOpen,
    ScenarioComplete,
    ScenarioIncomplete,
    Validation {
        window: ScenarioWindow,
        error: LaneValidationError,
    },
    Transition {
        window: ScenarioWindow,
        error: LaneTransitionError,
    },
    ReplayMismatch,
}

pub const M2_FINAL_DEBRIEF_REPLAY_ID: &str = "m2-two-window-final-debrief-v1";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FinalDebriefAttributionLimit {
    CommittedHistoryFactsOnly,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WindowDebriefSummary {
    window: ScenarioWindow,
    intent: LaneIntent,
    outcome: LaneOutcome,
    player_health: LaneHealth,
    player_position: LanePosition,
    wave_result: LaneWaveResult,
    coordination: LaneCoordinationReview,
    execution_trace: InputTrace,
    objective: TerminalObjectiveReview,
}

impl WindowDebriefSummary {
    pub fn window(self) -> ScenarioWindow {
        self.window
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn outcome(self) -> LaneOutcome {
        self.outcome
    }

    pub fn player_health(self) -> LaneHealth {
        self.player_health
    }

    pub fn player_position(self) -> LanePosition {
        self.player_position
    }

    pub fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub fn coordination(self) -> LaneCoordinationReview {
        self.coordination
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }

    pub fn objective(self) -> TerminalObjectiveReview {
        self.objective
    }

    fn report(self) -> VisibleWindowDebriefSummary {
        VisibleWindowDebriefSummary {
            window: self.window,
            intent: self.intent,
            outcome: self.outcome,
            player_health: self.player_health,
            player_position: self.player_position,
            wave_result: self.wave_result,
            coordination: self.coordination,
            objective: self.objective.disposition(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VisibleWindowDebriefSummary {
    window: ScenarioWindow,
    intent: LaneIntent,
    outcome: LaneOutcome,
    player_health: LaneHealth,
    player_position: LanePosition,
    wave_result: LaneWaveResult,
    coordination: LaneCoordinationReview,
    objective: ObjectiveDisposition,
}

impl VisibleWindowDebriefSummary {
    pub fn window(self) -> ScenarioWindow {
        self.window
    }

    pub fn intent(self) -> LaneIntent {
        self.intent
    }

    pub fn outcome(self) -> LaneOutcome {
        self.outcome
    }

    pub fn player_health(self) -> LaneHealth {
        self.player_health
    }

    pub fn player_position(self) -> LanePosition {
        self.player_position
    }

    pub fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub fn coordination(self) -> LaneCoordinationReview {
        self.coordination
    }

    pub fn objective(self) -> ObjectiveDisposition {
        self.objective
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScenarioDebriefReport {
    schema: &'static str,
    windows: [VisibleWindowDebriefSummary; 2],
    final_objective: ObjectiveDisposition,
    attribution_limit: FinalDebriefAttributionLimit,
}

impl ScenarioDebriefReport {
    pub fn schema(self) -> &'static str {
        self.schema
    }

    pub fn windows(self) -> [VisibleWindowDebriefSummary; 2] {
        self.windows
    }

    pub fn final_objective(self) -> ObjectiveDisposition {
        self.final_objective
    }

    pub fn attribution_limit(self) -> FinalDebriefAttributionLimit {
        self.attribution_limit
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioDebriefRecord {
    replay_id: &'static str,
    source_replay_id: &'static str,
    source_terminal_state_hash: StateHash,
    source_record_identities: [StateHash; 2],
    windows: [WindowDebriefSummary; 2],
    final_objective: ObjectiveDisposition,
    attribution_limit: FinalDebriefAttributionLimit,
    report: ScenarioDebriefReport,
}

impl ScenarioDebriefRecord {
    pub fn replay_id(&self) -> &'static str {
        self.replay_id
    }

    pub fn source_replay_id(&self) -> &'static str {
        self.source_replay_id
    }

    pub fn source_terminal_state_hash(&self) -> StateHash {
        self.source_terminal_state_hash
    }

    pub fn source_record_identities(&self) -> [StateHash; 2] {
        self.source_record_identities
    }

    pub fn windows(&self) -> [WindowDebriefSummary; 2] {
        self.windows
    }

    pub fn final_objective(&self) -> ObjectiveDisposition {
        self.final_objective
    }

    pub fn attribution_limit(&self) -> FinalDebriefAttributionLimit {
        self.attribution_limit
    }

    pub fn report(&self) -> ScenarioDebriefReport {
        self.report
    }

    pub fn verify_replay(&self, history: &LaneScenarioHistory) -> Result<(), ScenarioDebriefError> {
        let expected = build_scenario_debrief(history)?;
        if *self != expected {
            return Err(ScenarioDebriefError::ReplayMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScenarioDebriefError {
    History(ScenarioError),
    Objective(ObjectiveError),
    IncompleteHistory,
    ReplayMismatch,
}

pub fn build_scenario_debrief(
    history: &LaneScenarioHistory,
) -> Result<ScenarioDebriefRecord, ScenarioDebriefError> {
    history
        .verify_replay()
        .map_err(ScenarioDebriefError::History)?;
    if history.records().len() != 2 {
        return Err(ScenarioDebriefError::IncompleteHistory);
    }
    let mut summaries = [
        window_debrief_summary(&history.records()[0])?,
        window_debrief_summary(&history.records()[1])?,
    ];
    summaries[0].window = ScenarioWindow::First;
    summaries[1].window = ScenarioWindow::Second;
    let final_objective = if summaries
        .iter()
        .all(|summary| summary.objective.disposition() == ObjectiveDisposition::GoalAchieved)
    {
        ObjectiveDisposition::GoalAchieved
    } else {
        ObjectiveDisposition::GoalMissed
    };
    let attribution_limit = FinalDebriefAttributionLimit::CommittedHistoryFactsOnly;
    Ok(ScenarioDebriefRecord {
        replay_id: M2_FINAL_DEBRIEF_REPLAY_ID,
        source_replay_id: M2_TWO_WINDOW_REPLAY_ID,
        source_terminal_state_hash: history
            .terminal_state()
            .map_err(ScenarioDebriefError::History)?
            .hash(),
        source_record_identities: [
            lane_record_identity(history.records()[0].transition()),
            lane_record_identity(history.records()[1].transition()),
        ],
        windows: summaries,
        final_objective,
        attribution_limit,
        report: ScenarioDebriefReport {
            schema: M2_FINAL_DEBRIEF_REPLAY_ID,
            windows: [summaries[0].report(), summaries[1].report()],
            final_objective,
            attribution_limit,
        },
    })
}

fn window_debrief_summary(
    record: &LaneScenarioRecord,
) -> Result<WindowDebriefSummary, ScenarioDebriefError> {
    let transition = record.transition();
    let objective = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, transition)
        .map_err(ScenarioDebriefError::Objective)?;
    Ok(WindowDebriefSummary {
        window: record.window,
        intent: transition.command.intent,
        outcome: transition.result.outcome,
        player_health: transition.result.next_state.player().health(),
        player_position: transition.result.next_state.player().position(),
        wave_result: transition.inputs.execution.wave_result,
        coordination: LaneCoordinationReview::NotApplicable,
        execution_trace: transition.inputs.execution.trace,
        objective: objective.review,
    })
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
    replay_id: &'static str,
    parent_replay_id: &'static str,
    parent_record_index: usize,
    parent_initial_state_hash: StateHash,
    parent_terminal_state_hash: StateHash,
    parent_record_identity: StateHash,
    branch_id: Option<BranchId>,
    alternate_intent: LaneIntent,
    execution_mode: BranchExecutionMode,
    mana_policy: LaneBranchManaPolicy,
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

fn lane_record_identity(record: &LaneTransitionRecord) -> StateHash {
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
    hash = hash_bytes(
        hash,
        &[wave_result_tag(record.inputs.execution.wave_result)],
    );
    StateHash::from_raw(hash)
}

fn intent_tag(intent: LaneIntent) -> u8 {
    match intent {
        LaneIntent::Stabilize => 0,
        LaneIntent::Contest => 1,
        LaneIntent::Recall => 2,
        LaneIntent::Withdraw => 3,
        LaneIntent::Yield => 4,
    }
}

fn wave_result_tag(result: LaneWaveResult) -> u8 {
    match result {
        LaneWaveResult::Advanced => 0,
        LaneWaveResult::Held => 1,
        LaneWaveResult::Lost => 2,
    }
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

    fn river_side_state() -> LaneSnapshot {
        let state = LaneSnapshot::initial();
        LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            state.wave(),
            JungleThreatTruth::RiverSide,
            None,
        )
    }

    fn two_beat_state() -> LaneSnapshot {
        let state = LaneSnapshot::initial();
        LaneSnapshot::new_with_window(
            state.ruleset(),
            state.turn(),
            LaneWindow::TwoBeats,
            LanePhase::Open,
            state.player(),
            state.opponent(),
            state.wave(),
            state.jungle_threat(),
            None,
        )
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
        let second_observation = observe_player(&second, ObservationId::new(1)).observation();
        assert_eq!(
            observe_player(&first, ObservationId::new(1))
                .observation()
                .opponent()
                .last_known_position(),
            None
        );
        assert_eq!(
            second_observation.opponent().last_known_position(),
            Some(LanePosition::FarSide)
        );
        assert_eq!(second_observation.opponent().health(), HiddenValue::Unknown);
        assert_eq!(
            second_observation.opponent().posture(),
            HiddenValue::Unknown
        );
        assert_eq!(second_observation.jungle_threat(), ThreatReport::Unknown);

        let same_report_different_hidden_state = LaneSnapshot::new(
            M2_LANE_RULESET,
            second.turn(),
            LanePhase::Open,
            second.player(),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(9).expect("bounded"),
                LanePosition::FarSide,
                OpponentPosture::Aggressive,
            ),
            second.wave(),
            second.jungle_threat(),
            None,
        );
        assert_eq!(
            second_observation,
            observe_player(&same_report_different_hidden_state, ObservationId::new(1))
                .observation()
        );
    }

    #[test]
    fn far_side_opponent_report_replays_and_remains_allied_unknown() {
        let initial = LaneSnapshot::initial();
        let state = LaneSnapshot::new(
            initial.ruleset(),
            Turn::new(2),
            LanePhase::Open,
            initial.player(),
            OpponentTruth::new(
                OPPONENT_LANER,
                initial.opponent().health(),
                LanePosition::FarSide,
                initial.opponent().posture(),
            ),
            initial.wave(),
            initial.jungle_threat(),
            None,
        );
        let player_observation = observe_player(&state, ObservationId::new(12)).observation();
        assert_eq!(
            player_observation.opponent().last_known_position(),
            Some(LanePosition::FarSide)
        );
        assert_eq!(
            player_observation.opponent().last_seen_turn(),
            Some(Turn::new(2))
        );
        let allied_observation = observe_allied(&state, ObservationId::new(12)).observation();
        assert_eq!(allied_observation.opponent().last_known_position(), None);
        assert_eq!(allied_observation.opponent().last_seen_turn(), None);

        let (receipt, request) = request(&state, LaneIntent::Stabilize);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
            .expect("append");
        assert_eq!(
            history.records()[0].observation().opponent(),
            player_observation.opponent()
        );
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

    #[test]
    fn observation_reports_only_the_bounded_river_side_last_known_threat() {
        let state = LaneSnapshot::initial();
        let river_side = LaneSnapshot::new(
            state.ruleset(),
            Turn::new(3),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            state.wave(),
            JungleThreatTruth::RiverSide,
            None,
        );
        let river_observation = observe_player(&river_side, ObservationId::new(1)).observation();
        assert_eq!(
            river_observation.jungle_threat(),
            ThreatReport::LastKnown {
                region: JungleThreatRegion::RiverSide,
                last_seen_turn: Turn::new(3),
            }
        );
        assert_eq!(
            river_observation.jungle_threat().last_known_region(),
            Some(JungleThreatRegion::RiverSide)
        );
        assert_eq!(
            river_observation.jungle_threat().last_seen_turn(),
            Some(Turn::new(3))
        );
        assert_eq!(
            observe_player(&state, ObservationId::new(1))
                .observation()
                .jungle_threat(),
            ThreatReport::Unknown
        );
    }

    #[test]
    fn river_side_observation_replays_without_changing_transition_authority() {
        let initial = LaneSnapshot::new(
            M2_LANE_RULESET,
            Turn::new(0),
            LanePhase::Open,
            PlayerLaneState::new(
                PLAYER_LANER,
                LaneHealth::new(8).expect("bounded"),
                LanePosition::Center,
            ),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(7).expect("bounded"),
                LanePosition::Center,
                OpponentPosture::Aggressive,
            ),
            WaveState::new(WavePressure::new(1).expect("bounded")),
            JungleThreatTruth::RiverSide,
            None,
        );
        let (receipt, request) = request(&initial, LaneIntent::Contest);
        assert_eq!(
            receipt.observation().jungle_threat(),
            ThreatReport::LastKnown {
                region: JungleThreatRegion::RiverSide,
                last_seen_turn: Turn::new(0),
            }
        );
        let mut history = LaneHistory::new(initial).expect("valid initial state");
        history
            .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
            .expect("append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        assert_eq!(
            history.current_state().jungle_threat(),
            JungleThreatTruth::RiverSide
        );
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
    fn recall_is_player_legal_but_not_an_allied_policy_candidate() {
        let state = LaneSnapshot::initial();
        let player_observation = observe_player(&state, ObservationId::new(9)).observation();
        assert_eq!(
            player_observation.available_intents(),
            [
                LaneIntent::Stabilize,
                LaneIntent::Contest,
                LaneIntent::Yield,
                LaneIntent::Recall
            ]
        );
        let allied_observation = observe_allied(&state, ObservationId::new(9)).observation();
        let proposal = scripted_allied_proposal(allied_observation, trace(3, 3))
            .expect("allied policy should accept its observation");
        assert_eq!(
            proposal.candidates().map(AlliedCandidate::intent),
            [LaneIntent::Stabilize, LaneIntent::Contest]
        );

        let (receipt, recall_request) = request(&state, LaneIntent::Recall);
        let validated = validate_lane_request(&state, &receipt, &recall_request)
            .expect("recall is legal for the player");
        let result = transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held))
            .expect("recall transition");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            result.next_state().player().position(),
            LanePosition::NearTower
        );
        assert_eq!(result.debrief().intent(), LaneIntent::Recall);
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::PositionChanged {
                cause: LaneEffectCause::Intent,
                ..
            }
        )));
    }

    #[test]
    fn recall_requires_the_current_observation_to_advertise_it() {
        let state = LaneSnapshot::initial();
        let (mut receipt, recall_request) = request(&state, LaneIntent::Recall);
        receipt.observation.available_intents = [
            LaneIntent::Stabilize,
            LaneIntent::Contest,
            LaneIntent::Stabilize,
            LaneIntent::Stabilize,
        ];
        assert_eq!(
            validate_lane_request(&state, &receipt, &recall_request),
            Err(LaneValidationError::UnsupportedIntent)
        );

        let (mut stale_receipt, stale_request) = request(&state, LaneIntent::Recall);
        stale_receipt.source_state_hash = StateHash::from_raw(0);
        assert!(matches!(
            validate_lane_request(&state, &stale_receipt, &stale_request),
            Err(LaneValidationError::StaleObservation)
        ));

        let (current_receipt, current_request) = request(&state, LaneIntent::Recall);
        let validated =
            validate_lane_request(&state, &current_receipt, &current_request).expect("valid");
        let resolved = transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held))
            .expect("transition");
        let resolved_receipt = observe_player(&resolved.next_state(), ObservationId::new(9));
        assert_eq!(
            validate_lane_request(&resolved.next_state(), &resolved_receipt, &current_request),
            Err(LaneValidationError::WindowAlreadyResolved)
        );
    }

    #[test]
    fn recall_can_be_unfavorable_without_becoming_a_fallback() {
        let state = LaneSnapshot::initial();
        let (receipt, recall_request) = request(&state, LaneIntent::Recall);
        let validated =
            validate_lane_request(&state, &receipt, &recall_request).expect("recall is legal");
        let result = transition_lane(&state, &validated, &inputs(8, 0, LaneWaveResult::Held))
            .expect("fatal recall execution remains legal");
        assert_eq!(result.outcome(), LaneOutcome::ForcedOut);
        assert_eq!(result.debrief().intent(), LaneIntent::Recall);
        assert!(!result.debrief().fallback_activated());
        assert!(
            !result
                .events()
                .iter()
                .any(|event| matches!(event, LaneEvent::FallbackActivated { .. }))
        );
    }

    #[test]
    fn recall_replays_and_preserves_objective_attribution() {
        let state = LaneSnapshot::initial();
        let (receipt, recall_request) = request(&state, LaneIntent::Recall);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(
                &receipt,
                &recall_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("recall append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("recall objective review");
        assert_eq!(review.review().intent(), LaneIntent::Recall);
        assert_eq!(review.review().report().intent(), LaneIntent::Recall);
        review
            .verify_lane(record)
            .expect("replayable objective review");
    }

    #[test]
    fn withdraw_is_advertised_only_for_a_river_side_last_known_report() {
        let unknown = LaneSnapshot::initial();
        assert_eq!(
            observe_player(&unknown, ObservationId::new(9))
                .observation()
                .available_threat_response(),
            None
        );

        let river_side = river_side_state();
        let observation = observe_player(&river_side, ObservationId::new(9)).observation();
        assert_eq!(
            observation.available_threat_response(),
            Some(LaneIntent::Withdraw)
        );
        assert_eq!(
            observation.available_intents(),
            [
                LaneIntent::Stabilize,
                LaneIntent::Contest,
                LaneIntent::Yield,
                LaneIntent::Recall
            ]
        );
    }

    #[test]
    fn withdraw_requires_current_last_known_threat_and_preserves_explicit_inputs() {
        let unknown = LaneSnapshot::initial();
        let (unknown_receipt, unknown_request) = request(&unknown, LaneIntent::Withdraw);
        assert_eq!(
            validate_lane_request(&unknown, &unknown_receipt, &unknown_request),
            Err(LaneValidationError::UnsupportedIntent)
        );

        let river_side = river_side_state();
        let (mut stale_receipt, stale_request) = request(&river_side, LaneIntent::Withdraw);
        stale_receipt.source_state_hash = StateHash::from_raw(0);
        assert!(matches!(
            validate_lane_request(&river_side, &stale_receipt, &stale_request),
            Err(LaneValidationError::StaleObservation)
        ));

        let (receipt, withdraw_request) = request(&river_side, LaneIntent::Withdraw);
        let validated = validate_lane_request(&river_side, &receipt, &withdraw_request)
            .expect("current last-known report authorizes withdraw");
        let result = transition_lane(&river_side, &validated, &inputs(1, 2, LaneWaveResult::Lost))
            .expect("withdraw transition");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            result.next_state().player().position(),
            LanePosition::NearTower
        );
        assert_eq!(
            result.next_state().player().health(),
            LaneHealth::new(7).unwrap()
        );
        assert_eq!(
            result.next_state().wave().pressure(),
            WavePressure::new(0).unwrap()
        );
        assert_eq!(result.debrief().intent(), LaneIntent::Withdraw);
        assert!(!result.debrief().fallback_activated());
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::PositionChanged {
                cause: LaneEffectCause::Intent,
                ..
            }
        )));
    }

    #[test]
    fn withdraw_replays_and_preserves_objective_attribution() {
        let state = river_side_state();
        let (receipt, withdraw_request) = request(&state, LaneIntent::Withdraw);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(
                &receipt,
                &withdraw_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("withdraw append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("withdraw objective review");
        assert_eq!(review.review().intent(), LaneIntent::Withdraw);
        review
            .verify_lane(record)
            .expect("replayable objective review");
    }

    #[test]
    fn two_beat_window_closes_on_commit_and_replays_with_a_distinct_hash() {
        let state = two_beat_state();
        assert_eq!(state.window(), LaneWindow::TwoBeats);
        assert!(state.window().closes_on_commit());
        assert_ne!(state.hash(), LaneSnapshot::initial().hash());

        let player_receipt = observe_player(&state, ObservationId::new(9));
        assert_eq!(player_receipt.observation().window(), LaneWindow::TwoBeats);
        let allied_receipt = observe_allied(&state, ObservationId::new(9));
        assert_eq!(allied_receipt.observation().window(), LaneWindow::TwoBeats);
        let proposal = scripted_allied_proposal(allied_receipt.observation(), trace(3, 3))
            .expect("allied policy supports the bounded longer window");
        assert_eq!(
            proposal.candidates().map(AlliedCandidate::intent),
            [LaneIntent::Stabilize, LaneIntent::Contest]
        );

        let request =
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        let result = history
            .append(
                &player_receipt,
                &request,
                inputs(0, 1, LaneWaveResult::Held),
            )
            .expect("two-beat transition");
        assert_eq!(result.next_state().turn(), Turn::new(2));
        assert_eq!(result.next_state().window(), LaneWindow::TwoBeats);
        assert_eq!(result.next_state().phase(), LanePhase::Resolved);
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
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
        assert_eq!(
            result
                .effects()
                .iter()
                .find_map(|effect| match effect {
                    LaneEffect::PositionChanged { provenance, .. } => Some(*provenance),
                    _ => None,
                })
                .expect("fallback position effect"),
            LaneEffectProvenance::indirect_immediate()
        );
    }

    #[test]
    fn explicit_effects_are_direct_immediate_and_have_no_delayed_emission() {
        let state = LaneSnapshot::initial();
        let (receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
        let validated = validate_lane_request(&state, &receipt, &stabilize_request).expect("valid");
        let result = transition_lane(&state, &validated, &inputs(1, 1, LaneWaveResult::Advanced))
            .expect("transition");
        assert_eq!(result.effects().len(), 4);
        assert!(result.effects().iter().all(|effect| {
            let provenance = effect.provenance();
            provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        }));
        assert!(
            result
                .effects()
                .iter()
                .all(|effect| { effect.provenance().timing() != LaneEffectTiming::Delayed })
        );
    }

    #[test]
    fn mana_is_bounded_visible_and_binds_non_full_hashes_and_policy_inputs() {
        let full = LaneSnapshot::initial();
        let reduced_player = PlayerLaneState::new_with_mana(
            full.player().id(),
            full.player().health(),
            LaneMana::new(4).expect("bounded mana"),
            full.player().position(),
        );
        let reduced = LaneSnapshot::new(
            full.ruleset(),
            full.turn(),
            full.phase(),
            reduced_player,
            full.opponent(),
            full.wave(),
            full.jungle_threat(),
            full.terminal_outcome(),
        );
        assert_eq!(
            observe_player(&full, ObservationId::new(1))
                .observation()
                .self_mana(),
            LaneMana::full()
        );
        assert_eq!(
            observe_player(&reduced, ObservationId::new(1))
                .observation()
                .self_mana(),
            LaneMana::new(4).expect("bounded mana")
        );
        let full_allied = observe_allied(&full, ObservationId::new(1)).observation();
        let reduced_allied = observe_allied(&reduced, ObservationId::new(1)).observation();
        assert_eq!(full_allied.laner_mana(), LaneMana::full());
        assert_eq!(
            reduced_allied.laner_mana(),
            LaneMana::new(4).expect("bounded mana")
        );
        assert_ne!(full.hash(), reduced.hash());
        assert_ne!(
            allied_input_identity(full_allied, trace(3, 3)).visible_digest(),
            allied_input_identity(reduced_allied, trace(3, 3)).visible_digest()
        );
    }

    #[test]
    fn contest_mana_spend_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        let (receipt, contest_request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &contest_request).expect("valid");
        let spent = LaneMana::new(1).expect("bounded spend");
        let resolved_inputs = inputs(0, 1, LaneWaveResult::Advanced).with_mana_spent(spent);
        let result =
            transition_lane(&state, &validated, &resolved_inputs).expect("contest spend is valid");
        assert_eq!(
            result.next_state().player().mana(),
            LaneMana::new(5).unwrap()
        );
        assert_eq!(result.debrief().mana_spent(), spent);
        assert!(result.events().iter().any(|event| matches!(
            event,
            LaneEvent::ManaSpent {
                actor: PLAYER_LANER,
                amount,
                ..
            } if *amount == spent
        )));
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::ManaChanged {
                actor: PLAYER_LANER,
                before,
                after,
                cause: LaneEffectCause::Execution(_),
                provenance,
            } if *before == LaneMana::full()
                && *after == LaneMana::new(5).unwrap()
                && *provenance == LaneEffectProvenance::direct_immediate()
        )));
        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &contest_request, resolved_inputs)
            .expect("append");
        assert_eq!(
            history.verify_replay().expect("replay"),
            result.next_state()
        );
    }

    #[test]
    fn mana_spend_rejects_wrong_intent_and_insufficient_resource() {
        let state = LaneSnapshot::initial();
        let (stabilize_receipt, stabilize_request) = request(&state, LaneIntent::Stabilize);
        let stabilize =
            validate_lane_request(&state, &stabilize_receipt, &stabilize_request).expect("valid");
        let spent = LaneMana::new(1).expect("bounded spend");
        assert_eq!(
            transition_lane(
                &state,
                &stabilize,
                &inputs(0, 0, LaneWaveResult::Held).with_mana_spent(spent),
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ManaSpentWithoutContest {
                    intent: LaneIntent::Stabilize,
                    spent,
                }
            ))
        );

        let empty_player = PlayerLaneState::new_with_mana(
            state.player().id(),
            state.player().health(),
            LaneMana::zero(),
            state.player().position(),
        );
        let empty = LaneSnapshot::new(
            state.ruleset(),
            state.turn(),
            state.phase(),
            empty_player,
            state.opponent(),
            state.wave(),
            state.jungle_threat(),
            state.terminal_outcome(),
        );
        let (receipt, request) = request(&empty, LaneIntent::Contest);
        let validated = validate_lane_request(&empty, &receipt, &request).expect("valid");
        assert_eq!(
            transition_lane(
                &empty,
                &validated,
                &inputs(0, 0, LaneWaveResult::Held).with_mana_spent(spent),
            ),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ManaExceedsAvailable {
                    spent,
                    available: LaneMana::zero(),
                }
            ))
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
    fn matched_branch_clears_contest_only_mana_for_non_contest_and_binds_identity() {
        let state = LaneSnapshot::initial();
        let (receipt, parent_request) = request(&state, LaneIntent::Contest);
        let spent = LaneMana::new(1).expect("bounded spend");
        let spent_inputs = inputs(1, 1, LaneWaveResult::Held).with_mana_spent(spent);
        let mut spent_parent = LaneHistory::new(state).expect("valid");
        spent_parent
            .append(&receipt, &parent_request, spent_inputs)
            .expect("append");
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );
        let spent_branch = branch_from_window(
            &spent_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("matched branch normalizes intent-scoped spend");
        assert_eq!(
            spent_branch.record().inputs().execution().mana_spent(),
            LaneMana::zero()
        );
        assert_eq!(
            spent_branch.identity().mana_policy(),
            LaneBranchManaPolicy::NonContestSpendCleared
        );
        let spent_review = spent_branch.review(&spent_parent).expect("review");
        assert_eq!(
            spent_review.execution_relation(),
            LaneExecutionRelation::MatchedWithResourceNormalization
        );
        assert_eq!(
            spent_review.attribution_limit(),
            LaneAttributionLimit::DecisionAndResourceChanged
        );
        spent_branch
            .verify_replay(&spent_parent)
            .expect("branch replay");

        let mut no_spend_parent = LaneHistory::new(state).expect("valid");
        no_spend_parent
            .append(
                &receipt,
                &parent_request,
                inputs(1, 1, LaneWaveResult::Held),
            )
            .expect("append");
        let no_spend_branch = branch_from_window(
            &no_spend_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("matched branch");
        assert_eq!(
            no_spend_branch.identity().mana_policy(),
            LaneBranchManaPolicy::ParentSpendPreserved
        );
        assert_ne!(
            spent_branch.identity().parent_record_identity(),
            no_spend_branch.identity().parent_record_identity()
        );
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
    fn parent_record_identity_preserves_neutral_input_provenance() {
        let state = LaneSnapshot::initial();
        let (receipt, parent_request) = request(&state, LaneIntent::Contest);
        let parent_inputs = inputs(1, 1, LaneWaveResult::Held);
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );

        let mut first_parent = LaneHistory::new(state).expect("valid");
        first_parent
            .append(&receipt, &parent_request, parent_inputs)
            .expect("append");
        let changed_neutral_inputs = LaneResolvedInputs::new(
            trace(101, 101),
            trace(102, 102),
            trace(103, 103),
            trace(104, 104),
            parent_inputs.execution(),
        );
        let mut second_parent = LaneHistory::new(state).expect("valid");
        second_parent
            .append(&receipt, &parent_request, changed_neutral_inputs)
            .expect("append");

        let first_branch = branch_from_window(
            &first_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("branch");
        let second_branch = branch_from_window(
            &second_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("branch");
        assert_eq!(
            first_branch.record().result(),
            second_branch.record().result()
        );
        assert_ne!(
            first_branch.identity().parent_record_identity(),
            second_branch.identity().parent_record_identity()
        );
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

    fn coordinated_offer(
        state: &LaneSnapshot,
        policy_trace: InputTrace,
    ) -> (
        LaneObservationReceipt,
        AlliedObservationReceipt,
        AlliedProposalOffer,
    ) {
        let player_receipt = observe_player(state, ObservationId::new(9));
        let allied_receipt = observe_allied(state, ObservationId::new(9));
        let proposal = scripted_allied_proposal(allied_receipt.observation(), policy_trace)
            .expect("canonical proposal");
        let offer = offer_allied_proposal(proposal).expect("canonical offer");
        (player_receipt, allied_receipt, offer)
    }

    fn counter_to_stabilize(proposal_id: ProposalId) -> ProposalResponse {
        ProposalResponse::Counter {
            proposal_id,
            counter: CounterProposal::RequestIntent {
                requested_intent: LaneIntent::Stabilize,
                target: PLAYER_LANER,
                commitment: CoordinationCommitment::UntilWindowEnd,
                focus: SupportFocus::Wave,
                abort: SupportAbort::IfPlayerHealthAtMost(2),
                fallback: SupportFallback::HoldPosition,
            },
        }
    }

    #[test]
    fn allied_policy_is_visible_input_bound_and_hidden_state_invariant() {
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
        let first_receipt = observe_allied(&first, ObservationId::new(12));
        let second_receipt = observe_allied(&second, ObservationId::new(12));
        assert_eq!(first_receipt.observation(), second_receipt.observation());
        let first_proposal =
            scripted_allied_proposal(first_receipt.observation(), trace(3, 3)).expect("proposal");
        let second_proposal =
            scripted_allied_proposal(second_receipt.observation(), trace(3, 3)).expect("proposal");
        assert_eq!(first_proposal, second_proposal);
        assert_eq!(
            first_proposal.profile().profile_id(),
            SCRIPTED_ALLIED_PROFILE
        );
        assert_eq!(first_proposal.candidates()[0].score(), 2);
        assert_eq!(first_proposal.candidates()[1].score(), 5);
        assert_eq!(first_proposal.selected_intent(), LaneIntent::Contest);
        assert_eq!(
            offer_allied_proposal(first_proposal)
                .expect("offer")
                .support(),
            AlliedSupport::AssistContest
        );
    }

    #[test]
    fn allied_policy_changes_only_with_declared_visible_features() {
        let state = LaneSnapshot::new(
            M2_LANE_RULESET,
            Turn::new(0),
            LanePhase::Open,
            PlayerLaneState::new(
                PLAYER_LANER,
                LaneHealth::new(2).expect("bounded"),
                LanePosition::Center,
            ),
            LaneSnapshot::initial().opponent(),
            WaveState::new(WavePressure::new(3).expect("bounded")),
            JungleThreatTruth::InLane,
            None,
        );
        let receipt = observe_allied(&state, ObservationId::new(13));
        let proposal =
            scripted_allied_proposal(receipt.observation(), trace(3, 3)).expect("proposal");
        assert_eq!(proposal.candidates()[0].score(), 6);
        assert_eq!(proposal.candidates()[1].score(), 6);
        assert_eq!(proposal.selected_intent(), LaneIntent::Stabilize);
    }

    #[test]
    fn coordinated_accept_keeps_execution_and_state_in_the_base_lane_contract() {
        let state = LaneSnapshot::initial();
        let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
        let request = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
            ProposalResponse::Accept {
                proposal_id: offer.proposal().id(),
            },
        );
        let coordination_inputs =
            CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted);
        let lane_inputs = inputs(1, 2, LaneWaveResult::Advanced);
        let coordinated = resolve_coordinated_lane(
            &state,
            &player_receipt,
            &allied_receipt,
            &offer,
            &request,
            &coordination_inputs,
            &lane_inputs,
        )
        .expect("coordinated transition");
        let validated = validate_lane_request(&state, &player_receipt, &request.intent())
            .expect("base request");
        let base = transition_lane(&state, &validated, &lane_inputs).expect("base transition");
        assert_eq!(coordinated.next_state(), base.next_state());
        assert_eq!(coordinated.state_hash(), base.state_hash());
        assert_eq!(
            coordinated.coordination().disposition(),
            CoordinationDisposition::AcceptedOffer
        );
        assert!(matches!(
            coordinated.events()[0],
            CoordinatedEvent::ProposalOffered { .. }
        ));
        assert!(matches!(
            coordinated.effects()[0],
            CoordinatedEffect::SupportCommitted { .. }
        ));
        assert_eq!(
            coordinated.debrief().execution(),
            CoordinatedExecutionReview::ConditionalOnCoordination { trace: trace(5, 0) }
        );
    }

    #[test]
    fn coordination_maps_closed_responses_and_rejects_malformed_inputs() {
        let state = LaneSnapshot::initial();
        let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
        let accepted = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
            ProposalResponse::Accept {
                proposal_id: offer.proposal().id(),
            },
        );
        assert_eq!(
            resolve_coordination(
                &offer,
                &accepted,
                &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted),
            )
            .expect("accepted")
            .disposition(),
            CoordinationDisposition::AcceptedOffer
        );
        assert_eq!(
            resolve_coordination(
                &offer,
                &accepted,
                &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyDeclined),
            )
            .expect("declined")
            .disposition(),
            CoordinationDisposition::AllyDeclined
        );
        let rejected = CoordinatedLaneRequest::new(
            accepted.intent(),
            ProposalResponse::Reject {
                proposal_id: offer.proposal().id(),
            },
        );
        assert_eq!(
            resolve_coordination(
                &offer,
                &rejected,
                &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::NotRequested),
            )
            .expect("rejected")
            .disposition(),
            CoordinationDisposition::PlayerRejected
        );
        let counter = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Stabilize),
            counter_to_stabilize(offer.proposal().id()),
        );
        assert_eq!(
            resolve_coordination(
                &offer,
                &counter,
                &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted),
            )
            .expect("counter accepted")
            .disposition(),
            CoordinationDisposition::CounterAccepted
        );
        assert_eq!(
            resolve_coordination(
                &offer,
                &counter,
                &CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyDeclined),
            )
            .expect("counter declined")
            .disposition(),
            CoordinationDisposition::CounterRejected
        );
        let invalid_accept = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Stabilize),
            ProposalResponse::Accept {
                proposal_id: offer.proposal().id(),
            },
        );
        assert_eq!(
            validate_coordinated_request(
                &state,
                &player_receipt,
                &allied_receipt,
                &offer,
                &invalid_accept,
                trace(3, 3),
            ),
            Err(CoordinationError::AcceptIntentMismatch)
        );
        assert_eq!(
            resolve_coordination(
                &offer,
                &accepted,
                &CoordinationResolutionInputs::new(trace(4, 5), FollowThrough::NotRequested),
            ),
            Err(CoordinationError::MalformedFollowThrough)
        );
    }

    #[test]
    fn coordinated_history_replays_and_rejects_tampering() {
        let state = LaneSnapshot::initial();
        let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
        let request = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
            ProposalResponse::Reject {
                proposal_id: offer.proposal().id(),
            },
        );
        let mut history = CoordinatedLaneHistory::new(state).expect("valid initial state");
        history
            .append(
                &player_receipt,
                &allied_receipt,
                &offer,
                &request,
                CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::NotRequested),
                inputs(1, 1, LaneWaveResult::Held),
            )
            .expect("append");
        assert_eq!(history.records().len(), 1);
        assert_eq!(history.records()[0].replay_id(), M2_COORDINATION_REPLAY_ID);
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        history.records[0].request = CoordinatedLaneRequest::new(
            request.intent(),
            ProposalResponse::Reject {
                proposal_id: ProposalId(0),
            },
        );
        assert_eq!(
            history.verify_replay(),
            Err(CoordinationError::ReplayMismatch)
        );
        history.records[0].request = request;
        history.records[0].base_record.command = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            M2_LANE_RULESET,
            ObservationId::new(9),
            StateHash::from_raw(0),
            LaneIntent::Contest,
        );
        assert_eq!(
            history.verify_replay(),
            Err(CoordinationError::ReplayMismatch)
        );
    }

    #[test]
    fn hold_lane_objective_classifies_committed_lane_facts_without_changing_state() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
            .expect("append");
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("objective review");
        assert_eq!(review.source_replay_id(), M2_REPLAY_ID);
        assert_eq!(
            review.review().disposition(),
            ObjectiveDisposition::GoalAchieved
        );
        assert_eq!(
            review.review().criteria()[0].status(),
            ObjectiveCriterionStatus::Met
        );
        assert_eq!(
            review.review().criteria()[1].status(),
            ObjectiveCriterionStatus::Met
        );
        assert_eq!(
            review.review().attribution_limit(),
            ObjectiveAttributionLimit::CommittedFactsOnly
        );
        assert_eq!(review.review().report().schema(), M2_OBJECTIVE_SCHEMA);
        review.verify_lane(record).expect("objective replay");
        assert_eq!(history.current_state(), record.result().next_state());
        assert_eq!(
            record.result().state_hash(),
            record.result().next_state().hash()
        );
    }

    #[test]
    fn objective_covers_yielded_forced_out_partial_and_coordination_cases() {
        let direct_partial = ObjectiveEvaluationInputs::new(
            M2_REPLAY_ID,
            StateHash::from_raw(1),
            StateHash::from_raw(2),
            LaneOutcome::HeldSpace,
            LanePosition::Center,
            LaneHealth::zero(),
            LaneIntent::Contest,
            LaneWaveResult::Held,
            ObjectiveCoordination::NotApplicable,
            trace(5, 0),
        );
        assert_eq!(
            evaluate_terminal_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, &direct_partial,)
                .expect("partial")
                .disposition(),
            ObjectiveDisposition::GoalPartiallyAchieved
        );

        let state = LaneSnapshot::initial();
        let (stable_receipt, stable_request) = request(&state, LaneIntent::Stabilize);
        let mut stable_history = LaneHistory::new(state).expect("valid");
        stable_history
            .append(
                &stable_receipt,
                &stable_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("stable append");
        let stable_review = review_lane_objective(
            ScenarioGoal::HoldLaneSpaceThroughWindow,
            &stable_history.records()[0],
        )
        .expect("stable objective");
        assert_eq!(
            stable_review.review().disposition(),
            ObjectiveDisposition::GoalMissed
        );

        let (forced_receipt, forced_request) = request(&state, LaneIntent::Contest);
        let mut forced_history = LaneHistory::new(state).expect("valid");
        forced_history
            .append(
                &forced_receipt,
                &forced_request,
                inputs(8, 0, LaneWaveResult::Held),
            )
            .expect("forced append");
        let forced_review = review_lane_objective(
            ScenarioGoal::HoldLaneSpaceThroughWindow,
            &forced_history.records()[0],
        )
        .expect("forced objective");
        assert_eq!(
            forced_review.review().disposition(),
            ObjectiveDisposition::GoalMissed
        );

        let (player_receipt, allied_receipt, offer) = coordinated_offer(&state, trace(3, 3));
        let coordinated_request = CoordinatedLaneRequest::new(
            LaneIntentRequest::new(PLAYER_LANER, ObservationId::new(9), LaneIntent::Contest),
            ProposalResponse::Accept {
                proposal_id: offer.proposal().id(),
            },
        );
        let mut coordinated_history = CoordinatedLaneHistory::new(state).expect("valid");
        coordinated_history
            .append(
                &player_receipt,
                &allied_receipt,
                &offer,
                &coordinated_request,
                CoordinationResolutionInputs::new(trace(4, 4), FollowThrough::AllyCommitted),
                inputs(1, 1, LaneWaveResult::Held),
            )
            .expect("coordinated append");
        let coordinated_review = review_coordinated_objective(
            ScenarioGoal::HoldLaneSpaceThroughWindow,
            &coordinated_history.records()[0],
        )
        .expect("coordinated objective");
        assert_eq!(
            coordinated_review.review().coordination(),
            ObjectiveCoordination::Resolved(CoordinationDisposition::AcceptedOffer)
        );
        coordinated_review
            .verify_coordinated(&coordinated_history.records()[0])
            .expect("coordinated objective replay");
    }

    #[test]
    fn objective_replay_rejects_tampered_inputs_or_review_and_hides_state_hash_from_report() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let mut history = LaneHistory::new(state).expect("valid");
        history
            .append(&receipt, &request, inputs(1, 1, LaneWaveResult::Held))
            .expect("append");
        let record = &history.records()[0];
        let mut review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("review");
        let report = review.review().report();
        assert!(!format!("{report:?}").contains(&state.hash().value().to_string()));
        review.inputs = ObjectiveEvaluationInputs::new(
            M2_REPLAY_ID,
            StateHash::from_raw(999),
            review.inputs.terminal_state_hash(),
            review.inputs.outcome(),
            review.inputs.player_position(),
            review.inputs.player_health(),
            review.inputs.intent(),
            review.inputs.wave_result(),
            review.inputs.coordination(),
            review.inputs.execution_trace(),
        );
        assert_eq!(
            review.verify_lane(record),
            Err(ObjectiveError::ReviewMismatch)
        );

        let unsupported = ObjectiveEvaluationInputs::new(
            "unsupported-replay",
            StateHash::from_raw(1),
            StateHash::from_raw(2),
            LaneOutcome::HeldSpace,
            LanePosition::Center,
            LaneHealth::new(1).expect("bounded"),
            LaneIntent::Contest,
            LaneWaveResult::Held,
            ObjectiveCoordination::NotApplicable,
            trace(5, 0),
        );
        assert_eq!(
            evaluate_terminal_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, &unsupported,),
            Err(ObjectiveError::UnsupportedReplayId)
        );
    }

    #[test]
    fn named_strategy_fixtures_are_matched_input_and_replayable() {
        let fixtures = [
            StrategyFixtureId::HappyPath,
            StrategyFixtureId::RiskTaking,
            StrategyFixtureId::Conservative,
        ];
        let mut outcomes = Vec::new();
        for id in fixtures {
            let fixture = strategy_fixture(id).expect("fixture");
            let first = run_strategy_fixture(fixture).expect("first run");
            let second = run_strategy_fixture(fixture).expect("second run");
            assert_eq!(first.objective().review(), second.objective().review());
            assert_eq!(
                first.history().records()[0].result(),
                second.history().records()[0].result()
            );
            first
                .history()
                .verify_replay()
                .expect("fixture history replay");
            first
                .objective()
                .verify_coordinated(&first.history().records()[0])
                .expect("fixture objective replay");
            outcomes.push((
                fixture.id(),
                first.history().records()[0].result().lane().outcome(),
                first.objective().review().disposition(),
            ));
        }
        assert_eq!(
            outcomes,
            vec![
                (
                    StrategyFixtureId::HappyPath,
                    LaneOutcome::HeldSpace,
                    ObjectiveDisposition::GoalAchieved,
                ),
                (
                    StrategyFixtureId::RiskTaking,
                    LaneOutcome::YieldedSpace,
                    ObjectiveDisposition::GoalMissed,
                ),
                (
                    StrategyFixtureId::Conservative,
                    LaneOutcome::YieldedSpace,
                    ObjectiveDisposition::GoalMissed,
                ),
            ]
        );
        let mut tampered = strategy_fixture(StrategyFixtureId::RiskTaking).expect("fixture");
        tampered.expected_outcome = LaneOutcome::HeldSpace;
        assert!(matches!(
            run_strategy_fixture(tampered),
            Err(StrategyFixtureError::UnexpectedOutcome)
        ));
    }

    #[test]
    fn two_window_scenario_reopens_once_and_replays_both_commits() {
        let initial = LaneSnapshot::initial();
        assert!(matches!(
            reopen_resolved_snapshot(&initial),
            Err(ScenarioError::InvalidReopenState)
        ));
        let mut history = LaneScenarioHistory::new(initial).expect("valid scenario");
        let (first_receipt, first_request) = request(&initial, LaneIntent::Contest);
        let first = history
            .append(
                &first_receipt,
                &first_request,
                inputs(0, 1, LaneWaveResult::Advanced),
            )
            .expect("first window");
        let mut tampered_result = first.clone();
        tampered_result.state_hash = StateHash::from_raw(0);
        assert_eq!(
            reopen_lane_window(&tampered_result),
            Err(ScenarioError::InvalidReopenState)
        );
        let reopened = history.current_state();
        assert_eq!(first.next_state().phase(), LanePhase::Resolved);
        assert_eq!(reopened.phase(), LanePhase::Open);
        assert_eq!(reopened.turn(), Turn::new(1));
        assert_eq!(reopened.player(), first.next_state().player());
        assert_eq!(reopened.opponent(), first.next_state().opponent());
        assert_eq!(reopened.wave(), first.next_state().wave());
        assert_eq!(reopened.jungle_threat(), first.next_state().jungle_threat());
        assert_eq!(reopened.terminal_outcome(), None);
        assert_eq!(history.records()[0].window(), ScenarioWindow::First);
        assert_eq!(history.records()[0].reopened_state(), Some(reopened));
        assert_eq!(
            history.terminal_state(),
            Err(ScenarioError::ScenarioIncomplete)
        );

        let (second_receipt, second_request) = request(&reopened, LaneIntent::Stabilize);
        let second = history
            .append(
                &second_receipt,
                &second_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("second window");
        assert_eq!(history.records().len(), 2);
        assert_eq!(history.records()[1].window(), ScenarioWindow::Second);
        assert_eq!(history.records()[1].reopened_state(), None);
        assert_eq!(history.current_state(), second.next_state());
        assert_eq!(history.current_state().phase(), LanePhase::Resolved);
        assert_eq!(history.terminal_state(), Ok(history.current_state()));
        history.verify_replay().expect("scenario replay");
        assert_eq!(
            review_lane_objective(
                ScenarioGoal::HoldLaneSpaceThroughWindow,
                history.records()[0].transition(),
            )
            .expect("first objective")
            .review()
            .disposition(),
            ObjectiveDisposition::GoalAchieved
        );
        assert!(matches!(
            history.append(
                &second_receipt,
                &second_request,
                inputs(0, 0, LaneWaveResult::Held),
            ),
            Err(ScenarioError::ScenarioComplete)
        ));
    }

    #[test]
    fn two_window_scenario_replay_rejects_reopen_and_record_tampering() {
        let initial = LaneSnapshot::initial();
        let mut history = LaneScenarioHistory::new(initial).expect("valid scenario");
        let (first_receipt, first_request) = request(&initial, LaneIntent::Contest);
        history
            .append(
                &first_receipt,
                &first_request,
                inputs(0, 0, LaneWaveResult::Held),
            )
            .expect("first window");
        let reopened = history.current_state();
        let (second_receipt, second_request) = request(&reopened, LaneIntent::Contest);
        history
            .append(
                &second_receipt,
                &second_request,
                inputs(3, 0, LaneWaveResult::Lost),
            )
            .expect("second window");
        history.records[0].reopened_state = Some(initial);
        assert_eq!(history.verify_replay(), Err(ScenarioError::ReplayMismatch));
    }

    #[test]
    fn final_debrief_replays_committed_window_facts_and_redacts_provenance() {
        let build_history = || {
            let initial = LaneSnapshot::initial();
            let mut history = LaneScenarioHistory::new(initial).expect("valid scenario");
            let (first_receipt, first_request) = request(&initial, LaneIntent::Contest);
            history
                .append(
                    &first_receipt,
                    &first_request,
                    inputs(0, 1, LaneWaveResult::Advanced),
                )
                .expect("first window");
            let reopened = history.current_state();
            let (second_receipt, second_request) = request(&reopened, LaneIntent::Stabilize);
            history
                .append(
                    &second_receipt,
                    &second_request,
                    inputs(0, 0, LaneWaveResult::Held),
                )
                .expect("second window");
            history
        };
        let history = build_history();
        let debrief = build_scenario_debrief(&history).expect("debrief");
        assert_eq!(debrief.replay_id(), M2_FINAL_DEBRIEF_REPLAY_ID);
        assert_eq!(debrief.source_replay_id(), M2_TWO_WINDOW_REPLAY_ID);
        assert_eq!(debrief.windows()[0].window(), ScenarioWindow::First);
        assert_eq!(debrief.windows()[0].intent(), LaneIntent::Contest);
        assert_eq!(
            debrief.windows()[0].objective().disposition(),
            ObjectiveDisposition::GoalAchieved
        );
        assert_eq!(debrief.windows()[1].window(), ScenarioWindow::Second);
        assert_eq!(debrief.windows()[1].intent(), LaneIntent::Stabilize);
        assert_eq!(debrief.final_objective(), ObjectiveDisposition::GoalMissed);
        assert_eq!(
            debrief.attribution_limit(),
            FinalDebriefAttributionLimit::CommittedHistoryFactsOnly
        );
        assert!(
            !format!("{:?}", debrief.report())
                .contains(&debrief.source_terminal_state_hash().value().to_string())
        );
        debrief.verify_replay(&history).expect("debrief replay");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));

        let mut tampered = debrief.clone();
        tampered.source_terminal_state_hash = StateHash::from_raw(0);
        assert_eq!(
            tampered.verify_replay(&history),
            Err(ScenarioDebriefError::ReplayMismatch)
        );

        let incomplete = LaneScenarioHistory::new(LaneSnapshot::initial()).expect("valid");
        assert_eq!(
            build_scenario_debrief(&incomplete),
            Err(ScenarioDebriefError::IncompleteHistory)
        );
    }

    #[test]
    fn yield_is_player_legal_and_resolves_to_near_tower() {
        let state = LaneSnapshot::initial();
        let (receipt, yield_request) = request(&state, LaneIntent::Yield);
        let validated = validate_lane_request(&state, &receipt, &yield_request)
            .expect("yield is legal for the player");
        let result = transition_lane(&state, &validated, &inputs(0, 0, LaneWaveResult::Held))
            .expect("yield transition");
        assert_eq!(result.outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            result.next_state().player().position(),
            LanePosition::NearTower
        );
        assert_eq!(result.debrief().intent(), LaneIntent::Yield);
        assert!(result.effects().iter().any(|effect| matches!(
            effect,
            LaneEffect::PositionChanged {
                cause: LaneEffectCause::Intent,
                ..
            }
        )));
    }

    #[test]
    fn yield_replays_and_preserves_objective_attribution() {
        let state = LaneSnapshot::initial();
        let (receipt, yield_request) = request(&state, LaneIntent::Yield);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(&receipt, &yield_request, inputs(0, 0, LaneWaveResult::Held))
            .expect("yield append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        let record = &history.records()[0];
        let review = review_lane_objective(ScenarioGoal::HoldLaneSpaceThroughWindow, record)
            .expect("yield objective review");
        assert_eq!(review.review().intent(), LaneIntent::Yield);
        assert_eq!(review.review().report().intent(), LaneIntent::Yield);
        review
            .verify_lane(record)
            .expect("replayable objective review");
    }

    #[test]
    fn yield_rejects_mana_spend() {
        let state = LaneSnapshot::initial();
        let (receipt, yield_request) = request(&state, LaneIntent::Yield);
        let validated = validate_lane_request(&state, &receipt, &yield_request).expect("valid");
        let mut invalid_inputs = inputs(0, 0, LaneWaveResult::Held);
        invalid_inputs.execution.mana_spent = LaneMana::new(1).unwrap();
        assert!(matches!(
            transition_lane(&state, &validated, &invalid_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::ManaSpentWithoutContest {
                    intent: LaneIntent::Yield,
                    ..
                }
            ))
        ));
    }

    #[test]
    fn gold_is_bounded_and_default_zero() {
        assert_eq!(LaneGold::zero().value(), 0);
        assert_eq!(LaneGold::new(20).unwrap().value(), 20);
        assert!(LaneGold::new(21).is_err());
        let zero = LaneGold::zero();
        let earned = LaneGold::new(5).unwrap();
        assert_eq!(zero.add(earned), Some(earned));
        assert_eq!(earned.add(LaneGold::new(16).unwrap()), None);
        assert_eq!(
            earned.subtract(LaneGold::new(2).unwrap()),
            Some(LaneGold::new(3).unwrap())
        );
    }

    #[test]
    fn gold_earned_is_direct_immediate_and_replayable() {
        let state = LaneSnapshot::initial();
        assert_eq!(state.player().gold(), LaneGold::zero());

        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut gold_inputs = inputs(1, 2, LaneWaveResult::Held);
        gold_inputs.execution = gold_inputs
            .execution
            .with_gold_earned(LaneGold::new(5).unwrap());

        let result =
            transition_lane(&state, &validated, &gold_inputs).expect("transition with gold");
        assert_eq!(
            result.next_state().player().gold(),
            LaneGold::new(5).unwrap()
        );
        assert_ne!(state.hash(), result.next_state().hash());

        let player_obs = observe_player(&result.next_state(), ObservationId::new(1)).observation();
        assert_eq!(player_obs.self_gold(), LaneGold::new(5).unwrap());
        let allied_obs = observe_allied(&result.next_state(), ObservationId::new(2)).observation();
        assert_eq!(allied_obs.laner_gold(), LaneGold::new(5).unwrap());

        assert_eq!(result.debrief().gold_earned(), LaneGold::new(5).unwrap());

        assert!(result.events().iter().any(|e| matches!(
            e,
            LaneEvent::GoldEarned { amount, .. } if *amount == LaneGold::new(5).unwrap()
        )));
        assert!(result.effects().iter().any(|e| matches!(
            e,
            LaneEffect::GoldChanged {
                before,
                after,
                provenance,
                ..
            } if *before == LaneGold::zero()
                && *after == LaneGold::new(5).unwrap()
                && provenance.relation() == LaneEffectRelation::Direct
                && provenance.timing() == LaneEffectTiming::Immediate
        )));

        let mut history = LaneHistory::new(state).expect("valid history");
        history
            .append(&receipt, &request, gold_inputs)
            .expect("append gold execution");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

    #[test]
    fn gold_overflow_is_rejected() {
        let state = LaneSnapshot::initial();
        let (receipt, request) = request(&state, LaneIntent::Contest);
        let validated = validate_lane_request(&state, &receipt, &request).expect("valid");
        let mut overflow_inputs = inputs(1, 2, LaneWaveResult::Held);
        overflow_inputs.execution.gold_earned = LaneGold(21); // bypass constructor for error test

        assert!(matches!(
            transition_lane(&state, &validated, &overflow_inputs),
            Err(LaneTransitionError::Execution(
                LaneExecutionError::GoldOverflow { .. }
            ))
        ));
    }
}
