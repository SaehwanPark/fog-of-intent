use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObservationId(pub(crate) u64);

impl ObservationId {
  pub fn new(value: u64) -> Self {
    Self(value)
  }

  pub const fn value(self) -> u64 {
    self.0
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HiddenValue {
  Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneBelief<T> {
  Unknown,
  Observed { value: T, observed_turn: Turn },
  LastKnown { value: T, last_seen_turn: Turn },
}

impl<T: Copy> LaneBelief<T> {
  pub const fn unknown() -> Self {
    Self::Unknown
  }

  fn update(self, value: Option<T>, last_seen_turn: Option<Turn>, observation_turn: Turn) -> Self {
    match (value, last_seen_turn) {
      (Some(_), Some(last_seen_turn)) if last_seen_turn.value() > observation_turn.value() => {
        Self::Unknown
      }
      (Some(value), Some(last_seen_turn)) if last_seen_turn == observation_turn => Self::Observed {
        value,
        observed_turn: last_seen_turn,
      },
      (Some(value), Some(last_seen_turn)) => Self::LastKnown {
        value,
        last_seen_turn,
      },
      (None, None) => self,
      _ => Self::Unknown,
    }
  }
}

impl LaneBelief<LanePosition> {
  pub fn from_opponent_report(self, report: OpponentReport, observation_turn: Turn) -> Self {
    self.update(
      report.last_known_position(),
      report.last_seen_turn(),
      observation_turn,
    )
  }
}

impl LaneBelief<JungleThreatRegion> {
  pub fn from_threat_report(self, report: ThreatReport, observation_turn: Turn) -> Self {
    self.update(
      report.last_known_region(),
      report.last_seen_turn(),
      observation_turn,
    )
  }
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

  pub const fn last_known_position(self) -> Option<LanePosition> {
    self.last_known_position
  }

  pub const fn last_seen_turn(self) -> Option<Turn> {
    self.last_seen_turn
  }

  pub const fn health(self) -> HiddenValue {
    self.health
  }

  pub const fn posture(self) -> HiddenValue {
    self.posture
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LanerObservation {
  pub(crate) schema: &'static str,
  pub(crate) observer: ActorId,
  pub(crate) actors: LaneActorRoster,
  pub(crate) turn: Turn,
  pub(crate) observation_id: ObservationId,
  pub(crate) self_health: LaneHealth,
  pub(crate) self_resources: LaneResources,
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
  pub(crate) available_fallback_behaviors: [LaneFallbackBehavior; 4],
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
  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn actors(self) -> LaneActorRoster {
    self.actors
  }

  pub const fn turn(self) -> Turn {
    self.turn
  }

  pub const fn observation_id(self) -> ObservationId {
    self.observation_id
  }

  pub const fn self_health(self) -> LaneHealth {
    self.self_health
  }

  pub const fn self_resources(self) -> LaneResources {
    self.self_resources
  }

  pub const fn self_mana(self) -> LaneMana {
    self.self_resources.mana()
  }

  pub const fn self_gold(self) -> LaneGold {
    self.self_resources.gold()
  }

  pub const fn self_experience(self) -> LaneExperience {
    self.self_resources.experience()
  }

  pub const fn self_cooldown(self) -> LaneCooldown {
    self.self_resources.cooldown()
  }

  pub const fn self_position(self) -> LanePosition {
    self.self_position
  }

  pub const fn wave_pressure(self) -> WavePressure {
    self.wave_pressure
  }

  pub const fn opponent(self) -> OpponentReport {
    self.opponent
  }

  pub const fn jungle_threat(self) -> ThreatReport {
    self.jungle_threat
  }

  pub const fn available_intents(self) -> [LaneIntent; 4] {
    self.available_intents
  }

  pub const fn available_threat_response(self) -> Option<LaneIntent> {
    self.available_threat_response
  }

  pub const fn available_target_focuses(self) -> [LaneTargetFocus; 3] {
    self.available_target_focuses
  }

  pub const fn available_commitments(self) -> [LaneCommitment; 3] {
    self.available_commitments
  }

  pub const fn available_ping_signals(self) -> [LanePingSignal; 5] {
    self.available_ping_signals
  }

  pub const fn available_abort_conditions(self) -> [LaneAbortCondition; 4] {
    self.available_abort_conditions
  }

  pub const fn available_fallback_behaviors(self) -> [LaneFallbackBehavior; 4] {
    self.available_fallback_behaviors
  }

  pub const fn window(self) -> LaneWindow {
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
  pub const fn observation(self) -> LanerObservation {
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
      actors: LaneActorRoster::initial(),
      turn: state.turn(),
      observation_id,
      self_health: state.player().health(),
      self_resources: state.player().resources(),
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
      available_fallback_behaviors: [
        LaneFallbackBehavior::MaintainPlan,
        LaneFallbackBehavior::RetreatToTower,
        LaneFallbackBehavior::SafeFarm,
        LaneFallbackBehavior::ConserveResources,
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
  pub(crate) actors: LaneActorRoster,
  pub(crate) turn: Turn,
  pub(crate) observation_id: ObservationId,
  pub(crate) laner_health: LaneHealth,
  pub(crate) laner_resources: LaneResources,
  pub(crate) laner_position: LanePosition,
  pub(crate) wave_pressure: WavePressure,
  pub(crate) opponent: OpponentReport,
  pub(crate) jungle_threat: ThreatReport,
  pub(crate) available_intents: [LaneIntent; 2],
  pub(crate) window: LaneWindow,
}

impl AlliedLaneObservation {
  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn actors(self) -> LaneActorRoster {
    self.actors
  }

  pub const fn turn(self) -> Turn {
    self.turn
  }

  pub const fn observation_id(self) -> ObservationId {
    self.observation_id
  }

  pub const fn laner_health(self) -> LaneHealth {
    self.laner_health
  }

  pub const fn laner_resources(self) -> LaneResources {
    self.laner_resources
  }

  pub const fn laner_mana(self) -> LaneMana {
    self.laner_resources.mana()
  }

  pub const fn laner_gold(self) -> LaneGold {
    self.laner_resources.gold()
  }

  pub const fn laner_experience(self) -> LaneExperience {
    self.laner_resources.experience()
  }

  pub const fn laner_cooldown(self) -> LaneCooldown {
    self.laner_resources.cooldown()
  }

  pub const fn laner_position(self) -> LanePosition {
    self.laner_position
  }

  pub const fn wave_pressure(self) -> WavePressure {
    self.wave_pressure
  }

  pub const fn opponent(self) -> OpponentReport {
    self.opponent
  }

  pub const fn jungle_threat(self) -> ThreatReport {
    self.jungle_threat
  }

  pub const fn available_intents(self) -> [LaneIntent; 2] {
    self.available_intents
  }

  pub const fn window(self) -> LaneWindow {
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
  pub const fn observation(self) -> AlliedLaneObservation {
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
      actors: LaneActorRoster::initial(),
      turn: state.turn(),
      observation_id,
      laner_health: state.player().health(),
      laner_resources: state.player().resources(),
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
