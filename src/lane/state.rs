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
pub(crate) struct PlayerResources {
    pub(crate) mana: LaneMana,
    pub(crate) gold: LaneGold,
    pub(crate) experience: LaneExperience,
    pub(crate) cooldown: LaneCooldown,
    pub(crate) bounty: LaneBounty,
    pub(crate) level: LaneLevel,
    pub(crate) minion_kills: LaneMinionKills,
    pub(crate) shield: LaneShield,
    pub(crate) ward: LaneWard,
    pub(crate) potion: LanePotion,
    pub(crate) elixir: LaneElixir,
    pub(crate) trinket: LaneTrinket,
    pub(crate) relic: LaneRelic,
    pub(crate) charm: LaneCharm,
    pub(crate) scroll: LaneScroll,
    pub(crate) tome: LaneTome,
    pub(crate) rune: LaneRune,
    pub(crate) sigil: LaneSigil,
    pub(crate) talisman: LaneTalisman,
    pub(crate) amulet: LaneAmulet,
}

impl PlayerResources {
    fn baseline() -> Self {
        Self {
            mana: LaneMana::full(),
            gold: LaneGold::zero(),
            experience: LaneExperience::zero(),
            cooldown: LaneCooldown::zero(),
            bounty: LaneBounty::zero(),
            level: LaneLevel::initial(),
            minion_kills: LaneMinionKills::zero(),
            shield: LaneShield::zero(),
            ward: LaneWard::zero(),
            potion: LanePotion::zero(),
            elixir: LaneElixir::zero(),
            trinket: LaneTrinket::zero(),
            relic: LaneRelic::zero(),
            charm: LaneCharm::zero(),
            scroll: LaneScroll::zero(),
            tome: LaneTome::zero(),
            rune: LaneRune::zero(),
            sigil: LaneSigil::zero(),
            talisman: LaneTalisman::zero(),
            amulet: LaneAmulet::zero(),
        }
    }

    fn with_mana(self, mana: LaneMana) -> Self {
        Self { mana, ..self }
    }

    fn with_gold(self, gold: LaneGold) -> Self {
        Self { gold, ..self }
    }

    fn with_experience(self, experience: LaneExperience) -> Self {
        Self { experience, ..self }
    }

    fn with_cooldown(self, cooldown: LaneCooldown) -> Self {
        Self { cooldown, ..self }
    }

    fn with_bounty(self, bounty: LaneBounty) -> Self {
        Self { bounty, ..self }
    }

    fn with_level(self, level: LaneLevel) -> Self {
        Self { level, ..self }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlayerLaneState {
    pub(crate) id: ActorId,
    pub(crate) health: LaneHealth,
    pub(crate) mana: LaneMana,
    pub(crate) gold: LaneGold,
    pub(crate) experience: LaneExperience,
    pub(crate) cooldown: LaneCooldown,
    pub(crate) bounty: LaneBounty,
    pub(crate) level: LaneLevel,
    pub(crate) minion_kills: LaneMinionKills,
    pub(crate) shield: LaneShield,
    pub(crate) ward: LaneWard,
    pub(crate) potion: LanePotion,
    pub(crate) elixir: LaneElixir,
    pub(crate) trinket: LaneTrinket,
    pub(crate) relic: LaneRelic,
    pub(crate) charm: LaneCharm,
    pub(crate) scroll: LaneScroll,
    pub(crate) tome: LaneTome,
    pub(crate) rune: LaneRune,
    pub(crate) sigil: LaneSigil,
    pub(crate) talisman: LaneTalisman,
    pub(crate) amulet: LaneAmulet,
    pub(crate) position: LanePosition,
}

impl PlayerLaneState {
    pub(crate) fn from_resources(
        id: ActorId,
        health: LaneHealth,
        resources: PlayerResources,
        position: LanePosition,
    ) -> Self {
        Self {
            id,
            health,
            mana: resources.mana,
            gold: resources.gold,
            experience: resources.experience,
            cooldown: resources.cooldown,
            bounty: resources.bounty,
            level: resources.level,
            minion_kills: resources.minion_kills,
            shield: resources.shield,
            ward: resources.ward,
            potion: resources.potion,
            elixir: resources.elixir,
            trinket: resources.trinket,
            relic: resources.relic,
            charm: resources.charm,
            scroll: resources.scroll,
            tome: resources.tome,
            rune: resources.rune,
            sigil: resources.sigil,
            talisman: resources.talisman,
            amulet: resources.amulet,
            position,
        }
    }

    pub(crate) fn resources(self) -> PlayerResources {
        PlayerResources {
            mana: self.mana,
            gold: self.gold,
            experience: self.experience,
            cooldown: self.cooldown,
            bounty: self.bounty,
            level: self.level,
            minion_kills: self.minion_kills,
            shield: self.shield,
            ward: self.ward,
            potion: self.potion,
            elixir: self.elixir,
            trinket: self.trinket,
            relic: self.relic,
            charm: self.charm,
            scroll: self.scroll,
            tome: self.tome,
            rune: self.rune,
            sigil: self.sigil,
            talisman: self.talisman,
            amulet: self.amulet,
        }
    }

    pub fn new(id: ActorId, health: LaneHealth, position: LanePosition) -> Self {
        Self::from_resources(id, health, PlayerResources::baseline(), position)
    }

    pub fn new_with_mana(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources::baseline().with_mana(mana),
            position,
        )
    }

    pub fn new_with_resources(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources::baseline().with_mana(mana).with_gold(gold),
            position,
        )
    }

    pub fn new_with_all_resources(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        experience: LaneExperience,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources::baseline()
                .with_mana(mana)
                .with_gold(gold)
                .with_experience(experience),
            position,
        )
    }

    pub fn new_with_complete_state(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        experience: LaneExperience,
        cooldown: LaneCooldown,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources::baseline()
                .with_mana(mana)
                .with_gold(gold)
                .with_experience(experience)
                .with_cooldown(cooldown),
            position,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_full_state(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        experience: LaneExperience,
        cooldown: LaneCooldown,
        bounty: LaneBounty,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources::baseline()
                .with_mana(mana)
                .with_gold(gold)
                .with_experience(experience)
                .with_cooldown(cooldown)
                .with_bounty(bounty),
            position,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_entire_state(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        experience: LaneExperience,
        cooldown: LaneCooldown,
        bounty: LaneBounty,
        level: LaneLevel,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources::baseline()
                .with_mana(mana)
                .with_gold(gold)
                .with_experience(experience)
                .with_cooldown(cooldown)
                .with_bounty(bounty)
                .with_level(level),
            position,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_absolute_state(
        id: ActorId,
        health: LaneHealth,
        mana: LaneMana,
        gold: LaneGold,
        experience: LaneExperience,
        cooldown: LaneCooldown,
        bounty: LaneBounty,
        level: LaneLevel,
        minion_kills: LaneMinionKills,
        position: LanePosition,
    ) -> Self {
        Self::from_resources(
            id,
            health,
            PlayerResources {
                mana,
                gold,
                experience,
                cooldown,
                bounty,
                level,
                minion_kills,
                shield: LaneShield::zero(),
                ward: LaneWard::zero(),
                potion: LanePotion::zero(),
                elixir: LaneElixir::zero(),
                trinket: LaneTrinket::zero(),
                relic: LaneRelic::zero(),
                charm: LaneCharm::zero(),
                scroll: LaneScroll::zero(),
                tome: LaneTome::zero(),
                rune: LaneRune::zero(),
                sigil: LaneSigil::zero(),
                talisman: LaneTalisman::zero(),
                amulet: LaneAmulet::zero(),
            },
            position,
        )
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

    pub fn experience(self) -> LaneExperience {
        self.experience
    }

    pub fn cooldown(self) -> LaneCooldown {
        self.cooldown
    }

    pub fn bounty(self) -> LaneBounty {
        self.bounty
    }

    pub fn level(self) -> LaneLevel {
        self.level
    }

    pub fn minion_kills(self) -> LaneMinionKills {
        self.minion_kills
    }

    pub fn shield(self) -> LaneShield {
        self.shield
    }

    pub fn ward(self) -> LaneWard {
        self.ward
    }

    pub fn potion(self) -> LanePotion {
        self.potion
    }

    pub fn elixir(self) -> LaneElixir {
        self.elixir
    }

    pub fn trinket(self) -> LaneTrinket {
        self.trinket
    }

    pub fn relic(self) -> LaneRelic {
        self.relic
    }

    pub fn charm(self) -> LaneCharm {
        self.charm
    }

    pub fn scroll(self) -> LaneScroll {
        self.scroll
    }

    pub fn tome(self) -> LaneTome {
        self.tome
    }

    pub fn rune(self) -> LaneRune {
        self.rune
    }

    pub fn sigil(self) -> LaneSigil {
        self.sigil
    }

    pub fn talisman(self) -> LaneTalisman {
        self.talisman
    }

    pub fn amulet(self) -> LaneAmulet {
        self.amulet
    }

    pub fn position(self) -> LanePosition {
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
    pub(crate) pressure: WavePressure,
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
pub enum LaneDelayedEffectKind {
    SelfHealthRegen { amount: LaneHealth },
    SelfManaRegen { amount: LaneMana },
    SelfCooldownReduction { amount: LaneCooldown },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneDelayedEffect {
    pub(crate) delay_beats: u8,
    pub(crate) kind: LaneDelayedEffectKind,
}

impl LaneDelayedEffect {
    pub fn new(delay_beats: u8, kind: LaneDelayedEffectKind) -> Self {
        Self { delay_beats, kind }
    }

    pub fn delay_beats(self) -> u8 {
        self.delay_beats
    }

    pub fn kind(self) -> LaneDelayedEffectKind {
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

    pub fn count(self) -> u8 {
        self.count
    }

    pub fn is_empty(self) -> bool {
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
    pub(crate) phase: LanePhase,
    pub(crate) player: PlayerLaneState,
    pub(crate) opponent: OpponentTruth,
    pub(crate) wave: WaveState,
    pub(crate) jungle_threat: JungleThreatTruth,
    pub(crate) delayed_effects: LaneDelayedEffects,
    pub(crate) terminal_outcome: Option<LaneOutcome>,
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
        Self::new_with_delayed_effects(
            ruleset,
            turn,
            window,
            phase,
            player,
            opponent,
            wave,
            jungle_threat,
            LaneDelayedEffects::empty(),
            terminal_outcome,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_delayed_effects(
        ruleset: RulesetId,
        turn: Turn,
        window: LaneWindow,
        phase: LanePhase,
        player: PlayerLaneState,
        opponent: OpponentTruth,
        wave: WaveState,
        jungle_threat: JungleThreatTruth,
        delayed_effects: LaneDelayedEffects,
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
            delayed_effects,
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

    pub fn delayed_effects(self) -> LaneDelayedEffects {
        self.delayed_effects
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
        if self.player.bounty() != LaneBounty::zero() {
            hash = hash_bytes(hash, &[LANE_BOUNTY_HASH_TAG, self.player.bounty().value()]);
        }
        if self.player.level() != LaneLevel::initial() {
            hash = hash_bytes(hash, &[LANE_LEVEL_HASH_TAG, self.player.level().value()]);
        }
        if self.player.minion_kills() != LaneMinionKills::zero() {
            hash = hash_bytes(
                hash,
                &[
                    LANE_MINION_KILLS_HASH_TAG,
                    self.player.minion_kills().value(),
                ],
            );
        }
        if self.player.shield() != LaneShield::zero() {
            hash = hash_bytes(hash, &[LANE_SHIELD_HASH_TAG, self.player.shield().value()]);
        }
        if self.player.ward() != LaneWard::zero() {
            hash = hash_bytes(hash, &[LANE_WARD_HASH_TAG, self.player.ward().value()]);
        }
        if self.player.potion() != LanePotion::zero() {
            hash = hash_bytes(hash, &[LANE_POTION_HASH_TAG, self.player.potion().value()]);
        }
        if self.player.elixir() != LaneElixir::zero() {
            hash = hash_bytes(hash, &[LANE_ELIXIR_HASH_TAG, self.player.elixir().value()]);
        }
        if self.player.trinket() != LaneTrinket::zero() {
            hash = hash_bytes(
                hash,
                &[LANE_TRINKET_HASH_TAG, self.player.trinket().value()],
            );
        }
        if self.player.relic() != LaneRelic::zero() {
            hash = hash_bytes(hash, &[LANE_RELIC_HASH_TAG, self.player.relic().value()]);
        }
        if self.player.charm() != LaneCharm::zero() {
            hash = hash_bytes(hash, &[LANE_CHARM_HASH_TAG, self.player.charm().value()]);
        }
        if self.player.scroll() != LaneScroll::zero() {
            hash = hash_bytes(hash, &[LANE_SCROLL_HASH_TAG, self.player.scroll().value()]);
        }
        if self.player.tome() != LaneTome::zero() {
            hash = hash_bytes(hash, &[LANE_TOME_HASH_TAG, self.player.tome().value()]);
        }
        if self.player.rune() != LaneRune::zero() {
            hash = hash_bytes(hash, &[LANE_RUNE_HASH_TAG, self.player.rune().value()]);
        }
        if self.player.sigil() != LaneSigil::zero() {
            hash = hash_bytes(hash, &[LANE_SIGIL_HASH_TAG, self.player.sigil().value()]);
        }
        if self.player.talisman() != LaneTalisman::zero() {
            hash = hash_bytes(
                hash,
                &[LANE_TALISMAN_HASH_TAG, self.player.talisman().value()],
            );
        }
        if self.player.amulet() != LaneAmulet::zero() {
            hash = hash_bytes(hash, &[LANE_AMULET_HASH_TAG, self.player.amulet().value()]);
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
        hash = hash_bytes(hash, &[outcome_tag(self.terminal_outcome)]);
        StateHash::from_raw(hash)
    }

    pub(crate) fn is_valid_lane_state(self) -> bool {
        self.ruleset == M2_LANE_RULESET
            && self.player.id == PLAYER_LANER
            && self.opponent.id == OPPONENT_LANER
            && ((self.phase == LanePhase::Open && self.terminal_outcome.is_none())
                || (self.phase == LanePhase::Resolved && self.terminal_outcome.is_some()))
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
