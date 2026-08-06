use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LaneWaveResult {
    Advanced,
    Held,
    Lost,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneExecutionInputs {
    pub(crate) trace: InputTrace,
    pub(crate) self_damage: LaneDamage,
    pub(crate) opponent_damage: LaneDamage,
    pub(crate) wave_result: LaneWaveResult,
    pub(crate) mana_spent: LaneMana,
    pub(crate) gold_earned: LaneGold,
    pub(crate) experience_gained: LaneExperience,
    pub(crate) cooldown_set: LaneCooldown,
    pub(crate) bounty_earned: LaneBounty,
    pub(crate) level_gained: LaneLevel,
    pub(crate) minion_kills_gained: LaneMinionKills,
    pub(crate) shield_gained: LaneShield,
    pub(crate) ward_gained: LaneWard,
    pub(crate) potion_gained: LanePotion,
    pub(crate) potion_spent: LanePotion,
    pub(crate) elixir_gained: LaneElixir,
    pub(crate) elixir_spent: LaneElixir,
    pub(crate) trinket_gained: LaneTrinket,
    pub(crate) trinket_spent: LaneTrinket,
    pub(crate) relic_gained: LaneRelic,
    pub(crate) relic_spent: LaneRelic,
    pub(crate) charm_gained: LaneCharm,
    pub(crate) charm_spent: LaneCharm,
    pub(crate) scroll_gained: LaneScroll,
    pub(crate) scroll_spent: LaneScroll,
    pub(crate) tome_gained: LaneTome,
    pub(crate) tome_spent: LaneTome,
    pub(crate) rune_gained: LaneRune,
    pub(crate) rune_spent: LaneRune,
    pub(crate) sigil_gained: LaneSigil,
    pub(crate) sigil_spent: LaneSigil,
    pub(crate) talisman_gained: LaneTalisman,
    pub(crate) talisman_spent: LaneTalisman,
    pub(crate) amulet_gained: LaneAmulet,
    pub(crate) amulet_spent: LaneAmulet,
    pub(crate) phial_gained: LanePhial,
    pub(crate) phial_spent: LanePhial,
    pub(crate) delayed_effect: Option<LaneDelayedEffect>,
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
            experience_gained: LaneExperience::zero(),
            cooldown_set: LaneCooldown::zero(),
            bounty_earned: LaneBounty::zero(),
            level_gained: LaneLevel::zero(),
            minion_kills_gained: LaneMinionKills::zero(),
            shield_gained: LaneShield::zero(),
            ward_gained: LaneWard::zero(),
            potion_gained: LanePotion::zero(),
            potion_spent: LanePotion::zero(),
            elixir_gained: LaneElixir::zero(),
            elixir_spent: LaneElixir::zero(),
            trinket_gained: LaneTrinket::zero(),
            trinket_spent: LaneTrinket::zero(),
            relic_gained: LaneRelic::zero(),
            relic_spent: LaneRelic::zero(),
            charm_gained: LaneCharm::zero(),
            charm_spent: LaneCharm::zero(),
            scroll_gained: LaneScroll::zero(),
            scroll_spent: LaneScroll::zero(),
            tome_gained: LaneTome::zero(),
            tome_spent: LaneTome::zero(),
            rune_gained: LaneRune::zero(),
            rune_spent: LaneRune::zero(),
            sigil_gained: LaneSigil::zero(),
            sigil_spent: LaneSigil::zero(),
            talisman_gained: LaneTalisman::zero(),
            talisman_spent: LaneTalisman::zero(),
            amulet_gained: LaneAmulet::zero(),
            amulet_spent: LaneAmulet::zero(),
            phial_gained: LanePhial::zero(),
            phial_spent: LanePhial::zero(),
            delayed_effect: None,
        }
    }

    pub fn with_delayed_effect(mut self, delayed_effect: LaneDelayedEffect) -> Self {
        self.delayed_effect = Some(delayed_effect);
        self
    }

    pub fn delayed_effect(self) -> Option<LaneDelayedEffect> {
        self.delayed_effect
    }

    pub fn with_mana_spent(mut self, mana_spent: LaneMana) -> Self {
        self.mana_spent = mana_spent;
        self
    }

    pub fn with_gold_earned(mut self, gold_earned: LaneGold) -> Self {
        self.gold_earned = gold_earned;
        self
    }

    pub fn with_experience_gained(mut self, experience_gained: LaneExperience) -> Self {
        self.experience_gained = experience_gained;
        self
    }

    pub fn with_cooldown_set(mut self, cooldown_set: LaneCooldown) -> Self {
        self.cooldown_set = cooldown_set;
        self
    }

    pub fn with_bounty_earned(mut self, bounty_earned: LaneBounty) -> Self {
        self.bounty_earned = bounty_earned;
        self
    }

    pub fn with_level_gained(mut self, level_gained: LaneLevel) -> Self {
        self.level_gained = level_gained;
        self
    }

    pub fn with_minion_kills_gained(mut self, minion_kills_gained: LaneMinionKills) -> Self {
        self.minion_kills_gained = minion_kills_gained;
        self
    }

    pub fn with_shield_gained(mut self, shield_gained: LaneShield) -> Self {
        self.shield_gained = shield_gained;
        self
    }

    pub fn with_ward_gained(mut self, ward_gained: LaneWard) -> Self {
        self.ward_gained = ward_gained;
        self
    }

    pub fn with_potion_gained(mut self, potion_gained: LanePotion) -> Self {
        self.potion_gained = potion_gained;
        self
    }

    pub fn with_potion_spent(mut self, potion_spent: LanePotion) -> Self {
        self.potion_spent = potion_spent;
        self
    }

    pub fn with_elixir_gained(mut self, elixir_gained: LaneElixir) -> Self {
        self.elixir_gained = elixir_gained;
        self
    }

    pub fn with_elixir_spent(mut self, elixir_spent: LaneElixir) -> Self {
        self.elixir_spent = elixir_spent;
        self
    }

    pub fn with_trinket_gained(mut self, trinket_gained: LaneTrinket) -> Self {
        self.trinket_gained = trinket_gained;
        self
    }

    pub fn with_trinket_spent(mut self, trinket_spent: LaneTrinket) -> Self {
        self.trinket_spent = trinket_spent;
        self
    }

    pub fn with_relic_gained(mut self, relic_gained: LaneRelic) -> Self {
        self.relic_gained = relic_gained;
        self
    }

    pub fn with_relic_spent(mut self, relic_spent: LaneRelic) -> Self {
        self.relic_spent = relic_spent;
        self
    }

    pub fn with_charm_gained(mut self, charm_gained: LaneCharm) -> Self {
        self.charm_gained = charm_gained;
        self
    }

    pub fn with_charm_spent(mut self, charm_spent: LaneCharm) -> Self {
        self.charm_spent = charm_spent;
        self
    }

    pub fn with_scroll_gained(mut self, scroll_gained: LaneScroll) -> Self {
        self.scroll_gained = scroll_gained;
        self
    }

    pub fn with_scroll_spent(mut self, scroll_spent: LaneScroll) -> Self {
        self.scroll_spent = scroll_spent;
        self
    }

    pub fn with_tome_gained(mut self, tome_gained: LaneTome) -> Self {
        self.tome_gained = tome_gained;
        self
    }

    pub fn with_tome_spent(mut self, tome_spent: LaneTome) -> Self {
        self.tome_spent = tome_spent;
        self
    }

    pub fn with_rune_gained(mut self, rune_gained: LaneRune) -> Self {
        self.rune_gained = rune_gained;
        self
    }

    pub fn with_rune_spent(mut self, rune_spent: LaneRune) -> Self {
        self.rune_spent = rune_spent;
        self
    }

    pub fn with_sigil_gained(mut self, sigil_gained: LaneSigil) -> Self {
        self.sigil_gained = sigil_gained;
        self
    }

    pub fn with_sigil_spent(mut self, sigil_spent: LaneSigil) -> Self {
        self.sigil_spent = sigil_spent;
        self
    }

    pub fn with_talisman_gained(mut self, talisman_gained: LaneTalisman) -> Self {
        self.talisman_gained = talisman_gained;
        self
    }

    pub fn with_talisman_spent(mut self, talisman_spent: LaneTalisman) -> Self {
        self.talisman_spent = talisman_spent;
        self
    }

    pub fn with_amulet_gained(mut self, amulet_gained: LaneAmulet) -> Self {
        self.amulet_gained = amulet_gained;
        self
    }

    pub fn with_amulet_spent(mut self, amulet_spent: LaneAmulet) -> Self {
        self.amulet_spent = amulet_spent;
        self
    }

    pub fn with_phial_gained(mut self, phial_gained: LanePhial) -> Self {
        self.phial_gained = phial_gained;
        self
    }

    pub fn with_phial_spent(mut self, phial_spent: LanePhial) -> Self {
        self.phial_spent = phial_spent;
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

    pub fn experience_gained(self) -> LaneExperience {
        self.experience_gained
    }

    pub fn cooldown_set(self) -> LaneCooldown {
        self.cooldown_set
    }

    pub fn bounty_earned(self) -> LaneBounty {
        self.bounty_earned
    }

    pub fn level_gained(self) -> LaneLevel {
        self.level_gained
    }

    pub fn minion_kills_gained(self) -> LaneMinionKills {
        self.minion_kills_gained
    }

    pub fn shield_gained(self) -> LaneShield {
        self.shield_gained
    }

    pub fn ward_gained(self) -> LaneWard {
        self.ward_gained
    }

    pub fn potion_gained(self) -> LanePotion {
        self.potion_gained
    }

    pub fn potion_spent(self) -> LanePotion {
        self.potion_spent
    }

    pub fn elixir_gained(self) -> LaneElixir {
        self.elixir_gained
    }

    pub fn elixir_spent(self) -> LaneElixir {
        self.elixir_spent
    }

    pub fn trinket_gained(self) -> LaneTrinket {
        self.trinket_gained
    }

    pub fn trinket_spent(self) -> LaneTrinket {
        self.trinket_spent
    }

    pub fn relic_gained(self) -> LaneRelic {
        self.relic_gained
    }

    pub fn relic_spent(self) -> LaneRelic {
        self.relic_spent
    }

    pub fn charm_gained(self) -> LaneCharm {
        self.charm_gained
    }

    pub fn charm_spent(self) -> LaneCharm {
        self.charm_spent
    }

    pub fn scroll_gained(self) -> LaneScroll {
        self.scroll_gained
    }

    pub fn scroll_spent(self) -> LaneScroll {
        self.scroll_spent
    }

    pub fn tome_gained(self) -> LaneTome {
        self.tome_gained
    }

    pub fn tome_spent(self) -> LaneTome {
        self.tome_spent
    }

    pub fn rune_gained(self) -> LaneRune {
        self.rune_gained
    }

    pub fn rune_spent(self) -> LaneRune {
        self.rune_spent
    }

    pub fn sigil_gained(self) -> LaneSigil {
        self.sigil_gained
    }

    pub fn sigil_spent(self) -> LaneSigil {
        self.sigil_spent
    }

    pub fn talisman_gained(self) -> LaneTalisman {
        self.talisman_gained
    }

    pub fn talisman_spent(self) -> LaneTalisman {
        self.talisman_spent
    }

    pub fn amulet_gained(self) -> LaneAmulet {
        self.amulet_gained
    }

    pub fn amulet_spent(self) -> LaneAmulet {
        self.amulet_spent
    }

    pub fn phial_gained(self) -> LanePhial {
        self.phial_gained
    }

    pub fn phial_spent(self) -> LanePhial {
        self.phial_spent
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ResourceExecutionDeltas {
    mana_spent: LaneMana,
    gold_earned: LaneGold,
    experience_gained: LaneExperience,
    cooldown_set: LaneCooldown,
    bounty_earned: LaneBounty,
    level_gained: LaneLevel,
    minion_kills_gained: LaneMinionKills,
    shield_gained: LaneShield,
    ward_gained: LaneWard,
    potion_gained: LanePotion,
    potion_spent: LanePotion,
    elixir_gained: LaneElixir,
    elixir_spent: LaneElixir,
    trinket_gained: LaneTrinket,
    trinket_spent: LaneTrinket,
    relic_gained: LaneRelic,
    relic_spent: LaneRelic,
    charm_gained: LaneCharm,
    charm_spent: LaneCharm,
    scroll_gained: LaneScroll,
    scroll_spent: LaneScroll,
    tome_gained: LaneTome,
    tome_spent: LaneTome,
    rune_gained: LaneRune,
    rune_spent: LaneRune,
    sigil_gained: LaneSigil,
    sigil_spent: LaneSigil,
    talisman_gained: LaneTalisman,
    talisman_spent: LaneTalisman,
    amulet_gained: LaneAmulet,
    amulet_spent: LaneAmulet,
    phial_gained: LanePhial,
    phial_spent: LanePhial,
}

impl ResourceExecutionDeltas {
    fn from_execution(execution: LaneExecutionInputs) -> Self {
        Self {
            mana_spent: execution.mana_spent,
            gold_earned: execution.gold_earned,
            experience_gained: execution.experience_gained,
            cooldown_set: execution.cooldown_set,
            bounty_earned: execution.bounty_earned,
            level_gained: execution.level_gained,
            minion_kills_gained: execution.minion_kills_gained,
            shield_gained: execution.shield_gained,
            ward_gained: execution.ward_gained,
            potion_gained: execution.potion_gained,
            potion_spent: execution.potion_spent,
            elixir_gained: execution.elixir_gained,
            elixir_spent: execution.elixir_spent,
            trinket_gained: execution.trinket_gained,
            trinket_spent: execution.trinket_spent,
            relic_gained: execution.relic_gained,
            relic_spent: execution.relic_spent,
            charm_gained: execution.charm_gained,
            charm_spent: execution.charm_spent,
            scroll_gained: execution.scroll_gained,
            scroll_spent: execution.scroll_spent,
            tome_gained: execution.tome_gained,
            tome_spent: execution.tome_spent,
            rune_gained: execution.rune_gained,
            rune_spent: execution.rune_spent,
            sigil_gained: execution.sigil_gained,
            sigil_spent: execution.sigil_spent,
            talisman_gained: execution.talisman_gained,
            talisman_spent: execution.talisman_spent,
            amulet_gained: execution.amulet_gained,
            amulet_spent: execution.amulet_spent,
            phial_gained: execution.phial_gained,
            phial_spent: execution.phial_spent,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LaneResolvedInputs {
    pub(crate) environment: InputTrace,
    pub(crate) observation: InputTrace,
    pub(crate) policy: InputTrace,
    pub(crate) coordination: InputTrace,
    pub(crate) execution: LaneExecutionInputs,
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
    pub(crate) relation: LaneEffectRelation,
    pub(crate) timing: LaneEffectTiming,
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

    pub const fn direct_delayed() -> Self {
        Self {
            relation: LaneEffectRelation::Direct,
            timing: LaneEffectTiming::Delayed,
        }
    }

    pub const fn indirect_delayed() -> Self {
        Self {
            relation: LaneEffectRelation::Indirect,
            timing: LaneEffectTiming::Delayed,
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
    TargetFocusSelected {
        actor: ActorId,
        focus: LaneTargetFocus,
    },
    CommitmentSelected {
        actor: ActorId,
        commitment: LaneCommitment,
    },
    PingSignalSelected {
        actor: ActorId,
        ping_signal: LanePingSignal,
    },
    AbortConditionSelected {
        actor: ActorId,
        abort_condition: LaneAbortCondition,
    },
    AbortConditionTriggered {
        actor: ActorId,
        abort_condition: LaneAbortCondition,
    },
    FallbackBehaviorSelected {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
    },
    FallbackBehaviorSet {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
    },
    FallbackBehaviorTriggered {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
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
    ExperienceGained {
        actor: ActorId,
        amount: LaneExperience,
        trace: InputTrace,
    },
    CooldownTicked {
        actor: ActorId,
        amount: u32,
        trace: InputTrace,
    },
    CooldownSet {
        actor: ActorId,
        amount: LaneCooldown,
        trace: InputTrace,
    },
    BountyEarned {
        actor: ActorId,
        amount: LaneBounty,
        trace: InputTrace,
    },
    LevelGained {
        actor: ActorId,
        amount: LaneLevel,
        trace: InputTrace,
    },
    MinionKillsGained {
        actor: ActorId,
        amount: LaneMinionKills,
        trace: InputTrace,
    },
    ShieldGained {
        actor: ActorId,
        amount: LaneShield,
        trace: InputTrace,
    },
    WardGained {
        actor: ActorId,
        amount: LaneWard,
        trace: InputTrace,
    },
    PotionGained {
        actor: ActorId,
        amount: LanePotion,
        trace: InputTrace,
    },
    PotionSpent {
        actor: ActorId,
        amount: LanePotion,
        trace: InputTrace,
    },
    ElixirGained {
        actor: ActorId,
        amount: LaneElixir,
        trace: InputTrace,
    },
    ElixirSpent {
        actor: ActorId,
        amount: LaneElixir,
        trace: InputTrace,
    },
    TrinketGained {
        actor: ActorId,
        amount: LaneTrinket,
        trace: InputTrace,
    },
    TrinketSpent {
        actor: ActorId,
        amount: LaneTrinket,
        trace: InputTrace,
    },
    RelicGained {
        actor: ActorId,
        amount: LaneRelic,
        trace: InputTrace,
    },
    RelicSpent {
        actor: ActorId,
        amount: LaneRelic,
        trace: InputTrace,
    },
    CharmGained {
        actor: ActorId,
        amount: LaneCharm,
        trace: InputTrace,
    },
    CharmSpent {
        actor: ActorId,
        amount: LaneCharm,
        trace: InputTrace,
    },
    ScrollGained {
        actor: ActorId,
        amount: LaneScroll,
        trace: InputTrace,
    },
    ScrollSpent {
        actor: ActorId,
        amount: LaneScroll,
        trace: InputTrace,
    },
    TomeGained {
        actor: ActorId,
        amount: LaneTome,
        trace: InputTrace,
    },
    TomeSpent {
        actor: ActorId,
        amount: LaneTome,
        trace: InputTrace,
    },
    RuneGained {
        actor: ActorId,
        amount: LaneRune,
        trace: InputTrace,
    },
    RuneSpent {
        actor: ActorId,
        amount: LaneRune,
        trace: InputTrace,
    },
    SigilGained {
        actor: ActorId,
        amount: LaneSigil,
        trace: InputTrace,
    },
    SigilSpent {
        actor: ActorId,
        amount: LaneSigil,
        trace: InputTrace,
    },
    TalismanGained {
        actor: ActorId,
        amount: LaneTalisman,
        trace: InputTrace,
    },
    TalismanSpent {
        actor: ActorId,
        amount: LaneTalisman,
        trace: InputTrace,
    },
    AmuletGained {
        actor: ActorId,
        amount: LaneAmulet,
        trace: InputTrace,
    },
    AmuletSpent {
        actor: ActorId,
        amount: LaneAmulet,
        trace: InputTrace,
    },
    PhialGained {
        actor: ActorId,
        amount: LanePhial,
        trace: InputTrace,
    },
    PhialSpent {
        actor: ActorId,
        amount: LanePhial,
        trace: InputTrace,
    },
    DelayedEffectQueued {
        actor: ActorId,
        effect: LaneDelayedEffect,
        trace: InputTrace,
    },
    DelayedEffectResolved {
        actor: ActorId,
        effect: LaneDelayedEffect,
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
    ExperienceChanged {
        actor: ActorId,
        before: LaneExperience,
        after: LaneExperience,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    CooldownChanged {
        actor: ActorId,
        before: LaneCooldown,
        after: LaneCooldown,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    BountyChanged {
        actor: ActorId,
        before: LaneBounty,
        after: LaneBounty,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    LevelChanged {
        actor: ActorId,
        before: LaneLevel,
        after: LaneLevel,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    MinionKillsChanged {
        actor: ActorId,
        before: LaneMinionKills,
        after: LaneMinionKills,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    ShieldChanged {
        actor: ActorId,
        before: LaneShield,
        after: LaneShield,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    WardChanged {
        actor: ActorId,
        before: LaneWard,
        after: LaneWard,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    PotionChanged {
        actor: ActorId,
        before: LanePotion,
        after: LanePotion,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    ElixirChanged {
        actor: ActorId,
        before: LaneElixir,
        after: LaneElixir,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    TrinketChanged {
        actor: ActorId,
        before: LaneTrinket,
        after: LaneTrinket,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    RelicChanged {
        actor: ActorId,
        before: LaneRelic,
        after: LaneRelic,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    CharmChanged {
        actor: ActorId,
        before: LaneCharm,
        after: LaneCharm,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    ScrollChanged {
        actor: ActorId,
        before: LaneScroll,
        after: LaneScroll,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    TomeChanged {
        actor: ActorId,
        before: LaneTome,
        after: LaneTome,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    RuneChanged {
        actor: ActorId,
        before: LaneRune,
        after: LaneRune,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    SigilChanged {
        actor: ActorId,
        before: LaneSigil,
        after: LaneSigil,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    TalismanChanged {
        actor: ActorId,
        before: LaneTalisman,
        after: LaneTalisman,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    AmuletChanged {
        actor: ActorId,
        before: LaneAmulet,
        after: LaneAmulet,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    PhialChanged {
        actor: ActorId,
        before: LanePhial,
        after: LanePhial,
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
    DelayedEffectQueued {
        actor: ActorId,
        effect: LaneDelayedEffect,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    DelayedEffectResolved {
        actor: ActorId,
        effect: LaneDelayedEffect,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    TargetFocusSet {
        actor: ActorId,
        focus: LaneTargetFocus,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    CommitmentSet {
        actor: ActorId,
        commitment: LaneCommitment,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    PingSignalSet {
        actor: ActorId,
        ping_signal: LanePingSignal,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    AbortConditionSet {
        actor: ActorId,
        abort_condition: LaneAbortCondition,
        cause: LaneEffectCause,
        provenance: LaneEffectProvenance,
    },
    FallbackBehaviorSet {
        actor: ActorId,
        fallback_behavior: LaneFallbackBehavior,
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
            | Self::ExperienceChanged { provenance, .. }
            | Self::CooldownChanged { provenance, .. }
            | Self::BountyChanged { provenance, .. }
            | Self::LevelChanged { provenance, .. }
            | Self::MinionKillsChanged { provenance, .. }
            | Self::ShieldChanged { provenance, .. }
            | Self::WardChanged { provenance, .. }
            | Self::PotionChanged { provenance, .. }
            | Self::ElixirChanged { provenance, .. }
            | Self::TrinketChanged { provenance, .. }
            | Self::RelicChanged { provenance, .. }
            | Self::CharmChanged { provenance, .. }
            | Self::ScrollChanged { provenance, .. }
            | Self::TomeChanged { provenance, .. }
            | Self::RuneChanged { provenance, .. }
            | Self::SigilChanged { provenance, .. }
            | Self::TalismanChanged { provenance, .. }
            | Self::AmuletChanged { provenance, .. }
            | Self::PhialChanged { provenance, .. }
            | Self::PositionChanged { provenance, .. }
            | Self::DelayedEffectQueued { provenance, .. }
            | Self::DelayedEffectResolved { provenance, .. }
            | Self::TargetFocusSet { provenance, .. }
            | Self::CommitmentSet { provenance, .. }
            | Self::PingSignalSet { provenance, .. }
            | Self::AbortConditionSet { provenance, .. }
            | Self::FallbackBehaviorSet { provenance, .. } => provenance,
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
    ExperienceOverflow {
        gained: LaneExperience,
        current: LaneExperience,
    },
    CooldownOverflow {
        set: LaneCooldown,
        current: LaneCooldown,
    },
    BountyOverflow {
        earned: LaneBounty,
        current: LaneBounty,
    },
    LevelOverflow {
        gained: LaneLevel,
        current: LaneLevel,
    },
    MinionKillsOverflow {
        gained: LaneMinionKills,
        current: LaneMinionKills,
    },
    ShieldOverflow {
        gained: LaneShield,
        current: LaneShield,
    },
    WardOverflow {
        gained: LaneWard,
        current: LaneWard,
    },
    PotionOverflow {
        gained: LanePotion,
        current: LanePotion,
    },
    InsufficientPotion {
        spent: LanePotion,
        available: LanePotion,
    },
    ElixirOverflow {
        gained: LaneElixir,
        current: LaneElixir,
    },
    InsufficientElixir {
        spent: LaneElixir,
        available: LaneElixir,
    },
    TrinketOverflow {
        gained: LaneTrinket,
        current: LaneTrinket,
    },
    InsufficientTrinket {
        spent: LaneTrinket,
        available: LaneTrinket,
    },
    RelicOverflow {
        gained: LaneRelic,
        current: LaneRelic,
    },
    InsufficientRelic {
        spent: LaneRelic,
        available: LaneRelic,
    },
    CharmOverflow {
        gained: LaneCharm,
        current: LaneCharm,
    },
    InsufficientCharm {
        spent: LaneCharm,
        available: LaneCharm,
    },
    ScrollOverflow {
        gained: LaneScroll,
        current: LaneScroll,
    },
    InsufficientScroll {
        spent: LaneScroll,
        available: LaneScroll,
    },
    TomeOverflow {
        gained: LaneTome,
        current: LaneTome,
    },
    InsufficientTome {
        spent: LaneTome,
        available: LaneTome,
    },
    RuneOverflow {
        gained: LaneRune,
        current: LaneRune,
    },
    InsufficientRune {
        spent: LaneRune,
        available: LaneRune,
    },
    SigilOverflow {
        gained: LaneSigil,
        current: LaneSigil,
    },
    InsufficientSigil {
        spent: LaneSigil,
        available: LaneSigil,
    },
    TalismanOverflow {
        gained: LaneTalisman,
        current: LaneTalisman,
    },
    InsufficientTalisman {
        spent: LaneTalisman,
        available: LaneTalisman,
    },
    AmuletOverflow {
        gained: LaneAmulet,
        current: LaneAmulet,
    },
    InsufficientAmulet {
        spent: LaneAmulet,
        available: LaneAmulet,
    },
    PhialOverflow {
        gained: LanePhial,
        current: LanePhial,
    },
    InsufficientPhial {
        spent: LanePhial,
        available: LanePhial,
    },
    DelayedEffectOverflow,
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
    pub(crate) decision: LaneDecisionReview,
    pub(crate) coordination: LaneCoordinationReview,
    pub(crate) intent: LaneIntent,
    pub(crate) target_focus: LaneTargetFocus,
    pub(crate) commitment: LaneCommitment,
    pub(crate) ping_signal: LanePingSignal,
    pub(crate) abort_condition: LaneAbortCondition,
    pub(crate) fallback_behavior: LaneFallbackBehavior,
    pub(crate) self_damage: LaneDamage,
    pub(crate) mana_spent: LaneMana,
    pub(crate) gold_earned: LaneGold,
    pub(crate) experience_gained: LaneExperience,
    pub(crate) cooldown_set: LaneCooldown,
    pub(crate) bounty_earned: LaneBounty,
    pub(crate) level_gained: LaneLevel,
    pub(crate) minion_kills_gained: LaneMinionKills,
    pub(crate) shield_gained: LaneShield,
    pub(crate) ward_gained: LaneWard,
    pub(crate) potion_gained: LanePotion,
    pub(crate) potion_spent: LanePotion,
    pub(crate) elixir_gained: LaneElixir,
    pub(crate) elixir_spent: LaneElixir,
    pub(crate) trinket_gained: LaneTrinket,
    pub(crate) trinket_spent: LaneTrinket,
    pub(crate) relic_gained: LaneRelic,
    pub(crate) relic_spent: LaneRelic,
    pub(crate) charm_gained: LaneCharm,
    pub(crate) charm_spent: LaneCharm,
    pub(crate) scroll_gained: LaneScroll,
    pub(crate) scroll_spent: LaneScroll,
    pub(crate) tome_gained: LaneTome,
    pub(crate) tome_spent: LaneTome,
    pub(crate) rune_gained: LaneRune,
    pub(crate) rune_spent: LaneRune,
    pub(crate) sigil_gained: LaneSigil,
    pub(crate) sigil_spent: LaneSigil,
    pub(crate) talisman_gained: LaneTalisman,
    pub(crate) talisman_spent: LaneTalisman,
    pub(crate) amulet_gained: LaneAmulet,
    pub(crate) amulet_spent: LaneAmulet,
    pub(crate) phial_gained: LanePhial,
    pub(crate) phial_spent: LanePhial,
    pub(crate) wave_result: LaneWaveResult,
    pub(crate) fallback_activated: bool,
    pub(crate) delayed_effects_queued: u8,
    pub(crate) delayed_effects_resolved: u8,
    pub(crate) execution_trace: InputTrace,
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

    pub fn target_focus(self) -> LaneTargetFocus {
        self.target_focus
    }

    pub fn commitment(self) -> LaneCommitment {
        self.commitment
    }

    pub fn ping_signal(self) -> LanePingSignal {
        self.ping_signal
    }

    pub fn abort_condition(self) -> LaneAbortCondition {
        self.abort_condition
    }

    pub fn fallback_behavior(self) -> LaneFallbackBehavior {
        self.fallback_behavior
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

    pub fn experience_gained(self) -> LaneExperience {
        self.experience_gained
    }

    pub fn cooldown_set(self) -> LaneCooldown {
        self.cooldown_set
    }

    pub fn bounty_earned(self) -> LaneBounty {
        self.bounty_earned
    }

    pub fn level_gained(self) -> LaneLevel {
        self.level_gained
    }

    pub fn minion_kills_gained(self) -> LaneMinionKills {
        self.minion_kills_gained
    }

    pub fn shield_gained(self) -> LaneShield {
        self.shield_gained
    }

    pub fn ward_gained(self) -> LaneWard {
        self.ward_gained
    }

    pub fn potion_gained(self) -> LanePotion {
        self.potion_gained
    }

    pub fn potion_spent(self) -> LanePotion {
        self.potion_spent
    }

    pub fn elixir_gained(self) -> LaneElixir {
        self.elixir_gained
    }

    pub fn elixir_spent(self) -> LaneElixir {
        self.elixir_spent
    }

    pub fn trinket_gained(self) -> LaneTrinket {
        self.trinket_gained
    }

    pub fn trinket_spent(self) -> LaneTrinket {
        self.trinket_spent
    }

    pub fn relic_gained(self) -> LaneRelic {
        self.relic_gained
    }

    pub fn relic_spent(self) -> LaneRelic {
        self.relic_spent
    }

    pub fn charm_gained(self) -> LaneCharm {
        self.charm_gained
    }

    pub fn charm_spent(self) -> LaneCharm {
        self.charm_spent
    }

    pub fn scroll_gained(self) -> LaneScroll {
        self.scroll_gained
    }

    pub fn scroll_spent(self) -> LaneScroll {
        self.scroll_spent
    }

    pub fn tome_gained(self) -> LaneTome {
        self.tome_gained
    }

    pub fn tome_spent(self) -> LaneTome {
        self.tome_spent
    }

    pub fn rune_gained(self) -> LaneRune {
        self.rune_gained
    }

    pub fn rune_spent(self) -> LaneRune {
        self.rune_spent
    }

    pub fn sigil_gained(self) -> LaneSigil {
        self.sigil_gained
    }

    pub fn sigil_spent(self) -> LaneSigil {
        self.sigil_spent
    }

    pub fn talisman_gained(self) -> LaneTalisman {
        self.talisman_gained
    }

    pub fn talisman_spent(self) -> LaneTalisman {
        self.talisman_spent
    }

    pub fn amulet_gained(self) -> LaneAmulet {
        self.amulet_gained
    }

    pub fn amulet_spent(self) -> LaneAmulet {
        self.amulet_spent
    }

    pub fn phial_gained(self) -> LanePhial {
        self.phial_gained
    }

    pub fn phial_spent(self) -> LanePhial {
        self.phial_spent
    }

    pub fn wave_result(self) -> LaneWaveResult {
        self.wave_result
    }

    pub fn fallback_activated(self) -> bool {
        self.fallback_activated
    }

    pub fn delayed_effects_queued(self) -> u8 {
        self.delayed_effects_queued
    }

    pub fn delayed_effects_resolved(self) -> u8 {
        self.delayed_effects_resolved
    }

    pub fn execution_trace(self) -> InputTrace {
        self.execution_trace
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaneTransitionResult {
    pub(crate) next_state: LaneSnapshot,
    pub(crate) events: Vec<LaneEvent>,
    pub(crate) effects: Vec<LaneEffect>,
    pub(crate) outcome: LaneOutcome,
    pub(crate) debrief: LaneDebrief,
    pub(crate) state_hash: StateHash,
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
    pub(crate) decision: CoordinatedDecisionReview,
    pub(crate) response: CoordinatedResponseReview,
    pub(crate) coordination: CoordinationDisposition,
    pub(crate) execution: CoordinatedExecutionReview,
    pub(crate) luck: CoordinatedLuckReview,
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
    pub(crate) lane: LaneTransitionResult,
    pub(crate) coordination: CoordinationResolution,
    pub(crate) events: Vec<CoordinatedEvent>,
    pub(crate) effects: Vec<CoordinatedEffect>,
    pub(crate) debrief: CoordinatedDebrief,
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

fn advance_wave(pressure: WavePressure) -> Result<WavePressure, LaneExecutionError> {
    WavePressure::new(pressure.value() + 1)
        .map_err(|_| LaneExecutionError::WaveOverflow { pressure })
}

fn lose_wave(pressure: WavePressure) -> Result<WavePressure, LaneExecutionError> {
    pressure
        .value()
        .checked_sub(1)
        .map(WavePressure)
        .ok_or(LaneExecutionError::WaveUnderflow { pressure })
}

fn apply_player_resources(
    before: PlayerResources,
    deltas: ResourceExecutionDeltas,
    window_beats: u32,
    intent: LaneIntent,
) -> Result<PlayerResources, LaneExecutionError> {
    if deltas.mana_spent != LaneMana::zero() && intent != LaneIntent::Contest {
        return Err(LaneExecutionError::ManaSpentWithoutContest {
            intent,
            spent: deltas.mana_spent,
        });
    }
    let mana = before.mana.subtract(deltas.mana_spent).ok_or(
        LaneExecutionError::ManaExceedsAvailable {
            spent: deltas.mana_spent,
            available: before.mana,
        },
    )?;
    let gold = before
        .gold
        .add(deltas.gold_earned)
        .ok_or(LaneExecutionError::GoldOverflow {
            earned: deltas.gold_earned,
            current: before.gold,
        })?;
    let experience = before.experience.add(deltas.experience_gained).ok_or(
        LaneExecutionError::ExperienceOverflow {
            gained: deltas.experience_gained,
            current: before.experience,
        },
    )?;
    let ticked_cooldown = before.cooldown.tick(window_beats);
    let cooldown =
        ticked_cooldown
            .add(deltas.cooldown_set)
            .ok_or(LaneExecutionError::CooldownOverflow {
                set: deltas.cooldown_set,
                current: ticked_cooldown,
            })?;
    let bounty =
        before
            .bounty
            .add(deltas.bounty_earned)
            .ok_or(LaneExecutionError::BountyOverflow {
                earned: deltas.bounty_earned,
                current: before.bounty,
            })?;
    let level = before
        .level
        .add(deltas.level_gained)
        .ok_or(LaneExecutionError::LevelOverflow {
            gained: deltas.level_gained,
            current: before.level,
        })?;
    let minion_kills = before.minion_kills.add(deltas.minion_kills_gained).ok_or(
        LaneExecutionError::MinionKillsOverflow {
            gained: deltas.minion_kills_gained,
            current: before.minion_kills,
        },
    )?;
    let shield =
        before
            .shield
            .add(deltas.shield_gained)
            .ok_or(LaneExecutionError::ShieldOverflow {
                gained: deltas.shield_gained,
                current: before.shield,
            })?;
    let ward = before
        .ward
        .add(deltas.ward_gained)
        .ok_or(LaneExecutionError::WardOverflow {
            gained: deltas.ward_gained,
            current: before.ward,
        })?;
    let after_spend_potion = before.potion.subtract(deltas.potion_spent).ok_or(
        LaneExecutionError::InsufficientPotion {
            spent: deltas.potion_spent,
            available: before.potion,
        },
    )?;
    let potion =
        after_spend_potion
            .add(deltas.potion_gained)
            .ok_or(LaneExecutionError::PotionOverflow {
                gained: deltas.potion_gained,
                current: after_spend_potion,
            })?;
    let after_spend_elixir = before.elixir.subtract(deltas.elixir_spent).ok_or(
        LaneExecutionError::InsufficientElixir {
            spent: deltas.elixir_spent,
            available: before.elixir,
        },
    )?;
    let elixir =
        after_spend_elixir
            .add(deltas.elixir_gained)
            .ok_or(LaneExecutionError::ElixirOverflow {
                gained: deltas.elixir_gained,
                current: after_spend_elixir,
            })?;
    let after_spend_trinket = before.trinket.subtract(deltas.trinket_spent).ok_or(
        LaneExecutionError::InsufficientTrinket {
            spent: deltas.trinket_spent,
            available: before.trinket,
        },
    )?;
    let trinket = after_spend_trinket.add(deltas.trinket_gained).ok_or(
        LaneExecutionError::TrinketOverflow {
            gained: deltas.trinket_gained,
            current: after_spend_trinket,
        },
    )?;
    let after_spend_relic =
        before
            .relic
            .subtract(deltas.relic_spent)
            .ok_or(LaneExecutionError::InsufficientRelic {
                spent: deltas.relic_spent,
                available: before.relic,
            })?;
    let relic =
        after_spend_relic
            .add(deltas.relic_gained)
            .ok_or(LaneExecutionError::RelicOverflow {
                gained: deltas.relic_gained,
                current: after_spend_relic,
            })?;
    let after_spend_charm =
        before
            .charm
            .subtract(deltas.charm_spent)
            .ok_or(LaneExecutionError::InsufficientCharm {
                spent: deltas.charm_spent,
                available: before.charm,
            })?;
    let charm =
        after_spend_charm
            .add(deltas.charm_gained)
            .ok_or(LaneExecutionError::CharmOverflow {
                gained: deltas.charm_gained,
                current: after_spend_charm,
            })?;
    let after_spend_scroll = before.scroll.subtract(deltas.scroll_spent).ok_or(
        LaneExecutionError::InsufficientScroll {
            spent: deltas.scroll_spent,
            available: before.scroll,
        },
    )?;
    let scroll =
        after_spend_scroll
            .add(deltas.scroll_gained)
            .ok_or(LaneExecutionError::ScrollOverflow {
                gained: deltas.scroll_gained,
                current: after_spend_scroll,
            })?;
    let after_spend_tome =
        before
            .tome
            .subtract(deltas.tome_spent)
            .ok_or(LaneExecutionError::InsufficientTome {
                spent: deltas.tome_spent,
                available: before.tome,
            })?;
    let tome =
        after_spend_tome
            .add(deltas.tome_gained)
            .ok_or(LaneExecutionError::TomeOverflow {
                gained: deltas.tome_gained,
                current: after_spend_tome,
            })?;
    let after_spend_rune =
        before
            .rune
            .subtract(deltas.rune_spent)
            .ok_or(LaneExecutionError::InsufficientRune {
                spent: deltas.rune_spent,
                available: before.rune,
            })?;
    let rune =
        after_spend_rune
            .add(deltas.rune_gained)
            .ok_or(LaneExecutionError::RuneOverflow {
                gained: deltas.rune_gained,
                current: after_spend_rune,
            })?;
    let after_spend_sigil =
        before
            .sigil
            .subtract(deltas.sigil_spent)
            .ok_or(LaneExecutionError::InsufficientSigil {
                spent: deltas.sigil_spent,
                available: before.sigil,
            })?;
    let sigil =
        after_spend_sigil
            .add(deltas.sigil_gained)
            .ok_or(LaneExecutionError::SigilOverflow {
                gained: deltas.sigil_gained,
                current: after_spend_sigil,
            })?;
    let after_spend_talisman = before.talisman.subtract(deltas.talisman_spent).ok_or(
        LaneExecutionError::InsufficientTalisman {
            spent: deltas.talisman_spent,
            available: before.talisman,
        },
    )?;
    let talisman = after_spend_talisman.add(deltas.talisman_gained).ok_or(
        LaneExecutionError::TalismanOverflow {
            gained: deltas.talisman_gained,
            current: after_spend_talisman,
        },
    )?;
    let after_spend_amulet = before.amulet.subtract(deltas.amulet_spent).ok_or(
        LaneExecutionError::InsufficientAmulet {
            spent: deltas.amulet_spent,
            available: before.amulet,
        },
    )?;
    let amulet =
        after_spend_amulet
            .add(deltas.amulet_gained)
            .ok_or(LaneExecutionError::AmuletOverflow {
                gained: deltas.amulet_gained,
                current: after_spend_amulet,
            })?;
    let after_spend_phial =
        before
            .phial
            .subtract(deltas.phial_spent)
            .ok_or(LaneExecutionError::InsufficientPhial {
                spent: deltas.phial_spent,
                available: before.phial,
            })?;
    let phial =
        after_spend_phial
            .add(deltas.phial_gained)
            .ok_or(LaneExecutionError::PhialOverflow {
                gained: deltas.phial_gained,
                current: after_spend_phial,
            })?;
    Ok(PlayerResources {
        mana,
        gold,
        experience,
        cooldown,
        bounty,
        level,
        minion_kills,
        shield,
        ward,
        potion,
        elixir,
        trinket,
        relic,
        charm,
        scroll,
        tome,
        rune,
        sigil,
        talisman,
        amulet,
        phial,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedLaneExecution {
    next_state: LaneSnapshot,
    outcome: LaneOutcome,
    fallback_activated: bool,
    delayed_effects_resolved: Vec<LaneDelayedEffect>,
    delayed_effect_queued: Option<LaneDelayedEffect>,
}

fn resolve_lane_execution(
    state: &LaneSnapshot,
    command: &ValidatedLaneIntent,
    execution: LaneExecutionInputs,
) -> Result<ResolvedLaneExecution, LaneTransitionError> {
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
    let resource_deltas = ResourceExecutionDeltas::from_execution(execution);
    let mut after_resources = apply_player_resources(
        player.resources(),
        resource_deltas,
        state.window.beats(),
        command.command.intent,
    )
    .map_err(LaneTransitionError::Execution)?;
    let after_wave = match execution.wave_result {
        LaneWaveResult::Advanced => advance_wave(state.wave.pressure),
        LaneWaveResult::Held => Ok(state.wave.pressure),
        LaneWaveResult::Lost => lose_wave(state.wave.pressure),
    }
    .map_err(LaneTransitionError::Execution)?;
    let mut after_player_health = player
        .health
        .subtract(execution.self_damage)
        .expect("validated damage must be subtractable");
    let after_opponent_health = opponent
        .health
        .subtract(execution.opponent_damage)
        .expect("validated damage must be subtractable");

    let beats = state.window.beats() as u8;
    let mut next_delayed_effects = LaneDelayedEffects::empty();
    let mut delayed_effects_resolved = Vec::new();

    for effect in state.delayed_effects.items().iter().flatten() {
        if effect.delay_beats <= beats {
            delayed_effects_resolved.push(*effect);
            match effect.kind {
                LaneDelayedEffectKind::SelfHealthRegen { amount } => {
                    let val = (after_player_health.0 + amount.0).min(MAX_LANE_HEALTH);
                    after_player_health = LaneHealth::new(val).expect("valid bounded health");
                }
                LaneDelayedEffectKind::SelfManaRegen { amount } => {
                    after_resources.mana =
                        after_resources.mana.add(amount).unwrap_or(LaneMana::full());
                }
                LaneDelayedEffectKind::SelfCooldownReduction { amount } => {
                    after_resources.cooldown = after_resources.cooldown.tick(amount.value() as u32);
                }
            }
        } else {
            let remaining = effect.delay_beats - beats;
            next_delayed_effects
                .push(LaneDelayedEffect::new(remaining, effect.kind))
                .expect("valid queue bounds");
        }
    }

    let mut delayed_effect_queued = None;
    if let Some(new_delayed) = execution.delayed_effect {
        next_delayed_effects.push(new_delayed).map_err(|_| {
            LaneTransitionError::Execution(LaneExecutionError::DelayedEffectOverflow)
        })?;
        delayed_effect_queued = Some(new_delayed);
    }

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
    let next_player = PlayerLaneState::from_resources(
        player.id,
        after_player_health,
        after_resources,
        after_position,
    );
    let next_opponent = OpponentTruth::new(
        opponent.id,
        after_opponent_health,
        opponent.position,
        opponent.posture,
    );
    let next_state = LaneSnapshot::new_with_delayed_effects(
        state.ruleset,
        Turn::new(next_turn),
        state.window,
        LanePhase::Resolved,
        next_player,
        next_opponent,
        WaveState::new(after_wave),
        state.jungle_threat,
        next_delayed_effects,
        Some(outcome),
    );
    Ok(ResolvedLaneExecution {
        next_state,
        outcome,
        fallback_activated,
        delayed_effects_resolved,
        delayed_effect_queued,
    })
}

fn project_lane_events(
    state: &LaneSnapshot,
    command: &ValidatedLaneIntent,
    execution: LaneExecutionInputs,
    resolved: &ResolvedLaneExecution,
    trace: InputTrace,
) -> Vec<LaneEvent> {
    let player = state.player;
    let opponent = state.opponent;
    let mut events = vec![
        LaneEvent::IntentCommitted {
            actor: command.command.actor,
            intent: command.command.intent,
        },
        LaneEvent::TargetFocusSelected {
            actor: command.command.actor,
            focus: command.command.target_focus,
        },
        LaneEvent::CommitmentSelected {
            actor: command.command.actor,
            commitment: command.command.commitment,
        },
        LaneEvent::PingSignalSelected {
            actor: command.command.actor,
            ping_signal: command.command.ping_signal,
        },
        LaneEvent::AbortConditionSelected {
            actor: command.command.actor,
            abort_condition: command.command.abort_condition,
        },
        LaneEvent::FallbackBehaviorSelected {
            actor: command.command.actor,
            fallback_behavior: command.command.fallback_behavior,
        },
    ];
    if command.command.abort_condition != LaneAbortCondition::None {
        events.push(LaneEvent::AbortConditionTriggered {
            actor: command.command.actor,
            abort_condition: command.command.abort_condition,
        });
    }
    if command.command.fallback_behavior != LaneFallbackBehavior::MaintainPlan {
        events.push(LaneEvent::FallbackBehaviorSet {
            actor: command.command.actor,
            fallback_behavior: command.command.fallback_behavior,
        });
    }
    if execution.self_damage != LaneDamage::zero() {
        events.push(LaneEvent::PlayerDamaged {
            target: player.id,
            amount: execution.self_damage,
            trace,
        });
    }
    if execution.opponent_damage != LaneDamage::zero() {
        events.push(LaneEvent::OpponentDamaged {
            target: opponent.id,
            amount: execution.opponent_damage,
            trace,
        });
    }
    if execution.mana_spent != LaneMana::zero() {
        events.push(LaneEvent::ManaSpent {
            actor: player.id,
            amount: execution.mana_spent,
            trace,
        });
    }
    if execution.gold_earned != LaneGold::zero() {
        events.push(LaneEvent::GoldEarned {
            actor: player.id,
            amount: execution.gold_earned,
            trace,
        });
    }
    if execution.experience_gained != LaneExperience::zero() {
        events.push(LaneEvent::ExperienceGained {
            actor: player.id,
            amount: execution.experience_gained,
            trace,
        });
    }
    if resolved.next_state.player.cooldown != player.cooldown {
        if execution.cooldown_set != LaneCooldown::zero() {
            events.push(LaneEvent::CooldownSet {
                actor: player.id,
                amount: execution.cooldown_set,
                trace,
            });
        } else {
            events.push(LaneEvent::CooldownTicked {
                actor: player.id,
                amount: state.window.beats(),
                trace,
            });
        }
    }
    if execution.bounty_earned != LaneBounty::zero() {
        events.push(LaneEvent::BountyEarned {
            actor: player.id,
            amount: execution.bounty_earned,
            trace,
        });
    }
    if execution.level_gained != LaneLevel::zero() {
        events.push(LaneEvent::LevelGained {
            actor: player.id,
            amount: execution.level_gained,
            trace,
        });
    }
    if execution.minion_kills_gained != LaneMinionKills::zero() {
        events.push(LaneEvent::MinionKillsGained {
            actor: player.id,
            amount: execution.minion_kills_gained,
            trace,
        });
    }
    if execution.shield_gained != LaneShield::zero() {
        events.push(LaneEvent::ShieldGained {
            actor: player.id,
            amount: execution.shield_gained,
            trace,
        });
    }
    if execution.ward_gained != LaneWard::zero() {
        events.push(LaneEvent::WardGained {
            actor: player.id,
            amount: execution.ward_gained,
            trace,
        });
    }
    if execution.potion_gained != LanePotion::zero() {
        events.push(LaneEvent::PotionGained {
            actor: player.id,
            amount: execution.potion_gained,
            trace,
        });
    }
    if execution.potion_spent != LanePotion::zero() {
        events.push(LaneEvent::PotionSpent {
            actor: player.id,
            amount: execution.potion_spent,
            trace,
        });
    }
    if execution.elixir_gained != LaneElixir::zero() {
        events.push(LaneEvent::ElixirGained {
            actor: player.id,
            amount: execution.elixir_gained,
            trace,
        });
    }
    if execution.elixir_spent != LaneElixir::zero() {
        events.push(LaneEvent::ElixirSpent {
            actor: player.id,
            amount: execution.elixir_spent,
            trace,
        });
    }
    if execution.trinket_gained != LaneTrinket::zero() {
        events.push(LaneEvent::TrinketGained {
            actor: player.id,
            amount: execution.trinket_gained,
            trace,
        });
    }
    if execution.trinket_spent != LaneTrinket::zero() {
        events.push(LaneEvent::TrinketSpent {
            actor: player.id,
            amount: execution.trinket_spent,
            trace,
        });
    }
    if execution.relic_gained != LaneRelic::zero() {
        events.push(LaneEvent::RelicGained {
            actor: player.id,
            amount: execution.relic_gained,
            trace,
        });
    }
    if execution.relic_spent != LaneRelic::zero() {
        events.push(LaneEvent::RelicSpent {
            actor: player.id,
            amount: execution.relic_spent,
            trace,
        });
    }
    if execution.charm_gained != LaneCharm::zero() {
        events.push(LaneEvent::CharmGained {
            actor: player.id,
            amount: execution.charm_gained,
            trace,
        });
    }
    if execution.charm_spent != LaneCharm::zero() {
        events.push(LaneEvent::CharmSpent {
            actor: player.id,
            amount: execution.charm_spent,
            trace,
        });
    }
    if execution.scroll_gained != LaneScroll::zero() {
        events.push(LaneEvent::ScrollGained {
            actor: player.id,
            amount: execution.scroll_gained,
            trace,
        });
    }
    if execution.scroll_spent != LaneScroll::zero() {
        events.push(LaneEvent::ScrollSpent {
            actor: player.id,
            amount: execution.scroll_spent,
            trace,
        });
    }
    if execution.tome_gained != LaneTome::zero() {
        events.push(LaneEvent::TomeGained {
            actor: player.id,
            amount: execution.tome_gained,
            trace,
        });
    }
    if execution.tome_spent != LaneTome::zero() {
        events.push(LaneEvent::TomeSpent {
            actor: player.id,
            amount: execution.tome_spent,
            trace,
        });
    }
    if execution.rune_gained != LaneRune::zero() {
        events.push(LaneEvent::RuneGained {
            actor: player.id,
            amount: execution.rune_gained,
            trace,
        });
    }
    if execution.rune_spent != LaneRune::zero() {
        events.push(LaneEvent::RuneSpent {
            actor: player.id,
            amount: execution.rune_spent,
            trace,
        });
    }
    if execution.sigil_gained != LaneSigil::zero() {
        events.push(LaneEvent::SigilGained {
            actor: player.id,
            amount: execution.sigil_gained,
            trace,
        });
    }
    if execution.sigil_spent != LaneSigil::zero() {
        events.push(LaneEvent::SigilSpent {
            actor: player.id,
            amount: execution.sigil_spent,
            trace,
        });
    }
    if execution.talisman_gained != LaneTalisman::zero() {
        events.push(LaneEvent::TalismanGained {
            actor: player.id,
            amount: execution.talisman_gained,
            trace,
        });
    }
    if execution.talisman_spent != LaneTalisman::zero() {
        events.push(LaneEvent::TalismanSpent {
            actor: player.id,
            amount: execution.talisman_spent,
            trace,
        });
    }
    if execution.amulet_gained != LaneAmulet::zero() {
        events.push(LaneEvent::AmuletGained {
            actor: player.id,
            amount: execution.amulet_gained,
            trace,
        });
    }
    if execution.amulet_spent != LaneAmulet::zero() {
        events.push(LaneEvent::AmuletSpent {
            actor: player.id,
            amount: execution.amulet_spent,
            trace,
        });
    }
    if execution.phial_gained != LanePhial::zero() {
        events.push(LaneEvent::PhialGained {
            actor: player.id,
            amount: execution.phial_gained,
            trace,
        });
    }
    if execution.phial_spent != LanePhial::zero() {
        events.push(LaneEvent::PhialSpent {
            actor: player.id,
            amount: execution.phial_spent,
            trace,
        });
    }
    if let Some(queued) = resolved.delayed_effect_queued {
        events.push(LaneEvent::DelayedEffectQueued {
            actor: player.id,
            effect: queued,
            trace,
        });
    }
    for item in &resolved.delayed_effects_resolved {
        events.push(LaneEvent::DelayedEffectResolved {
            actor: player.id,
            effect: *item,
            trace,
        });
    }
    events.push(LaneEvent::WaveResolved {
        before: state.wave.pressure,
        after: resolved.next_state.wave.pressure,
        trace,
    });
    if resolved.fallback_activated {
        events.push(LaneEvent::FallbackActivated {
            actor: player.id,
            intent: command.command.intent,
        });
        if command.command.fallback_behavior != LaneFallbackBehavior::MaintainPlan {
            events.push(LaneEvent::FallbackBehaviorTriggered {
                actor: player.id,
                fallback_behavior: command.command.fallback_behavior,
            });
        }
    }
    events.push(LaneEvent::WindowResolved {
        outcome: resolved.outcome,
    });
    events
}

fn project_lane_effects(
    state: &LaneSnapshot,
    command: &ValidatedLaneIntent,
    execution: LaneExecutionInputs,
    resolved: &ResolvedLaneExecution,
    trace: InputTrace,
) -> Vec<LaneEffect> {
    let player = state.player;
    let opponent = state.opponent;
    let next_state = resolved.next_state;
    let next_player = next_state.player;
    let mut effects = vec![
        LaneEffect::TargetFocusSet {
            actor: player.id,
            focus: command.command.target_focus,
            cause: LaneEffectCause::Intent,
            provenance: LaneEffectProvenance::direct_immediate(),
        },
        LaneEffect::CommitmentSet {
            actor: player.id,
            commitment: command.command.commitment,
            cause: LaneEffectCause::Intent,
            provenance: LaneEffectProvenance::direct_immediate(),
        },
        LaneEffect::PingSignalSet {
            actor: player.id,
            ping_signal: command.command.ping_signal,
            cause: LaneEffectCause::Intent,
            provenance: LaneEffectProvenance::direct_immediate(),
        },
        LaneEffect::AbortConditionSet {
            actor: player.id,
            abort_condition: command.command.abort_condition,
            cause: LaneEffectCause::Intent,
            provenance: LaneEffectProvenance::direct_immediate(),
        },
        LaneEffect::FallbackBehaviorSet {
            actor: player.id,
            fallback_behavior: command.command.fallback_behavior,
            cause: LaneEffectCause::Intent,
            provenance: LaneEffectProvenance::direct_immediate(),
        },
    ];
    if execution.self_damage != LaneDamage::zero() {
        effects.push(LaneEffect::HealthChanged {
            actor: player.id,
            before: player.health,
            after: next_player.health,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.opponent_damage != LaneDamage::zero() {
        effects.push(LaneEffect::HealthChanged {
            actor: opponent.id,
            before: opponent.health,
            after: next_state.opponent.health,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.mana_spent != LaneMana::zero() {
        effects.push(LaneEffect::ManaChanged {
            actor: player.id,
            before: player.mana,
            after: next_player.mana,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.gold_earned != LaneGold::zero() {
        effects.push(LaneEffect::GoldChanged {
            actor: player.id,
            before: player.gold,
            after: next_player.gold,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.experience_gained != LaneExperience::zero() {
        effects.push(LaneEffect::ExperienceChanged {
            actor: player.id,
            before: player.experience,
            after: next_player.experience,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.cooldown != player.cooldown {
        effects.push(LaneEffect::CooldownChanged {
            actor: player.id,
            before: player.cooldown,
            after: next_player.cooldown,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.bounty_earned != LaneBounty::zero() {
        effects.push(LaneEffect::BountyChanged {
            actor: player.id,
            before: player.bounty,
            after: next_player.bounty,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.level_gained != LaneLevel::zero() {
        effects.push(LaneEffect::LevelChanged {
            actor: player.id,
            before: player.level,
            after: next_player.level,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.minion_kills_gained != LaneMinionKills::zero() {
        effects.push(LaneEffect::MinionKillsChanged {
            actor: player.id,
            before: player.minion_kills,
            after: next_player.minion_kills,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.shield_gained != LaneShield::zero() {
        effects.push(LaneEffect::ShieldChanged {
            actor: player.id,
            before: player.shield,
            after: next_player.shield,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if execution.ward_gained != LaneWard::zero() {
        effects.push(LaneEffect::WardChanged {
            actor: player.id,
            before: player.ward,
            after: next_player.ward,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.potion != player.potion {
        effects.push(LaneEffect::PotionChanged {
            actor: player.id,
            before: player.potion,
            after: next_player.potion,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.elixir != player.elixir {
        effects.push(LaneEffect::ElixirChanged {
            actor: player.id,
            before: player.elixir,
            after: next_player.elixir,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.trinket != player.trinket {
        effects.push(LaneEffect::TrinketChanged {
            actor: player.id,
            before: player.trinket,
            after: next_player.trinket,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.relic != player.relic {
        effects.push(LaneEffect::RelicChanged {
            actor: player.id,
            before: player.relic,
            after: next_player.relic,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.charm != player.charm {
        effects.push(LaneEffect::CharmChanged {
            actor: player.id,
            before: player.charm,
            after: next_player.charm,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.scroll != player.scroll {
        effects.push(LaneEffect::ScrollChanged {
            actor: player.id,
            before: player.scroll,
            after: next_player.scroll,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.tome != player.tome {
        effects.push(LaneEffect::TomeChanged {
            actor: player.id,
            before: player.tome,
            after: next_player.tome,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.rune != player.rune {
        effects.push(LaneEffect::RuneChanged {
            actor: player.id,
            before: player.rune,
            after: next_player.rune,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.sigil != player.sigil {
        effects.push(LaneEffect::SigilChanged {
            actor: player.id,
            before: player.sigil,
            after: next_player.sigil,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.talisman != player.talisman {
        effects.push(LaneEffect::TalismanChanged {
            actor: player.id,
            before: player.talisman,
            after: next_player.talisman,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.amulet != player.amulet {
        effects.push(LaneEffect::AmuletChanged {
            actor: player.id,
            before: player.amulet,
            after: next_player.amulet,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.phial != player.phial {
        effects.push(LaneEffect::PhialChanged {
            actor: player.id,
            before: player.phial,
            after: next_player.phial,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if let Some(queued) = resolved.delayed_effect_queued {
        effects.push(LaneEffect::DelayedEffectQueued {
            actor: player.id,
            effect: queued,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    for item in &resolved.delayed_effects_resolved {
        effects.push(LaneEffect::DelayedEffectResolved {
            actor: player.id,
            effect: *item,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_delayed(),
        });
    }
    if next_state.wave.pressure != state.wave.pressure {
        effects.push(LaneEffect::WavePressureChanged {
            before: state.wave.pressure,
            after: next_state.wave.pressure,
            cause: LaneEffectCause::Execution(trace),
            provenance: LaneEffectProvenance::direct_immediate(),
        });
    }
    if next_player.position != player.position {
        let cause = if resolved.fallback_activated {
            LaneEffectCause::Fallback
        } else {
            LaneEffectCause::Intent
        };
        let provenance = if resolved.fallback_activated {
            LaneEffectProvenance::indirect_immediate()
        } else {
            LaneEffectProvenance::direct_immediate()
        };
        effects.push(LaneEffect::PositionChanged {
            actor: player.id,
            before: player.position,
            after: next_player.position,
            cause,
            provenance,
        });
    }
    effects
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
    let resolved = resolve_lane_execution(state, command, execution)?;
    let next_state = resolved.next_state;
    let outcome = resolved.outcome;
    let trace = execution.trace;
    let events = project_lane_events(state, command, execution, &resolved, trace);
    let effects = project_lane_effects(state, command, execution, &resolved, trace);
    let debrief = LaneDebrief {
        decision: LaneDecisionReview::InformationConsistent,
        coordination: LaneCoordinationReview::NotApplicable,
        intent: command.command.intent,
        target_focus: command.command.target_focus,
        commitment: command.command.commitment,
        ping_signal: command.command.ping_signal,
        abort_condition: command.command.abort_condition,
        fallback_behavior: command.command.fallback_behavior,
        self_damage: execution.self_damage,
        mana_spent: execution.mana_spent,
        gold_earned: execution.gold_earned,
        experience_gained: execution.experience_gained,
        cooldown_set: execution.cooldown_set,
        bounty_earned: execution.bounty_earned,
        level_gained: execution.level_gained,
        minion_kills_gained: execution.minion_kills_gained,
        shield_gained: execution.shield_gained,
        ward_gained: execution.ward_gained,
        potion_gained: execution.potion_gained,
        potion_spent: execution.potion_spent,
        elixir_gained: execution.elixir_gained,
        elixir_spent: execution.elixir_spent,
        trinket_gained: execution.trinket_gained,
        trinket_spent: execution.trinket_spent,
        relic_gained: execution.relic_gained,
        relic_spent: execution.relic_spent,
        charm_gained: execution.charm_gained,
        charm_spent: execution.charm_spent,
        scroll_gained: execution.scroll_gained,
        scroll_spent: execution.scroll_spent,
        tome_gained: execution.tome_gained,
        tome_spent: execution.tome_spent,
        rune_gained: execution.rune_gained,
        rune_spent: execution.rune_spent,
        sigil_gained: execution.sigil_gained,
        sigil_spent: execution.sigil_spent,
        talisman_gained: execution.talisman_gained,
        talisman_spent: execution.talisman_spent,
        amulet_gained: execution.amulet_gained,
        amulet_spent: execution.amulet_spent,
        phial_gained: execution.phial_gained,
        phial_spent: execution.phial_spent,
        wave_result: execution.wave_result,
        fallback_activated: resolved.fallback_activated,
        delayed_effects_queued: if resolved.delayed_effect_queued.is_some() {
            1
        } else {
            0
        },
        delayed_effects_resolved: resolved.delayed_effects_resolved.len() as u8,
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
