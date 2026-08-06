use super::*;

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

/// A compatibility view of the lifecycle state. The snapshot stores only
/// [`LaneStatus`], so an open state can never carry a terminal outcome.
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
pub enum LaneStatus {
    Open,
    Resolved(LaneOutcome),
}

impl LaneStatus {
    pub const fn phase(self) -> LanePhase {
        match self {
            Self::Open => LanePhase::Open,
            Self::Resolved(_) => LanePhase::Resolved,
        }
    }

    pub const fn outcome(self) -> Option<LaneOutcome> {
        match self {
            Self::Open => None,
            Self::Resolved(outcome) => Some(outcome),
        }
    }
}

/// The bounded resources that are part of the current M2 contract.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneResources {
    pub(crate) mana: LaneMana,
    pub(crate) gold: LaneGold,
    pub(crate) experience: LaneExperience,
    pub(crate) cooldown: LaneCooldown,
}

impl Default for LaneResources {
    fn default() -> Self {
        Self::initial()
    }
}

impl LaneResources {
    pub const fn initial() -> Self {
        Self {
            mana: LaneMana::full(),
            gold: LaneGold::zero(),
            experience: LaneExperience::zero(),
            cooldown: LaneCooldown::zero(),
        }
    }

    pub const fn new(
        mana: LaneMana,
        gold: LaneGold,
        experience: LaneExperience,
        cooldown: LaneCooldown,
    ) -> Self {
        Self {
            mana,
            gold,
            experience,
            cooldown,
        }
    }

    pub const fn mana(self) -> LaneMana {
        self.mana
    }

    pub const fn gold(self) -> LaneGold {
        self.gold
    }

    pub const fn experience(self) -> LaneExperience {
        self.experience
    }

    pub const fn cooldown(self) -> LaneCooldown {
        self.cooldown
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlayerLaneState {
    pub(crate) id: ActorId,
    pub(crate) health: LaneHealth,
    pub(crate) resources: LaneResources,
    pub(crate) position: LanePosition,
}

impl PlayerLaneState {
    pub const fn new(
        id: ActorId,
        health: LaneHealth,
        resources: LaneResources,
        position: LanePosition,
    ) -> Self {
        Self {
            id,
            health,
            resources,
            position,
        }
    }

    pub(crate) const fn from_resources(
        id: ActorId,
        health: LaneHealth,
        resources: LaneResources,
        position: LanePosition,
    ) -> Self {
        Self::new(id, health, resources, position)
    }

    pub const fn id(self) -> ActorId {
        self.id
    }

    pub const fn health(self) -> LaneHealth {
        self.health
    }

    pub const fn resources(self) -> LaneResources {
        self.resources
    }

    pub const fn mana(self) -> LaneMana {
        self.resources.mana()
    }

    pub const fn gold(self) -> LaneGold {
        self.resources.gold()
    }

    pub const fn experience(self) -> LaneExperience {
        self.resources.experience()
    }

    pub const fn cooldown(self) -> LaneCooldown {
        self.resources.cooldown()
    }

    pub const fn position(self) -> LanePosition {
        self.position
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpponentTruth {
    pub(crate) id: ActorId,
    pub(crate) health: LaneHealth,
    pub(crate) position: LanePosition,
    pub(crate) posture: OpponentPosture,
}

impl OpponentTruth {
    pub const fn new(
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

    pub const fn id(self) -> ActorId {
        self.id
    }

    pub const fn health(self) -> LaneHealth {
        self.health
    }

    pub const fn position(self) -> LanePosition {
        self.position
    }

    pub const fn posture(self) -> OpponentPosture {
        self.posture
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WaveState {
    pub(crate) pressure: WavePressure,
}

impl WaveState {
    pub const fn new(pressure: WavePressure) -> Self {
        Self { pressure }
    }

    pub const fn pressure(self) -> WavePressure {
        self.pressure
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDelay(u8);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDelayError {
    pub value: u8,
}

impl fmt::Display for LaneDelayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "lane delay must be non-zero (got {})",
            self.value
        )
    }
}

impl LaneDelay {
    pub fn new(value: u8) -> Result<Self, LaneDelayError> {
        if value == 0 {
            Err(LaneDelayError { value })
        } else {
            Ok(Self(value))
        }
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneDelayedEffectKind {
    SelfHealthRegen { amount: LaneHealth },
    SelfManaRegen { amount: LaneMana },
    SelfCooldownReduction { amount: LaneCooldown },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDelayedEffect {
    pub(crate) delay: LaneDelay,
    pub(crate) kind: LaneDelayedEffectKind,
}

impl LaneDelayedEffect {
    pub fn new(delay: LaneDelay, kind: LaneDelayedEffectKind) -> Self {
        Self { delay, kind }
    }

    pub const fn delay(self) -> LaneDelay {
        self.delay
    }

    pub const fn delay_beats(self) -> u8 {
        self.delay.value()
    }

    pub const fn kind(self) -> LaneDelayedEffectKind {
        self.kind
    }
}

pub(crate) const MAX_DELAYED_EFFECTS: usize = 4;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct LaneDelayedEffects {
    pub(crate) count: u8,
    pub(crate) items: [Option<LaneDelayedEffect>; MAX_DELAYED_EFFECTS],
}

impl LaneDelayedEffects {
    pub const fn empty() -> Self {
        Self {
            count: 0,
            items: [None; MAX_DELAYED_EFFECTS],
        }
    }

    pub const fn count(self) -> u8 {
        self.count
    }

    pub const fn is_empty(self) -> bool {
        self.count == 0
    }

    pub fn items(&self) -> &[Option<LaneDelayedEffect>] {
        &self.items[..self.count as usize]
    }

    pub fn push(&mut self, effect: LaneDelayedEffect) -> Result<(), LaneBoundsError> {
        if (self.count as usize) < MAX_DELAYED_EFFECTS {
            self.items[self.count as usize] = Some(effect);
            self.count += 1;
            Ok(())
        } else {
            Err(LaneBoundsError {
                value: self.count + 1,
                maximum: MAX_DELAYED_EFFECTS as u8,
            })
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneSnapshot {
    pub(crate) ruleset: RulesetId,
    pub(crate) turn: Turn,
    pub(crate) window: LaneWindow,
    pub(crate) status: LaneStatus,
    pub(crate) player: PlayerLaneState,
    pub(crate) opponent: OpponentTruth,
    pub(crate) wave: WaveState,
    pub(crate) jungle_threat: JungleThreatTruth,
    pub(crate) delayed_effects: LaneDelayedEffects,
}

impl LaneSnapshot {
    pub fn initial() -> Self {
        Self::new(
            M2_LANE_RULESET,
            Turn::new(0),
            LaneStatus::Open,
            PlayerLaneState::new(
                PLAYER_LANER,
                LaneHealth::new(8).expect("fixture health must be bounded"),
                LaneResources::initial(),
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
        )
    }

    pub fn new(
        ruleset: RulesetId,
        turn: Turn,
        status: LaneStatus,
        player: PlayerLaneState,
        opponent: OpponentTruth,
        wave: WaveState,
        jungle_threat: JungleThreatTruth,
    ) -> Self {
        Self::new_with_window(
            ruleset,
            turn,
            LaneWindow::OneBeat,
            status,
            player,
            opponent,
            wave,
            jungle_threat,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_window(
        ruleset: RulesetId,
        turn: Turn,
        window: LaneWindow,
        status: LaneStatus,
        player: PlayerLaneState,
        opponent: OpponentTruth,
        wave: WaveState,
        jungle_threat: JungleThreatTruth,
    ) -> Self {
        Self::new_with_delayed_effects(
            ruleset,
            turn,
            window,
            status,
            player,
            opponent,
            wave,
            jungle_threat,
            LaneDelayedEffects::empty(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_delayed_effects(
        ruleset: RulesetId,
        turn: Turn,
        window: LaneWindow,
        status: LaneStatus,
        player: PlayerLaneState,
        opponent: OpponentTruth,
        wave: WaveState,
        jungle_threat: JungleThreatTruth,
        delayed_effects: LaneDelayedEffects,
    ) -> Self {
        Self {
            ruleset,
            turn,
            window,
            status,
            player,
            opponent,
            wave,
            jungle_threat,
            delayed_effects,
        }
    }

    pub const fn ruleset(self) -> RulesetId {
        self.ruleset
    }

    pub const fn turn(self) -> Turn {
        self.turn
    }

    pub const fn window(self) -> LaneWindow {
        self.window
    }

    pub const fn status(self) -> LaneStatus {
        self.status
    }

    pub const fn phase(self) -> LanePhase {
        self.status.phase()
    }

    pub const fn player(self) -> PlayerLaneState {
        self.player
    }

    pub const fn opponent(self) -> OpponentTruth {
        self.opponent
    }

    pub const fn wave(self) -> WaveState {
        self.wave
    }

    pub const fn jungle_threat(self) -> JungleThreatTruth {
        self.jungle_threat
    }

    pub const fn delayed_effects(self) -> LaneDelayedEffects {
        self.delayed_effects
    }

    pub const fn terminal_outcome(self) -> Option<LaneOutcome> {
        self.status.outcome()
    }

    pub fn hash(self) -> StateHash {
        let mut hash = FNV_OFFSET_BASIS;
        hash = hash_bytes(hash, &self.ruleset.value().to_le_bytes());
        hash = hash_bytes(hash, &self.turn.value().to_le_bytes());
        if self.window != LaneWindow::OneBeat {
            hash = hash_bytes(hash, &[window_tag(self.window)]);
        }
        hash = hash_bytes(hash, &[phase_tag(self.phase())]);
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
        if self.player.experience() != LaneExperience::zero() {
            hash = hash_bytes(
                hash,
                &[LANE_EXPERIENCE_HASH_TAG, self.player.experience().value()],
            );
        }
        if self.player.cooldown() != LaneCooldown::zero() {
            hash = hash_bytes(
                hash,
                &[LANE_COOLDOWN_HASH_TAG, self.player.cooldown().value()],
            );
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
        if !self.delayed_effects.is_empty() {
            hash = hash_bytes(
                hash,
                &[LANE_DELAYED_EFFECT_HASH_TAG, self.delayed_effects.count()],
            );
            for item in self.delayed_effects.items().iter().flatten() {
                hash = hash_bytes(hash, &[item.delay_beats()]);
                match item.kind() {
                    LaneDelayedEffectKind::SelfHealthRegen { amount } => {
                        hash = hash_bytes(hash, &[0x01, amount.value()]);
                    }
                    LaneDelayedEffectKind::SelfManaRegen { amount } => {
                        hash = hash_bytes(hash, &[0x02, amount.value()]);
                    }
                    LaneDelayedEffectKind::SelfCooldownReduction { amount } => {
                        hash = hash_bytes(hash, &[0x03, amount.value()]);
                    }
                }
            }
        }
        hash = hash_bytes(hash, &[outcome_tag(self.terminal_outcome())]);
        StateHash::from_raw(hash)
    }

    pub(crate) fn is_valid_lane_state(self) -> bool {
        self.ruleset == M2_LANE_RULESET
            && self.player.id == PLAYER_LANER
            && self.opponent.id == OPPONENT_LANER
            && matches!(self.status, LaneStatus::Open | LaneStatus::Resolved(_))
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
