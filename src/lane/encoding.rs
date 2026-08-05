use super::*;

pub(crate) const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
pub(crate) const MAX_LANE_HEALTH: u8 = 10;
pub(crate) const MAX_LANE_MANA: u8 = 6;
pub(crate) const MAX_LANE_GOLD: u8 = 20;
pub(crate) const MAX_LANE_EXPERIENCE: u8 = 50;
pub(crate) const MAX_LANE_COOLDOWN: u8 = 10;
pub(crate) const MAX_LANE_BOUNTY: u8 = 100;
pub(crate) const MAX_LANE_LEVEL: u8 = 18;
pub(crate) const MAX_LANE_MINION_KILLS: u8 = 200;
pub(crate) const MAX_LANE_SHIELD: u8 = 50;
pub(crate) const MAX_LANE_WARD: u8 = 5;
pub(crate) const MAX_WAVE_PRESSURE: u8 = 3;
pub(crate) const LANE_MANA_HASH_TAG: u8 = 0x4d;
pub(crate) const LANE_GOLD_HASH_TAG: u8 = 0x47;
pub(crate) const LANE_EXPERIENCE_HASH_TAG: u8 = 0x45;
pub(crate) const LANE_COOLDOWN_HASH_TAG: u8 = 0x43;
pub(crate) const LANE_BOUNTY_HASH_TAG: u8 = 0x42;
pub(crate) const LANE_LEVEL_HASH_TAG: u8 = 0x4c;
pub(crate) const LANE_MINION_KILLS_HASH_TAG: u8 = 0x4b;
pub(crate) const LANE_SHIELD_HASH_TAG: u8 = 0x53;
pub(crate) const LANE_WARD_HASH_TAG: u8 = 0x57;
pub(crate) const LANE_TARGET_FOCUS_HASH_TAG: u8 = 0x54;
pub(crate) const LANE_COMMITMENT_HASH_TAG: u8 = 0x56;
pub(crate) const LANE_DELAYED_EFFECT_HASH_TAG: u8 = 0x44;
pub(crate) const LANE_PING_SIGNAL_HASH_TAG: u8 = 0x50;
pub(crate) const LANE_ABORT_CONDITION_HASH_TAG: u8 = 0x41;

pub(crate) fn phase_tag(phase: LanePhase) -> u8 {
    match phase {
        LanePhase::Open => 0,
        LanePhase::Resolved => 1,
    }
}

pub(crate) fn position_tag(position: LanePosition) -> u8 {
    match position {
        LanePosition::NearTower => 0,
        LanePosition::Center => 1,
        LanePosition::FarSide => 2,
    }
}

pub(crate) fn posture_tag(posture: OpponentPosture) -> u8 {
    match posture {
        OpponentPosture::Aggressive => 0,
        OpponentPosture::Passive => 1,
    }
}

pub(crate) fn threat_tag(threat: JungleThreatTruth) -> u8 {
    match threat {
        JungleThreatTruth::Absent => 0,
        JungleThreatTruth::RiverSide => 1,
        JungleThreatTruth::InLane => 2,
    }
}

pub(crate) fn outcome_tag(outcome: Option<LaneOutcome>) -> u8 {
    match outcome {
        None => 0,
        Some(LaneOutcome::HeldSpace) => 1,
        Some(LaneOutcome::YieldedSpace) => 2,
        Some(LaneOutcome::ForcedOut) => 3,
    }
}
pub(crate) fn window_tag(window: LaneWindow) -> u8 {
    match window {
        LaneWindow::OneBeat => 0,
        LaneWindow::TwoBeats => 1,
    }
}
pub(crate) fn intent_tag(intent: LaneIntent) -> u8 {
    match intent {
        LaneIntent::Stabilize => 0,
        LaneIntent::Contest => 1,
        LaneIntent::Recall => 2,
        LaneIntent::Withdraw => 3,
        LaneIntent::Yield => 4,
    }
}

pub(crate) fn wave_result_tag(result: LaneWaveResult) -> u8 {
    match result {
        LaneWaveResult::Advanced => 0,
        LaneWaveResult::Held => 1,
        LaneWaveResult::Lost => 2,
    }
}

pub(crate) fn target_focus_tag(focus: LaneTargetFocus) -> u8 {
    match focus {
        LaneTargetFocus::Minions => 0,
        LaneTargetFocus::OpposingLaner => 1,
        LaneTargetFocus::Tower => 2,
    }
}

pub(crate) fn commitment_tag(commitment: LaneCommitment) -> u8 {
    match commitment {
        LaneCommitment::Standard => 0,
        LaneCommitment::Cautious => 1,
        LaneCommitment::Aggressive => 2,
    }
}

pub(crate) fn ping_signal_tag(signal: LanePingSignal) -> u8 {
    match signal {
        LanePingSignal::None => 0,
        LanePingSignal::Danger => 1,
        LanePingSignal::OnMyWay => 2,
        LanePingSignal::Assist => 3,
        LanePingSignal::EnemyMissing => 4,
    }
}

pub(crate) fn abort_condition_tag(condition: LaneAbortCondition) -> u8 {
    match condition {
        LaneAbortCondition::None => 0,
        LaneAbortCondition::HealthThreshold => 1,
        LaneAbortCondition::ThreatSpotted => 2,
        LaneAbortCondition::ResourceDepleted => 3,
    }
}
