use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObservationId(pub(crate) u64);

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
    pub(crate) last_known_position: Option<LanePosition>,
    pub(crate) last_seen_turn: Option<Turn>,
    pub(crate) health: HiddenValue,
    pub(crate) posture: HiddenValue,
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
pub struct LanerObservation {
    pub(crate) schema: &'static str,
    pub(crate) observer: ActorId,
    pub(crate) turn: Turn,
    pub(crate) observation_id: ObservationId,
    pub(crate) self_health: LaneHealth,
    pub(crate) self_mana: LaneMana,
    pub(crate) self_gold: LaneGold,
    pub(crate) self_experience: LaneExperience,
    pub(crate) self_cooldown: LaneCooldown,
    pub(crate) self_bounty: LaneBounty,
    pub(crate) self_level: LaneLevel,
    pub(crate) self_minion_kills: LaneMinionKills,
    pub(crate) self_shield: LaneShield,
    pub(crate) self_ward: LaneWard,
    pub(crate) self_position: LanePosition,
    pub(crate) wave_pressure: WavePressure,
    pub(crate) opponent: OpponentReport,
    pub(crate) jungle_threat: ThreatReport,
    pub(crate) available_intents: [LaneIntent; 4],
    pub(crate) available_threat_response: Option<LaneIntent>,
    pub(crate) available_target_focuses: [LaneTargetFocus; 3],
    pub(crate) available_commitments: [LaneCommitment; 3],
    pub(crate) available_ping_signals: [LanePingSignal; 5],
    pub(crate) available_abort_conditions: [LaneAbortCondition; 4],
    pub(crate) window: LaneWindow,
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

    pub fn self_experience(self) -> LaneExperience {
        self.self_experience
    }

    pub fn self_cooldown(self) -> LaneCooldown {
        self.self_cooldown
    }

    pub fn self_bounty(self) -> LaneBounty {
        self.self_bounty
    }

    pub fn self_level(self) -> LaneLevel {
        self.self_level
    }

    pub fn self_minion_kills(self) -> LaneMinionKills {
        self.self_minion_kills
    }

    pub fn self_shield(self) -> LaneShield {
        self.self_shield
    }

    pub fn self_ward(self) -> LaneWard {
        self.self_ward
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

    pub fn available_target_focuses(self) -> [LaneTargetFocus; 3] {
        self.available_target_focuses
    }

    pub fn available_commitments(self) -> [LaneCommitment; 3] {
        self.available_commitments
    }

    pub fn available_ping_signals(self) -> [LanePingSignal; 5] {
        self.available_ping_signals
    }

    pub fn available_abort_conditions(self) -> [LaneAbortCondition; 4] {
        self.available_abort_conditions
    }

    pub fn window(self) -> LaneWindow {
        self.window
    }
}

#[derive(Clone, Copy)]
pub struct LaneObservationReceipt {
    pub(crate) observation: LanerObservation,
    pub(crate) source_state_hash: StateHash,
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
            self_experience: state.player().experience(),
            self_cooldown: state.player().cooldown(),
            self_bounty: state.player().bounty(),
            self_level: state.player().level(),
            self_minion_kills: state.player().minion_kills(),
            self_shield: state.player().shield(),
            self_ward: state.player().ward(),
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
            available_target_focuses: [
                LaneTargetFocus::Minions,
                LaneTargetFocus::OpposingLaner,
                LaneTargetFocus::Tower,
            ],
            available_commitments: [
                LaneCommitment::Standard,
                LaneCommitment::Cautious,
                LaneCommitment::Aggressive,
            ],
            available_ping_signals: [
                LanePingSignal::None,
                LanePingSignal::Danger,
                LanePingSignal::OnMyWay,
                LanePingSignal::Assist,
                LanePingSignal::EnemyMissing,
            ],
            available_abort_conditions: [
                LaneAbortCondition::None,
                LaneAbortCondition::HealthThreshold,
                LaneAbortCondition::ThreatSpotted,
                LaneAbortCondition::ResourceDepleted,
            ],
            window: state.window(),
        },
        source_state_hash: state.hash(),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AlliedLaneObservation {
    pub(crate) schema: &'static str,
    pub(crate) observer: ActorId,
    pub(crate) turn: Turn,
    pub(crate) observation_id: ObservationId,
    pub(crate) laner_health: LaneHealth,
    pub(crate) laner_mana: LaneMana,
    pub(crate) laner_gold: LaneGold,
    pub(crate) laner_experience: LaneExperience,
    pub(crate) laner_cooldown: LaneCooldown,
    pub(crate) laner_bounty: LaneBounty,
    pub(crate) laner_level: LaneLevel,
    pub(crate) laner_minion_kills: LaneMinionKills,
    pub(crate) laner_shield: LaneShield,
    pub(crate) laner_ward: LaneWard,
    pub(crate) laner_position: LanePosition,
    pub(crate) wave_pressure: WavePressure,
    pub(crate) opponent: OpponentReport,
    pub(crate) jungle_threat: ThreatReport,
    pub(crate) available_intents: [LaneIntent; 2],
    pub(crate) window: LaneWindow,
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

    pub fn laner_experience(self) -> LaneExperience {
        self.laner_experience
    }

    pub fn laner_cooldown(self) -> LaneCooldown {
        self.laner_cooldown
    }

    pub fn laner_bounty(self) -> LaneBounty {
        self.laner_bounty
    }

    pub fn laner_level(self) -> LaneLevel {
        self.laner_level
    }

    pub fn laner_minion_kills(self) -> LaneMinionKills {
        self.laner_minion_kills
    }

    pub fn laner_shield(self) -> LaneShield {
        self.laner_shield
    }

    pub fn laner_ward(self) -> LaneWard {
        self.laner_ward
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
    pub(crate) observation: AlliedLaneObservation,
    pub(crate) source_state_hash: StateHash,
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
            laner_experience: state.player().experience(),
            laner_cooldown: state.player().cooldown(),
            laner_bounty: state.player().bounty(),
            laner_level: state.player().level(),
            laner_minion_kills: state.player().minion_kills(),
            laner_shield: state.player().shield(),
            laner_ward: state.player().ward(),
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
